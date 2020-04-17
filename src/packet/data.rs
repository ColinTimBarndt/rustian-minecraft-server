pub mod read {
    use crate::packet::*;
    use std::error::Error;
    use json::JsonValue;

    pub fn bool(buffer: &mut Vec<u8>) -> Result<bool, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => match byte {
                0 => Ok(false),
                _ => Ok(true)
            },
            None => Err(Box::new(PacketParsingError::EndOfInput))
        }
    }

    pub fn i8(buffer: &mut Vec<u8>) -> Result<i8, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => Ok(byte as i8),
            None => Err(Box::new(PacketParsingError::EndOfInput))
        }
    }
    pub fn u8(buffer: &mut Vec<u8>) -> Result<u8, Box<dyn Error>> {
        match buffer.drain(0..1).next() {
            Some(byte) => Ok(byte),
            None => Err(Box::new(PacketParsingError::EndOfInput))
        }
    }

    pub fn i16(buffer: &mut Vec<u8>) -> Result<i16, Box<dyn Error>> {
        Ok(u16(buffer)? as i16)
    }
    pub fn u16(buffer: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
        let mut bytes: [u8; 2] = [0; 2];
        let mut drain = buffer.drain(0..2);
        for i in 0..2 {
            bytes[i] = match drain.next() {
                Some(b) => b,
                None => return Err(Box::new(PacketParsingError::EndOfInput))
            };
        };
        Ok(
              (bytes[1] as u16)
            |((bytes[0] as u16)<<8)
        )
    }

    pub fn i32(buffer: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
        let mut bytes: [u8; 4] = [0; 4];
        let mut drain = buffer.drain(0..4);
        for i in 0..4 {
            bytes[i] = match drain.next() {
                Some(b) => b,
                None => return Err(Box::new(PacketParsingError::EndOfInput))
            };
        };
        Ok((
              (bytes[3] as u32)
            |((bytes[2] as u32)<<8)
            |((bytes[1] as u32)<<16)
            |((bytes[0] as u32)<<24)
        ) as i32)
    }

    pub fn i64(buffer: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
        Ok(u64(buffer)? as i64)
    }
    pub fn u64(buffer: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
        let mut bytes: [u8; 8] = [0; 8];
        let mut drain = buffer.drain(0..8);
        for i in 0..8 {
            bytes[i] = match drain.next() {
                Some(b) => b,
                None => return Err(Box::new(PacketParsingError::EndOfInput))
            };
        };
        Ok(
              (bytes[7] as u64)
            |((bytes[6] as u64)<<8)
            |((bytes[5] as u64)<<16)
            |((bytes[4] as u64)<<24)
            |((bytes[3] as u64)<<32)
            |((bytes[2] as u64)<<40)
            |((bytes[1] as u64)<<48)
            |((bytes[1] as u64)<<56)
        )
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
        let mut num_read = 0u32;
        let mut result: u32 = 0;
        loop {
            let read: u8 = match buffer.drain(0..1).next() {
                Some(v) => v,
                None => return Err(Box::new(PacketParsingError::EndOfInput))
            };
            let val = read & 0b01111111;
            result |= (val as u32) << (7*num_read);

            num_read+=1;
            if num_read > 5 {
                return Err(Box::new(PacketParsingError::VarNumberTooBig))
            }
            if (read & 0b10000000) == 0 {
                return Ok(result);
            }
        }
    }
    pub fn var_u64(buffer: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
        let mut num_read = 0u32;
        let mut result: u64 = 0;
        loop {
            let read: u8 = match buffer.drain(0..1).next() {
                Some(v) => v,
                None => return Err(Box::new(PacketParsingError::EndOfInput))
            };
            let val = read & 0b01111111;
            result |= (val as u64) << (7*num_read);

            num_read+=1;
            if num_read > 10 {
                return Err(Box::new(PacketParsingError::VarNumberTooBig))
            }
            if (read & 0b10000000) == 0 {
                return Ok(result);
            }
        }
    }

    pub fn json(buffer: &mut Vec<u8>) -> Result<JsonValue, Box<dyn Error>> {
        let str: String = string(buffer)?;
        Ok(json::parse(str.as_str())?)
    }

    pub fn pos(buffer: &mut Vec<u8>) -> Result<(i32, i16, i32), Box<dyn Error>> {
        let data = u64(buffer)?;
        let mut x = (data >> 38) as i32;
        let mut y = (data & 0xfff) as i16;
        let mut z = ((data << 26) >> 38) as i32;
        if x >= 2^25 { x -= 2^26 }
        if y >= 2^11 { y -= 2^12 }
        if z >= 2^25 { z -= 2^26 }
        Ok((x, y, z))
    }
}
pub mod write {
    use json::JsonValue;

    pub fn bool(buffer: &mut Vec<u8>, b: bool) {
        match b {
            false => buffer.push(0),
            true => buffer.push(1)
        };
    }

    pub fn i8(buffer: &mut Vec<u8>, n: i8) {
        buffer.push(n as u8);
    }
    pub fn u8(buffer: &mut Vec<u8>, n: u8) {
        buffer.push(n);
    }

    pub fn i16(buffer: &mut Vec<u8>, n: i16) {
        u16(buffer, n as u16);
    }
    pub fn u16(buffer: &mut Vec<u8>, n: u16) {
        buffer.push((n>>8) as u8);
        buffer.push(n as u8);
    }

    pub fn i32(buffer: &mut Vec<u8>, n: i32) {
        u32(buffer, n as u32);
    }
    pub fn u32(buffer: &mut Vec<u8>, n: u32) {
        buffer.push((n>>24) as u8);
        buffer.push((n>>16) as u8);
        buffer.push((n>>8) as u8);
        buffer.push(n as u8);
    }

    pub fn i64(buffer: &mut Vec<u8>, n: i64) {
        u64(buffer, n as u64);
    }
    pub fn u64(buffer: &mut Vec<u8>, n: u64) {
        buffer.push((n>>56) as u8);
        buffer.push((n>>48) as u8);
        buffer.push((n>>40) as u8);
        buffer.push((n>>32) as u8);
        buffer.push((n>>24) as u8);
        buffer.push((n>>16) as u8);
        buffer.push((n>>8) as u8);
        buffer.push(n as u8);
    }

    pub fn string(buffer: &mut Vec<u8>, s: String) {
        let mut bytes = s.into_bytes();
        var_u32(buffer, bytes.len() as u32);
        buffer.append(&mut bytes);
    }

    pub fn var_i32(buffer: &mut Vec<u8>, n: i32) {
        var_u32(buffer, n as u32);
    }
    pub fn var_i64(buffer: &mut Vec<u8>, n: i64) {
        var_u64(buffer, n as u64);
    }

    pub fn var_u32(buffer: &mut Vec<u8>, mut n: u32) {
        loop {
            let mut temp: u8 = (n&0b01111111) as u8;
            n >>= 7;
            if n != 0 {
                temp |= 0b10000000;
            }
            buffer.push(temp);
            if n == 0 {
                return;
            }
        }
    }
    pub fn var_u64(buffer: &mut Vec<u8>, mut n: u64) {
        loop {
            let mut temp: u8 = (n&0b01111111) as u8;
            n >>= 7;
            if n != 0 {
                temp |= 0b10000000;
            }
            buffer.push(temp);
            if n == 0 {
                return;
            }
        }
    }

    pub fn json(buffer: &mut Vec<u8>, json: &JsonValue) {
        string(buffer, json.dump());
    }
}
