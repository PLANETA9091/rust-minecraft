#!/usr/bin/env python3
"""THE ACCESSIBILITY SCREEN'S OWN SEAT PROBE (the autopush-176 round):
the sub-screen the fovEffectScale row hosts — javap-equivalent walks of
`net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen`
(the addOptions/init/removed bodies + the constant pool's utf8 words),
the OptionsScreen's own accessibility button row (the init walk the
`options.accessibility` button creation wears), and the Options
clinit's fovEffectScale registration (the label key + the UnitDouble
value formatter the slider's caption speaks).

Usage: python3 scripts/jar_work/access_screen_262.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"

MNEMONIC = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x09: "lconst_0", 0x0a: "lconst_1", 0x0b: "fconst_0",
    0x0c: "fconst_1", 0x0d: "fconst_2", 0x0e: "dconst_0", 0x0f: "dconst_1",
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x14: "ldc2_w",
    0x15: "iload", 0x16: "lload", 0x17: "fload", 0x18: "dload", 0x19: "aload",
    0x1a: "iload_0", 0x1b: "iload_1", 0x1c: "iload_2", 0x1d: "iload_3",
    0x1e: "lload_0", 0x1f: "lload_1", 0x20: "lload_2", 0x21: "lload_3",
    0x22: "fload_0", 0x23: "fload_1", 0x24: "fload_2", 0x25: "fload_3",
    0x26: "dload_0", 0x27: "dload_1", 0x28: "dload_2", 0x29: "dload_3",
    0x2a: "aload_0", 0x2b: "aload_1", 0x2c: "aload_2", 0x2d: "aload_3",
    0x2e: "iaload", 0x2f: "laload", 0x30: "faload", 0x31: "daload",
    0x32: "aaload", 0x33: "baload", 0x34: "caload", 0x35: "saload",
    0x36: "istore", 0x37: "lstore", 0x38: "fstore", 0x39: "dstore",
    0x3a: "astore", 0x3b: "istore_0", 0x3c: "istore_1", 0x3d: "istore_2",
    0x3e: "istore_3", 0x3f: "lstore_0", 0x40: "lstore_1", 0x41: "lstore_2",
    0x42: "lstore_3", 0x43: "fstore_0", 0x44: "fstore_1", 0x45: "fstore_2",
    0x46: "fstore_3", 0x47: "dstore_0", 0x48: "dstore_1", 0x49: "dstore_2",
    0x4a: "dstore_3", 0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2",
    0x4e: "astore_3", 0x4f: "iastore", 0x50: "lastore", 0x51: "fastore",
    0x52: "dastore", 0x53: "aastore", 0x54: "bastore", 0x55: "castore",
    0x56: "sastore", 0x57: "pop", 0x58: "pop2", 0x59: "dup", 0x5a: "dup_x1",
    0x5b: "dup_x2", 0x5c: "dup2", 0x5d: "dup2_x1", 0x5e: "dup2_x2",
    0x5f: "swap", 0x60: "iadd", 0x61: "ladd", 0x62: "fadd", 0x63: "dadd",
    0x64: "isub", 0x65: "lsub", 0x66: "fsub", 0x67: "dsub",
    0x68: "imul", 0x69: "lmul", 0x6a: "fmul", 0x6b: "dmul",
    0x6c: "idiv", 0x6d: "ldiv", 0x6e: "fdiv", 0x6f: "ddiv",
    0x70: "irem", 0x71: "lrem", 0x72: "frem", 0x73: "drem",
    0x74: "ineg", 0x75: "lneg", 0x76: "fneg", 0x77: "dneg",
    0x78: "ishl", 0x79: "lshl", 0x7a: "ishr", 0x7b: "lshr",
    0x7c: "iushr", 0x7d: "lushr", 0x7e: "iand", 0x7f: "land",
    0x80: "ior", 0x81: "lor", 0x82: "ixor", 0x83: "lxor", 0x84: "iinc",
    0x85: "i2l", 0x86: "i2f", 0x87: "i2d", 0x88: "l2i", 0x89: "l2f",
    0x8a: "l2d", 0x8b: "f2i", 0x8c: "f2l", 0x8d: "f2d", 0x8e: "d2i",
    0x8f: "d2l", 0x90: "d2f", 0x91: "i2b", 0x92: "i2c", 0x93: "i2s",
    0x94: "lcmp", 0x95: "fcmpl", 0x96: "fcmpg", 0x97: "dcmpl",
    0x98: "dcmpg", 0x99: "ifeq", 0x9a: "ifne", 0x9b: "iflt", 0x9c: "ifge",
    0x9d: "ifgt", 0x9e: "ifle", 0x9f: "if_icmpeq", 0xa0: "if_icmpne",
    0xa1: "if_icmplt", 0xa2: "if_icmpge", 0xa3: "if_icmpgt",
    0xa4: "if_icmple", 0xa5: "if_acmpeq", 0xa6: "if_acmpne",
    0xa7: "goto", 0xa8: "jsr", 0xa9: "ret", 0xaa: "tableswitch",
    0xab: "lookupswitch", 0xac: "ireturn", 0xad: "lreturn", 0xae: "freturn",
    0xaf: "dreturn", 0xb0: "areturn", 0xb1: "return",
    0xb2: "getstatic", 0xb3: "putstatic", 0xb4: "getfield", 0xb5: "putfield",
    0xb6: "invokevirtual", 0xb7: "invokespecial", 0xb8: "invokestatic",
    0xb9: "invokeinterface", 0xba: "invokedynamic",
    0xbb: "new", 0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc6: "ifnull", 0xc7: "ifnonnull",
    0xd2: "dastore", 0xc2: "monitorenter", 0xc3: "monitorexit",
}


def parse_cp(d):
    cpcount = struct.unpack_from(">H", d, 8)[0]
    cp = [None] * cpcount
    p = 10
    i = 1
    while i < cpcount:
        tag = d[p]; p += 1
        if tag == 1:
            ln = struct.unpack_from(">H", d, p)[0]; p += 2
            cp[i] = ("u", d[p:p + ln]); p += ln
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, struct.unpack_from(">H", d, p)[0]); p += 2
        elif tag == 15:
            cp[i] = (tag, d[p:p + 3]); p += 3
        elif tag in (5, 6):
            cp[i] = (tag, struct.unpack_from(">q", d, p)[0]); p += 8
            i += 1
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            cp[i] = (tag, struct.unpack_from(">I", d, p)[0]); p += 4
        else:
            raise ValueError(f"tag {tag} at cp {i}")
        i += 1
    return cp, p


def utf(cp, i):
    return cp[i][1].decode("utf-8", "replace")


def ref_name(cp, i):
    e = cp[i]
    cls_idx, nt_idx = struct.unpack(">HH", struct.pack(">I", e[1]))
    cls = utf(cp, cp[cls_idx][1]) if cp[cls_idx][0] == 7 else f"cls@{cls_idx}"
    nt = cp[nt_idx]
    name_idx, desc_idx = struct.unpack(">HH", struct.pack(">I", nt[1]))
    try:
        return f"{cls}.{utf(cp, name_idx)}{utf(cp, desc_idx)}"
    except Exception:
        return f"{cls}.nt@{nt_idx}"


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


def dump_method(z, cls, needle, max_pc=400):
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


def class_utf8_words(z, cls, needles):
    path = f"{cls.replace('.', '/')}.class"
    d = z.read(path)
    cp, _ = parse_cp(d)
    print(f"=== {cls} constant-pool utf8 words (needles: {needles})")
    for i, e in enumerate(cp):
        if e and e[0] == "u":
            w = utf(cp, i)
            if any(n in w for n in needles):
                print(f"  cp#{i}: {w!r}")


z = zipfile.ZipFile(JAR)

print("=== AccessibilityOptionsScreen constant pool (fov/accessibility words)")
class_utf8_words(
    z,
    "net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen",
    ["fov", "FOV", "accessibility", "tooltip", "Narrator", "narrator",
     "subtitle", "darkness", "Darkness", "lineSpacing", "panorama"],
)

print()
print("=== AccessibilityOptionsScreen.addOptions (the row list's own walk)")
dump_method(
    z,
    "net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen",
    "addOptions",
)

print()
print("=== AccessibilityOptionsScreen.removed (the save seat question)")
dump_method(
    z,
    "net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen",
    "removed",
)

print()
print("=== OptionsScreen utf8 words (accessibility button row)")
class_utf8_words(
    z,
    "net.minecraft.client.gui.screens.options.OptionsScreen",
    ["accessibility", "accessibilityLink", "chat", "sound", "screen",
     "TODO", "empty"],
)

print()
print("=== OptionsScreen.init (the accessibility button's own creation)")
dump_method(
    z,
    "net.minecraft.client.gui.screens.options.OptionsScreen",
    "init",
)

print()
print("=== AccessibilityOptionsScreen.options(Options) static (the row array's own order)")
dump_method(
    z,
    "net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen",
    "options([",
)
