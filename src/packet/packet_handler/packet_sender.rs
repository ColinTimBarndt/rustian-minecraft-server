use super::data;
use openssl::symm::*;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::Receiver;
extern crate colorful;
use colorful::Color;
use colorful::Colorful;

// use tokio::net::tcp::WriteHalf;
// type Writer = WriteHalf<'static>;
use tokio::io::WriteHalf;
type Writer = WriteHalf<tokio::net::TcpStream>;

pub struct PacketSender {
    writer: Writer,
    receiver: Receiver<PacketSenderMessage>,
    encrypter: Option<Crypter>,
    compression_threshold: Option<u32>,
}

#[derive(Debug)]
pub enum PacketSenderMessage {
    Packet(u32, Vec<u8>),
    Encrypt(Vec<u8>),
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
    pub async fn listen(mut self) -> Writer {
        loop {
            use PacketSenderMessage::*;
            match self.receiver.recv().await {
                Some(message) => match message {
                    Packet(packet_id, raw_packet) => match self.send(raw_packet, packet_id).await {
                        Ok(()) => (),
                        Err(e) => eprintln!(
                            "{}: {}",
                            "Error in packet sender thread".color(Color::Red),
                            e
                        ),
                    },
                    Encrypt(shared_secret) => self.set_encryption(&shared_secret),
                    Shutdown => {
                        return self.writer;
                    }
                },
                None => panic!("Outgoing channel got dropped"),
            };
        }
    }
    /// Sends a packet
    pub async fn send(
        &mut self,
        mut packet_data: Vec<u8>,
        packet_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let mut body_buffer = Vec::with_capacity(packet_data.len() + 4);

        // Compose body (id + packet)
        data::write::var_u32(&mut body_buffer, packet_id);
        body_buffer.append(&mut packet_data);

        // Compose header (length)
        data::write::var_u32(&mut buffer, body_buffer.len() as u32);
        buffer.append(&mut body_buffer);

        match &self.compression_threshold {
            Some(_t) => {
                // Compress
            }
            None => (),
        }

        // Send packet
        //#[cfg(debug_sending_packets)]
        println!(
            "{}: {:X} ({}) {}",
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
    pub fn set_encryption(&mut self, secret: &[u8]) {
        let cipher = Cipher::aes_128_cfb8();
        self.encrypter = Some(Crypter::new(cipher, Mode::Encrypt, secret, Some(secret)).unwrap())
    }
    #[allow(dead_code)]
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
}
