#!/usr/bin/env python3
"""Audit the 26.2 jar for the WIND_CHARGE word 1051 (autopush-462).

1) sipush scan over every class: find `sipush 1051` call sites
   (the levelEvent caller walk).
2) LevelEventHandler case 1051 body: dump the switch arm's bytecode
   (the law the client fold answers).
3) SoundEvents.<clinit> walk: the WIND_CHARGE_* fields' keys.
"""
import struct
import zipfile
import sys
from collections import defaultdict

JAR = "/home/z/my-project/client262.jar"

def parse_class(raw):
    off = 8
    (cp_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    pool = {}
    i = 1
    while i < cp_count:
        tag = raw[off]
        if tag == 1:
            (ln,) = struct.unpack_from(">H", raw, off + 1)
            pool[i] = ("utf8", raw[off + 3 : off + 3 + ln])
            off += 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            pool[i] = ("ref", struct.unpack_from(">H", raw, off + 1)[0])
            off += 3
        elif tag == 15:
            pool[i] = ("mh", None)
            off += 4
        elif tag in (3, 4):
            pool[i] = ("num", struct.unpack_from(">I", raw, off + 1)[0])
            off += 5
        elif tag in (5, 6):
            pool[i] = ("num8", None)
            off += 9
            i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            pool[i] = ("ref2", struct.unpack_from(">HH", raw, off + 1))
            off += 5
        else:
            raise ValueError(f"bad tag {tag} at {off}")
        i += 1
    (access, this_idx, super_idx) = struct.unpack_from(">HHH", raw, off)
    off += 6
    ifaces_n = struct.unpack_from(">H", raw, off)[0]
    off += 2 + 2 * ifaces_n
    for _ in range(2):  # fields, methods
        n = struct.unpack_from(">H", raw, off)[0]
        off += 2
        members = []
        for _ in range(n):
            (f_acc, f_name, f_desc, f_attr_n) = struct.unpack_from(">HHHH", raw, off)
            off += 8
            attrs = []
            for _ in range(f_attr_n):
                (a_name, a_len) = struct.unpack_from(">HI", raw, off)
                off += 6
                attrs.append((a_name, raw[off : off + a_len]))
                off += a_len
            members.append((f_acc, f_name, f_desc, attrs))
        if _ == 0:
            fields = members
        else:
            methods = members
    cls = {"pool": pool, "this": pool[this_idx][1] if this_idx in pool else "?", "fields": fields, "methods": methods}
    return cls

def utf8(pool, idx):
    if idx in pool and pool[idx][0] == "utf8":
        return pool[idx][1].decode("utf-8", "replace")
    return None

def scan_sipush(z, words):
    hits = defaultdict(list)
    for name in z.namelist():
        if not name.endswith(".class"):
            continue
        try:
            cls = parse_class(z.read(name))
        except Exception:
            continue
        for (acc, mname, mdesc, attrs) in cls["methods"]:
            for (an, ab) in attrs:
                if utf8(cls["pool"], an) != "Code":
                    continue
                b = ab
                # skip 6 header bytes (max_stack, max_locals u16 each) then code_len
                (code_len,) = struct.unpack_from(">I", b, 10)
                code = b[14 : 14 + code_len]
                i = 0
                while i < len(code) - 2:
                    if code[i] == 0x11:  # sipush
                        v = struct.unpack_from(">h", code, i + 1)[0]
                        if v in words:
                            hits[v].append(f"{cls['this']}.{mname}")
                    i += 1
    return hits

def main():
    z = zipfile.ZipFile(JAR)
    # 1) callers of 1051 (and 1032 context)
    hits = scan_sipush(z, {1051, 1032})
    for w in sorted(hits):
        print(f"== sipush {w} callers ==")
        for s in hits[w]:
            print("  ", s)
    if not hits:
        print("== no sipush hits for 1051/1032 ==")

if __name__ == "__main__":
    main()
