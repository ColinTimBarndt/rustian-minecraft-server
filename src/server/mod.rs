use crate::packet::PlayerConnection;
use crate::server::universe::Player;
use futures::{future::FutureExt, select};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};

pub mod universe;

pub struct MinecraftServer {
    pub address: SocketAddr,
    listener: TcpListener,
    connections: HashMap<SocketAddr, PlayerConnection>,
    players: Vec<Player>,
    pub key_pair: Arc<Rsa<Private>>,
}

impl MinecraftServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(&addr).await?;

        let private_key = Rsa::generate(1024)?;

        println!("=== PUBLIC KEY ===");
        for byte in private_key.public_key_to_der()? {
            if byte < 0x10 {
                print!("0{:X}", byte);
            } else {
                print!("{:X}", byte);
            }
        }
        println!("\n=== END PUBLIC KEY ===");

        let server = MinecraftServer {
            address: addr,
            listener: listener,
            connections: HashMap::new(),
            key_pair: Arc::new(private_key),
            players: Vec::new(),
        };

        return Ok(server);
    }

    pub fn create_player() -> u32 {
        0
    }

    pub fn listen(
        mut server: MinecraftServer,
    ) -> (MinecraftServerHandle, tokio::task::JoinHandle<()>) {
        /*tokio::run(self.listener.incoming()
            .map_err(|e| eprintln!("Failed to accept connection: {:?}", e))
            .for_each(|socket| {
                Ok()
            })
        );*/
        let (send, recv) = channel(128);
        let send2 = send.clone();
        let server_thread_handle = tokio::spawn(handle_connections(server, send, recv));
        return (MinecraftServerHandle::new(send2), server_thread_handle);

        async fn handle_connections(
            mut server: MinecraftServer,
            send: Sender<MinecraftServerHandleMessage>,
            mut recv: Receiver<MinecraftServerHandleMessage>,
        ) {
            loop {
                select! {
                    res = server.listener.accept().fuse() => {
                        let (socket, addr): (TcpStream, SocketAddr) = match res {
                            Ok((a, b)) => (a, b),
                            Err(e) => {
                                println!("Error in new connection with: {}", e);
                                continue;
                            }
                        };
                        match handle_client(MinecraftServerHandle::new(send.clone()), socket, addr, server.key_pair.clone()).await {
                            Ok(connection) => {server.connections.insert(connection.address.clone(), connection);},
                            Err(e) => println!("Severe error in new connection: {}", e),
                        };
                        std::thread::sleep(std::time::Duration::from_millis(400));
                    },
                    msg_opt = recv.recv().fuse() => {
                        if let Some(msg) = msg_opt {
                            match msg {
                                MinecraftServerHandleMessage::Shutdown => {
                                    //TODO dispatch shutdown event to universe
                                    println!("Shutting down server");
                                    return;
                                }
                                MinecraftServerHandleMessage::PlayerDisconnect(addr) => {
                                    //TODO dispatch disconnect event to universe
                                    println!("Disconnect from {}", addr);
                                    server.connections.remove(&addr);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        async fn handle_client(
            server: MinecraftServerHandle,
            socket: TcpStream,
            address: SocketAddr,
            encryption: Arc<Rsa<Private>>,
        ) -> Result<PlayerConnection, String> {
            println!("Connection from {}", address);
            let connection =
                match PlayerConnection::new(server, socket, address.clone(), encryption).await {
                    Ok(o) => o,
                    Err(e) => return Err(format!("{}", e)),
                };

            println!("Connection handled for ip {}", address);

            //let mut server_lock = server.lock().await;
            //let packet_queue = connection.outgoing_queue.clone();
            //server_lock.connections.insert(address.clone(), packet_queue);
            //drop(server_lock);

            //connection.listen().await;

            //let mut server_lock = server.lock().await;
            //server_lock.connections.remove(&address);
            //drop(server_lock);
            Ok(connection)

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

#[derive(Clone, Debug)]
pub struct MinecraftServerHandle {
    server_channel: Sender<MinecraftServerHandleMessage>,
}

impl MinecraftServerHandle {
    fn new(sender: Sender<MinecraftServerHandleMessage>) -> Self {
        Self {
            server_channel: sender,
        }
    }
    pub async fn shutdown(&mut self) -> Result<(), SendError<MinecraftServerHandleMessage>> {
        self.server_channel
            .send(MinecraftServerHandleMessage::Shutdown)
            .await?;
        Ok(())
    }
    pub async fn player_disconnect(
        &mut self,
        addr: SocketAddr,
    ) -> Result<(), SendError<MinecraftServerHandleMessage>> {
        self.server_channel
            .send(MinecraftServerHandleMessage::PlayerDisconnect(addr))
            .await?;
        Ok(())
    }
}

pub enum MinecraftServerHandleMessage {
    Shutdown,
    PlayerDisconnect(SocketAddr),
}
