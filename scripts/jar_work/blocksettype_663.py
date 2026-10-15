#!/usr/bin/env python3
"""THE BLOCKSETTYPE'S OWN PLATE ROWS probe (the autopush-663 round): dump
BlockSetType's static init — the ctor rows' canButtonBeActivatedByArrows
int args and the pressurePlateClickOn/Off field assignments the plate
voices ride — plus PressurePlateBlock's ctor + getSignalStrength walks
(the sensitivity derivation the ladder's verdicts keep).

Usage: python3 scripts/jar_work/blocksettype_663.py <client.jar>
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from plate_ladder_663 import (  # noqa: E402
    parse_cp, utf, ref_name, walk_code, methods_of, bootstrap_methods, mh_target,
)

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"


def dump_class(z, cls, methods_filter=None, fields_filter=None):
    d = z.read(cls)
    cp, cpend = parse_cp(d)
    print(f"##### {cls} #####")
    # fields
    p = cpend + 6
    ifc = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc
    nf = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nf):
        acc = struct.unpack_from(">H", d, p)[0]
        fn = struct.unpack_from(">H", d, p + 2)[0]
        fd = struct.unpack_from(">H", d, p + 4)[0]
        na = struct.unpack_from(">H", d, p + 6)[0]
        name = utf(cp, fn)
        show = True
        if fields_filter is not None:
            show = any(f in name for f in fields_filter)
        if show:
            print(f"  FIELD {'static ' if acc & 0x8 else ''}{name} : {utf(cp, fd)}")
        p += 8
        for _fa in range(na):
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    nm = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nm):
        acc = struct.unpack_from(">H", d, p)[0]
        mn = struct.unpack_from(">H", d, p + 2)[0]
        md = struct.unpack_from(">H", d, p + 4)[0]
        na = struct.unpack_from(">H", d, p + 6)[0]
        name = utf(cp, mn)
        p += 8
        for _a in range(na):
            an = struct.unpack_from(">H", d, p)[0]
            alen = struct.unpack_from(">I", d, p + 2)[0]
            body = d[p + 6:p + 6 + alen]
            if utf(cp, an) == "Code":
                show = methods_filter is None or any(f in name for f in methods_filter)
                if show:
                    clen = struct.unpack_from(">I", body, 4)[0]
                    code = body[8:8 + clen]
                    print(f"  --- {name}{utf(cp, md)} ---")
                    for pc, mnem, detail in walk_code(cp, code):
                        print(f"    {pc:5d}: {mnem} {detail}")
            p += 6 + alen


def main():
    with zipfile.ZipFile(JAR) as z:
        # BlockSetType's clinit: the ctor rows + the click-field assignments
        dump_class(
            z,
            "net/minecraft/world/level/block/state/properties/BlockSetType.class",
            methods_filter=["<clinit>", "<init>"],
            fields_filter=["pressurePlateClick", "canButtonBeActivatedByArrows", "buttonClick"],
        )
        # PressurePlateBlock: the ctor's sensitivity derivation + the walks
        dump_class(
            z,
            "net/minecraft/world/level/block/PressurePlateBlock.class",
            methods_filter=["<init>", "getSignalStrength", "getSignalForState", "setSignalForState", "playSound"],
        )


if __name__ == "__main__":
    main()
