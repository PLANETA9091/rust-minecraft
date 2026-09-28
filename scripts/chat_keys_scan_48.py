#!/usr/bin/env python3
"""Scan the 26.2 jar for the chat-open key fields (dev-48).

Pass 1: every class whose constant pool mentions keyChat / keyCommand /
keySocialInteractions / keyAdvancements (Utf8), with the class the Fieldref
names as owner. Pass 2: for reader classes, trace the reading methods.
"""
import sys, zipfile

sys.path.insert(0, "scripts")
from classfile_dump import Reader, parse_constant_pool, cp_utf8

JAR = "/home/z/my-project/client262.jar"
NEEDLES = ("keyChat", "keyCommand", "keySocialInteractions", "keyAdvancements")

owners = {}
readers = {}

with zipfile.ZipFile(JAR) as z:
    names = [n for n in z.namelist() if n.endswith(".class")]
    for n in names:
        try:
            raw = z.read(n)
        except Exception:
            continue
        try:
            r = Reader(raw[8:])
            cp = parse_constant_pool(r)
        except Exception:
            continue
        cls = n[:-6]
        for i, entry in enumerate(cp):
            if entry is None:
                continue
            if entry[0] == "Utf8" and entry[1] in NEEDLES:
                owners.setdefault(entry[1], set()).add(cls)
            if entry[0] == "Ref" and len(entry) >= 4:
                # Fieldref: ('Ref', tag, class_idx, nat_idx)
                try:
                    cls_name = cp_utf8(cp, entry[2])
                    nat = cp[entry[3]]
                    nm = cp_utf8(cp, nat[1])
                except Exception:
                    continue
                if nm in NEEDLES:
                    readers.setdefault(nm, set()).add((cls, cls_name))

for k in NEEDLES:
    print(f"== {k} ==")
    print("  field-owner classes (mention):", sorted(owners.get(k, []))[:6])
    rd = sorted(readers.get(k, []))
    print(f"  field refs: {len(rd)}")
    for ref_cls, owner_cls in rd[:12]:
        print(f"    {ref_cls}  (owner {owner_cls})")
