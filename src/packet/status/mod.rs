use crate::packet::{
    packet_handler::PacketParsingError, PacketHandlerMessage, PacketReceiver, PacketSerialIn,
};
use std::error::Error;

pub mod receive;
pub mod send;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    mut buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
    match id {
        receive::Request::ID => {
            // Handle server status request
            use receive::Request;
            use send::Response;

            Request::read(&mut buffer)?;
            //#[cfg(debug_status_packets)]
            {
                //println!("Client requested the server status");
            }
            let answer = Response::new();
            receiver.send_packet(answer).await?;
            Ok(())
        }
        receive::Ping::ID => {
            // Handle ping
            use receive::Ping;
            use send::Pong;

            let packet = Ping::read(&mut buffer)?;
            //#[cfg(debug_status_packets)]
            {
                //println!("Client sent a ping, answering with pong");
            }
            let answer = Pong::new(packet.payload);
            receiver.send_packet(answer).await?;
            receiver
                .handler_channel
                .send(PacketHandlerMessage::CloseChannel)
                .await?;
            Ok(())
        }
        _ => return Err(Box::new(PacketParsingError::UnknownPacket(id))),
    }
}
