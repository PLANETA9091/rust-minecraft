#!/usr/bin/env python3
"""Constant-pool probe for one class: decode FLOAT/Int/Long/Double constants.
Reuses full_disasm's proven parse_cp. Usage: python3 scripts/cp_probe.py <jar> <class> [cp indices...]"""
import struct, sys, zipfile
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8

def main():
    jar, cls = sys.argv[1], sys.argv[2]
    want = set(int(x) for x in sys.argv[3:]) if len(sys.argv) > 3 else None
    z = zipfile.ZipFile(jar)
    data = z.read(cls + ".class")
    cp = parse_cp(data)[0]
    for i, e in sorted(cp.items()):
        if want and i not in want:
            continue
        if not e:
            continue
        kind = e[0]
        if kind == 4:  # Float stored as raw u32 by parse_cp
            f = struct.unpack(">f", struct.pack(">I", e[1]))[0]
            print(f"#{i} = Float {f!r}")
        elif kind == 3:
            print(f"#{i} = Int {e[1]}")
        elif kind in (5, 6):
            raw = struct.pack(">Q", e[1])
            v = struct.unpack(">d", raw)[0] if kind == 6 else struct.unpack(">q", raw)[0]
            print(f"#{i} = {'Double' if kind == 6 else 'Long'} {v!r}")
        elif kind == 8:
            print(f"#{i} = String \"{utf8(cp, e[1])}\"")

if __name__ == "__main__":
    main()
