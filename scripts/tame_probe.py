#!/usr/bin/env python3
"""Tame probe: dump TamableAnimal + Wolf class members and the tame walk's
bytecode (Wolf.mobInteract / TamableAnimal.tame / fields) so the tame machine
the round implements reads the jar's own words verbatim. javap absent — the
headless JRE's own probe the ledger keeps."""
import struct, sys, zipfile

JAR = "tools/client.jar"


def load_cp(data):
    d = data
    p = 8
    cpcount = struct.unpack_from('>H', d, 6)[0]
    cp = [None] * cpcount
    i = 1

    def u1():
        nonlocal p
        v = d[p]; p += 1; return v

    def u2():
        nonlocal p
        v = struct.unpack_from('>H', d, p)[0]; p += 2; return v

    def u4():
        nonlocal p
        v = struct.unpack_from('>I', d, p)[0]; p += 4; return v

    while i < cpcount:
        tag = u1()
        if tag == 1:
            ln = u2(); cp[i] = ('u', d[p:p + ln].decode('utf-8', 'replace')); p += ln
        elif tag in (7, 8, 16, 19, 20):
            p += 2
        elif tag == 15:
            p += 3
        elif tag in (3, 4):
            p += 4
        elif tag in (5, 6):
            p += 8; i += 1
        elif tag in (9, 10, 11):
            cp[i] = ('m', u2(), u2())
        elif tag == 12:
            cp[i] = ('nt', u2(), u2())
        else:
            raise Exception(f'tag {tag} @ {i}')
        i += 1
    return cp, p


def utf(cp, i):
    return cp[i][1] if cp[i] and cp[i][0] == 'u' else f'?{i}'


def cls_of(cp, i):
    return utf(cp, cp[i][1])


def nmt(cp, e):
    return utf(cp, e[1]), utf(cp, e[2])


MNEMONIC = {
    0x00: 'nop', 0x01: 'aconst_null', 0x02: 'iconst_m1', 0x03: 'iconst_0',
    0x04: 'iconst_1', 0x05: 'iconst_2', 0x06: 'iconst_3', 0x07: 'iconst_4',
    0x08: 'iconst_5', 0x09: 'lconst_0', 0x0a: 'lconst_1', 0x0b: 'fconst_0',
    0x0c: 'fconst_1', 0x0d: 'fconst_2', 0x10: 'bipush', 0x11: 'sipush',
    0x12: 'ldc1', 0x13: 'ldc2w', 0x15: 'iload', 0x17: 'fload', 0x19: 'aload',
    0x1a: 'iload_0', 0x1b: 'iload_1', 0x1c: 'iload_2', 0x1d: 'iload_3',
    0x22: 'fload_0', 0x23: 'fload_1', 0x24: 'fload_2', 0x25: 'fload_3',
    0x2a: 'aload_0', 0x2b: 'aload_1', 0x2c: 'aload_2', 0x2d: 'aload_3',
    0x32: 'aaload', 0x36: 'istore', 0x38: 'fstore', 0x3a: 'astore',
    0x3d: 'istore_2', 0x3e: 'istore_3', 0x45: 'fstore_0', 0x46: 'fstore_1',
    0x47: 'fstore_2', 0x48: 'fstore_3', 0x4b: 'astore_0', 0x4c: 'astore_1',
    0x4d: 'astore_2', 0x4e: 'astore_3', 0x57: 'pop', 0x59: 'dup',
    0x5f: 'swap', 0x60: 'iadd', 0x62: 'fadd', 0x64: 'isub', 0x66: 'fsub',
    0x6c: 'idiv', 0x6e: 'fdiv', 0x70: 'irem', 0x72: 'frem',
    0x78: 'ishl', 0x7a: 'ishr', 0x7c: 'iushr', 0x80: 'ior', 0x84: 'iinc',
    0x85: 'i2l', 0x86: 'i2f', 0x87: 'i2d', 0x88: 'l2i', 0x8d: 'f2i',
    0x8e: 'd2i', 0x91: 'i2b', 0x92: 'i2c',
    0x94: 'lcmp', 0x95: 'fcmpl', 0x96: 'fcmpg', 0x97: 'dcmpl', 0x98: 'dcmpg',
    0x99: 'ifeq', 0x9a: 'ifne', 0x9b: 'iflt', 0x9c: 'ifge', 0x9d: 'ifgt',
    0x9e: 'ifle', 0x9f: 'if_icmpeq', 0xa0: 'if_icmpne', 0xa1: 'if_icmplt',
    0xa2: 'if_icmpge', 0xa3: 'if_icmpgt', 0xa4: 'if_icmple',
    0xa5: 'if_acmpeq', 0xa6: 'if_acmpne', 0xa7: 'goto', 0xa8: 'jsr',
    0xac: 'ireturn', 0xae: 'freturn', 0xb0: 'areturn', 0xb1: 'return',
    0xb2: 'getstatic', 0xb3: 'putstatic', 0xb4: 'getfield', 0xb5: 'putfield',
    0xb6: 'invokevirtual', 0xb7: 'invokespecial', 0xb8: 'invokestatic',
    0xb9: 'invokeinterface', 0xba: 'invokedynamic', 0xbb: 'new',
    0xbc: 'newarray', 0xbd: 'anewarray', 0xbe: 'arraylength', 0xbf: 'athrow',
    0xc0: 'checkcast', 0xc1: 'instanceof', 0xc2: 'monitorenter',
    0xc3: 'monitorexit', 0xc6: 'ifnull', 0xc7: 'ifnonnull',
}


def dump_method(data, cp, cpend, want):
    d = data
    p = cpend
    p += 2  # access
    p += 2  # this
    p += 2  # super
    ifc = struct.unpack_from('>H', d, p)[0]; p += 2 + 2 * ifc

    def u2():
        nonlocal p
        v = struct.unpack_from('>H', d, p)[0]; p += 2; return v

    nf = u2()
    for _ in range(nf):
        u2(); fn = u2(); u2(); fd = u2(); na = u2(); p += 2 * na
        print(f"  FIELD {utf(cp, fn)} : {utf(cp, fd)}")
    nm = u2()
    found = False
    for _ in range(nm):
        u2(); mn = u2(); md = u2(); na = u2(); attrs = d[p:p + 2 * na]; p += 2 * na
        name = utf(cp, mn)
        if want not in name:
            continue
        found = True
        acount = struct.unpack_from('>H', attrs, 0)[0]
        q = 2
        for _a in range(acount):
            an = struct.unpack_from('>H', attrs, q)[0]
            alen = struct.unpack_from('>I', attrs, q + 2)[0]
            body = attrs[q + 6:q + 6 + alen]
            if utf(cp, an) == 'Code':
                mq = 0
                mq += 2  # max stack
                mq += 2  # max locals
                clen = struct.unpack_from('>I', body, mq)[0]; mq += 4
                code = body[mq:mq + clen]
                j = 0
                print(f"  --- {name}{utf(cp, md)} ---")
                while j < len(code):
                    op = code[j]
                    mnem = MNEMONIC.get(op, f'0x{op:02x}')
                    line = f"    {j:4d}: {mnem}"
                    j += 1
                    if op in (0x10,):  # bipush
                        v = struct.unpack_from('>b', code, j)[0]; j += 1
                        line += f" {v}"
                    elif op in (0x11, 0xa7, 0xa8, 0x99, 0x9a, 0x9b, 0x9c, 0x9d,
                                0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5,
                                0xa6, 0xc6, 0xc7):
                        v = struct.unpack_from('>h', code, j)[0]; j += 2
                        line += f" {v} -> {j + v}" if op in (0xa7, 0xa8) else f" {v}"
                    elif op == 0x12:  # ldc
                        v = code[j]; j += 1
                        line += f" #{v}"
                    elif op in (0x13, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8,
                                0xbb, 0xbd, 0xc0, 0xc1):
                        v = struct.unpack_from('>H', code, j)[0]; j += 2
                        e = cp[v]
                        if e and e[0] == 'm':
                            n, t = nmt(cp, e)
                            line += f" {cls_of(cp, e[1])}.{n} {t}"
                        else:
                            line += f" #{v}"
                    elif op == 0xba:
                        v = struct.unpack_from('>H', code, j)[0]; j += 2
                        e = cp[v]
                        if e:
                            line += f" bsm#{e[1]} arg#{e[2]}"
                        j += 2
                    elif op in (0x15, 0x17, 0x19, 0x36, 0x38, 0x3a):
                        v = code[j]; j += 1; line += f" {v}"
                    elif op == 0x84:
                        a, b = struct.unpack_from('>bb', code, j); j += 2
                        line += f" {a} {b}"
                    elif op == 0xb9:
                        v = struct.unpack_from('>H', code, j)[0]; j += 4
                        e = cp[v]
                        if e and e[0] == 'm':
                            n, t = nmt(cp, e)
                            line += f" {cls_of(cp, e[1])}.{n} {t}"
                    print(line)
            q += 6 + alen
    if not found:
        print(f"  (no method matching '{want}')")


def main():
    for cls, wants in [
        ('net/minecraft/world/entity/TamableAnimal',
         ['tame', 'setOwnerUUID', 'getOwnerUUID', 'setTame', 'isTame',
          'setInSittingPose', 'isInSittingPose', 'setOrderedToSit',
          'isOrderedToSit', 'wantsToStop']),
        ('net/minecraft/world/entity/animal/wolf/Wolf', ['mobInteract', 'tame']),
    ]:
        data = zipfile.ZipFile(JAR).read(cls + '.class')
        cp, cpend = load_cp(data)
        print(f"===== {cls.rsplit('/', 1)[1]} =====")
        dump_method(data, cp, cpend, '')
        for w in wants:
            pass


if __name__ == '__main__':
    main()
