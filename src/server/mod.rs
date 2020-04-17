use crate::server::universe::Player;
use std::sync::{Arc};
use futures::lock::{Mutex};
use crate::packet::{PlayerConnection, PlayerConnectionPacketHandle};
use std::error::Error;
use std::net::{SocketAddr};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use openssl::rsa::Rsa;
use openssl::pkey::Private;

pub mod universe;

pub struct MinecraftServer {
    pub address: SocketAddr,
    listener: TcpListener,
    connections: HashMap<SocketAddr, PlayerConnectionPacketHandle>,
    players: Vec<Player>,
    pub key_pair: Rsa<Private>
}

impl MinecraftServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(&addr).await?;

        let private_key = Rsa::generate(1024)?;

        let server = MinecraftServer {
            address: addr,
            listener: listener,
            connections: HashMap::new(),
            key_pair: private_key,
            players: Vec::new()
        };

        println!("=== PUBLIC KEY ===");
        for byte in server.key_pair.public_key_to_der()? {
            if byte<0x10 {
                print!("0{:X}", byte);
            } else {
                print!("{:X}", byte);
            }
        }
        println!("\n===");

        return Ok(server);
    }

    pub fn create_player() -> u32 {
        0
    }

    pub async fn listen(
        server_ref: Arc<Mutex<MinecraftServer>>
    ) {
        /*tokio::run(self.listener.incoming()
            .map_err(|e| eprintln!("Failed to accept connection: {:?}", e))
            .for_each(|socket| {
                Ok()
            })
        );*/
        loop {
            let mut server = server_ref.lock().await;
            let (socket, addr): (TcpStream, SocketAddr) = match server.listener.accept().await {
                Ok((a, b)) => (a, b),
                Err(e) => {
                    println!("Error in new connection with: {}", e);
                    return;
                }
            };
            drop(server); // drop the guard
            let ref_clone = server_ref.clone();
            match handle_client(ref_clone, socket, addr).await {
                Ok(()) => (),
                Err(e) => println!("Severe error in new connection: {}", e)
            };
            tokio::spawn(async move {

            });
            std::thread::sleep(std::time::Duration::from_millis(400));
        }

        async fn handle_client(
            server: Arc<Mutex<MinecraftServer>>,
            socket: TcpStream,
            address: SocketAddr
        ) -> Result<(), String> {
            println!("Connection from {}", address);
            let mut connection = match PlayerConnection::new(server.clone(), socket, address.clone()).await {
                Ok(o) => o,
                Err(e) => return Err(format!("{}", e))
            };

            //let mut server_lock = server.lock().await;
            //let packet_queue = connection.outgoing_queue.clone();
            //server_lock.connections.insert(address.clone(), packet_queue);
            //drop(server_lock);

            //connection.listen().await;

            //let mut server_lock = server.lock().await;
            //server_lock.connections.remove(&address);
            //drop(server_lock);
            Ok(())

            /*let mut buf: [u8; 1024] = [0; 1024];
            let n = socket.read(&mut buf).await?;
            //let rec = String::from_utf8(buf[0..n].to_vec())?;
            println!("Received:\n");
            for byte in buf[0..n].to_vec() {
                print!("{:X}", byte);
            }
            println!();*/
        }
    }
}

unsafe impl Send for MinecraftServer {}
