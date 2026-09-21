#!/usr/bin/env python3
"""Probe BootstrapMethods recipes of a class in a jar (uses the repo's classfile_dump parser)."""
import sys, struct
sys.path.insert(0, '/home/z/my-project/rust-minecraft/scripts')
from classfile_dump import parse_class, parse_constant_pool, Reader, cp_utf8

jar_path, cls = sys.argv[1], sys.argv[2]
import zipfile
d = zipfile.ZipFile(jar_path).read(f"{cls}.class")
r = Reader(d)
assert r.u4() == 0xCAFEBABE
r.u2(); r.u2()
cp = parse_constant_pool(r)
r.u2(); r.u2(); r.u2()          # access/this/super
for _ in range(r.u2()): r.u2()  # interfaces
for section in ('fields', 'methods'):
    for _ in range(r.u2()):
        r.u2(); r.u2(); r.u2()
        for _ in range(r.u2()):
            aname = cp_utf8(cp, r.u2()); alen = r.u4(); adata = r.raw(alen)
# class attributes
for _ in range(r.u2()):
    aname = cp_utf8(cp, r.u2()); alen = r.u4(); adata = r.raw(alen)
    if aname == 'BootstrapMethods':
        rr = Reader(adata)
        n = rr.u2()
        for bm in range(n):
            mh = rr.u2(); argc = rr.u2()
            args = [rr.u2() for _ in range(argc)]
            resolved = [cp_utf8(cp, a) if cp[a][0] == 1 else f"#{a}({cp[a][0]})" for a in args]
            print(f"bootstrap[{bm}] handle=#{mh} -> {resolved}")
