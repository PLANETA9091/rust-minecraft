#!/usr/bin/env python3
"""THE MOUSE ROWS' OWN CTOR WALK (autopush-227): the Options ctor the
six mouse registrations the pc ranges answer — the label/inputs/
caption lambdas the OptionInstance ctors keep.
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

d = dict(methods_of(z, "net.minecraft.client.Options"))
sig, (cp, body) = next(
    (s, b) for s, b in d.items() if s.startswith("<init>(")
)
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
lines = list(walk_code(cp, code))

NEEDLES = [
    "options.mouseWheelSensitivity",
    "options.discreteMouseScroll",
    "options.invertMouseX",
    "options.invertMouseY",
    "options.allowCursorChanges",
    "options.rawMouseInput",
    "mouseWheelSensitivity",
    "discreteMouseScroll",
    "invertMouseX",
    "invertMouseY",
    "allowCursorChanges",
    "rawMouseInput",
]
for i, t in enumerate(lines):
    if any(n in str(t) for n in NEEDLES):
        lo = max(0, i - 14)
        hi = min(len(lines), i + 22)
        print(f"--- ctx around line {i} (pc {lines[i].split(':')[0] if isinstance(lines[i], str) else '?'})")
        for j in range(lo, hi):
            print(f"  {lines[j]}")
        print()
