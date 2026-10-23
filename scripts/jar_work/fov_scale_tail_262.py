#!/usr/bin/env python3
"""THE FOV EFFECT SCALE'S OWN TAIL PROBE v2 (the autopush-175 round):
the desync the v1 walk wore (the 0x42/0x44/0x46 operand rows the
glass_685 MNEMONIC keeps — the standard JVM the fadd/fmul no-operand
laws speak) — the RAW scan answers: every class the `fovEffectScale`
utf8 the constant pool carries, the reference kinds (field ref on
Options / method refs), and the getFieldOfViewModifier code the
CORRECT opcode-length walk disassembles from the fovEffectScale
instruction's own window.
Usage: python3 scripts/jar_work/fov_scale_tail_262.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"


def parse_cp(d):
    """Returns (cp list, end offset of constant pool)."""
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
            cp[i] = (tag, struct.unpack_from(">q", d, p)[0]); p += 8; i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        else:
            raise Exception(f"tag {tag} @ {i}")
        i += 1
    return cp, p


def resolve_name(cp, idx):
    """Utf8-ish name for a cp entry — NameAndType the descent answers
    (the ref's (12, name_idx, desc_idx) shape the [1] index speaks)."""
    e = cp[idx]
    if e is None:
        return "?"
    if e[0] == "u":
        return e[1]
    if e[0] == 12:
        return cp[e[1]][1] if cp[e[1]] else "?"
    if e[0] in (9, 10, 11, 17, 18):
        return resolve_name(cp, e[2])
    if e[0] == 7:
        return cp[e[1]][1] if cp[e[1]] else "?"
    return f"{e}"


def class_name(d, cp, p):
    # access_flags(p) then this_class(p+2) — the parse_cp end offset speaks
    idx = struct.unpack_from(">H", d, p + 2)[0]
    return resolve_name(cp, idx).replace("/", ".")


def scan_jar(z, needle):
    hits = []
    for name in z.namelist():
        if not name.endswith(".class"):
            continue
        d = z.read(name)
        if needle.encode() not in d:
            continue
        try:
            cp, pend = parse_cp(d)
        except Exception as ex:
            hits.append((name, f"cp-parse-fail {ex}", []))
            continue
        refs = []
        for i in range(1, len(cp)):
            e = cp[i]
            if e and e[0] in (9, 10, 11, 12, 17, 18):
                nm = cp[e[2]][1] if cp[e[2]] else ""
                if nm == needle:
                    kind = {9: "get/putstatic-field", 9: "field", 10: "method", 11: "imethod",
                            12: "name+type", 17: "dyn", 18: "indy"}.get(e[0], str(e[0]))
                    cls = resolve_name(cp, e[1]).replace("/", ".") if e[0] in (9, 10, 11) else ""
                    refs.append((i, kind, cls, nm))
        hits.append((class_name(d, cp, pend), "ok", refs))
    return hits


# ---- the correct JVM opcode-length walk (operands consumed per spec) ----
# op: (mnemonic, operand bytes) — the arithmetic/logic families carry NONE.
OPS = {
    0x00: ("nop", 0), 0x01: ("aconst_null", 0), 0x02: ("iconst_m1", 0),
    0x03: ("iconst_0", 0), 0x04: ("iconst_1", 0), 0x05: ("iconst_2", 0),
    0x06: ("iconst_3", 0), 0x07: ("iconst_4", 0), 0x08: ("iconst_5", 0),
    0x09: ("lconst_0", 0), 0x0a: ("lconst_1", 0), 0x0b: ("fconst_0", 0),
    0x0c: ("fconst_1", 0), 0x0d: ("fconst_2", 0), 0x0e: ("dconst_0", 0),
    0x0f: ("dconst_1", 0),
    0x10: ("bipush", 1), 0x11: ("sipush", 2), 0x12: ("ldc", 1), 0x13: ("ldc_w", 2),
    0x14: ("ldc2_w", 2),
    0x15: ("iload", 1), 0x16: ("lload", 1), 0x17: ("fload", 1), 0x18: ("dload", 1),
    0x19: ("aload", 1),
    0x1a: ("iload_0", 0), 0x1b: ("iload_1", 0), 0x1c: ("iload_2", 0), 0x1d: ("iload_3", 0),
    0x1e: ("lload_0", 0), 0x1f: ("lload_1", 0), 0x20: ("lload_2", 0), 0x21: ("lload_3", 0),
    0x22: ("fload_0", 0), 0x23: ("fload_1", 0), 0x24: ("fload_2", 0), 0x25: ("fload_3", 0),
    0x26: ("dload_0", 0), 0x27: ("dload_1", 0), 0x28: ("dload_2", 0), 0x29: ("dload_3", 0),
    0x2a: ("aload_0", 0), 0x2b: ("aload_1", 0), 0x2c: ("aload_2", 0), 0x2d: ("aload_3", 0),
    0x2e: ("iaload", 0), 0x2f: ("laload", 0), 0x30: ("faload", 0), 0x31: ("daload", 0),
    0x32: ("aaload", 0), 0x33: ("baload", 0), 0x34: ("caload", 0), 0x35: ("saload", 0),
    0x36: ("istore", 1), 0x37: ("lstore", 1), 0x38: ("fstore", 1), 0x39: ("dstore", 1),
    0x3a: ("astore", 1),
    0x3b: ("istore_0", 0), 0x3c: ("istore_1", 0), 0x3d: ("istore_2", 0), 0x3e: ("istore_3", 0),
    0x3f: ("lstore_0", 0), 0x40: ("lstore_1", 0), 0x41: ("lstore_2", 0), 0x42: ("lstore_3", 0),
    0x43: ("fstore_0", 0), 0x44: ("fstore_1", 0), 0x45: ("fstore_2", 0), 0x46: ("fstore_3", 0),
    0x47: ("dstore_0", 0), 0x48: ("dstore_1", 0), 0x49: ("dstore_2", 0), 0x4a: ("dstore_3", 0),
    0x4b: ("astore_0", 0), 0x4c: ("astore_1", 0), 0x4d: ("astore_2", 0), 0x4e: ("astore_3", 0),
    0x4f: ("iastore", 0), 0x50: ("lastore", 0), 0x51: ("fastore", 0), 0x52: ("dastore", 0),
    0x53: ("aastore", 0), 0x54: ("bastore", 0), 0x55: ("castore", 0), 0x56: ("sastore", 0),
    0x57: ("pop", 0), 0x58: ("pop2", 0), 0x59: ("dup", 0), 0x5a: ("dup_x1", 0),
    0x5b: ("dup_x2", 0), 0x5c: ("dup2", 0), 0x5d: ("dup2_x1", 0), 0x5e: ("dup2_x2", 0),
    0x5f: ("swap", 0),
    0x60: ("iadd", 0), 0x61: ("ladd", 0), 0x62: ("fadd", 0), 0x63: ("dadd", 0),
    0x64: ("isub", 0), 0x65: ("lsub", 0), 0x66: ("fsub", 0), 0x67: ("dsub", 0),
    0x68: ("imul", 0), 0x69: ("lmul", 0), 0x6a: ("fmul", 0), 0x6b: ("dmul", 0),
    0x6c: ("idiv", 0), 0x6d: ("ldiv", 0), 0x6e: ("fdiv", 0), 0x6f: ("ddiv", 0),
    0x70: ("irem", 0), 0x71: ("lrem", 0), 0x72: ("frem", 0), 0x73: ("drem", 0),
    0x74: ("ineg", 0), 0x75: ("lneg", 0), 0x76: ("fneg", 0), 0x77: ("dneg", 0),
    0x78: ("ishl", 0), 0x79: ("lshl", 0), 0x7a: ("ishr", 0), 0x7b: ("lshr", 0),
    0x7c: ("iushr", 0), 0x7d: ("lushr", 0), 0x7e: ("iand", 0), 0x7f: ("land", 0),
    0x80: ("ior", 0), 0x81: ("lor", 0), 0x82: ("ixor", 0), 0x83: ("lxor", 0),
    0x84: ("iinc", 2),
    0x85: ("i2l", 0), 0x86: ("i2f", 0), 0x87: ("i2d", 0), 0x88: ("l2i", 0),
    0x89: ("l2f", 0), 0x8a: ("l2d", 0), 0x8b: ("f2i", 0), 0x8c: ("f2l", 0),
    0x8d: ("f2d", 0), 0x8e: ("d2i", 0), 0x8f: ("d2l", 0), 0x90: ("d2f", 0),
    0x91: ("i2b", 0), 0x92: ("i2c", 0), 0x93: ("i2s", 0),
    0x94: ("lcmp", 0), 0x95: ("fcmpl", 0), 0x96: ("fcmpg", 0), 0x97: ("dcmpl", 0),
    0x98: ("dcmpg", 0),
    0x99: ("ifeq", 2), 0x9a: ("ifne", 2), 0x9b: ("iflt", 2), 0x9c: ("ifge", 2),
    0x9d: ("ifgt", 2), 0x9e: ("ifle", 2),
    0x9f: ("if_icmpeq", 2), 0xa0: ("if_icmpne", 2), 0xa1: ("if_icmplt", 2),
    0xa2: ("if_icmpge", 2), 0xa3: ("if_icmpgt", 2), 0xa4: ("if_icmple", 2),
    0xa5: ("if_acmpeq", 2), 0xa6: ("if_acmpne", 2),
    0xa7: ("goto", 2), 0xa8: ("jsr", 2), 0xa9: ("ret", 1),
    0xaa: ("tableswitch", -1), 0xab: ("lookupswitch", -1),
    0xac: ("ireturn", 0), 0xad: ("lreturn", 0), 0xae: ("freturn", 0),
    0xaf: ("dreturn", 0), 0xb0: ("areturn", 0), 0xb1: ("return", 0),
    0xb2: ("getstatic", 2), 0xb3: ("putstatic", 2), 0xb4: ("getfield", 2),
    0xb5: ("putfield", 2), 0xb6: ("invokevirtual", 2), 0xb7: ("invokespecial", 2),
    0xb8: ("invokestatic", 2), 0xb9: ("invokeinterface", 4), 0xba: ("invokedynamic", 4),
    0xbb: ("new", 2), 0xbc: ("newarray", 1), 0xbd: ("anewarray", 2),
    0xbe: ("arraylength", 0), 0xbf: ("athrow", 0),
    0xc0: ("checkcast", 2), 0xc1: ("instanceof", 2),
    0xc2: ("monitorenter", 0), 0xc3: ("monitorexit", 0),
    0xc4: ("wide", -2), 0xc5: ("multianewarray", 3),
    0xc6: ("ifnull", 2), 0xc7: ("ifnonnull", 2),
    0xc8: ("goto_w", 4), 0xc9: ("jsr_w", 4),
}


def disasm(cp, code, cp_extra=None):
    out = []
    j = 0
    while j < len(code):
        pc = j
        op = code[j]; j += 1
        mn, nb = OPS.get(op, (f"0x{op:02x}", 0))
        detail = ""
        if nb > 0:
            raw = code[j:j + nb]; j += nb
            if mn == "ldc":
                e = cp[raw[0]]
                detail = str(cp_get(cp, e))
            elif mn in ("ldc_w", "ldc2_w", "getstatic", "putstatic", "getfield",
                        "putfield", "invokevirtual", "invokespecial", "invokestatic",
                        "new", "anewarray", "checkcast", "instanceof"):
                idx = struct.unpack(">H", raw)[0]
                detail = cp_ref(cp, idx)
            elif mn == "invokeinterface":
                idx = struct.unpack(">H", raw[:2])[0]
                detail = cp_ref(cp, idx)
            elif mn == "invokedynamic":
                idx = struct.unpack(">H", raw[:2])[0]
                detail = f"indy@{idx}"
            elif mn.endswith("switch"):
                detail = f"(switch@{pc})"
                if mn == "tableswitch":
                    pad = (4 - (j % 4)) % 4
                    j += pad
                    default, lo, hi = struct.unpack_from(">iii", code, j)
                    j += 12 + 4 * (hi - lo + 1)
                    detail = f"tableswitch lo={lo} hi={hi}"
                else:
                    pad = (4 - (j % 4)) % 4
                    j += pad
                    default, npairs = struct.unpack_from(">ii", code, j)
                    j += 8 + 8 * npairs
                    detail = f"lookupswitch npairs={npairs}"
            elif mn == "wide":
                wop = code[j]
                wmn, wnb = OPS.get(wop, (f"0x{wop:02x}", 0))
                j += 2
                if wop == 0x84:
                    j += 2
                detail = f"wide {wmn}"
            elif mn in ("ifeq", "ifne", "iflt", "ifge", "ifgt", "ifle",
                        "if_icmpeq", "if_icmpne", "if_icmplt", "if_icmpge",
                        "if_icmpgt", "if_icmple", "if_acmpeq", "if_acmpne",
                        "goto", "jsr", "ifnull", "ifnonnull"):
                off = struct.unpack(">h", raw)[0]
                detail = f"-> {pc + off}"
            elif nb == 1:
                detail = str(raw[0])
            else:
                detail = " ".join(str(b) for b in raw)
        out.append((pc, f"{pc}: {mn} {detail}".rstrip()))
    return out


def cp_get(cp, e):
    if e is None:
        return "?"
    if e[0] == "u":
        return e[1]
    if e[0] == 3:
        return f"int {struct.unpack('>i', struct.pack('>I', e[1]))[0]}"
    if e[0] == 4:
        return f"float {struct.unpack('>f', struct.pack('>I', e[1]))[0]}"
    if e[0] == 5:
        return f"long {e[1]}"
    if e[0] == 6:
        return f"double {struct.unpack('>d', struct.pack('>q', e[1]))[0]}"
    return f"cp{e}"


def cp_ref(cp, idx):
    e = cp[idx]
    if e is None:
        return f"?{idx}"
    if e[0] in (9, 10, 11):
        cls = resolve_name(cp, e[1]).replace("/", ".")
        nm = resolve_name(cp, e[2])
        return f"{cls}.{nm}"
    if e[0] == 12:
        return f"{resolve_name(cp, e[1])}.{resolve_name(cp, e[2])}"
    if e[0] == 7:
        return resolve_name(cp, e[1]).replace("/", ".")
    return cp_get(cp, e)


def method_bodies(z, cls):
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
            data = d[p + 6:p + 6 + ln]
            if cp[an_i][1] == "Code":
                body = data
            p += 6 + ln
        out[(cp[name_i][1], cp[desc_i][1])] = body
    return cp, out


def main():
    z = zipfile.ZipFile(JAR)
    print("==== 1) classes the fovEffectScale utf8 carries ====")
    for cls, status, refs in scan_jar(z, "fovEffectScale"):
        print(f"-- {cls} [{status}]")
        for i, kind, owner, nm in refs:
            print(f"   cp#{i} {kind} owner={owner} name={nm}")
    print("==== 2) AbstractClientPlayer.getFieldOfViewModifier correct disasm ====")
    rb = method_bodies(z, "net.minecraft.client.player.AbstractClientPlayer")
    cp, bodies = rb
    for (nm, desc), body in bodies.items():
        if "ieldOfView" in nm or "fov" in nm.lower():
            print(f"== {nm}{desc}")
            if body is None:
                continue
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            for pc, line in disasm(cp, code):
                print("   ", line)
    print("==== 3) the fovEffectScale consumer's own methods (the scan's classes) ====")
    for cls, status, refs in scan_jar(z, "fovEffectScale"):
        if cls in ("net.minecraft.client.Options",):
            continue  # the registration the options_init bank already keeps
        print(f"-- consumer: {cls}")
        rb = method_bodies(z, cls)
        if rb is None:
            continue
        cp, bodies = rb
        for (nm, desc), body in bodies.items():
            if body is None:
                continue
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            text = "\n".join(l for _, l in disasm(cp, code))
            if "fovEffectScale" in text:
                print(f"== {cls} :: {nm}{desc}")
                for _, l in disasm(cp, code):
                    print("   ", l)


if __name__ == "__main__":
    main()
