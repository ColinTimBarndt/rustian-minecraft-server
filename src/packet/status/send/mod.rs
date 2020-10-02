use crate::packet::{data::write, PacketSerialOut};
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
    pub version_name: &'static str,
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
            version_name: "1.15.2",
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

    /// Creates a json string with the given information
    pub fn to_json(&self) -> String {
        use serde_json::{Map, Value};
        let mut obj = json::ServerStatusRoot {
            version: json::ServerStatusVersion {
                name: self.version_name,
                protocol: self.protocol_version,
            },
            players: json::ServerStatusPlayers {
                max: self.max_players,
                online: self.online_players,
                sample: self
                    .sample
                    .iter()
                    .map(|(name, uuid)| json::ServerStatusPlayersSample {
                        name: name.clone(),
                        id: uuid.to_hyphenated().to_string(),
                    })
                    .collect(),
            },
            description: json::ServerStatusDescription {
                text: self.description.clone(),
            },
            other: Map::with_capacity(1),
        };
        if let Some(favicon) = &self.favicon {
            obj.other
                .insert("favicon".to_owned(), Value::String(favicon.clone()));
        }
        return serde_json::ser::to_string(&obj).unwrap();
    }
}

mod json {
    use serde::Serialize;
    use serde_json::{Map, Value};
    #[derive(Serialize)]
    pub struct ServerStatusRoot {
        pub version: ServerStatusVersion,
        pub players: ServerStatusPlayers,
        pub description: ServerStatusDescription,
        #[serde(flatten)]
        pub other: Map<String, Value>,
    }

    #[derive(Serialize)]
    pub struct ServerStatusVersion {
        pub name: &'static str,
        pub protocol: u32,
    }

    #[derive(Serialize)]
    pub struct ServerStatusPlayers {
        pub max: u32,
        pub online: u32,
        pub sample: Vec<ServerStatusPlayersSample>,
    }

    #[derive(Serialize)]
    pub struct ServerStatusPlayersSample {
        pub name: String,
        pub id: String,
    }

    #[derive(Serialize)]
    pub struct ServerStatusDescription {
        pub text: String,
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
        let json_string = self.status.to_json();
        write::string(buffer, &json_string);
        println!("Status JSON: {:#?}", json_string);
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
