use std::error::Error;
use crate::crypto::hash;

// https://wiki.vg/Protocol_Encryption
pub fn digest_server_id(buffer: &mut Vec<u8>, server_id: &String, public_key: &String, secret: &String) -> Result<(), Box<dyn Error>> {
    digest(buffer, &[&server_id, &secret, &public_key])
}

pub fn digest(buffer: &mut Vec<u8>, feed: &[&String]) -> Result<(), Box<dyn Error>> {
    let mut bytes: Vec<&str> = Vec::new();
    for vec in feed {
        bytes.push(vec.as_str());
    }
    for byte in hash::hash(&bytes).as_bytes() {
        buffer.push(*byte);
    }
    Ok(())
}
