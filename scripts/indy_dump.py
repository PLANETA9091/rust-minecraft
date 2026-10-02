#!/usr/bin/env python3
"""Dump BootstrapMethods + InvokeDynamic constants of one class.

Usage: python3 scripts/indy_dump.py <jar> <net/minecraft/Class>
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402


def main():
    jar_path, cls = sys.argv[1], sys.argv[2]
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    cp, i = parse_cp(data)
    # skip access/this/super
    i += 6
    nif = struct.unpack_from(">H", data, i)[0]
    i += 2 + 2 * nif
    nfields = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nfields):
        i += 6
        na = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(na):
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            i += 4 + alen
    nmethods = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nmethods):
        i += 6  # access_flags, name_idx, desc_idx
        na = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(na):
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            i += 4 + alen
    # class attributes
    ncls = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(ncls):
        name_idx = struct.unpack_from(">H", data, i)[0]
        i += 2
        alen = struct.unpack_from(">I", data, i)[0]
        i += 4
        if utf8(cp, name_idx) == "BootstrapMethods":
            nb = struct.unpack_from(">H", data, i)[0]
            j = i + 2
            for b in range(nb):
                mh_idx, nargs = struct.unpack_from(">HH", data, j)
                j += 4
                mh = cp[mh_idx]
                # MethodHandle: tag(1) reference_kind(1) ref_idx(2)
                ref = cp[mh[1]]
                print(f"bootstrap {b}: handle {utf8(cp, ref[1][0])}.{utf8(cp, cp[ref[1][1]][1][0])}"
                      if ref[0] == 'method' else f"bootstrap {b}: handle {ref}")
                args = []
                for _a in range(nargs):
                    aidx = struct.unpack_from(">H", data, j)[0]
                    j += 2
                    args.append(f"#{aidx}")
                print(f"   args: {args}")
    # indy constants
    for idx in sorted(cp):
        e = cp[idx]
        if isinstance(e, tuple) and e and e[0] == 18:
            bsm, nt = e[1]
            nte = cp.get(nt)
            name = utf8(cp, nte[1][0]) if nte and nte[0] == "name_type" else f"#{nt}"
            print(f"indy #{idx}: bootstrap {bsm} name={name}")


if __name__ == "__main__":
    main()
