use super::*;
use tokio::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use futures::lock::Mutex;
use tokio::spawn;

mod packet_sender;
mod packet_receiver;

pub use packet_sender::*;
pub use packet_receiver::*;

type ArcMutex<T> = Arc<Mutex<T>>;

#[derive(Debug)]
pub enum PacketHandlerMessage {
    CloseChannel
}

pub struct PacketHandler {
    pub server: ArcMutex<MinecraftServer>,
    address: SocketAddr,
    outgoing_channel: Sender<PacketSenderMessage>,
    receiver_shutdown_channel: Sender<()>
}

#[macro_export]
macro_rules! send_packet {
    ($packet:expr => $var:ident . $sender:ident) => {
        {
            use crate::packet::{PacketSerialOut};
            let packet = $packet;
            let id = crate::packet::get_packet_id_out(&packet);
            let mut buffer = Vec::new();
            match packet.consume_write(&mut buffer) {
                Ok(()) => $var.$sender(id, buffer).await,
                Err(e) => Err(format!("{}", e))
            }
        }
    };
}

impl PacketHandler {
    pub async fn create(
        stream: TcpStream,
        addr: SocketAddr,
        server: ArcMutex<MinecraftServer>
    ) -> (Sender<PacketHandlerMessage>, Sender<PacketSenderMessage>) {
        let (sender, receiver) = channel(512);
        let (shutdown_sender, shutdown_receiver) = channel(1);
        let (handler_sender, mut handler_receiver) = channel(127);
        let handler_sender_clone = handler_sender.clone();
        let sender_clone = sender.clone();
        let mut me = Self {
            server,
            address: addr,
            outgoing_channel: sender.clone(),
            receiver_shutdown_channel: shutdown_sender
        };
        {
            spawn(async move {
                //use tokio::net::tcp::{ReadHalf, WriteHalf};
                let (reader, writer)/* : (ReadHalf<'_>, WriteHalf<'_>) */ = tokio::io::split(stream);
                /*let static_reader: ReadHalf<'static>;
                let static_writer: WriteHalf<'static>;
                unsafe {
                    // I was trying to avoid using unsafe here but io::split is broken
                    // This code should work because ´stream´ outlives both threads
                    static_reader = std::mem::transmute(reader);
                    static_writer = std::mem::transmute(writer);
                }*/
                let packet_sender = PacketSender::new(
                    writer,
                    receiver
                );
                let writer_handle = spawn(async {
                    packet_sender.listen().await
                });
                let packet_receiver = PacketReceiver::new(
                    reader,
                    sender_clone,
                    handler_sender_clone,
                    me.server.lock().await.key_pair.clone()
                );
                let reader_handle = spawn(async move {
                    packet_receiver.listen(shutdown_receiver).await
                });
                'HandlerChannelLoop: loop {
                    let msg = handler_receiver.recv().await;
                    use PacketHandlerMessage::*;
                    match msg {
                        Some(msg) => match msg {
                            CloseChannel => {
                                match me.close_channel().await {
                                    Ok(()) => (),
                                    Err(e) => eprintln!("Error while closing channel: {}", e)
                                };
                                break 'HandlerChannelLoop;
                            }
                        },
                        None => {
                            //panic!("Handler channel got dropped");
                            break 'HandlerChannelLoop;
                        }
                    }
                }
                let (reader_res, writer_res) = futures::join!(reader_handle, writer_handle);
                let (reader, writer) = (
                    reader_res.expect("Reader channel failed shutting down"),
                    writer_res.expect("Writer channel failed shutting down")
                );
                use tokio::io::ReadHalf;
                let stream: TcpStream = ReadHalf::unsplit(reader, writer);
                stream.shutdown(std::net::Shutdown::Both).unwrap_or_else(|err| {
                    panic!("Failed to shut down connection {}: {}", addr, err);
                });
            });
        }
        (handler_sender, sender)
    }
    pub async fn send_packet(&mut self, id: u32, buffer: Vec<u8>) -> Result<(), String> {
        if let Err(e) = self.outgoing_channel.send(PacketSenderMessage::Packet(id, buffer)).await {
            return Err(format!("{}", e));
        };
        Ok(())
    }
    pub async fn close_channel(&mut self) -> Result<(), String> {
        let r = futures::join!(
            self.receiver_shutdown_channel.send(()),
            self.outgoing_channel.send(PacketSenderMessage::Shutdown)
        );
        if let Err(e) = r.0 {
            return Err(format!("{}", e));
        }
        if let Err(e) = r.1 {
            return Err(format!("{}", e));
        }
        Ok(())
    }
}
