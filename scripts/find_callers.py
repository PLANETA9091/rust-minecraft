#!/usr/bin/env python3
"""Find which methods of a class invoke a given method (by name) via Methodref/InterfaceMethodref.

Usage: python3 find_callers.py <jar> <class-with-slashes> <called-method-name>
Prints each calling method signature and the invoke pc where the call happens.
"""
import sys
import struct
import zipfile

sys.path.insert(0, "/home/z/my-project/rust-minecraft/scripts")
from full_disasm import parse_cp, utf8


def main():
    jar, cls, needle = sys.argv[1], sys.argv[2], sys.argv[3]
    data = zipfile.ZipFile(jar).read(f"{cls}.class")
    cp, _ = parse_cp(data)

    # 1) collect UTF8 indices equal to needle
    needle_utf8 = set()
    for idx, e in cp.items():
        if isinstance(e, tuple) and e[0] == 1 and utf8(cp, idx) == needle:
            needle_utf8.add(idx)
    # 2) NameAndType entries whose name is needle
    nats = set()
    for idx, e in cp.items():
        if isinstance(e, tuple) and e[0] == 12 and e[1] in needle_utf8:
            nats.add(idx)
    # 3) Methodref(10)/InterfaceMethodref(11) pointing at those NATs
    refs = set()
    for idx, e in cp.items():
        if isinstance(e, tuple) and e[0] in (10, 11) and e[2] in nats:
            refs.add(idx)
    if not refs:
        print(f"no methodrefs to {needle!r} found in constant pool")
        return

    # 4) walk methods, scan Code for invoke* opcodes referencing those CP slots
    i = 10  # skip header: magic(4)+minor(2)+major(2) = 8, then cp ends where parse_cp left; re-walk like full_disasm main
    # replicate full_disasm.main's skipping logic
    # header: 8 bytes; parse_cp consumed constant pool already (returned end index i)
    # full_disasm main: i += 6 (access+this+super) starting from returned i
    i_end = i
    # Re-do the same field/method walk as full_disasm.main:
    import io, contextlib
    # We cannot reuse main(); re-implement:
    i = i_end
    # recompute end-of-cp index by calling parse_cp again is unnecessary; parse_cp returned i already
    return


def walk(jar, cls, needle, refs):
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
                code = body[8:8 + clen]
                # raw scan: invoke* opcodes followed by the target CP index
                # (opcode has 1 operand byte pair; match anywhere — a big
                # CP index makes mid-instruction false positives unlikely)
                for op in (0xB6, 0xB7, 0xB8, 0xB9):
                    pat = struct.pack(">BH", op, 0)  # placeholder, patch below
                import re
                for idx in sorted(refs):
                    for op in (0xB6, 0xB7, 0xB8, 0xB9):
                        pat = bytes([op]) + struct.pack(">H", idx)
                        start = 0
                        while True:
                            hit = code.find(pat, start)
                            if hit < 0:
                                break
                            print(f"{name}{desc}  pc={hit}  op=0x{op:02x} cp#{idx}")
                            start = hit + 1
            i += 6 + alen


if __name__ == "__main__":
    jar, cls, needle = sys.argv[1], sys.argv[2], sys.argv[3]
    data0 = zipfile.ZipFile(jar).read(f"{cls}.class")
    cp0, _ = parse_cp(data0)
    needle_utf8 = {idx for idx, e in cp0.items()
                   if isinstance(e, tuple) and e[0] == "utf8" and e[1] == needle}
    nats = {idx for idx, e in cp0.items()
            if isinstance(e, tuple) and e[0] == "name_type" and e[1][0] in needle_utf8}
    refs = {idx for idx, e in cp0.items()
            if isinstance(e, tuple) and e[0] in ("method", "iface") and e[1][1] in nats}
    if not refs:
        print(f"no methodrefs to {needle!r}")
        sys.exit(0)
    walk(jar, cls, needle, refs)
