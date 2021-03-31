use super::chunk_generator::ChunkGenerator;
use super::chunk_loader::ChunkLoader;
use super::{entity_list::EntityListEntry, region::*, Block, EntityList};
use super::{Chunk, ChunkPosition};
use crate::actor_model::*;
use crate::helpers::{NamespacedKey, Vec3d};
use crate::packet::{packet_handler::ConnectionError, PlayerConnectionPacketHandle};
use crate::server::universe::entity::player::online_controller::{
    Controller as OnlinePlayer, ControllerHandle as OnlinePlayerHandle,
};
use crate::server::universe::{entity, UniverseHandle};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub struct BlockWorld {
    regions: HashMap<RegionPosition, (JoinHandle<Region>, RegionHandle)>,
    loaded_chunks: HashSet<super::ChunkPosition>,
    handle: Option<<BlockWorld as Actor>::Handle>,
    pub id: NamespacedKey,
    pub universe: UniverseHandle,
    pub generator: Box<dyn ChunkGenerator>,
    pub loader: Box<dyn ChunkLoader>,
    pub entities: EntityList,
    pub spawn_position: Vec3d<f64>,
}

impl BlockWorld {
    pub fn new<G, L>(universe: UniverseHandle, id: NamespacedKey, generator: G, loader: L) -> Self
    where
        G: Into<Box<dyn ChunkGenerator>>,
        L: Into<Box<dyn ChunkLoader>>,
    {
        Self {
            id,
            universe,
            handle: None,
            regions: HashMap::new(),
            loaded_chunks: HashSet::new(),
            generator: generator.into(),
            loader: loader.into(),
            entities: Default::default(),
            spawn_position: Vec3d::new(8.0, 16.0, 8.0),
        }
    }
    /// Returns a block at the given world position. If the chunk containing the
    /// block is not loaded, it is loaded/generated if `load = true`, otherwise
    /// `None` will be returned. `None` will also be returned if the position is
    /// outside of the world bounds.
    pub async fn get_block_at_pos(&mut self, pos: Vec3d<i32>, load: bool) -> Option<Block> {
        if !(0..256).contains(&pos.y) {
            // Outside of chunk building limit
            return None;
        }
        let region_position: RegionPosition = pos.into();
        let chunk_position: ChunkPosition = pos.into();
        if self.loaded_chunks.contains(&chunk_position) {
            let region = self.get_region_handle(region_position).unwrap();
            match region
                .get_block(Vec3d::new(pos.x % 32, pos.y, pos.z % 32))
                .await
            {
                Ok(b) => b,
                Err(_) => {
                    eprintln!("[world.rs] Unable to get block in region");
                    return None;
                }
            }
        } else {
            if load {
                let block = self
                    .load_chunk(chunk_position, |chunk| {
                        chunk.get_block_at_pos(Vec3d::new(
                            (pos.x % 16) as u8,
                            pos.y as u8,
                            (pos.z % 16) as u8,
                        ))
                    })
                    .await;
                Some(block)
            } else {
                None
            }
        }
    }

    pub async fn set_block_at_pos(&mut self, pos: Vec3d<i32>, block: Block) {
        if !(0..256).contains(&pos.y) {
            // Outside of chunk building limit
            return;
        }
        let region_position: RegionPosition = pos.into();
        let chunk_position: ChunkPosition = pos.into();
        if self.loaded_chunks.contains(&chunk_position) {
            let region = self.get_region_handle(region_position).unwrap();
            match region
                .set_block(Vec3d::new(pos.x % 32, pos.y, pos.z % 32), block)
                .await
            {
                Ok(()) => (),
                Err(_) => {
                    eprintln!("[world.rs] Unable to get block in region");
                    return;
                }
            }
        } else {
            self.load_chunk(chunk_position, |chunk| {
                chunk.set_block_at_pos(
                    Vec3d::new((pos.x % 16) as u8, pos.y as u8, (pos.z % 16) as u8),
                    block,
                );
            })
            .await;
        }
    }

    #[inline]
    pub fn get_region_handle(&mut self, pos: RegionPosition) -> Option<&mut RegionHandle> {
        self.regions.get_mut(&pos).map(|(_jh, handle)| handle)
    }

    /// Loads a chunk and (optionally) processes it
    pub async fn load_chunk<T: Sized>(
        &mut self,
        pos: ChunkPosition,
        modifier: impl FnOnce(&mut Chunk) -> T,
    ) -> T {
        let region_pos: RegionPosition = pos.into();
        let mut chunk = if let Some(loaded_chk) = self.loader.load_chunk(pos) {
            loaded_chk
        } else {
            self.generator.generate(pos)
        };
        let r = modifier(&mut chunk);
        if let Some(region) = self.get_region_handle(region_pos) {
            region.set_chunk(chunk).await.unwrap();
            self.loaded_chunks.insert(pos);
        } else {
            let chunks = Box::new([None, None, None, None]);
            let mut region_struct = Region::new(region_pos, chunks);
            region_struct.set_chunk(chunk);
            let (jh, handle) = region_struct.spawn_actor();
            self.regions.insert(region_pos, (jh, handle));
            self.loaded_chunks.insert(pos);
        }
        r
    }
}

#[async_trait]
impl Actor for BlockWorld {
    type Handle = WorldHandle;

    async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
        match message {
            WorldMessage::GetBlockAtPos(pos, callback, load) => {
                let get_fut = self.get_block_at_pos(pos, load);
                match callback.send(get_fut.await) {
                    Ok(()) => (),
                    Err(_) => {
                        eprintln!("[world.rs] Failed to send GetBlockAtPos result");
                    }
                }
                true
            }
            WorldMessage::SetBlockAtPos(pos, block) => {
                self.set_block_at_pos(pos, block).await;
                true
            }
            WorldMessage::GetSpawnPosition(callback) => {
                match callback.send(self.spawn_position) {
                    Ok(()) => (),
                    Err(_) => {
                        eprintln!("[world.rs] Failed to send GetSpawnPosition result");
                    }
                }
                true
            }
            WorldMessage::SpawnEntityPlayerOnline {
                mut connection,
                mut entity,
                generate_id,
                callback,
            } => {
                let mut universe = self.universe.clone();
                let mut world = self.clone_handle();
                let spawn_position = self.spawn_position.clone();
                // Do this async because this thread has better things to do
                tokio::task::spawn(async move {
                    if let Err(e) = (async move {
                        use crate::packet::play::send::{
                            world_border::*, PlayerPositionAndLook, SpawnPosition,
                            UpdateViewPosition,
                        };
                        if generate_id {
                            entity.id = universe.reserve_entity_id().await?;
                        }
                        let entity_id = entity.id;
                        // Send these packets:
                        // - UpdateLight
                        // - ChunkData
                        // - WorldBorder
                        // - SpawnPosition
                        // - PlayerPositionAndLook

                        // ✅ UpdateViewPosition
                        connection
                            .send_packet(UpdateViewPosition {
                                position: super::ChunkPosition::from(entity.position),
                            })
                            .await?;

                        let mut controller = entity::player::online_controller::Controller::new(
                            connection,
                            entity,
                            world.clone(),
                        );
                        // ✅ UpdateLight
                        // ✅ ChunkData
                        {
                            // The client expects to receive 7*7 chunks around the player to
                            // be sent before spawning.
                            let radius: i32 = 3;
                            let chunks_total = ((radius * 2 + 1) * (radius * 2 + 1)) as usize;
                            let mut joins = Vec::with_capacity(chunks_total);
                            let player_chunk: ChunkPosition = controller.entity.position.into();
                            for x in player_chunk.x - radius..=player_chunk.x + radius {
                                for z in player_chunk.z - radius..=player_chunk.z + radius {
                                    let chunk_pos = ChunkPosition::new(x, z);
                                    let future = world
                                        .player_subscribe_chunk(
                                            chunk_pos,
                                            entity_id,
                                            controller.connection.clone(),
                                            // Send the full chunk
                                            true,
                                        )
                                        .await?;
                                    // Wait for the region to respond
                                    joins.push(tokio::task::spawn(async move {
                                        future.await.map(|handle| (chunk_pos, handle))
                                    }));
                                }
                            }
                            // Wait until all chunks have been sent
                            for join in joins {
                                let (chunk_pos, handle) = join.await.map_err(|_| {
                                    ActorMessagingError::new("Failed to load chunk (task panicked)")
                                })??;
                                // Save the subscription so that the player can unsubscribe
                                // when the chunk gets out of viewing distance or a
                                // disconnect happens.
                                controller.chunk_subscriptions.insert(chunk_pos, handle);
                            }
                        }

                        // ✅ World Border
                        // TODO: Get this data from the world
                        controller
                            .connection
                            .send_packet(WorldBorder::Initialize {
                                position: (8.0, 8.0),
                                lerp: WorldBorderLerp {
                                    from: 1024.0,
                                    to: 1024.0,
                                    speed: 0,
                                },
                                teleport_boundary: WorldBorder::DEFAULT_TELEPORT_BOUNDARY,
                                warning_time: 0,
                                warning_blocks: 0,
                            })
                            .await?;

                        // ✅ Spawn Position
                        controller
                            .connection
                            .send_packet(SpawnPosition {
                                position: spawn_position.into(),
                            })
                            .await?;

                        println!("Sending player position...");

                        // ✅ PlayerPositionAndLook
                        controller
                            .connection
                            .send_teleport_packet(PlayerPositionAndLook::create_abs(
                                0,
                                controller.entity.position,
                                controller.entity.head_rotation,
                            ))
                            // Send teleport
                            .await?
                            // Wait for client response
                            .await?;

                        println!("Received teleport callback!!11!elf!");
                        // Then, spawn player actor
                        let (jh, handle) = controller.spawn_actor();
                        let cloned_handle = handle.clone();
                        let mut cloned_world = world.clone();
                        world
                            .insert_entity_player_online((
                                tokio::task::spawn(async move {
                                    let controller = jh.await;
                                    let _ = cloned_world.entity_actor_stopped(entity_id).await;
                                    // This may panic. This is intended
                                    controller.unwrap()
                                }),
                                handle,
                            ))
                            .await
                            .expect("[world.rs] Failed to communicate with own world");
                        callback.send(cloned_handle).map_err(|_| {
                            ActorMessagingError::new(
                                "Failed to send world callback on spawning player",
                            )
                        })?;
                        Ok(())
                    })
                    .await
                    {
                        // Declare type of error
                        let e: ConnectionError = e;
                        // Error handling
                        eprintln!("[world.rs/SpawnEntityPlayerOnline] {}", e);
                        // TODO: handle error
                    }
                });
                true
            }
            WorldMessage::PrivateInsertEntityPlayerOnline(tuple) => {
                self.entities.insert(tuple);
                true
            }
            WorldMessage::PrivateEntityActorStopped { id } => {
                use crate::server::registries::EntityType;
                match self.entities.get_type_of_entity(id) {
                    Some(EntityType::Player) => {
                        if EntityListEntry::<OnlinePlayer>::has(&self.entities, id) {
                            // Player
                            let (jh, _handle) = self.entities.remove(id).unwrap();
                            let result: Result<OnlinePlayer, _> = jh.await;
                            match result {
                                Ok(mut controller) => {
                                    // Attempt to close the connection if it is not already closed
                                    let _ = controller.connection.close_channel().await;
                                    // Try to unsubscribe from all chunks
                                    let _ = controller.unsubscribe_all_chunks().await;
                                }
                                Err(_) => {
                                    // Can't do anything in this case
                                } // TODO: Tell other online players to remove this player
                            }
                        } else {
                            // NPC
                        }
                    }
                    Some(_) => {
                        unimplemented!();
                        // TODO
                    }
                    // Stopping the actor was planned, the entity has already
                    // been removed from the list.
                    None => (),
                }
                true
            }
            WorldMessage::PlayerSubscribeChunk {
                chunk,
                player_id,
                connection,
                send_complete,
                callback,
            } => {
                let region_pos: RegionPosition = chunk.into();
                if !self.loaded_chunks.contains(&chunk) {
                    self.load_chunk(chunk, |_| ()).await;
                }
                let region = self.get_region_handle(region_pos).unwrap();
                if let Err(_e) = region
                    .player_subscribe(chunk, player_id, connection, send_complete, callback)
                    .await
                {
                    eprintln!("[world.rs] Failed to communicate with region");
                    return false;
                }
                true
            }
        }
    }

    fn create_handle(
        &self,
        sender: mpsc::Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
    ) -> Self::Handle {
        sender.into()
    }

    fn set_handle(&mut self, handle: Self::Handle) {
        self.handle = Some(handle);
    }

    fn clone_handle(&self) -> Self::Handle {
        self.handle.as_ref().unwrap().clone()
    }
}

pub enum WorldMessage {
    /// A position, a callback and whether to load unloaded chunks
    GetBlockAtPos(Vec3d<i32>, oneshot::Sender<Option<Block>>, bool),
    SetBlockAtPos(Vec3d<i32>, Block),
    GetSpawnPosition(oneshot::Sender<Vec3d<f64>>),
    SpawnEntityPlayerOnline {
        connection: PlayerConnectionPacketHandle,
        entity: entity::player::EntityPlayer,
        generate_id: bool,
        callback: oneshot::Sender<entity::player::online_controller::ControllerHandle>,
    },
    PlayerSubscribeChunk {
        chunk: ChunkPosition,
        player_id: u32,
        connection: PlayerConnectionPacketHandle,
        /// If the chunk should be sent as a complete
        /// chunk first
        send_complete: bool,
        callback: oneshot::Sender<RegionHandle>,
    },
    PrivateInsertEntityPlayerOnline(EntityPlayerOnlineHandleTuple),
    PrivateEntityActorStopped {
        id: u32,
    },
}

type EntityPlayerOnlineHandleTuple = (
    tokio::task::JoinHandle<entity::player::online_controller::Controller>,
    entity::player::online_controller::ControllerHandle,
);

pub type WorldHandle = ActorHandleStruct<WorldMessage>;

impl WorldHandle {
    /// Gets the block at the specified position
    pub async fn get_block_at_pos(
        &mut self,
        pos: Vec3d<i32>,
        load_if_needed: bool,
    ) -> ActorMessagingResult<Option<Block>> {
        let (send, recv) = oneshot::channel();
        self.send_raw_message(ActorMessage::Other(WorldMessage::GetBlockAtPos(
            pos,
            send,
            load_if_needed,
        )))
        .await?;
        Ok(recv.await?)
    }
    /// Gets the block at the specified position
    pub async fn set_block_at_pos(
        &mut self,
        pos: Vec3d<i32>,
        block: Block,
    ) -> ActorMessagingResult {
        self.send_raw_message(ActorMessage::Other(WorldMessage::SetBlockAtPos(pos, block)))
            .await
    }
    /// Gets the spawn position of this world.
    pub async fn get_spawn_position(&mut self) -> ActorMessagingResult<Vec3d<f64>> {
        let (send, recv) = oneshot::channel();
        self.send_raw_message(ActorMessage::Other(WorldMessage::GetSpawnPosition(send)))
            .await?;
        Ok(recv.await?)
    }
    /// Spawns a new online player entity.
    /// The entity id does not matter if generate_id
    /// is set to `true` because it is then going to
    /// be overridden with a new unique id generated
    /// by the universe.
    ///
    /// This function is also going to handle the process
    /// of telling the connected client about the world by
    /// sending the following packets:
    ///
    /// - PlayerPositionAndLook
    /// - UpdateViewPosition
    /// - UpdateLight
    /// - ChunkData
    /// - WorldBorder
    /// - SpawnPosition
    /// - Player Position And Look
    pub async fn spawn_entity_player_online(
        &mut self,
        connection: PlayerConnectionPacketHandle,
        entity: entity::player::EntityPlayer,
        generate_id: bool,
    ) -> ActorMessagingResult<entity::player::online_controller::ControllerHandle> {
        let (send, recv) = oneshot::channel();
        self.send_raw_message(ActorMessage::Other(WorldMessage::SpawnEntityPlayerOnline {
            connection,
            entity,
            generate_id,
            callback: send,
        }))
        .await?;
        println!("[world.rs] DEBUG: sent SpawnEntityPlayerOnline");
        let r = recv.await?;
        println!("[world.rs] DEBUG: SpawnEntityPlayerOnline callback");
        Ok(r)
    }
    async fn insert_entity_player_online(
        &mut self,
        handles: EntityPlayerOnlineHandleTuple,
    ) -> ActorMessagingResult {
        self.send_raw_message(ActorMessage::Other(
            WorldMessage::PrivateInsertEntityPlayerOnline(handles),
        ))
        .await
    }
    async fn entity_actor_stopped(&mut self, id: u32) -> ActorMessagingResult {
        self.send_raw_message(ActorMessage::Other(
            WorldMessage::PrivateEntityActorStopped { id },
        ))
        .await
    }
    /// This is a two-step async function. The first future sends the request
    /// and the second future awaits the response.
    ///
    /// This function should only be used internally by the online player actor.
    /// Sending this message for a player actor will result in an invalid state
    /// because the region thinks that the player is subscribed, but the player
    /// is not aware of this. This means that if the player leaves, it does not
    /// unsubscribe from the region.
    pub async fn player_subscribe_chunk(
        &mut self,
        chunk: ChunkPosition,
        player_id: u32,
        connection: PlayerConnectionPacketHandle,
        send_complete: bool,
    ) -> ActorMessagingResult<impl std::future::Future<Output = ActorMessagingResult<RegionHandle>>>
    {
        use futures::FutureExt;
        let (send, recv) = oneshot::channel();
        self.send_raw_message(ActorMessage::Other(WorldMessage::PlayerSubscribeChunk {
            chunk,
            player_id,
            connection,
            send_complete,
            callback: send,
        }))
        .await?;
        Ok(recv.map(|r| r.map_err(|e| e.into())))
    }
}

impl fmt::Display for BlockWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockWorld '{}'", self.id)
    }
}
