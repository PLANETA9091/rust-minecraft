#!/usr/bin/env python3
"""Dump BootstrapMethods for one class (scan-based, no classfile walk).

Usage: python3 scripts/bsm_probe.py <jar> <net/minecraft/Class>
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8


def main():
    jar, cls = sys.argv[1], sys.argv[2]
    z = zipfile.ZipFile(jar)
    data = z.read(cls + ".class")
    cp, _ = parse_cp(data)

    bsm_name_idx = None
    for idx, ent in cp.items():
        if ent[0] == "utf8" and ent[1] == "BootstrapMethods":
            bsm_name_idx = idx
            break
    assert bsm_name_idx, "no BootstrapMethods attribute in class"

    pat = struct.pack(">H", bsm_name_idx)
    pos = 0
    found = False
    while True:
        i = data.find(pat, pos)
        if i < 0:
            break
        pos = i + 1
        if i + 6 > len(data):
            continue
        (ln,) = struct.unpack_from(">I", data, i + 2)
        if ln <= 0 or ln > len(data) or i + 6 + ln > len(data):
            continue
        payload = data[i + 6 : i + 6 + ln]
        (n_bsm,) = struct.unpack_from(">H", payload, 0)
        if 2 + n_bsm * 6 > ln or n_bsm == 0 or n_bsm > 64:
            continue
        found = True
        print(f"BootstrapMethods ({n_bsm} entries)")
        p = 2
        for bi in range(n_bsm):
            mh_idx, n_args = struct.unpack_from(">HH", payload, p)
            p += 4
            mh = cp.get(mh_idx)
            print(f"  bsm#{bi}: MethodHandle@{mh_idx} -> {mh}")
            if mh and mh[0] == 15:
                kind, ref_idx = mh[1], mh[2]
                ref = cp.get(ref_idx)
                if ref and ref[0] == "method":
                    cls_i, nat_i = ref[1]
                    cls_e = cp.get(cls_i)
                    nat_e = cp.get(nat_i)
                    cls_name = utf8(cp, cls_e[1]) if cls_e and cls_e[0] == 7 else str(cls_e)
                    if nat_e and nat_e[0] == "name_type":
                        nat = (utf8(cp, nat_e[1][0]), utf8(cp, nat_e[1][1]))
                    else:
                        nat = str(nat_e)
                    print(f"      impl: kind={kind} class={cls_name} nat={nat}")
            for j in range(n_args):
                a = struct.unpack_from(">H", payload, p)[0]
                p += 2
                print(f"    arg: @{a} -> {cp[a]}")
    if not found:
        print("no BootstrapMethods attribute found")


if __name__ == "__main__":
    main()
