#!/usr/bin/env python3
"""autopush-126 recon — the DiodeBlock.tick branch targets.

The fold-trace the classfile_dump.py walks prints the invokes but the
branch opcodes the tick's three arms answer never speak. The probe
extracts the tick method's raw code and prints every branch's jump
target: the (flag, flag1) pair the three arms (off-flip / on-flip /
reschedule) the steady-state law needs.
"""
import struct
import sys
import zipfile

MNEM = {
    0x99: 'ifeq', 0x9A: 'ifne', 0x9B: 'iflt', 0x9C: 'ifge', 0x9D: 'ifgt',
    0x9E: 'ifle', 0x9F: 'if_icmpeq', 0xA0: 'if_icmpne', 0xA1: 'if_icmplt',
    0xA2: 'if_icmpge', 0xA3: 'if_icmpgt', 0xA4: 'if_icmple',
    0xA5: 'if_acmpeq', 0xA6: 'if_acmpne', 0xA7: 'goto',
    0xC6: 'ifnull', 0xC7: 'ifnonnull',
}
TWO = set(range(0x15, 0x1A)) | set(range(0x36, 0x3B)) | {0x10, 0x84, 0xBC, 0xA9}
THREE = (
    set(range(0xB2, 0xB9)) | {0x11, 0x13, 0x14, 0xBB, 0xBD, 0xC0, 0xC1}
)
FIVE = {0xB9, 0xBA}


class R:
    def __init__(s, d):
        s.d = d
        s.p = 0

    def u1(s):
        v = s.d[s.p]
        s.p += 1
        return v

    def u2(s):
        v = struct.unpack_from('>H', s.d, s.p)[0]
        s.p += 2
        return v

    def u4(s):
        v = struct.unpack_from('>I', s.d, s.p)[0]
        s.p += 4
        return v

    def u8(s):
        v = struct.unpack_from('>Q', s.d, s.p)[0]
        s.p += 8
        return v

    def raw(s, n):
        v = s.d[s.p:s.p + n]
        s.p += n
        return v


def main() -> None:
    jar = zipfile.ZipFile(sys.argv[1])
    cls = sys.argv[2]
    want = sys.argv[3] if len(sys.argv) > 3 else 'tick'
    r = R(jar.read(f'{cls}.class'))
    assert r.u4() == 0xCAFEBABE
    r.u2()
    r.u2()
    n = r.u2()
    cp = [None] * n
    i = 1
    while i < n:
        tag = r.u1()
        if tag == 1:
            cp[i] = r.raw(r.u2()).decode('utf8', 'replace')
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = ('r', r.u2())
        elif tag == 15:
            cp[i] = ('m', r.u1(), r.u2())
        elif tag in (3, 4):
            cp[i] = ('n', r.u4())
        elif tag in (5, 6):
            cp[i] = ('n', r.u8())
            i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = ('rr', r.u2(), r.u2())
        else:
            raise Exception(f'bad cp tag {tag}')
        i += 1
    utf = lambda i: cp[i] if isinstance(cp[i], str) else '?'
    r.u2()
    r.u2()
    r.u2()
    for _ in range(r.u2()):
        r.u2()
    for _ in range(r.u2()):
        r.u2()
        r.u2()
        r.u2()
        for _ in range(r.u2()):
            r.u2()
            r.raw(r.u4())
    for _ in range(r.u2()):
        r.u2()
        mname = utf(r.u2())
        r.u2()
        for _ in range(r.u2()):
            an = utf(r.u2())
            ln = r.u4()
            if an == 'Code' and mname == want:
                body = r.raw(ln)
                br = R(body)
                br.u2()
                br.u2()
                clen = br.u4()
                code = br.raw(clen)
                p = 0
                while p < len(code):
                    op = code[p]
                    if op in MNEM:
                        tgt = struct.unpack_from('>h', code, p + 1)[0]
                        print(f'{p:5d} {MNEM[op]} -> {p + tgt}')
                        p += 3
                    elif op == 0xC4:
                        w = code[p + 1]
                        p += 7 if w == 0x11 else 6
                    elif op == 0xC5:
                        p += 4
                    elif op in FIVE:
                        p += 5
                    elif op in THREE:
                        p += 3
                    elif op in TWO:
                        p += 2
                    else:
                        p += 1
                return
            r.raw(ln)


if __name__ == '__main__':
    main()
