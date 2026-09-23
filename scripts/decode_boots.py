#!/usr/bin/env python3
"""Decode the BootstrapMethods attribute of one class in a jar.

Usage: python3 scripts/decode_boots.py <jar> <net/minecraft/Class>
Reuses classfile_dump.py's proven constant-pool walker; prints every
bootstrap method (method handle resolved) + static argument refs, so an
invokedynamic's lambda implementation method can be named.
"""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from classfile_dump import Reader, parse_class, cp_utf8, cp_class_name, cp_nat, cp_ref  # noqa: E402


def main():
    import zipfile as zf
    jar = zf.ZipFile(sys.argv[1])
    cls = sys.argv[2]
    if not cls.endswith('.class'):
        cls += '.class'
    data = jar.read(cls)
    import classfile_dump as cfd
    r2 = cfd.Reader(data)
    assert r2.u4() == 0xCAFEBABE
    r2.u2(); r2.u2()
    cp = cfd.parse_constant_pool(r2)
    r2.u2(); r2.u2(); r2.u2()  # access, this, super
    for _ in range(r2.u2()):
        r2.u2()  # interfaces
    for section in ('fields', 'methods'):
        cnt = r2.u2()
        for _ in range(cnt):
            r2.u2(); r2.u2(); r2.u2()  # access, name, desc
            acnt = r2.u2()
            for _ in range(acnt):
                r2.u2()
                alen = r2.u4()
                r2.raw(alen)
    boots = None
    acnt = r2.u2()
    for _ in range(acnt):
        aname = cp_utf8(cp, r2.u2())
        alen = r2.u4()
        adata = r2.raw(alen)
        if aname == 'BootstrapMethods':
            boots = adata
    if boots is None:
        print('no BootstrapMethods attribute')
        return
    def resolve_mh(idx):
        e = cp[idx]
        if e[0] == 'MethodHandle':
            kind, ref_idx = e[1], e[2]
            return 'kind%d %s' % (kind, cp_ref(cp, ref_idx))
        return '%r' % (e,)
    n = struct.unpack_from('>H', boots, 0)[0]
    q = 2
    for bi in range(n):
        mh_idx = struct.unpack_from('>H', boots, q)[0]; q += 2
        argc = struct.unpack_from('>H', boots, q)[0]; q += 2
        print('bootstrap#%d: %s' % (bi, resolve_mh(mh_idx)))
        for _ in range(argc):
            ai = struct.unpack_from('>H', boots, q)[0]; q += 2
            e = cp[ai]
            kind = e[0]
            if kind == 'Ref':
                print('    arg member: %s' % (cp_ref(cp, ai),))
            elif kind == 'Class':
                print('    arg class: %s' % cp_class_name(cp, ai))
            elif kind in ('Integer', 'Float', 'Long', 'Double'):
                print('    arg const: %r' % (e[1],))
            elif kind == 'String':
                print('    arg string: %s' % cp_utf8(cp, e[1]))
            elif kind == 'MethodType':
                print('    arg methodtype: %s' % cp_utf8(cp, e[1]))
            elif kind == 'MethodHandle':
                print('    arg methodhandle: %s' % resolve_mh(ai))
            else:
                print('    arg %s: %r' % (kind, e[1:]))


if __name__ == '__main__':
    main()
