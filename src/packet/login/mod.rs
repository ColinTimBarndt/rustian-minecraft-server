use crate::helpers::mojang_api;
use crate::packet::{
    LoggingInState, PacketHandlerMessage, PacketParsingError, PacketReceiver, PacketSerialIn,
    PlayerConnectionState,
};
use crate::server::universe::entity::player::EntityPlayer;
use std::error::Error;

pub mod receive;
pub mod send;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    mut buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
    match id {
        receive::LoginStart::ID => {
            // Handle Login request
            use receive::LoginStart;
            use send::EncryptionRequest;

            let packet = LoginStart::read(&mut buffer)?;
            receiver.login_name = Some(packet.name);
            //println!("User attempts to log in: {}", packet.name);
            let key = receiver.key.as_ref().unwrap();
            let answer = EncryptionRequest::new(
                "".to_string(), // Empty when game version >= 1.7.0
                rsa_der::public_key_to_der(&mut key.n().to_vec(), &mut key.e().to_vec()),
                1, // Key verification security (vanilla: 1)
            );
            receiver.verify_token = Some(answer.verify_token.clone());
            //print!("Requesting encryption. Verify token = ");
            /*for byte in &answer.verify_token {
                if *byte < 0x10 {
                    print!("0{:X}", byte);
                } else {
                    print!("{:X}", byte);
                }
            }
            println!();*/
            receiver.send_packet(answer).await?;
            Ok(())
        }
        receive::EncryptionResponse::ID => {
            // Handle Encryption Response
            use receive::EncryptionResponse;
            use send::LoginSuccess;

            let packet = EncryptionResponse::read(&mut buffer)?;
            //println!("User responded to encryption request");
            let key = receiver.key.as_ref().unwrap();
            let mut verify_token = vec![0; key.size() as usize];
            let mut shared_secret = vec![0; key.size() as usize];
            {
                let amount_vt = key.private_decrypt(
                    &packet.verify_token, // from
                    &mut verify_token,    // to
                    openssl::rsa::Padding::PKCS1,
                )?;
                let amount_ss = key.private_decrypt(
                    &packet.shared_secret, // from
                    &mut shared_secret,    // to
                    openssl::rsa::Padding::PKCS1,
                )?;
                verify_token.truncate(amount_vt);
                shared_secret.truncate(amount_ss);
            }
            let correct_token = verify_token == *receiver.verify_token.as_ref().unwrap();
            //println!("\nVerify Token correct: {}", correct_token);
            if correct_token {
                let user_name = receiver.login_name.as_ref().unwrap().clone();
                receiver.logging_in = LoggingInState::LoggingIn;
                // Get User from Mojang API
                let profile = match mojang_api::has_joined(
                    "",
                    &shared_secret,
                    key,
                    &user_name,
                    &receiver.address,
                )
                .await
                {
                    Ok(Some(p)) => p,
                    Ok(None) => {
                        use crate::helpers::chat_components::{
                            ChatColor, ChatComponent, ChatComponentType,
                        };
                        receiver
                            .send_packet(send::Disconnect::from(
                                &ChatComponent::new(ChatComponentType::Translate {
                                    key: "disconnect.loginFailedInfo.invalidSession".into(),
                                    with: vec![],
                                })
                                .set_color(ChatColor::Red),
                            ))
                            .await?;
                        receiver
                            .handler_channel
                            .send(PacketHandlerMessage::CloseChannel)
                            .await?;
                        return Ok(());
                    }
                    Err(mojang_api::Error::MalformedResponse) => {
                        use crate::helpers::chat_components::{
                            ChatColor, ChatComponent, ChatComponentType,
                        };
                        receiver
                            .send_packet(send::Disconnect::from(
                                &ChatComponent::new(ChatComponentType::Translate {
                                    key: "disconnect.loginFailedInfo".into(),
                                    with: vec![ChatComponent::text(
                                        "Malformed response from the authentication server",
                                    )],
                                })
                                .set_color(ChatColor::Red),
                            ))
                            .await?;
                        receiver
                            .handler_channel
                            .send(PacketHandlerMessage::CloseChannel)
                            .await?;
                        return Ok(());
                    }
                    Err(mojang_api::Error::ServiceUnavailable) => {
                        use crate::helpers::chat_components::{
                            ChatColor, ChatComponent, ChatComponentType,
                        };
                        receiver
                            .send_packet(send::Disconnect::from(
                                &ChatComponent::new(ChatComponentType::Translate {
                                    key: "disconnect.serversUnavailable".into(),
                                    with: vec![],
                                })
                                .set_color(ChatColor::Red),
                            ))
                            .await?;
                        receiver
                            .handler_channel
                            .send(PacketHandlerMessage::CloseChannel)
                            .await?;
                        return Ok(());
                    }
                };
                let user_uuid = profile.uuid.clone();
                //
                let answer = LoginSuccess {
                    uuid: user_uuid.clone(),
                    username: user_name.clone(),
                };
                println!("Profile: {:#?}", profile);
                receiver.login_name = None;
                receiver.set_encryption(shared_secret).await?;
                receiver.send_packet(answer).await?;
                receiver.state = PlayerConnectionState::Play;
                receiver.verify_token = None;
                receiver.key = None;
                receiver.last_ping = Some(tokio::time::Instant::now());

                // Continue login squence
                use crate::packet::play::send::JoinGame;
                use crate::server::universe::{
                    world::{Dimension::*, LevelType::*},
                    Gamemode::*,
                };
                let (send, recv) = tokio::sync::oneshot::channel();
                receiver
                    .handler_channel
                    .send(super::PacketHandlerMessage::GetServer(send))
                    .await?;
                let mut server = recv.await?;
                let mut universe = server.get_universe(user_uuid.clone()).await?;
                let entity_player = {
                    let eid = universe
                        .reserve_entity_id()
                        .await
                        .map_err(|_| "Failed to reserve entity id")?;
                    EntityPlayer::new(eid, profile)
                };

                // TODO: Send information about the actual world
                let join_game_packet = JoinGame {
                    entity_id: entity_player.id,
                    gamemode: Creative,
                    dimension: Overworld,
                    seed_hash: 0,
                    level_type: Flat,
                    view_distance: 2,
                    reduced_debug_info: false,
                    show_respawn_screen: true,
                };
                receiver.intermediate_player = Some(Box::new(entity_player));
                // ✅ Join Game
                receiver.send_packet(join_game_packet).await?;
                use crate::helpers::NamespacedKey;
                // ✅ Plugin Message (brand)
                receiver
                    .send_packet(super::play::send::PluginMessage {
                        channel: NamespacedKey::new("minecraft", "brand"),
                        data: b"rustian",
                    })
                    .await?;
                // ✅ Difficulty
                receiver
                    .send_packet(super::play::send::Difficulty {
                        difficulty: crate::server::universe::world::Difficulty::Hard,
                        locked: false,
                    })
                    .await?;
                receiver.universe = Some(universe);
                receiver.logging_in = LoggingInState::AwaitClientSettings;
                // ✅ Player Abilities
                receiver
                    .send_packet(super::play::send::PlayerAbilities {
                        ..std::default::Default::default()
                    })
                    .await?;
            } else {
                receiver
                    .handler_channel
                    .send(PacketHandlerMessage::CloseChannel)
                    .await?;
            }
            Ok(())
        }
        _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
    }
}
