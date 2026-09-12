//! VarInt / VarLong primitives (Minecraft's LEB128-style encoding).

use std::io::{self, Read};

/// Maximum byte length of an encoded VarInt.
pub const VARINT_MAX_SIZE: usize = 5;
/// Maximum byte length of an encoded VarLong.
pub const VARLONG_MAX_SIZE: usize = 10;

/// Append an i32 as a VarInt to `out`.
pub fn write_varint(out: &mut Vec<u8>, value: i32) {
    let mut v = value as u32;
    loop {
        let mut byte = (v & 0x7F) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if v == 0 {
            break;
        }
    }
}

/// Append an i64 as a VarLong to `out`.
pub fn write_varlong(out: &mut Vec<u8>, value: i64) {
    let mut v = value as u64;
    loop {
        let mut byte = (v & 0x7F) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if v == 0 {
            break;
        }
    }
}

/// Read a VarInt from any reader. Errors after 5 bytes.
pub fn read_varint(r: &mut impl Read) -> io::Result<i32> {
    let mut result: u32 = 0;
    for i in 0..VARINT_MAX_SIZE {
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        result |= ((b[0] & 0x7F) as u32) << (7 * i);
        if b[0] & 0x80 == 0 {
            return Ok(result as i32);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "varint too long (> 5 bytes)",
    ))
}

/// Read a VarLong from any reader. Errors after 10 bytes.
pub fn read_varlong(r: &mut impl Read) -> io::Result<i64> {
    let mut result: u64 = 0;
    for i in 0..VARLONG_MAX_SIZE {
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        result |= ((b[0] & 0x7F) as u64) << (7 * i);
        if b[0] & 0x80 == 0 {
            return Ok(result as i64);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "varlong too long (> 10 bytes)",
    ))
}

/// Encode a UTF-8 string the way Minecraft does (VarInt length + bytes).
pub fn write_string(out: &mut Vec<u8>, s: &str) {
    write_varint(out, s.len() as i32);
    out.extend_from_slice(s.as_bytes());
}

/// Read a Minecraft-encoded string (VarInt length + UTF-8 bytes). The
/// length must be sane (>= 0, < 16 MiB) — garbage lengths are rejected.
pub fn read_string(r: &mut impl Read) -> io::Result<String> {
    let len = read_varint(r)?;
    if len < 0 || len as usize > 16 * 1024 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid string length {len}"),
        ));
    }
    let mut bytes = vec![0u8; len as usize];
    r.read_exact(&mut bytes)?;
    String::from_utf8(bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("utf-8: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rt_varint(v: i32) -> i32 {
        let mut buf = Vec::new();
        write_varint(&mut buf, v);
        assert!(buf.len() <= VARINT_MAX_SIZE);
        read_varint(&mut io::Cursor::new(&buf)).unwrap()
    }

    fn rt_varlong(v: i64) -> i64 {
        let mut buf = Vec::new();
        write_varlong(&mut buf, v);
        assert!(buf.len() <= VARLONG_MAX_SIZE);
        read_varlong(&mut io::Cursor::new(&buf)).unwrap()
    }

    #[test]
    fn varint_roundtrip() {
        for v in [0, 1, 127, 128, 255, 256, 65_535, 2_097_151, i32::MAX, -1, i32::MIN] {
            assert_eq!(rt_varint(v), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn varint_known_encodings() {
        let mut b = Vec::new();
        write_varint(&mut b, 0);
        assert_eq!(b, vec![0x00]);
        b.clear();
        write_varint(&mut b, 127);
        assert_eq!(b, vec![0x7F]);
        b.clear();
        write_varint(&mut b, 128);
        assert_eq!(b, vec![0x80, 0x01]);
        b.clear();
        write_varint(&mut b, 2097151);
        assert_eq!(b, vec![0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn varlong_roundtrip() {
        for v in [0, 1, 127, 128, i64::MAX, -1, i64::MIN] {
            assert_eq!(rt_varlong(v), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn string_encoding() {
        let mut b = Vec::new();
        write_string(&mut b, "abc");
        assert_eq!(b, vec![0x03, b'a', b'b', b'c']);
    }

    #[test]
    fn varint_rejects_overflow() {
        let garbage = [0x80u8; 6];
        assert!(read_varint(&mut io::Cursor::new(&garbage)).is_err());
    }
}
