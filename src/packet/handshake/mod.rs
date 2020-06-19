use crate::packet::{PacketParsingError, PacketReceiver, PacketSerialIn};
use std::error::Error;

pub mod receive;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    buffer: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match id {
        receive::Handshake::ID => {
            // Handle handshake
            use receive::Handshake;
            let packet = Handshake::consume_read(buffer)?;
            receiver.state = packet.next_state.clone();
            //#[cfg(debug_handshake_packets)]
            /*{
                println!(
                    "Version {ver} Handshake using ip {addr}:{port} => {next}",
                    ver=packet.version,
                    addr=packet.address,
                    port=packet.port,
                    next=packet.next_state
                );
            }*/
            Ok(())
        }
        _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
    }
}
