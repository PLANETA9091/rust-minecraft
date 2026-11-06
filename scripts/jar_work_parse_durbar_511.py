#!/usr/bin/env python3
"""durbar_511 — the damage bar's own laws the jar's own bytes speak
(the autopush-487 round, the JRE-free walk the sandbox wears): the
ItemStack.getBarWidth / getBarColor / isBarVisible / isDamaged /
getDamageValue reads the bar's math banks, the GuiGraphics
renderDurabilityBar the strip's geometry + colors the fills carry,
the Mth.hsvToRgb the green-to-red hue fold the color lane wears.
Usage: python3 scripts/jar_work_parse_durbar_511.py >
scripts/jar_work/durbar_511.txt"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts")
from classfile_dump import Reader, parse_constant_pool, cp_utf8, cp_ref  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"

STACK_WANTED = (
    "getBarWidth",
    "getBarColor",
    "isBarVisible",
    "isDamaged",
    "getDamageValue",
    "isDamageableItem",
    "getMaxDamage",
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
    # the ItemStack's own reads (the bar's math the stack keeps)
    scp, smethods = load_class(jar, "net/minecraft/world/item/ItemStack")
    print("== net/minecraft/world/item/ItemStack ==")
    for (nm, desc) in sorted(smethods):
        if any(nm == w for w in STACK_WANTED):
            dump_method(scp, smethods, (nm, desc))
    print()
    # the Item's own defaults (the bar methods the stack delegates to —
    # the virtual dispatch the reads answer)
    icp, imethods = load_class(jar, "net/minecraft/world/item/Item")
    print("== net/minecraft/world/item/Item ==")
    for (nm, desc) in sorted(imethods):
        if nm in STACK_WANTED or nm in ("isDamageableItem", "isDamaged"):
            dump_method(icp, imethods, (nm, desc))
    print()
    # the extractor's own strip (the geometry + the two fills the
    # render lane keeps — the bar's background + color pair); the 26.2
    # GUI lane the GuiGraphicsExtractor class carries
    gcp, gmethods = load_class(jar, "net/minecraft/client/gui/GuiGraphicsExtractor")
    print("== net/minecraft/client/gui/GuiGraphicsExtractor ==")
    for (nm, desc) in sorted(gmethods):
        if "urab" in nm or "ecorat" in nm or nm == "itemBar":
            dump_method(gcp, gmethods, (nm, desc), cap=260)
    print()
    # the Mth's own hue fold (the green-to-red the color lane wears)
    mcp, mmethods = load_class(jar, "net/minecraft/util/Mth")
    print("== net/minecraft/util/Mth ==")
    for (nm, desc) in sorted(mmethods):
        if nm in ("hsvToRgb", "hsvToArgb"):
            dump_method(mcp, mmethods, (nm, desc), cap=140)
    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
