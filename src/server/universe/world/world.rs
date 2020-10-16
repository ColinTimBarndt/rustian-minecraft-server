use super::chunk_generator::ChunkGenerator;
use super::chunk_loader::ChunkLoader;
use super::{entity_list::EntityListEntry, region::*, Block, EntityList};
use crate::actor_model::*;
use crate::helpers::{NamespacedKey, Vec3d};
use crate::packet::PlayerConnectionPacketHandle;
use crate::server::universe::{entity, UniverseHandle};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokio::sync::{mpsc, oneshot};

pub struct BlockWorld {
  regions: HashMap<RegionPosition, RegionHandle>,
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
    let region = self.regions.get_mut(&region_position);
    // Is the region of the chunk loaded?
    if let Some(loaded_region) = region {
      let chunk_loaded = self
        .loaded_chunks
        .contains(&super::ChunkPosition::from(pos));
      // Is the chunk in the region loaded?
      if !chunk_loaded {
        // Load or generate the chunk
        if load {
          let chk_pos = super::ChunkPosition::from(pos);
          let chunk = if let Some(loaded_chk) = self.loader.load_chunk(chk_pos) {
            loaded_chk
          } else {
            self.generator.generate(chk_pos)
          };
          let block = chunk.get_block_at_pos(Vec3d::new(
            (pos.x % 16) as u8,
            pos.y as u8,
            (pos.z % 16) as u8,
          ));
          loaded_region.set_chunk(chunk).await.unwrap();
          self.loaded_chunks.insert(chk_pos);
          Some(block)
        } else {
          None
        }
      } else {
        loaded_region
          .get_block(Vec3d::new(
            pos.x % super::CHUNK_BLOCK_WIDTH as i32,
            pos.y,
            pos.z % super::CHUNK_BLOCK_WIDTH as i32,
          ))
          .await
          .expect("Unable to get block in region")
          .map(|b| Some(b))
          .unwrap_or(None)
      }
    } else {
      None
    }
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
        // Do this async because this thread has better things to do
        tokio::task::spawn(async move {
          if let Err(e) = (async {
            {
              use crate::packet::play::send::{
                ChunkData, PlayerPositionAndLook, UpdateViewPosition,
              };
              if generate_id {
                if let Ok(id) = universe.reserve_entity_id().await {
                  entity.id = id;
                } else {
                  return Err(format!(
                    "[world.rs] Failed to reserve id for player entity {} ({})",
                    entity.profile.name, entity.profile.uuid
                  ));
                }
              }
              // Send these packets:
              // - UpdateViewPosition
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

              // TODO

              // ✅ PlayerPositionAndLook
              connection
                .send_teleport_packet(PlayerPositionAndLook::create_abs(
                  0,
                  entity.position,
                  entity.head_rotation,
                ))
                .await?
                .await
                .map_err(|_| "Failed to await teleport callback")?;

              println!("Received teleport callback!!11!elf!");

              // Then, spawn player actor
              let controller =
                entity::player::online_controller::Controller::new(connection, entity);
              let (jh, handle) = controller.spawn_actor();
              let cloned_handle = handle.clone();
              world
                .insert_entity_player_online((jh, handle))
                .await
                .expect("[world.rs] Failed to communicate with own world");
              let _ = callback.send(cloned_handle);
              Ok(())
            }
          })
          .await
          {
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
  GetSpawnPosition(oneshot::Sender<Vec3d<f64>>),
  SpawnEntityPlayerOnline {
    connection: PlayerConnectionPacketHandle,
    entity: entity::player::EntityPlayer,
    generate_id: bool,
    callback: oneshot::Sender<entity::player::online_controller::ControllerHandle>,
  },
  PrivateInsertEntityPlayerOnline(EntityPlayerOnlineHandleTuple),
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
  ) -> Result<Option<Block>, ()> {
    let (send, recv) = oneshot::channel();
    match self
      .send_raw_message(ActorMessage::Other(WorldMessage::GetBlockAtPos(
        pos,
        send,
        load_if_needed,
      )))
      .await
    {
      Ok(_) => Ok(recv.await.unwrap()),
      Err(_) => Err(()),
    }
  }
  /// Gets the spawn position of this world.
  pub async fn get_spawn_position(&mut self) -> Result<Vec3d<f64>, ()> {
    let (send, recv) = oneshot::channel();
    match self
      .send_raw_message(ActorMessage::Other(WorldMessage::GetSpawnPosition(send)))
      .await
    {
      Ok(_) => Ok(recv.await.unwrap()),
      Err(_) => Err(()),
    }
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
  ) -> Result<entity::player::online_controller::ControllerHandle, ()> {
    let (send, recv) = oneshot::channel();
    match self
      .send_raw_message(ActorMessage::Other(WorldMessage::SpawnEntityPlayerOnline {
        connection,
        entity,
        generate_id,
        callback: send,
      }))
      .await
    {
      Ok(_) => Ok(recv.await.unwrap()),
      Err(_) => Err(()),
    }
  }
  async fn insert_entity_player_online(
    &mut self,
    handles: EntityPlayerOnlineHandleTuple,
  ) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(
        WorldMessage::PrivateInsertEntityPlayerOnline(handles),
      ))
      .await
      .map_err(|_| ())
  }
}

impl fmt::Display for BlockWorld {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "BlockWorld '{}'", self.id)
  }
}
