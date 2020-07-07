#![allow(unused)]

pub mod receive;
pub mod send;

use crate::packet::{PacketParsingError, PacketReceiver, PacketSerialIn};
use crate::send_packet;
use std::error::Error;
//extern crate colorful;
use colorful::{Color, Colorful};

pub async fn handle(
  receiver: &mut PacketReceiver,
  id: u32,
  mut buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
  match id {
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
      use receive::PluginMessage;
      // TODO: Do something with this information
      let packet = PluginMessage::read(&mut buffer)?;
      println!(
        "Plugin Message ({}): {}",
        format!("{}", packet.channel).color(Color::Gold3b),
        String::from_utf8(packet.data.clone())
          .unwrap_or_else(|_| {
            packet
              .data
              .iter()
              .map(|byte| {
                if *byte < 10 {
                  format!("0{}", byte)
                } else {
                  format!("{}", byte)
                }
              })
              .collect()
          })
          .color(Color::LightGray)
      );
    }
    receive::ClientSettings::ID => {
      use receive::ClientSettings;
      let packet = ClientSettings::read(&mut buffer)?;
      // Get access to the player struct and apply the settings
      // TODO: Handle a change of settings AFTER the login sequence finished
      let mut plock = receiver.player.as_ref().unwrap().write().await;
      plock.settings = packet.settings;
      drop(plock);
      if (receiver.logging_in) {
        {
          {
            // TODO: Take value from player struct
            let held_item_packet = super::play::send::HeldItemChange { hotbar_slot: 4 };
            send_packet!(held_item_packet => receiver.send_packet)?;
            receiver
              .player
              .as_ref()
              .unwrap()
              .write()
              .await
              .set_selected_hotbar_slot_server_side(4);
          }

          {
            // TODO: Load recipes into universe and use them here
            let recipes_packet = super::play::send::DeclareRecipes {
              ..Default::default()
            };
            send_packet!(recipes_packet => receiver.send_packet)?;
          }

          {
            // TODO: Store tags on server
            let tags_packet = super::play::send::Tags::new();
            send_packet!(tags_packet => receiver.send_packet)?;
          }

          {
            // TODO: Get OP permission level from universe
            let status = super::play::send::EntityStatus {
              entity: receiver
                .player
                .as_ref()
                .unwrap()
                .read()
                .await
                .get_entity_id(),
              status: super::play::send::entity_status::status::SET_OP_PERMISSION_0,
            };
            send_packet!(status => receiver.send_packet)?;
          }

          {
            use crate::server::universe::commands::parsing::{NodeGraph, NodeType};
            // TODO: Get commands from universe
            let mut graph = NodeGraph::new();
            {
              let root = graph.create_node(NodeType::Root);
              graph.set_root(&root);
              let command1 = graph.create_node(NodeType::Literal("foobarbaz".into()));
              graph.set_child(&root, &command1);
            }
            let commands_packet = super::play::send::DeclareCommands {
              command_parsing_graph: &graph,
            };
            send_packet!(commands_packet => receiver.send_packet)?;
          }

          {
            use super::play::send::unlock_recipes::{UnlockRecipes, UnlockRecipesAction};
            let packet = UnlockRecipes {
              action: UnlockRecipesAction::Initialize {
                init_recipes: &Vec::new(),
                unlocked_recipes: &Vec::new(),
              },
              crafting_recipe_book: Default::default(),
              smelting_recipe_book: Default::default(),
            };
            send_packet!(packet => receiver.send_packet)?;
          }

          // TODO: Temporary
          let mut con_handle = receiver.create_player_connection_handle();
          tokio::spawn(async move {
            use tokio::time::{delay_for, Duration};
            let delay = delay_for(Duration::new(10 /*sec*/, 0 /*nanosec*/));
            delay.await;
            println!("[play/mod.rs] Kicking user");
            use crate::helpers::chat_components::{ChatColor, ChatComponent, ChatComponentType};
            send_packet!(crate::packet::play::send::Disconnect::from(
                        ChatComponent::new(ChatComponentType::Text(
                            "Server is not ready yet".to_string(),
                        ))
                        .set_color(ChatColor::DarkAqua)) => con_handle.send_packet)
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
      let mut player = receiver.player.as_ref().unwrap().write().await;
      player.set_selected_hotbar_slot_server_side(packet.hotbar_slot);
      // TODO: DEBUG
      println!(
        "[play/mod.rs] Player {} selected hotbar slot {}",
        player.get_name(),
        packet.hotbar_slot
      );
    }
    _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
  }
  Ok(())
}
