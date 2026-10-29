#!/usr/bin/env python3
"""autopush-320 — THE BEACON BEAM'S OWN CONSTANT POOL probe.

Resolves every ldc the BeaconRenderer methods wear:
  - the two submitBeaconBeam overloads' REAL descriptors (the FFIII
    dump line the round's arg-count question answers),
  - ldc #107  — the beamRadiusScale's own divisor (the extract arm),
  - ldc #139 / #140 — the core / halo radius factors (the 9-arg fold),
  - ldc #155 — the frac's floor factor,
  - ldc #172 / #173 — the core rotation's 2.25 / 45.0 pair,
  - ldc #182 / #183 — the v bot base + the core v-span factor.

Exit 0 iff the class parses and every probe prints.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'
CLS = 'net/minecraft/client/renderer/blockentity/BeaconRenderer'


def cp_utf8(cp, idx):
    """THE INDIRECT READ (the Class / NameAndType rows the two-hop
    walk keeps — the entry the utf8 tail answers, else the #i marker)."""
    e = cp.get(idx)
    if not e:
        return f"#{idx}"
    if e[0] == "utf8":
        return e[1]
    if isinstance(e[0], int) and e[0] == 7:
        return utf8(cp, e[1])
    if e[0] == "name_type":
        return f"{utf8(cp, e[1][0])}:{utf8(cp, e[1][1])}"
    return f"#{idx}"


def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read(CLS + '.class')
    cp, n = parse_cp(data)
    print(f"## {CLS} — cp entries {n}")

    # THE CONSTANT POOL'S OWN WALK (the Float / Integer / String /
    # Methodref rows the probes wear)
    for i in sorted(cp):
        tag, *rest = cp[i] if isinstance(cp[i], tuple) else (cp[i],)
        if tag == 4:  # Float (raw u32 bits)
            (val,) = struct.unpack('>f', struct.pack('>I', rest[0]))
            print(f"  #{i} Float {val!r}")
        elif tag == 3:  # Integer
            print(f"  #{i} Integer {rest[0]}")
        elif tag == 8:  # String -> utf8
            print(f"  #{i} String {utf8(cp, rest[0])!r}")
        elif tag in (9, 10, 11, "field", "method", "iface"):
            # Ref rows — (class_idx, name_type_idx) the pair keeps
            pair = rest[0] if len(rest) == 1 else rest
            ci, nt = pair
            cn = cp_utf8(cp, ci)
            ntn = cp_utf8(cp, nt)
            if 'Beacon' in cn or 'beaconBeam' in ntn or 'renderPart' in ntn or 'renderQuad' in ntn or 'submit' in ntn:
                print(f"  #{i} Ref {cn} :: {ntn}")
        elif tag == 1:  # Utf8 — the method descriptors the round pins
            s = rest[0]
            if 'submitBeaconBeam' in s or 'renderPart' in s or 'renderQuad' in s:
                print(f"  #{i} Utf8 {s!r}")

    print("\n## done")
    return 0


if __name__ == '__main__':
    sys.exit(main())
