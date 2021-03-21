use super::ConnectionError;
use crate::packet::{PacketParsingError, PacketReceiver, PacketSerialIn};

pub mod receive;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    mut buffer: &[u8],
) -> Result<(), ConnectionError> {
    match id {
        receive::Handshake::ID => {
            // Handle handshake
            use receive::Handshake;
            let packet = Handshake::read(&mut buffer)?;
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
        _ => return Err(PacketParsingError::UnknownPacket(id).into()),
    }
}
