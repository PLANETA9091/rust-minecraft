#!/usr/bin/env python3
"""THE CONCRETE'S OWN PROBE (the autopush-679 round): the builder's
fifteen dyed rungs the palette's tail banks — javap Blocks clinit the
26.2 jar (pinned sha1 2dc72797) the ColorCollection.registerBlocks(...,
BlockItemIds.CONCRETE) row the sixteen-color collection's own walk
speaks, the Items clinit the registerBlock pairing answers, the
fifteen {color}_concrete blockstates the ONE-shape maps bank (the
single "" variant), the fifteen {color}_concrete.json models the
cube_all parent wear (the ONE #all ref), the fifteen {name}.png
masters the jar's own 16x16 colortype-3 tiles speak. The generated
dig/sounds/drops tables the REAL bootstrapped registry's rows cite
(the same law the wool 677 / terracotta 678 probes keep). javap
absent — the JRE's own bytecode the python walk reads.

Usage: python3 scripts/jar_work/concrete_679.py <client.jar>
"""
import json
import re
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"

COLORS = ["orange", "magenta", "light_blue", "yellow", "lime", "pink",
          "gray", "light_gray", "cyan", "purple", "blue", "brown",
          "green", "red", "black"]


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
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc2_w",
    0x15: "iload", 0x19: "aload", 0x1a: "iload_0", 0x1b: "iload_1",
    0x1c: "iload_2", 0x1d: "iload_3", 0x2a: "aload_0", 0x2b: "aload_1",
    0x2c: "aload_2", 0x2d: "aload_3", 0x32: "aaload", 0x36: "istore",
    0x3a: "astore", 0x3b: "istore_0", 0x3c: "istore_1", 0x3d: "istore_2",
    0x3e: "istore_3", 0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2",
    0x4e: "astore_3", 0x57: "pop", 0x58: "pop2", 0x59: "dup",
    0x5a: "dup_x1", 0x5c: "dup2", 0x5f: "swap", 0x60: "iadd",
    0x64: "isub", 0x84: "iinc", 0x99: "ifeq", 0x9a: "ifne", 0x9b: "iflt",
    0x9c: "ifge", 0x9d: "ifgt", 0x9e: "ifle", 0x9f: "if_icmpeq",
    0xa0: "if_icmpne", 0xa1: "if_icmplt", 0xa2: "if_icmpge",
    0xa3: "if_icmpgt", 0xa4: "if_icmple", 0xa5: "if_acmpeq",
    0xa6: "if_acmpne", 0xa7: "goto", 0xac: "ireturn", 0xb0: "areturn",
    0xb1: "return", 0xb2: "getstatic", 0xb3: "putstatic", 0xb4: "getfield",
    0xb5: "putfield", 0xb6: "invokevirtual", 0xb7: "invokespecial",
    0xb8: "invokestatic", 0xb9: "invokeinterface", 0xba: "invokedynamic",
    0xbb: "new", 0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc6: "ifnull", 0xc7: "ifnonnull",
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
            detail = utf(cp, e[1]) if e[0] == "u" else (ref_name(cp, e[1]) if e[0] in (7,) else f"ldc@{idx}")
        elif op in (0x13, 0x14):
            idx = struct.unpack_from(">H", code, j)[0]; j += 2
            e = cp[idx]
            detail = utf(cp, e[1]) if e and e[0] == "u" else f"ldc2w@{idx}"
        elif op in (0x15, 0x19, 0x36, 0x3a):
            detail = str(code[j]); j += 1
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
                j += 12
            else:
                npairs = struct.unpack_from(">i", code, j + 4)[0]
                j += 8 + 8 * npairs
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


def clinit_window(z, cls, needle, span=14):
    ms = methods_of(z, cls)
    if not ms:
        print(f"!! class {cls} not found")
        return
    for sig, (cp, body) in ms.items():
        if not sig.startswith("<clinit>") or not body:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for i, line in enumerate(lines):
            if needle in line:
                lo = max(0, i - 2)
                hi = min(len(lines), i + span)
                print(f"--- clinit window ({needle}) pc {lines[lo].split(':')[0]}..{lines[hi-1].split(':')[0]}")
                for l in lines[lo:hi]:
                    print(l)
                return
    print(f"!! clinit needle not found: {needle}")


z = zipfile.ZipFile(JAR)

print("=== Blocks clinit the CONCRETE collection row")
clinit_window(z, "net.minecraft.world.level.block.Blocks",
              "Blocks.CONCRETE:", span=18)

print()
print("=== Blocks clinit the CONCRETE_POWDER row (the white pair's own walk)")
clinit_window(z, "net.minecraft.world.level.block.Blocks",
              "Blocks.CONCRETE_POWDER:", span=12)

print()
print("=== Items clinit the CONCRETE pairing row")
clinit_window(z, "net.minecraft.world.item.Items",
              "Blocks.CONCRETE:", span=8)

print()
for c in COLORS:
    name = f"{c}_concrete"
    print(f"=== blockstates/{name}.json")
    print(z.read(f"assets/minecraft/blockstates/{name}.json").decode().strip())
    j = json.loads(z.read(f"assets/minecraft/models/block/{name}.json"))
    print(f"--- models/block/{name}.json parent={j.get('parent')} textures={j.get('textures')}")
    raw = z.read(f"assets/minecraft/textures/block/{name}.png")
    w, h = struct.unpack_from(">II", raw, 16)
    ct = raw[25]
    print(f"--- textures/block/{name}.png {w}x{h} colortype-{ct} ({len(raw)} bytes)")
    print()

print("=== the generated block_dig rows (the REAL registry's own verdict)")
src = open("crates/mc-registry/src/generated/block_dig.rs").read()
names = re.search(r"DIG_NAMES: \[&str; DIG_BLOCK_COUNT\] = \[(.*?)\];", src, re.S).group(1)
names = re.findall(r'"([^"]+)"', names)
speeds = re.search(r"DIG_DESTROY_SPEED: \[f32; DIG_BLOCK_COUNT\] = \[(.*?)\];", src, re.S).group(1)
speeds = [float(x) for x in re.findall(r"[-0-9.eE]+", speeds)]
tools = re.search(r"DIG_REQUIRES_TOOL: \[bool; DIG_BLOCK_COUNT\] = \[(.*?)\];", src, re.S).group(1)
tools = [x.strip() == "true" for x in tools.split(",")]
for c in COLORS:
    name = f"{c}_concrete"
    idx = names.index(name)
    print(f"  {name}: destroy_speed={speeds[idx]} requires_tool={tools[idx]}")

print()
print("=== the generated block_sounds rows (SOUND_BREAK column)")
ssrc = open("crates/mc-registry/src/generated/block_sounds.rs").read()
snames = re.findall(r'"([a-z_0-9]+)"', re.search(
    r"SOUND_NAMES: \[&str; SOUND_BLOCK_COUNT\] = \[(.*?)\];", ssrc, re.S).group(1))
sbrk = re.findall(r'"([a-z_.]+)"', re.search(
    r"SOUND_BREAK: \[&str; SOUND_BLOCK_COUNT\] = \[(.*?)\];", ssrc, re.S).group(1))
for c in COLORS:
    i = snames.index(f"{c}_concrete")
    print(f"  {c}_concrete: {sbrk[i]}")

print()
print("=== the generated block_drops rows (self-entry pools)")
dsrc = open("crates/mc-registry/src/generated/block_drops.rs").read()
for c in COLORS:
    n = f"{c}_concrete"
    m = re.search(r"DROPS_" + n.upper() + r": \[DropPool; 1\] = \[(.*?)\];", dsrc, re.S)
    hit = re.search(r'item: "([a-z_]+)"', m.group(1)) if m else None
    print(f"  {n} -> drops {hit.group(1) if hit else '??'}")
