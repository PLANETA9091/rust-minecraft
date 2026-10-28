#!/usr/bin/env python3
"""Robust classfile method lister (the raw_disasm.py cp_all header-skip law).

Usage: python3 scripts/class_list.py <jar> <net/minecraft/Class>
Prints every method name + descriptor (and CODE length when present).

THE DESYNC THE autopush-302 ROUND BANKS: the raw_disasm.cp_all walk read the
cp count from offset 0 — the magic 0xCAFEBABE the 0xCAFE count wore, the walk
the opcode bytes the bogus tags hit (tag 186 = 0xBA the invokedynamic OPCODE
the misaligned read answers). The header the 8 bytes (magic + minor + major)
the count precedes — the skip the both cp walkers (cp_all + parse_class) wear.
"""
import struct, sys, zipfile

TAG_SIZES = {1: None, 3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4, 11: 4,
             12: 4, 15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2}


def parse_cp(r):
    """Robust constant-pool reader: skips the classfile header itself,
    returns (cp, count). Unknown tags raise with the byte offset named."""
    magic = r.u4()
    if magic != 0xCAFEBABE:
        raise ValueError(f'not a classfile: magic 0x{magic:08x} at 0')
    r.u2()  # minor
    r.u2()  # major
    n = r.u2()
    cp = [None] * n
    i = 1
    while i < n:
        off = r.p
        tag = r.u1()
        if tag == 1:
            ln = r.u2()
            cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
        elif tag in TAG_SIZES:
            sz = TAG_SIZES[tag]
            raw = r.raw(sz)
            if tag == 5:
                cp[i] = ('Long', struct.unpack_from('>q', raw)[0]); i += 1
            elif tag == 6:
                cp[i] = ('Double', struct.unpack_from('>d', raw)[0]); i += 1
            elif tag == 3:
                cp[i] = ('Int', struct.unpack_from('>i', raw)[0])
            elif tag == 4:
                cp[i] = ('Float', struct.unpack_from('>f', raw)[0])
            elif tag == 15:
                kind, idx = struct.unpack_from('>BH', raw)
                cp[i] = ('MethodHandle', kind, idx)
            elif tag in (7, 8, 16, 19, 20):
                cp[i] = ('Idx', struct.unpack_from('>H', raw)[0])
            else:
                a, b = struct.unpack_from('>HH', raw)
                cp[i] = ('Idx2', a, b)
        else:
            raise ValueError(f'unknown cp tag {tag} (0x{tag:02x}) at byte offset {off}')
        i += 1
    return cp, n


def utf8(cp, i):
    e = cp[i] if i < len(cp) else None
    return e[1] if e and e[0] == 'Utf8' else '?'


def main():
    jar, cls = sys.argv[1], sys.argv[2]
    data = zipfile.ZipFile(jar).read(f'{cls}.class')

    class R:
        def __init__(self, d):
            self.d = d; self.p = 0
        def u1(self):
            v = self.d[self.p]; self.p += 1; return v
        def u2(self):
            v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v
        def u4(self):
            v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v
        def raw(self, n):
            v = self.d[self.p:self.p+n]; self.p += n; return v

    r = R(data)
    cp, n = parse_cp(r)
    flags = r.u2(); this_cls = r.u2(); super_cls = r.u2()
    print(f'class {utf8(cp, this_cls)} extends {utf8(cp, super_cls)} (cp {n-1} entries)')
    nif = r.u2()
    for _ in range(nif):
        r.u2()
    nf = r.u2()
    for _ in range(nf):
        fi = r.u2(); ni = r.u2(); di = r.u2()
        print(f'  field {utf8(cp, ni)} {utf8(cp, di)}')
        na = r.u2()
        for _ in range(na):
            r.u2(); alen = r.u4(); r.raw(alen)
    nm = r.u2()
    for _ in range(nm):
        r.u2(); ni = r.u2(); di = r.u2()
        na = r.u2()
        clen = ''
        for _ in range(na):
            an_i = r.u2(); alen = r.u4(); adata = r.raw(alen)
            if utf8(cp, an_i) == 'Code':
                clen = f' code_len={struct.unpack_from(">I", adata, 4)[0]}'
        print(f'  method {utf8(cp, ni)} {utf8(cp, di)}{clen}')


if __name__ == '__main__':
    main()
