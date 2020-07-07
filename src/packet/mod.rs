//#![allow(unused)]

use crate::server::MinecraftServerHandle;
use core::hash::{Hash, Hasher};
use num_derive::FromPrimitive;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
extern crate colorful;

pub mod packet_handler;
use packet_handler::*;

// Serial helper
pub mod data;

// Packets
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

pub trait PacketSerialIn: Sized {
    const ID: u32;
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError>;
}

pub trait PacketSerialOut: Sized {
    const ID: u32;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String>;
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
        self.write(buffer)
    }
}

/*#[derive(Debug)]
pub enum Packet {
    Handshake(crate::packet::handshake::receive::Handshake)
}*/

#[inline]
pub fn get_packet_id_out<T: PacketSerialOut>(_: &T) -> u32 {
    T::ID
}
#[inline]
pub fn get_packet_id_in<T: PacketSerialIn>(_: &T) -> u32 {
    T::ID
}

#[derive(Debug)]
pub struct PlayerConnection {
    pub address: SocketAddr,
    pub server: MinecraftServerHandle,
    handler_channel: Sender<PacketHandlerMessage>,
    sender_channel: Sender<PacketSenderMessage>,
    pub player: Option<Uuid>,
}

#[derive(Clone, Debug)]
pub struct PlayerConnectionPacketHandle {
    handler_channel: Sender<PacketHandlerMessage>,
    sender_channel: Sender<PacketSenderMessage>,
}

/*pub struct PlayerConnectionEncryption {
    pub shared_secret: Vec<u8>,
    pub cipher: Cipher,
    pub encrypter: Crypter,
    pub decrypter: Crypter
}*/

#[derive(FromPrimitive, Debug, Clone)]
pub enum PlayerConnectionState {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl Display for PlayerConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use PlayerConnectionState::*;
        write!(
            f,
            "{}",
            match self {
                Handshake => "Handshake",
                Status => "Status",
                Login => "Login",
                Play => "Play",
            }
        )
    }
}

impl Hash for PlayerConnection {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.address.hash(hasher);
    }
}

impl PlayerConnection {
    pub async fn new(
        server: MinecraftServerHandle,
        socket: TcpStream,
        addr: SocketAddr,
        encryption: Arc<Rsa<Private>>,
    ) -> Result<Self, Box<dyn Error>> {
        let (handler_channel, sender_channel) =
            PacketHandler::create(socket, addr, server.clone(), encryption).await;
        Ok(Self {
            address: addr.clone(),
            server: server,
            player: None,
            handler_channel,
            sender_channel,
        })
    }

    pub fn new_handle(&self) -> PlayerConnectionPacketHandle {
        (self.handler_channel.clone(), self.sender_channel.clone()).into()
    }

    pub async fn close_channel(&mut self) -> Result<(), SendError<PacketHandlerMessage>> {
        self.handler_channel
            .send(PacketHandlerMessage::CloseChannel)
            .await
    }

    /*pub async fn listen(&mut self) {
        if self.listen {
            return;
        }
        self.listen = true;
        loop {
            match self.handle_packet().await {
                Ok(()) => {
                    if self.listen {
                        continue;
                    } else {
                        return;
                    }
                },
                Err(e) => {
                    println!(
                        "{} {}: {}",
                        "Error in connection with".color(Color::LightRed),
                        self.ip_address,
                        e
                    );
                    self.close();
                    return;
                }
            }
        }
    }*/

    /*pub fn close(&mut self) {
        self.listen = false;
        println!("Shutting down socket {}", self.ip_address);
        match self.socket.shutdown(std::net::Shutdown::Both) {
            Ok(()) => (),
            Err(e) => println!("Error while shutting down socket {}: {}", self.ip_address, e)
        }
    }*/
}

impl PlayerConnectionPacketHandle {
    pub async fn send_packet(&mut self, id: u32, buffer: Vec<u8>) -> Result<(), String> {
        if let Err(e) = self
            .sender_channel
            .send(PacketSenderMessage::Packet(id, buffer))
            .await
        {
            return Err(format!("{}", e));
        };
        Ok(())
    }
    pub async fn close_channel(&mut self) -> Result<(), SendError<PacketHandlerMessage>> {
        self.handler_channel
            .send(PacketHandlerMessage::CloseChannel)
            .await
    }
}

impl From<(Sender<PacketHandlerMessage>, Sender<PacketSenderMessage>)>
    for PlayerConnectionPacketHandle
{
    fn from(from: (Sender<PacketHandlerMessage>, Sender<PacketSenderMessage>)) -> Self {
        Self {
            handler_channel: from.0,
            sender_channel: from.1,
        }
    }
}

/*#[allow(dead_code)]
impl PlayerConnectionEncryption {
    pub fn new(secret: Vec<u8>) -> Self {
        let cipher = Cipher::aes_128_cfb8();
        Self {
            cipher: cipher,
            encrypter: Crypter::new(cipher, symm::Mode::Encrypt, &secret, Some(&secret)).unwrap(),
            decrypter: Crypter::new(cipher, symm::Mode::Decrypt, &secret, Some(&secret)).unwrap(),
            shared_secret: secret
        }
    }
    fn encrypt_byte(&mut self, byte: u8) -> u8 {
        let mut result = [0; 1];
        match self.encrypter.update(&[byte], &mut result) {
            Ok(amount) => {
                assert_eq!(
                    amount, 1,
                    "This should not happen. Encrypted result is too short"
                );
                return result[0];
            },
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }
    fn encrypt_vec(&mut self, vec: Vec<u8>) -> Vec<u8> {
        let mut result = vec![0; vec.len()];
        match self.encrypter.update(&vec, &mut result) {
            Ok(amount) => {
                assert_eq!(
                    amount, vec.len(),
                    "This should not happen. Encrypted result is too short"
                );
                return result;
            },
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }
    fn decrypt_byte(&mut self, byte: u8) -> u8 {
        let mut result = [0; 1];
        match self.decrypter.update(&[byte], &mut result) {
            Ok(amount) => {
                assert_eq!(
                    amount, 1,
                    "This should not happen. Encrypted result is too short"
                );
                return result[0];
            },
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }
    fn decrypt_vec(&mut self, vec: Vec<u8>) -> Vec<u8> {
        let mut result = vec![0; vec.len()];
        match self.decrypter.update(&vec, &mut result) {
            Ok(amount) => {
                assert_eq!(
                    amount, vec.len(),
                    "This should not happen. Encrypted result is too short"
                );
                return result;
            },
            Err(e) => {
                panic!("Error when encrypting: {}", e);
            }
        };
    }
}*/
