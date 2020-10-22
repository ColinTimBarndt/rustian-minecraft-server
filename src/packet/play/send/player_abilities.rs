use crate::packet::{data::write, packet_ids::PLAY_CB_PLAYER_ABILITIES, PacketSerialOut};

/// # Player Abilities (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Player_Abilities_.28clientbound.29)
///
/// The latter 2 floats are used to indicate the field of view and flying speed
/// respectively, while the first byte is used to determine the value of 4 booleans.
#[derive(Clone, Debug)]
pub struct PlayerAbilities {
  pub invulnerable: bool,
  pub flying: bool,
  pub allow_flying: bool,
  pub break_blocks_instantly: bool,
  pub flying_speed: f32,
  pub field_of_view_modifier: f32,
}

impl PacketSerialOut for PlayerAbilities {
  const ID: u32 = PLAY_CB_PLAYER_ABILITIES;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    let bits: u8 = (self.invulnerable as u8)
      | ((self.flying as u8) << 1)
      | ((self.allow_flying as u8) << 2)
      | ((self.break_blocks_instantly as u8) << 3);
    write::u8(buffer, bits);
    write::f32(buffer, self.flying_speed);
    write::f32(buffer, self.field_of_view_modifier);
    Ok(())
  }
}

impl std::default::Default for PlayerAbilities {
  fn default() -> Self {
    Self {
      invulnerable: false,
      flying: false,
      allow_flying: false,
      break_blocks_instantly: false,
      flying_speed: 0.05,
      field_of_view_modifier: 0.1,
    }
  }
}
