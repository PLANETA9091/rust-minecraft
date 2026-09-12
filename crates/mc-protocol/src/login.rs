//! Login (protocol state 2): the offline-mode sequence the client speaks
//! to join a server — handshake(next_state=2) → Login Start → [Set
//! Compression] → Login Success | Login Disconnect.
//!
//! Clean-room wiki.vg shapes for 1.20.5+ / 26.x:
//! - Login Start (serverbound 0x00): Name (String 16) + UUID (16 bytes).
//!   The vanilla client sends the OFFLINE profile UUID in offline mode:
//!   `UUID.nameUUIDFromBytes("OfflinePlayer:" + name)` — MD5 with version 3
//!   + IETF variant bits (reimplemented here, no crypto dependency).
//! - Set Compression (clientbound 0x03): Threshold (VarInt) — both sides
//!   switch to the compressed envelope from the NEXT packet on.
//! - Login Success (clientbound 0x02): UUID (16 bytes) + Username (String).
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

/// Clientbound login packet ids (wiki.vg, login state).
mod clientbound {
    pub const DISCONNECT: i32 = 0x00;
    pub const ENCRYPTION_REQUEST: i32 = 0x01;
    pub const LOGIN_SUCCESS: i32 = 0x02;
    pub const SET_COMPRESSION: i32 = 0x03;
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

/// Login Success (0x02): UUID (16 bytes) + Username (String).
pub fn encode_login_success(uuid: &[u8; 16], username: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(24 + username.len());
    write_varint(&mut p, clientbound::LOGIN_SUCCESS);
    p.extend_from_slice(uuid);
    write_string(&mut p, username);
    p
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

/// Parse Login Success payload: 16-byte UUID + username string.
pub fn parse_login_success(payload: &[u8]) -> Result<([u8; 16], String)> {
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
    Ok((uuid, username))
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
/// carries the REAL protocol version (the generated
/// `mc_registry::build::PROTOCOL_VERSION`, 776 for 26.2) — status pings
/// tolerate 0, login does not.
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
                let (uuid, username) = parse_login_success(&pkt)?;
                return Ok(LoginOutcome::Success { uuid, username });
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
        let (uuid, name) = parse_login_success(&encode_login_success(&[7u8; 16], "Steve"))
            .expect("parse success");
        assert_eq!(uuid, [7u8; 16]);
        assert_eq!(name, "Steve");
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
