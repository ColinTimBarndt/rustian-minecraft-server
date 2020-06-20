use crate::helpers::NamespacedKey;
use crate::packet::{data::read, PacketSerialIn};

/// # Plugin Message (serverbound)
/// [Documentation](https://wiki.vg/Protocol#Plugin_Message_.28serverbound.29)
///
/// Mods and plugins can use this to send their data. Minecraft itself uses some plugin channels.
/// These internal channels are in the minecraft namespace.
///
/// More documentation on this: http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/
///
/// Note that the length of Data is known only from the packet length, since the packet has no
/// length field of any kind.
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
