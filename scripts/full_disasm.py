#!/usr/bin/env python3
"""Full (lossless) javap-style bytecode dump for one method of one class.

Usage: python3 scripts/full_disasm.py <jar> <net/minecraft/Class> <method-name-substring>

Prints every instruction with pc, mnemonic and decoded operands (branch
targets resolved), plus the exception table. Unlike raw_disasm.py this
does NOT fold branches/arithmetic — the ?? gaps the simple dumper keeps
are the seams the evidence walk needs.
"""
import struct
import sys
import zipfile

MNEMONIC = {
    0x00: ("nop", 0), 0x01: ("aconst_null", 0), 0x02: ("iconst_m1", 0),
    0x03: ("iconst_0", 0), 0x04: ("iconst_1", 0), 0x05: ("iconst_2", 0),
    0x06: ("iconst_3", 0), 0x07: ("iconst_4", 0), 0x08: ("iconst_5", 0),
    0x09: ("lconst_0", 0), 0x0A: ("lconst_1", 0), 0x0B: ("fconst_0", 0),
    0x0C: ("fconst_1", 0), 0x0D: ("fconst_2", 0), 0x0E: ("dconst_0", 0),
    0x0F: ("dconst_1", 0), 0x10: ("bipush", 1), 0x11: ("sipush", 2),
    0x12: ("ldc", 1), 0x13: ("ldc_w", 2), 0x14: ("ldc2_w", 2),
    0x15: ("iload", 1), 0x16: ("lload", 1), 0x17: ("fload", 1),
    0x18: ("dload", 1), 0x19: ("aload", 1),
    0x1A: ("iload_0", 0), 0x1B: ("iload_1", 0), 0x1C: ("iload_2", 0), 0x1D: ("iload_3", 0),
    0x1E: ("lload_0", 0), 0x1F: ("lload_1", 0), 0x20: ("lload_2", 0), 0x21: ("lload_3", 0),
    0x22: ("fload_0", 0), 0x23: ("fload_1", 0), 0x24: ("fload_2", 0), 0x25: ("fload_3", 0),
    0x26: ("dload_0", 0), 0x27: ("dload_1", 0), 0x28: ("dload_2", 0), 0x29: ("dload_3", 0),
    0x2A: ("aload_0", 0), 0x2B: ("aload_1", 0), 0x2C: ("aload_2", 0), 0x2D: ("aload_3", 0),
    0x2E: ("iaload", 0), 0x2F: ("laload", 0), 0x30: ("faload", 0), 0x31: ("daload", 0),
    0x32: ("aaload", 0), 0x33: ("baload", 0), 0x34: ("caload", 0), 0x35: ("saload", 0),
    0x36: ("istore", 1), 0x37: ("lstore", 1), 0x38: ("fstore", 1),
    0x39: ("dstore", 1), 0x3A: ("astore", 1),
    0x3B: ("istore_0", 0), 0x3C: ("istore_1", 0), 0x3D: ("istore_2", 0), 0x3E: ("istore_3", 0),
    0x3F: ("lstore_0", 0), 0x40: ("lstore_1", 0), 0x41: ("lstore_2", 0), 0x42: ("lstore_3", 0),
    0x43: ("fstore_0", 0), 0x44: ("fstore_1", 0), 0x45: ("fstore_2", 0), 0x46: ("fstore_3", 0),
    0x47: ("dstore_0", 0), 0x48: ("dstore_1", 0), 0x49: ("dstore_2", 0), 0x4A: ("dstore_3", 0),
    0x4B: ("astore_0", 0), 0x4C: ("astore_1", 0), 0x4D: ("astore_2", 0), 0x4E: ("astore_3", 0),
    0x4F: ("iastore", 0), 0x50: ("lastore", 0), 0x51: ("fastore", 0), 0x52: ("dastore", 0),
    0x53: ("aastore", 0), 0x54: ("bastore", 0), 0x55: ("castore", 0), 0x56: ("sastore", 0),
    0x57: ("pop", 0), 0x58: ("pop2", 0), 0x59: ("dup", 0), 0x5A: ("dup_x1", 0),
    0x5B: ("dup_x2", 0), 0x5C: ("dup2", 0), 0x5D: ("dup2_x1", 0), 0x5E: ("dup2_x2", 0),
    0x5F: ("swap", 0),
    0x60: ("iadd", 0), 0x61: ("ladd", 0), 0x62: ("fadd", 0), 0x63: ("dadd", 0),
    0x64: ("isub", 0), 0x65: ("lsub", 0), 0x66: ("fsub", 0), 0x67: ("dsub", 0),
    0x68: ("imul", 0), 0x69: ("lmul", 0), 0x6A: ("fmul", 0), 0x6B: ("dmul", 0),
    0x6C: ("idiv", 0), 0x6D: ("ldiv", 0), 0x6E: ("fdiv", 0), 0x6F: ("ddiv", 0),
    0x70: ("irem", 0), 0x71: ("lrem", 0), 0x72: ("frem", 0), 0x73: ("drem", 0),
    0x74: ("ineg", 0), 0x75: ("lneg", 0), 0x76: ("fneg", 0), 0x77: ("dneg", 0),
    0x78: ("ishl", 0), 0x79: ("lshl", 0), 0x7A: ("ishr", 0), 0x7B: ("lshr", 0),
    0x7C: ("iushr", 0), 0x7D: ("lushr", 0), 0x7E: ("iand", 0), 0x7F: ("land", 0),
    0x80: ("ior", 0), 0x81: ("lor", 0), 0x82: ("ixor", 0), 0x83: ("lxor", 0),
    0x84: ("iinc", 2), 0x85: ("i2l", 0), 0x86: ("i2f", 0), 0x87: ("i2d", 0),
    0x88: ("l2i", 0), 0x89: ("l2f", 0), 0x8A: ("l2d", 0), 0x8B: ("f2i", 0),
    0x8C: ("f2l", 0), 0x8D: ("f2d", 0), 0x8E: ("d2i", 0), 0x8F: ("d2l", 0),
    0x90: ("d2f", 0), 0x91: ("i2b", 0), 0x92: ("i2c", 0), 0x93: ("i2s", 0),
    0x94: ("lcmp", 0), 0x95: ("fcmpl", 0), 0x96: ("fcmpg", 0),
    0x97: ("dcmpl", 0), 0x98: ("dcmpg", 0),
    0x99: ("ifeq", 2), 0x9A: ("ifne", 2), 0x9B: ("iflt", 2), 0x9C: ("ifge", 2),
    0x9D: ("ifgt", 2), 0x9E: ("ifle", 2),
    0x9F: ("if_icmpeq", 2), 0xA0: ("if_icmpne", 2), 0xA1: ("if_icmplt", 2),
    0xA2: ("if_icmpge", 2), 0xA3: ("if_icmpgt", 2), 0xA4: ("if_icmple", 2),
    0xA5: ("if_acmpeq", 2), 0xA6: ("if_acmpne", 2),
    0xA7: ("goto", 2), 0xA8: ("jsr", 2), 0xA9: ("ret", 1),
    0xAA: ("tableswitch", -1), 0xAB: ("lookupswitch", -1),
    0xAC: ("ireturn", 0), 0xAD: ("lreturn", 0), 0xAE: ("freturn", 0),
    0xAF: ("dreturn", 0), 0xB0: ("areturn", 0), 0xB1: ("return", 0),
    0xB2: ("getstatic", 2), 0xB3: ("putstatic", 2), 0xB4: ("getfield", 2),
    0xB5: ("putfield", 2), 0xB6: ("invokevirtual", 2), 0xB7: ("invokespecial", 2),
    0xB8: ("invokestatic", 2), 0xB9: ("invokeinterface", 4), 0xBA: ("invokedynamic", 4),
    0xBB: ("new", 2), 0xBC: ("newarray", 1), 0xBD: ("anewarray", 2),
    0xBE: ("arraylength", 0), 0xBF: ("athrow", 0),
    0xC0: ("checkcast", 2), 0xC1: ("instanceof", 2),
    0xC2: ("monitorenter", 0), 0xC3: ("monitorexit", 0),
    0xC4: ("wide", -1), 0xC5: ("multianewarray", 3),
    0xC6: ("ifnull", 2), 0xC7: ("ifnonnull", 2), 0xC8: ("goto_w", 4), 0xC9: ("jsr_w", 4),
}


def parse_cp(data):
    count = struct.unpack_from(">H", data, 8)[0]
    cp = {}
    i = 10
    idx = 1
    while idx < count:
        tag = data[i]
        if tag == 1:
            ln = struct.unpack_from(">H", data, i + 1)[0]
            cp[idx] = ("utf8", data[i + 3:i + 3 + ln].decode("utf-8", "replace"))
            i += 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            cp[idx] = (tag, struct.unpack_from(">H", data, i + 1)[0])
            i += 3
        elif tag == 15:
            cp[idx] = (tag, data[i + 1], struct.unpack_from(">H", data, i + 2)[0])
            i += 4
        elif tag in (17, 18):
            cp[idx] = (tag, struct.unpack_from(">HH", data, i + 1))
            i += 5
        elif tag in (3, 4):
            cp[idx] = (tag, struct.unpack_from(">I", data, i + 1)[0])
            i += 5
        elif tag in (5, 6):
            cp[idx] = (tag, struct.unpack_from(">Q", data, i + 1)[0])
            i += 9
            idx += 1  # long/double take two slots
        elif tag == 9:
            cp[idx] = ("field", struct.unpack_from(">HH", data, i + 1))
            i += 5
        elif tag == 10:
            cp[idx] = ("method", struct.unpack_from(">HH", data, i + 1))
            i += 5
        elif tag == 11:
            cp[idx] = ("iface", struct.unpack_from(">HH", data, i + 1))
            i += 5
        elif tag == 12:
            cp[idx] = ("name_type", struct.unpack_from(">HH", data, i + 1))
            i += 5
        else:
            raise ValueError(f"unknown cp tag {tag} at index {idx}")
        idx += 1
    return cp, i


def utf8(cp, i):
    e = cp.get(i)
    if e and e[0] == "utf8":
        return e[1]
    return f"#{i}"


def member_name(cp, ref_idx):
    e = cp.get(ref_idx)
    if not e:
        return f"#{ref_idx}"
    cls = utf8(cp, e[1][0])
    nt = cp.get(e[1][1])
    if nt and nt[0] == "name_type":
        return f"{cls}.{utf8(cp, nt[1][0])}:{utf8(cp, nt[1][1])}"
    return f"{cls}#{e[1][1]}"


def disasm(code, cp):
    pc = 0
    out = []
    n = len(code)
    while pc < n:
        op = code[pc]
        name, extra = MNEMONIC.get(op, (f"0x{op:02x}?", 0))
        start = pc
        pc += 1
        args = ""
        if name == "tableswitch":
            pad = (4 - (pc % 4)) % 4
            pc += pad
            default = struct.unpack_from(">i", code, pc)[0]
            lo = struct.unpack_from(">i", code, pc + 4)[0]
            hi = struct.unpack_from(">i", code, pc + 8)[0]
            pc += 12
            targets = []
            for k in range(hi - lo + 1):
                off = struct.unpack_from(">i", code, pc)[0]
                targets.append(f"{lo + k}->start+{start + off}")
                pc += 4
            args = f"default=start+{start + default} " + " ".join(targets)
        elif name == "lookupswitch":
            pad = (4 - (pc % 4)) % 4
            pc += pad
            default = struct.unpack_from(">i", code, pc)[0]
            npairs = struct.unpack_from(">i", code, pc + 4)[0]
            pc += 8
            pairs = []
            for k in range(npairs):
                match, off = struct.unpack_from(">ii", code, pc)
                pairs.append(f"{match}->start+{start + off}")
                pc += 8
            args = f"default=start+{start + default} " + " ".join(pairs)
        elif name == "wide":
            wop = code[pc]
            wname, _ = MNEMONIC.get(wop, (f"0x{wop:02x}?", 0))
            if wop == 0x84:
                idxb = struct.unpack_from(">H", code, pc + 1)[0]
                const = struct.unpack_from(">h", code, pc + 3)[0]
                args = f"{wname} {idxb} {const}"
                pc += 5
            else:
                idxb = struct.unpack_from(">H", code, pc + 1)[0]
                args = f"{wname} {idxb}"
                pc += 3
        elif extra == 1:
            v = code[pc]
            pc += 1
            if name in ("ldc",):
                e = cp.get(v)
                args = f"#{v}" + (f" <{e[1]!r}>" if e and e[0] == "utf8" else "")
            else:
                args = f"{v}"
        elif extra == 2:
            if name in ("bipush",):
                args = str(struct.unpack_from(">b", code, pc)[0])
            elif name in ("sipush",):
                args = str(struct.unpack_from(">h", code, pc)[0])
            elif name.startswith("if") or name in ("goto", "jsr"):
                off = struct.unpack_from(">h", code, pc)[0]
                args = f"start+{start + off}"
            elif name in ("getstatic", "putstatic", "getfield", "putfield"):
                idx = struct.unpack_from(">H", code, pc)[0]
                args = f"{member_name(cp, idx)}"
            elif name.startswith("invoke") or name in ("new", "anewarray", "checkcast", "instanceof"):
                idx = struct.unpack_from(">H", code, pc)[0]
                e = cp.get(idx)
                if e and e[0] in ("method", "field", "iface"):
                    args = member_name(cp, idx)
                elif e and e[0] == 7:
                    args = utf8(cp, e[1])
                else:
                    args = f"#{idx}"
            else:
                idx = struct.unpack_from(">H", code, pc)[0]
                args = f"#{idx}"
            pc += 2
        elif extra == 4:
            idx = struct.unpack_from(">H", code, pc)[0]
            e = cp.get(idx)
            args = member_name(cp, idx) if e and e[0] == "iface" else f"#{idx}"
            pc += 4
            if name == "invokeinterface":
                pc += 0  # count+zero already inside the 4 bytes (2 used)
                args = f"{args}"
        elif extra == 3:
            args = f"#{struct.unpack_from('>H', code, pc)[0]}"
            pc += 3
        elif extra == -1:
            pass
        out.append((start, name, args))
    return out


def main():
    jar_path, cls, needle = sys.argv[1], sys.argv[2], sys.argv[3]
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    cp, i = parse_cp(data)
    i += 6  # access_flags, this_class, super_class
    nif = struct.unpack_from(">H", data, i)[0]
    i += 2 + 2 * nif
    nfields = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nfields):
        i += 6
        attrs = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(attrs):
            i += 2
            alen = struct.unpack_from(">I", data, i)[0]
            i += 4 + alen
    nmethods = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(nmethods):
        _acc = struct.unpack_from(">H", data, i)[0]
        name = utf8(cp, struct.unpack_from(">H", data, i + 2)[0])
        desc = utf8(cp, struct.unpack_from(">H", data, i + 4)[0])
        i += 6
        attrs = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(attrs):
            aname = utf8(cp, struct.unpack_from(">H", data, i)[0])
            alen = struct.unpack_from(">I", data, i + 2)[0]
            body = data[i + 6:i + 6 + alen]
            if aname == "Code" and (needle in name or needle in f"{name}{desc}"):
                max_stack, max_locals, clen = struct.unpack_from(">HHI", body, 0)
                code_bytes = body[8:8 + clen]
                print(f"=== {name}{desc} max_stack={max_stack} max_locals={max_locals} ===")
                for start, mn, args in disasm(code_bytes, cp):
                    print(f"  {start:5d}: {mn} {args}")
                print()
            i += 6 + alen


if __name__ == "__main__":
    main()
