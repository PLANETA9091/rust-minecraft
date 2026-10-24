#!/usr/bin/env python3
"""THE BG-OPACITY LANES' OWN PROBE (autopush-180): the Options
background twins the full descriptor bank keeps — getBackgroundColor(I)I,
getBackgroundColor(F)I (the float overload the Hud sidebar's 0.3/0.4 and
the SubtitleOverlay's 0.8 walk wear), getBackgroundOpacity(F)F (the
nametag/TextDisplay lane) — the exact bytecode the Everywhere/Chat gate
the implementation speaks. Plus the OptionsRenderState mirror methods
(the render-state copy the submitNameTag walk reads).

Usage: python3 scripts/jar_work/bgopacity_lanes_180.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"


def utf(cp, i):
    e = cp[i]
    return e[1] if e and e[0] == "u" else f"?{i}"


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


def ref_name(cp, i):
    e = cp[i]
    cls = utf(cp, cp[e[1]][1])
    nt = cp[e[2]]
    return f"{cls}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"


MNEMONIC = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x09: "lconst_0", 0x0a: "lconst_1", 0x0b: "fconst_0",
    0x0c: "fconst_1", 0x0d: "fconst_2", 0x0e: "dconst_0", 0x0f: "dconst_1",
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x14: "ldc2_w",
    0x15: "iload", 0x17: "fload", 0x19: "aload", 0x1a: "iload_0",
    0x1b: "iload_1", 0x1c: "iload_2", 0x1d: "iload_3", 0x2a: "fload_0",
    0x2b: "fload_1", 0x2c: "fload_2", 0x2d: "fload_3", 0x2e: "iaload",
    0x32: "aaload", 0x35: "faload", 0x36: "istore", 0x38: "fstore",
    0x3a: "astore", 0x3b: "istore_0", 0x3c: "istore_1", 0x3d: "istore_2",
    0x3e: "istore_3", 0x45: "fstore_0", 0x46: "fstore_1", 0x47: "fstore_2",
    0x48: "fstore_3", 0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2",
    0x4e: "astore_3", 0x57: "pop", 0x59: "dup", 0x5f: "swap", 0x60: "iadd",
    0x62: "fadd", 0x64: "isub", 0x66: "fsub", 0x68: "imul", 0x6a: "fmul",
    0x6c: "idiv", 0x6e: "fdiv", 0x70: "irem", 0x72: "frem", 0x74: "ineg",
    0x76: "fneg", 0x78: "ishl", 0x7a: "ishr", 0x7c: "iushr", 0x7e: "iand",
    0x7f: "ior", 0x81: "ixor", 0x84: "iinc", 0x85: "i2l", 0x86: "i2f",
    0x87: "i2d", 0x88: "l2i", 0x8b: "f2i", 0x8d: "f2d", 0x8e: "d2f",
    0x91: "i2b", 0x92: "i2c", 0x94: "lcmp", 0x95: "fcmpl", 0x96: "fcmpg",
    0x97: "dcmpl", 0x98: "dcmpg", 0x99: "ifeq", 0x9a: "ifne", 0x9b: "iflt",
    0x9c: "ifge", 0x9d: "ifgt", 0x9e: "ifle", 0x9f: "if_icmpeq",
    0xa0: "if_icmpne", 0xa1: "if_icmplt", 0xa2: "if_icmpge",
    0xa3: "if_icmpgt", 0xa4: "if_icmple", 0xa5: "if_acmpeq",
    0xa6: "if_acmpne", 0xa7: "goto", 0xa9: "ret", 0xaa: "tableswitch",
    0xab: "lookupswitch", 0xac: "ireturn", 0xad: "lreturn",
    0xae: "freturn", 0xb0: "areturn", 0xb1: "return", 0xb2: "getstatic",
    0xb4: "getfield", 0xb5: "putfield", 0xb6: "invokevirtual",
    0xb7: "invokespecial", 0xb8: "invokestatic", 0xb9: "invokeinterface",
    0xba: "invokedynamic", 0xbb: "new", 0xbc: "newarray",
    0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc2: "monitorenter",
    0xc3: "monitorexit", 0xc5: "multianewarray", 0xc6: "ifnull",
    0xc7: "ifnonnull", 0xc8: "goto_w", 0xc9: "jsr_w",
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
            if e[0] == 1:
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
            elif e[0] == 1:
                detail = utf(cp, e[1])
            else:
                detail = f"ldcw@{idx}"
        elif op in (0x15, 0x17, 0x19, 0x21, 0x23, 0x25, 0x36, 0x38, 0x3a,
                    0x42, 0x44, 0x46):
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


def main():
    z = zipfile.ZipFile(JAR)
    for cls, needle in [
        ("net.minecraft.client.Options", ("getBackgroundColor", "getBackgroundOpacity")),
        ("net.minecraft.client.renderer.state.OptionsRenderState",
         ("getBackgroundColor", "getBackgroundOpacity")),
        ("net.minecraft.util.ARGB", ("as8BitChannel", "colorFromFloat", "color(")),
    ]:
        print(f"##### {cls}")
        ms = methods_of(z, cls)
        for sig in sorted(ms):
            if any(sig.startswith(n) for n in needle):
                cp, body = ms[sig]
                print(f"== {cls} :: {sig}")
                if body is None:
                    print("   (no Code)")
                    continue
                clen = struct.unpack_from(">I", body, 4)[0]
                code = body[8:8 + clen]
                for line in walk_code(cp, code):
                    print(f"   {line}")
                print()


if __name__ == "__main__":
    main()
