/// Read data from raw packet bytes
///
/// Note: Numbers use the network byte order (big endian)
pub mod read {
    use crate::helpers::Vec3d;
    use crate::packet::*;
    use json::JsonValue;
    use std::error::Error;

    // TODO: Use the vec in reverse order and pop()

    pub fn bool(buffer: &mut Vec<u8>) -> Result<bool, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => match byte {
                0 => Ok(false),
                _ => Ok(true),
            },
            None => Err(Box::new(PacketParsingError::EndOfInput)),
        }
    }

    pub fn i8(buffer: &mut Vec<u8>) -> Result<i8, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => Ok(byte as i8),
            None => Err(Box::new(PacketParsingError::EndOfInput)),
        }
    }
    pub fn u8(buffer: &mut Vec<u8>) -> Result<u8, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => Ok(byte),
            None => Err(Box::new(PacketParsingError::EndOfInput)),
        }
    }

    pub fn i16(buffer: &mut Vec<u8>) -> Result<i16, Box<dyn Error>> {
        Ok(u16(buffer)? as i16)
    }
    pub fn u16(buffer: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
        if buffer.len() < 2 {
            return Err(Box::new(PacketParsingError::EndOfInput));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&buffer[0..2]);
        buffer.drain(0..2);
        Ok(u16::from_be_bytes(bytes))
    }

    pub fn i32(buffer: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
        Ok(u32(buffer)? as i32)
    }
    pub fn u32(buffer: &mut Vec<u8>) -> Result<u32, Box<dyn Error>> {
        if buffer.len() < 4 {
            return Err(Box::new(PacketParsingError::EndOfInput));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&buffer[0..4]);
        buffer.drain(0..4);
        Ok(u32::from_be_bytes(bytes))
    }

    pub fn i64(buffer: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
        Ok(u64(buffer)? as i64)
    }
    pub fn u64(buffer: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
        if buffer.len() < 8 {
            return Err(Box::new(PacketParsingError::EndOfInput));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&buffer[0..8]);
        buffer.drain(0..8);
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn f32(buffer: &mut Vec<u8>) -> Result<f32, Box<dyn Error>> {
        let bits = super::read::u32(buffer)?;
        Ok(f32::from_bits(bits))
    }
    pub fn f64(buffer: &mut Vec<u8>) -> Result<f64, Box<dyn Error>> {
        let bits = super::read::u64(buffer)?;
        Ok(f64::from_bits(bits))
    }

    pub fn string(buffer: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
        let len = var_u32(buffer)? as usize;
        let bytes = buffer.drain(0..len).collect::<Vec<u8>>();
        return Ok(String::from_utf8(bytes)?);
    }

    pub fn var_i32(buffer: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
        Ok(var_u32(buffer)? as i32)
    }
    pub fn var_i64(buffer: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
        Ok(var_u64(buffer)? as i64)
    }

    pub fn var_u32(buffer: &mut Vec<u8>) -> Result<u32, Box<dyn Error>> {
        return match read_byte(buffer, 1) {
            Ok(num) => Ok(num),
            Err(true) => Err(Box::new(PacketParsingError::VarNumberTooBig)),
            Err(false) => Err(Box::new(PacketParsingError::EndOfInput)),
        };

        fn read_byte(buffer: &mut Vec<u8>, len: u8) -> Result<u32, bool> {
            if len > 5 {
                return Err(true);
            }
            let byte: u8 = match buffer.drain(0..1).next() {
                Some(v) => v,
                None => return Err(false),
            };
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
    pub fn var_u64(buffer: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
        return match read_byte(buffer, 1) {
            Ok(num) => Ok(num),
            Err(true) => Err(Box::new(PacketParsingError::VarNumberTooBig)),
            Err(false) => Err(Box::new(PacketParsingError::EndOfInput)),
        };

        fn read_byte(buffer: &mut Vec<u8>, len: u8) -> Result<u64, bool> {
            if len > 10 {
                return Err(true);
            }
            let byte: u8 = match buffer.drain(0..1).next() {
                Some(v) => v,
                None => return Err(false),
            };
            let x = (byte & 0b01111111) as u64;
            if (byte & 0b10000000) == 0 {
                Ok(x)
            } else {
                Ok(x | (read_byte(buffer, len + 1)? << 7))
            }
        }
    }

    pub fn json(buffer: &mut Vec<u8>) -> Result<JsonValue, Box<dyn Error>> {
        let str: String = string(buffer)?;
        Ok(json::parse(str.as_str())?)
    }

    pub fn block_position(buffer: &mut Vec<u8>) -> Result<Vec3d<i32>, Box<dyn Error>> {
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
    use crate::helpers::Vec3d;
    use json::JsonValue;

    pub fn bool(buffer: &mut Vec<u8>, b: bool) {
        match b {
            false => buffer.push(0),
            true => buffer.push(1),
        };
    }

    pub fn i8(buffer: &mut Vec<u8>, n: i8) {
        buffer.push(n as u8);
    }
    pub fn u8(buffer: &mut Vec<u8>, n: u8) {
        buffer.push(n);
    }

    pub fn i16(buffer: &mut Vec<u8>, n: i16) {
        buffer.extend(&i16::to_be_bytes(n));
    }
    pub fn u16(buffer: &mut Vec<u8>, n: u16) {
        buffer.extend(&u16::to_be_bytes(n));
    }

    pub fn i32(buffer: &mut Vec<u8>, n: i32) {
        buffer.extend(&i32::to_be_bytes(n));
    }
    pub fn u32(buffer: &mut Vec<u8>, n: u32) {
        buffer.extend(&u32::to_be_bytes(n));
    }

    pub fn i64(buffer: &mut Vec<u8>, n: i64) {
        buffer.extend(&i64::to_be_bytes(n));
    }
    pub fn u64(buffer: &mut Vec<u8>, n: u64) {
        buffer.extend(&u64::to_be_bytes(n));
    }

    pub fn f32(buffer: &mut Vec<u8>, n: f32) {
        u32(buffer, n.to_bits());
    }
    pub fn f64(buffer: &mut Vec<u8>, n: f64) {
        u64(buffer, n.to_bits());
    }

    pub fn string(buffer: &mut Vec<u8>, s: impl AsRef<str>) {
        let bytes = s.as_ref().as_bytes();
        var_u32(buffer, bytes.len() as u32);
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
        buffer.push(byte);
        if n > 0b01111111 {
            buffer.push(byte >> 7);
            // Recursion is slower:
            // // var_u8(buffer, byte >> 7);
        }
    }
    pub fn var_u16(buffer: &mut Vec<u8>, n: u16) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        buffer.push(byte);
        if next {
            var_u16(buffer, n >> 7);
        }
    }
    pub fn var_u32(buffer: &mut Vec<u8>, n: u32) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        buffer.push(byte);
        if next {
            var_u32(buffer, n >> 7);
        }
    }
    pub fn var_u64(buffer: &mut Vec<u8>, n: u64) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        buffer.push(byte);
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
        buffer.push(byte);
        if next {
            var_usize(buffer, n >> 7);
        }
    }

    pub fn json(buffer: &mut Vec<u8>, json: &JsonValue) {
        string(buffer, json.dump());
    }

    /// Converts a block position into a 64-bit unsigned composite number
    /// X (signed 26-bit int) + Z (signed 26-bit int) + Y (signed 12-bit int)
    ///
    /// [Documentation](https://wiki.vg/Protocol#Position)
    pub fn block_position(buffer: &mut Vec<u8>, pos: &Vec3d<i32>) {
        let x = pos.get_x();
        assert!(x >= -33554432, "X must be >= -33554432");
        assert!(x < 33554431, "X must be < 33554431");
        let z = pos.get_z();
        assert!(z >= -33554432, "X must be >= -33554432");
        assert!(z < 33554431, "X must be < 33554431");
        let y = pos.get_y();
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
        let decoded: Vec3d<i32> = read::block_position(&mut buffer).unwrap();
        assert_eq!(pos.get_x(), decoded.get_x(), "ne x");
        assert_eq!(pos.get_y(), decoded.get_y(), "ne y");
        assert_eq!(pos.get_z(), decoded.get_z(), "ne z");
    }

    #[test]
    fn test_packet_i64() {
        let mut buffer = Vec::new();
        let x = 1234567890i64;
        write::i64(&mut buffer, x);
        write::i64(&mut buffer, -x);
        let decoded = read::i64(&mut buffer).unwrap();
        assert_eq!(decoded, x);
        let decoded = read::i64(&mut buffer).unwrap();
        assert_eq!(decoded, -x);
    }
}
