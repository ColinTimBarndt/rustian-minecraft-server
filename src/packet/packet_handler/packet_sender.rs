use super::data;
use crate::actor_model::*;
use crate::packet::packet_handler::ConnectionError;
use colorful::Color;
use colorful::Colorful;
use openssl::symm::*;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Error as IoError;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{Receiver, Sender};

// use tokio::net::tcp::WriteHalf;
// type Writer = WriteHalf<'static>;
use tokio::net::tcp::OwnedWriteHalf;
type Writer = OwnedWriteHalf;

pub struct PacketSender {
    writer: Writer,
    receiver: Receiver<PacketSenderMessage>,
    encrypter: Option<Crypter>,
    compression_threshold: Option<u32>,
}

#[derive(Debug)]
pub enum PacketSenderMessage {
    /// Tell the packet sender actor to send a packet
    Packet(u32, Vec<u8>),
    /// Tell the packet sender actor to send a packet
    PacketArc(u32, Arc<Vec<u8>>),
    /// Tell the packet sender actor to send a packet
    /// The actor is going to wait for the broadcast
    /// to send the packet
    PacketBroadcast(broadcast::Receiver<Option<Arc<(u32, Vec<u8>)>>>),
    /// Tell the sender thread to enable encryption with
    /// the given shared secret
    Encrypt(Vec<u8>),
    /// Tell the sender actor to shut down
    /// and return itself back to
    /// the packet handler thread
    Shutdown,
}

impl PacketSender {
    pub fn new(writer: Writer, receiver: Receiver<PacketSenderMessage>) -> Self {
        Self {
            writer,
            receiver,
            encrypter: None,
            compression_threshold: None,
        }
    }
    /// Listens for any outgoing packets that have to be sent
    pub async fn listen(mut self) -> Self {
        loop {
            use PacketSenderMessage::*;
            match self.receiver.recv().await {
                Some(message) => match message {
                    Packet(packet_id, raw_packet) => {
                        match self.send(&raw_packet, packet_id).await {
                            Ok(()) => (),
                            Err(e) => eprintln!(
                                "{}: {}",
                                "Error in packet sender thread".color(Color::Red),
                                e
                            ),
                        }
                    }
                    PacketArc(packet_id, raw_packet) => {
                        match self.send(&*raw_packet, packet_id).await {
                            Ok(()) => (),
                            Err(e) => eprintln!(
                                "{}: {}",
                                "Error in packet sender thread".color(Color::Red),
                                e
                            ),
                        }
                    }
                    PacketBroadcast(mut receiver) => match receiver.recv().await {
                        Ok(Some(shared_packet)) => {
                            let (packet_id, raw_packet) = &*shared_packet;
                            match self.send(raw_packet, *packet_id).await {
                                Ok(()) => (),
                                Err(e) => eprintln!(
                                    "{}: {}",
                                    "Error in packet sender thread".color(Color::Red),
                                    e
                                ),
                            }
                        }
                        _ => {
                            continue;
                        }
                    },
                    Encrypt(shared_secret) => self.set_encryption(&shared_secret),
                    Shutdown => {
                        return self;
                    }
                },
                None => panic!("Outgoing channel got dropped"),
            };
        }
    }
    /// Sends a packet
    pub async fn send(&mut self, packet_data: &[u8], packet_id: u32) -> Result<(), IoError> {
        let mut buffer = Vec::new();
        let mut body_buffer = Vec::with_capacity(packet_data.len() + 4);

        // Compose body (id + packet)
        data::write::var_u32(&mut body_buffer, packet_id);
        body_buffer.extend(packet_data);

        // Compose header (length)
        data::write::var_u32(&mut buffer, body_buffer.len() as u32);
        buffer.extend(body_buffer);

        match &self.compression_threshold {
            Some(_t) => {
                // Compress
            }
            None => (),
        }

        // Send packet
        //#[cfg(debug_sending_packets)]
        println!(
            "{}: {:#02X} ({}) {}",
            "▶ Sending packet".color(Color::DarkGray),
            //self.state,
            packet_id,
            if self.compression_threshold.is_some() {
                "compressed".color(Color::DarkMagenta1)
            } else {
                "no compression".color(Color::BlueViolet)
            },
            if self.encrypter.is_some() {
                //format!("🔐{}", "▮".color(Color::DarkGreen))
                "🔐"
            } else {
                //format!("🔓{}", "▮".color(Color::DarkOrange))
                "🔓"
            }
        );
        /*for byte in &buffer {
            //#[cfg(debug_sending_packets)]
            {
                if *byte < 0x10 {
                    print!("0{:X}", byte);
                } else {
                    print!("{:X}", byte);
                }
            }
        }*/

        // Encryption
        if self.encrypter.is_some() {
            buffer = self.encrypt_vec(buffer);
        }

        //#[cfg(debug_sending_packets)]
        //println!();
        self.writer.write(&buffer).await?;
        Ok(())
    }
    /// Activates encryption (only!) on the sending side
    pub fn set_encryption(&mut self, secret: &[u8]) {
        let cipher = Cipher::aes_128_cfb8();
        self.encrypter = Some(Crypter::new(cipher, Mode::Encrypt, secret, Some(secret)).unwrap())
    }
    #[allow(dead_code)]
    /// Encrypts one byte of data
    fn encrypt_byte(&mut self, byte: u8) -> u8 {
        let mut result = [0; 1];
        match self
            .encrypter
            .as_mut()
            .expect("Encryption is unset")
            .update(&[byte], &mut result)
        {
            Ok(amount) => {
                assert_eq!(
                    amount, 1,
                    "This should not happen. Encrypted result is too short"
                );
                return result[0];
            }
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }
    /// Encrypts multiple bytes of data
    fn encrypt_vec(&mut self, vec: Vec<u8>) -> Vec<u8> {
        let mut result = vec![0; vec.len()];
        match self
            .encrypter
            .as_mut()
            .expect("Encryption is unset")
            .update(&vec, &mut result)
        {
            Ok(amount) => {
                assert_eq!(
                    amount,
                    vec.len(),
                    "This should not happen. Encrypted result is too short"
                );
                return result;
            }
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }

    pub fn to_writer(self) -> Writer {
        self.writer
    }
}

#[derive(Clone)]
pub struct PacketSenderHandle {
    channel: Sender<PacketSenderMessage>,
}
impl PacketSenderHandle {
    pub fn new(channel: Sender<PacketSenderMessage>) -> Self {
        Self { channel }
    }

    pub async fn send_packet<P>(&mut self, packet: P) -> PacketSendResult
    where
        P: crate::packet::PacketSerialOut + Sized,
    {
        let id = P::ID;
        let mut buffer: Vec<u8> = Vec::new();
        packet
            .consume_write(&mut buffer)
            .map_err(|msg| ConnectionError::FailedToSend(id, msg))?;
        self.send_packet_raw(id, buffer).await
    }

    async fn send_packet_raw(&mut self, id: u32, buffer: Vec<u8>) -> PacketSendResult {
        self.channel
            .send(PacketSenderMessage::Packet(id, buffer))
            .await
            .map_err(|_| ActorMessagingError::new("Failed to send packet to packet sender"))?;
        Ok(())
    }
}

pub type PacketSendResult<T = ()> = Result<T, ConnectionError>;
