#!/usr/bin/env python3
"""enchant_quickmove_404 — the EnchantmentMenu.quickMoveStack the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
class file the constant pool + the code attribute the parser reads,
the moveItemStackTo call sites the args the print keeps. Usage:
python3 scripts/jar_work_parse_enchant_quickmove_404.py"""
import struct
import sys
import zipfile

JAR = ".mc2rs/classpath/client.jar"
CLASS = "net/minecraft/world/inventory/EnchantmentMenu.class"

# opcode name table (the subset the walk needs)
NAMES = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x09: "lconst_0", 0x0A: "lconst_1", 0x10: "bipush",
    0x11: "sipush", 0x12: "ldc", 0x13: "ldc_w", 0x15: "iload", 0x19: "aload",
    0x1A: "iload_0",
    0x1B: "iload_1", 0x1C: "iload_2", 0x1D: "iload_3", 0x1E: "lload_0",
    0x2A: "aload_0", 0x2B: "aload_1", 0x2C: "aload_2", 0x2D: "aload_3",
    0x36: "istore", 0x3A: "astore", 0x3C: "istore_0", 0x3D: "istore_1",
    0x3E: "istore_2",
    0x3F: "istore_3", 0x4B: "astore_0", 0x4C: "astore_1", 0x4D: "astore_2",
    0x4E: "astore_3", 0x57: "pop", 0x59: "dup", 0x60: "iadd", 0x64: "isub",
    0x68: "imul", 0x70: "irem", 0x74: "ineg", 0x78: "ishl", 0x7A: "ishr",
    0x84: "iinc", 0x85: "i2l", 0x88: "i2s", 0x99: "ifeq", 0x9A: "ifne",
    0x9B: "iflt", 0x9C: "ifge", 0x9D: "ifgt", 0x9E: "ifle",
    0x9F: "if_icmpeq", 0xA0: "if_icmpne", 0xA1: "if_icmplt", 0xA2: "if_icmpge",
    0xA3: "if_icmpgt", 0xA4: "if_icmple", 0xA5: "if_acmpeq", 0xA6: "if_acmpne",
    0xA7: "goto", 0xAC: "ireturn", 0xB0: "areturn", 0xB1: "return",
    0xB2: "getstatic", 0xB4: "getfield", 0xB5: "putfield",
    0xB6: "invokevirtual", 0xB7: "invokespecial", 0xB8: "invokestatic",
    0xB9: "invokeinterface", 0xBB: "new", 0xBD: "anewarray", 0xBF: "athrow",
    0xC0: "checkcast", 0xC1: "instanceof", 0xC6: "ifnull", 0xC7: "ifnonnull",
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
            pool[i] = (tag, struct.unpack_from(">HH", data, off + 1))
            off += 5
        elif tag == CONSTANT_MethodHandle:
            pool[i] = (tag, struct.unpack_from(">BH", data, off + 1))
            off += 4
        elif tag in (CONSTANT_Integer, CONSTANT_Float):
            pool[i] = (tag, struct.unpack_from(">i", data, off + 1)[0])
            off += 5
        elif tag in (CONSTANT_Long, CONSTANT_Double):
            pool[i] = (tag, struct.unpack_from(">q", data, off + 1)[0])
            off += 9
            i += 1  # the two-slot entries the count keeps
        else:
            raise SystemExit(f"unknown constant tag {tag} at {i}")
        i += 1
    return pool, off


def utf8(pool, idx):
    return pool[idx][1]


def method_name(pool, idx):
    tag, (cls_idx, nat_idx) = pool[idx]
    _, (name_idx, _) = pool[nat_idx]
    return f"{utf8(pool, pool[cls_idx][1])}.{utf8(pool, name_idx)}"


def main():
    with zipfile.ZipFile(JAR) as zf:
        data = zf.read(CLASS)
    pool, off = read_const_pool(data)
    acc_flags, this_idx, _ = struct.unpack_from(">HHH", data, off)
    off += 6
    ifc_count = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc_count
    fcount = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(fcount):
        off += 6
        acount = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(acount):
            off += 6
            alen = struct.unpack_from(">I", data, off)[0]
            off += 4 + alen
    mcount = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(mcount):
        macc, name_idx, desc_idx = struct.unpack_from(">HHH", data, off)
        off += 6
        name = utf8(pool, name_idx)
        acount = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(acount):
            aname_idx = struct.unpack_from(">H", data, off)[0]
            off += 2
            alen = struct.unpack_from(">I", data, off)[0]
            off += 4
            if utf8(pool, aname_idx) == "Code" and name == "quickMoveStack":
                code = parse_code(data, off)
                disasm(pool, name, utf8(pool, desc_idx), code)
            off += alen
    return 0


def parse_code(data, attr_off):
    # Code attribute: max_stack u2, max_locals u2, code_len u4, code bytes
    clen = struct.unpack_from(">I", data, attr_off + 4)[0]
    return data[attr_off + 8: attr_off + 8 + clen]


def disasm(pool, mname, mdesc, code):
    print(f"=== {mname}{mdesc} ({len(code)} code bytes) ===")
    i = 0
    while i < len(code):
        pc = i
        op = code[i]
        name = NAMES.get(op, f"0x{op:02X}")
        arg = ""
        if op in (0x10,):  # bipush
            arg = str(struct.unpack_from(">b", code, i + 1)[0]); i += 2
        elif op in (0x11,):  # sipush
            arg = str(struct.unpack_from(">h", code, i + 1)[0]); i += 3
        elif op in (0x12, 0x13):
            w = 3 if op == 0x13 else 2
            arg = str(struct.unpack_from(">H", code, i + 1)[0]); i += w
        elif op in (0x15, 0x19, 0x36, 0x3A, 0xBC):
            arg = str(code[i + 1]); i += 2
        elif op == 0x84:  # iinc
            arg = f"{code[i+1]} {struct.unpack_from('>b', code, i+2)[0]}"; i += 3
        elif op in (0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1,
                    0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xC6, 0xC7):
            arg = f"-> {pc + struct.unpack_from('>h', code, i+1)[0]}"; i += 3
        elif op in (0xB2, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xBB, 0xBD,
                    0xC0, 0xC1):
            idx = struct.unpack_from(">H", code, i + 1)[0]
            if op in (0xB6, 0xB7, 0xB8):
                arg = method_name(pool, idx)
            elif op in (0xBB, 0xBD, 0xC0, 0xC1):
                arg = utf8(pool, pool[idx][1])
            else:
                arg = f"#{idx}"
            i += 3
        elif op == 0xB9:
            idx = struct.unpack_from(">H", code, i + 1)[0]
            arg = method_name(pool, idx)
            i += 5
        else:
            i += 1
        print(f"  {pc:4d}: {name} {arg}".rstrip())


if __name__ == "__main__":
    sys.exit(main())
