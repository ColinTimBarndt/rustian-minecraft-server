use crate::helpers::NamespacedKey;
use crate::packet::{data::read, PacketSerialIn};

pub struct PluginMessage {
    pub channel: NamespacedKey,
    pub data: Vec<u8>,
}

impl PacketSerialIn for PluginMessage {
    const ID: u32 = 0x0B;
    fn consume_read(mut buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        use std::convert::TryInto;
        Ok(Self {
            channel: read::string(&mut buffer)?.try_into()?,
            data: {
                let len = read::var_u32(&mut buffer)? as usize;
                buffer[0..len].into()
            },
        })
    }
}

impl std::convert::From<super::super::send::PluginMessage> for PluginMessage {
    fn from(other: super::super::send::PluginMessage) -> Self {
        Self {
            channel: other.channel,
            data: other.data,
        }
    }
}
