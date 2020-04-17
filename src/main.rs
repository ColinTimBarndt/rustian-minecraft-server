use std::error::Error;
use std::sync::{Arc};
use futures::lock::Mutex;
extern crate colorful;
use colorful::Color;
use colorful::Colorful;

pub mod packet;
pub mod server;
pub mod helpers;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use server::*;
    use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
    println!("Creating Server");
    let server = match MinecraftServer::new(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 25565))
    ).await {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    println!(
        "{msg}{ip}",
        msg="Server is up and running on ".color(Color::LightSeaGreen).bold(),
        ip=server.address
    );
    MinecraftServer::listen(Arc::new(Mutex::new(server))).await;
    Ok(())
}
