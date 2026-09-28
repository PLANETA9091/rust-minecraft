#!/usr/bin/env python3
"""Scan the 26.2 jar for sipush <word> call sites (autopush-461 band).

Targets: the ONE-DRAW PITCH voice words 1029/1030/1031/1039/1042/
1043/1044/1045/1046/1047/1052 — the caller walk the server's own
classes speak (`levelEvent(<N>, pos, 0)` tails). Linear bytecode pass
over every class: sipush (0x11) operands matched, the enclosing
class/method recorded.
"""
import struct
import zipfile
import sys
from collections import defaultdict

JAR = "/home/z/my-project/client262.jar"
WORDS = {1051, 1032}  # the handoff's stragglers too (1048/1025 context)

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
            pool[i] = ("num", None)
            off += 5
        elif tag in (5, 6):
            pool[i] = ("num8", None)
            off += 9
            i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            pool[i] = ("ref2", struct.unpack_from(">HH", raw, off + 1))
            off += 5
        else:
            raise SystemExit(f"tag {tag}")
        i += 1
    def utf8(idx):
        kind, val = pool[idx]
        if kind == "ref":  # Class entry -> Utf8 index hop
            return pool[val][1].decode("utf-8", "replace")
        return val.decode("utf-8", "replace")
    (access, this_c, super_c) = struct.unpack_from(">HHH", raw, off)
    off += 6
    cls_name = utf8(this_c).replace("/", ".")
    (ifc_count,) = struct.unpack_from(">H", raw, off)
    off += 2 + 2 * ifc_count
    (fld_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    for _ in range(fld_count):
        off += 6
        (attr_count,) = struct.unpack_from(">H", raw, off)
        off += 2
        for _ in range(attr_count):
            (an, alen) = struct.unpack_from(">HI", raw, off)
            off += 6 + alen
    (mth_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    out = []
    for _ in range(mth_count):
        m_acc, m_name, m_desc = struct.unpack_from(">HHH", raw, off)
        off += 6
        (attr_count,) = struct.unpack_from(">H", raw, off)
        off += 2
        code = None
        for _ in range(attr_count):
            (an, alen) = struct.unpack_from(">HI", raw, off)
            if utf8(an) == "Code":
                code = raw[off + 6 : off + 6 + alen]
            off += 6 + alen
        out.append((utf8(m_name), utf8(m_desc), code))
    return cls_name, out

def sipush_ops(code):
    (max_stack, max_locals, codelen) = struct.unpack_from(">HHI", code, 0)
    bc = code[8 : 8 + codelen]
    end = min(len(bc), 8 + codelen) if False else len(bc)
    i = 0
    ops = []
    while i + 3 <= len(bc):  # sipush needs 3 bytes; guard the tail
        op = bc[i]
        if op == 0x11:  # sipush
            ops.append(struct.unpack_from(">h", bc, i + 1)[0])
            i += 3
        elif op == 0xC4:  # wide
            nxt = bc[i + 1]
            i += 6 if nxt == 0x84 else 4
        else:
            i += 1
    return ops

def main():
    hits = defaultdict(list)
    with zipfile.ZipFile(JAR) as z:
        names = [n for n in z.namelist() if n.endswith(".class") and "$" not in n]
        for n in names:
            try:
                raw = z.read(n)
                cls, methods = parse_class(raw)
            except SystemExit:
                continue
            for mname, mdesc, code in methods:
                if code is None:
                    continue
                for w in sipush_ops(code):
                    if w in WORDS:
                        hits[w].append(f"{cls}.{mname}{mdesc}")
    for w in sorted(WORDS):
        rows = hits.get(w, [])
        print(f"=== sipush {w}: {len(rows)} site(s)")
        for r in rows[:8]:
            print(f"    {r}")
        if len(rows) > 8:
            print(f"    ... +{len(rows) - 8} more")

if __name__ == "__main__":
    sys.exit(main())
