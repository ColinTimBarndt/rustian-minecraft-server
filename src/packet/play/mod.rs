#![allow(unused)]

pub mod receive;
pub mod send;

use crate::packet::{PacketParsingError, PacketReceiver, PacketSerialIn};
use std::error::Error;
extern crate colorful;
use colorful::{Color, Colorful};

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    buffer: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match id {
        receive::ClientSettings::ID => {
            use receive::ClientSettings;
            let packet = ClientSettings::consume_read(buffer)?;
            // TODO: Do something with this information
            println!("Player Settings: {}", packet);
        }
        receive::PluginMessage::ID => {
            use receive::PluginMessage;
            // TODO: Do something with this information
            let packet = PluginMessage::consume_read(buffer)?;
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
            println!("[play/mod.rs] Kicking user");
            use crate::helpers::chat_components::{ChatColor, ChatComponent, ChatComponentType};
            receiver
                .kick(
                    ChatComponent::new(ChatComponentType::Text(
                        "Server is not ready yet".to_string(),
                    ))
                    .set_color(ChatColor::DarkAqua),
                )
                .await?;
        }
        _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
    }
    Ok(())
}
