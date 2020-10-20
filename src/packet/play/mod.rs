#![allow(unused)]

pub mod receive;
pub mod send;

use crate::helpers::chat_components::{ChatColor, ChatComponent};
use crate::packet::{
  LoggingInState, PacketHandlerMessage, PacketParsingError, PacketReceiver, PacketSerialIn,
};
use crate::server::universe::entity::EntityActorHandle;
use colorful::{Color, Colorful};
use std::error::Error;

static PLAYER_COMMUNICATION_ERROR: &'static str = "Communication with player actor failed";

pub async fn handle(
  receiver: &mut PacketReceiver,
  id: u32,
  mut buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
  match id {
    receive::TeleportConfirm::ID => {
      let packet = receive::TeleportConfirm::read(&mut buffer)?;
      receiver
        .handler_channel
        .send(PacketHandlerMessage::RecvTeleportConfirm(packet.id))
        .await?;
    }
    receive::KeepAlive::ID => {
      use receive::KeepAlive;
      use tokio::time;
      let packet = KeepAlive::read(&mut buffer)?;
      let expected_id = receiver.last_ping_identifier;
      let received_id = packet.keep_alive_id;
      if receiver.waiting_for_ping && expected_id == received_id {
        if let Some(last_ping) = receiver.last_ping {
          receiver.waiting_for_ping = false;
          let now = time::Instant::now();
          receiver.last_ping_received = now;
          receiver.ping = now.duration_since(last_ping);
          receiver
            .handler_channel
            .send(super::packet_handler::PacketHandlerMessage::SetPing(
              receiver.ping,
            ))
            .await?;
          // Plan the next ping
          receiver.last_ping =
            Some(now + time::Duration::from_secs(super::packet_handler::PING_INTERVAL));
        } else {
          // User sent junk
          receiver.close_channel().await?;
        }
      } else {
        // User sent junk
        receiver.close_channel().await?;
      }
    }
    receive::PluginMessage::ID => {
      // TODO: Do something with this information
      let packet = receive::PluginMessage::read(&mut buffer)?;
      println!(
        "Plugin Message ({}): {}",
        format!("{}", packet.channel).color(Color::Gold3b),
        String::from_utf8(packet.data.clone())
          .unwrap_or_else(|_| {
            packet
              .data
              .iter()
              .map(|byte| format!("{:02X}", byte))
              .collect()
          })
          .color(Color::LightGray)
      );
    }
    receive::PlayerPosition::ID => {
      if receiver.logging_in == LoggingInState::LoggedIn {
        let packet = receive::PlayerPosition::read(&mut buffer)?;
        receiver
          .get_player()
          .player_moved(Some(packet.position), None, packet.on_ground)
          .await
          .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
      }
    }
    receive::PlayerPositionAndRotation::ID => {
      if receiver.logging_in == LoggingInState::LoggedIn {
        let packet = receive::PlayerPositionAndRotation::read(&mut buffer)?;
        receiver
          .get_player()
          .player_moved(
            Some(packet.position),
            Some(packet.rotation),
            packet.on_ground,
          )
          .await
          .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
      }
    }
    receive::PlayerRotation::ID => {
      if receiver.logging_in == LoggingInState::LoggedIn {
        let packet = receive::PlayerRotation::read(&mut buffer)?;
        receiver
          .get_player()
          .player_moved(None, Some(packet.rotation), packet.on_ground)
          .await
          .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
      }
    }
    receive::ClientSettings::ID => {
      let packet = receive::ClientSettings::read(&mut buffer)?;
      if receiver.logging_in == LoggingInState::LoggedIn {
        // Tell the player about the change in settings
        receiver
          .get_player()
          .update_settings(packet.settings)
          .await
          .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
        return Ok(());
      }
      // Is this part of the login sequence?
      if receiver.logging_in == LoggingInState::AwaitClientSettings {
        // Get the player out of the receiver, as it is
        // consumed later in order to spawn the player actor.
        let mut player = *std::mem::replace(&mut receiver.intermediate_player, None).unwrap();
        let mut universe = receiver.universe.as_ref().unwrap().clone();
        {
          {
            let held_item_packet = super::play::send::HeldItemChange {
              hotbar_slot: player.selected_hotbar_slot,
            };
            // ✅ Held Item Change
            receiver.send_packet(held_item_packet).await?;
          }

          {
            // TODO: Get recipes from universe
            let recipes_packet = super::play::send::DeclareRecipes {
              ..Default::default()
            };
            // ✅ Declare Recipes
            receiver.send_packet(recipes_packet).await?;
          }

          {
            // TODO: Store tags on server
            let tags_packet = super::play::send::Tags::new();
            // ✅ Tags
            receiver.send_packet(tags_packet).await?;
          }

          {
            // TODO: Get OP permission level from universe
            let status = super::play::send::EntityStatus {
              entity: player.id,
              status: super::play::send::entity_status::status::SET_OP_PERMISSION_0,
            };
            // ✅ Entity Status
            receiver.send_packet(status).await?;
          }

          {
            use crate::server::universe::commands::parsing::{NodeGraph, NodeType};
            // TODO: Get commands from universe
            let mut graph = NodeGraph::new();
            {
              // Build a parsing graph for testing:
              // - ROOT
              //   - "foobarbaz"
              // This means that the command `/foobarbaz` should exist now
              let root = graph.create_node(NodeType::Root);
              graph.set_root(&root);
              let command1 = graph.create_node(NodeType::Literal("foobarbaz".into()));
              graph.set_child(&root, &command1);
            }
            let commands_packet = super::play::send::DeclareCommands {
              command_parsing_graph: &graph,
            };
            // ✅ Declare Commands
            receiver.send_packet(commands_packet).await?;
          }

          {
            // TODO: Load unlocked recipes from a player record
            use super::play::send::unlock_recipes::{UnlockRecipes, UnlockRecipesAction};
            let packet = UnlockRecipes {
              action: UnlockRecipesAction::Initialize {
                init_recipes: &Vec::new(),
                unlocked_recipes: &Vec::new(),
              },
              crafting_recipe_book: Default::default(),
              smelting_recipe_book: Default::default(),
            };
            // ✅ Unlock Recipes
            receiver.send_packet(packet).await?;
          }

          // Continue by spawning the player
          receiver.logging_in = LoggingInState::AwaitSpawnPlayer;

          let mut world = universe
            .join_world(player.profile.uuid.clone())
            .await
            .map_err(|_| "Failed to join a world")?;

          let spawn = world
            .get_spawn_position()
            .await
            .map_err(|_| "Could not get spawn position of world")?;

          player.position = spawn;

          // This is going to continue the login sequence
          let mut player = world
            .spawn_entity_player_online(receiver.create_player_connection_handle(), player, false)
            .await
            .map_err(|_| "Failed to spawn player")?;

          // Done. Save the player handle
          receiver.player = Some(player);
          receiver.logging_in = LoggingInState::LoggedIn;

          // Kick player after some time to avoid having to restart
          // the game each and every time I attempted to test this
          // TODO: Temporary
          let mut con_handle = receiver.create_player_connection_handle();
          tokio::spawn(async move {
            use tokio::time::{delay_for, Duration};
            let delay = delay_for(Duration::new(20 /*sec*/, 0 /*nanosec*/));
            delay.await;
            println!("[play/mod.rs] Kicking user");
            use crate::helpers::chat_components::{ChatColor, ChatComponent};
            con_handle
              .send_packet(crate::packet::play::send::Disconnect::from(
                &ChatComponent::text("Server is not ready yet".to_owned())
                  .set_color(ChatColor::DarkAqua),
              ))
              .await
              .expect("[play/mod.rs] Error while sending kick packet");
            con_handle
              .close_channel()
              .await
              .expect("[play/mod.rs] Error in handler channel");
          });
        }
      }
    }
    receive::HeldItemChange::ID => {
      use receive::HeldItemChange;
      let packet = HeldItemChange::read(&mut buffer)?;
      if receiver.logging_in == LoggingInState::LoggedIn {
        let player = receiver.player.as_mut().unwrap();
        player.set_selected_hotbar_slot(packet.hotbar_slot, false);
        // TODO: DEBUG
        println!(
          "[play/mod.rs] Player {} selected hotbar slot {}",
          player.get_name(),
          packet.hotbar_slot
        );
      } else if let Some(player) = &receiver.intermediate_player {
        if player.selected_hotbar_slot != packet.hotbar_slot {
          // Wrong callback
          receiver
            .kick(
              ChatComponent::translate(
                "disconnect.loginFailedInfo",
                vec![ChatComponent::text("Invalid HeldItemChange callback")],
              )
              .set_color(ChatColor::Red),
            )
            .await?;
        }
      } else {
        // Unexpected packet
        receiver
          .kick(
            ChatComponent::translate(
              "disconnect.loginFailedInfo",
              vec![ChatComponent::text("Unexpected HeldItemChange packet")],
            )
            .set_color(ChatColor::Red),
          )
          .await?;
      }
    }
    _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
  }
  Ok(())
}
