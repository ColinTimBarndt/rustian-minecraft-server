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
    use std::io::Read;

    pub fn byte_vec<R: Read + ?Sized>(
        buffer: &mut R,
        len: usize,
    ) -> Result<Vec<u8>, PacketParsingError> {
        let mut buf = vec![0; len];
        match buffer.read_exact(&mut buf) {
            Ok(()) => Ok(buf),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn bool<R: Read + ?Sized>(buffer: &mut R) -> Result<bool, PacketParsingError> {
        match buffer.read_u8() {
            Ok(byte) => Ok(byte != 0),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn i8<R: Read + ?Sized>(buffer: &mut R) -> Result<i8, PacketParsingError> {
        match buffer.read_i8() {
            Ok(byte) => Ok(byte),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }
    #[inline(always)]
    pub fn u8<R: Read + ?Sized>(buffer: &mut R) -> Result<u8, PacketParsingError> {
        match buffer.read_u8() {
            Ok(byte) => Ok(byte),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn i16<R: Read + ?Sized>(buffer: &mut R) -> Result<i16, PacketParsingError> {
        match buffer.read_i16::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }
    #[inline(always)]
    pub fn u16<R: Read + ?Sized>(buffer: &mut R) -> Result<u16, PacketParsingError> {
        match buffer.read_u16::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn i32<R: Read + ?Sized>(buffer: &mut R) -> Result<i32, PacketParsingError> {
        match buffer.read_i32::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }
    #[inline(always)]
    pub fn u32<R: Read + ?Sized>(buffer: &mut R) -> Result<u32, PacketParsingError> {
        match buffer.read_u32::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn i64<R: Read + ?Sized>(buffer: &mut R) -> Result<i64, PacketParsingError> {
        match buffer.read_i64::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }
    #[inline(always)]
    pub fn u64<R: Read + ?Sized>(buffer: &mut R) -> Result<u64, PacketParsingError> {
        match buffer.read_u64::<BigEndian>() {
            Ok(n) => Ok(n),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn f32<R: Read + ?Sized>(buffer: &mut R) -> Result<f32, PacketParsingError> {
        match buffer.read_f32::<BigEndian>() {
            Ok(f) => Ok(f),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }
    #[inline(always)]
    pub fn f64<R: Read + ?Sized>(buffer: &mut R) -> Result<f64, PacketParsingError> {
        match buffer.read_f64::<BigEndian>() {
            Ok(f) => Ok(f),
            Err(_) => Err(PacketParsingError::EndOfInput),
        }
    }

    #[inline(always)]
    pub fn string<R: Read + ?Sized>(buffer: &mut R) -> Result<String, PacketParsingError> {
        let len = var_u32(buffer)? as usize;
        let bytes = byte_vec(buffer, len)?;
        match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(PacketParsingError::InvalidUnicode),
        }
    }

    #[inline(always)]
    pub fn var_i32<R: Read + ?Sized>(buffer: &mut R) -> Result<i32, PacketParsingError> {
        Ok(var_u32(buffer)? as i32)
    }
    #[inline(always)]
    pub fn var_i64<R: Read + ?Sized>(buffer: &mut R) -> Result<i64, PacketParsingError> {
        Ok(var_u64(buffer)? as i64)
    }

    pub fn var_u32<R: Read + ?Sized>(buffer: &mut R) -> Result<u32, PacketParsingError> {
        return read_byte(buffer, 1);

        fn read_byte<R: Read + ?Sized>(buffer: &mut R, len: u8) -> Result<u32, PacketParsingError> {
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
    pub fn var_u64<R: Read + ?Sized>(buffer: &mut R) -> Result<u64, PacketParsingError> {
        return read_byte(buffer, 1);

        fn read_byte<R: Read + ?Sized>(buffer: &mut R, len: u8) -> Result<u64, PacketParsingError> {
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

    pub fn json<R: Read + ?Sized>(buffer: &mut R) -> Result<JsonValue, PacketParsingError> {
        let s: String = string(buffer)?;
        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            Err(_) => Err(PacketParsingError::InvalidJson),
        }
    }

    pub fn block_position<R: Read + ?Sized>(buffer: &mut R) -> Result<Vec3d<i32>, Box<dyn Error>> {
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
    use std::io::Write;
    use uuid::Uuid;

    #[inline(always)]
    pub fn bool<W: Write + ?Sized>(buffer: &mut W, b: bool) {
        buffer.write_u8(b as u8).unwrap();
    }

    #[inline(always)]
    pub fn i8<W: Write + ?Sized>(buffer: &mut W, n: i8) {
        buffer.write_i8(n).unwrap();
    }
    #[inline(always)]
    pub fn u8<W: Write + ?Sized>(buffer: &mut W, n: u8) {
        buffer.write_u8(n).unwrap();
    }

    #[inline(always)]
    pub fn i16<W: Write + ?Sized>(buffer: &mut W, n: i16) {
        buffer.write_i16::<BigEndian>(n).unwrap();
    }
    #[inline(always)]
    pub fn u16<W: Write + ?Sized>(buffer: &mut W, n: u16) {
        buffer.write_u16::<BigEndian>(n).unwrap();
    }

    #[inline(always)]
    pub fn i32<W: Write + ?Sized>(buffer: &mut W, n: i32) {
        buffer.write_i32::<BigEndian>(n).unwrap();
    }
    #[inline(always)]
    pub fn u32<W: Write + ?Sized>(buffer: &mut W, n: u32) {
        buffer.write_u32::<BigEndian>(n).unwrap();
    }

    #[inline(always)]
    pub fn i64<W: Write + ?Sized>(buffer: &mut W, n: i64) {
        buffer.write_i64::<BigEndian>(n).unwrap();
    }
    #[inline(always)]
    pub fn u64<W: Write + ?Sized>(buffer: &mut W, n: u64) {
        buffer.write_u64::<BigEndian>(n).unwrap();
    }

    #[inline(always)]
    pub fn f32<W: Write + ?Sized>(buffer: &mut W, f: f32) {
        u32(buffer, f.to_bits());
    }
    #[inline(always)]
    pub fn f64<W: Write + ?Sized>(buffer: &mut W, f: f64) {
        u64(buffer, f.to_bits());
    }

    #[inline(always)]
    pub fn raw<W: Write + ?Sized>(buffer: &mut W, raw: &[u8]) {
        var_usize(buffer, raw.len());
        buffer.write_all(raw).unwrap();
    }
    #[inline(always)]
    pub fn string<W: Write + ?Sized>(buffer: &mut W, s: &str) {
        raw(buffer, s.as_bytes());
    }

    #[inline(always)]
    pub fn var_i16<W: Write + ?Sized>(buffer: &mut W, n: i16) {
        var_u16(buffer, n as u16);
    }
    #[inline(always)]
    pub fn var_i32<W: Write + ?Sized>(buffer: &mut W, n: i32) {
        var_u32(buffer, n as u32);
    }
    #[inline(always)]
    pub fn var_i64<W: Write + ?Sized>(buffer: &mut W, n: i64) {
        var_u64(buffer, n as u64);
    }

    pub fn var_u8<W: Write + ?Sized>(buffer: &mut W, n: u8) {
        let byte = ((n as u8) & 0b01111111) | (((n > 0b01111111) as u8) << 7);
        u8(buffer, byte);
        if n > 0b01111111 {
            u8(buffer, n >> 7);
            // Recursion is inefficient:
            // var_u8(buffer, byte >> 7);
        }
    }
    pub fn var_u16<W: Write + ?Sized>(buffer: &mut W, n: u16) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u16(buffer, n >> 7);
        }
    }
    pub fn var_u32<W: Write + ?Sized>(buffer: &mut W, n: u32) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u32(buffer, n >> 7);
        }
    }
    pub fn var_u64<W: Write + ?Sized>(buffer: &mut W, n: u64) {
        let next = n > 0b01111111;
        let byte = ((n as u8) & 0b01111111) | ((next as u8) << 7);
        u8(buffer, byte);
        if next {
            var_u64(buffer, n >> 7);
        }
    }
    pub fn var_usize<W: Write + ?Sized>(buffer: &mut W, /*mut*/ n: usize) {
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

    #[inline(always)]
    pub fn json<W: Write + ?Sized, S: serde::ser::Serialize + ?Sized>(buffer: &mut W, json: &S) {
        let jsb = serde_json::ser::to_vec(json).unwrap();
        raw(buffer, &jsb);
    }

    #[inline(always)]
    pub fn chat_component<W: Write + ?Sized>(buffer: &mut W, comp: &ChatComponent) {
        let mut jsb = Vec::new();
        comp.serialize_json(&mut jsb);
        raw(buffer, &jsb);
    }

    pub fn chat_components<W: Write + ?Sized>(buffer: &mut W, comps: &[ChatComponent]) {
        if comps.len() == 1 {
            chat_component(buffer, &comps[0]);
            return;
        }
        let mut jsb = Vec::new();
        jsb.push(b'[');
        {
            let mut iter = comps.iter();
            if let Some(comp) = iter.next() {
                comp.serialize_json(&mut jsb);
                for comp in iter {
                    jsb.push(b',');
                    comp.serialize_json(&mut jsb);
                }
            }
        }
        jsb.push(b']');
        raw(buffer, &jsb);
    }

    #[inline(always)]
    pub fn uuid<W: Write + ?Sized>(buffer: &mut W, uuid: Uuid) {
        // TODO: Test endianness
        buffer.write_all(uuid.as_bytes()).unwrap();
    }

    /// Converts a block position into a 64-bit unsigned composite number
    /// X (signed 26-bit int) + Z (signed 26-bit int) + Y (signed 12-bit int)
    ///
    /// [Documentation](https://wiki.vg/Protocol#Position)
    pub fn block_position<W: Write + ?Sized>(
        buffer: &mut W,
        pos: &Vec3d<i32>,
    ) -> Result<(), &'static str> {
        let x = pos.x;
        if !(x >= -33554432) {
            return Err("X must be >= -33554432");
        }
        if !(x < 33554431) {
            return Err("X must be < 33554431");
        }
        let z = pos.z;
        if !(z >= -33554432) {
            return Err("X must be >= -33554432");
        }
        if !(z < 33554431) {
            return Err("X must be < 33554431");
        }
        let y = pos.y;
        if !(y >= -2048) {
            return Err("Y must be >= -2048");
        }
        if !(y < 2048) {
            return Err("Y must be < 2048");
        }

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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::Vec3d;

    #[test]
    fn test_packet_block_position() -> Result<(), &'static str> {
        let mut buffer: Vec<u8> = Vec::new();
        let pos: Vec3d<i32> = Vec3d::new(10, 255, -10);
        write::block_position(&mut buffer, &pos)?;
        let mut buf_slice = &buffer[..];
        let decoded: Vec3d<i32> = read::block_position(&mut buf_slice).unwrap();
        assert_eq!(pos.x, decoded.x, "ne x");
        assert_eq!(pos.y, decoded.y, "ne y");
        assert_eq!(pos.z, decoded.z, "ne z");
        Ok(())
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
