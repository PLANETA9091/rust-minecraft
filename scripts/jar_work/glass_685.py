#!/usr/bin/env python3
"""THE GLASS FAMILY'S OWN PROBE (the dev-57 round): the seventeen kinds
the palette banks (the glass the window pane's own parent + the sixteen
stained cloths the DyeColor ladder wears) — javap GlassBlock /
StainedGlassBlock / HalfTransparentBlock the 26.2 jar (pinned sha1
2dc72797) the skipRendering law the same-block cull speaks, the ctor
chains the strength(0.3) + noOcclusion + sound(GLASS) rows bank, the
Blocks clinit the 17 register rows the Properties chains carry, the
blockstates the ONE-variant maps wear, the models the cube_all parents
speak, the textures the alpha histograms answer (the cutout zeros the
glass wears, the translucent band the stained cloths keep), the loot
tables the empty survival drops bank, the sounds.json the block.glass.*
events carry, the mineable/pickaxe tag the membership answers, the Items
clinit the pairing rows bank.

Usage: python3 scripts/jar_work/glass_685.py <client.jar>
"""
import json
import struct
import sys
import zipfile
import zlib

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"


# ---------------------------------------------------------------- PNG
def png_pixels(data):
    """Minimal PNG decoder (bit depth 8, colortype 0/2/3/4/6 + tRNS).
    Returns (width, height, flat RGBA list)."""
    assert data[:8] == b"\x89PNG\r\n\x1a\n", "not a png"
    p = 8
    width = height = None
    bitd = ctype = None
    palette = []
    trns = b""
    idat = b""
    while p < len(data):
        ln = struct.unpack_from(">I", data, p)[0]
        typ = data[p + 4:p + 8]
        chunk = data[p + 8:p + 8 + ln]
        if typ == b"IHDR":
            width, height, bitd, ctype = struct.unpack_from(">IIBB", chunk, 0)
        elif typ == b"PLTE":
            palette = [tuple(chunk[i:i + 3]) for i in range(0, len(chunk), 3)]
        elif typ == b"tRNS":
            trns = chunk
        elif typ == b"IDAT":
            idat += chunk
        elif typ == b"IEND":
            break
        p += 12 + ln
    assert bitd == 8, f"bit depth {bitd} unsupported"
    nch = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[ctype]
    raw = zlib.decompress(idat)
    stride = width * nch
    px = bytearray()
    prev = bytearray(stride)
    pos = 0
    for _ in range(height):
        f = raw[pos]; pos += 1
        line = bytearray(raw[pos:pos + stride]); pos += stride
        if f == 1:
            for i in range(nch, stride):
                line[i] = (line[i] + line[i - nch]) & 0xFF
        elif f == 2:
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 0xFF
        elif f == 3:
            for i in range(stride):
                a = line[i - nch] if i >= nch else 0
                line[i] = (line[i] + ((a + prev[i]) >> 1)) & 0xFF
        elif f == 4:
            for i in range(stride):
                a = line[i - nch] if i >= nch else 0
                b = prev[i]
                c = prev[i - nch] if i >= nch else 0
                pp = a + b - c
                pa, pb, pc = abs(pp - a), abs(pp - b), abs(pp - c)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[i] = (line[i] + pr) & 0xFF
        px += line
        prev = line
    rgba = []
    for y in range(height):
        for x in range(width):
            o = (y * width + x) * nch
            if ctype == 0:
                v = px[o]; rgba += [v, v, v, 255]
            elif ctype == 2:
                rgba += [px[o], px[o + 1], px[o + 2], 255]
            elif ctype == 3:
                idx = px[o]
                r, g, b = palette[idx]
                a = trns[idx] if idx < len(trns) else 255
                rgba += [r, g, b, a]
            elif ctype == 4:
                rgba += [px[o], px[o], px[o], px[o + 1]]
            elif ctype == 6:
                rgba += [px[o], px[o + 1], px[o + 2], px[o + 3]]
    return width, height, rgba


def alpha_hist(rgba):
    h = {}
    for i in range(3, len(rgba), 4):
        h[rgba[i]] = h.get(rgba[i], 0) + 1
    return dict(sorted(h.items()))


# ------------------------------------------------------- class parse
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
            raw = d[p:p + 8]; p += 8
            cp[i] = (tag, struct.unpack(">d" if tag == 6 else ">q", raw)[0]); i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        else:
            raise Exception(f"tag {tag} @ {i}")
        i += 1
    return cp, p


def utf(cp, i):
    e = cp[i]
    return e[1] if e and e[0] == "u" else f"?{i}"


def ref_name(cp, i):
    e = cp[i]
    cls = utf(cp, cp[e[1]][1])
    nt = cp[e[2]]
    return f"{cls}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"


MNEMONIC = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x0b: "fconst_0", 0x0c: "fconst_1", 0x0d: "fconst_2",
    0x0e: "dconst_0", 0x0f: "dconst_1",
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x14: "ldc2_w",
    0x15: "iload", 0x19: "aload", 0x1a: "iload_0", 0x1b: "iload_1",
    0x1c: "iload_2", 0x1d: "iload_3", 0x1e: "lload_0", 0x2a: "aload_0",
    0x2b: "aload_1", 0x2c: "aload_2", 0x2d: "aload_3",
    0x32: "aaload", 0x36: "istore", 0x3a: "astore", 0x3b: "istore_0",
    0x3c: "istore_1", 0x3d: "istore_2", 0x3e: "istore_3",
    0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2", 0x4e: "astore_3",
    0x57: "pop", 0x58: "pop2", 0x59: "dup", 0x5a: "dup_x1", 0x5c: "dup2",
    0x5f: "swap", 0x60: "iadd", 0x64: "isub", 0x70: "irem", 0x71: "lrem",
    0x84: "iinc", 0x99: "ifeq", 0x9a: "ifne", 0x9b: "iflt", 0x9c: "ifge",
    0x9d: "ifgt", 0x9e: "ifle", 0x9f: "if_icmpeq", 0xa0: "if_icmpne",
    0xa1: "if_icmplt", 0xa2: "if_icmpge", 0xa3: "if_icmpgt", 0xa4: "if_icmple",
    0xa5: "if_acmpeq", 0xa6: "if_acmpne", 0xa7: "goto", 0xac: "ireturn",
    0xad: "lreturn", 0xae: "freturn", 0xaf: "dreturn", 0xb0: "areturn",
    0xb1: "return", 0xb2: "getstatic", 0xb3: "putstatic", 0xb4: "getfield",
    0xb5: "putfield", 0xb6: "invokevirtual", 0xb7: "invokespecial",
    0xb8: "invokestatic", 0xb9: "invokeinterface", 0xba: "invokedynamic",
    0xbb: "new", 0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc6: "ifnull", 0xc7: "ifnonnull",
    0xc2: "monitorenter", 0xc3: "monitorexit",
}


def walk_code(cp, code):
    j = 0
    out = []
    while j < len(code):
        pc = j
        op = code[j]; j += 1
        mnem = MNEMONIC.get(op, f"0x{op:02x}")
        detail = ""
        if op == 0x10:
            detail = str(struct.unpack_from(">b", code, j)[0]); j += 1
        elif op == 0x11:
            detail = str(struct.unpack_from(">h", code, j)[0]); j += 2
        elif op == 0x12:
            idx = code[j]; j += 1
            e = cp[idx]
            if e[0] == "u":
                detail = utf(cp, e[1])
            elif e[0] == 8:
                detail = utf(cp, e[1])
            elif e[0] == 7:
                detail = utf(cp, cp[e[1]][1])
            elif e[0] == 3:
                detail = f"int {struct.unpack('>i', struct.pack('>I', e[1]))[0]}"
            elif e[0] == 4:
                detail = f"float {struct.unpack('>f', struct.pack('>I', e[1]))[0]}"
            elif e[0] in (9, 10, 11, 12):
                detail = ref_name(cp, idx)
            else:
                detail = f"ldc@{idx}"
        elif op in (0x13, 0x14):
            idx = struct.unpack_from(">H", code, j)[0]; j += 2
            e = cp[idx]
            if e[0] == 3:
                detail = f"int {struct.unpack('>i', struct.pack('>I', e[1]))[0]}"
            elif e[0] == 4:
                detail = f"float {struct.unpack('>f', struct.pack('>I', e[1]))[0]}"
            elif e[0] == 5:
                detail = f"long {e[1]}"
            elif e[0] == 6:
                detail = f"double {e[1]}"
            elif e[0] == "u":
                detail = utf(cp, e[1])
            else:
                detail = f"ldcw@{idx}"
        elif op in (0x15, 0x17, 0x19, 0x21, 0x23, 0x25, 0x36, 0x38, 0x3a, 0x42, 0x44, 0x46):
            detail = f"{MNEMONIC.get(op, hex(op))} {code[j]}"; j += 1
        elif op == 0x84:
            a, b = struct.unpack_from(">bb", code, j); j += 2
            detail = f"{a} {b}"
        elif op in (0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1,
                    0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xc6, 0xc7):
            off = struct.unpack_from(">h", code, j)[0]; j += 2
            detail = f"-> {pc + off}"
        elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xbb,
                    0xbd, 0xc0, 0xc1):
            idx = struct.unpack_from(">H", code, j)[0]; j += 2
            e = cp[idx]
            if e[0] in (9, 10, 11, 12):
                detail = ref_name(cp, idx)
            elif e[0] == 7:
                detail = utf(cp, e[1])
            else:
                detail = f"cp@{idx}"
        elif op == 0xba:
            j += 4; detail = "indy"
        elif op == 0xbc:
            j += 1
        elif op == 0xc5:
            j += 3
        elif op in (0xaa, 0xab):
            pad = (4 - (j % 4)) % 4
            j += pad
            if op == 0xaa:
                default, lo, hi = struct.unpack_from(">iii", code, j)
                j += 12 + 4 * (hi - lo + 1)
                detail = f"tableswitch lo={lo} hi={hi} default={default}"
            else:
                npairs = struct.unpack_from(">i", code, j + 4)[0]
                j += 8 + 8 * npairs
                detail = f"lookupswitch npairs={npairs}"
        out.append(f"{pc}: {mnem} {detail}".rstrip())
    return out


def methods_of(z, cls):
    path = f"{cls.replace('.', '/')}.class"
    try:
        d = z.read(path)
    except KeyError:
        return None
    cp, p = parse_cp(d)
    p += 6
    ifc_n = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc_n
    f_n = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(f_n):
        p += 6
        attrs = struct.unpack_from(">H", d, p)[0]; p += 2
        for _ in range(attrs):
            ln = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + ln
    m_n = struct.unpack_from(">H", d, p)[0]; p += 2
    out = {}
    for _ in range(m_n):
        access = struct.unpack_from(">H", d, p)[0]; p += 2
        name_i = struct.unpack_from(">H", d, p)[0]; p += 2
        desc_i = struct.unpack_from(">H", d, p)[0]; p += 2
        attrs = struct.unpack_from(">H", d, p)[0]; p += 2
        body = None
        for _ in range(attrs):
            an_i = struct.unpack_from(">H", d, p)[0]
            ln = struct.unpack_from(">I", d, p + 2)[0]
            name = utf(cp, an_i)
            if name == "Code" and body is None:
                body = d[p + 6:p + 6 + ln]
            p += 6 + ln
        out[f"{utf(cp, name_i)}{utf(cp, desc_i)}"] = (cp, body)
    return out


def class_header(z, cls):
    path = f"{cls.replace('.', '/')}.class"
    try:
        d = z.read(path)
    except KeyError:
        return None
    cp, p = parse_cp(d)
    access = struct.unpack_from(">H", d, p)[0]
    this_i = struct.unpack_from(">H", d, p + 2)[0]
    super_i = struct.unpack_from(">H", d, p + 4)[0]
    p += 6
    ifc_n = struct.unpack_from(">H", d, p)[0]; p += 2
    ifcs = [utf(cp, struct.unpack_from(">H", d, p + 2 * k)[0]) for k in range(ifc_n)]
    return this_i and (utf(cp, this_i), utf(cp, super_i) if super_i else None, ifcs)


def dump_method(z, cls, needle, max_pc=140):
    ms = methods_of(z, cls)
    if not ms:
        print(f"!! class {cls} not found")
        return
    hit = False
    for sig, (cp, body) in ms.items():
        if needle.split("(")[0] in sig.split("(")[0] and body:
            hit = True
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            print(f"--- {cls}.{sig}")
            try:
                lines = walk_code(cp, code)
            except Exception as ex:
                print(f"   !! walk desync: {ex}")
                lines = None
            if lines:
                for line in lines[:max_pc]:
                    print(line)
    if not hit:
        print(f"!! {cls}: no method matching {needle}")


def clinit_lines(z, cls):
    ms = methods_of(z, cls)
    if not ms:
        return None
    for sig, (cp, body) in ms.items():
        if not sig.startswith("<clinit>") or not body:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        return walk_code(cp, code)
    return None


def clinit_window(z, cls, needle, span=9, hits=1):
    lines = clinit_lines(z, cls)
    if lines is None:
        print(f"!! {cls} no clinit")
        return
    shown = 0
    for i, line in enumerate(lines):
        if needle in line:
            lo = max(0, i - span)
            hi = min(len(lines), i + span + 1)
            print(f"--- clinit window @ {needle} (pc {lines[lo].split(':')[0]}..)")
            for l in lines[lo:hi]:
                print(l)
            shown += 1
            if shown >= hits:
                return
    if shown == 0:
        print(f"!! clinit needle not found: {needle}")


z = zipfile.ZipFile(JAR)

print("=== class headers")
for c in ("net.minecraft.world.level.block.GlassBlock",
          "net.minecraft.world.level.block.StainedGlassBlock",
          "net.minecraft.world.level.block.HalfTransparentBlock",
          "net.minecraft.world.level.block.TransparentBlock"):
    h = class_header(z, c)
    print(f"--- {c}: {h}")

print()
print("=== HalfTransparentBlock methods")
for m in ("<init>", "skipRendering", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.HalfTransparentBlock", m, max_pc=40)

print()
print("=== GlassBlock methods")
for m in ("<init>", "skipRendering", "<clinit>"):
    dump_method(z, "net.minecraft.world.level.block.GlassBlock", m, max_pc=40)

print()
print("=== StainedGlassBlock methods")
for m in ("<init>", "skipRendering", "getColor"):
    dump_method(z, "net.minecraft.world.level.block.StainedGlassBlock", m, max_pc=30)

print()
print("=== Blocks clinit — the glass register rows")
clinit_window(z, "net.minecraft.world.level.block.Blocks", "GLASS;putstatic", hits=2)
clinit_window(z, "net.minecraft.world.level.block.Blocks", "WHITE_STAINED_GLASS;putstatic", hits=1)

print()
print("=== blockstates + models")
for name in ("glass", "white_stained_glass", "orange_stained_glass", "black_stained_glass"):
    try:
        print(f"--- blockstates/{name}.json")
        print(z.read(f"assets/minecraft/blockstates/{name}.json").decode())
    except KeyError:
        print("ABSENT")
    try:
        j = json.loads(z.read(f"assets/minecraft/models/block/{name}.json"))
        print(f"--- models/block/{name}.json: {json.dumps(j)}")
    except KeyError:
        print(f"--- models/block/{name}.json ABSENT")

print()
print("=== texture alpha histograms (the cutout/translucent evidence)")
for name in ("glass", "white_stained_glass", "orange_stained_glass",
             "magenta_stained_glass", "light_blue_stained_glass",
             "yellow_stained_glass", "lime_stained_glass",
             "pink_stained_glass", "gray_stained_glass",
             "light_gray_stained_glass", "cyan_stained_glass",
             "purple_stained_glass", "blue_stained_glass",
             "brown_stained_glass", "green_stained_glass",
             "red_stained_glass", "black_stained_glass"):
    key = f"assets/minecraft/textures/block/{name}.png"
    try:
        w, h, rgba = png_pixels(z.read(key))
        hist = alpha_hist(rgba)
        print(f"--- {name}: {w}x{h} alpha {hist}")
    except KeyError:
        print(f"--- {name}: ABSENT")

print()
print("=== loot tables (the survival drop law)")
for name in ("glass", "white_stained_glass"):
    for base in ("data/minecraft/loot_table/blocks", "data/minecraft/loot_tables/blocks"):
        try:
            print(f"--- {name} loot: {z.read(f'{base}/{name}.json').decode()}")
            break
        except KeyError:
            continue
    else:
        print(f"--- {name}: loot ABSENT")

print()
print("=== sounds.json block.glass.* events")
try:
    j = json.loads(z.read("assets/minecraft/sounds.json"))
    for k in sorted(j):
        if k.startswith("block.glass."):
            print(f"    {k}: {json.dumps(j[k])[:140]}")
except KeyError:
    print("sounds.json ABSENT")

print()
print("=== tags")
for t in ("mineable/pickaxe", "glass", "wither_immune"):
    try:
        j = json.loads(z.read(f"data/minecraft/tags/block/{t.split('/')[-1]}.json")) if False else None
    except KeyError:
        pass
for t in ("mineable/pickaxe", "glass", "impermeable"):
    path = f"data/minecraft/tags/block/{t}.json"
    try:
        j = json.loads(z.read(path))
        vals = j.get("values", [])
        glassy = [v for v in vals if "glass" in v or t == "impermeable"]
        print(f"--- tag {t}: {len(vals)} values; glassy rows: {glassy if t != 'impermeable' else vals}")
    except KeyError:
        print(f"--- tag {t}: ABSENT")

print()
print("=== Items clinit — the BlockItem pairing rows")
clinit_window(z, "net.minecraft.world.item.Items", "GLASS;putstatic", hits=2)
