use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::entity::player::game_profile::{GameProfile, GameProfileProperty};
use crate::server::universe::Gamemode;

#[derive(Debug)]
pub enum PlayerInfo<'a, 'b> {
  AddPlayer(Vec<PlayerInfoAddPlayerEntry<'a, 'b>>),
  UpdateGamemode(Vec<(uuid::Uuid, Gamemode)>),
  UpdateLatency(Vec<(uuid::Uuid, i32)>),
  UpdateDisplayName(Vec<(uuid::Uuid, Option<&'b [ChatComponent]>)>),
  RemovePlayer(Vec<uuid::Uuid>),
}

/// # Player Info
/// [Documentation](https://wiki.vg/Protocol#Player_Info)
///
/// Sent by the server to update the user list (<tab> in the client).
#[derive(Debug)]
pub struct PlayerInfoAddPlayerEntry<'a, 'b> {
  pub profile: &'a GameProfile,
  pub gamemode: Gamemode,
  /// Ping in ms. Corresponds to a connection icon on the client side:
  /// - A ping that negative (i.e. not known to the server yet) will result in the no connection icon.
  /// - A ping under 150 milliseconds will result in 5 bars
  /// - A ping under 300 milliseconds will result in 4 bars
  /// - A ping under 600 milliseconds will result in 3 bars
  /// - A ping under 1000 milliseconds (1 second) will result in 2 bars
  /// - A ping greater than or equal to 1 second will result in 1 bar.
  pub ping: i32,
  pub display_name: Option<&'b [ChatComponent]>,
}

impl PacketSerialOut for PlayerInfo<'_, '_> {
  const ID: u32 = 0x34;

  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    match self {
      Self::AddPlayer(entries) => {
        write::var_u8(buffer, 0);
        write::var_usize(buffer, entries.len());
        for entry in entries {
          write::uuid(buffer, entry.profile.uuid);
          write::string(buffer, &entry.profile.name);
          // Properties
          for (_, property) in &entry.profile.properties {
            write::string(buffer, &property.name);
            write::string(buffer, &property.value);
            if let Some(signature) = &property.signature {
              write::bool(buffer, true);
              write::string(buffer, signature);
            } else {
              write::bool(buffer, false);
            }
          }
          //
          write::var_u8(buffer, entry.gamemode as u8);
          write::var_i32(buffer, entry.ping);
          if let Some(display_name) = &entry.display_name {
            write::bool(buffer, true);
            write::chat_components(buffer, display_name);
          } else {
            write::bool(buffer, false);
          }
        }
      }
      Self::UpdateGamemode(entries) => {
        write::var_u8(buffer, 1);
        write::var_usize(buffer, entries.len());
        for (uuid, gamemode) in entries {
          write::uuid(buffer, *uuid);
          write::var_u8(buffer, *gamemode as u8);
        }
      }
      Self::UpdateLatency(entries) => {
        write::var_u8(buffer, 2);
        write::var_usize(buffer, entries.len());
        for (uuid, ping) in entries {
          write::uuid(buffer, *uuid);
          write::var_i32(buffer, *ping);
        }
      }
      Self::UpdateDisplayName(entries) => {
        write::var_u8(buffer, 3);
        write::var_usize(buffer, entries.len());
        for (uuid, display_name) in entries {
          write::uuid(buffer, *uuid);
          if let Some(display_name) = display_name {
            write::bool(buffer, true);
            write::chat_components(buffer, display_name);
          } else {
            write::bool(buffer, false);
          }
        }
      }
      Self::RemovePlayer(entries) => {
        write::var_u8(buffer, 4);
        write::var_usize(buffer, entries.len());
        for uuid in entries {
          write::uuid(buffer, *uuid);
        }
      }
    }
    Ok(())
  }
}
