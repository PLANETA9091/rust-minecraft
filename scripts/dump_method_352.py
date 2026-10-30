#!/usr/bin/env python3
"""Dump a .class method's code as a readable opcode + cp-ref stream.

Usage: dump_method_350.py <class-file> [method-name ...]
Without method names: lists all methods. With names: disassembles each
matched method body at mnemonic level (opcodes of interest resolved
against the constant pool).
"""
import struct
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from cpool_350 import parse  # noqa: E402

# opcode -> (mnemonic, operand bytes, kind)  kind: none/branch/cp/local/iinc/lookup/table
OPS = {
    0x00: ("nop", 0, "none"), 0x01: ("aconst_null", 0, "none"), 0x02: ("iconst_m1", 0, "none"),
    0x03: ("iconst_0", 0, "none"), 0x04: ("iconst_1", 0, "none"), 0x05: ("iconst_2", 0, "none"),
    0x06: ("iconst_3", 0, "none"), 0x07: ("iconst_4", 0, "none"), 0x08: ("iconst_5", 0, "none"),
    0x09: ("lconst_0", 0, "none"), 0x0A: ("lconst_1", 0, "none"), 0x0B: ("fconst_0", 0, "none"),
    0x0C: ("fconst_1", 0, "none"), 0x0D: ("fconst_2", 0, "none"), 0x0E: ("dconst_0", 0, "none"),
    0x0F: ("dconst_1", 0, "none"),
    0x10: ("bipush", 1, "none"), 0x11: ("sipush", 2, "none"),
    0x12: ("ldc", 1, "cp"), 0x13: ("ldc_w", 2, "cp"), 0x14: ("ldc2_w", 2, "cp"),
    0x15: ("iload", 1, "local"), 0x17: ("lload", 1, "local"), 0x19: ("aload", 1, "local"),
    0x1A: ("iload_0", 0, "none"), 0x1B: ("iload_1", 0, "none"), 0x1C: ("iload_2", 0, "none"), 0x1D: ("iload_3", 0, "none"),
    0x1E: ("lload_0", 0, "none"), 0x1F: ("lload_1", 0, "none"), 0x20: ("lload_2", 0, "none"), 0x21: ("lload_3", 0, "none"),
    0x2A: ("aload_0", 0, "none"), 0x2B: ("aload_1", 0, "none"), 0x2C: ("aload_2", 0, "none"), 0x2D: ("aload_3", 0, "none"),
    0x22: ("fload_0", 0, "none"), 0x23: ("fload_1", 0, "none"), 0x24: ("fload_2", 0, "none"), 0x25: ("fload_3", 0, "none"),
    0x26: ("dload_0", 0, "none"), 0x27: ("dload_1", 0, "none"), 0x28: ("dload_2", 0, "none"), 0x29: ("dload_3", 0, "none"),
    0x18: ("dload", 1, "local"),
    0x43: ("fstore_0", 0, "none"), 0x44: ("fstore_1", 0, "none"), 0x45: ("fstore_2", 0, "none"), 0x46: ("fstore_3", 0, "none"),
    0x47: ("dstore_0", 0, "none"), 0x48: ("dstore_1", 0, "none"), 0x49: ("dstore_2", 0, "none"), 0x4A: ("dstore_3", 0, "none"),
    0x2E: ("iaload2", 0, "none"), 0x2F: ("laload", 0, "none"), 0x30: ("faload", 0, "none"), 0x31: ("daload", 0, "none"),
    0x33: ("baload", 0, "none"), 0x34: ("caload", 0, "none"),
    0x38: ("fstore", 1, "local"), 0x39: ("dstore", 1, "local"),
    0x3A: ("astore", 1, "local"),
    0x4F: ("iastore", 0, "none"), 0x50: ("lastore", 0, "none"), 0x51: ("fastore", 0, "none"),
    0x52: ("dastore", 0, "none"), 0x53: ("aastore", 0, "none"), 0x54: ("bastore", 0, "none"),
    0x55: ("castore", 0, "none"), 0x56: ("sastore", 0, "none"),
    0x58: ("pop2", 0, "none"), 0x5B: ("dup_x2", 0, "none"), 0x5C: ("dup2", 0, "none"),
    0x5D: ("dup2_x1", 0, "none"), 0x5E: ("dup2_x2", 0, "none"),
    0x61: ("ladd", 0, "none"), 0x62: ("fadd", 0, "none"), 0x63: ("dadd", 0, "none"),
    0x65: ("lsub", 0, "none"), 0x66: ("fsub", 0, "none"), 0x67: ("dsub", 0, "none"),
    0x69: ("lmul", 0, "none"), 0x6A: ("fmul", 0, "none"), 0x6B: ("dmul", 0, "none"),
    0x6C: ("idiv", 0, "none"), 0x6D: ("ldiv", 0, "none"), 0x6E: ("fdiv", 0, "none"), 0x6F: ("ddiv", 0, "none"),
    0x70: ("irem", 0, "none"), 0x71: ("lrem", 0, "none"), 0x72: ("frem", 0, "none"), 0x73: ("drem", 0, "none"),
    0x75: ("lneg", 0, "none"), 0x76: ("fneg", 0, "none"), 0x77: ("dneg", 0, "none"),
    0x78: ("ishl", 0, "none"), 0x79: ("ishr", 0, "none"), 0x7A: ("iushr", 0, "none"),
    0x7C: ("lshr", 0, "none"), 0x7D: ("lushr", 0, "none"), 0x7E: ("iand", 0, "none"),
    0x7F: ("land", 0, "none"), 0x81: ("lor", 0, "none"), 0x83: ("lxor2", 0, "none"),
    0x86: ("l2i", 0, "none"), 0x87: ("l2f", 0, "none"), 0x88: ("l2d", 0, "none"),
    0x8A: ("f2l", 0, "none"), 0x8C: ("d2i", 0, "none"), 0x8D: ("d2l", 0, "none"), 0x8E: ("d2f", 0, "none"),
    0x8F: ("i2b", 0, "none"), 0x90: ("i2c", 0, "none"), 0x91: ("i2s", 0, "none"),
    0xA9: ("ret", 1, "local"),
    0x32: ("aaload", 0, "none"), 0x35: ("iaload", 0, "none"),
    0x36: ("istore", 1, "local"), 0x37: ("lstore", 1, "local"), 0x3B: ("istore_0", 0, "none"),
    0x3C: ("istore_1", 0, "none"), 0x3D: ("istore_2", 0, "none"), 0x3E: ("istore_3", 0, "none"),
    0x3F: ("lstore_0", 0, "none"), 0x40: ("lstore_1", 0, "none"), 0x41: ("lstore_2", 0, "none"), 0x42: ("lstore_3", 0, "none"),
    0x4B: ("astore_0", 0, "none"), 0x4C: ("astore_1", 0, "none"), 0x4D: ("astore_2", 0, "none"), 0x4E: ("astore_3", 0, "none"),
    0x57: ("pop", 0, "none"), 0x59: ("dup", 0, "none"), 0x5A: ("dup_x1", 0, "none"), 0x5F: ("swap", 0, "none"),
    0x60: ("iadd", 0, "none"), 0x64: ("isub", 0, "none"), 0x68: ("imul", 0, "none"),
    0x74: ("ineg", 0, "none"),
    0x75: ("lshl", 0, "none"), 0x7B: ("lushr", 0, "none"),
    0x80: ("ior", 0, "none"), 0x83: ("ixor", 0, "none"),
    0x84: ("iinc", 2, "iinc"),
    0x85: ("i2l", 0, "none"), 0x88: ("i2f", 0, "none"), 0x8B: ("f2i", 0, "none"),
    0x94: ("lcmp", 0, "none"), 0x95: ("fcmpl", 0, "none"), 0x96: ("fcmpg", 0, "none"), 0x97: ("dcmpl", 0, "none"), 0x98: ("dcmpg", 0, "none"),
    0x99: ("ifeq", 2, "branch"), 0x9A: ("ifne", 2, "branch"), 0x9B: ("iflt", 2, "branch"),
    0x9C: ("ifge", 2, "branch"), 0x9D: ("ifgt", 2, "branch"), 0x9E: ("ifle", 2, "branch"),
    0x9F: ("if_icmpeq", 2, "branch"), 0xA0: ("if_icmpne", 2, "branch"), 0xA1: ("if_icmplt", 2, "branch"),
    0xA2: ("if_icmpge", 2, "branch"), 0xA3: ("if_icmpgt", 2, "branch"), 0xA4: ("if_icmple", 2, "branch"),
    0xA5: ("if_acmpeq", 2, "branch"), 0xA6: ("if_acmpne", 2, "branch"),
    0xA7: ("goto", 2, "branch"),
    0xAA: ("tableswitch", -1, "table"), 0xAB: ("lookupswitch", -1, "lookup"),
    0xAC: ("ireturn", 0, "none"), 0xAD: ("lreturn", 0, "none"), 0xAE: ("freturn", 0, "none"),
    0xAF: ("dreturn", 0, "none"), 0xB0: ("areturn", 0, "none"), 0xB1: ("return", 0, "none"),
    0xB2: ("getstatic", 2, "cp"), 0xB3: ("putstatic", 2, "cp"), 0xB4: ("getfield", 2, "cp"), 0xB5: ("putfield", 2, "cp"),
    0xB6: ("invokevirtual", 2, "cp"), 0xB7: ("invokespecial", 2, "cp"), 0xB8: ("invokestatic", 2, "cp"),
    0xB9: ("invokeinterface", 4, "cp"), 0xBA: ("invokedynamic", 4, "cp2"),
    0xBB: ("new", 2, "cp"), 0xBC: ("newarray", 1, "none"), 0xBD: ("anewarray", 2, "cp"),
    0xBE: ("arraylength", 0, "none"), 0xBF: ("athrow", 0, "none"),
    0xC0: ("checkcast", 2, "cp"), 0xC1: ("instanceof", 2, "cp"),
    0xC2: ("monitorenter", 0, "none"), 0xC3: ("monitorexit", 0, "none"),
    0xC6: ("ifnull", 2, "branch"), 0xC7: ("ifnonnull", 2, "branch"),
    0xC8: ("goto_w", 4, "branch"), 0xC9: ("jsr_w", 4, "branch"),
}


def disasm(code, pool, out):
    i = 0
    n = len(code)

    def utf(idx):
        e = pool[idx] if 0 < idx < len(pool) else None
        return e[1] if e and e[0] == "utf8" else f"?{idx}"

    def resolve(idx, depth=0):
        e = pool[idx] if 0 < idx < len(pool) else None
        if e is None or depth > 6:
            return f"?{idx}"
        if e[0] == "utf8":
            return e[1]
        if e[0] == "ref":
            return resolve(e[1], depth + 1)
        if e[0] == "ref2":
            return f"{resolve(e[1], depth+1)}.{resolve(e[2], depth+1)}"
        if e[0] == "num":
            v = e[1]
            if v >= 0x80000000:
                v -= 0x100000000
            return str(v)
        if e[0] == "num8":
            return str(e[1])
        if e[0] == "mh":
            return f"mh:{resolve(e[2], depth+1)}"
        return f"?{idx}"

    while i < n:
        op = code[i]
        name, nb, kind = OPS.get(op, (f"op_{op:02x}", 0, "none"))
        s = f"  {i:4d}: {name}"
        if kind == "none":
            if nb:
                vals = code[i+1:i+1+nb]
                s += " " + " ".join(str(v) for v in vals)
            i += 1 + nb
        elif kind in ("branch",):
            off = struct.unpack_from(">h", code, i + 1)[0]
            s += f" -> {i + off}"
            i += 3
        elif kind == "cp":
            idx = code[i+1] if nb == 1 else struct.unpack_from(">H", code, i + 1)[0]
            s += f" #{idx} {resolve(idx)}"
            i += 1 + nb
        elif kind == "cp2":  # invokedynamic / invokeinterface
            idx = struct.unpack_from(">H", code, i + 1)[0]
            s += f" #{idx} {resolve(idx)}"
            i += 1 + nb
        elif kind == "local":
            s += f" {code[i+1]}"
            i += 2
        elif kind == "iinc":
            s += f" {code[i+1]} {struct.unpack_from('>b', code, i+2)[0]}"
            i += 3
        elif kind == "table":
            base = i
            pad = (4 - ((i + 1) % 4)) % 4
            i2 = i + 1 + pad
            df, lo, hi = struct.unpack_from(">iii", code, i2)
            s += f" default={base+df} lo={lo} hi={hi}"
            i = i2 + 12 + (hi - lo + 1) * 4
        elif kind == "lookup":
            base = i
            pad = (4 - ((i + 1) % 4)) % 4
            i2 = i + 1 + pad
            df, npairs = struct.unpack_from(">ii", code, i2)
            s += f" default={base+df} pairs={npairs}"
            i = i2 + 8 + npairs * 8
        else:
            i += 1 + nb
        out.append(s)


def find_code(data_path):
    _, members, pool = parse(data_path)
    return members, pool


if __name__ == "__main__":
    path = sys.argv[1]
    targets = sys.argv[2:]
    _, members, pool = parse(path)

    # re-walk the raw file to reach method Code attributes: reuse parse's field walk
    # then continue into methods by re-parsing the buffer manually
    data = open(path, "rb").read()
    off = 8
    count = struct.unpack_from(">H", data, off)[0]
    off += 2
    # skip pool
    i = 1
    while i < count:
        tag = data[off]
        off += 1
        if tag == 1:
            ln = struct.unpack_from(">H", data, off)[0]
            off += 2 + ln
        elif tag in (7, 8, 16, 19, 20):
            off += 2
        elif tag == 15:
            off += 3
        elif tag in (3, 4):
            off += 4
        elif tag in (5, 6):
            off += 8
            i += 1
        else:
            off += 4
        i += 1
    off += 6
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc

    def utf(idx):
        e = pool[idx] if 0 < idx < len(pool) else None
        return e[1] if e and e[0] == "utf8" else f"?{idx}"

    def skip_attrs(off):
        n = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(n):
            off += 2
            alen = struct.unpack_from(">I", data, off)[0]
            off += 4 + alen
        return off

    # fields
    nf = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(nf):
        off += 6
        off = skip_attrs(off)
    # methods
    nm = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(nm):
        acc, mname, mdesc = struct.unpack_from(">HHH", data, off)
        off += 6
        na = struct.unpack_from(">H", data, off)[0]
        off += 2
        name = utf(mname)
        desc = utf(mdesc)
        for _ in range(na):
            aname = utf(struct.unpack_from(">H", data, off)[0])
            off += 2
            alen = struct.unpack_from(">I", data, off)[0]
            body_off = off + 4
            if aname == "Code" and (not targets or name in targets):
                maxs = struct.unpack_from(">HH", data, body_off,)[0]
                clen = struct.unpack_from(">I", data, body_off + 4)[0]
                code = data[body_off + 8 : body_off + 8 + clen]
                print(f"\n=== {name}{desc} (code {clen} bytes) ===")
                out = []
                try:
                    disasm(code, pool, out)
                except Exception as e:
                    out.append(f"  <disasm stopped: {e}>")
                print("\n".join(out))
            off += 4 + alen
