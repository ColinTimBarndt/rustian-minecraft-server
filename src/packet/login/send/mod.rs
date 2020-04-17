use crate::packet::{PacketSerialOut,data::write};
use json::{JsonValue, object};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Disconnect {
    pub reason: JsonValue
}

impl From<String> for Disconnect {
    fn from(msg: String) -> Self {
        Self {
            reason: object!{
                "text" => msg
            }
        }
    }
}

impl From<&str> for Disconnect {
    fn from(msg: &str) -> Self {
        Self {
            reason: object!{
                "text" => msg
            }
        }
    }
}

impl From<JsonValue> for Disconnect {
    fn from(msg: JsonValue) -> Self {
        Self {
            reason: msg
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionRequest {
    pub server_identifier: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>
}

impl EncryptionRequest {
    pub fn new(sid: String, p_key: Vec<u8>, verify_sec: u8) -> Self {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut v_token = Vec::with_capacity((verify_sec*4) as usize);
        for _ in 0..verify_sec {
            let n = rng.next_u64();
            v_token.push(n as u8);
            v_token.push((n>>8) as u8);
            v_token.push((n>>16) as u8);
            v_token.push((n>>24) as u8);
        }
        Self {
            server_identifier: sid,
            public_key: p_key,
            verify_token: v_token
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String
}

#[derive(Debug, Clone)]
pub struct SetCompression {
    pub threshold: u32
}

#[derive(Debug, Clone)]
pub struct LoginPluginRequest {
    pub message_identifier: u32,
    pub channel_namespace: String,
    pub data: Option<Vec<u8>>
}

impl PacketSerialOut for Disconnect {
    const ID: u32 = 0x00;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::json(buffer, &self.reason);
        Ok(())
    }
}

impl PacketSerialOut for EncryptionRequest {
    const ID: u32 = 0x01;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        (*self).clone().consume_write(buffer)
    }
    fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, self.server_identifier);

        write::var_u32(buffer, self.public_key.len() as u32);
        buffer.append(&mut self.public_key);

        write::var_u32(buffer, self.verify_token.len() as u32);
        buffer.append(&mut self.verify_token);

        Ok(())
    }
}

impl PacketSerialOut for LoginSuccess {
    const ID: u32 = 0x02;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, self.uuid.hyphenated().to_string());
        write::string(buffer, self.username.clone());
        Ok(())
    }
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, self.uuid.hyphenated().to_string());
        write::string(buffer, self.username);
        Ok(())
    }
}

impl PacketSerialOut for SetCompression {
    const ID: u32 = 0x03;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        (*self).clone().consume_write(buffer)
    }
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
        if self.threshold > 0x7f_ff_ff_ff {
            panic!("Compression threshold too big for the serializer");
        }
        write::var_u32(buffer, self.threshold);
        Ok(())
    }
}

impl PacketSerialOut for LoginPluginRequest {
    const ID: u32 = 0x04;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::var_u32(buffer, self.message_identifier.clone());
        write::string(buffer, self.channel_namespace.clone());
        match &self.data {
            Some(data) => {
                buffer.append(&mut data.clone());
            },
            None => ()
        }
        Ok(())
    }
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String>  {
        write::var_u32(buffer, self.message_identifier);
        write::string(buffer, self.channel_namespace);
        match self.data {
            Some(mut data) => {
                buffer.append(&mut data);
            },
            None => ()
        }
        Ok(())
    }
}
