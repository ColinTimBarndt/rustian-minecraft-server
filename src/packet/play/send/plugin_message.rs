use crate::helpers::NamespacedKey;
use crate::packet::{data::write, PacketSerialOut};

pub struct PluginMessage {
    pub channel: NamespacedKey,
    pub data: Vec<u8>,
}

impl PacketSerialOut for PluginMessage {
    const ID: u32 = 0x19;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let channel = format!("{}", self.channel);
        write::string(buffer, channel);
        write::var_u32(buffer, self.data.len() as u32);
        buffer.append(&mut self.data.clone());
        Ok(())
    }
    fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let channel = format!("{}", self.channel);
        write::string(buffer, channel);
        write::var_u32(buffer, self.data.len() as u32);
        buffer.append(&mut self.data);
        Ok(())
    }
}

impl std::convert::From<super::super::receive::PluginMessage> for PluginMessage {
    fn from(other: super::super::receive::PluginMessage) -> Self {
        Self {
            channel: other.channel,
            data: other.data,
        }
    }
}
