#!/usr/bin/env python3
"""Print LocalVariableTable / MethodParameters of one method.

Usage: python3 scripts/lvt_dump.py <jar> <net/minecraft/Class> <method-substring>
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402


def skip_attrs(data, i, cp):
    n = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(n):
        i += 2
        alen = struct.unpack_from(">I", data, i)[0]
        i += 4
        yield (i, alen)
        i += alen


def main():
    jar_path, cls, needle = sys.argv[1], sys.argv[2], sys.argv[3]
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    cp, i = parse_cp(data)
    i += 6
    nif = struct.unpack_from(">H", data, i)[0]
    i += 2 + 2 * nif
    nfields = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nfields):
        i += 6
        nfa = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(nfa):
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            i += 4 + alen
    nmethods = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nmethods):
        acc = struct.unpack_from(">H", data, i)[0]
        name_i, desc_i = struct.unpack_from(">HH", data, i + 2)
        name = utf8(cp, name_i)
        i += 6
        if needle not in name:
            na = struct.unpack_from(">H", data, i)[0]
            i += 2
            for _ in range(na):
                i += 2
                alen = struct.unpack_from(">I", data, i)[0]
                i += 4 + alen
            continue
        print(f"=== {name} {utf8(cp, desc_i)}")
        na = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(na):
            aname = utf8(cp, struct.unpack_from(">H", data, i)[0])
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            body = data[i:i + alen]
            i += alen
            if aname == "LocalVariableTable":
                cnt = struct.unpack_from(">H", body, 0)[0]
                j = 2
                print("  LocalVariableTable:")
                for _e in range(cnt):
                    start, ln, vname_i, vdesc_i, slot = struct.unpack_from(">HHHHH", body, j)
                    j += 10
                    print(f"    slot {slot}: {utf8(cp, vname_i)} {utf8(cp, vdesc_i)} (pc {start}+{ln})")
            elif aname == "MethodParameters":
                cnt = struct.unpack_from(">H", body, 0)[0]
                j = 2
                names = []
                for _e in range(cnt):
                    ni, flags = struct.unpack_from(">HH", body, j)
                    j += 4
                    names.append(utf8(cp, ni) if ni else "?")
                print(f"  MethodParameters: {names}")
            else:
                print(f"  attr {aname} ({alen} bytes)")


if __name__ == "__main__":
    main()
