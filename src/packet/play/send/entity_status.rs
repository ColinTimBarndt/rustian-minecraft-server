use crate::packet::{data::write, PacketSerialOut};

/// # Entity Status
/// [Documentation](https://wiki.vg/Protocol#Entity_Status)
///
/// [Entity statuses](https://wiki.vg/Entity_statuses)
/// generally trigger an animation for an entity.
/// The available statuses vary by the entity's type (and are
/// available to subclasses of that type as well).
#[derive(Clone, Debug)]
pub struct EntityStatus {
  /// Entity id
  pub entity: u32,
  /// Take a look at the module [`status`]
  pub status: u8,
}

impl PacketSerialOut for EntityStatus {
  const ID: u32 = 0x1C;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::u32(buffer, self.entity);
    write::u8(buffer, self.status);
    Ok(())
  }
}

/// https://wiki.vg/Entity_statuses
pub mod status {
  /// **Tipped Arrow**\
  /// Spawns tipped arrow particle effects, if the color is not -1.
  pub const ARROW_POTION_PARTICLES: u8 = 0;

  /// **Rabbit**\
  /// Causes the rabbit to use its rotated jumping animation, and displays jumping particles.
  pub const RABBIT_JUMPS: u8 = 1;

  /// **Minecart Spawner**\
  /// Resets the delay of the spawner to 200 ticks (the default minimum value).
  pub const MINECART_SPAWNER_RESET_DELAY: u8 = 1;

  /// **Living Entity**\
  /// Plays the hurt animation and hurt sound
  pub const LIVING_ENTITY_HURT: u8 = 2;

  /// **Thrown Snowball**\
  /// Displays 8 `snowballpoof` particles at the snowball's location
  ///
  /// **Thrown Egg**\
  /// Displays 8 `iconcrack` particles with the egg as a parameter at the egg's location
  pub const PROJECTILE_CRACK: u8 = 3;

  /// **Living Entity**\
  /// Plays the death sound and death animation
  pub const LIVING_ENTITY_DEATH: u8 = 3;

  /// **Iron Golem**\
  /// Plays attack animation and attack sound
  ///
  /// **Evocation Fangs**\
  /// Starts the attack animation, and plays the `entity.evocation_fangs.attack` sound.
  ///
  /// **Ravager**\
  /// Starts the attack animation.
  pub const ENTITY_ATTACK: u8 = 4;

  // 5 is unused

  /// **Ridable Animal (AbstractHorse)**\
  /// Spawn smoke particles (taming failed)
  ///
  /// **Tameable Animal**\
  /// Spawn smoke particles (taming failed)
  pub const TAMING_FAILED: u8 = 6;

  /// **Ridable Animal (AbstractHorse)**\
  /// Spawn heart particles (taming succeeded)
  ///
  /// **Tameable Animal**\
  /// Spawn heart particles (taming succeeded)
  pub const TAMING_SUCCEEDED: u8 = 7;

  /// **Wolf**\
  /// Play wolf shaking water animation
  pub const SHAKING_WATER_OFF: u8 = 8;

  /// **Player**\
  /// Marks item use as finished (finished eating, finished drinking, etc)
  ///
  /// This status is not required if you want more control on the server side,
  /// this basicly finishes the interaction on the client side to decrease the
  /// ood quantity, arrow quantity, ...
  ///
  /// But you can trigger these changes manually through other packets or prevent
  /// those changes on the client.
  ///
  /// Examples:
  /// * Create a 'Infinity Bow' without the first arrow in your inventory
  ///   constantly changing in quantity.
  /// * Create a custom found that is infinite, and prevent the stack quantity
  ///   from descreasing.
  /// * ...
  ///
  /// Note: This works together with the 'Hand data' so this will have to be
  /// updated accordingly to 'finish' the interaction manually.
  pub const PLAYER_FINISHED_USE: u8 = 9;

  /// **Sheep**\
  /// Causes the sheep to play the eating grass animation for the next 40 ticks
  pub const SHEEP_EATS_GRASS: u8 = 10;

  /// **Minecart TNT**
  /// Causes the TNT to ignite. Does not play a sound; the sound must be played separately.
  pub const MINECART_TNT_IGNITION: u8 = 10;

  /// **Iron Golem**\
  /// Causes golem to hold out a ~~rose~~ poppy for 400 ticks (20 seconds)
  pub const IRON_GOLEM_HOLDS_FLOWER: u8 = 11;

  /// **Villager**\
  /// Spawn villager mating heart particles
  pub const VILLAGER_LOVE_MOVE: u8 = 12;

  /// **Villager**\
  /// Spawn villager angry particles
  pub const VILLAGER_ANGRY: u8 = 13;

  /// **Villager**\
  /// Spawn villager happy particles
  pub const VILLAGER_HAPPY: u8 = 14;

  /// **Witch**\
  /// Spawns between 10 and 45 witchMagic particles.
  /// This status has a .075% chance of happening each tick.
  pub const WITCH_PERFORMS_MAGIC: u8 = 15;

  /// **Zombie Villager**\
  /// Plays the zombie cure finished sound effect (unless the entity is silent)
  pub const ZOMBIE_VILLAGER_CURED: u8 = 16;

  /// **Fireworks**
  /// Triggers the firework explosion effect (based off of the firework info metadata)
  pub const FIREWORK_EXPLODES: u8 = 17;

  /// **Animal**\
  /// Spawn "love mode" heart particles
  pub const ANIMAL_LOVE_MODE: u8 = 18;

  /// **Squid**\
  /// Resets the squid's rotation to 0 radians. Occurs whenever the server
  /// calculates that the squid has rotated more than 2 pi radians.
  pub const SQUID_RESET_ROTATION: u8 = 19;

  /// **Insentient Entity**\
  /// Spawn explosion particle. Used when:
  /// * A silverfish enters a block
  /// * A silverfish exits a block
  /// * A mob spawner (or minecart mob spawner) spawns an
  ///   entity (only with entities that support this status)
  pub const INSENTIENT_ENTITY_SMOKE: u8 = 20;

  /// **Guardian**\
  /// Plays the guardian attack sound effect from this entity.
  pub const GUARDIAN_ATTACK: u8 = 21;

  /// **Player**\
  /// Enables reduced [debug screen](http://minecraft.gamepedia.com/Debug_screen) information
  pub const ENABLE_REDUCED_DEBUG_SCREEN_INFORMATION: u8 = 22;

  /// **Player**\
  /// Disables reduced [debug screen](http://minecraft.gamepedia.com/Debug_screen) information
  pub const DISABLE_REDUCED_DEBUG_SCREEN_INFORMATION: u8 = 23;

  /// **Player**\
  /// Set [op permission level](http://minecraft.gamepedia.com/Server.properties%23op-permission-level) to 0
  pub const SET_OP_PERMISSION_0: u8 = 24;

  /// **Player**\
  /// Set [op permission level](http://minecraft.gamepedia.com/Server.properties%23op-permission-level) to 1
  pub const SET_OP_PERMISSION_1: u8 = 25;

  /// **Player**\
  /// Set [op permission level](http://minecraft.gamepedia.com/Server.properties%23op-permission-level) to 2
  pub const SET_OP_PERMISSION_2: u8 = 26;

  /// **Player**\
  /// Set [op permission level](http://minecraft.gamepedia.com/Server.properties%23op-permission-level) to 3
  pub const SET_OP_PERMISSION_3: u8 = 27;

  /// **Player**\
  /// Set [op permission level](http://minecraft.gamepedia.com/Server.properties%23op-permission-level) to 4
  pub const SET_OP_PERMISSION_4: u8 = 28;

  /// **Living Entity**\
  /// Plays the shield block sound
  pub const SHIELD_BLOCKED: u8 = 29;

  /// **Living Entity**\
  /// Plays the shield break sound
  pub const SHIELD_BROKEN: u8 = 30;

  /// **Fishing Hook**\
  /// If the caught entity is the connected player, then cause them
  /// to be pulled toward the caster of the fishing rod.
  pub const FISHING_HOOK_PULL_TOWARDS_CASTER: u8 = 31;

  /// **Armor Stand**\
  /// Plays the hit sound, and resets a hit cooldown.
  pub const ARMOR_STAND_HIT: u8 = 32;

  /// **Living Entity**\
  /// Plays the thorns sound in addition to playing the hurt animation and hurt sound
  pub const HURT_BY_THORNS: u8 = 33;

  /// **Iron Golem**\
  /// Puts away golem's poppy
  pub const IRON_GOLEM_PUTS_AWAY_FLOWER: u8 = 34;

  /// **Entity**\
  /// Plays the Totem of Undying animation
  pub const TOTEM_OF_UNDYING_ACTIVATED: u8 = 35;

  /// **Living Entity**\
  /// Plays the hurt animation and drown hurt sound
  pub const HURT_BY_WATER_SUFFOCATION: u8 = 36;

  /// **Living Entity**\
  /// Plays the hurt animation and burn hurt sound
  pub const HURT_BY_FIRE: u8 = 37;

  /// **Dolphin**\
  /// Causes several "happy villager" particles to appear;
  /// used when the dolphin has been fed and is locating a structure
  pub const DOLPHIN_HAPPY: u8 = 38;

  /// **Ravager**\
  /// Marks the ravager as stunned for the next 40 ticks.
  pub const RAVAGER_STUNNED: u8 = 39;

  /// **Cat**\
  /// Spawn smoke particles (taming failed)
  pub const TAMING_CAT_FAILED: u8 = 40;

  /// **Cat**\
  /// Spawn heart particles (taming succeeded)
  pub const TAMING_CAT_SUCCEEDED: u8 = 41;

  /// **Villager**\
  /// Spawn "splash" particles. Triggered with 1% chance each tick while a raid is active.
  pub const VILLAGER_PANIC_SWEAT: u8 = 42;

  /// **Player**\
  /// Spawn cloud particles at the player. Sent to a
  /// player whose Bad Omen effect is removed to either
  /// start a raid or increase its difficulty.
  pub const BAD_OMEN_ACTIVATED: u8 = 43;

  /// **Living Entity**\
  /// Plays the hurt animation and sweet berry bush hurt sound
  pub const HURT_BY_SWEET_BERRY_BUSH: u8 = 44;

  /// **Fox**\
  /// Spawns particles based on the item on the fox's mouth (technically its main hand) to indicate them chewing on it
  pub const CHEWS_ON_ITEM: u8 = 45;

  /// **Living Entity**\
  /// Spawns portal particles when teleporting due to consumption of a chorus fruit or being an endermen
  pub const CHORUS_FRUIT_TELEPORTATION: u8 = 46;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the main hand
  pub const EQUIPMENT_BROKE_IN_MAIN_HAND: u8 = 47;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the off hand
  pub const EQUIPMENT_BROKE_IN_OFF_HAND: u8 = 48;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the head slot
  pub const EQUIPMENT_BROKE_IN_HEAD_SLOT: u8 = 49;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the chest slot
  pub const EQUIPMENT_BROKE_IN_CHEST_SLOT: u8 = 50;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the legs slot
  pub const EQUIPMENT_BROKE_IN_LEGS_SLOT: u8 = 51;

  /// **Living Entity**\
  /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the feet slot
  pub const EQUIPMENT_BROKE_IN_FEET_SLOT: u8 = 52;
}
