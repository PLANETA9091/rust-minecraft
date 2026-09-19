//! Packet framing: length-prefixed packets with optional zlib compression
//! (enabled by the "Set Compression" packet during login/status).

use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::varint::{read_varint, write_varint};

/// A connection carrying framed Minecraft packets.
pub struct PacketConn<S: Read + Write> {
    pub stream: S,
    /// Compression threshold in bytes; `None` = compression disabled.
    pub compression_threshold: Option<i32>,
    /// THE BANDWIDTH MONITOR'S OWN COUNTER (the vanilla
    /// BandwidthDebugMonitor.bytesReceived AtomicInteger's own lane —
    /// the Varint21FrameDecoder's `onReceive(frameLen +
    /// VarInt.getByteSize(frameLen))` call the received wire bytes
    /// accumulate: the length varint + the frame the socket
    /// delivered). The tick's own drain zeroes it — see the
    /// session's `take_received_bytes`.
    pub bytes_received: u64,
}

impl<S: Read + Write> PacketConn<S> {
    pub fn new(stream: S) -> Self {
        Self { stream, compression_threshold: None, bytes_received: 0 }
    }

    /// Enable compression with the threshold sent by the server.
    pub fn enable_compression(&mut self, threshold: i32) {
        self.compression_threshold = Some(threshold);
    }

    /// Encode one packet (adds length prefix + compression envelope).
    pub fn encode_packet(&self, payload: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut out = Vec::with_capacity(payload.len() + 8);
        match self.compression_threshold {
            None => {
                write_varint(&mut out, payload.len() as i32);
                out.extend_from_slice(payload);
            }
            Some(threshold) => {
                if (payload.len() as i32) >= threshold {
                    let mut z = ZlibEncoder::new(Vec::new(), Compression::new(6));
                    z.write_all(payload)?;
                    let compressed = z.finish()?;
                    let mut inner = Vec::with_capacity(compressed.len() + 5);
                    write_varint(&mut inner, payload.len() as i32);
                    inner.extend_from_slice(&compressed);
                    write_varint(&mut out, inner.len() as i32);
                    out.extend_from_slice(&inner);
                } else {
                    let mut inner = Vec::with_capacity(payload.len() + 5);
                    write_varint(&mut inner, 0); // uncompressed marker
                    inner.extend_from_slice(payload);
                    write_varint(&mut out, inner.len() as i32);
                    out.extend_from_slice(&inner);
                }
            }
        }
        Ok(out)
    }

    /// Send one packet.
    pub fn send_packet(&mut self, payload: &[u8]) -> std::io::Result<()> {
        let framed = self.encode_packet(payload)?;
        self.stream.write_all(&framed)?;
        self.stream.flush()
    }

    /// Read one packet (blocking), handling the compression envelope.
    pub fn read_packet(&mut self) -> std::io::Result<Vec<u8>> {
        let mut pkt_len = read_varint(&mut self.stream)? as usize;
        if pkt_len == 0 || pkt_len > 8 * 1024 * 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid packet length {pkt_len}"),
            ));
        }
        let mut buf = vec![0u8; pkt_len];
        self.stream.read_exact(&mut buf)?;
        // THE DECODER'S OWN COUNT (the Varint21FrameDecoder's
        // `onReceive(frameLen + VarInt.getByteSize(frameLen))` walk —
        // the frame's payload bytes + the length varint's own size
        // the wire delivered, the monitor's counter the getAndAdd
        // accumulates)
        let varint_size =
            ((32 - (pkt_len as u32).leading_zeros()) as usize).div_ceil(7).max(1);
        self.bytes_received += (varint_size + pkt_len) as u64;

        match self.compression_threshold {
            None => Ok(buf),
            Some(_) => {
                let mut cursor = &buf[..];
                let data_len = read_varint(&mut cursor)? as usize;
                if data_len == 0 {
                    Ok(cursor.to_vec())
                } else {
                    if data_len > 8 * 1024 * 1024 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("uncompressed size too large {data_len}"),
                        ));
                    }
                    let mut z = ZlibDecoder::new(cursor);
                    let mut out = Vec::with_capacity(data_len);
                    z.read_to_end(&mut out)?;
                    pkt_len = out.len();
                    if pkt_len != data_len {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("zlib size mismatch: got {pkt_len}, expected {data_len}"),
                        ));
                    }
                    Ok(out)
                }
            }
        }
    }
}

/// Encode a handshake packet payload (packet id 0x00, GENERATED from the
/// official jar's real handshake protocol — vanilla names it
/// "minecraft:intention").
pub fn encode_handshake(protocol_version: i32, host: &str, port: u16, next_state: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(16 + host.len());
    crate::varint::write_varint(&mut p, crate::generated::packet_ids::HSB_INTENTION);
    crate::varint::write_varint(&mut p, protocol_version);
    crate::varint::write_string(&mut p, host);
    p.extend_from_slice(&port.to_be_bytes());
    crate::varint::write_varint(&mut p, next_state);
    p
}

/// Encode an empty status-request packet payload (packet id 0x00,
/// GENERATED from the official jar's real status protocol).
pub fn encode_status_request() -> Vec<u8> {
    let mut p = Vec::with_capacity(1);
    crate::varint::write_varint(&mut p, crate::generated::packet_ids::TSB_STATUS_REQUEST);
    p
}

/// Encode a login-start packet payload (packet id 0x00, protocol >= 1.20.5
/// shape, GENERATED from the official jar's real login protocol — vanilla
/// names it "minecraft:hello").
pub fn encode_login_start(username: &str, uuid: [u8; 16]) -> Vec<u8> {
    let mut p = Vec::with_capacity(32 + username.len());
    crate::varint::write_varint(&mut p, crate::generated::packet_ids::LSB_HELLO);
    crate::varint::write_string(&mut p, username);
    p.extend_from_slice(&uuid);
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn handshake_bytes() {
        let p = encode_handshake(0, "localhost", 25565, 1);
        // id 0x00, protocol 0, len 9 "localhost", port BE, next_state 1
        assert_eq!(
            p,
            vec![0x00, 0x00, 0x09, b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't', 0x63, 0xDD, 0x01]
        );
    }

    #[test]
    fn login_start_shape() {
        let p = encode_login_start("Steve", [0u8; 16]);
        assert_eq!(p[0], 0x00);
        assert_eq!(p[1], 5);
        assert_eq!(&p[2..7], b"Steve");
        assert_eq!(p.len(), 7 + 16);
    }

    #[test]
    fn framing_uncompressed_roundtrip() {
        let conn = PacketConn::new(Cursor::new(Vec::new()));
        let framed = conn.encode_packet(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(framed, vec![0x03, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn framing_compressed_roundtrip() {
        let payload = vec![7u8; 600];
        let mut writer = PacketConn::new(Cursor::new(Vec::new()));
        writer.enable_compression(256);
        let framed = writer.encode_packet(&payload).unwrap();
        // Read it back through the same framing logic.
        let mut reader = PacketConn::new(Cursor::new(framed));
        reader.enable_compression(256);
        let got = reader.read_packet().unwrap();
        assert_eq!(got, payload);
    }

    #[test]
    fn framing_small_packets_stay_uncompressed() {
        let mut writer = PacketConn::new(Cursor::new(Vec::new()));
        writer.enable_compression(256);
        let framed = writer.encode_packet(&[0x00]).unwrap();
        // marker 0 + payload, length 2
        assert_eq!(framed, vec![0x02, 0x00, 0x00]);
    }

    #[test]
    fn framing_counts_the_received_wire_bytes() {
        // THE MONITOR'S OWN COUNTER (the Varint21FrameDecoder's
        // `onReceive(frameLen + VarInt.getByteSize(frameLen))` walk —
        // the length varint + the frame the read keeps, the
        // accumulation the two packets' pair pins)
        let writer = PacketConn::new(Cursor::new(Vec::new()));
        let f1 = writer.encode_packet(&[0x01, 0x02, 0x03]).unwrap(); // len 3 + 1 varint byte
        let f2 = writer.encode_packet(&vec![7u8; 300]).unwrap(); // len 300 + 2 varint bytes
        let mut reader = PacketConn::new(Cursor::new([f1, f2].concat()));
        assert_eq!(reader.bytes_received, 0);
        let got = reader.read_packet().unwrap();
        assert_eq!(got, vec![0x01, 0x02, 0x03]);
        assert_eq!(reader.bytes_received, 4, "1 varint byte (len 3) + the 3-byte frame");
        let got = reader.read_packet().unwrap();
        assert_eq!(got, vec![7u8; 300]);
        assert_eq!(
            reader.bytes_received,
            306,
            "the pair's accumulation (4 + 302): the getAndAdd law the tick's getAndSet(0) drains"
        );
    }
}
