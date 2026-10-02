#!/usr/bin/env python3
"""Resolve CP float/int constants in a class: cp_float.py <jar> <class> <idx> [idx...]
Prints `#idx = <float>` for each tag-3 (int) / tag-4 (float) entry."""
import struct, sys, zipfile
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp

def main():
    jar, cls = sys.argv[1], sys.argv[2]
    data = zipfile.ZipFile(jar).read(f"{cls}.class")
    cp, _ = parse_cp(data)
    for a in sys.argv[3:]:
        i = int(a)
        e = cp.get(i)
        if e and e[0] == 4:  # Float
            bits = e[1]
            val = struct.unpack(">f", struct.pack(">I", bits))[0]
            print(f"#{i} = float {val}")
        elif e and e[0] == 3:  # Integer
            bits = e[1]
            val = struct.unpack(">i", struct.pack(">I", bits))[0]
            print(f"#{i} = int {val}")
        else:
            print(f"#{i} = {e}")

if __name__ == "__main__":
    main()
