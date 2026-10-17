#!/usr/bin/env python3
"""wall_684b: the SECOND probe — Block.boxZ/column folds + the
remaining WallBlock law + <init> float constants + tags."""
import json
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
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
    0x0e: "dconst_0", 0x0f: "dconst_1",
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
                detail = f"tableswitch lo={lo} hi={hi}"
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
                for line in walk_code(cp, code)[:max_pc]:
                    print(line)
            except Exception as ex:
                print(f"   !! desync: {ex}")
    if not hit:
        print(f"!! {cls}: no method matching {needle}")


z = zipfile.ZipFile(JAR)

print("=== Block.boxZ / Block.column folds")
for m in ("boxZ", "column"):
    dump_method(z, "net.minecraft.world.level.block.Block", m, max_pc=120)

print()
print("=== WallBlock <init> (floats resolved)")
dump_method(z, "net.minecraft.world.level.block.WallBlock", "<init>", max_pc=130)

print()
print("=== WallBlock rest")
for m in ("sideUpdate", "topUpdate", "updateSides", "isCovered",
          "isConnected", "isExceptionForConnection"):
    dump_method(z, "net.minecraft.world.level.block.WallBlock", m, max_pc=240)

print()
print("=== WallSide enum")
ms = methods_of(z, "net.minecraft.world.level.block.state.properties.WallSide")
if ms is None:
    print("!! WallSide CLASS ABSENT")
else:
    for sig in ms:
        print(f"--- WallSide.{sig}")

print()
print("=== WALL_POST_OVERRIDE tag")
try:
    j = json.loads(z.read("data/minecraft/tags/block/wall_post_override.json"))
    for v in j.get("values", []):
        print(f"    {v}")
except KeyError:
    print("ABSENT")

print()
print("=== Blocks clinit — COBBLESTONE_WALL rows")
lines = None
ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
for sig, (cp, body) in ms.items():
    if sig.startswith("<clinit>") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        lines = walk_code(cp, body[8:8 + clen])
if lines:
    for i, line in enumerate(lines):
        if "COBBLESTONE_WALL" in line:
            lo = max(0, i - 5)
            hi = min(len(lines), i + 6)
            print(f"--- clinit window @ pc {lines[lo].split(':')[0]}")
            for l in lines[lo:hi]:
                print(l)
