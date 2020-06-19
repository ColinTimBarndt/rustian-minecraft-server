use crate::packet::{
    PacketHandlerMessage, PacketParsingError, PacketReceiver, PacketSerialIn, PlayerConnectionState,
};
use std::error::Error;
#[macro_use]
use crate::send_packet;

pub mod receive;
pub mod send;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    buffer: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match id {
        receive::LoginStart::ID => {
            // Handle Login request
            use receive::LoginStart;
            use send::EncryptionRequest;

            let packet = LoginStart::consume_read(buffer)?;
            //println!("User attempts to log in: {}", packet.name);
            let key = receiver.key.as_ref().unwrap();
            let answer = EncryptionRequest::new(
                "".to_string(), // Empty when game version >= 1.7.0
                rsa_der::public_key_to_der(&mut key.n().to_vec(), &mut key.e().to_vec()),
                1, // Key verification security (vanilla: 1)
            );
            receiver.verify_token = Some(answer.verify_token.clone());
            //print!("Requesting encryption. Verify token = ");
            for byte in &answer.verify_token {
                if *byte < 0x10 {
                    print!("0{:X}", byte);
                } else {
                    print!("{:X}", byte);
                }
            }
            println!();
            send_packet!(answer => receiver.send_packet)?;
            Ok(())
        }
        receive::EncryptionResponse::ID => {
            // Handle Encryption Response
            use receive::EncryptionResponse;
            use send::LoginSuccess;

            let packet = EncryptionResponse::consume_read(buffer)?;
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
                let answer = LoginSuccess {
                    uuid: uuid::Uuid::new_v4(),
                    username: "TEST".to_string(),
                };
                receiver.set_encryption(shared_secret).await?;
                send_packet!(answer => receiver.send_packet)?;
                receiver.state = PlayerConnectionState::Play;
                receiver.verify_token = None;
                receiver.key = None;

                // Continue login squence
                use crate::packet::play::send::JoinGame;
                use crate::server::universe::{
                    world::{Dimension::*, LevelType::*},
                    Gamemode::*,
                };
                let join_game_packet = JoinGame {
                    entity_id: 1000,
                    gamemode: Creative,
                    dimension: Overworld,
                    seed_hash: 0,
                    level_type: Flat,
                    view_distance: 2,
                    reduced_debug_info: false,
                    show_respawn_screen: true,
                };
                send_packet!(join_game_packet => receiver.send_packet)?;
                use crate::helpers::NamespacedKey;
                send_packet!(super::play::send::PluginMessage {
                    channel: NamespacedKey::new("minecraft", "brand"),
                    data: b"rustian".to_vec()
                } => receiver.send_packet)?;
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
