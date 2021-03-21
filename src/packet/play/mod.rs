use std::sync::Arc;

use super::ConnectionError;
use crate::helpers::chat_components::{ChatColor, ChatComponent};
use crate::packet::{
    LoggingInState, PacketHandlerMessage, PacketParsingError, PacketReceiver, PacketSerialIn,
};
use crate::{actor_model::*, helpers::chat_components::ChatComponentType};
use colorful::{Color, Colorful};
use tokio::sync::Mutex;

pub mod receive;
pub mod send;

static PLAYER_COMMUNICATION_ERROR: ActorMessagingError =
    ActorMessagingError::new("Communication with player actor failed");

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    mut buffer: &[u8],
) -> Result<(), ConnectionError> {
    match id {
        receive::TeleportConfirm::ID => {
            let packet = receive::TeleportConfirm::read(&mut buffer)?;
            receiver
                .handler_channel
                .send(PacketHandlerMessage::RecvTeleportConfirm(packet.id))
                .await
                .map_err(|_| {
                    ActorMessagingError::new(
                        "Failed to send message RecvTeleportConfirm to packet handler",
                    )
                })?;
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
                        .await
                        .map_err(|_| {
                            ActorMessagingError::new(
                                "Failed to send message SetPing to packet handler",
                            )
                        })?;
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
            if receiver.logging_in_state().await == LoggingInState::LoggedIn {
                let mut guard = receiver.get_player().await;
                if let Some(player) = &mut *guard {
                    let packet = receive::PlayerPosition::read(&mut buffer)?;
                    player
                        .player_moved(Some(packet.position), None, packet.on_ground)
                        .await
                        .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
                }
            }
            return Ok(());
        }
        receive::PlayerPositionAndRotation::ID => {
            if receiver.logging_in_state().await == LoggingInState::LoggedIn {
                let mut guard = receiver.get_player().await;
                if let Some(player) = &mut *guard {
                    let packet = receive::PlayerPositionAndRotation::read(&mut buffer)?;
                    player
                        .player_moved(
                            Some(packet.position),
                            Some(packet.rotation),
                            packet.on_ground,
                        )
                        .await
                        .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
                }
            }
            return Ok(());
        }
        receive::PlayerRotation::ID => {
            if receiver.logging_in_state().await == LoggingInState::LoggedIn {
                let mut guard = receiver.get_player().await;
                if let Some(player) = &mut *guard {
                    let packet = receive::PlayerRotation::read(&mut buffer)?;
                    player
                        .player_moved(None, Some(packet.rotation), packet.on_ground)
                        .await
                        .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
                }
            }
            return Ok(());
        }
        receive::ClientSettings::ID => {
            let packet = receive::ClientSettings::read(&mut buffer)?;

            // Is this part of the login sequence?
            if receiver.logging_in == LoggingInState::AwaitClientSettings {
                // Get the player out of the receiver, as it is
                // consumed later in order to spawn the player actor.
                let mut player =
                    *std::mem::replace(&mut receiver.intermediate_player, None).unwrap();
                let mut universe = receiver.universe.as_ref().unwrap().clone();

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
                        let test_command =
                            graph.create_node(NodeType::Literal("test-command".into()));
                        graph.set_child(&root, &test_command);
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
                let spawn_player_callback = Arc::new(Mutex::from(false));
                receiver.logging_in = LoggingInState::AwaitSpawnPlayer;

                receiver.logging_in_spawn_player_callback = Some(spawn_player_callback.clone());

                let mut world = universe.join_world(player.profile.uuid.clone()).await?;

                let spawn = world.get_spawn_position().await?;

                player.position = spawn;

                {
                    // Add player to TAB menu
                    receiver
                        .send_packet(crate::packet::play::send::PlayerInfo::AddPlayer(&[
                            crate::packet::play::send::PlayerInfoAddPlayerEntry {
                                display_name: Some(&[
                                    ChatComponent::text("UwU").set_color(ChatColor::Gold)
                                ]),
                                gamemode: crate::server::universe::Gamemode::Adventure,
                                ping: -1,
                                profile: &player.profile,
                            },
                        ]))
                        .await?;
                }

                let mut world_clone = world.clone();
                let mut player_mutex_guard = receiver.player.clone().lock_owned().await;
                let con_handle = receiver.create_player_connection_handle();
                tokio::spawn(async move {
                    // This is going to continue the login sequence
                    let player = match world_clone
                        .spawn_entity_player_online(con_handle, player, false)
                        .await
                    {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("ERROR: {}", e);
                            panic!();
                        }
                    };

                    *player_mutex_guard = Some(player);
                    drop(player_mutex_guard);
                    *(spawn_player_callback.lock().await) = true;
                });
            } else if receiver.logging_in == LoggingInState::LoggedIn {
                // Tell the player actor about the change in settings
                let mut guard = receiver.get_player().await;
                if let Some(player) = &mut *guard {
                    player
                        .update_settings(packet.settings)
                        .await
                        .map_err(|_| PLAYER_COMMUNICATION_ERROR)?;
                } else {
                    return Err(ConnectionError::UnexpectedPacket(
                        receive::ClientSettings::ID,
                    ));
                }
            }
            return Ok(());
        }
        receive::HeldItemChange::ID => {
            use receive::HeldItemChange;
            let packet = HeldItemChange::read(&mut buffer)?;
            if receiver.logging_in_state().await == LoggingInState::LoggedIn {
                let mut guard = receiver.get_player().await;
                if let Some(player) = &mut *guard {
                    player
                        .set_selected_hotbar_slot(packet.hotbar_slot, false)
                        .await?;
                    // TODO: DEBUG
                    println!(
                        "[play/mod.rs] Player {} selected hotbar slot {}",
                        player.get_name(),
                        packet.hotbar_slot
                    );
                }
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
        _ => return Err(PacketParsingError::UnknownPacket(id).into()),
    }
    Ok(())
}
