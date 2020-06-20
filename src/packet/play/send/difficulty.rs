use crate::packet::{data::write, PacketSerialOut};

/// # Server Difficulty
/// [Documentation](https://wiki.vg/Protocol#Server_Difficulty)
///
/// Changes the difficulty setting in the client's option menu
#[derive(Clone, Debug)]
pub struct Difficulty {
  pub difficulty: crate::server::universe::world::Difficulty,
  pub locked: bool,
}

impl PacketSerialOut for Difficulty {
  const ID: u32 = 0x0E;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::u8(buffer, self.difficulty as u8);
    write::bool(buffer, self.locked);
    Ok(())
  }
}
