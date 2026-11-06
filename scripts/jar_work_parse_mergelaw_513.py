#!/usr/bin/env python3
"""mergelaw_513 — the enchanted stack's own refuse-to-merge law the
jar's own bytes speak (the autopush-490 round, the JRE-free walk the
sandbox wears): the Inventory.hasRemainingSpaceForItem the merge
candidate's component gate banks, the Inventory.getSlotWithRemaining
Space the scan the gate rides, the Inventory.add/addResource the walk
the rides re-wear, the ItemStack.isSameItemSameComponents the
components equality the gate answers (the enchanted stacks' own
components the plain stacks never share — the refuse-to-merge verdict
the port's gate law wears).
Usage: python3 scripts/jar_work_parse_mergelaw_513.py >
scripts/jar_work/mergelaw_513.txt"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts")
from classfile_dump import Reader, parse_constant_pool, cp_utf8, cp_ref  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"

INV_WANTED = (
    "hasRemainingSpaceForItem",
    "getSlotWithRemainingSpace",
    "addResource",
    "getFreeSlot",
)

STACK_WANTED = (
    "isSameItemSameComponents",
    "isSameItem",
    "isStackable",
    "areComponentsEqual",
)


def load_class(jar, name):
    data = jar.read(f"{name}.class")
    r = Reader(data)
    assert r.u4() == 0xCAFEBABE
    r.u2(); r.u2()
    cp = parse_constant_pool(r)
    r.u2(); r.u2(); r.u2()
    for _ in range(r.u2()):
        r.u2()
    fields_count = r.u2()
    for _ in range(fields_count):
        r.u2(); r.u2(); r.u2()
        for _ in range(r.u2()):
            r.u2()
            alen = r.u4(); r.raw(alen)
    methods = {}
    for _ in range(r.u2()):
        r.u2(); name = cp_utf8(cp, r.u2()); desc = cp_utf8(cp, r.u2())
        attrs = {}
        for _ in range(r.u2()):
            aname = cp_utf8(cp, r.u2()); alen = r.u4(); attrs[aname] = r.raw(alen)
        methods[(name, desc)] = attrs
    return cp, methods


def code_bytes(methods, pair):
    attrs = methods.get(pair)
    if attrs is None or "Code" not in attrs:
        return None
    a = attrs["Code"]
    clen = struct.unpack_from(">I", a, 4)[0]
    return a[8:8 + clen]


class Op:
    __slots__ = ("op", "a", "pc")

    def __init__(self, op, a, pc):
        self.op = op
        self.a = a
        self.pc = pc


def decode(code, cp):
    ops = []
    i, n = 0, len(code)
    while i < n:
        pc = i
        op = code[i]; i += 1
        a = None
        if op == 0x10:
            a = struct.unpack_from(">b", code, i)[0]; i += 1
        elif op == 0x11:
            a = struct.unpack_from(">h", code, i)[0]; i += 2
        elif 0x02 <= op <= 0x08:
            a = op - 0x03
        elif op in (0x12,):
            cp_i = code[i]; i += 1
            t = cp[cp_i][0]
            a = ("int", cp[cp_i][1]) if t in ("Integer", "Float") else ("other", None)
        elif op in (0x13, 0x14):
            cp_i = struct.unpack_from(">H", code, i)[0]; i += 2
            t = cp[cp_i][0]
            a = ("int", cp[cp_i][1]) if t in ("Integer", "Float", "Double") else ("other", None)
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3A, 0xA9):
            a = code[i]; i += 1
        elif op == 0x84:
            i += 2
        elif op in (0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBB, 0xBD,
                    0xC0, 0xC1, 0xC5):
            idx = struct.unpack_from(">H", code, i)[0]; i += 2
            if op == 0xB9:
                i += 2
            elif op == 0xC5:
                i += 1
            if op in (0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9):
                a = ("ref", cp_ref(cp, idx))
            else:
                a = ("cls", cp_utf8(cp, cp[idx][1]) if isinstance(cp[idx][1], int) else cp[idx][1])
        elif op == 0xBA:
            i += 4
        elif op == 0xBC:
            i += 1
        elif op == 0xC4:
            wop = code[i]
            i += 5 if wop == 0x84 else 3
        elif op in (0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2,
                    0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xC6, 0xC7):
            a = pc + struct.unpack_from(">h", code, i)[0]; i += 2
        elif op in (0xAA, 0xAB):
            pad = (4 - (i % 4)) % 4
            i += pad
            dflt, = struct.unpack_from(">i", code, i)
            if op == 0xAA:
                lo, hi = struct.unpack_from(">ii", code, i + 4)
                i += 8 + 4 * (hi - lo + 1)
            else:
                n2, = struct.unpack_from(">i", code, i + 4)
                i += 8 + 8 * n2
            a = ("switch", dflt + pc)
        ops.append(Op(op, a, pc))
    return ops


def ref_str(a):
    if not isinstance(a, tuple):
        return "" if a is None else str(a)
    if a[0] == "ref":
        owner, m, d = a[1]
        return f"{owner.split('/')[-1]}.{m}"
    if a[0] == "cls":
        return a[1].split("/")[-1]
    if a[0] == "int":
        return str(a[1])
    return ""


def fmt_op(o):
    return f"  {o.pc}: {o.op:02x} {ref_str(o.a)}"


def dump_method(cp, methods, pair, cap=200):
    cb = code_bytes(methods, pair)
    if cb is None:
        print(f"  METHOD {pair[0]} {pair[1]}  (absent or no code)")
        return
    aops = decode(cb, cp)
    print(f"  METHOD {pair[0]} {pair[1]}  (code={len(cb)})")
    for w in aops[:cap]:
        print(fmt_op(w))
    if len(aops) > cap:
        print(f"  ... ({len(aops) - cap} more ops)")


def main():
    jar = zipfile.ZipFile(JAR)
    # the Inventory's own merge scan (the candidate's gate the walk rides)
    icp, imethods = load_class(jar, "net/minecraft/world/entity/player/Inventory")
    print("== net/minecraft/world/entity/player/Inventory ==")
    for (nm, desc) in sorted(imethods):
        if nm in INV_WANTED:
            dump_method(icp, imethods, (nm, desc), cap=260)
    print()
    # the ItemStack's own components equality (the gate's verdict)
    scp, smethods = load_class(jar, "net/minecraft/world/item/ItemStack")
    print("== net/minecraft/world/item/ItemStack ==")
    for (nm, desc) in sorted(smethods):
        if nm in STACK_WANTED:
            dump_method(scp, smethods, (nm, desc), cap=200)
    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
