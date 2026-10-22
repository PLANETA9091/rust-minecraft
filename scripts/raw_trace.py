#!/usr/bin/env python3
"""RAW bytecode trace for one method — every opcode + operands, no folding.

Usage: python3 scripts/raw_trace.py <jar> <net/minecraft/Class> <method> [method2 ...]
The classfile_dump.py folding hides loop bodies (the case-3 particle loops
the snowball/egg walks wear); this tracer prints the straight-line code the
count laws answer. The autopush-140 round's tool (the Snowball case-3's
bipush-8 loop the folding never showed).
"""
import struct, sys, zipfile

OPCODES = {
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
    0x2e: 'iaload', 0x32: 'aaload', 0x33: 'baload',
    0x36: 'istore', 0x37: 'lstore', 0x38: 'fstore', 0x39: 'dstore', 0x3a: 'astore',
    0x3b: 'istore_0', 0x3c: 'istore_1', 0x3d: 'istore_2', 0x3e: 'istore_3',
    0x3f: 'lstore_0', 0x40: 'lstore_1', 0x41: 'lstore_2', 0x42: 'lstore_3',
    0x43: 'fstore_0', 0x44: 'fstore_1', 0x45: 'fstore_2', 0x46: 'fstore_3',
    0x47: 'dstore_0', 0x48: 'dstore_1', 0x49: 'dstore_2', 0x4a: 'dstore_3',
    0x4b: 'astore_0', 0x4c: 'astore_1', 0x4d: 'astore_2', 0x4e: 'astore_3',
    0x4f: 'iastore', 0x50: 'lastore', 0x51: 'fastore', 0x52: 'dastore', 0x53: 'aastore',
    0x57: 'pop', 0x58: 'pop2', 0x59: 'dup', 0x5a: 'dup_x1', 0x5b: 'dup_x2',
    0x5c: 'dup2', 0x5d: 'dup2_x1', 0x5e: 'dup2_x2', 0x5f: 'swap',
    0x60: 'iadd', 0x61: 'ladd', 0x62: 'fadd', 0x63: 'dadd',
    0x64: 'isub', 0x65: 'lsub', 0x66: 'fsub', 0x67: 'dsub',
    0x68: 'imul', 0x69: 'lmul', 0x6a: 'fmul', 0x6b: 'dmul',
    0x6c: 'idiv', 0x6d: 'ldiv', 0x6e: 'fdiv', 0x6f: 'ddiv',
    0x70: 'irem', 0x71: 'lrem', 0x72: 'frem', 0x73: 'drem',
    0x74: 'ineg', 0x78: 'ishl', 0x7a: 'ishr', 0x7c: 'iushr',
    0x80: 'iand', 0x82: 'ior', 0x84: 'iinc',
    0x85: 'i2l', 0x86: 'i2f', 0x87: 'i2d', 0x88: 'l2i', 0x8b: 'f2d', 0x8e: 'd2f',
    0x91: 'i2b', 0x94: 'lcmp', 0x95: 'fcmpl', 0x96: 'fcmpg', 0x97: 'dcmpl', 0x98: 'dcmpg',
    0x99: 'ifeq', 0x9a: 'ifne', 0x9b: 'iflt', 0x9c: 'ifge', 0x9d: 'ifgt', 0x9e: 'ifle',
    0x9f: 'if_icmpeq', 0xa0: 'if_icmpne', 0xa1: 'if_icmplt', 0xa2: 'if_icmpge',
    0xa3: 'if_icmpgt', 0xa4: 'if_icmple', 0xa5: 'if_acmpeq', 0xa6: 'if_acmpne',
    0xa7: 'goto', 0xa9: 'ret', 0xaa: 'tableswitch', 0xab: 'lookupswitch',
    0xac: 'ireturn', 0xad: 'lreturn', 0xae: 'freturn', 0xaf: 'dreturn',
    0xb0: 'areturn', 0xb1: 'return',
    0xb2: 'getstatic', 0xb4: 'getfield', 0xb5: 'putfield',
    0xb6: 'invokevirtual', 0xb7: 'invokespecial', 0xb8: 'invokestatic', 0xb9: 'invokeinterface',
    0xbb: 'new', 0xbc: 'newarray', 0xbd: 'anewarray', 0xbe: 'arraylength',
    0xbf: 'athrow', 0xc0: 'checkcast', 0xc1: 'instanceof', 0xc6: 'ifnull', 0xc7: 'ifnonnull',
}

# operand byte counts for the straight-line ops the walker sizes (the
# invoke ops carry their own branches below)
WIDE1 = set(range(0x02, 0x10)) | set(range(0x1a, 0x36)) | set(range(0x3b, 0x57)) \
    | {0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f} \
    | set(range(0x60, 0x85)) - {0x84} | set(range(0x85, 0x99)) | {0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xbe, 0xbf}


def parse_cp_and_code(jar_path, cls, want):
    z = zipfile.ZipFile(jar_path)
    d = z.read(f'{cls}.class')
    p = 0

    def u1():
        nonlocal p; v = d[p]; p += 1; return v

    def u2():
        nonlocal p; v = struct.unpack_from('>H', d, p)[0]; p += 2; return v

    def u4():
        nonlocal p; v = struct.unpack_from('>I', d, p)[0]; p += 4; return v

    magic = u4()
    assert magic == 0xCAFEBABE
    p += 4  # minor(u2) + major(u2) — the magic u4 already advanced
    cpc = u2()
    cp = [None] * cpc
    i = 1
    while i < cpc:
        tag = u1()
        if tag == 1:
            ln = u2(); cp[i] = ('Utf8', d[p:p+ln].decode('utf-8', 'replace')); p += ln
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, u2())
        elif tag == 15:
            cp[i] = (tag, u1(), u2())  # ref_kind + ref_index — the helpers advance p
        elif tag in (3, 4):
            cp[i] = (tag, struct.unpack_from('>I', d, p)[0]); p += 4
        elif tag in (5, 6):
            cp[i] = (tag, d[p:p+8]); p += 8; i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        else:
            raise SystemExit(f'cp tag {tag}')
        i += 1

    def utf8(i):
        return cp[i][1] if cp[i] and cp[i][0] == 'Utf8' else f'#{i}'

    def name_of(i):  # NameAndType
        if cp[i] and cp[i][0] in (9, 10, 11, 12, 17, 18):
            return f'{utf8(cp[i][1])}.{utf8(cp[i][2])}'
        return f'#{i}'

    def cls_of(i):
        if cp[i] and cp[i][0] == 7:
            return utf8(cp[i][1])
        return f'#{i}'

    p += 6  # access, this, super
    icc = u2(); p += icc * 2
    fc = u2()
    for _ in range(fc):
        p += 6
        ac = u2()
        for _ in range(ac):
            p += 2; alen = u4(); p += alen
    mcount = u2()
    for _ in range(mcount):
        macc, nidx, didx = u2(), u2(), u2()
        name, desc = utf8(nidx), utf8(didx)
        ac = u2()
        for _ in range(ac):
            aname = utf8(u2()); alen = u4(); adata = d[p:p+alen]; p += alen
            if aname != 'Code':
                continue
            if name not in want and want:
                continue
            clen = struct.unpack_from('>I', adata, 4)[0]
            code = adata[8:8+clen]
            print(f'=== {name}{desc} (len {clen}) ===')
            q = 0
            while q < len(code):
                op = code[q]
                nm = OPCODES.get(op, f'0x{op:02x}')
                line = f'{q:4d} {nm}'
                if op == 0x10:  # bipush
                    line += f' {struct.unpack_from(">b", code, q+1)[0]}'; q += 2
                elif op == 0x12:  # ldc — the constant the pool holds
                    idx = code[q+1]
                    ent = cp[idx] if idx < len(cp) else None
                    detail = ''
                    if ent and ent[0] == 4:
                        detail = f' = {struct.unpack(">f", struct.pack(">I", ent[1]))[0]}'
                    elif ent and ent[0] == 3:
                        detail = f' = {ent[1]}'
                    elif ent and ent[0] == 'Utf8':
                        detail = f' = "{ent[1]}"'
                    line += f' #{idx}{detail}'; q += 2
                elif op == 0x11:  # sipush
                    line += f' {struct.unpack_from(">h", code, q+1)[0]}'; q += 3
                elif op in (0x13, 0x14, 0xb2, 0xb4, 0xb5, 0xbb, 0xbd, 0xc0, 0xc1):
                    idx = struct.unpack_from('>H', code, q+1)[0]
                    ent = cp[idx] if idx < len(cp) else None
                    detail = ''
                    if ent and ent[0] in (9, 10, 11):
                        detail = f' {cls_of(ent[1])}.{name_of(idx)}'
                    elif ent and ent[0] in (3, 4):
                        detail = f' = {struct.unpack(">i" if ent[0] == 3 else ">f", struct.pack(">I", ent[1]))[0]}'
                    line += f' #{idx}{detail}'; q += 3
                elif op in (0x15, 0x36, 0xa9, 0x16, 0x17, 0x18, 0x19, 0x37, 0x38, 0x39, 0x3a):
                    line += f' {code[q+1]}'; q += 2
                elif op == 0x84:  # iinc
                    line += f' {code[q+1]} {struct.unpack_from(">b", code, q+2)[0]}'; q += 3
                elif 0x99 <= op <= 0xa8 or op in (0xc6, 0xc7):
                    off = struct.unpack_from('>h', code, q+1)[0]
                    line += f' -> {q + off}'; q += 3
                elif op == 0xaa:  # tableswitch
                    base = (q + 4) & ~3
                    dflt = struct.unpack_from('>i', code, base)[0]
                    lo = struct.unpack_from('>i', code, base+4)[0]
                    hi = struct.unpack_from('>i', code, base+8)[0]
                    line += f' lo={lo} hi={hi} default->{q + dflt}'
                    for k in range(hi - lo + 1):
                        t = struct.unpack_from('>i', code, base+12+k*4)[0]
                        line += f' | {lo+k}->{q + t}'
                    q = base + 12 + (hi - lo + 1) * 4
                    print(line)
                    continue
                elif op in (0xb6, 0xb7, 0xb8):
                    idx = struct.unpack_from('>H', code, q+1)[0]
                    ent = cp[idx]
                    line += f' {cls_of(ent[1])}.{name_of(idx)}'; q += 3
                elif op == 0xb9:
                    idx = struct.unpack_from('>H', code, q+1)[0]
                    ent = cp[idx]
                    line += f' {cls_of(ent[1])}.{name_of(idx)}'; q += 5
                elif op in WIDE1:
                    q += 1
                else:
                    q += 1
                print(line)


if __name__ == '__main__':
    parse_cp_and_code(sys.argv[1], sys.argv[2], set(sys.argv[3:]))
