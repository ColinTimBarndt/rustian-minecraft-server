use super::ConnectionError;
use crate::actor_model::*;
use crate::helpers::chat_components::{ChatColor, ChatComponent};
use crate::packet::{
    packet_handler::PacketParsingError, PacketHandlerMessage, PacketReceiver, PacketSerialIn,
};

pub mod receive;
pub mod send;

pub async fn handle(
    receiver: &mut PacketReceiver,
    id: u32,
    mut buffer: &[u8],
) -> Result<(), ConnectionError> {
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
            let mut answer = Response::new();
            let description = [
                ChatComponent::text("A ").set_color(ChatColor::Gray),
                ChatComponent::text("Rust").set_color(ChatColor::Red),
                ChatComponent::text("ian Minecraft server").set_color(ChatColor::Gray),
            ];
            let sample = [("Hello", uuid::Uuid::nil()), ("World!", uuid::Uuid::nil())];
            answer.status.description = &description;
            answer.status.sample = &sample;
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
                .await
                .map_err(|_| ActorMessagingError::new("Failed to close channel"))?;
            Ok(())
        }
        id => return Err(PacketParsingError::UnknownPacket(id).into()),
    }
}
