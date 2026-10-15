#!/usr/bin/env python3
"""THE REDSTONE WIRE'S OWN PROBE v2 (the autopush-665 round): the
multipart wire blockstate, the dust model family, the PNG grayscale
verdict, the class walks. javap absent — the python walk reads.
"""
import json
import struct
import sys
import zipfile
import zlib

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

# 1) all redstone_dust* model names in the jar
names = [n for n in z.namelist() if n.startswith("assets/minecraft/models/block/redstone_dust")]
print("dust models in jar:")
for n in sorted(names):
    print("   ", n)

# 2) the multipart wire blockstate
bs = json.loads(z.read("assets/minecraft/blockstates/redstone_wire.json"))
print("\nredstone_wire.json top keys:", list(bs.keys()))
parts = bs.get("multipart", [])
print(f"multipart parts: {len(parts)}")
for i, part in enumerate(parts):
    when = part.get("when", {})
    apply = part.get("apply")
    print(f"part[{i}] when={json.dumps(when)}")
    rows = apply if isinstance(apply, list) else [apply]
    for r in rows:
        print(f"   -> model={r.get('model')} x={r.get('x')} y={r.get('y')}")

# 3) each dust model's elements/tintindex
for n in sorted(names):
    stem = n.split("/")[-1][:-5]
    mj = json.loads(z.read(n))
    print(f"\n-- model {stem}: parent={mj.get('parent')}")
    print("   textures:", json.dumps(mj.get("textures")))
    for el in mj.get("elements", []):
        print("   element from", el.get("from"), "to", el.get("to"))
        for fn, fd in el.get("faces", {}).items():
            print(f"     face {fn}: tex={fd.get('texture')} tintindex={fd.get('tintindex')} uv={fd.get('uv')}")

# 4) the dust PNGs — proper IHDR parse
def png_pixels(raw):
    w, h, bitd, ctyp = struct.unpack_from(">IIBB", raw, 16)
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[ctyp]
    p = 8
    idat = b""
    plte = None
    while p < len(raw):
        ln = struct.unpack_from(">I", raw, p)[0]
        typ = raw[p + 4:p + 8]
        if typ == b"IDAT":
            idat += raw[p + 8:p + 8 + ln]
        elif typ == b"PLTE":
            plte = raw[p + 8:p + 8 + ln]
        p += 12 + ln
    dec = zlib.decompress(idat)
    bpp = channels * (bitd // 8) if bitd >= 8 else 1
    stride = w * bpp + 1
    rows = []
    prev = bytearray(w * bpp)
    for row in range(h):
        base = row * stride
        f = dec[base]
        line = bytearray(dec[base + 1:base + 1 + w * bpp])
        if f == 1:
            for i in range(bpp, len(line)):
                line[i] = (line[i] + line[i - bpp]) & 0xFF
        elif f == 2:
            for i in range(len(line)):
                line[i] = (line[i] + prev[i]) & 0xFF
        elif f == 3:
            for i in range(len(line)):
                a = line[i - bpp] if i >= bpp else 0
                line[i] = (line[i] + ((a + prev[i]) >> 1)) & 0xFF
        elif f == 4:
            for i in range(len(line)):
                a = line[i - bpp] if i >= bpp else 0
                b = prev[i]
                c = prev[i - bpp] if i >= bpp else 0
                pp = a + b - c
                pa, pb, pc = abs(pp - a), abs(pp - b), abs(pp - c)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[i] = (line[i] + pr) & 0xFF
        prev = line
        rows.append(bytes(line))
    return w, h, ctyp, channels, rows, plte


for t in ["redstone_dust_dot", "redstone_dust_overlay"] + [n.split("/")[-1][:-5] for n in sorted(names) if "line" in n or "overlay" in n or "dot" in n]:
    try:
        raw = z.read(f"assets/minecraft/textures/block/{t}.png")
    except KeyError:
        continue
    w, h, ctyp, ch, rows, plte = png_pixels(raw)
    print(f"\n-- texture {t}: {w}x{h} colortype={ctyp} channels={ch} bytes={len(raw)}")
    gray = True
    sample = []
    opq = 0
    for y in range(h):
        line = rows[y]
        for x in range(w):
            if ctyp == 6:
                r, g, b, a = line[x * 4:x * 4 + 4]
            elif ctyp == 2:
                r, g, b = line[x * 3:x * 3 + 3]
                a = 255
            elif ctyp == 4:
                r = g = b = line[x * 2]
                a = line[x * 2 + 1]
            elif ctyp == 3:
                i = line[x]
                r, g, b = plte[i * 3:i * 3 + 3]
                a = 255
            else:
                continue
            if a > 0:
                opq += 1
                if r != g or g != b:
                    gray = False
                if len(sample) < 8 and a > 0:
                    sample.append((x, y, r, g, b, a))
    print(f"   grayscale={gray} opaque_px={opq}/256 samples={sample}")
