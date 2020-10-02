use super::*;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::oneshot;
use tokio::time;

mod packet_receiver;
mod packet_sender;

pub use packet_receiver::*;
pub use packet_sender::*;

#[derive(Debug)]
pub enum PacketHandlerMessage {
  CloseChannel,
  StopListening,
  SetPing(time::Duration),
  GetPing(oneshot::Sender<time::Duration>),
  GetServer(oneshot::Sender<MinecraftServerHandle>),
}

pub struct PacketHandler {
  pub server: MinecraftServerHandle,
  address: SocketAddr,
  outgoing_channel: Sender<PacketSenderMessage>,
  receiver_shutdown_channel: Sender<()>,
  pub ping: time::Duration,
}

impl PacketHandler {
  pub async fn create(
    stream: TcpStream,
    addr: SocketAddr,
    server: MinecraftServerHandle,
    encryption: Arc<Rsa<Private>>,
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
      receiver_shutdown_channel: shutdown_sender,
      ping: time::Duration::from_secs(0),
    };
    {
      spawn(async move {
        // This function got added to Tokio later. Thank you so much 🙌!
        let (reader, writer) = TcpStream::into_split(stream);
        //use tokio::net::tcp::{ReadHalf, WriteHalf};
        /* let (reader, writer): (ReadHalf<'_>, WriteHalf<'_>) = tokio::io::split(stream); */
        /*let static_reader: ReadHalf<'static>;
        let static_writer: WriteHalf<'static>;
        unsafe {
          // I was trying to avoid using unsafe here but io::split is broken
          // This code should work because ´stream´ outlives both threads
          // But it is still unstable, do not do this
          static_reader = std::mem::transmute(reader);
          static_writer = std::mem::transmute(writer);
        }*/
        let packet_sender = PacketSender::new(writer, receiver);
        let writer_handle = spawn(async { packet_sender.listen().await });
        let packet_receiver = PacketReceiver::new(
          reader,
          sender_clone,
          handler_sender_clone,
          encryption,
          me.address.clone(),
        );
        let reader_handle = spawn(async move { packet_receiver.listen(shutdown_receiver).await });
        'HandlerChannelLoop: loop {
          let msg = handler_receiver.recv().await;
          use PacketHandlerMessage::*;
          match msg {
            Some(msg) => match msg {
              CloseChannel => {
                match me.close_channel().await {
                  Ok(()) => (),
                  Err(e) => eprintln!("Error while closing channel: {}", e),
                };
                break 'HandlerChannelLoop;
              }
              StopListening => {
                break 'HandlerChannelLoop;
              }
              SetPing(ping) => {
                me.ping = ping;
              }
              GetPing(sender) => {
                // Ignore the result
                let _ = sender.send(me.ping);
              }
              GetServer(sender) => {
                let _ = sender.send(me.server.clone());
              }
            },
            None => {
              //println!("Handler channel got dropped");
              break 'HandlerChannelLoop;
            }
          }
        }
        // Sleep until the connection can be closed
        let (reader_res, writer_res) = futures::join!(reader_handle, writer_handle);
        let (p_receiver, p_sender) = (reader_res.ok(), writer_res.ok());
        if let Some(p_receiver) = p_receiver {
          let reader = p_receiver.to_reader();
          if let Some(p_sender) = p_sender {
            let writer = p_sender.to_writer();
            let stream = reader.reunite(writer).unwrap();
            stream
              .shutdown(std::net::Shutdown::Both)
              .unwrap_or_else(|err| {
                eprintln!("Failed to shut down connection {}: {}", addr, err);
              });
          } else {
            eprintln!(
              "Failed to shut down connection {}: {}",
              addr, "Sender thread panicked"
            );
          }
        } else {
          if let Some(p_sender) = p_sender {
            let mut writer = p_sender.to_writer();
            writer.shutdown().await.unwrap_or_else(|err| {
              eprintln!("Failed to shut down connection {}: {}", addr, err);
            });
            eprintln!(
              "Failed to fully shut down connection {}: {}",
              addr, "Receiver thread panicked"
            );
          } else {
            eprintln!(
              "Failed to shut down connection {}: {}",
              addr, "Sender and receiver threads panicked"
            );
          }
        }
        //use tokio::io::ReadHalf;
        //let stream: TcpStream =
        //    ReadHalf::unsplit(p_receiver.to_reader(), p_sender.to_writer());
        //stream
        //    .shutdown(std::net::Shutdown::Both)
        //    .unwrap_or_else(|err| {
        //        panic!("Failed to shut down connection {}: {}", addr, err);
        //    });
        match me.server.player_disconnect(me.address).await {
          Ok(()) => (),
          Err(e) => eprintln!("Failed to dispatch player disconnect to server: {}", e),
        }
      });
    }
    (handler_sender, sender)
  }
  pub async fn send_packet(&mut self, id: u32, buffer: Vec<u8>) -> Result<(), String> {
    if let Err(e) = self
      .outgoing_channel
      .send(PacketSenderMessage::Packet(id, buffer))
      .await
    {
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
      return Err(format!("Receiver Thread shutdown channel error: {}", e));
    }
    if let Err(e) = r.1 {
      return Err(format!("Sender Thread channel error: {}", e));
    }
    Ok(())
  }
}
