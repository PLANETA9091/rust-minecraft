#!/usr/bin/env python3
"""THE FENCE FAMILY'S OWN PROBE (the dev-55 round): the thirteen
kinds the palette banks (the twelve wood fences + the nether brick
fence) — javap FenceBlock the 26.2 jar (pinned sha1 2dc72797) the
connection law the connectsTo walk speaks (the fence-gate arm, the
solid-side arm, the exception list), the <clinit> the makeShapes
walk the post/arm collision boxes banks (the 0..24 y-band the
anti-jump law wears), the FenceGateBlock.updateShape the in_wall
predicate (the fence the gate raises?), the blockstates the
multipart the when-rules bank, the models the post/side elements
speak, the tags the fences/wooden_fences unions answer, the Blocks
clinit the 13 register rows the Properties chains carry, the Items
clinit the pairing rows bank.

Usage: python3 scripts/jar_work/fence_683.py <client.jar>
"""
import json
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
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x14: "ldc2_w",
    0x15: "iload", 0x19: "aload", 0x1a: "iload_0", 0x1b: "iload_1",
    0x1c: "iload_2", 0x1d: "iload_3", 0x2a: "aload_0", 0x2b: "aload_1",
    0x2c: "aload_2", 0x2d: "aload_3", 0x32: "aaload", 0x36: "istore",
    0x3a: "astore", 0x3b: "istore_0", 0x3c: "istore_1", 0x3d: "istore_2",
    0x3e: "istore_3", 0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2",
    0x4e: "astore_3", 0x57: "pop", 0x58: "pop2", 0x59: "dup",
    0x5a: "dup_x1", 0x5c: "dup2", 0x5f: "swap", 0x60: "iadd",
    0x64: "isub", 0x70: "irem", 0x71: "lrem", 0x84: "iinc", 0x99: "ifeq",
    0x9a: "ifne", 0x9b: "iflt", 0x9c: "ifge", 0x9d: "ifgt", 0x9e: "ifle",
    0x9f: "if_icmpeq", 0xa0: "if_icmpne", 0xa1: "if_icmplt",
    0xa2: "if_icmpge", 0xa3: "if_icmpgt", 0xa4: "if_icmple",
    0xa5: "if_acmpeq", 0xa6: "if_acmpne", 0xa7: "goto", 0xac: "ireturn",
    0xad: "lreturn", 0xae: "freturn", 0xaf: "dreturn", 0xb0: "areturn",
    0xb1: "return", 0xb2: "getstatic", 0xb3: "putstatic", 0xb4: "getfield",
    0xb5: "putfield", 0xb6: "invokevirtual", 0xb7: "invokespecial",
    0xb8: "invokestatic", 0xb9: "invokeinterface", 0xba: "invokedynamic",
    0xbb: "new", 0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc6: "ifnull", 0xc7: "ifnonnull",
    0xd2: "dastore", 0xc2: "monitorenter", 0xc3: "monitorexit",
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


def dump_method(z, cls, needle, max_pc=200):
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
                print(f"   !! walk desync: {ex} — partial dump only")
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


def clinit_window(z, cls, needle, span=7, all_windows=False):
    lines = clinit_lines(z, cls)
    if lines is None:
        print(f"!! {cls} no clinit")
        return
    shown = 0
    for i, line in enumerate(lines):
        if needle in line:
            lo = max(0, i - span)
            hi = min(len(lines), i + span + 1)
            print(f"--- clinit window @ pc {lines[lo].split(':')[0]}..{lines[hi-1].split(':')[0]} ({needle})")
            for l in lines[lo:hi]:
                print(l)
            shown += 1
            if not all_windows and shown >= 2:
                return
    if shown == 0:
        print(f"!! clinit needle not found: {needle}")


z = zipfile.ZipFile(JAR)

print("=== blockstates/oak_fence.json")
try:
    print(z.read("assets/minecraft/blockstates/oak_fence.json").decode())
except KeyError:
    print("ABSENT")
print("=== blockstates/nether_brick_fence.json")
try:
    print(z.read("assets/minecraft/blockstates/nether_brick_fence.json").decode())
except KeyError:
    print("ABSENT")

for m in ("oak_fence_post", "oak_fence_side", "oak_fence_side_wall",
          "nether_brick_fence_post", "nether_brick_fence_side"):
    try:
        j = json.loads(z.read(f"assets/minecraft/models/block/{m}.json"))
        print(f"=== models/block/{m}.json")
        print(json.dumps(j, indent=1))
    except KeyError:
        print(f"=== {m}: ABSENT")

print()
print("=== tags")
for t in ("fences", "wooden_fences", "fence_gates", "walls"):
    for base in ("data/minecraft/tags/block", "data/minecraft/tags/block"):
        try:
            j = json.loads(z.read(f"{base}/{t}.json"))
            vals = j.get("values", [])
            print(f"--- {t}: {len(vals)} values")
            for v in vals:
                print(f"    {v}")
            break
        except KeyError:
            continue
    else:
        print(f"--- {t}: ABSENT")

print()
print("=== FenceBlock fields")
ms = methods_of(z, "net.minecraft.world.level.block.FenceBlock")
if ms is None:
    print("!! FenceBlock CLASS ABSENT")
else:
    for m in ("<init>", "connectsTo", "updateShape", "getConnectionState",
              "isPathfindable", "getShape", "makeShapes", "<clinit>"):
        dump_method(z, "net.minecraft.world.level.block.FenceBlock", m, max_pc=180)

print()
print("=== FenceGateBlock.updateShape — the in_wall predicate (the fence the gate raises?)")
for m in ("updateShape", "isWall"):
    dump_method(z, "net.minecraft.world.level.block.FenceGateBlock", m, max_pc=120)

print()
print("=== Block.isExceptionForConnection — the solid-side law's exception list")
for cls in ("net.minecraft.world.level.block.Block",
            "net.minecraft.world.level.block.state.BlockBehaviour"):
    ms = methods_of(z, cls)
    if ms:
        for sig, (cp, body) in ms.items():
            if "isExceptionForConnection" in sig and body:
                print(f"--- {cls}.{sig}")
                clen = struct.unpack_from(">I", body, 4)[0]
                for line in walk_code(cp, body[8:8 + clen])[:60]:
                    print(line)

print()
print("=== Blocks clinit — the 13 fence register rows")
for needle in ("Blocks.OAK_FENCE =", "Blocks.SPRUCE_FENCE =",
               "Blocks.NETHER_BRICK_FENCE =", "Blocks.WARPED_FENCE ="):
    clinit_window(z, "net.minecraft.world.level.block.Blocks", needle, span=9)

print()
print("=== Items clinit — the fence pairing rows")
for needle in ("BlockItemIds.OAK_FENCE", "BlockItemIds.NETHER_BRICK_FENCE"):
    clinit_window(z, "net.minecraft.world.item.Items", needle, span=7)
