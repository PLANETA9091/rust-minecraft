#!/usr/bin/env python3
"""palette_oracle.py -- the independent oracle for mc-registry's palette.

A hand-written PNG reader (no PIL) over the REAL client jar: it answers
"what is the average color of this texture rect?" with ZERO shared code
with the Rust chain, so the two decoders can only agree when both honor
the same format facts.

The v0.3.77 round used it to pin stone (126,126,126), dirt (134,96,67)
and grass_block_top (147,147,147) bit-exactly -- and it caught two
fidelity facts the hard way:

1. The grass block SIDE is two quads (#side + the #overlay fringe), and
   the fringe's 135-byte grayscale PNG carries a tRNS chunk -- an oracle
   that ignores tRNS reads (27,27,27) over all 256 pixels instead of
   (155,155,155) over the 45 opaque ones.
2. 26.2's snowy grass keeps grass_block_top on TOP (the snow look is a
   runtime tint, not texels) with grass_block_snow TEXTURE on the sides.

Usage:
  python3 scripts/palette_oracle.py <client.jar> <texture-path-in-jar>
      # prints size + the full-rect opaque average
  python3 scripts/palette_oracle.py <client.jar> <texture-path> x0 y0 x1 y1
      # prints the rect average (uv space, (0,0) = top-left,
      # exclusive upper bound -- the quad_average rect convention)

The average rule mirrors the Rust side (mc-registry palette.rs):
  - a pixel is OPAQUE iff its alpha != 0 (with tRNS honored for
    gray/palette/truecolor sources that carry no alpha channel);
  - the average is the per-channel mean over the opaque pixels;
  - the mean rounds HALF-UP (a .5 boundary rounds away from zero --
    Python's round() is banker's rounding, hence the manual floor(x+0.5)).
"""

import struct
import sys
import zlib


def read_png(data: bytes):
    """Decode a PNG into (width, height, rows) with rows as lists of
    (r, g, b, a) tuples. Supports gray (1/2/4/8/16-bit), palette,
    truecolor and truecolor+alpha, plus tRNS transparency."""
    assert data[:8] == b"\x89PNG\r\n\x1a\n", "not a PNG"
    pos = 8
    idat = b""
    palette = None
    trns = None
    header = None
    while pos < len(data):
        (length,) = struct.unpack(">I", data[pos : pos + 4])
        ctype = data[pos + 4 : pos + 8]
        payload = data[pos + 8 : pos + 8 + length]
        pos += 12 + length
        if ctype == b"IHDR":
            header = struct.unpack(">IIBBBBB", payload)
        elif ctype == b"PLTE":
            palette = [tuple(payload[i : i + 3]) for i in range(0, len(payload), 3)]
        elif ctype == b"tRNS":
            trns = payload
        elif ctype == b"IDAT":
            idat += payload
        elif ctype == b"IEND":
            break
    width, height, depth, color, _, _, interlace = header
    assert interlace == 0, "interlaced PNGs unsupported"
    raw = zlib.decompress(idat)
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[color]
    bpp = max(1, (width * channels * depth + 7) // 8)
    stride = bpp
    # Reconstruct the scanlines (the five PNG filters).
    out = bytearray()
    prev = bytearray(stride)
    off = 0
    for _ in range(height):
        ftype = raw[off]
        off += 1
        line = bytearray(raw[off : off + stride])
        off += stride
        if ftype == 1:  # Sub
            for i in range(bpp, stride):
                line[i] = (line[i] + line[i - bpp]) & 0xFF
        elif ftype == 2:  # Up
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 0xFF
        elif ftype == 3:  # Average
            for i in range(stride):
                left = line[i - bpp] if i >= bpp else 0
                line[i] = (line[i] + ((left + prev[i]) >> 1)) & 0xFF
        elif ftype == 4:  # Paeth
            for i in range(stride):
                a = line[i - bpp] if i >= bpp else 0
                b = prev[i]
                c = prev[i - bpp] if i >= bpp else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[i] = (line[i] + pr) & 0xFF
        out += line
        prev = line

    def sample(x: int, y: int):
        if color == 2 or color == 6:  # truecolor (+alpha), 8-bit assumed
            base = y * stride + x * channels
            r, g, b = out[base], out[base + 1], out[base + 2]
            a = out[base + 3] if color == 6 else 255
            return (r, g, b, a)
        if color == 0:  # grayscale
            if depth == 8:
                v = out[y * stride + x]
            elif depth == 16:
                v = out[y * stride + 2 * x]
            else:
                per = 8 // depth
                byte = out[y * stride + x // per]
                shift = 8 - depth * (x % per + 1)
                v = (byte >> shift) & ((1 << depth) - 1)
                v = v * 255 // ((1 << depth) - 1)
            if depth == 16:
                v >>= 8
            # tRNS for gray: a u16 sample value that IS transparent
            if trns and len(trns) >= 2:
                key = struct.unpack(">H", trns[:2])[0] >> 8 if depth == 16 else trns[0]
                if v == key:
                    return (v, v, v, 0)
            return (v, v, v, 255)
        if color == 4:  # gray + alpha, 8/16-bit
            if depth == 8:
                v, a = out[y * stride + 2 * x], out[y * stride + 2 * x + 1]
            else:
                v, a = out[y * stride + 4 * x], out[y * stride + 4 * x + 2]
            if depth == 16:
                v, a = v >> 8, a >> 8
            return (v, v, v, a)
        if color == 3:  # palette
            per = 8 // depth
            byte = out[y * stride + x // per]
            shift = 8 - depth * (x % per + 1)
            idx = (byte >> shift) & ((1 << depth) - 1)
            r, g, b = palette[idx]
            a = 255
            if trns and idx < len(trns):
                a = trns[idx]
            return (r, g, b, a)
        raise AssertionError(f"color type {color} unsupported")

    return width, height, sample


def rect_average(sample, x0: int, y0: int, x1: int, y1: int):
    """The quad_average rule: the per-channel mean over the rect's OPAQUE
    pixels (alpha != 0), rounded HALF-UP per channel. Returns
    (avg-or-None, opaque_count, total_count)."""
    sr = sg = sb = n = 0
    total = 0
    for y in range(y0, y1):
        for x in range(x0, x1):
            r, g, b, a = sample(x, y)
            total += 1
            if a != 0:
                sr += r
                sg += g
                sb += b
                n += 1
    if n == 0:
        return None, 0, total

    def half_up(s):
        return (s + n // 2) // n if n % 2 else (2 * s + n) // (2 * n)

    return (half_up(sr), half_up(sg), half_up(sb)), n, total


def main():
    import zipfile

    jar, path = sys.argv[1], sys.argv[2]
    with zipfile.ZipFile(jar) as z:
        data = z.read(path)
    w, h, sample = read_png(data)
    print(f"{path}: {w}x{h}")
    if len(sys.argv) >= 7:
        x0, y0, x1, y1 = (int(v) for v in sys.argv[3:7])
        avg, opaque, total = rect_average(sample, x0, y0, x1, y1)
        print(f"rect ({x0},{y0})-({x1},{y1}): avg={avg} opaque={opaque}/{total}")
    else:
        avg, opaque, total = rect_average(sample, 0, 0, w, h)
        print(f"full: avg={avg} opaque={opaque}/{total}")


if __name__ == "__main__":
    main()
