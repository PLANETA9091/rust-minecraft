#!/usr/bin/env python3
"""THE REDSTONE WIRE'S OWN CLASS WALK (the autopush-665 round): the
RedStoneWireBlock methods' bytecode + the Blocks/Items clinit rows.
The parse harness the weighted_plate_49 probe keeps.
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"


def parse_cp(d):
    cpcount = struct.unpack_from(">H", d, 8)[0]
    cp = [None] * cpcount
    p = 10
    i = 1

    def u1():
        nonlocal p
        v = d[p]; p += 1; return v

    def u2():
        nonlocal p
        v = struct.unpack_from(">H", d, p)[0]; p += 2; return v

    def u4():
        nonlocal p
        v = struct.unpack_from(">I", d, p)[0]; p += 4; return v

    while i < cpcount:
        tag = u1()
        if tag == 1:
            ln = u2(); cp[i] = ("u", d[p:p + ln].decode("utf-8", "replace")); p += ln
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, u2())
        elif tag == 15:
            cp[i] = (tag, u1(), u2())
        elif tag in (3, 4):
            cp[i] = (tag, u4())
        elif tag in (5, 6):
            p += 8; i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        else:
            raise Exception(f"tag {tag} @ {i}")
        i += 1
    return cp, p


def utf(cp, i):
    e = cp[i]
    return e[1] if e and e[0] == "u" else f"?{i}"


def cls_name(cp, i):
    e = cp[i]
    return utf(cp, e[1]) if e and e[0] == 7 else f"?{i}"


def describe_ref(cp, e):
    try:
        if e[0] in (9, 10, 11):
            f = cp[e[1]]
            nt = cp[f[2]]
            return f"{cls_name(cp, f[1])}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"
    except Exception:
        pass
    return "?"


def disassemble(code, cp, indent="  "):
    out = []
    p = 0
    n = len(code)

    def u1():
        nonlocal p
        v = code[p]; p += 1; return v

    def u2():
        nonlocal p
        v = struct.unpack_from(">H", code, p)[0]; p += 2; return v

    def s2():
        nonlocal p
        v = struct.unpack_from(">h", code, p)[0]; p += 2; return v

    while p < n:
        pc = p
        op = u1()
        line = f"{indent}{pc}: "
        if op in (0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08):
            line += f"iconst_{op - 3}"
        elif op in (0x0b, 0x0c, 0x0d):
            line += f"fconst_{op - 0x0b}"
        elif op == 0x10:
            line += f"bipush {struct.unpack_from('>b', code, p)[0]}"; p += 1
        elif op == 0x11:
            line += f"sipush {struct.unpack_from('>h', code, p)[0]}"; p += 2
        elif op in (0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d):
            line += f"load {op:#x}"
        elif op in (0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e):
            line += f"store {op:#x}"
        elif op in (0x60, 0x62, 0x64, 0x66, 0x68, 0x6a, 0x6c, 0x6e, 0x70, 0x72, 0x74, 0x76, 0x78, 0x7a, 0x7c, 0x7e, 0x80, 0x82):
            line += f"arith {op:#x}"
        elif op in (0x8d, 0x8e):
            line += f"i2f/l2f {op:#x}"
        elif op == 0x8b:
            line += "i2f"
        elif op in (0x95, 0x96, 0x97, 0x98):
            line += f"cmp {op:#x}"
        elif 0x99 <= op <= 0xa6:
            off = s2()
            line += f"if_{op:#x} -> {pc + off}"
        elif op == 0xa7:
            off = s2()
            line += f"goto -> {pc + off}"
        elif op == 0xaa:
            while (p - 0) % 4:
                u1()
            df = pc + struct.unpack_from(">i", code, p)[0]; p += 4
            lo = struct.unpack_from(">i", code, p)[0]; p += 4
            hi = struct.unpack_from(">i", code, p)[0]; p += 4
            tgts = [(lo + k, pc + struct.unpack_from(">i", code, p + 4 * k)[0]) for k in range(hi - lo + 1)]
            p += 4 * (hi - lo + 1)
            line += f"tableswitch {lo}..{hi} default->{df} [" + ", ".join(f"{m}->{t}" for m, t in tgts) + "]"
        elif op == 0xab:
            while (p - 0) % 4:
                u1()
            df = pc + struct.unpack_from(">i", code, p)[0]; p += 4
            np = struct.unpack_from(">i", code, p)[0]; p += 4
            pairs = []
            for _ in range(np):
                m = struct.unpack_from(">i", code, p)[0]; p += 4
                t = pc + struct.unpack_from(">i", code, p)[0]; p += 4
                pairs.append(f"{m}->{t}")
            line += f"lookupswitch default->{df} [" + ", ".join(pairs) + "]"
        elif op in (0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1):
            line += f"return {op:#x}"
        elif op == 0xb2:
            i2 = u2(); line += f"getstatic #{i2} ({describe_ref(cp, cp[i2])})"
        elif op == 0xb4:
            i2 = u2(); line += f"getfield #{i2} ({describe_ref(cp, cp[i2])})"
        elif op == 0xb5:
            i2 = u2(); line += f"putfield #{i2} ({describe_ref(cp, cp[i2])})"
        elif op in (0xb6, 0xb7, 0xb8):
            i2 = u2()
            nm = {0xb6: "invokevirtual", 0xb7: "invokespecial", 0xb8: "invokestatic"}[op]
            line += f"{nm} #{i2} ({describe_ref(cp, cp[i2])})"
        elif op == 0xba:
            i2 = u2(); u2(); line += f"invokedynamic #{i2}"
        elif op == 0xbb:
            i2 = u2(); line += f"new #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xbd:
            i2 = u2(); line += f"anewarray #{i2}"
        elif op == 0xbe:
            line += "arraylength"
        elif op == 0xc0:
            i2 = u2(); line += f"checkcast #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xc1:
            i2 = u2(); line += f"instanceof #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xc6:
            off = s2(); line += f"ifnull -> {pc + off}"
        elif op == 0xc7:
            off = s2(); line += f"ifnonnull -> {pc + off}"
        elif op == 0x01:
            line += "aconst_null"
        elif op == 0x00:
            line += "nop"
        elif op == 0x53:
            line += "aastore"
        elif op in (0x84,):
            line += "iinc"
            idx = struct.unpack_from(">H", code, p)[0]; p += 2
            c = struct.unpack_from(">b", code, p)[0]; p += 1
            line += f" {idx} {c}"
        else:
            line += f"op_{op:#x}"
            if op in (0x12, 0x13, 0x15, 0x17, 0x19, 0x36, 0x38, 0x3a, 0xa9):
                p += 1
            elif op in (0x14, 0x16, 0x18, 0x37, 0x39, 0x3b + 40):
                pass
            elif op in (0x84,):
                p += 2
        out.append(line)
    return out


def dump_class(z, path, methods_filter=None, label=None):
    print(f"\n{'=' * 70}\n== {label or path}\n{'=' * 70}")
    d = z.read(path)
    cp, p = parse_cp(d)
    p += 4  # flags + this + super handled below
    struct.unpack_from(">H", d, p - 4)
    # re-walk: flags(2) this(2) super(2)
    p = 10 + 2 + 2 + 8  # after parse_cp p points past cp; safer: recompute
    # parse_cp returned p AFTER the constant pool = offset of access_flags
    p = parse_cp(d)[1]
    p += 2  # access flags
    this_i = struct.unpack_from(">H", d, p)[0]; p += 2
    p += 2  # super
    print("class:", cls_name(cp, this_i))
    icount = struct.unpack_from(">H", d, p)[0]; p += 2
    p += 2 * icount
    fcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(fcount):
        p += 6
    atcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(atcount):
        alen = struct.unpack_from(">I", d, p + 2)[0]
        p += 6 + alen
    mcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(mcount):
        struct.unpack_from(">H", d, p)
        nidx = struct.unpack_from(">H", d, p + 2)[0]
        didx = struct.unpack_from(">H", d, p + 4)[0]
        p += 6
        acount = struct.unpack_from(">H", d, p)[0]; p += 2
        code_attr = None
        for _ in range(acount):
            anidx = struct.unpack_from(">H", d, p)[0]
            alen = struct.unpack_from(">I", d, p + 2)[0]
            adata = d[p + 6:p + 6 + alen]
            if utf(cp, anidx) == "Code":
                code_attr = adata
            p += 6 + alen
        name, desc = utf(cp, nidx), utf(cp, didx)
        if methods_filter and not any(name == f or (f.endswith("*") and name.startswith(f[:-1])) for f in methods_filter):
            continue
        if code_attr is None:
            print(f"\n-- {name}{desc} (no code)")
            continue
        clen = struct.unpack_from(">I", code_attr, 4)[0]
        code = code_attr[8:8 + clen]
        print(f"\n-- {name}{desc}  (code len {clen})")
        for line in disassemble(code, cp):
            print(line)


def dump_consts(z, path, label):
    d = z.read(path)
    cp, _ = parse_cp(d)
    fl = [(i, struct.unpack(">f", struct.pack(">I", e[1]))[0]) for i, e in enumerate(cp) if e and e[0] == 4]
    print(f"\n== {label} constant floats:", [f"{i}:{v:.4g}" for i, v in fl])
    ints = [(i, struct.unpack(">i", struct.pack(">I", e[1]))[0]) for i, e in enumerate(cp) if e and e[0] == 3]
    print(f"== {label} constant ints:", [f"{i}:{v}" for i, v in ints][:30])


z = zipfile.ZipFile(JAR)
dump_class(z, "net/minecraft/world/level/block/RedStoneWireBlock.class",
           label="RedStoneWireBlock")
dump_consts(z, "net/minecraft/world/level/block/RedStoneWireBlock.class", "wire")
