#!/usr/bin/env python3
"""THE WIRE'S CLASS WALK v4 — the harness the weighted_plate_49 probe
keeps (exec-prefixed), the RedStoneWireBlock methods + the Blocks/Items
clinit rows the wire's own round reads.
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/weighted_plate_49.py").read()
cut = src.index("def parse_cp")
prefix = src[cut:]
prefix = prefix.split('if __name__ == "__main__":')[0]
exec(prefix)

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)


def dump(path, label, only=None):
    print(f"\n{'=' * 70}\n== {label}\n{'=' * 70}")
    d = z.read(path)
    cp, cpend = parse_cp(d)
    for (mn, md, an, body) in methods_of(d, cp, cpend):
        if an != "Code":
            continue
        if only and not any(mn == f or (f.endswith("*") and mn.startswith(f[:-1])) for f in only):
            continue
        mx, ml = struct.unpack_from(">HH", body, 0)
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"\n-- {mn}{md}  (code len {clen})")
        for pc, mnem, detail in walk_code(cp, code):
            print(f"  {pc}: {mnem} {detail}")


def consts(path, label):
    d = z.read(path)
    cp, _ = parse_cp(d)
    fl = [(i, struct.unpack(">f", struct.pack(">I", e[1]))[0]) for i, e in enumerate(cp) if e and e[0] == 4]
    print(f"\n== {label} floats:", [f"{i}:{v:.4g}" for i, v in fl])
    ints = [(i, struct.unpack(">i", struct.pack(">I", e[1]))[0]) for i, e in enumerate(cp) if e and e[0] == 3]
    print(f"== {label} ints:", [f"{i}:{v}" for i, v in ints][:30])


dump("net/minecraft/world/level/block/RedStoneWireBlock.class", "RedStoneWireBlock")
consts("net/minecraft/world/level/block/RedStoneWireBlock.class", "wire")
