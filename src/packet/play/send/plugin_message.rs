use crate::helpers::NamespacedKey;
use crate::packet::{data::write, PacketSerialOut};

/// # Plugin Message (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Plugin_Message_.28clientbound.29)
///
/// Mods and plugins can use this to send their data. Minecraft itself uses several plugin channels.
/// These internal channels are in the minecraft namespace.
///
/// More documentation on this: <http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/>
pub struct PluginMessage<'a> {
    pub channel: NamespacedKey,
    pub data: &'a [u8],
}

impl PacketSerialOut for PluginMessage<'_> {
    const ID: u32 = 0x19;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let channel = self.channel.to_string();
        write::string(buffer, channel);
        write::var_u32(buffer, self.data.len() as u32);
        buffer.extend_from_slice(self.data);
        Ok(())
    }
    fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
        let channel = self.channel.to_string();
        write::string(buffer, channel);
        write::var_u32(buffer, self.data.len() as u32);
        buffer.extend_from_slice(self.data);
        Ok(())
    }
}
