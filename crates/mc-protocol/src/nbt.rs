//! Network NBT (1.20.5+ anonymous-root, big-endian) — the project's first
//! RETAINED NBT reader. The chunk path walks NBT and throws it away
//! (`play::skip_network_nbt`); the configuration Registry Data is the
//! first surface that must actually UNDERSTAND the tree — the
//! `minecraft:dimension_type` registry carries the world's vertical
//! layout (min_y / height) inside a list of compounds.
//!
//! Wire facts (clean-room, wiki.vg NBT spec):
//! - Payload = one anonymous root TAG_Compound (0x0A, no name — the
//!   1.20.5+ network format; disk NBT names the root).
//! - Every tag: id byte, then (for compounds/lists) a u16 length + UTF-8
//!   name, then the payload.
//! - TAG_List = inner tag id byte + i32 count + that many inner payloads;
//!   an EMPTY list conventionally declares TAG_End (0x00) as its inner
//!   type — the writer here does exactly that (the inner type of an empty
//!   list is not retained, documented lossless-for-non-empty tradeoff).
//! - Strings = u16 length + bytes, read lossily (registry content is
//!   plain identifiers; nothing downstream wants a hard error on exotic
//!   bytes).
//!
//! The reader is bounded (128 nesting levels) so a hostile packet cannot
//! overflow the stack; the chunk-path skipper predates this module and is
//! untouched. The writer is the reader's mirror — mock servers and future
//! clientbound NBT (cookies) emit through it, and the roundtrip test
//! pins reader and writer to each other.

use std::io::{self, Read};

/// One NBT value. Compounds keep insertion order (registry "value" lists
/// and element fields are read positionally by name anyway).
#[derive(Debug, Clone, PartialEq)]
pub enum Nbt {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Nbt>),
    Compound(Vec<(String, Nbt)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// The hard nesting bound (real vanilla registries nest ~6 deep).
const MAX_DEPTH: u32 = 128;

/// Read one anonymous-root network NBT value from the cursor, consuming
/// exactly its bytes on success.
pub fn read_network_nbt(r: &mut &[u8]) -> io::Result<Nbt> {
    let root = read_u8(r)?;
    if root != 0x0A {
        return Err(corrupt(&format!(
            "expected an NBT root compound (0x{root:02X})"
        )));
    }
    read_compound_contents(r, 0)
}

/// Read one BARE network NBT tag — the id byte + payload, no root
/// framing. The entity-metadata components' shape: a chat component
/// (`ComponentSerialization.TRUSTED_STREAM_CODEC` =
/// `fromCodecWithRegistriesTrusted` = `tagCodec` + the NBT-ops adapter)
/// arrives as a bare TAG_String (plain text), TAG_Compound (the
/// structured form), TAG_List (siblings) or a scalar literal — never
/// as the anonymous-root compound the chunk packets frame.
pub fn read_bare_nbt(r: &mut &[u8]) -> io::Result<Nbt> {
    let tag = read_u8(r)?;
    read_payload(r, tag, 0)
}

impl Nbt {
    /// Compound lookup by name (None for non-compounds and absent keys).
    pub fn get(&self, key: &str) -> Option<&Nbt> {
        match self {
            Nbt::Compound(entries) => {
                entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
            }
            _ => None,
        }
    }

    /// Numeric view across the integer tags (Byte/Short/Int always;
    /// Long only when it fits i32 — registry fields like min_y/height
    /// are plain ints).
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Nbt::Byte(v) => Some(v as i32),
            Nbt::Short(v) => Some(v as i32),
            Nbt::Int(v) => Some(v),
            Nbt::Long(v) => i32::try_from(v).ok(),
            _ => None,
        }
    }

    /// Float view (the timeline track values: the element codecs write
    /// FLOAT, but a DOUBLE rides just as legally — both widen honestly).
    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Nbt::Float(v) => Some(v),
            Nbt::Double(v) => Some(v as f32),
            _ => None,
        }
    }

    /// String view.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Nbt::String(s) => Some(s),
            _ => None,
        }
    }

    /// List view.
    pub fn as_list(&self) -> Option<&[Nbt]> {
        match self {
            Nbt::List(items) => Some(items),
            _ => None,
        }
    }
}

/// Write one NBT value as an anonymous-root network compound (the root
/// MUST be a compound — the network format has no named root).
pub fn write_network_nbt(value: &Nbt, out: &mut Vec<u8>) -> io::Result<()> {
    let entries = match value {
        Nbt::Compound(entries) => entries,
        _ => return Err(corrupt("network NBT root must be a compound")),
    };
    // the root tag is written HERE (write_payload emits its own tag for
    // nested values — the root must not get a second one)
    out.push(0x0A);
    write_compound_contents(entries, out, 0)
}

/// Write one BARE network NBT tag (the bare reader's mirror — the
/// entity-metadata test builders and future clientbound component
/// surfaces emit through it). The tag id byte + payload: exactly what
/// a compound entry writer emits between the name and the next entry,
/// standing alone.
pub fn write_bare_nbt(value: &Nbt, out: &mut Vec<u8>) -> io::Result<()> {
    out.push(tag_of(value));
    write_payload(value, out, 0)
}

/// Write one NBT value in the named-root FILE format vanilla's
/// `NbtIo.write(CompoundTag, Path)` speaks (javap the 26.2 jar — the
/// writeByte the root tag's own id keeps, the writeUTF the root name
/// pairs — hotbar.nbt's `""` root the empty string names). The file
/// bytes: tag id + u16-prefixed name + payload — the read_file_nbt
/// reader's mirror walk.
pub fn write_file_nbt(root_name: &str, value: &Nbt, out: &mut Vec<u8>) -> io::Result<()> {
    out.push(tag_of(value));
    write_nbt_string(root_name, out)?;
    write_payload(value, out, 0)
}

/// Read one named-root FILE value (the `NbtIo.read` walk's mirror):
/// the tag id byte, the u16-prefixed root name, the payload. The
/// (name, value) pair returns — the hotbar.nbt lane the root's name
/// never reads, the honest pair the caller keeps.
pub fn read_file_nbt(r: &mut &[u8]) -> io::Result<(String, Nbt)> {
    let tag = read_u8(r)?;
    let name = read_nbt_string(r)?;
    let value = read_payload(r, tag, 0)?;
    Ok((name, value))
}

// ---------------------------------------------------------------------------
// reader internals
// ---------------------------------------------------------------------------

fn read_compound_contents(r: &mut &[u8], depth: u32) -> io::Result<Nbt> {
    let depth = depth + 1;
    if depth > MAX_DEPTH {
        return Err(corrupt("nbt: nesting deeper than 128"));
    }
    let mut entries = Vec::new();
    loop {
        let tag = read_u8(r)?;
        if tag == 0x00 {
            return Ok(Nbt::Compound(entries));
        }
        let name = read_nbt_string(r)?;
        let value = read_payload(r, tag, depth)?;
        entries.push((name, value));
    }
}

fn read_payload(r: &mut &[u8], tag: u8, depth: u32) -> io::Result<Nbt> {
    Ok(match tag {
        0x01 => Nbt::Byte(read_u8(r)? as i8),
        0x02 => Nbt::Short(read_i16(r)?),
        0x03 => Nbt::Int(read_i32(r)?),
        0x04 => Nbt::Long(read_i64(r)?),
        0x05 => Nbt::Float(f32::from_be_bytes(read_arr(r)?)),
        0x06 => Nbt::Double(f64::from_be_bytes(read_arr(r)?)),
        0x07 => {
            let n = read_i32(r)?;
            let n = usize::try_from(n).map_err(|_| corrupt("nbt: negative byte array"))?;
            let mut buf = vec![0u8; n];
            r.read_exact(&mut buf)?;
            Nbt::ByteArray(buf)
        }
        0x08 => Nbt::String(read_nbt_string(r)?),
        0x09 => {
            let inner = read_u8(r)?;
            let n = read_i32(r)?;
            let n = usize::try_from(n).map_err(|_| corrupt("nbt: negative list length"))?;
            let mut items = Vec::with_capacity(n.min(1024));
            for _ in 0..n {
                items.push(read_payload(r, inner, depth + 1)?);
            }
            Nbt::List(items)
        }
        0x0A => read_compound_contents(r, depth)?,
        0x0B => {
            let n = read_i32(r)?;
            let n = usize::try_from(n).map_err(|_| corrupt("nbt: negative int array"))?;
            let mut items = Vec::with_capacity(n.min(1024));
            for _ in 0..n {
                items.push(read_i32(r)?);
            }
            Nbt::IntArray(items)
        }
        0x0C => {
            let n = read_i32(r)?;
            let n = usize::try_from(n).map_err(|_| corrupt("nbt: negative long array"))?;
            let mut items = Vec::with_capacity(n.min(1024));
            for _ in 0..n {
                items.push(read_i64(r)?);
            }
            Nbt::LongArray(items)
        }
        other => return Err(corrupt(&format!("nbt: unknown tag id 0x{other:02X}"))),
    })
}

fn read_nbt_string(r: &mut &[u8]) -> io::Result<String> {
    let n = read_u16(r)? as usize;
    if r.len() < n {
        return Err(corrupt("nbt: truncated string"));
    }
    let (bytes, rest) = r.split_at(n);
    *r = rest;
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

fn read_u8(r: &mut &[u8]) -> io::Result<u8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u16(r: &mut &[u8]) -> io::Result<u16> {
    Ok(u16::from_be_bytes(read_arr(r)?))
}

fn read_i16(r: &mut &[u8]) -> io::Result<i16> {
    Ok(i16::from_be_bytes(read_arr(r)?))
}

fn read_i32(r: &mut &[u8]) -> io::Result<i32> {
    Ok(i32::from_be_bytes(read_arr(r)?))
}

fn read_i64(r: &mut &[u8]) -> io::Result<i64> {
    Ok(i64::from_be_bytes(read_arr(r)?))
}

fn read_arr<const N: usize>(r: &mut &[u8]) -> io::Result<[u8; N]> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

// ---------------------------------------------------------------------------
// writer internals (the reader's mirror)
// ---------------------------------------------------------------------------

fn write_payload(value: &Nbt, out: &mut Vec<u8>, depth: u32) -> io::Result<()> {
    if depth > MAX_DEPTH {
        return Err(corrupt("nbt: nesting deeper than 128"));
    }
    // PAYLOAD ONLY — no tag byte (compound entries and list elements are
    // named/tagged by their CONTAINER; the container writes the tag)
    match value {
        Nbt::Byte(v) => out.push(*v as u8),
        Nbt::Short(v) => out.extend_from_slice(&v.to_be_bytes()),
        Nbt::Int(v) => out.extend_from_slice(&v.to_be_bytes()),
        Nbt::Long(v) => out.extend_from_slice(&v.to_be_bytes()),
        Nbt::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
        Nbt::Double(v) => out.extend_from_slice(&v.to_be_bytes()),
        Nbt::ByteArray(bytes) => {
            out.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
            out.extend_from_slice(bytes);
        }
        Nbt::String(s) => write_nbt_string(s, out)?,
        Nbt::List(items) => {
            // inner tag: derived from the first item; the conventional
            // TAG_End for an empty list (documented tradeoff)
            let inner = match items.first() {
                Some(Nbt::Byte(_)) => 0x01,
                Some(Nbt::Short(_)) => 0x02,
                Some(Nbt::Int(_)) => 0x03,
                Some(Nbt::Long(_)) => 0x04,
                Some(Nbt::Float(_)) => 0x05,
                Some(Nbt::Double(_)) => 0x06,
                Some(Nbt::ByteArray(_)) => 0x07,
                Some(Nbt::String(_)) => 0x08,
                Some(Nbt::List(_)) => 0x09,
                Some(Nbt::Compound(_)) => 0x0A,
                Some(Nbt::IntArray(_)) => 0x0B,
                Some(Nbt::LongArray(_)) => 0x0C,
                None => 0x00,
            };
            out.push(inner);
            out.extend_from_slice(&(items.len() as i32).to_be_bytes());
            for item in items {
                write_payload(item, out, depth + 1)?;
            }
        }
        Nbt::Compound(entries) => write_compound_contents(entries, out, depth + 1)?,
        Nbt::IntArray(items) => {
            out.extend_from_slice(&(items.len() as i32).to_be_bytes());
            for v in items {
                out.extend_from_slice(&v.to_be_bytes());
            }
        }
        Nbt::LongArray(items) => {
            out.extend_from_slice(&(items.len() as i32).to_be_bytes());
            for v in items {
                out.extend_from_slice(&v.to_be_bytes());
            }
        }
    }
    Ok(())
}

/// The tag id byte for one value.
fn tag_of(value: &Nbt) -> u8 {
    match value {
        Nbt::Byte(_) => 0x01,
        Nbt::Short(_) => 0x02,
        Nbt::Int(_) => 0x03,
        Nbt::Long(_) => 0x04,
        Nbt::Float(_) => 0x05,
        Nbt::Double(_) => 0x06,
        Nbt::ByteArray(_) => 0x07,
        Nbt::String(_) => 0x08,
        Nbt::List(_) => 0x09,
        Nbt::Compound(_) => 0x0A,
        Nbt::IntArray(_) => 0x0B,
        Nbt::LongArray(_) => 0x0C,
    }
}

fn write_compound_contents(entries: &[(String, Nbt)], out: &mut Vec<u8>, depth: u32) -> io::Result<()> {
    if depth > MAX_DEPTH {
        return Err(corrupt("nbt: nesting deeper than 128"));
    }
    for (name, value) in entries {
        // every named entry: tag id + u16 name + payload
        out.push(tag_of(value));
        write_nbt_string(name, out)?;
        write_payload(value, out, depth)?;
    }
    out.push(0x00); // end of compound
    Ok(())
}

fn write_nbt_string(s: &str, out: &mut Vec<u8>) -> io::Result<()> {
    let bytes = s.as_bytes();
    let n = u16::try_from(bytes.len())
        .map_err(|_| corrupt("nbt: string longer than 65535 bytes"))?;
    out.extend_from_slice(&n.to_be_bytes());
    out.extend_from_slice(bytes);
    Ok(())
}

/// The codebase's packet-shape error (same style as play.rs).
fn corrupt(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A registry-shaped tree exercising all 13 tag types.
    fn registry_tree() -> Nbt {
        Nbt::Compound(vec![
            ("type".to_string(), Nbt::String("minecraft:dimension_type".into())),
            (
                "value".to_string(),
                Nbt::List(vec![
                    Nbt::Compound(vec![
                        ("name".to_string(), Nbt::String("minecraft:overworld".into())),
                        ("id".to_string(), Nbt::Int(0)),
                        (
                            "element".to_string(),
                            Nbt::Compound(vec![
                                ("min_y".to_string(), Nbt::Int(-64)),
                                ("height".to_string(), Nbt::Int(384)),
                                ("ultrawarm".to_string(), Nbt::Byte(0)),
                                ("ambient_light".to_string(), Nbt::Float(0.0)),
                                ("infiniburn".to_string(), Nbt::String("#minecraft:infiniburn_overworld".into())),
                                (
                                    "monster_spawn_light_level".to_string(),
                                    Nbt::Compound(vec![
                                        ("type".to_string(), Nbt::String("uniform".into())),
                                        ("value".to_string(), Nbt::Compound(vec![
                                            ("max_inclusive".to_string(), Nbt::Int(7)),
                                            ("min_inclusive".to_string(), Nbt::Int(0)),
                                        ])),
                                    ]),
                                ),
                            ]),
                        ),
                    ]),
                    Nbt::Compound(vec![
                        ("name".to_string(), Nbt::String("minecraft:the_nether".into())),
                        ("id".to_string(), Nbt::Int(2)),
                        (
                            "element".to_string(),
                            Nbt::Compound(vec![
                                ("min_y".to_string(), Nbt::Int(0)),
                                ("height".to_string(), Nbt::Int(256)),
                                ("ultrawarm".to_string(), Nbt::Byte(1)),
                                (
                                    "fixed_time".to_string(),
                                    Nbt::Long(18_000),
                                ),
                            ]),
                        ),
                    ]),
                ]),
            ),
            (
                "meta".to_string(),
                Nbt::Compound(vec![
                    ("flags".to_string(), Nbt::ByteArray(vec![0xDE, 0xAD, 0xBE, 0xEF])),
                    ("deltas".to_string(), Nbt::IntArray(vec![-1, 0, 1])),
                    ("seeds".to_string(), Nbt::LongArray(vec![i64::MIN, i64::MAX])),
                    ("d".to_string(), Nbt::Double(0.5)),
                    ("nested".to_string(), Nbt::List(vec![Nbt::List(vec![Nbt::Byte(1), Nbt::Byte(2)])])),
                ]),
            ),
        ])
    }

    #[test]
    fn roundtrip_survives_a_full_registry_tree() {
        let tree = registry_tree();
        let mut buf = Vec::new();
        write_network_nbt(&tree, &mut buf).expect("write");
        let mut cursor: &[u8] = &buf;
        let back = read_network_nbt(&mut cursor).expect("read");
        assert!(cursor.is_empty(), "the reader consumes exactly the value");
        assert_eq!(back, tree, "bit-faithful roundtrip (non-empty lists)");
    }

    /// The BARE-tag pair: a component's shapes — a TAG_String, a
    /// TAG_Compound, a TAG_List root — round-trip through
    /// `write_bare_nbt`/`read_bare_nbt` with no root framing. The
    /// closing pin is the TRAP the two entry points encode: a bare
    /// reader handed the root-compound shape does NOT error — 0x0A is
    /// a legal compound tag id and the named contents read as garbage
    /// entries — so the chunk path's named roots and the metadata's
    /// bare components MUST go through their own readers.
    #[test]
    fn bare_tags_roundtrip_without_root_framing() {
        for tree in [
            Nbt::String("Duke".into()),
            Nbt::Compound(vec![
                ("text".into(), Nbt::String("A".into())),
                ("bold".into(), Nbt::Byte(1)),
            ]),
            Nbt::List(vec![Nbt::String("a".into()), Nbt::String("b".into())]),
            Nbt::Int(-42),
        ] {
            let mut buf = Vec::new();
            write_bare_nbt(&tree, &mut buf).expect("write");
            let mut cursor: &[u8] = &buf;
            let back = read_bare_nbt(&mut cursor).expect("read");
            assert!(cursor.is_empty(), "the bare reader consumes exactly the tag");
            assert_eq!(back, tree);
        }
        // the trap, pinned: 0x0A through the bare reader is a legal
        // (empty) compound — no refusal, just the WRONG entry point
        let mut cursor: &[u8] = &[0x0A, 0x00];
        assert_eq!(read_bare_nbt(&mut cursor).unwrap(), Nbt::Compound(vec![]));
    }

    #[test]
    fn accessors_answer_registry_questions() {
        let tree = registry_tree();
        let dims = tree.get("value").and_then(Nbt::as_list).expect("value list");
        let overworld = &dims[0];
        assert_eq!(overworld.get("name").and_then(Nbt::as_str), Some("minecraft:overworld"));
        let element = overworld.get("element").expect("element");
        assert_eq!(element.get("min_y").and_then(Nbt::as_i32), Some(-64));
        assert_eq!(element.get("height").and_then(Nbt::as_i32), Some(384));
        assert_eq!(element.get("ultrawarm").and_then(Nbt::as_i32), Some(0), "byte widens");
        assert_eq!(element.get("absent").and_then(Nbt::as_i32), None);
        assert_eq!(tree.get("min_y"), None, "a deep key is not found at the wrong level");
        let nether = &dims[1];
        assert_eq!(
            nether.get("element").and_then(|e| e.get("fixed_time")).and_then(Nbt::as_i32),
            Some(18_000),
            "long narrows when it fits"
        );
    }

    #[test]
    fn reader_rejects_garbage_shapes() {
        // wrong root tag
        let mut bad: &[u8] = &[0x03, 0x00, 0x00];
        assert!(read_network_nbt(&mut bad).is_err());
        // unknown tag id inside a compound
        let junk = vec![0x0A, 0x7F, 0x00, 0x01, b'x', 0x00];
        let mut cursor: &[u8] = &junk;
        assert!(read_network_nbt(&mut cursor).is_err());
        // truncated string payload
        let cut = vec![0x0A, 0x08, 0x00, 0x01, b's', 0x00, 0x05, b'o', b'n', b'l'];
        let mut cursor: &[u8] = &cut;
        assert!(read_network_nbt(&mut cursor).is_err());
        // writer refuses a non-compound root
        let mut out = Vec::new();
        assert!(write_network_nbt(&Nbt::Int(1), &mut out).is_err());
        // hostile nesting cannot blow the stack
        let mut deep = Vec::new();
        for _ in 0..300 {
            deep.push(0x0A);
        }
        deep.push(0x00);
        let mut cursor: &[u8] = &deep;
        assert!(read_network_nbt(&mut cursor).is_err());
    }

    #[test]
    fn empty_list_roundtrips_through_the_conventional_end_tag() {
        let tree = Nbt::Compound(vec![(
            "empty".to_string(),
            Nbt::List(vec![]),
        )]);
        let mut buf = Vec::new();
        write_network_nbt(&tree, &mut buf).expect("write");
        assert_eq!(buf[1], 0x09, "list tag");
        assert_eq!(buf[2], 0x00, "empty list declares TAG_End");
        let mut cursor: &[u8] = &buf;
        let back = read_network_nbt(&mut cursor).expect("read");
        assert_eq!(back, tree);
    }

    #[test]
    fn file_nbt_the_named_root_wears() {
        // THE BYTES THE FILE FORMAT KEEPS (javap NbtIo.write — the
        // writeByte the tag id keeps, the writeUTF the name pairs): the
        // 0x0A root, the u16 "" name, the payload
        let tree = Nbt::Compound(vec![("DataVersion".to_string(), Nbt::Int(4903))]);
        let mut buf = Vec::new();
        write_file_nbt("", &tree, &mut buf).expect("write");
        assert_eq!(buf[0], 0x0A, "root compound tag");
        assert_eq!(buf[1], 0x00, "root name length high byte");
        assert_eq!(buf[2], 0x00, "root name length low byte");
        assert_eq!(buf[3], 0x03, "the entry's Int tag the container writes first");
        assert_eq!(&buf[4..6], &[0x00, 0x0B], "entry name length");
        assert_eq!(&buf[6..17], b"DataVersion", "entry name");
        assert_eq!(&buf[17..21], &4903_i32.to_be_bytes(), "the version Int");
        // THE READ MIRROR (the NbtIo.read walk) — the pair returns
        let mut cursor: &[u8] = &buf;
        let (name, back) = read_file_nbt(&mut cursor).expect("read");
        assert_eq!(name, "", "the hotbar.nbt root the empty string names");
        assert_eq!(back, tree);
        // THE NAMED ROOT (a foreign file's own name the honest pair keeps)
        let mut buf2 = Vec::new();
        write_file_nbt("SomeServer", &tree, &mut buf2).expect("write");
        let mut cursor: &[u8] = &buf2;
        let (name2, back2) = read_file_nbt(&mut cursor).expect("read");
        assert_eq!(name2, "SomeServer");
        assert_eq!(back2, tree);
    }
}
