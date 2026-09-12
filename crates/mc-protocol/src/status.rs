//! Status (server list ping) — the classic protocol hello-world.
//!
//! Works against every server version because the status handshake accepts
//! `protocol_version = 0`.

use std::net::TcpStream;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use serde_json::Value;

use crate::framing::{encode_handshake, encode_status_request, PacketConn};
use crate::{NEXT_STATE_STATUS, DEFAULT_TIMEOUT_SECS};

#[derive(Debug, Clone)]
pub struct PingResponse {
    /// Protocol number reported by the server.
    pub protocol: i32,
    /// Version name reported by the server, e.g. "26.2".
    pub version_name: String,
    pub players_online: i64,
    pub players_max: i64,
    /// Flattened MOTD text.
    pub motd: String,
    /// Base64 PNG favicon if the server sends one.
    pub favicon: Option<String>,
    pub latency_ms: u128,
    /// The raw status JSON for advanced consumers.
    pub raw: Value,
}

/// Recursively flatten a chat component (description) into plain text.
pub fn extract_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(parts) => parts.iter().map(extract_text).collect::<Vec<_>>().join(""),
        Value::Object(o) => {
            let mut out = String::new();
            if let Some(t) = o.get("text").and_then(|t| t.as_str()) {
                out.push_str(t);
            }
            if let Some(extra) = o.get("extra") {
                out.push_str(&extract_text(extra));
            }
            if out.is_empty() {
                if let Some(t) = o.get("translate").and_then(|t| t.as_str()) {
                    out.push_str(t);
                }
            }
            out
        }
        _ => String::new(),
    }
}

/// Ping a Minecraft server (status request) and collect its info.
pub fn ping(host: &str, port: u16) -> Result<PingResponse> {
    let addr = format!("{host}:{port}");
    let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let started = Instant::now();
    let stream = TcpStream::connect(&addr)
        .with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    let _connect_ms = started.elapsed().as_millis();

    let mut conn = PacketConn::new(stream);
    conn.send_packet(&encode_handshake(0, host, port, NEXT_STATE_STATUS))?;
    conn.send_packet(&encode_status_request())?;

    let response = conn.read_packet()?;
    anyhow::ensure!(!response.is_empty(), "empty status packet");
    let mut cursor: &[u8] = &response;
    let packet_id = crate::varint::read_varint(&mut cursor)?;
    // GENERATED from the official jar's real status protocol
    anyhow::ensure!(
        packet_id == crate::generated::packet_ids::TCB_STATUS_RESPONSE,
        "expected the status response packet, got 0x{packet_id:02X}"
    );
    let str_len = crate::varint::read_varint(&mut cursor)? as usize;
    anyhow::ensure!(cursor.len() >= str_len, "truncated status json");
    let json_bytes = &cursor[..str_len];
    // Trim trailing bytes some servers append; parse leniently.
    let raw: Value = parse_json_lenient(json_bytes)?;

    let version = raw.get("version").cloned().unwrap_or(Value::Null);
    let players = raw.get("players").cloned().unwrap_or(Value::Null);

    let latency_ms = started.elapsed().as_millis();
    Ok(PingResponse {
        protocol: version.get("protocol").and_then(|p| p.as_i64()).unwrap_or(-1) as i32,
        version_name: version
            .get("name")
            .map(extract_text)
            .unwrap_or_else(|| "unknown".into()),
        players_online: players.get("online").and_then(|p| p.as_i64()).unwrap_or(0),
        players_max: players.get("max").and_then(|p| p.as_i64()).unwrap_or(0),
        motd: raw.get("description").map(extract_text).unwrap_or_default(),
        favicon: raw.get("favicon").and_then(|f| f.as_str()).map(String::from),
        latency_ms,
        raw,
    })
}

fn parse_json_lenient(bytes: &[u8]) -> Result<Value> {
    if let Ok(v) = serde_json::from_slice::<Value>(bytes) {
        return Ok(v);
    }
    // Some servers pad the string; try trimming trailing NULs/whitespace.
    let trimmed: Vec<u8> = bytes
        .iter()
        .copied()
        .take_while(|&b| b != 0)
        .collect();
    let s = String::from_utf8_lossy(&trimmed);
    let v = serde_json::from_str::<Value>(s.trim())
        .map_err(|e| anyhow!("status json parse failed: {e}"))?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motd_flattening() {
        let v: Value = serde_json::json!({
            "text": "Hello ",
            "extra": [ { "text": "world", "color": "red" }, "!"]
        });
        assert_eq!(extract_text(&v), "Hello world!");
        assert_eq!(extract_text(&Value::String("plain".into())), "plain");
    }
}
