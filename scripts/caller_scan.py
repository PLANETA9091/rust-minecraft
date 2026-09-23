#!/usr/bin/env python3
"""Caller audit: scan ONE class for `sipush <N>` sites using full_disasm's
proven parse_cp (the cp walk returns the fields/methods start — no
re-walk drift). Dumps an instruction window around each hit.

Usage: python3 scripts/caller_scan.py <jar> <net/minecraft/Class> <v1> [<v2> ...]
"""
import struct, sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402
from sipush_scan import walk_code  # noqa: E402


def methods(data, cp, fields_start):
    p = fields_start
    p += 6  # access_flags + this_class + super_class
    ic = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    p += 2 * ic  # each interface is one u2 class index
    fc = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    for _ in range(fc):
        p += 6
        an = struct.unpack('>H', data[p:p + 2])[0]; p += 2
        for _ in range(an):
            p += 2; aln = struct.unpack('>I', data[p:p + 4])[0]; p += 4 + aln
    mc = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    for _ in range(mc):
        acc, name_i, desc_i = struct.unpack('>HHH', data[p:p + 6]); p += 6
        ac = struct.unpack('>H', data[p:p + 2])[0]; p += 2
        for _ in range(ac):
            an_i = struct.unpack('>H', data[p:p + 2])[0]
            aln = struct.unpack('>I', data[p + 2:p + 6])[0]
            body = data[p + 6:p + 6 + aln]
            p += 6 + aln
            if cp.get(an_i) and cp[an_i][0] == 'utf8' and cp[an_i][1] == 'Code' and aln >= 12:
                clen = struct.unpack('>I', body[4:8])[0]
                yield utf8(cp, name_i), body[8:8 + clen]


def main():
    jar, cls = sys.argv[1], sys.argv[2]
    vals = set(int(v) for v in sys.argv[3:])
    data = zipfile.ZipFile(jar).read(f"{cls}.class")
    cp, fields_start = parse_cp(data)
    for name, code in methods(data, cp, fields_start):
        rows = list(walk_code(code))
        hits = [(pc, struct.unpack('>h', opnd[:2])[0]) for pc, op, opnd in rows
                if op == 0x11 and struct.unpack('>h', opnd[:2])[0] in vals]
        if not hits:
            continue
        print(f"=== {cls} :: {name} ===")
        idx_by_pc = {pc: i for i, (pc, _, _) in enumerate(rows)}
        for pc, v in hits:
            i = idx_by_pc[pc]
            lo, hi = max(0, i - 4), min(len(rows), i + 8)
            for j in range(lo, hi):
                rpc, rop, ropnd = rows[j]
                mark = "->" if j == i else "  "
                if rop in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9):
                    cpi = struct.unpack('>H', ropnd[:2])[0]
                    e = cp.get(cpi)
                    label = f"cp#{cpi}"
                    if e and e[0] in ('method', 'iface', 'field'):
                        ci, nti = e[1]
                        c = cp.get(ci)
                        nat = cp.get(nti)
                        if c and nat:
                            cname = utf8(cp, c[1]) if c[0] == 'utf8' else '?'
                            nname = utf8(cp, nat[1][0])
                            label = f"{cname}.{nname}"
                    print(f"  {mark} {rpc:5d}: {rop:#04x} {label}")
                else:
                    extra = ''
                    if rop == 0x11:
                        extra = f" {struct.unpack('>h', ropnd[:2])[0]}"
                    elif rop == 0x10:
                        extra = f" {ropnd[0]}"
                    print(f"  {mark} {rpc:5d}: {rop:#04x}{extra}")


if __name__ == '__main__':
    main()
