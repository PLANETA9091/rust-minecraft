#!/usr/bin/env python3
"""sprites_461 — the TradeOfferButton's own widget states the jar's own
bytes speak (the javap-less walk the JRE-only sandbox wears): the class
file the constant pool + the code attribute the parser reads, the
sprite walk the 461 round banks (the renderWidget's own blit lane, the
extract* static helpers the background/arrow/scroller seats speak).
Usage: python3 scripts/jar_work_parse_sprites_461.py > scripts/jar_work/sprites_461.txt"""
import struct
import sys
import zipfile

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$TradeOfferButton.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$1.class",
]

NAMES = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x09: "lconst_0", 0x0A: "lconst_1", 0x10: "bipush",
    0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x14: "ldc2_w",
    0x15: "iload", 0x16: "lload", 0x19: "aload",
    0x1A: "iload_0", 0x1B: "iload_1", 0x1C: "iload_2", 0x1D: "iload_3",
    0x1E: "lload_0", 0x1F: "lload_1", 0x2A: "aload_0", 0x2B: "aload_1",
    0x2C: "aload_2", 0x2D: "aload_3", 0x2E: "iaload", 0x32: "aaload",
    0x33: "laload", 0x34: "faload", 0x35: "daload", 0x36: "istore",
    0x37: "lstore", 0x38: "fstore", 0x39: "dstore", 0x3A: "astore",
    0x3B: "istore_0", 0x3C: "istore_1", 0x3D: "istore_2", 0x3E: "istore_3",
    0x3F: "lstore_0", 0x40: "lstore_1", 0x41: "lstore_2", 0x42: "lstore_3",
    0x43: "fstore_0", 0x44: "fstore_1", 0x45: "fstore_2", 0x46: "fstore_3",
    0x47: "dstore_0", 0x48: "dstore_1", 0x49: "dstore_2", 0x4A: "dstore_3",
    0x4B: "astore_0", 0x4C: "astore_1", 0x4D: "astore_2", 0x4E: "astore_3",
    0x4F: "iastore", 0x53: "aastore", 0x54: "bastore",
    0x57: "pop", 0x58: "pop2",
    0x59: "dup", 0x5A: "dup_x1", 0x5B: "dup_x2", 0x5C: "dup2",
    0x5D: "dup2_x1", 0x5E: "dup2_x2", 0x5F: "swap",
    0x60: "iadd", 0x61: "ladd", 0x62: "fadd", 0x63: "dadd",
    0x64: "isub", 0x66: "fsub", 0x67: "dsub",
    0x68: "imul", 0x6A: "fmul", 0x6B: "dmul",
    0x6C: "idiv", 0x6D: "ldiv", 0x6E: "fdiv", 0x70: "irem",
    0x74: "ineg", 0x76: "fneg",
    0x78: "ishl", 0x7A: "ishr", 0x7C: "iushr", 0x7E: "iand", 0x80: "ior",
    0x84: "iinc", 0x85: "i2l", 0x86: "i2f", 0x87: "i2d", 0x88: "i2s",
    0x8B: "f2i", 0x8D: "f2d", 0x8E: "d2i", 0x91: "d2f",
    0x94: "lcmp", 0x95: "fcmpl", 0x96: "fcmpg", 0x97: "dcmpl", 0x98: "dcmpg",
    0x99: "ifeq", 0x9A: "ifne", 0x9B: "iflt", 0x9C: "ifge", 0x9D: "ifgt",
    0x9E: "ifle",
    0x9F: "if_icmpeq", 0xA0: "if_icmpne", 0xA1: "if_icmplt", 0xA2: "if_icmpge",
    0xA3: "if_icmpgt", 0xA4: "if_icmple", 0xA5: "if_acmpeq", 0xA6: "if_acmpne",
    0xA7: "goto", 0xA8: "jsr", 0xAC: "ireturn", 0xAD: "lreturn",
    0xAE: "freturn", 0xB0: "areturn", 0xB1: "return",
    0xB2: "getstatic", 0xB3: "putstatic", 0xB4: "getfield", 0xB5: "putfield",
    0xB6: "invokevirtual", 0xB7: "invokespecial", 0xB8: "invokestatic",
    0xB9: "invokeinterface", 0xBA: "invokedynamic",
    0xBB: "new", 0xBC: "newarray",
    0xBD: "anewarray", 0xBE: "arraylength", 0xBF: "athrow",
    0xC0: "checkcast", 0xC1: "instanceof", 0xC2: "monitorenter",
    0xC3: "monitorexit", 0xC6: "ifnull", 0xC7: "ifnonnull",
}

CONSTANT_Utf8, CONSTANT_Class, CONSTANT_String = 1, 7, 8
CONSTANT_Field, CONSTANT_Method, CONSTANT_Iface = 9, 10, 11
CONSTANT_NameAndType, CONSTANT_MethodType = 12, 16
CONSTANT_Dynamic, CONSTANT_InvokeDynamic = 17, 18
CONSTANT_MethodHandle, CONSTANT_Integer, CONSTANT_Long, CONSTANT_Float = 15, 3, 5, 4
CONSTANT_Double = 6


def read_const_pool(data):
    count = struct.unpack_from(">H", data, 8)[0]
    pool = [None] * count
    off = 10
    i = 1
    while i < count:
        tag = data[off]
        if tag == CONSTANT_Utf8:
            ln = struct.unpack_from(">H", data, off + 1)[0]
            pool[i] = ("Utf8", data[off + 3: off + 3 + ln].decode("utf-8", "replace"))
            off += 3 + ln
        elif tag in (CONSTANT_Class, CONSTANT_String, CONSTANT_MethodType):
            pool[i] = (tag, struct.unpack_from(">H", data, off + 1)[0])
            off += 3
        elif tag in (CONSTANT_Field, CONSTANT_Method, CONSTANT_Iface,
                     CONSTANT_NameAndType, CONSTANT_Dynamic, CONSTANT_InvokeDynamic):
            pool[i] = (tag,) + struct.unpack_from(">HH", data, off + 1)
            off += 5
        elif tag in (CONSTANT_Integer, CONSTANT_Float):
            # THE HALF-WORD READ THE 480 ROUND FIXES (the 4-byte payload
            # the >HH pair never wore — the high half the resolve read,
            # the low half the word loses): the full u32 the >I unpack
            # banks, the resolve the same single-word law reads.
            pool[i] = (tag, struct.unpack_from(">I", data, off + 1)[0])
            off += 5
        elif tag in (CONSTANT_Long, CONSTANT_Double):
            pool[i] = (tag, struct.unpack_from(">Q", data, off + 1)[0])
            off += 9
            i += 1  # wide entries take two pool slots
        elif tag in (CONSTANT_Dynamic, CONSTANT_InvokeDynamic):
            pool[i] = (tag,) + struct.unpack_from(">HH", data, off + 1)
            off += 5
        elif tag == CONSTANT_MethodHandle:
            pool[i] = (tag, struct.unpack_from(">BH", data, off + 1))
            off += 4
        else:
            raise ValueError(f"unknown cp tag {tag} at {off}")
        i += 1
    return pool


def utf8(pool, i):
    return pool[i][1]


def resolve(pool, i):
    """cp index -> readable string."""
    ent = pool[i]
    tag = ent[0]
    if tag == "Utf8":
        return ent[1]
    if tag == CONSTANT_String:
        return f'"{utf8(pool, ent[1])}"'
    if tag in (CONSTANT_Field, CONSTANT_Method, CONSTANT_Iface):
        cls = utf8(pool, pool[ent[1]][1])
        nat = pool[ent[2]]
        return f"{cls}.{utf8(pool, nat[1])}:{utf8(pool, nat[2])}"
    if tag == CONSTANT_NameAndType:
        return f"{utf8(pool, ent[1])}:{utf8(pool, ent[2])}"
    if tag == CONSTANT_Class:
        return utf8(pool, ent[1])
    if tag == CONSTANT_Integer:
        return str(ent[1])
    if tag == CONSTANT_Float:
        return f"{struct.unpack('>f', struct.pack('>I', ent[1]))[0]}f"
    if tag == CONSTANT_Long:
        return str(ent[1])
    if tag == CONSTANT_Double:
        return f"{struct.unpack('>d', struct.pack('>Q', ent[1]))[0]}"
    if tag in (CONSTANT_Dynamic, CONSTANT_InvokeDynamic):
        nat = pool[ent[2]]
        return f"dynamic#{ent[1]}:{utf8(pool, nat[1])}"
    if tag == CONSTANT_MethodHandle:
        return f"mh{ent[1]}"
    return f"cp#{i}"


def disasm(code, pool, indent="    "):
    out = []
    off = 0
    while off < len(code):
        pc = off
        op = code[off]
        name = NAMES.get(op, f"op_{op:02x}")
        off += 1
        if op == 0x10:  # bipush
            v = struct.unpack_from(">b", code, off)[0]
            off += 1
            out.append(f"{indent}{pc}: {name} {v}")
        elif op == 0x11:  # sipush
            v = struct.unpack_from(">h", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {v}")
        elif op == 0x12:  # ldc
            i = code[off]
            off += 1
            out.append(f"{indent}{pc}: {name} {resolve(pool, i)}")
        elif op in (0x13, 0x14):  # ldc_w, ldc2_w
            i = struct.unpack_from(">H", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {resolve(pool, i)}")
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39,
                    0x3A):
            idx = code[off]
            off += 1
            out.append(f"{indent}{pc}: {name} {idx}")
        elif op == 0x84:  # iinc
            idx, delta = struct.unpack_from(">Bb", code, off)
            off += 2
            out.append(f"{indent}{pc}: {name} {idx} {delta}")
        elif op in (0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1,
                    0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xC6, 0xC7):
            br = struct.unpack_from(">h", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {pc + br}")
        elif op == 0xBA:  # invokedynamic
            i = struct.unpack_from(">H", code, off)[0]
            off += 4
            out.append(f"{indent}{pc}: invokedynamic {resolve(pool, i)}")
        elif op in (0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xBB, 0xBD,
                    0xC0, 0xC1):
            i = struct.unpack_from(">H", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {resolve(pool, i)}")
        elif op == 0xB9:  # invokeinterface
            i = struct.unpack_from(">H", code, off)[0]
            off += 4
            out.append(f"{indent}{pc}: {name} {resolve(pool, i)}")
        elif op == 0xBC:  # newarray
            off += 1
            out.append(f"{indent}{pc}: {name}")
        else:
            out.append(f"{indent}{pc}: {name}")
    return out


def dump_class(data, wanted):
    pool = read_const_pool(data)
    cp_count = struct.unpack_from(">H", data, 8)[0]
    off = 10
    i = 1
    while i < cp_count:
        tag = data[off]
        if tag == CONSTANT_Utf8:
            ln = struct.unpack_from(">H", data, off + 1)[0]
            off += 3 + ln
        elif tag in (CONSTANT_Long, CONSTANT_Double):
            off += 9
            i += 1
        elif tag in (CONSTANT_Field, CONSTANT_Method, CONSTANT_Iface,
                     CONSTANT_NameAndType, CONSTANT_Integer, CONSTANT_Float,
                     CONSTANT_Dynamic, CONSTANT_InvokeDynamic):
            off += 5
        elif tag == CONSTANT_MethodHandle:
            off += 4
        else:
            off += 3
        i += 1
    off += 6  # access_flags + this_class + super_class
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc
    fs = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(fs):
        fflags, fname, fdesc = struct.unpack_from(">HHH", data, off)
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        if wanted(f"FIELD {utf8(pool, fname)}:{utf8(pool, fdesc)}", ""):
            print(f"  FIELD {utf8(pool, fname)}:{utf8(pool, fdesc)} flags=0x{fflags:x}")
        for _ in range(ac):
            aname_i = struct.unpack_from(">H", data, off)[0]
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6
            off += alen
    ms = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(ms):
        mflags, mname, mdesc = struct.unpack_from(">HHH", data, off)
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        mname_s = utf8(pool, mname)
        mdesc_s = utf8(pool, mdesc)
        for _ in range(ac):
            aname_i = struct.unpack_from(">H", data, off)[0]
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6
            aname = utf8(pool, aname_i)
            if aname == "Code":
                clen2 = struct.unpack_from(">I", data, off + 4)[0]
                body = data[off + 8: off + 8 + clen2]
                if wanted(mname_s, mdesc_s):
                    print(f"  METHOD {mname_s} {mdesc_s}  (code={clen2})")
                    for line in disasm(body, pool):
                        print(line)
            off += alen
    print()


def main():
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                continue
            print(f"== {cls}")
            dump_class(data, lambda n, d: True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
