#![allow(unused)]

pub mod send;
pub mod receive;

use std::error::Error;
use crate::packet::{PacketReceiver, PacketSerialIn, PacketParsingError};

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    buffer: Vec<u8>
) -> Result<(), Box<dyn Error>> {
    println!("PACKET PLAY {}", id);
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
                packet.channel,
                String::from_utf8(packet.data.clone()).unwrap_or_else(|_| {
                    packet.data.iter().map(|byte| {
                        if *byte < 10 {
                            format!("0{}", byte)
                        } else {
                            format!("{}", byte)
                        }
                    }).collect()
                })
            );
        },
        _ => return Err(Box::new(
            PacketParsingError::UnknownPacket(id)
        ))
    }
    Ok(())
}
