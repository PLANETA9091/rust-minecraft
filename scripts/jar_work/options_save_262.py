#!/usr/bin/env python3
"""THE OPTIONS SAVE PROBE (autopush-168): the Options class's own file law —
the keys the OptionInstances wear (the constant pool's real strings), the
save() method the options.txt the vanilla writes (the process: which entries,
the key:value line shape), the load() read lane. The jar's own facts the
persistence seat banks — no memory, the constant pool speaks.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

ms = methods_of(z, "net.minecraft.client.Options")

# 1) THE KEYS: the <init> ldc_w strings near each putfield option field.
cp, body = next(b for s, b in ms.items() if s.startswith("<init>"))
clen = struct.unpack_from(">I", body, 4)[0]
text = list(walk_code(cp, body[8:8 + clen]))
print("=== <init>: ldc string -> putfield option (the real keys)")
for i, t in enumerate(text):
    if "putfield" in t and "OptionInstance" in t:
        window = " ~ ".join(text[max(0, i - 12):i + 1])
        # find the nearest ldc_w string in the window
        keys = [w for w in window.split(" ~ ") if w.strip().startswith("ldc")]
        print(t.split("putfield")[1].strip()[:70], " <- ", keys[-1] if keys else "?")

# 2) THE SAVE: the options.txt writer's own bytecode.
print()
print("=== save() methods")
for s, (cp2, body2) in ms.items():
    if "save" in s:
        print("--- method:", s)
        clen2 = struct.unpack_from(">I", body2, 4)[0]
        for t in walk_code(cp2, body2[8:8 + clen2]):
            print("   ", t)

# 3) THE LOAD: the reader's own shape (split on ':', the order).
print()
print("=== load() method head")
for s, (cp2, body2) in ms.items():
    if s.startswith("load"):
        clen2 = struct.unpack_from(">I", body2, 4)[0]
        lines = [str(t) for t in walk_code(cp2, body2[8:8 + clen2])]
        for t in lines[:60]:
            print("   ", t)
        print("    ... total", len(lines), "ops")
