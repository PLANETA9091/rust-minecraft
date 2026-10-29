#!/usr/bin/env python3
"""the book texture's own palette decode (the entity/
enchantment/enchanting_table_book.png the colortype-3 PNG the jar
serves — the PLTE + tRNS the RGBA expansion serves, the per-region
occupancy the 64x32 map prints — the lids/pages/seam texOffs
rects the createBodyLayer stream pins the occupancy compares)."""
import struct
import zlib

data = open(
    "assets-cache/jar/assets/minecraft/textures/entity/enchantment/enchanting_table_book.png",
    "rb",
).read()
pos = 8
w = h = 0
idat = b""
plte = b""
trns = b""
while pos < len(data):
    ln, typ = struct.unpack(">I4s", data[pos : pos + 8])
    if typ == b"IHDR":
        w, h, bd, ctype = struct.unpack(">IIBB", data[pos + 8 : pos + 18])
    elif typ == b"PLTE":
        plte = data[pos + 8 : pos + 8 + ln]
    elif typ == b"tRNS":
        trns = data[pos + 8 : pos + 8 + ln]
    elif typ == b"IDAT":
        idat += data[pos + 8 : pos + 8 + ln]
    pos += 12 + ln
print("size", w, h, "bitdepth", bd, "colortype", ctype, "palette", len(plte) // 3)
raw = zlib.decompress(idat)
stride = w
out = bytearray()


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


prev = bytearray(w)
o = 0
for y in range(h):
    f = raw[o]
    o += 1
    line = bytearray(raw[o : o + stride])
    o += stride
    if f == 1:
        for i in range(1, stride):
            line[i] = (line[i] + line[i - 1]) & 255
    elif f == 2:
        for i in range(stride):
            line[i] = (line[i] + prev[i]) & 255
    elif f == 3:
        for i in range(stride):
            line[i] = (line[i] + ((line[i - 1] if i else 0) + prev[i]) // 2) & 255
    elif f == 4:
        for i in range(stride):
            line[i] = (line[i] + paeth(line[i - 1] if i else 0, prev[i], prev[i - 1] if i else 0)) & 255
    out += line
    prev = line


def px(x, y):
    pi = out[y * w + x]
    r, g, b = plte[pi * 3 : pi * 3 + 3]
    a = trns[pi] if pi < len(trns) else 255
    return (r, g, b, a)


regions = {
    "left_lid  (0,0)-(5,9)": (0, 0, 6, 10),
    "right_lid (16,0)-(21,9)": (16, 0, 6, 10),
    "seam      (12,0)-(13,9)": (12, 0, 2, 10),
    "lp_top    (1,10)-(5,10)": (1, 10, 5, 1),
    "lp_front  (1,11)-(5,18)": (1, 11, 5, 8),
    "lp_back   (7,11)-(11,18)": (7, 11, 5, 8),
    "rp_top    (13,10)-(17,10)": (13, 10, 5, 1),
    "rp_front  (13,11)-(17,18)": (13, 11, 5, 8),
    "rp_back   (19,11)-(23,18)": (19, 11, 5, 8),
    "fp_front  (25,11)-(29,18)": (25, 11, 5, 8),
}
for name, (rx, ry, rw, rh) in regions.items():
    colors = {}
    for y in range(ry, ry + rh):
        for x in range(rx, rx + rw):
            c = px(x, y)
            colors[c] = colors.get(c, 0) + 1
    top = sorted(colors.items(), key=lambda kv: -kv[1])[:4]
    print("%-26s %s" % (name, ["%02x%02x%02x%02x x%d" % (c[0], c[1], c[2], c[3], n) for c, n in top]))

print()
print("full 64x32 occupancy map (# dark, + mid, . transparent):")
for y in range(h):
    row = ""
    for x in range(w):
        r, g, b, a = px(x, y)
        if a == 0:
            row += "."
        elif r + g + b < 200:
            row += "#"
        elif r + g + b < 600:
            row += "+"
        else:
            row += " "
    print("%2d %s" % (y, row))
