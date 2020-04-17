use crate::packet::PlayerConnection;
use super::entity::EntityPlayer;
use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    player_connection: PlayerConnection,
    name: String,
    uuid: Uuid,
    pub locale: String,
    pub chat_mode: PlayerChatMode,
    pub chat_colors_enabled: bool,
    pub displayed_model_parts: DisplayedPlayerModelParts,
    pub main_hand: PlayerHand,
    pub player_entity: Option<u32>
}

#[derive(Debug, Clone, num_derive::FromPrimitive)]
pub enum PlayerChatMode {
    Enabled = 0, CommandsOnly = 1, Disabled = 2
}

#[derive(Debug, Clone, num_derive::FromPrimitive)]
pub enum PlayerHand {
    Left = 0, Right = 1
}

#[derive(Debug, Clone)]
pub struct DisplayedPlayerModelParts(u8);

impl Player {
    pub fn new(player_connection: PlayerConnection, name: String, uuid: Uuid) -> Self {
        Self {
            player_connection,
            name,
            uuid,
            locale: String::from("en_US"),
            chat_mode: PlayerChatMode::Enabled,
            chat_colors_enabled: true,
            displayed_model_parts: DisplayedPlayerModelParts::DISPLAY_ALL,
            main_hand: PlayerHand::Right,
            player_entity: None
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl DisplayedPlayerModelParts {
    pub const DISPLAY_ALL: DisplayedPlayerModelParts = DisplayedPlayerModelParts(0b01111111);
    const CAPE: u8            = 0b00000001;
    const JACKET: u8          = 0b00000010;
    const LEFT_SLEEVE: u8     = 0b00000100;
    const RIGHT_SLEEVE: u8    = 0b00001000;
    const LEFT_PANTS_LEG: u8  = 0b00010000;
    const RIGHT_PANTS_LEG: u8 = 0b00100000;
    const HAT: u8             = 0b01000000;

    pub fn new(byte: u8) -> Self {Self(byte)}
    pub fn to_inner(self) -> u8 {self.0}

    pub fn get_cape(&self)            -> bool {(Self::CAPE & self.0)>0}
    pub fn get_jacket(&self)          -> bool {(Self::JACKET & self.0)>0}
    pub fn get_left_sleeve(&self)     -> bool {(Self::LEFT_SLEEVE & self.0)>0}
    pub fn get_right_sleeve(&self)    -> bool {(Self::RIGHT_SLEEVE & self.0)>0}
    pub fn get_left_pants_leg(&self)  -> bool {(Self::LEFT_PANTS_LEG & self.0)>0}
    pub fn get_right_pants_leg(&self) -> bool {(Self::RIGHT_PANTS_LEG & self.0)>0}
    pub fn get_hat(&self)             -> bool {(Self::HAT & self.0)>0}
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

        if self.get_cape() {displayed.push("cape")}
        else {hidden.push("cape")};
        if self.get_jacket() {displayed.push("jacket")}
        else {hidden.push("jacket")};
        if self.get_left_sleeve() {displayed.push("left_sleeve")}
        else {hidden.push("left_sleeve")};
        if self.get_right_sleeve() {displayed.push("right_sleeve")}
        else {hidden.push("right_sleeve")};
        if self.get_left_pants_leg() {displayed.push("left_pants_leg")}
        else {hidden.push("left_pants_leg")};
        if self.get_right_pants_leg() {displayed.push("right_pants_leg")}
        else {hidden.push("right_pants_leg")};
        if self.get_hat() {displayed.push("hat")}
        else {hidden.push("hat")};

        write!(f, "Displayed=[{}], Hidden=[{}]", displayed.join(", "), hidden.join(", "))
    }
}

impl std::fmt::Display for PlayerChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PlayerChatMode::*;
        write!(f, "{}", match self {
            Enabled => "Enabled",
            CommandsOnly => "Commands Only",
            Disabled => "Disabled"
        })
    }
}

impl std::fmt::Display for PlayerHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PlayerHand::*;
        write!(f, "{}", match self {
            Left => "Left",
            Right => "Right"
        })
    }
}