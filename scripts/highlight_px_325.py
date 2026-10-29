#!/usr/bin/env python3
"""highlight_px_325.py — the 325 probe's own pixel decode (the two
highlight sprites the vanilla law draws): the border/center alpha
verdicts the tests re-key. rc=0 banks."""
import struct
import zlib

R = "/home/z/my-project/rust-minecraft/assets-cache/jar/assets/minecraft/textures/gui/sprites/container"


def decode(p):
    d = open(p, "rb").read()
    w, h = struct.unpack(">II", d[16:24])
    bd, ct = d[24], d[25]
    assert bd == 8, f"bitdepth {bd}"
    assert ct == 4, f"colortype {ct}"  # the gray+alpha the vanilla wears
    bpp = 2
    pos, idat = 8, b""
    while pos < len(d):
        ln = struct.unpack(">I", d[pos:pos + 4])[0]
        ct = d[pos + 4:pos + 8]
        if ct == b"IDAT":
            idat += d[pos + 8:pos + 8 + ln]
        pos += 12 + ln
    raw = zlib.decompress(idat)
    stride = w * bpp + 1
    out = []
    prev = bytearray(w * bpp)
    for row in range(h):
        off = row * stride
        f = raw[off]
        line = bytearray(raw[off + 1:off + stride])
        for i in range(len(line)):
            a_ = line[i - bpp] if i >= bpp else 0
            b_ = prev[i]
            c_ = prev[i - bpp] if i >= bpp else 0
            if f == 1:
                line[i] = (line[i] + a_) & 255
            elif f == 2:
                line[i] = (line[i] + b_) & 255
            elif f == 3:
                line[i] = (line[i] + ((a_ + b_) >> 1)) & 255
            elif f == 4:
                pp = a_ + b_ - c_
                pa, pb, pc = abs(pp - a_), abs(pp - b_), abs(pp - c_)
                pr = a_ if pa <= pb and pa <= pc else (b_ if pb <= pc else c_)
                line[i] = (line[i] + pr) & 255
        prev = line
        out.append(line)
    return w, h, out


def show(name, pts):
    w, h, px = decode(f"{R}/{name}.png")
    print(f"{name}: {w}x{h}")
    for (x, y) in pts:
        i = x * 2  # the per-row bytearray the x indexes
        gray, a = px[y][i], px[y][i + 1]
        print(f"  ({x},{y}) gray={gray} alpha={a}")


PTS = [(0, 0), (12, 0), (0, 12), (12, 12), (4, 4), (12, 4), (23, 23)]
show("slot_highlight_back", PTS)
show("slot_highlight_front", PTS)
