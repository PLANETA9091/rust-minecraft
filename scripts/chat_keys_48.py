#!/usr/bin/env python3
"""Audit the 26.2 jar for the CHAT-OPEN keys (dev-48).

Finds which net/minecraft/client/Minecraft methods reference the
Options.keyChat / keyCommand / keySocialInteractions / keyAdvancements
fields, and traces those methods' bytecode (the law the client's slash
lane answers). Uses the house walker (classfile_dump).
"""
import sys, zipfile

sys.path.insert(0, "scripts")
from classfile_dump import parse_class, read_code, trace_code, cp_ref

JAR = "/home/z/my-project/client262.jar"
TARGETS = ("keyChat", "keyCommand", "keySocialInteractions", "keyAdvancements")

with zipfile.ZipFile(JAR) as z:
    raw = z.read("net/minecraft/client/Minecraft.class")

cp, methods = parse_class(raw)

def key_refs(cp, code):
    names = set()
    for i, b in enumerate(code):
        if 0xB2 <= b <= 0xB5 and i + 2 < len(code):
            idx = (code[i + 1] << 8) | code[i + 2]
            if idx < len(cp) and cp[idx] and cp[idx][0] == "Ref":
                cls_name, nm, _desc = cp_ref(cp, idx)
                if cls_name.endswith("client/Options") and nm in TARGETS:
                    names.add(nm)
    return names

print("=== methods referencing chat-open keys ===")
hits = []
for (name, desc), attrs in sorted(methods.items()):
    got = read_code(attrs)
    if got is None:
        continue
    names = key_refs(cp, got[1])
    if names:
        hits.append((name, desc, sorted(names)))
        print(f"  {name}{desc} -> {sorted(names)}")

for (name, desc, names) in hits:
    if "keyCommand" not in names:
        continue
    got = read_code(methods[(name, desc)])
    print(f"\n=== {name}{desc} ===")
    for line in trace_code(cp, got[1]):
        print(line)
