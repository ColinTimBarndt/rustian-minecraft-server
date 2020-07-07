use crate::packet::{data::write, PacketSerialOut};
use json::{object, JsonValue};
use uuid::Uuid;

#[derive(Debug)]
pub struct Response {
    pub status: ServerStatus,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: ServerStatus::new(),
        }
    }
}

#[derive(Debug)]
pub struct ServerStatus {
    pub version_name: String,
    pub protocol_version: u32,
    pub max_players: u32,
    pub online_players: u32,
    pub sample: Vec<(String, Uuid)>,
    pub description: String,
    pub favicon: Option<String>,
}

impl ServerStatus {
    pub fn new() -> Self {
        Self {
            version_name: "1.15.2".to_owned(),
            protocol_version: 578,
            max_players: 20,
            online_players: 0,
            sample: Vec::new(),
            description: format!(
                "{0}A {1}Rust{0}ian Minecraft server",
                "\u{00A7}7", "\u{00A7}c"
            ),
            favicon: None,
        }
    }

    pub fn to_json(&self) -> JsonValue {
        let mut sample = JsonValue::new_array();
        for (name, uuid) in self.sample.iter() {
            match sample.push(object! {
                "name" => name.to_string(),
                "id" => uuid.to_hyphenated().to_string()
            }) {
                Ok(()) => (),
                Err(e) => panic!("Will never happen: {}", e),
            };
        }
        let mut obj = object! {
            "version" => object!{
                "name" => self.version_name.to_string(),
                "protocol" => self.protocol_version
            },
            "players" => object!{
                "max" => self.max_players,
                "online" => self.online_players,
                "sample" => sample
            },
            "description" => object!{
                "text" => self.description.to_string()
            }
        };
        match &self.favicon {
            Some(icon) => match obj.insert("favicon", icon.to_string()) {
                Ok(()) => (),
                Err(e) => panic!("Will never happen: {}", e),
            },
            None => (),
        };
        return obj;
    }
}

#[derive(Debug)]
pub struct Pong {
    pub payload: u64,
}

impl Pong {
    pub fn new(payload: u64) -> Self {
        Self { payload: payload }
    }
}

impl PacketSerialOut for Response {
    const ID: u32 = 0x00;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let json = self.status.to_json();
        write::string(buffer, json.dump());
        //println!("{}", json.pretty(2));
        Ok(())
    }
}

impl PacketSerialOut for Pong {
    const ID: u32 = 0x01;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::u64(buffer, self.payload);
        Ok(())
    }
}
