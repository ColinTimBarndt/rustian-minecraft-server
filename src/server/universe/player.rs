use crate::packet::PlayerConnectionPacketHandle;
use std::default::Default;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
#[macro_use]
use crate::send_packet;

pub type SharedPlayer = Arc<RwLock<Player>>;

#[derive(Debug)]
pub struct Player {
    player_connection: PlayerConnectionPacketHandle,
    name: String,
    uuid: Uuid,
    entity_id: u32,
    pub settings: PlayerSettings,
    pub player_entity: Option<u32>,
    selected_hotbar_slot: u8,
}

#[derive(Debug, Clone)]
pub struct PlayerSettings {
    pub locale: String,
    pub view_distance: u16,
    pub chat_mode: PlayerChatMode,
    pub chat_colors_enabled: bool,
    pub displayed_model_parts: DisplayedPlayerModelParts,
    pub main_hand: PlayerHand,
}

#[derive(Debug, Clone, Copy, num_derive::FromPrimitive)]
pub enum PlayerChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Disabled = 2,
}

#[derive(Debug, Clone, Copy, num_derive::FromPrimitive)]
pub enum PlayerHand {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayedPlayerModelParts(u8);

impl Player {
    pub fn new(
        player_connection: PlayerConnectionPacketHandle,
        name: String,
        uuid: Uuid,
        entity_id: u32,
    ) -> Self {
        Self {
            player_connection,
            name,
            uuid,
            entity_id: entity_id,
            settings: Default::default(),
            player_entity: None,
            selected_hotbar_slot: 0,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
    pub fn get_entity_id(&self) -> u32 {
        self.entity_id
    }
    pub fn get_selected_hotbar_slot(&self) -> u8 {
        self.selected_hotbar_slot
    }
    pub async fn set_selected_hotbar_slot(&mut self, slot: u8) -> Result<(), String> {
        let packet = crate::packet::play::send::HeldItemChange { hotbar_slot: slot };
        let con = &mut self.player_connection;
        send_packet!(packet => con.send_packet)?;
        self.selected_hotbar_slot = slot;
        Ok(())
    }
    /// Should only be used when sending the packet manually
    pub fn set_selected_hotbar_slot_server_side(&mut self, slot: u8) {
        self.selected_hotbar_slot = slot;
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player \"{}\" ({})", self.name, self.uuid)
    }
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            locale: "en_US".into(),
            view_distance: 1,
            chat_mode: PlayerChatMode::Enabled,
            chat_colors_enabled: true,
            displayed_model_parts: DisplayedPlayerModelParts::DISPLAY_ALL,
            main_hand: PlayerHand::Right,
        }
    }
}

impl std::fmt::Display for PlayerSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "locale={}, view_distance={}, chat_mode={}, chat_colors={}, displayed_model_parts={}, main_hand={}",
            self.locale,
            self.view_distance,
            self.chat_mode,
            self.chat_colors_enabled,
            self.displayed_model_parts,
            self.main_hand
        )
    }
}

impl DisplayedPlayerModelParts {
    pub const DISPLAY_ALL: DisplayedPlayerModelParts = DisplayedPlayerModelParts(0b01111111);
    const CAPE: u8 = 0b00000001;
    const JACKET: u8 = 0b00000010;
    const LEFT_SLEEVE: u8 = 0b00000100;
    const RIGHT_SLEEVE: u8 = 0b00001000;
    const LEFT_PANTS_LEG: u8 = 0b00010000;
    const RIGHT_PANTS_LEG: u8 = 0b00100000;
    const HAT: u8 = 0b01000000;

    pub fn new(byte: u8) -> Self {
        Self(byte)
    }
    pub fn to_inner(self) -> u8 {
        self.0
    }

    pub fn get_cape(&self) -> bool {
        (Self::CAPE & self.0) > 0
    }
    pub fn get_jacket(&self) -> bool {
        (Self::JACKET & self.0) > 0
    }
    pub fn get_left_sleeve(&self) -> bool {
        (Self::LEFT_SLEEVE & self.0) > 0
    }
    pub fn get_right_sleeve(&self) -> bool {
        (Self::RIGHT_SLEEVE & self.0) > 0
    }
    pub fn get_left_pants_leg(&self) -> bool {
        (Self::LEFT_PANTS_LEG & self.0) > 0
    }
    pub fn get_right_pants_leg(&self) -> bool {
        (Self::RIGHT_PANTS_LEG & self.0) > 0
    }
    pub fn get_hat(&self) -> bool {
        (Self::HAT & self.0) > 0
    }
}

impl Default for DisplayedPlayerModelParts {
    fn default() -> Self {
        Self::DISPLAY_ALL
    }
}

impl std::ops::Deref for DisplayedPlayerModelParts {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

impl std::fmt::Display for DisplayedPlayerModelParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut displayed = Vec::new();
        let mut hidden = Vec::new();

        if self.get_cape() {
            displayed.push("cape")
        } else {
            hidden.push("cape")
        };
        if self.get_jacket() {
            displayed.push("jacket")
        } else {
            hidden.push("jacket")
        };
        if self.get_left_sleeve() {
            displayed.push("left_sleeve")
        } else {
            hidden.push("left_sleeve")
        };
        if self.get_right_sleeve() {
            displayed.push("right_sleeve")
        } else {
            hidden.push("right_sleeve")
        };
        if self.get_left_pants_leg() {
            displayed.push("left_pants_leg")
        } else {
            hidden.push("left_pants_leg")
        };
        if self.get_right_pants_leg() {
            displayed.push("right_pants_leg")
        } else {
            hidden.push("right_pants_leg")
        };
        if self.get_hat() {
            displayed.push("hat")
        } else {
            hidden.push("hat")
        };

        write!(
            f,
            "Displayed=[{}], Hidden=[{}]",
            displayed.join(", "),
            hidden.join(", ")
        )
    }
}

impl std::fmt::Display for PlayerChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PlayerChatMode::*;
        write!(
            f,
            "{}",
            match self {
                Enabled => "Enabled",
                CommandsOnly => "Commands Only",
                Disabled => "Disabled",
            }
        )
    }
}

impl std::fmt::Display for PlayerHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PlayerHand::*;
        write!(
            f,
            "{}",
            match self {
                Left => "Left",
                Right => "Right",
            }
        )
    }
}
