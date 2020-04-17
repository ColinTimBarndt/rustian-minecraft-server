use crate::packet::{
    PacketSerialOut, data::write
};
use crate::helpers::NamespacedKey;

pub struct PluginMessage {
    pub channel: NamespacedKey,
    pub data: Vec<u8>
}

impl PacketSerialOut for PluginMessage {
    const ID: u32 = 0x19;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, format!("{}", self.channel));
        buffer.append(&mut self.data.clone());
        Ok(())
    }
    fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, format!("{}", self.channel));
        buffer.append(&mut self.data);
        Ok(())
    }
}

impl std::convert::From<super::super::receive::PluginMessage> for PluginMessage {
    fn from(other: super::super::receive::PluginMessage) -> Self {
        Self {
            channel: other.channel,
            data: other.data
        }
    }
}