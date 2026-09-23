#!/usr/bin/env python3
"""Scan ONE class file inside a jar for `sipush <N>` instruction sites and
dump a small window of decoded instructions around each hit (the block
classes' own caller audit the LevelEvent sound rounds speak).

Usage: python3 scripts/sipush_class_scan.py <jar> <net/minecraft/Class> <v1> [<v2> ...]
"""
import struct, sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from sipush_scan import W, walk_code, cp_utf8s, method_codes  # noqa: E402


def main():
    jar, cls = sys.argv[1], sys.argv[2]
    vals = set(int(v) for v in sys.argv[3:])
    data = zipfile.ZipFile(jar).read(f"{cls}.class")
    u8 = cp_utf8s(data)
    for name, code in method_codes(data, u8):
        sites = []
        for pc, op, operands in walk_code(code):
            if op == 0x11:  # sipush
                v = struct.unpack('>h', operands[:2])[0]
                if v in vals:
                    sites.append((pc, v))
        if not sites:
            continue
        print(f"=== {cls} :: {name} ===")
        # decode the whole method linearly once for the window
        rows = list(walk_code(code))
        idx_by_pc = {}
        for i, (pc, op, operands) in enumerate(rows):
            idx_by_pc[pc] = i
        for pc, v in sites:
            i = idx_by_pc[pc]
            lo = max(0, i - 6)
            hi = min(len(rows), i + 10)
            print(f"-- sipush {v} at pc {pc} (window) --")
            for j in range(lo, hi):
                rpc, rop, ropnd = rows[j]
                if rop in (0xb2, 0xb3, 0xb4, 0xb5):
                    mark = "->" if j == i else "  "
                    print(f"  {mark} {rpc:5d}: {rop:#04x} cp#{struct.unpack('>H', ropnd[:2])[0]}")
                elif rop in (0xb6, 0xb7, 0xb8):
                    mark = "->" if j == i else "  "
                    print(f"  {mark} {rpc:5d}: {rop:#04x} invoke cp#{struct.unpack('>H', ropnd[:2])[0]}")
                else:
                    mark = "->" if j == i else "  "
                    print(f"  {mark} {rpc:5d}: {rop:#04x}")


if __name__ == '__main__':
    main()
