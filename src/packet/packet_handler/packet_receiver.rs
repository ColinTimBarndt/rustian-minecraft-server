use super::{data, PacketHandlerMessage, PacketSenderMessage, PlayerConnectionState};
use crate::server::universe::SharedPlayer;
use futures::{future::FutureExt, pin_mut, select};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use openssl::symm::*;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::{Receiver, Sender};
extern crate colorful;
use colorful::{Color, Colorful};

// use tokio::net::tcp::ReadHalf;
// type Reader = /*ReadHalf<tokio::net::TcpStream>*/ReadHalf<'static>;
use tokio::net::tcp::OwnedReadHalf;
type Reader = OwnedReadHalf;

pub struct PacketReceiver {
    reader: Reader,
    decrypter: Option<Crypter>,
    compression_threshold: Option<u32>,
    outgoing_channel: Sender<PacketSenderMessage>,
    pub handler_channel: Sender<PacketHandlerMessage>,
    pub state: PlayerConnectionState,
    pub key: Option<Arc<Rsa<Private>>>,
    pub verify_token: Option<Vec<u8>>,
    pub login_name: Option<String>,
    pub player: Option<SharedPlayer>,
    pub logging_in: bool,
}

macro_rules! handle_err {
    ($expr:expr => $fmt:literal) => {
        match $expr {
            Ok(o) => o,
            Err(e) => return Err(format!($fmt, e)),
        }
    };
}

impl PacketReceiver {
    pub fn new(
        reader: Reader,
        packet_sender: Sender<PacketSenderMessage>,
        handler_channel: Sender<PacketHandlerMessage>,
        key: Arc<Rsa<Private>>,
    ) -> Self {
        Self {
            reader: reader,
            decrypter: None,
            compression_threshold: None,
            verify_token: None,
            outgoing_channel: packet_sender,
            handler_channel,
            state: PlayerConnectionState::Handshake,
            key: Some(key),
            login_name: None,
            player: None,
            logging_in: true,
        }
    }
    /// Listens for any incoming packets
    pub async fn listen(mut self, mut cancel: Receiver<()>) -> Self {
        let cancel_task = cancel.recv().fuse();
        pin_mut!(cancel_task);
        // This loop processes all packets to be sent
        // The sending future is cancelled in case a cancel message is received
        loop {
            select! {
                r = self.handle_packet().fuse() => {
                    match r {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("Error in packet receiver thread: {}", e);

                            let r = futures::join!(
                                self.outgoing_channel.send(PacketSenderMessage::Shutdown),
                                self.handler_channel.send(PacketHandlerMessage::StopListening)
                            );

                            r.0.expect("Failed to shut down sending channel");
                            r.1.expect("Failed to notify packet handler");
                            return self;

                            //self.handler.lock().await.close_channel().await;
                        }
                    }
                    continue;
                },
                opt = cancel_task => match opt {
                    Some(()) => {
                        //self.reader.as_ref().shutdown(std::net::Shutdown::Read);
                        return self;
                    },
                    None => panic!("Shutdown channel got dropped")
                },
            }
        }
    }
    /// Waits for an incoming packet and handles it
    async fn handle_packet(&mut self) -> Result<(), String> {
        //println!("[packet_receiver:86] Listening for next packet ...");

        let len = if self.decrypter.is_some() {
            handle_err!(read_enc_var_i32(self).await => "Error while reading encrypted packet length: {}")
        } else {
            handle_err!(read_var_i32(&mut self.reader).await => "Error while reading packet length: {}")
        } as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(len);

        {
            let mut bb: Vec<u8> = vec![0u8; len];
            handle_err!(self.reader.read_exact(&mut bb).await => "Error while reading packet: {}");
            buffer.append(&mut bb);
        }

        if self.decrypter.is_some() {
            buffer = self.decrypt_vec(buffer);
        }

        {
            //println!("[packet_receiver:106] Received packet!");
            /*for byte in &buffer {
                if *byte < 0x10 {
                    print!("0{:X}", byte);
                } else {
                    print!("{:X}", byte);
                }
            }
            println!();*/
        }

        handle_err!(self.handle_uncompressed_packet(buffer).await => "Error while processing packet: {}");

        return Ok(());

        #[inline]
        // Read a VarInt packet data type
        // See https://wiki.vg/Protocol#Data_types
        async fn read_var_i32(reader: &mut Reader) -> Result<i32, Box<dyn Error>> {
            let mut num_read = 0u32;
            let mut result: i32 = 0;
            loop {
                let mut buf = [0u8; 1];
                //println!("[packet_receiver:127] Reading byte"); // DEBUG
                reader.read_exact(&mut buf).await?;
                let read: u8 = buf[0];
                let val = read & 0b01111111;
                result |= (val as i32) << (7 * num_read);

                num_read += 1;
                if num_read > 5 {
                    return Err(Box::new(PacketParsingError::VarNumberTooBig));
                }
                if (read & 0b10000000) == 0 {
                    return Ok(result);
                }
            }
        }

        #[inline]
        // Read a VarInt packet data type (encrypted)
        // See https://wiki.vg/Protocol#Data_types
        async fn read_enc_var_i32(receiver: &mut PacketReceiver) -> Result<i32, Box<dyn Error>> {
            let mut num_read = 0u32;
            let mut result: i32 = 0;
            loop {
                let mut buf = [0u8; 1];
                receiver.reader.read_exact(&mut buf).await?;
                let read: u8 = receiver.decrypt_byte(buf[0]);
                let val = read & 0b01111111;
                result |= (val as i32) << (7 * num_read);

                num_read += 1;
                if num_read > 5 {
                    return Err(Box::new(PacketParsingError::VarNumberTooBig));
                }
                if (read & 0b10000000) == 0 {
                    return Ok(result);
                }
            }
        }
    }

    /// Handles the raw, uncompressed binary data of a packet by deserializing and processing it
    async fn handle_uncompressed_packet(
        &mut self,
        mut buffer: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let id = data::read::var_i32(&mut buffer)? as u32;
        /*#[cfg(debug_receiving_packets)]
        print!(
            "[packet_receiver:173] Packet {} {} received:\n -> ",
            self.state, id
        );*/
        /*for byte in buffer.iter() {
            if *byte < 0x10 {
                print!("0{:X}", byte);
            } else {
                print!("{:X}", byte);
            }
        }*/
        println!(
            "{}: {} ({}) {}",
            "◀ Received packet".color(Color::DarkGray),
            //self.state,
            format!("{}::{:#04X}", self.state, id),
            if self.compression_threshold.is_some() {
                "compressed".color(Color::DarkMagenta1)
            } else {
                "no compression".color(Color::BlueViolet)
            },
            if self.decrypter.is_some() {
                //format!("🔐{}", "▮".color(Color::DarkGreen))
                "🔐"
            } else {
                //format!("🔓{}", "▮".color(Color::DarkOrange))
                "🔓"
            }
        );

        use crate::packet;
        match self.state {
            PlayerConnectionState::Handshake => packet::handshake::handle(self, id, buffer).await,
            PlayerConnectionState::Status => packet::status::handle(self, id, buffer).await,
            PlayerConnectionState::Login => packet::login::handle(self, id, buffer).await,
            PlayerConnectionState::Play => packet::play::handle(self, id, buffer).await,
        }
    }

    /// Sends a packet
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

    /// Clones the packet sender channel
    pub fn clone_packet_sender(&self) -> Sender<PacketSenderMessage> {
        self.outgoing_channel.clone()
    }

    pub fn create_player_connection_handle(&self) -> super::PlayerConnectionPacketHandle {
        (self.handler_channel.clone(), self.outgoing_channel.clone()).into()
    }

    /// Send the correct kick packet and close the connection
    pub async fn kick(
        &mut self,
        msg: crate::helpers::chat_components::ChatComponent,
    ) -> Result<(), Box<dyn Error>> {
        println!(
            "[packet_receiver:204] Kicking: {}",
            json::stringify(msg.make_json())
        );
        use PlayerConnectionState::*;
        match self.state {
            Login => {
                crate::send_packet!(crate::packet::login::send::Disconnect::from(msg) => self.send_packet)
            }
            Play => {
                crate::send_packet!(crate::packet::play::send::Disconnect::from(msg) => self.send_packet)
            }
            _ => Ok(()),
        }?;
        self.handler_channel
            .send(PacketHandlerMessage::CloseChannel)
            .await?;
        Ok(())
    }
    /// Activates encryption on the receiving and sending side of the connection
    pub async fn set_encryption(&mut self, secret: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let cipher = Cipher::aes_128_cfb8();
        self.decrypter = Some(Crypter::new(cipher, Mode::Decrypt, &secret, Some(&secret)).unwrap());
        self.outgoing_channel
            .send(PacketSenderMessage::Encrypt(secret))
            .await?;
        Ok(())
    }
    /// Decrypts one byte of data
    fn decrypt_byte(&mut self, byte: u8) -> u8 {
        let mut result = [0; 1];
        match self
            .decrypter
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
    /// Decrypts multiple bytes of data
    fn decrypt_vec(&mut self, vec: Vec<u8>) -> Vec<u8> {
        let mut result = vec![0; vec.len()];
        match self
            .decrypter
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

    pub fn to_reader(self) -> Reader {
        self.reader
    }
}

#[derive(Debug)]
pub enum PacketParsingError {
    UnknownPacket(u32),
    EndOfInput,
    VarNumberTooBig,
    InvalidPacket(String),
    ConnectionClosed,
}

impl std::fmt::Display for PacketParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PacketParsingError::*;
        match self {
            UnknownPacket(id) => write!(f, "Unknown packet {:02X}", id),
            EndOfInput => write!(f, "Unexpected end of input"),
            VarNumberTooBig => write!(f, "Variable number is too big"),
            InvalidPacket(desc) => write!(f, "Invalid Packet: {}", desc),
            ConnectionClosed => write!(f, "Connection Closed"),
        }
    }
}

impl Error for PacketParsingError {}
