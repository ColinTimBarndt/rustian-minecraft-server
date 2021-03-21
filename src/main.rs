#![recursion_limit = "512"]

use colorful::Color;
use colorful::Colorful;
use std::error::Error;
extern crate colorful;

pub mod actor_model;
pub mod helpers;
pub mod packet;
pub mod server;

fn main() -> Result<(), Box<dyn Error>> {
    use tokio::runtime;
    let mut rt: runtime::Runtime = runtime::Builder::new()
        .threaded_scheduler()
        .max_threads(1024)
        .thread_stack_size(2 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    let future = rt.enter(|| tokio_main());
    rt.block_on(future)?;
    Ok(())
}

async fn tokio_main() -> Result<(), Box<dyn Error>> {
    use server::*;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    println!("Creating Server");
    let server = MinecraftServer::new(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        25565,
    )))
    .await?;
    println!(
        "{msg}{ip}",
        msg = "Server is up and running on "
            .color(Color::LightSeaGreen)
            .bold(),
        ip = server.address
    );
    let (_server_handle, server_thread_handle) = MinecraftServer::listen(server);
    //chunk_test();
    // Let the server run
    server_thread_handle.await?;
    Ok(())
}

/*fn chunk_test() {
    use crate::helpers::Vec3d;
    use crate::server::universe::world::{
        block::blocks::GrassBlockData, chunk_generator::*, chunk_loader::*, Block, ChunkPosition,
    };

    let mut generator = FlatWorldGenerator::new(vec![
        (Block::Bedrock, 1),
        (Block::Stone, 5),
        (Block::Dirt, 2),
        (Block::GrassBlock(GrassBlockData { snowy: false }), 1),
        (Block::Grass, 1),
    ]);
    let chunk = generator.generate(ChunkPosition::new(0, 0));
    let mut loader = TemplateChunkLoader::new(chunk);
    let mut chunk = loader.load(ChunkPosition::new(1, 1));
    chunk.set_block_at_pos(Vec3d::new(1, 0, 0), Block::Granite);
    println!("{}", chunk);
    for y in 0..16 {
        println!(
            "Block[{}]: {}",
            y,
            chunk.get_block_at_pos(Vec3d::new(0, y, 0)).get_key()
        );
    }
    /*let secs = &chunk.sections;
    secs[0]
        .as_ref()
        .unwrap()
        .debug_dump_data("chunk_data.txt".into());*/
}*/
