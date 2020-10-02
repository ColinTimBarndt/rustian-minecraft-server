use crate::actor_model::*;
use crate::packet::PlayerConnection;
use futures::{future::FutureExt, select};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use uuid::Uuid;

pub mod registries;
pub mod universe;
use universe::{Universe, UniverseHandle};

pub struct MinecraftServer {
    pub address: SocketAddr,
    pub listener: TcpListener,
    pub connections: HashMap<SocketAddr, PlayerConnection>,
    pub key_pair: Arc<Rsa<Private>>,
    pub universe: universe::UniverseHandle,
}

impl MinecraftServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(&addr).await?;

        let private_key = Rsa::generate(1024)?;

        println!("=== PUBLIC KEY ===");
        for byte in private_key.public_key_to_der()? {
            print!("{:02X}", byte);
        }
        println!("\n=== END PUBLIC KEY ===");

        let (_, u_handle) = create_universe().await;

        let server = MinecraftServer {
            address: addr,
            listener: listener,
            connections: HashMap::new(),
            key_pair: Arc::new(private_key),
            universe: u_handle,
        };

        return Ok(server);
    }

    pub fn listen(self) -> (MinecraftServerHandle, tokio::task::JoinHandle<()>) {
        /*tokio::run(self.listener.incoming()
            .map_err(|e| eprintln!("Failed to accept connection: {:?}", e))
            .for_each(|socket| {
                Ok()
            })
        );*/
        let (send, recv) = channel(128);
        let send2 = send.clone();
        let server_thread_handle = tokio::spawn(self.start_actor(send, recv));
        return (MinecraftServerHandle::new(send2), server_thread_handle);
    }

    async fn start_actor(
        mut self,
        send: Sender<MinecraftServerHandleMessage>,
        mut recv: Receiver<MinecraftServerHandleMessage>,
    ) {
        loop {
            select! {
                res = self.listener.accept().fuse() => {
                    let (socket, addr): (TcpStream, SocketAddr) = match res {
                        Ok((a, b)) => (a, b),
                        Err(e) => {
                            println!("Error in new connection with: {}", e);
                            continue;
                        }
                    };
                    match MinecraftServer::handle_client(MinecraftServerHandle::new(send.clone()), socket, addr, self.key_pair.clone()).await {
                        Ok(connection) => {self.connections.insert(connection.address.clone(), connection);},
                        Err(e) => eprintln!("Severe error in new connection: {}", e),
                    };
                    std::thread::sleep(std::time::Duration::from_millis(400));
                },
                msg_opt = recv.recv().fuse() => {
                    if let Some(msg) = msg_opt {
                        if !self.handle_actor_message(msg).await {
                            return;
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

    /// Returns whether the actor should continue running
    async fn handle_actor_message(&mut self, msg: MinecraftServerHandleMessage) -> bool {
        match msg {
            MinecraftServerHandleMessage::Shutdown => {
                //TODO dispatch shutdown event to universe
                println!("Shutting down server");
                for (_, connection) in &mut self.connections {
                    connection
                        .close_channel()
                        .await
                        .expect("Failed to close connections");
                }
                return false;
            }
            MinecraftServerHandleMessage::PlayerDisconnect(addr) => {
                //TODO dispatch disconnect event to universe
                println!("Disconnect from {}", addr);
                self.connections.remove(&addr);
            }
            MinecraftServerHandleMessage::GetUniverse(_uuid, sender) => {
                match sender.send(self.universe.clone()) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!("[server/mod.rs] Failed to send universe handle");
                    }
                }
            }
        }
        true
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
    pub async fn get_universe(&mut self, player: Uuid) -> Result<UniverseHandle, String> {
        let (send, recv) = oneshot::channel();
        self.server_channel
            .send(MinecraftServerHandleMessage::GetUniverse(player, send))
            .await
            .map_err(|e| format!("{}", e))?;
        Ok(recv.await.map_err(|e| format!("{}", e))?)
    }
}

pub enum MinecraftServerHandleMessage {
    Shutdown,
    PlayerDisconnect(SocketAddr),
    GetUniverse(Uuid, oneshot::Sender<UniverseHandle>),
}

async fn create_universe() -> (JoinHandle<Universe>, UniverseHandle) {
    use crate::helpers::NamespacedKey;
    use crate::server::universe::world::{blocks, Block};
    use crate::server::universe::world::{chunk_generator, chunk_loader};
    let overworld_key =
        NamespacedKey::new(crate::helpers::MINECRAFT_NAMESPACE, universe::OVERWORLD_KEY);
    let universe = Universe::new(overworld_key.clone());
    let (jh, mut handle) = universe.spawn_actor();
    let generator = chunk_generator::FlatWorldGenerator::new(vec![
        (Block::Bedrock, 1),
        (Block::Dirt, 3),
        (Block::GrassBlock(blocks::GrassBlockData::default()), 1),
    ]);
    handle
        .create_world(
            overworld_key,
            Box::new(generator),
            Box::new(chunk_loader::VoidChunkLoader::new()),
        )
        .await
        .expect("Failed to communicate with World Actor");
    (jh, handle)
}
