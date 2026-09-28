#!/usr/bin/env python3
"""Sculk 3006 caller probe: walk a list of classes, find `sipush 3006`
sites and print the enclosing method + surrounding levelEvent context.
Reuses full_disasm's parser.

Usage: python3 scripts/sculk_probe.py <jar> <class> [<class> ...]
"""
import struct, sys, zipfile
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8, MNEMONIC


def code_widths(code_bytes, cp):
    """Linear opcode walk (method length) — returns list of (pc, mn, args)."""
    out = []
    pc = 0
    n = len(code_bytes)
    while pc < n:
        op = code_bytes[pc]
        mn, w = MNEMONIC.get(op, ("??", 0))
        if op in (0xAA, 0xAB):  # tableswitch / lookupswitch
            base = pc + 1
            base = (base + 3) & ~3
            if op == 0xAA:
                lo = struct.unpack_from(">i", code_bytes, base + 4)[0]
                hi = struct.unpack_from(">i", code_bytes, base + 8)[0]
                cnt = hi - lo + 1
                w = (base + 12 + 4 * cnt) - pc
            else:
                npairs = struct.unpack_from(">i", code_bytes, base + 4)[0]
                w = (base + 8 + 8 * npairs) - pc
            out.append((pc, mn, f"w={w}"))
            pc += w
            continue
        if w == -1:
            out.append((pc, f"??op{op:#x}", ""))
            pc += 1
            continue
        args_bytes = code_bytes[pc + 1:pc + 1 + w]
        args = ""
        if op in (0x12,):  # ldc
            idx = args_bytes[0]
            e = cp.get(idx, ("?",))
            args = f"#{idx} {e}"
        elif w:
            args = " ".join(str(b) for b in args_bytes)
        out.append((pc, mn, args))
        pc += 1 + w
    return out


def main():
    jar = sys.argv[1]
    z = zipfile.ZipFile(jar)
    for cls in sys.argv[2:]:
        try:
            data = z.read(cls + ".class")
        except KeyError:
            print(f"# missing {cls}")
            continue
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
                    ins = list(code_widths(code_bytes, cp))
                    hits = [(pc, mn, args) for pc, mn, args in ins
                            if mn == "sipush" and args.startswith("3006")]
                    if hits:
                        print(f"=== {cls}::{name}{desc} ===")
                        for pc, mn, args in hits:
                            print(f"  @{pc}: {mn} {args}")
                            # print context: 40 instructions before/after
                            k = next(j for j, (p, _, _) in enumerate(ins) if p == pc)
                            for p, m2, a2 in ins[max(0, k - 25):k + 30]:
                                print(f"    {p:5d}: {m2} {a2}")
                i += 6 + alen


if __name__ == "__main__":
    main()
