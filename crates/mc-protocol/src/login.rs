//! Login (protocol state 2): the offline-mode sequence the client speaks
//! to join a server — handshake(next_state=2) → Login Start → [Set
//! Compression] → Login Success | Login Disconnect.
//!
//! Payload shapes javap-proven against the official 26.2 jar (the ids
//! are GENERATED — see the clientbound module docs for the drift story;
//! the SHAPES below replaced wiki-era guesses the same way):
//! - Login Start (serverbound 0x00): Name (String 16) + UUID (16 bytes)
//!   — ServerboundHelloPacket writes name THEN uuid (bytecode-proven
//!   order). The vanilla client sends the OFFLINE profile UUID in
//!   offline mode: `UUID.nameUUIDFromBytes("OfflinePlayer:" + name)` —
//!   MD5 with version 3 + IETF variant bits (reimplemented here, no
//!   crypto dependency).
//! - Set Compression (clientbound 0x03): Threshold (VarInt) — both sides
//!   switch to the compressed envelope from the NEXT packet on.
//! - Login Success (clientbound 0x02): GameProfile(uuid, name,
//!   properties) + sessionId — ClientboundLoginFinishedPacket is the
//!   record (GameProfile, UUID sessionId) composited as
//!   ByteBufCodecs.GAME_PROFILE then UUIDUtil.STREAM_CODEC, and
//!   GAME_PROFILE = composite(UUID, String(16), PropertyMap): a VarInt
//!   property count (max 16) then per property name (String 64), value
//!   (String 32767), nullable signature (Bool + String). The wiki-era
//!   "UUID + Username" shape was one full field pair SHORT — a real
//!   26.2 server sends at least the empty property list (VarInt 0) and
//!   a 16-byte session id after the username; the old parser only
//!   survived because it ignored trailing bytes.
//! - Login Disconnect (clientbound 0x00): Reason (chat JSON string).
//!
//! Online-mode servers answer with Encryption Request (0x01) instead of
//! success — implementing it would need the session-server RSA/AES dance;
//! this layer rejects it explicitly (offline-mode servers only).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};

use crate::framing::{encode_handshake, encode_login_start, PacketConn};
use crate::varint::{read_string, read_varint, write_string, write_varint};
use crate::{DEFAULT_TIMEOUT_SECS, NEXT_STATE_LOGIN};

/// Clientbound login packet ids, GENERATED from the official jar's REAL
/// bootstrapped login protocol (`mc2rs-gen dump-packet-ids` binds the
/// vanilla LoginProtocols and dumps the live dispatch codec's id map).
/// The runtime facts for 26.2: the four ids the wiki-era table carried
/// are numerically right, but the vanilla names moved on — disconnect is
/// login_disconnect, the encryption request is "hello", login success is
/// "login_finished" and set compression is "login_compression" — and
/// the flow grew custom_query (4) + cookie_request (5) past them. The
/// public names below are the client's own vocabulary, aliased onto the
/// generated truth.
pub mod clientbound {
    pub use crate::generated::packet_ids::{
        LCB_HELLO as ENCRYPTION_REQUEST, LCB_LOGIN_COMPRESSION as SET_COMPRESSION,
        LCB_LOGIN_DISCONNECT as DISCONNECT, LCB_LOGIN_FINISHED as LOGIN_SUCCESS,
    };
}

/// One authenticated profile property from Login Success (the `textures`
/// entry carries the skin/cape — the surface a later run will decode).
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    /// The signed-property signature (present only for Mojang-signed
    /// entries; offline servers send none).
    pub signature: Option<String>,
}

/// How a login attempt ended.
#[derive(Debug, Clone, PartialEq)]
pub enum LoginOutcome {
    /// The server accepted the profile (Login Success).
    Success {
        /// The server-assigned profile UUID (equals the requested one in
        /// offline mode).
        uuid: [u8; 16],
        username: String,
        /// The profile properties (empty on offline servers; the signed
        /// `textures` entry is what a real account carries).
        properties: Vec<ProfileProperty>,
        /// The server-minted session UUID (the second record component
        /// of the 26.2 login_finished — new vs the wiki-era shape).
        session: [u8; 16],
    },
    /// The server refused the profile (Login Disconnect); carries the
    /// flattened chat-component reason.
    Disconnected { reason: String },
}

// ---------------------------------------------------------------------------
// offline profile UUID (vanilla NameUUIDFromBytes)
// ---------------------------------------------------------------------------

/// MD5 (RFC 1321) — the only hash vanilla needs for offline profile UUIDs.
/// Implemented locally so the protocol layer stays dependency-free.
fn md5(input: &[u8]) -> [u8; 16] {
    const S: [usize; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, //
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, //
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, //
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;
    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, word) in m.iter_mut().enumerate() {
            *word = u32::from_le_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f2 = f
                .wrapping_add(a)
                .wrapping_add(K[i])
                .wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f2.rotate_left(S[i] as u32));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..16].copy_from_slice(&d0.to_le_bytes());
    out
}

/// The offline-mode profile UUID, exactly like vanilla
/// `UUID.nameUUIDFromBytes(("OfflinePlayer:" + name).getBytes(UTF_8))`:
/// MD5 digest with version 3 and IETF (RFC 4122) variant bits.
pub fn offline_uuid(username: &str) -> [u8; 16] {
    let mut h = md5(format!("OfflinePlayer:{username}").as_bytes());
    h[6] = (h[6] & 0x0f) | 0x30; // version 3
    h[8] = (h[8] & 0x3f) | 0x80; // IETF variant
    h
}

// ---------------------------------------------------------------------------
// packet builders (clientbound shapes — used by the mock server in tests to
// exercise the client path against both framing modes)
// ---------------------------------------------------------------------------

/// Set Compression (0x03): Threshold VarInt.
pub fn encode_set_compression(threshold: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, clientbound::SET_COMPRESSION);
    write_varint(&mut p, threshold);
    p
}

/// Login Success (0x02): the full 26.2 shape — UUID + Username + EMPTY
/// property list (VarInt 0) + a zero session UUID. Test-fixture builder
/// (the production client only PARSES this packet); a real vanilla
/// server mints a random session id, offline servers carry no
/// properties, so this is the honest minimal server answer.
pub fn encode_login_success(uuid: &[u8; 16], username: &str) -> Vec<u8> {
    encode_login_success_with_properties(uuid, username, &[])
}

/// Login Success with an EXPLICIT property list — the mock-server shape
/// a profile-metadata test needs (the `textures` entry a real account
/// carries). `props` = (name, value) pairs, no signatures (offline
/// servers never sign).
pub fn encode_login_success_with_properties(
    uuid: &[u8; 16],
    username: &str,
    props: &[(&str, &str)],
) -> Vec<u8> {
    let mut p = Vec::with_capacity(48 + username.len());
    write_varint(&mut p, clientbound::LOGIN_SUCCESS);
    p.extend_from_slice(uuid);
    write_string(&mut p, username);
    write_varint(&mut p, props.len() as i32);
    for (name, value) in props {
        write_string(&mut p, name);
        write_string(&mut p, value);
        // the signature flag: absent (Bool 0) — unsigned entries
        p.push(0);
    }
    p.extend_from_slice(&[0u8; 16]); // session id
    p
}

// ---------------------------------------------------------------------------
// the profile's skin model (the textures property's metadata)
// ---------------------------------------------------------------------------

/// Standard-alphabet base64 decode (RFC 4648, with padding) — local so
/// the protocol layer stays dependency-free, like the MD5 above. `None`
/// on any malformed input (length, padding, alphabet) — profile data is
/// untrusted and a decode failure means "no opinion", never a panic.
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    fn val(b: u8) -> Option<u32> {
        match b {
            b'A'..=b'Z' => Some((b - b'A') as u32),
            b'a'..=b'z' => Some((b - b'a') as u32 + 26),
            b'0'..=b'9' => Some((b - b'0') as u32 + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let raw = input.as_bytes();
    // strip newlines/whitespace (real profile payloads may wrap)
    let bytes: Vec<u8> = raw.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for chunk in bytes.chunks_exact(4) {
        let pad = chunk.iter().rev().take_while(|&&b| b == b'=').count();
        if pad > 2 {
            return None;
        }
        let mut acc = 0u32;
        for (i, &b) in chunk.iter().enumerate() {
            if b == b'=' {
                // '=' is legal only in the last two positions
                if i < 2 {
                    return None;
                }
            } else {
                acc |= val(b)? << (18 - 6 * i);
            }
        }
        for k in 0..3 {
            if k < 3 - pad {
                out.push(((acc >> (16 - 8 * k)) & 0xFF) as u8);
            }
        }
    }
    Some(out)
}

/// The skin model the login profile's `textures` property names — the
/// slim/wide selection a real account carries (vanilla reads
/// `SKIN.metadata.model` out of the base64 property value; `"slim"` is
/// the only value that changes the first-person arm, everything else is
/// the classic wide model). `None` = the wide default: no textures
/// property, no SKIN entry, no metadata, an unknown model value, or ANY
/// malformed input (the profile data is untrusted — a decode/parse
/// failure is "no opinion", never a panic and never a guess).
pub fn profile_skin_model(properties: &[ProfileProperty]) -> Option<String> {
    let textures = properties.iter().find(|p| p.name == "textures")?;
    let bytes = base64_decode(&textures.value)?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    let model = json
        .get("SKIN")?
        .get("metadata")?
        .get("model")?
        .as_str()?
        .to_string();
    // the only value the client acts on; anything else is the classic
    if model == "slim" {
        Some(model)
    } else {
        None
    }
}

/// Test-support base64 ENCODER (standard alphabet + padding) — the
/// fixtures a profile-metadata mock needs (the production side only
/// DECODES; the decoder is pinned by the RFC vectors in the tests).
#[cfg(test)]
pub(crate) fn test_base64_encode(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

/// Login Disconnect (0x00): Reason (chat JSON string).
pub fn encode_login_disconnect(reason_json: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(8 + reason_json.len());
    write_varint(&mut p, clientbound::DISCONNECT);
    write_string(&mut p, reason_json);
    p
}

// ---------------------------------------------------------------------------
// packet parsers
// ---------------------------------------------------------------------------

/// Parse Set Compression payload ("id 0x03" already consumed by caller? no —
/// these helpers take the packet id + body: the driver reads one framed
/// packet, then dispatches on its first VarInt). Each helper consumes the
/// FULL payload including the packet id.
pub fn parse_set_compression(payload: &[u8]) -> Result<i32> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_COMPRESSION,
        "not a set-compression packet (0x{id:02X})"
    );
    let threshold = read_varint(&mut c)?;
    Ok(threshold)
}

/// Parse Login Success payload — the javap-proven 26.2 shape: 16-byte
/// UUID + username + property list (VarInt count, max 16, then per
/// entry name/value/nullable-signature) + 16-byte session UUID.
/// Strict on the trailing session id: the wiki-era parser silently
/// ignored it, and a payload that ends early is protocol drift worth
/// erroring on, the way vanilla's own decoder would refuse it.
pub fn parse_login_success(payload: &[u8]) -> Result<([u8; 16], String, Vec<ProfileProperty>, [u8; 16])> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::LOGIN_SUCCESS,
        "not a login-success packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 16, "login success truncated (uuid)");
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&c[..16]);
    c = &c[16..];
    let username = read_string(&mut c)?;
    let count = read_varint(&mut c)?;
    anyhow::ensure!(count <= 16, "login success carries {count} properties (max 16)");
    let mut properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name = read_string(&mut c)?;
        let value = read_string(&mut c)?;
        anyhow::ensure!(!c.is_empty(), "login success truncated (property signature flag)");
        let signed = c[0] != 0;
        c = &c[1..];
        let signature = if signed { Some(read_string(&mut c)?) } else { None };
        properties.push(ProfileProperty { name, value, signature });
    }
    anyhow::ensure!(c.len() >= 16, "login success truncated (session id)");
    let mut session = [0u8; 16];
    session.copy_from_slice(&c[..16]);
    c = &c[16..];
    anyhow::ensure!(c.is_empty(), "trailing bytes after the login success session id");
    Ok((uuid, username, properties, session))
}

/// Parse Login Disconnect payload: the chat-JSON reason, flattened to plain
/// text (raw string kept when the JSON does not parse).
pub fn parse_login_disconnect(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::DISCONNECT,
        "not a login-disconnect packet (0x{id:02X})"
    );
    let reason = read_string(&mut c)?;
    let flat = serde_json::from_str::<serde_json::Value>(&reason)
        .map(|v| crate::status::extract_text(&v))
        .unwrap_or(reason);
    Ok(flat)
}

// ---------------------------------------------------------------------------
// the login driver
// ---------------------------------------------------------------------------

/// Run the offline-mode login over an established stream. The handshake
/// carries the REAL protocol version — the generated
/// `mc_protocol::generated::pack_claim::PROTOCOL_VERSION` (776 for 26.2;
/// the manifest emitter REFUSES a jar that does not state the number, so
/// it can never be the -1 "unknown" the build_info stamp may legally
/// carry) — status pings tolerate 0, login does not.
pub fn login<S: Read + Write>(
    stream: S,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
    uuid: &[u8; 16],
) -> Result<LoginOutcome> {
    let mut conn = PacketConn::new(stream);
    conn.send_packet(&encode_handshake(protocol_version, host, port, NEXT_STATE_LOGIN))?;
    login_conn(&mut conn, username, uuid)
}

/// Run the login exchange on an ALREADY-handshaked connection (the caller
/// keeps ownership of the `PacketConn`, so the configuration driver can
/// continue on the same wire after Login Success — the vanilla client
/// never reopens the socket between login and configuration).
pub fn login_conn<S: Read + Write>(
    conn: &mut PacketConn<S>,
    username: &str,
    uuid: &[u8; 16],
) -> Result<LoginOutcome> {
    conn.send_packet(&encode_login_start(username, *uuid))?;

    loop {
        let pkt = conn.read_packet().context("read login packet")?;
        anyhow::ensure!(!pkt.is_empty(), "empty login packet");
        let mut c: &[u8] = &pkt;
        let id = read_varint(&mut c)?;
        match id {
            clientbound::SET_COMPRESSION => {
                let threshold = parse_set_compression(&pkt)?;
                anyhow::ensure!(threshold >= 0, "negative compression threshold");
                conn.enable_compression(threshold);
            }
            clientbound::LOGIN_SUCCESS => {
                let (uuid, username, properties, session) = parse_login_success(&pkt)?;
                return Ok(LoginOutcome::Success { uuid, username, properties, session });
            }
            clientbound::DISCONNECT => {
                let reason = parse_login_disconnect(&pkt)?;
                return Ok(LoginOutcome::Disconnected { reason });
            }
            clientbound::ENCRYPTION_REQUEST => {
                return Err(anyhow!(
                    "server requested encryption (online mode) — this client logs in to \
                     offline-mode servers only"
                ));
            }
            other => {
                return Err(anyhow!("unexpected packet 0x{other:02X} during login"));
            }
        }
    }
}

/// Connect + log in with the vanilla offline profile UUID for `username`.
pub fn connect_and_login(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<LoginOutcome> {
    let addr = format!("{host}:{port}");
    let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let started = Instant::now();
    let stream = TcpStream::connect(&addr).with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    let _connect_ms = started.elapsed().as_millis();
    login(stream, host, port, protocol_version, username, &offline_uuid(username))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::{TcpListener, TcpStream};
    use std::thread::JoinHandle;

    #[test]
    fn md5_known_vectors() {
        // RFC 1321 test suite
        let hex = |b: &[u8]| -> String { md5(b).iter().map(|x| format!("{x:02x}")).collect() };
        assert_eq!(hex(b""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex(b"a"), "0cc175b9c0f1b6a831c399e269772661");
        assert_eq!(hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(hex(b"message digest"), "f96b697d7cb7938d525a2f31aaf161d0");
        assert_eq!(
            hex(b"abcdefghijklmnopqrstuvwxyz"),
            "c3fcd3d76192e4007dfb496cca67e13b"
        );
    }

    /// The base64 payload a real account's `textures` property carries
    /// (the vanilla shape: signature/timestamp/value). The slim variant
    /// is `SKIN.metadata.model = "slim"`.
    fn textures_value(model: Option<&str>) -> String {
        let metadata = match model {
            Some(m) => format!(r#","metadata":{{"model":"{m}"}}"#),
            None => String::new(),
        };
        let json = format!(r#"{{"SKIN":{{"url":"http://textures.example.net/skin.png"{metadata}}}}}"#);
        // encode with the RFC vector to keep the encoder out of the code
        // under test: a tiny local table-encoder (the decoder is the
        // production side)
        const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let bytes = json.as_bytes();
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [
                chunk[0],
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
            ];
            let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
            out.push(T[(n >> 18) as usize & 63] as char);
            out.push(T[(n >> 12) as usize & 63] as char);
            out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
            out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
        }
        out
    }

    fn prop(name: &str, value: &str) -> ProfileProperty {
        ProfileProperty { name: name.into(), value: value.into(), signature: None }
    }

    #[test]
    fn base64_decode_matches_the_rfc_vectors() {
        // RFC 4648 test vectors
        assert_eq!(base64_decode("Zg=="), Some(b"f".to_vec()));
        assert_eq!(base64_decode("Zm8="), Some(b"fo".to_vec()));
        assert_eq!(base64_decode("Zm9v"), Some(b"foo".to_vec()));
        assert_eq!(base64_decode("Zm9vYg=="), Some(b"foob".to_vec()));
        assert_eq!(base64_decode("Zm9vYmE="), Some(b"fooba".to_vec()));
        assert_eq!(base64_decode("Zm9vYmFy"), Some(b"foobar".to_vec()));
        // malformed shapes refuse (untrusted profile data -> no opinion)
        assert_eq!(base64_decode("A"), None, "length % 4 != 0");
        assert_eq!(base64_decode("A==="), None, "over-padded");
        assert_eq!(base64_decode("=AAA"), None, "leading pad");
        assert_eq!(base64_decode("Zm9v!"), None, "non-alphabet byte");
        // whitespace is tolerated (real payloads may wrap)
        assert_eq!(base64_decode("Zm9v\nYmFy"), Some(b"foobar".to_vec()));
    }

    #[test]
    fn the_profile_skin_model_reads_the_slim_metadata() {
        // the slim shape: SKIN.metadata.model == "slim" (the only value
        // the client acts on)
        assert_eq!(
            profile_skin_model(&[prop("textures", &textures_value(Some("slim")))]),
            Some("slim".to_string())
        );
        // a skins property WITHOUT metadata (or with an unknown model
        // value) is the classic wide default — no opinion, not a guess
        assert_eq!(profile_skin_model(&[prop("textures", &textures_value(None))]), None);
        assert_eq!(
            profile_skin_model(&[prop("textures", &textures_value(Some("classic")))]),
            None,
            "an unknown model value is the wide default"
        );
        // no textures property at all (the offline server shape)
        assert_eq!(profile_skin_model(&[]), None);
        assert_eq!(
            profile_skin_model(&[prop("cape", &textures_value(Some("slim")))]),
            None,
            "the wrong property name contributes nothing"
        );
        // MALFORMED input is "no opinion", never a panic: bad base64,
        // bad JSON, the wrong JSON shape
        assert_eq!(profile_skin_model(&[prop("textures", "!!!not-base64!!!")]), None);
        assert_eq!(profile_skin_model(&[prop("textures", "bm90LWpzb24=")]), None, "valid base64, not JSON");
        let no_skin = base64_of(br#"{"CAPE":{"metadata":{"model":"slim"}}}"#);
        assert_eq!(
            profile_skin_model(&[prop("textures", &no_skin)]),
            None,
            "no SKIN entry"
        );
    }

    /// A local table-encoder for the test fixtures (the decoder is the
    /// production side and is pinned by the RFC vectors above).
    fn base64_of(bytes: &[u8]) -> String {
        const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [
                chunk[0],
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
            ];
            let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
            out.push(T[(n >> 18) as usize & 63] as char);
            out.push(T[(n >> 12) as usize & 63] as char);
            out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
            out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
        }
        out
    }

    #[test]
    fn the_login_success_encoder_roundtrips_properties() {
        // the properties-capable encoder writes what parse_login_success
        // reads back: name + value + the absent signature
        let slim = textures_value(Some("slim"));
        let p = encode_login_success_with_properties(
            &[7u8; 16],
            "mc2rs",
            &[("textures", slim.as_str()), ("cape", "unused")],
        );
        let (uuid, username, properties, _session) =
            parse_login_success(&p).expect("the shaped packet parses");
        assert_eq!(uuid, [7u8; 16]);
        assert_eq!(username, "mc2rs");
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "textures");
        assert_eq!(properties[0].value, slim);
        assert_eq!(properties[0].signature, None, "offline servers never sign");
        assert_eq!(properties[1].name, "cape");
        // and the profile read-through lands on the model
        assert_eq!(profile_skin_model(&properties), Some("slim".to_string()));
        // the 2-arg encoder stays the empty-property shape
        let p2 = encode_login_success(&[7u8; 16], "mc2rs");
        let (_, _, properties2, _) =
            parse_login_success(&p2).expect("the minimal packet parses");
        assert!(properties2.is_empty());
        assert_eq!(profile_skin_model(&properties2), None);
    }

    #[test]
    fn offline_uuid_matches_vanilla_nameuuidfrombytes() {
        // independent truth (python hashlib): md5("OfflinePlayer:<name>"),
        // version 3, IETF variant — the well-known offline UUID of Notch
        let u = offline_uuid("Notch");
        assert_eq!(
            format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}", u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7], u[8], u[9], u[10], u[11], u[12], u[13], u[14], u[15]),
            "b50ad385-829d-3141-a216-7e7d7539ba7f"
        );
        let m = offline_uuid("mc2rs");
        assert_eq!(m[6] >> 4, 3, "version nibble must be 3");
        assert_eq!(m[8] >> 6, 0b10, "variant must be IETF");
    }

    #[test]
    fn login_packet_shapes() {
        let p = encode_set_compression(256);
        assert_eq!(p, vec![0x03, 0x80, 0x02]);
        let (uuid, name, properties, session) =
            parse_login_success(&encode_login_success(&[7u8; 16], "Steve"))
                .expect("parse success");
        assert_eq!(uuid, [7u8; 16]);
        assert_eq!(name, "Steve");
        assert_eq!(properties, vec![], "offline server: no properties");
        assert_eq!(session, [0u8; 16]);
        let reason = parse_login_disconnect(&encode_login_disconnect(
            r#"{"text":"You are ","extra":[{"text":"banned","color":"red"}]}"#,
        ))
        .expect("parse disconnect");
        assert_eq!(reason, "You are banned");
        // a plain (non-JSON) reason survives verbatim
        let raw = parse_login_disconnect(&encode_login_disconnect("bye")).unwrap();
        assert_eq!(raw, "bye");
    }

    #[test]
    fn parsers_reject_wrong_ids() {
        assert!(parse_set_compression(&[0x00, 0x01]).is_err());
        assert!(parse_login_success(&[0x00, 1, 2, 3]).is_err());
        assert!(parse_login_disconnect(&[0x02, 1]).is_err());
    }

    #[test]
    fn login_success_carries_properties_and_the_session_uuid() {
        // the javap-proven 26.2 shape, hand-assembled: GameProfile (uuid,
        // name, ONE signed property) + the session uuid — the exact bytes
        // a real authenticated server sends
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::LOGIN_SUCCESS);
        p.extend_from_slice(&[0xA5u8; 16]); // profile uuid
        write_string(&mut p, "Notch");
        write_varint(&mut p, 1); // one property
        write_string(&mut p, "textures");
        write_string(&mut p, "eyJza2lu");
        p.push(1); // signed
        write_string(&mut p, "sig==");
        p.extend_from_slice(&[0x5Eu8; 16]); // session uuid
        let (uuid, name, props, session) = parse_login_success(&p).expect("full shape parses");
        assert_eq!(uuid, [0xA5u8; 16]);
        assert_eq!(name, "Notch");
        assert_eq!(
            props,
            vec![ProfileProperty {
                name: "textures".into(),
                value: "eyJza2lu".into(),
                signature: Some("sig==".into()),
            }]
        );
        assert_eq!(session, [0x5Eu8; 16]);
        // an unsigned property (offline server with plugins) parses too
        let mut q = Vec::new();
        write_varint(&mut q, clientbound::LOGIN_SUCCESS);
        q.extend_from_slice(&[1u8; 16]);
        write_string(&mut q, "Steve");
        write_varint(&mut q, 1);
        write_string(&mut q, "note");
        write_string(&mut q, "hi");
        q.push(0); // unsigned
        q.extend_from_slice(&[2u8; 16]);
        let (_, _, props, _) = parse_login_success(&q).expect("unsigned property parses");
        assert_eq!(props[0].signature, None);
        // TRUNCATION now errors: the wiki-era parser accepted a uuid+name
        // payload and silently dropped the property list + session id —
        // the drift that masked the 26.2 shape
        let mut t = Vec::new();
        write_varint(&mut t, clientbound::LOGIN_SUCCESS);
        t.extend_from_slice(&[7u8; 16]);
        write_string(&mut t, "Steve");
        assert!(parse_login_success(&t).is_err(), "no property list, no session");
        // a 17th property is vanilla's own readCount cap
        let mut many = Vec::new();
        write_varint(&mut many, clientbound::LOGIN_SUCCESS);
        many.extend_from_slice(&[3u8; 16]);
        write_string(&mut many, "X");
        write_varint(&mut many, 17);
        assert!(parse_login_success(&many).is_err(), "count > 16 rejected");
    }

    /// One mock server scenario: read the client's handshake + login start
    /// (dogfooding the parsers), then answer per the scenario.
    enum Scenario {
        /// compression on, then success (both directions compressed)
        SuccessCompressed,
        /// straight login disconnect with a chat reason
        Disconnect,
        /// a packet id that has no meaning in the login state
        Garbage,
        /// online-mode style encryption request — must be rejected
        Encryption,
    }

    fn spawn_mock(scenario: Scenario) -> (u16, JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().expect("accept");
            let mut srv = PacketConn::new(stream);
            // handshake: id 0x00, protocol, host, port, next_state=2
            let hs = srv.read_packet().expect("handshake");
            let mut c: &[u8] = &hs;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00, "handshake id");
            let proto = read_varint(&mut c).unwrap();
            let host = read_string(&mut c).unwrap();
            let mut port_b = [0u8; 2];
            std::io::Read::read_exact(&mut c, &mut port_b).unwrap();
            let next = read_varint(&mut c).unwrap();
            assert_eq!(next, NEXT_STATE_LOGIN, "handshake must target login");
            // login start: id 0x00, name, uuid
            let start = srv.read_packet().expect("login start");
            let mut c: &[u8] = &start;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00, "login start id");
            let name = read_string(&mut c).unwrap();
            let mut uuid = [0u8; 16];
            std::io::Read::read_exact(&mut c, &mut uuid).unwrap();
            let expected = offline_uuid(&name);
            assert_eq!(uuid, expected, "client must send the offline uuid");
            let log = vec![format!("{proto}/{host}/{}{}:{name}", port_b[0], port_b[1])];

            match scenario {
                Scenario::SuccessCompressed => {
                    srv.send_packet(&encode_set_compression(256)).unwrap();
                    srv.enable_compression(256);
                    srv.send_packet(&encode_login_success(&uuid, &name)).unwrap();
                }
                Scenario::Disconnect => {
                    srv.send_packet(&encode_login_disconnect(
                        r#"{"text":"invalid session"}"#,
                    ))
                    .unwrap();
                }
                Scenario::Garbage => {
                    srv.send_packet(&[0x42]).unwrap();
                }
                Scenario::Encryption => {
                    let mut p = Vec::new();
                    write_varint(&mut p, clientbound::ENCRYPTION_REQUEST);
                    write_string(&mut p, "");
                    p.extend_from_slice(&[0u8; 162]); // server key + nonce filler
                    srv.send_packet(&p).unwrap();
                }
            }
            log
        });
        (port, handle)
    }

    fn client_login(port: u16, scenario_check: impl FnOnce(LoginOutcome)) {
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        // the REAL protocol number shape: login must carry a concrete version
        let out = login(stream, "127.0.0.1", port, 776, "mc2rs", &offline_uuid("mc2rs"))
            .expect("login flow");
        scenario_check(out);
    }

    #[test]
    fn login_success_with_compression_roundtrip() {
        let (port, srv) = spawn_mock(Scenario::SuccessCompressed);
        client_login(port, |out| match out {
            LoginOutcome::Success { username, .. } => assert_eq!(username, "mc2rs"),
            other => panic!("expected success, got {other:?}"),
        });
        assert_eq!(srv.join().unwrap().len(), 1, "server saw a clean exchange");
    }

    #[test]
    fn login_disconnect_carries_flattened_reason() {
        let (port, srv) = spawn_mock(Scenario::Disconnect);
        client_login(port, |out| match out {
            LoginOutcome::Disconnected { reason } => assert_eq!(reason, "invalid session"),
            other => panic!("expected disconnect, got {other:?}"),
        });
        srv.join().unwrap();
    }

    #[test]
    fn login_rejects_garbage_packet_id() {
        let (port, srv) = spawn_mock(Scenario::Garbage);
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let err = login(stream, "127.0.0.1", port, 776, "mc2rs", &offline_uuid("mc2rs"))
            .expect_err("garbage packet must fail the login");
        assert!(format!("{err:#}").contains("unexpected packet 0x42"), "{err:#}");
        srv.join().unwrap();
    }

    #[test]
    fn login_rejects_encryption_request() {
        let (port, srv) = spawn_mock(Scenario::Encryption);
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let err = login(stream, "127.0.0.1", port, 776, "mc2rs", &offline_uuid("mc2rs"))
            .expect_err("encryption request must be rejected");
        assert!(format!("{err:#}").contains("offline-mode"), "{err:#}");
        srv.join().unwrap();
    }

    /// Cursor-based (no sockets) end-to-end framing sanity: the driver
    /// handles the compression envelope switch mid-stream.
    #[test]
    fn compression_switch_applies_to_subsequent_packets() {
        // client-side framing: enable compression from the threshold packet
        let mut conn = PacketConn::new(Cursor::new(Vec::new()));
        assert!(conn.compression_threshold.is_none());
        conn.enable_compression(128);
        assert_eq!(conn.compression_threshold, Some(128));
        // a small payload stays uncompressed under the envelope
        let framed = conn.encode_packet(&[0x01]).unwrap();
        assert_eq!(framed, vec![0x02, 0x00, 0x01]);
    }
}
