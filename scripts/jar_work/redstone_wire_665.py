#!/usr/bin/env python3
"""THE REDSTONE WIRE'S OWN PROBE (the autopush-665 round): dump the
RedStoneWireBlock class walk — the ctor's Properties row, the canSurvive
support verdict, the getSignal/getDirectSignal power reads, the
getWireColor tint math the dust's own ramp speaks, the connection walk
the model picker serves — plus the Blocks clinit factory row, the Items
clinit REDSTONE row (the BlockItem pairing), the blockstate's variant
key format (the power={0..15} rows the palette arm reads), the dust
model element rows (the y the flat quad wears), and the dust PNGs'
grayscale verdict. javap absent — the JRE's own bytecode the python
walk reads (the same law weighted_plate_49 keeps).

Usage: python3 scripts/jar_work/redstone_wire_665.py <client.jar>
"""
import json
import struct
import sys
import zipfile
import zlib

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
    if e and e[0] == 7:
        return utf(cp, e[1])
    return f"?{i}"


def float_of(raw):
    return struct.unpack(">f", struct.pack(">I", raw))[0]


def disassemble(code, cp, indent="  "):
    """Minimal bytecode printer — the pc-relative walk the javap reads."""
    out = []
    p = 0
    n = len(code)

    def u1():
        nonlocal p
        v = code[p]; p += 1; return v

    def u2():
        nonlocal p
        v = struct.unpack_from(">H", code, p)[0]; p += 2; return v

    def u4():
        nonlocal p
        v = struct.unpack_from(">I", code, p)[0]; p += 4; return v

    while p < n:
        pc = p
        op = u1()
        line = f"{indent}{pc}: "
        if op == 0x00:
            line += "nop"
        elif op == 0x01:
            line += "aconst_null"
        elif op in (0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08):
            line += f"iconst_{op - 0x03}"
        elif op == 0x09:
            line += "lconst_0"
        elif op in (0x0a, 0x0b, 0x0c, 0x0d):
            line += f"fconst_{op - 0x0b}"
        elif op == 0x10:
            line += f"bipush {struct.unpack_from('>b', code, p)[0]}"
            p += 1
        elif op == 0x11:
            line += f"sipush {struct.unpack_from('>h', code, p)[0]}"
            p += 2
        elif op == 0x12:
            line += f"ldc #{u1()}"
        elif op == 0x13:
            line += f"ldc_w #{u2()}"
        elif op in (0x15, 0x17, 0x19, 0x1a, 0x1c, 0x1e, 0x20, 0x22, 0x24, 0x26, 0x28, 0x2a):
            line += f"load_{op:#x} {u1()}"
        elif 0x1b <= op <= 0x2d:
            line += f"load_{op:#x}"
        elif op in (0x36, 0x38, 0x3a, 0x3b, 0x3d, 0x3f, 0x41, 0x43, 0x45, 0x47, 0x49, 0x4b):
            line += f"store_{op:#x} {u1()}"
        elif 0x3c <= op <= 0x4e:
            line += f"store_{op:#x}"
        elif 0x60 <= op <= 0x83:
            line += f"arith_{op:#x}"
        elif 0x84 <= op <= 0x93:
            line += f"conv_{op:#x}"
        elif op == 0x94:
            line += "lcmp"
        elif 0x95 <= op <= 0x98:
            line += f"cmp{op - 0x94}"
        elif 0x99 <= op <= 0xa6:
            off = struct.unpack_from(">h", code, p)[0]; p += 2
            line += f"if_{op:#x} -> {pc + off}"
        elif op in (0xa7, 0xa8):
            off = struct.unpack_from(">h", code, p)[0]; p += 2
            line += f"goto{'_w' if op == 0xa8 else ''} -> {pc + off}"
        elif op in (0xa9,):
            line += "jsr"; p += 2
        elif op == 0xaa:
            p += (4 - (p % 4)) % 4
            df = pc + struct.unpack_from(">i", code, p)[0]; p += 4
            lo = struct.unpack_from(">i", code, p)[0]; p += 4
            hi = struct.unpack_from(">i", code, p)[0]; p += 4
            cnt = hi - lo + 1
            tgts = [pc + struct.unpack_from(">i", code, p + 4 * k)[0] for k in range(cnt)]
            p += 4 * cnt
            line += f"tableswitch {lo}..{hi} default->{df} [" + ", ".join(f"{lo+k}->{t}" for k, t in enumerate(tgts)) + "]"
        elif op == 0xab:
            p += (4 - (p % 4)) % 4
            df = pc + struct.unpack_from(">i", code, p)[0]; p += 4
            npairs = struct.unpack_from(">i", code, p)[0]; p += 4
            pairs = []
            for _ in range(npairs):
                m = struct.unpack_from(">i", code, p)[0]; p += 4
                t = pc + struct.unpack_from(">i", code, p)[0]; p += 4
                pairs.append(f"{m}->{t}")
            line += f"lookupswitch default->{df} [" + ", ".join(pairs) + "]"
        elif op in (0xac, 0xad, 0xae, 0xb0):
            line += f"return_{op:#x}"
        elif op in (0xaf, 0xb0, 0xb1):
            line += f"return_{op:#x}"
        elif op == 0xbb:
            line += f"new #{u2()} {cls_name(cp, u2()) if False else ''}"
        elif op in (0xbc, 0xbe, 0xbf, 0xc0, 0xc1):
            line += f"op_{op:#x} #{u2() if op not in (0xbe,) else ''}"
            if op != 0xbe:
                p = p  # already consumed u2 in format; fix below
        elif op == 0xb2:
            i2 = u2()
            e = cp[i2]
            line += f"getstatic #{i2} ({describe_ref(cp, e)})"
        elif op == 0xb3:
            i2 = u2()
            e = cp[i2]
            line += f"putstatic #{i2} ({describe_ref(cp, e)})"
        elif op == 0xb4:
            i2 = u2()
            e = cp[i2]
            line += f"getfield #{i2} ({describe_ref(cp, e)})"
        elif op == 0xb5:
            i2 = u2()
            e = cp[i2]
            line += f"putfield #{i2} ({describe_ref(cp, e)})"
        elif op in (0xb6, 0xb7, 0xb8):
            i2 = u2()
            e = cp[i2]
            nm = "invokevirtual" if op == 0xb6 else ("invokespecial" if op == 0xb7 else "invokestatic")
            line += f"{nm} #{i2} ({describe_ref(cp, e)})"
        elif op == 0xb9:
            i2 = u2(); u2()
            line += f"invokeinterface #{i2} ({describe_ref(cp, cp[i2])})"
        elif op == 0xba:
            i2 = u2(); u2()
            line += f"invokedynamic #{i2}"
        elif op == 0xbb:
            i2 = u2()
            line += f"new #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xbd:
            i2 = u2()
            line += f"anewarray #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xbe:
            line += "arraylength"
        elif op == 0xbf:
            line += "athrow"
        elif op == 0xc0:
            i2 = u2()
            line += f"checkcast #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xc1:
            i2 = u2()
            line += f"instanceof #{i2} ({cls_name(cp, cp[i2]) if cp[i2] and cp[i2][0] == 7 else '?'})"
        elif op == 0xc4:
            wop = u1()
            if wop in (0x99, 0xa7):
                off = struct.unpack_from(">i", code, p)[0]; p += 4
                line += f"wide if/goto -> {pc + off}"
            else:
                idx = struct.unpack_from(">H", code, p)[0]; p += 2
                line += f"wide {wop:#x} {idx}"
        elif op == 0xc6:
            off = struct.unpack_from(">h", code, p)[0]; p += 2
            line += f"ifnull -> {pc + off}"
        elif op == 0xc7:
            off = struct.unpack_from(">h", code, p)[0]; p += 2
            line += f"ifnonnull -> {pc + off}"
        else:
            line += f"op_{op:#x}"
        out.append(line)
    return out


def describe_ref(cp, e):
    try:
        if e[0] == 9 or e[0] == 10 or e[0] == 11:
            f = cp[e[1]]
            nt = cp[f[2]]
            return f"{cls_name(cp, f[1])}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"
    except Exception:
        pass
    return "?"


def dump_class(z, path, methods_filter=None, label=None):
    print(f"\n{'=' * 70}\n== {label or path}\n{'=' * 70}")
    try:
        d = z.read(path)
    except KeyError:
        print("!! class not found:", path)
        return None
    cp, p = parse_cp(d)
    flags = struct.unpack_from(">H", d, p)[0]; p += 2
    this_i = struct.unpack_from(">H", d, p)[0]; p += 2
    p += 2  # super
    print("class:", cls_name(cp, this_i), "flags", hex(flags))
    # skip interfaces
    icount = struct.unpack_from(">H", d, p)[0]; p += 2
    p += 2 * icount
    fcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(fcount):
        p += 6
    atcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(atcount):
        aname_i, alen = struct.unpack_from(">HI", d, p)[0:2] if False else struct.unpack_from(">H", d, p)[0], struct.unpack_from(">I", d, p + 2)[0]
        p += 6 + alen
    mcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(mcount):
        aflags, nidx, didx = struct.unpack_from(">HHH", d, p)[0:3] if False else struct.unpack_from(">H", d, p)[0], struct.unpack_from(">H", d, p + 2)[0], struct.unpack_from(">H", d, p + 4)[0]
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
        name = utf(cp, nidx)
        desc = utf(cp, didx)
        if methods_filter and not any(name == f or (f.endswith("*") and name.startswith(f[:-1])) for f in methods_filter):
            continue
        if code_attr is None:
            print(f"\n-- {name}{desc} (no code)")
            continue
        mx, ml = struct.unpack_from(">HH", code_attr, 0)[0:2] if False else struct.unpack_from(">H", code_attr, 0)[0], struct.unpack_from(">H", code_attr, 2)[0]
        clen = struct.unpack_from(">I", code_attr, 4)[0]
        code = code_attr[8:8 + clen]
        print(f"\n-- {name}{desc}  (code len {clen})")
        for line in disassemble(code, cp):
            print(line)
    return True


def dump_const_f(z, path):
    """constant pool floats — the tint math's own numbers"""
    d = z.read(path)
    cp, _ = parse_cp(d)
    fl = [(i, float_of(e[1])) for i, e in enumerate(cp) if e and e[0] == 4]
    ints = [(i, struct.unpack(">i", struct.pack(">I", e[1]))[0]) for i, e in enumerate(cp) if e and e[0] == 3]
    print(f"\n-- {path} constant floats:", [f"{i}:{v:.4g}" for i, v in fl][:40])
    print(f"-- {path} constant ints:", [f"{i}:{v}" for i, v in ints][:40])


def main():
    z = zipfile.ZipFile(JAR)

    # 1) the blockstate's variant key format
    print("=" * 70)
    bs = json.loads(z.read("assets/minecraft/blockstates/redstone_wire.json"))
    variants = bs.get("variants", {})
    keys = sorted(variants.keys())
    print(f"redstone_wire.json variants: {len(keys)} keys")
    print("first 6:", keys[:6])
    for k in keys:
        if "power=0" in k:
            print("power=0 sample:", k, "->", variants[k])
            break
    for k in keys:
        if "power=15" in k:
            print("power=15 sample:", k, "->", variants[k])
            break
    dot_models = set()
    for k, v in variants.items():
        val = v if isinstance(v, list) else [v]
        for row in val:
            dot_models.add(row.get("model", "?"))
    print(f"distinct models referenced: {len(dot_models)}")
    for m in sorted(dot_models):
        print("   ", m)

    # 2) the dust models' element rows + tintindex
    for m in ["redstone_dust_dot", "redstone_dust_line0", "redstone_dust_line1"]:
        try:
            mj = json.loads(z.read(f"assets/minecraft/models/block/{m}.json"))
        except KeyError:
            print(f"!! model {m} missing")
            continue
        print(f"\n-- model {m}: parent={mj.get('parent')}")
        print("   textures:", mj.get("textures"))
        for el in mj.get("elements", []):
            print("   element from", el.get("from"), "to", el.get("to"))
            for fn, fd in el.get("faces", {}).items():
                print(f"     face {fn}: tex={fd.get('texture')} tintindex={fd.get('tintindex')} uv={fd.get('uv')}")

    # 3) the dust PNGs — grayscale verdict
    for t in ["redstone_dust_dot", "redstone_dust_line0", "redstone_dust_line1"]:
        raw = z.read(f"assets/minecraft/textures/block/{t}.png")
        w, h = struct.unpack_from(">II", raw, 16)
        print(f"\n-- texture {t}: {w}x{h}, {len(raw)} bytes")
        # find IDAT
        p = 8
        idat = b""
        while p < len(raw):
            ln = struct.unpack_from(">I", raw, p)[0]
            typ = raw[p + 4:p + 8]
            if typ == b"IDAT":
                idat += raw[p + 8:p + 8 + ln]
            p += 12 + ln
        dec = zlib.decompress(idat)
        stride = w * 4 + 1
        px = []
        for row in range(h):
            base = row * stride
            px.append(dec[base + 1:base + 1 + w * 4])
        gray = True
        sample = []
        for y in range(0, h, 4):
            for x in range(0, w, 4):
                r, g, b, a = px[y][x * 4:x * 4 + 4]
                if a > 0 and not (r == g == b):
                    gray = False
                if a > 0 and len(sample) < 6:
                    sample.append((x, y, r, g, b, a))
        print(f"   grayscale={gray}, opaque-pixel samples: {sample}")

    # 4) the class walks
    dump_class(z, "net/minecraft/world/level/block/RedStoneWireBlock.class",
               label="RedStoneWireBlock — the full walk")
    dump_const_f(z, "net/minecraft/world/level/block/RedStoneWireBlock.class")

    # 5) the Blocks clinit row + the Items clinit row (the REDSTONE pairing)
    dump_class(z, "net/minecraft/world/level/block/Blocks.class",
               methods_filter=["clinit*"], label="Blocks.<clinit> — the REDSTONE_WIRE row")
    dump_class(z, "net/minecraft/world/item/Items.class",
               methods_filter=["clinit*"], label="Items.<clinit> — the REDSTONE row")

    # 6) the sounds — the wire's soundtype the Blocks row carries
    try:
        snd = json.loads(z.read("assets/minecraft/sounds.json"))
        for k in snd:
            if "grass" in k:
                print(f"\nsounds.json grass row {k}:", json.dumps(snd[k])[:220])
                break
    except KeyError:
        pass


if __name__ == "__main__":
    main()
