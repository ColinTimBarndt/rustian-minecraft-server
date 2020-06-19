use crate::packet::{
    packet_handler::PacketParsingError, PacketHandlerMessage, PacketReceiver, PacketSerialIn,
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
        receive::Request::ID => {
            // Handle server status request
            use receive::Request;
            use send::Response;

            Request::consume_read(buffer)?;
            //#[cfg(debug_status_packets)]
            {
                //println!("Client requested the server status");
            }
            let answer = Response::new();
            send_packet!(answer => receiver.send_packet)?;
            Ok(())
        }
        receive::Ping::ID => {
            // Handle ping
            use receive::Ping;
            use send::Pong;

            let packet = Ping::consume_read(buffer)?;
            //#[cfg(debug_status_packets)]
            {
                //println!("Client sent a ping, answering with pong");
            }
            let answer = Pong::new(packet.payload);
            send_packet!(answer => receiver.send_packet)?;
            receiver
                .handler_channel
                .send(PacketHandlerMessage::CloseChannel)
                .await?;
            Ok(())
        }
        _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
    }
}
