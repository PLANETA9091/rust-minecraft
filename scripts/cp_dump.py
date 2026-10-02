#!/usr/bin/env python3
"""Dump the full constant pool of one class: numbers, strings, member refs.

Usage: python3 scripts/cp_dump.py <jar> <net/minecraft/Class>
Reuses parse_cp from full_disasm.py.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402


def main():
    jar_path, cls = sys.argv[1], sys.argv[2]
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    cp, _ = parse_cp(data)
    for idx in sorted(cp):
        e = cp[idx]
        tag = e[0]
        if tag == "utf8":
            print(f"#{idx} utf8 {e[1]!r}")
        elif tag == 3:
            f = struct.unpack(">f", struct.pack(">I", e[1]))[0]
            print(f"#{idx} float {f!r} (0x{e[1]:08x})")
        elif tag == 4:
            d = struct.unpack(">d", struct.pack(">Q", e[1]))[0]
            print(f"#{idx} double {d!r} (0x{e[1]:016x})")
        elif tag in ("field", "method", "iface"):
            clsname = utf8(cp, e[1][0])
            nt = cp.get(e[1][1])
            if nt and nt[0] == "name_type":
                print(f"#{idx} {tag} {clsname}.{utf8(cp, nt[1][0])}:{utf8(cp, nt[1][1])}")
            else:
                print(f"#{idx} {tag} {clsname} #{e[1][1]}")
        elif tag == "name_type":
            print(f"#{idx} name_type {utf8(cp, e[1][0])}:{utf8(cp, e[1][1])}")
        elif tag == 9 or tag == 10:
            pass
        else:
            print(f"#{idx} tag{tag} {e}")


if __name__ == "__main__":
    main()
