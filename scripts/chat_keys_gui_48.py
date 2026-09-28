#!/usr/bin/env python3
"""Trace Gui's chat-open key lane in the 26.2 jar (dev-48).

Finds the net/minecraft/client/gui/Gui methods reading keyChat/keyCommand,
prints the consumeClick walk + the chat() call shape the client folds.
"""
import sys, zipfile

sys.path.insert(0, "scripts")
from classfile_dump import parse_class, read_code, trace_code, cp_ref

JAR = "/home/z/my-project/client262.jar"
TARGETS = ("keyChat", "keyCommand", "keySocialInteractions", "keyAdvancements")

with zipfile.ZipFile(JAR) as z:
    raw = z.read("net/minecraft/client/gui/Gui.class")

cp, methods = parse_class(raw)

def key_refs(cp, code):
    names = set()
    for i, b in enumerate(code):
        if 0xB2 <= b <= 0xB5 and i + 2 < len(code):
            idx = (code[i + 1] << 8) | code[i + 2]
            if idx < len(cp) and cp[idx] and cp[idx][0] == "Ref":
                cls_name, nm, _desc = cp_ref(cp, idx)
                if nm in TARGETS:
                    names.add((cls_name, nm))
    return names

print("=== Gui methods referencing chat-open keys ===")
for (name, desc), attrs in sorted(methods.items()):
    got = read_code(attrs)
    if got is None:
        continue
    names = key_refs(cp, got[1])
    if names:
        print(f"  {name}{desc} -> {sorted(n for _, n in names)}")

for (name, desc), attrs in sorted(methods.items()):
    got = read_code(attrs)
    if got is None:
        continue
    names = key_refs(cp, got[1])
    if any(nm in ("keyChat", "keyCommand") for _, nm in names):
        print(f"\n=== {name}{desc} ===")
        for line in trace_code(cp, got[1]):
            print(line)
