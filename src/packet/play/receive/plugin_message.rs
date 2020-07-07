use crate::helpers::NamespacedKey;
use crate::packet::{data::read, PacketParsingError, PacketSerialIn};

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
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
        use std::convert::TryInto;
        Ok(Self {
            channel: match read::string(buffer)?.try_into() {
                Ok(ch) => ch,
                Err(err) => return Err(PacketParsingError::InvalidPacket(err.to_string())),
            },
            data: {
                let len = read::var_u32(buffer)? as usize;
                read::byte_vec(buffer, len)?
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
