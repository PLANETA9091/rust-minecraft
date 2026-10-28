#!/usr/bin/env python3
"""Raw bytecode disassembler (full opcode table incl. d-arith the
classfile_dump.py trace walker leaves as '?? opcode' noise).

Usage: python3 scripts/raw_disasm.py <jar> <net/minecraft/Class> <method> [<method> ...]
Prints pc, mnemonic, operand for EVERY instruction of the named methods.
"""
import struct, sys, zipfile

MNEMONIC = {
    0x00: 'nop', 0x01: 'aconst_null', 0x02: 'iconst_m1', 0x03: 'iconst_0',
    0x04: 'iconst_1', 0x05: 'iconst_2', 0x06: 'iconst_3', 0x07: 'iconst_4',
    0x08: 'iconst_5', 0x09: 'lconst_0', 0x0a: 'lconst_1', 0x0b: 'fconst_0',
    0x0c: 'fconst_1', 0x0d: 'fconst_2', 0x0e: 'dconst_0', 0x0f: 'dconst_1',
    0x10: 'bipush', 0x11: 'sipush', 0x12: 'ldc', 0x13: 'ldc_w', 0x14: 'ldc2_w',
    0x15: 'iload', 0x16: 'lload', 0x17: 'fload', 0x18: 'dload', 0x19: 'aload',
    0x1a: 'iload_0', 0x1b: 'iload_1', 0x1c: 'iload_2', 0x1d: 'iload_3',
    0x1e: 'lload_0', 0x1f: 'lload_1', 0x20: 'lload_2', 0x21: 'lload_3',
    0x22: 'fload_0', 0x23: 'fload_1', 0x24: 'fload_2', 0x25: 'fload_3',
    0x26: 'dload_0', 0x27: 'dload_1', 0x28: 'dload_2', 0x29: 'dload_3',
    0x2a: 'aload_0', 0x2b: 'aload_1', 0x2c: 'aload_2', 0x2d: 'aload_3',
    0x2e: 'iaload', 0x2f: 'laload', 0x30: 'faload', 0x31: 'daload',
    0x32: 'aaload', 0x33: 'baload', 0x34: 'caload', 0x35: 'saload',
    0x36: 'istore', 0x37: 'lstore', 0x38: 'fstore', 0x39: 'dstore',
    0x3a: 'astore',
    0x3b: 'istore_0', 0x3c: 'istore_1', 0x3d: 'istore_2', 0x3e: 'istore_3',
    0x3f: 'lstore_0', 0x40: 'lstore_1', 0x41: 'lstore_2', 0x42: 'lstore_3',
    0x43: 'fstore_0', 0x44: 'fstore_1', 0x45: 'fstore_2', 0x46: 'fstore_3',
    0x47: 'dstore_0', 0x48: 'dstore_1', 0x49: 'dstore_2', 0x4a: 'dstore_3',
    0x4b: 'astore_0', 0x4c: 'astore_1', 0x4d: 'astore_2', 0x4e: 'astore_3',
    0x4f: 'iastore', 0x50: 'lastore', 0x51: 'fastore', 0x52: 'dastore',
    0x53: 'aastore', 0x54: 'bastore', 0x55: 'castore', 0x56: 'sastore',
    0x57: 'pop', 0x58: 'pop2', 0x59: 'dup', 0x5a: 'dup_x1', 0x5b: 'dup_x2',
    0x5c: 'dup2', 0x5d: 'dup2_x1', 0x5e: 'dup2_x2', 0x5f: 'swap',
    0x60: 'iadd', 0x61: 'ladd', 0x62: 'fadd', 0x63: 'dadd',
    0x64: 'isub', 0x65: 'lsub', 0x66: 'fsub', 0x67: 'dsub',
    0x68: 'imul', 0x69: 'lmul', 0x6a: 'fmul', 0x6b: 'dmul',
    0x6c: 'idiv', 0x6d: 'ldiv', 0x6e: 'fdiv', 0x6f: 'ddiv',
    0x70: 'irem', 0x71: 'lrem', 0x72: 'frem', 0x73: 'drem',
    0x74: 'ineg', 0x75: 'lneg', 0x76: 'fneg', 0x77: 'dneg',
    0x78: 'ishl', 0x79: 'lshl', 0x7a: 'ishr', 0x7b: 'lshr',
    0x7c: 'iushr', 0x7d: 'lushr', 0x7e: 'iand', 0x7f: 'land',
    0x80: 'ior', 0x81: 'lor', 0x82: 'ixor', 0x83: 'lor',
    0x84: 'iinc', 0x85: 'i2l', 0x86: 'i2f', 0x87: 'i2d', 0x88: 'l2i',
    0x89: 'l2f', 0x8a: 'l2d', 0x8b: 'f2i', 0x8c: 'f2l', 0x8d: 'f2d',
    0x8e: 'd2i', 0x8f: 'd2l', 0x90: 'd2f', 0x91: 'i2b', 0x92: 'i2c',
    0x93: 'i2s',
    0x94: 'lcmp', 0x95: 'fcmpl', 0x96: 'fcmpg', 0x97: 'dcmpl', 0x98: 'dcmpg',
    0x99: 'ifeq', 0x9a: 'ifne', 0x9b: 'iflt', 0x9c: 'ifge', 0x9d: 'ifgt',
    0x9e: 'ifle',
    0x9f: 'if_icmpeq', 0xa0: 'if_icmpne', 0xa1: 'if_icmplt', 0xa2: 'if_icmpge',
    0xa3: 'if_icmpgt', 0xa4: 'if_icmple', 0xa5: 'if_acmpeq', 0xa6: 'if_acmpne',
    0xa7: 'goto', 0xa8: 'jsr', 0xa9: 'ret',
    0xaa: 'tableswitch', 0xab: 'lookupswitch',
    0xac: 'ireturn', 0xad: 'lreturn', 0xae: 'freturn', 0xaf: 'dreturn',
    0xb0: 'areturn', 0xb1: 'return',
    0xb2: 'getstatic', 0xb3: 'putstatic', 0xb4: 'getfield', 0xb5: 'putfield',
    0xb6: 'invokevirtual', 0xb7: 'invokespecial', 0xb8: 'invokestatic',
    0xb9: 'invokeinterface', 0xba: 'invokedynamic',
    0xbb: 'new', 0xbc: 'newarray', 0xbd: 'anewarray', 0xbe: 'arraylength',
    0xbf: 'athrow', 0xc0: 'checkcast', 0xc1: 'instanceof',
    0xc2: 'monitorenter', 0xc3: 'monitorexit',
    0xc4: 'wide', 0xc5: 'multianewarray',
    0xc6: 'ifnull', 0xc7: 'ifnonnull', 0xc8: 'goto_w', 0xc9: 'jsr_w',
}
# opcodes with 1 local operand, 2-byte branch, cp index(2), etc.
U1_OPS = {0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3a,
          0xa9, 0xbc, 0x12}
U2_OPS = {0x13, 0x14, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0,
          0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xb2, 0xb3,
          0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xba, 0xbb, 0xbd, 0xc0, 0xc1,
          0xc2, 0xc3, 0xc6, 0xc7, 0xc8, 0xc9, 0xc5}
LOCAL2_OPS = {0x84}  # iinc: index + const


class R:
    def __init__(self, d):
        self.d = d
        self.p = 0

    def u1(self):
        v = self.d[self.p]; self.p += 1; return v

    def u2(self):
        v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v

    def u4(self):
        v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v

    def s2(self):
        v = struct.unpack_from('>h', self.d, self.p)[0]; self.p += 2; return v

    def s4(self):
        v = struct.unpack_from('>i', self.d, self.p)[0]; self.p += 4; return v

    def f4(self):
        v = struct.unpack_from('>f', self.d, self.p)[0]; self.p += 4; return v

    def f8(self):
        v = struct.unpack_from('>d', self.d, self.p)[0]; self.p += 8; return v

    def raw(self, n):
        v = self.d[self.p:self.p+n]; self.p += n; return v


def cp_all(data):
    r = R(data)
    # THE HEADER SKIP the autopush-302 round banks: the cp count the
    # 8-byte classfile header FOLLOWS (magic 0xCAFEBABE + minor + major)
    # — the offset-0 read the magic's 0xCAFE the count wore, the walk
    # the opcode bytes the bogus tags hit (the tag-186 = 0xBA the
    # invokedynamic OPCODE the misaligned reader answers)
    magic = r.u4()
    if magic != 0xCAFEBABE:
        raise ValueError(f'not a classfile: magic 0x{magic:08x}')
    r.u2()  # minor
    r.u2()  # major
    n = r.u2()
    cp = [None] * n
    i = 1
    while i < n:
        tag = r.u1()
        if tag == 1:
            ln = r.u2(); cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
        elif tag == 3: cp[i] = ('Int', r.s4())
        elif tag == 4: cp[i] = ('Float', r.f4())
        elif tag == 5: cp[i] = ('Long', struct.unpack_from('>q', r.raw(8))[0]); i += 1
        elif tag == 6: cp[i] = ('Double', r.f8()); i += 1
        elif tag == 7: cp[i] = ('Class', r.u2())
        elif tag == 8: cp[i] = ('Str', r.u2())
        elif tag in (9, 10, 11): cp[i] = ('Ref', r.u2(), r.u2())
        elif tag == 12: cp[i] = ('Name', r.u2(), r.u2())
        elif tag == 15: cp[i] = ('MethodHandle', r.u1(), r.u2())
        elif tag == 16: cp[i] = ('MethodType', r.u2())
        elif tag == 17: cp[i] = ('Dynamic', r.u2(), r.u2())
        elif tag == 18: cp[i] = ('InvokeDynamic', r.u2(), r.u2())
        elif tag == 19: cp[i] = ('Module', r.u2())
        elif tag == 20: cp[i] = ('Package', r.u2())
        else: raise ValueError(f'tag {tag} at offset {r.p}')
        i += 1
    return cp


def utf8(cp, i):
    e = cp[i]
    if e and e[0] == 'Utf8':
        return e[1]
    return '?'


def ref_str(cp, i):
    e = cp[i]
    if e and e[0] == 'Class':
        return utf8(cp, e[1])
    if e and e[0] == 'Str':
        return utf8(cp, e[1])
    return '?'


def member_str(cp, i):
    e = cp[i]
    if e and e[0] == 'Ref':
        cls = utf8(cp, cp[e[1]][1]) if cp[e[1]][0] == 'Class' else '?'
        nt = cp[e[2]]
        name = utf8(cp, nt[1]); desc = utf8(cp, nt[2])
        return f"{cls}.{name} {desc}"
    return '?'


def disasm(code, cp):
    r = R(code)
    out = []
    while r.p < len(code):
        pc = r.p
        op = r.u1()
        m = MNEMONIC.get(op, f'0x{op:02x}?')
        if op == 0xaa:  # tableswitch
            pad = (4 - (r.p % 4)) % 4
            r.raw(pad)
            df = r.s4(); lo = r.s4(); hi = r.s4()
            n = hi - lo + 1
            offs = [r.s4() for _ in range(n)]
            out.append(f"{pc:5d} tableswitch lo={lo} hi={hi} default->{pc+df} "
                       + " ".join(f"{lo+i}->{pc+o}" for i, o in enumerate(offs)))
            continue
        if op == 0xab:  # lookupswitch
            pad = (4 - (r.p % 4)) % 4
            r.raw(pad)
            df = r.s4(); n = r.s4()
            pairs = [(r.s4(), r.s4()) for _ in range(n)]
            out.append(f"{pc:5d} lookupswitch default->{pc+df} "
                       + " ".join(f"{k}->{pc+v}" for k, v in pairs))
            continue
        if op == 0xc4:  # wide
            wop = r.u1()
            if wop == 0x84:
                idx = r.u2(); c = struct.unpack_from('>h', r.raw(2))[0]
                out.append(f"{pc:5d} wide iinc {idx} {c}")
            else:
                idx = r.u2()
                out.append(f"{pc:5d} wide {MNEMONIC.get(wop, wop)} {idx}")
            continue
        if op == 0x14 or op == 0x13:
            i = r.u2()
            e = cp[i]
            v = e[1] if e and e[0] in ('Long', 'Double') else ref_str(cp, i)
            out.append(f"{pc:5d} {m} #{i} = {v}")
            continue
        if op == 0x12:
            i = r.u1()
            out.append(f"{pc:5d} ldc #{i} = {ref_str(cp, i)}")
            continue
        if op in U1_OPS:
            out.append(f"{pc:5d} {m} {r.u1()}")
            continue
        if op in LOCAL2_OPS:
            i = r.u1(); c = struct.unpack_from('>b', r.raw(1))[0]
            out.append(f"{pc:5d} iinc {i} {c}")
            continue
        if op == 0x10:
            out.append(f"{pc:5d} bipush {struct.unpack_from('>b', r.raw(1))[0]}")
            continue
        if op == 0x11:
            out.append(f"{pc:5d} sipush {r.s2()}")
            continue
        if op == 0xb9:  # invokeinterface
            i = r.u2(); r.u2()
            out.append(f"{pc:5d} invokeinterface {member_str(cp, i)}")
            continue
        if op == 0xba:  # invokedynamic
            i = r.u2(); r.u2()
            out.append(f"{pc:5d} invokedynamic #{i}")
            continue
        if op in U2_OPS:
            i = r.u2()
            note = ''
            if op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8):
                note = member_str(cp, i)
            elif op in (0xbb, 0xbd, 0xc0, 0xc1):
                note = ref_str(cp, i)
            out.append(f"{pc:5d} {m} #{i}" + (f"  {note}" if note else ""))
            continue
        out.append(f"{pc:5d} {m}")
    return out


def parse_class(data, want):
    r = R(data)
    r.raw(4)          # magic
    r.raw(2) + r.raw(2)  # minor+major
    n = r.u2()
    # parse cp inline (need reader positions)
    cp = [None] * n
    i = 1
    while i < n:
        tag = r.u1()
        if tag == 1:
            ln = r.u2(); cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
        elif tag == 3: cp[i] = ('Int', r.s4())
        elif tag == 4: cp[i] = ('Float', r.f4())
        elif tag == 5: cp[i] = ('Long', struct.unpack_from('>q', r.raw(8))[0]); i += 1
        elif tag == 6: cp[i] = ('Double', r.f8()); i += 1
        elif tag == 7: cp[i] = ('Class', r.u2())
        elif tag == 8: cp[i] = ('Str', r.u2())
        elif tag in (9, 10, 11): cp[i] = ('Ref', r.u2(), r.u2())
        elif tag == 12: cp[i] = ('Name', r.u2(), r.u2())
        elif tag == 15: cp[i] = ('MethodHandle', r.u1(), r.u2())
        elif tag == 16: cp[i] = ('MethodType', r.u2())
        elif tag == 17: cp[i] = ('Dynamic', r.u2(), r.u2())
        elif tag == 18: cp[i] = ('InvokeDynamic', r.u2(), r.u2())
        elif tag == 19: cp[i] = ('Module', r.u2())
        elif tag == 20: cp[i] = ('Package', r.u2())
        else: raise ValueError(f'tag {tag}')
        i += 1
    flags = r.u2(); this_cls = r.u2(); super_cls = r.u2()
    nif = r.u2()
    for _ in range(nif): r.u2()
    nf = r.u2()
    for _ in range(nf):
        r.u2(); r.u2(); r.u2()
        na = r.u2()
        for _ in range(na):
            r.u2()
            alen = r.u4(); r.raw(alen)
    nm = r.u2()
    for _ in range(nm):
        r.u2(); name_i = r.u2(); desc_i = r.u2()
        name = utf8(cp, name_i); desc = utf8(cp, desc_i)
        na = r.u2()
        code = None
        for _ in range(na):
            a_name = utf8(cp, r.u2())
            alen = r.u4(); adata = r.raw(alen)
            if a_name == 'Code':
                clen = struct.unpack_from('>I', adata, 4)[0]
                code = adata[8:8+clen]
        if name in want:
            yield name, desc, code, cp


def main():
    jar_path, cls, *names = sys.argv[1:]
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    want = set(names)
    for name, desc, code, cp in parse_class(data, want):
        print(f"=== {name} {desc} ===")
        if code is None:
            print("  (abstract/native)")
            continue
        for line in disasm(code, cp):
            print(line)
        print()


if __name__ == '__main__':
    main()
