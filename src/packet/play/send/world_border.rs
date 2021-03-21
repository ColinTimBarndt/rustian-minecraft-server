use crate::packet::{data::write, packet_ids::PLAY_CB_WORLD_BORDER, PacketSerialOut};

/// # World Border
/// [Documentation](https://wiki.vg/Protocol#World_Border)
#[derive(Debug, Clone, Copy)]
pub enum WorldBorder {
  SetSize(f64),
  LerpSize(WorldBorderLerp),
  SetCenter(f64, f64),
  Initialize {
    position: (f64, f64),
    lerp: WorldBorderLerp,
    teleport_boundary: u32,
    warning_time: i32,
    warning_blocks: i32,
  },
  SetWarningTime(i32),
  SetWarningBlocks(i32),
}

#[derive(Debug, Clone, Copy)]
pub struct WorldBorderLerp {
  pub from: f64,
  pub to: f64,
  pub speed: u64,
}

impl PacketSerialOut for WorldBorder {
  const ID: u32 = PLAY_CB_WORLD_BORDER;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    self.consume_write(buffer)
  }
  fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
    match self {
      Self::SetSize(d) => {
        write::var_u8(buffer, 0);
        write::f64(buffer, d);
      }
      Self::LerpSize(lerp) => {
        write::var_u8(buffer, 1);
        lerp.consume_write(buffer);
      }
      Self::SetCenter(x, z) => {
        write::var_u8(buffer, 2);
        write::f64(buffer, x);
        write::f64(buffer, z);
      }
      Self::Initialize {
        position: (x, z),
        lerp,
        teleport_boundary,
        warning_time,
        warning_blocks,
      } => {
        write::var_u8(buffer, 3);
        write::f64(buffer, x);
        write::f64(buffer, z);
        lerp.consume_write(buffer);
        write::var_u32(buffer, teleport_boundary);
        write::var_i32(buffer, warning_time);
        write::var_i32(buffer, warning_blocks);
      }
      Self::SetWarningTime(time) => {
        write::var_u8(buffer, 4);
        write::var_i32(buffer, time);
      }
      Self::SetWarningBlocks(distance) => {
        write::var_u8(buffer, 5);
        write::var_i32(buffer, distance);
      }
    }
    Ok(())
  }
}

impl WorldBorder {
  pub const DEFAULT_TELEPORT_BOUNDARY: u32 = 29999984;
}

impl WorldBorderLerp {
  fn consume_write(self, buffer: &mut Vec<u8>) {
    write::f64(buffer, self.from);
    write::f64(buffer, self.to);
    write::var_u64(buffer, self.speed);
  }
}
