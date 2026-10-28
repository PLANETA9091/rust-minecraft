#!/usr/bin/env python3
"""Scan a jar for classes whose constant pool references a given method.

Usage: python3 scripts/cp_ref_scan.py <jar> <net/minecraft/SomeClass> <methodName>
Prints every class name + (name descriptor) of Methodref/InterfaceMethodref
entries targeting Class.methodName.
"""
import struct, sys, zipfile
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from class_list import parse_cp, utf8


def class_cp_refs(data, target_cls, target_m):
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
    try:
        cp, n = parse_cp(r)
    except Exception:
        return None  # not a parseable classfile (module-info etc.)
    hits = []
    for i, e in enumerate(cp):
        if e and e[0] == 'Idx2':
            a, b = e[1], e[2]
            ea = cp[a] if a < len(cp) else None
            eb = cp[b] if b < len(cp) else None
            if ea and ea[0] == 'Idx' and eb and eb[0] == 'Idx2':
                cls_name = utf8(cp, ea[1])
                if cls_name.endswith(target_cls):
                    mname = utf8(cp, eb[1]); mdesc = utf8(cp, eb[2])
                    if mname == target_m:
                        hits.append((mname, mdesc))
    return hits


def main():
    jar, target_cls, target_m = sys.argv[1], sys.argv[2], sys.argv[3]
    z = zipfile.ZipFile(jar)
    for name in z.namelist():
        if not name.endswith('.class') or '$' in name.rsplit('/', 1)[-1]:
            continue
        data = z.read(name)
        hits = class_cp_refs(data, target_cls, target_m)
        if hits:
            cls = name[:-6]
            for mname, mdesc in hits:
                print(f'{cls} -> {target_cls}.{mname} {mdesc}')


if __name__ == '__main__':
    main()
