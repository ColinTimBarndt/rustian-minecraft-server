use crate::packet::{data::write, packet_ids::LOGIN_CB_LOGIN_PLUGIN_REQUEST, PacketSerialOut};

/// [Documentation](https://wiki.vg/Protocol#Login_Plugin_Request)
#[derive(Debug, Clone)]
pub struct LoginPluginRequest {
    pub message_identifier: u32,
    pub channel_namespace: String,
    pub data: Option<Vec<u8>>,
}

impl PacketSerialOut for LoginPluginRequest {
    const ID: u32 = LOGIN_CB_LOGIN_PLUGIN_REQUEST;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::var_u32(buffer, self.message_identifier);
        write::string(buffer, &self.channel_namespace);
        match &self.data {
            Some(data) => {
                write::raw(buffer, data);
            }
            None => (),
        }
        Ok(())
    }
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::var_u32(buffer, self.message_identifier);
        write::string(buffer, &self.channel_namespace);
        match self.data {
            Some(mut data) => {
                buffer.append(&mut data);
            }
            None => (),
        }
        Ok(())
    }
}
