use crate::packet::PacketParsingError;

pub fn finite_f32(f: f32) -> Result<f32, PacketParsingError> {
    if f.is_finite() {
        Ok(f)
    } else {
        Err(PacketParsingError::InvalidPacket(
            "unexpected non-finite floating point value".to_owned(),
        ))
    }
}

pub fn finite_f64(f: f64) -> Result<f64, PacketParsingError> {
    if f.is_finite() {
        Ok(f)
    } else {
        Err(PacketParsingError::InvalidPacket(
            "unexpected non-finite position".to_owned(),
        ))
    }
}

/// Read data from raw packet bytes
///
/// Note: Numbers use the network byte order (big endian)
pub mod read {
    use crate::helpers::Vec3d;
    use crate::packet::*;
    use byteorder::{BigEndian, ReadBytesExt};
    use serde_json::Value as JsonValue;
    use std::error::Error;

    // TODO: Use the vec in reverse order and pop()

    pub fn byte_vec(buffer: &mut &[u8], len: usize) -> Result<Vec<u8>, PacketParsingError> {
        if buffer.len() >= len {
            let vec = buffer[0..len].iter().map(|&x| x).collect();
            *buffer = &buffer[len..];
            Ok(vec)
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn bool(buffer: &mut &[u8]) -> Result<bool, PacketParsingError> {
        if buffer.len() >= 1 {
            Ok(buffer.read_u8().unwrap() > 0)
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn i8(buffer: &mut &[u8]) -> Result<i8, PacketParsingError> {
        if buffer.len() >= 1 {
            Ok(buffer.read_i8().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }
    pub fn u8(buffer: &mut &[u8]) -> Result<u8, PacketParsingError> {
        if buffer.len() >= 1 {
            Ok(buffer.read_u8().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn i16(buffer: &mut &[u8]) -> Result<i16, PacketParsingError> {
        if buffer.len() >= 2 {
            Ok(buffer.read_i16::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }
    pub fn u16(buffer: &mut &[u8]) -> Result<u16, PacketParsingError> {
        if buffer.len() >= 2 {
            Ok(buffer.read_u16::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn i32(buffer: &mut &[u8]) -> Result<i32, PacketParsingError> {
        if buffer.len() >= 4 {
            Ok(buffer.read_i32::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }
    pub fn u32(buffer: &mut &[u8]) -> Result<u32, PacketParsingError> {
        if buffer.len() >= 4 {
            Ok(buffer.read_u32::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn i64(buffer: &mut &[u8]) -> Result<i64, PacketParsingError> {
        if buffer.len() >= 8 {
            Ok(buffer.read_i64::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }
    pub fn u64(buffer: &mut &[u8]) -> Result<u64, PacketParsingError> {
        if buffer.len() >= 8 {
            Ok(buffer.read_u64::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn f32(buffer: &mut &[u8]) -> Result<f32, PacketParsingError> {
        if buffer.len() >= 4 {
            Ok(buffer.read_f32::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }
    pub fn f64(buffer: &mut &[u8]) -> Result<f64, PacketParsingError> {
        if buffer.len() >= 4 {
            Ok(buffer.read_f64::<BigEndian>().unwrap())
        } else {
            Err(PacketParsingError::EndOfInput)
        }
    }

    pub fn string(buffer: &mut &[u8]) -> Result<String, PacketParsingError> {
        let len = var_u32(buffer)? as usize;
        let bytes = byte_vec(buffer, len)?;
        match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(PacketParsingError::InvalidUnicode),
        }
    }

    pub fn var_i32(buffer: &mut &[u8]) -> Result<i32, PacketParsingError> {
        Ok(var_u32(buffer)? as i32)
    }
    pub fn var_i64(buffer: &mut &[u8]) -> Result<i64, PacketParsingError> {
        Ok(var_u64(buffer)? as i64)
    }

    pub fn var_u32(buffer: &mut &[u8]) -> Result<u32, PacketParsingError> {
        return read_byte(buffer, 1);

        fn read_byte(buffer: &mut &[u8], len: u8) -> Result<u32, PacketParsingError> {
            if len > 5 {
                return Err(PacketParsingError::VarNumberTooBig);
            }
            let byte: u8 = u8(buffer)?;
            let x = (byte & 0b01111111) as u32;
            if (byte & 0b10000000) == 0 {
                Ok(x)
            } else {
                Ok(x | (read_byte(buffer, len + 1)? << 7))
            }
        }
    }
    // //pub fn var_u64(buffer: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
    // //    let mut num_read = 0u32;
    // //    let mut result: u64 = 0;
    // //    loop {
    // //        let read: u8 = match buffer.drain(0..1).next() {
    // //            Some(v) => v,
    // //            None => return Err(Box::new(PacketParsingError::EndOfInput)),
    // //        };
    // //        let val = read & 0b01111111;
    // //        result |= (val as u64) << (7 * num_read);
    // //
    // //        num_read += 1;
    // //        if num_read > 10 {
    // //            return Err(Box::new(PacketParsingError::VarNumberTooBig));
    // //        }
    // //        if (read & 0b10000000) == 0 {
    // //            return Ok(result);
    // //        }
    // //    }
    // //}
    pub fn var_u64(buffer: &mut &[u8]) -> Result<u64, PacketParsingError> {
        return read_byte(buffer, 1);

        fn read_byte(buffer: &mut &[u8], len: u8) -> Result<u64, PacketParsingError> {
            if len > 10 {
                return Err(PacketParsingError::VarNumberTooBig);
            }
            let byte: u8 = u8(buffer)?;
            let x = (byte & 0b01111111) as u64;
            if (byte & 0b10000000) == 0 {
                Ok(x)
            } else {
                Ok(x | (read_byte(buffer, len + 1)? << 7))
            }
        }
    }

    pub fn json(buffer: &mut &[u8]) -> Result<JsonValue, PacketParsingError> {
        let s: String = string(buffer)?;
        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            Err(_) => Err(PacketParsingError::InvalidJson),
        }
    }

    pub fn block_position(buffer: &mut &[u8]) -> Result<Vec3d<i32>, Box<dyn Error>> {
        let data = u64(buffer)?;
        let mut x = (data >> 38) as u32;
        let mut z = ((data << 26) >> 38) as u32;
        let mut y = (data & 0xFFF) as u32;
        // Test for 26-bit sign
        if x >= (1 << 25) {
            // negative number, convert to 32-bit
            x = (x & 0x01_FF_FF_FF) | 0xFE_00_00_00;
        }
        // Test for 26-bit sign
        if z >= (1 << 25) {
            // negative number, convert to 32-bit
            z = (z & 0x01_FF_FF_FF) | 0xFE_00_00_00;
        }
        // Test for 12-bit sign
        if y >= (1 << 11) {
            // negative number, convert to 32-bit
            y = (y & 0x7_FF) | 0xFF_FF_F8_00;
        }
        Ok(Vec3d::new(x as i32, y as i32, z as i32))
    }
}
pub mod write {
    use crate::helpers::{chat_components::ChatComponent, Vec3d};
    use byteorder::{BigEndian, WriteBytesExt};
    use serde_json::Value as JsonValue;
    use uuid::Uuid;

    pub fn bool(buffer: &mut Vec<u8>, b: bool) {
        buffer.write_u8(b as u8).unwrap();
    }

    pub fn i8(buffer: &mut Vec<u8>, n: i8) {
        buffer.write_i8(n).unwrap();
    }
    pub fn u8(buffer: &mut Vec<u8>, n: u8) {
        buffer.write_u8(n).unwrap();
    }

    pub fn i16(buffer: &mut Vec<u8>, n: i16) {
        buffer.write_i16::<BigEndian>(n).unwrap();
    }
    pub fn u16(buffer: &mut Vec<u8>, n: u16) {
        buffer.write_u16::<BigEndian>(n).unwrap();
    }

    pub fn i32(buffer: &mut Vec<u8>, n: i32) {
        buffer.write_i32::<BigEndian>(n).unwrap();
    }
    pub fn u32(buffer: &mut Vec<u8>, n: u32) {
        buffer.write_u32::<BigEndian>(n).unwrap();
    }

    pub fn i64(buffer: &mut Vec<u8>, n: i64) {
        buffer.write_i64::<BigEndian>(n).unwrap();
    }
    pub fn u64(buffer: &mut Vec<u8>, n: u64) {
        buffer.write_u64::<BigEndian>(n).unwrap();
    }

    pub fn f32(buffer: &mut Vec<u8>, n: f32) {
        u32(buffer, n.to_bits());
    }
    pub fn f64(buffer: &mut Vec<u8>, n: f64) {
        u64(buffer, n.to_bits());
    }

    pub fn raw(buffer: &mut Vec<u8>, raw: &[u8]) {
        var_usize(buffer, raw.len());
        buffer.extend(raw);
    }
    pub fn string(buffer: &mut Vec<u8>, s: impl AsRef<str>) {
        let bytes = s.as_ref().as_bytes();
        var_usize(buffer, bytes.len());
        buffer.extend(bytes);
    }

    pub fn var_i16(buffer: &mut Vec<u8>, n: i16) {
        var_u16(buffer, n as u16);
    }
    pub fn var_i32(buffer: &mut Vec<u8>, n: i32) {
        var_u32(buffer, n as u32);
    }
    pub fn var_i64(buffer: &mut Vec<u8>, n: i64) {
        var_u64(buffer, n as u64);
    }

    pub fn var_u8(buffer: &mut Vec<u8>, n: u8) {
        let byte = ((n as u8) & 0b01111111) | (((n > 0b01111111) as u8) << 7);
        u8(buffer, byte);
        if n > 0b01111111 {
            u8(buffer, n >> 7);
            // Recursion is inefficient:
            // var_u8(buffer, byte >> 7);
        }
    }
    pub fn var_u16(buffer: &mut Vec<u8>, n: u16) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u16(buffer, n >> 7);
        }
    }
    pub fn var_u32(buffer: &mut Vec<u8>, n: u32) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u32(buffer, n >> 7);
        }
    }
    pub fn var_u64(buffer: &mut Vec<u8>, n: u64) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u64(buffer, n >> 7);
        }
    }
    pub fn var_usize(buffer: &mut Vec<u8>, /*mut*/ n: usize) {
        // //loop {
        // //    let byte: u8 = ((n as u8) & 0b01111111) | (((n != 0) as u8) << 7);
        // //    n >>= 7;
        // //    buffer.push(byte);
        // //    if n == 0 {
        // //        return;
        // //    }
        // //}
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_usize(buffer, n >> 7);
        }
    }

    pub fn json(buffer: &mut Vec<u8>, json: &JsonValue) {
        let jsb = serde_json::ser::to_vec(json).unwrap();
        var_usize(buffer, jsb.len());
        // No error can occur here because the writer is a byte vec
        buffer.extend(jsb);
    }

    pub fn chat_component(buffer: &mut Vec<u8>, comp: &ChatComponent) {
        let mut json = Vec::new();
        comp.serialize_json(&mut json);
        var_usize(buffer, json.len());
        println!("JSON: {:#?}", json);
        buffer.append(&mut json);
    }

    pub fn chat_components(buffer: &mut Vec<u8>, comps: &[ChatComponent]) {
        if comps.len() == 1 {
            chat_component(buffer, &comps[0]);
            return;
        }
        let mut json = Vec::new();
        json.push(b'[');
        {
            let mut iter = comps.iter();
            if let Some(comp) = iter.next() {
                comp.serialize_json(&mut json);
                for comp in iter {
                    json.push(b',');
                    comp.serialize_json(&mut json);
                }
            }
        }
        json.push(b']');
        var_usize(buffer, json.len());
        println!("JSON: {:#?}", json);
        buffer.append(&mut json);
    }

    pub fn uuid(buffer: &mut Vec<u8>, uuid: Uuid) {
        // TODO: Test endianness
        buffer.extend_from_slice(uuid.as_bytes());
    }

    /// Converts a block position into a 64-bit unsigned composite number
    /// X (signed 26-bit int) + Z (signed 26-bit int) + Y (signed 12-bit int)
    ///
    /// [Documentation](https://wiki.vg/Protocol#Position)
    pub fn block_position(buffer: &mut Vec<u8>, pos: &Vec3d<i32>) {
        let x = pos.x;
        assert!(x >= -33554432, "X must be >= -33554432");
        assert!(x < 33554431, "X must be < 33554431");
        let z = pos.z;
        assert!(z >= -33554432, "X must be >= -33554432");
        assert!(z < 33554431, "X must be < 33554431");
        let y = pos.y;
        assert!(y >= -2048, "Y must be >= -2048");
        assert!(y < 2048, "Y must be < 2048");

        let x = if x < 0 {
            // Set the 26-bit int sign manually
            ((x as u64) & 0x01_FF_FF_FF) | 0x02_00_00_00
        } else {
            x as u64
        };
        let z = if z < 0 {
            // Set the 26-bit int sign manually
            ((z as u64) & 0x01_FF_FF_FF) | 0x02_00_00_00
        } else {
            z as u64
        };
        let y = if y < 0 {
            // Set the 12-bit int sign manually
            ((y as u64) & 0x7_FF) | 0x8_00
        } else {
            y as u64
        };
        let combined: u64 =
            ((x & 0x03_FF_FF_FF) << 38) | ((z & 0x03_FF_FF_FF) << 12) | (y & 0x0F_FF);
        u64(buffer, combined);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::Vec3d;

    #[test]
    fn test_packet_block_position() {
        let mut buffer: Vec<u8> = Vec::new();
        let pos: Vec3d<i32> = Vec3d::new(10, 255, -10);
        write::block_position(&mut buffer, &pos);
        let mut buf_slice = &buffer[..];
        let decoded: Vec3d<i32> = read::block_position(&mut buf_slice).unwrap();
        assert_eq!(pos.x, decoded.x, "ne x");
        assert_eq!(pos.y, decoded.y, "ne y");
        assert_eq!(pos.z, decoded.z, "ne z");
    }

    #[test]
    fn test_packet_i64() {
        let mut buffer = Vec::new();
        let x = 1234567890i64;
        write::i64(&mut buffer, x);
        write::i64(&mut buffer, -x);
        let mut buf_slice = &buffer[..];
        let decoded = read::i64(&mut buf_slice).unwrap();
        assert_eq!(decoded, x);
        let decoded = read::i64(&mut buf_slice).unwrap();
        assert_eq!(decoded, -x);
    }

    #[test]
    fn test_floats() {
        // Property based testing
        for _ in 0..100 {
            let float: f32 = rand::random();
            let mut buf = Vec::with_capacity(4);
            write::f32(&mut buf, float);
            let mut buf_slice = &buf[..];
            let result: f32 = read::f32(&mut buf_slice).unwrap();
            assert_eq!(buf_slice.len(), 0);
            assert!(result == float);
        }
    }
}
