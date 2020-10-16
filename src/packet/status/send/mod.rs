use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, PacketSerialOut};
use uuid::Uuid;

#[derive(Debug)]
/// # Response (Server List Ping)
/// [Documentation](https://wiki.vg/Server_List_Ping)
pub struct Response<'a> {
    pub status: ServerStatus<'a>,
}

impl Response<'_> {
    pub fn new() -> Self {
        Self {
            status: ServerStatus::new(),
        }
    }
}

#[derive(Debug)]
pub struct ServerStatus<'a> {
    pub version_name: &'static str,
    pub protocol_version: u32,
    pub max_players: u32,
    pub online_players: u32,
    pub sample: &'a [(&'a str, Uuid)],
    pub description: &'a [ChatComponent],
    pub favicon: Option<&'a str>,
}

impl ServerStatus<'_> {
    pub fn new() -> Self {
        Self {
            version_name: "1.15.2",
            protocol_version: 578,
            max_players: 20,
            online_players: 0,
            sample: &[],
            description: &[],
            favicon: None,
        }
    }

    /// Creates a json string with the given information
    pub fn make_json(&self) -> serde_json::Value {
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
                        name: name.to_string(),
                        id: uuid.to_hyphenated().to_string(),
                    })
                    .collect(),
            },
            description: Value::Array(
                self.description
                    .iter()
                    .map(|comp| comp.make_json())
                    .collect(),
            ),
            other: Map::with_capacity(1),
        };
        if let Some(favicon) = self.favicon {
            obj.other
                .insert("favicon".to_owned(), Value::String(favicon.to_owned()));
        }
        return serde_json::to_value(&obj).unwrap();
    }
    /// Function optimized for serialiting a server status to
    /// a buffer by minimizing memory allocation.
    pub fn serialize_json(&self, buffer: &mut Vec<u8>) {
        use crate::helpers::fast::json::escape_write;
        buffer.extend_from_slice(br#"{"version":{"name":""#);
        escape_write(self.version_name, buffer);
        buffer.extend_from_slice(br#"","protocol":"#);
        buffer.extend(self.protocol_version.to_string().as_bytes());
        buffer.extend_from_slice(br#"},"players":{"max":"#);
        buffer.extend(self.max_players.to_string().as_bytes());
        buffer.extend_from_slice(br#","online":"#);
        buffer.extend(self.online_players.to_string().as_bytes());
        buffer.extend_from_slice(br#","sample":["#);
        {
            let mut iter = self.sample.iter();
            if let Some(i) = iter.next() {
                serialize_sample(&i.0, &i.1, buffer);
                for i in iter {
                    buffer.push(b',');
                    serialize_sample(&i.0, &i.1, buffer);
                }
            }
            #[inline]
            fn serialize_sample(name: &str, uuid: &Uuid, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(br#"{"name":""#);
                escape_write(name, buffer);
                buffer.extend_from_slice(br#"","id":""#);
                escape_write(&uuid.to_hyphenated().to_string(), buffer);
                buffer.extend_from_slice(br#""}"#);
            }
        }
        buffer.extend_from_slice(br#"]},"description":"#);
        if self.description.len() != 1 {
            buffer.push(b'[');
            let mut iter = self.description.iter();
            if let Some(i) = iter.next() {
                i.serialize_json(buffer);
                for i in iter {
                    buffer.push(b',');
                    i.serialize_json(buffer);
                }
            }
            buffer.push(b']');
        } else {
            self.description[0].serialize_json(buffer);
        }
        if let Some(favicon) = self.favicon {
            buffer.extend_from_slice(br#","favicon":"data:image/png;base64,"#);
            buffer.extend_from_slice(favicon.as_bytes());
            buffer.push(b'"');
        }
        buffer.push(b'}');
    }
}

mod json {
    use serde::Serialize;
    use serde_json::{Map, Value};
    #[derive(Serialize)]
    pub struct ServerStatusRoot {
        pub version: ServerStatusVersion,
        pub players: ServerStatusPlayers,
        pub description: serde_json::Value,
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

impl PacketSerialOut for Response<'_> {
    const ID: u32 = 0x00;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let mut json_string = Vec::with_capacity(200);
        self.status.serialize_json(&mut json_string);
        write::raw(buffer, &json_string);
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
