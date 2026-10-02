#!/usr/bin/env python3
"""Jar-wide caller search: find every class whose constant pool holds a
Methodref/InterfaceMethodref to <target class>.<method name> (any descriptor).

Usage: python3 scripts/callers_480.py <jar> <target/class/Name> <methodName> [<methodName2> ...]
"""
import sys
import zipfile

sys.path.insert(0, "/home/z/my-project/rust-minecraft/scripts")
from full_disasm import parse_cp, utf8  # noqa: E402


def scan(jar, target_cls, names):
    zf = zipfile.ZipFile(jar)
    hits = []
    for info in zf.infolist():
        if not info.filename.endswith(".class"):
            continue
        data = zf.read(info.filename)
        try:
            cp, _ = parse_cp(data)
        except Exception:
            continue
        for idx, e in cp.items():
            if not isinstance(e, tuple) or e[0] not in ("method", "iface"):
                continue
            cls_idx, nat_idx = e[1][0], e[1][1]
            cls_entry = cp.get(cls_idx)
            nat_entry = cp.get(nat_idx)
            if not cls_entry or cls_entry[0] != 7:
                continue
            if not nat_entry or nat_entry[0] != "name_type":
                continue
            try:
                cls_name = utf8(cp, cls_entry[1])
                mname = utf8(cp, nat_entry[1][0])
                mdesc = utf8(cp, nat_entry[1][1])
            except Exception:
                continue
            if cls_name == target_cls and mname in names:
                hits.append((info.filename, mname, mdesc))
    return hits


def main():
    jar, target = sys.argv[1], sys.argv[2]
    names = set(sys.argv[3:])
    for fname, mname, mdesc in scan(jar, target, names):
        print(f"{fname}  ->  {mname} {mdesc}")


if __name__ == "__main__":
    main()
