#!/usr/bin/env python3
"""Scan ALL methods of one class for a memberref needle; print the calling
method + surrounding disasm. Usage: python3 scripts/sound_call_probe.py <jar> <class> <needle>"""
import struct, sys, zipfile
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8, member_name, disasm

def main():
    jar, cls, needle = sys.argv[1], sys.argv[2], sys.argv[3]
    data = zipfile.ZipFile(jar).read(f"{cls}.class")
    cp, i = parse_cp(data)
    i += 6
    nif = struct.unpack_from(">H", data, i)[0]
    i += 2 + 2 * nif
    nfields = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nfields):
        i += 6
        attrs = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(attrs):
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            i += 4 + alen
    nmethods = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nmethods):
        _acc = struct.unpack_from(">H", data, i)[0]
        name = utf8(cp, struct.unpack_from(">H", data, i + 2)[0])
        desc = utf8(cp, struct.unpack_from(">H", data, i + 4)[0])
        i += 6
        attrs = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(attrs):
            aname = utf8(cp, struct.unpack_from(">H", data, i)[0])
            alen = struct.unpack_from(">I", data, i + 2)[0]
            body = data[i + 6:i + 6 + alen]
            if aname == "Code":
                max_stack, max_locals, clen = struct.unpack_from(">HHI", body, 0)
                code_bytes = body[8:8 + clen]
                ins = disasm(code_bytes, cp)
                hits = [(s, m, a) for s, m, a in ins if needle in a]
                if hits:
                    print(f"=== {name}{desc} ===")
                    for h in hits:
                        print(f"  HIT {h[0]:5d}: {h[1]} {h[2]}")
                    # context: 12 instrs around each hit
                    for s, m, a in hits:
                        idx = next(k for k, (ss, _, _) in enumerate(ins) if ss == s)
                        lo = max(0, idx - 8)
                        for ss, mm, aa in ins[lo:idx + 6]:
                            mark = " >>" if ss == s else "   "
                            print(f"  {mark} {ss:5d}: {mm} {aa}")
                        print("  ---")
                    print()
            i += 6 + alen

if __name__ == "__main__":
    main()
