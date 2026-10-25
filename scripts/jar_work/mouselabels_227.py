#!/usr/bin/env python3
"""THE MOUSE ROWS' OWN LABELS (autopush-227): the Options ctor the
mouse registrations the cp strings answer + the logMouse law the
wheel row's own scale speaks.
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

# resolve the ldc_w operands near the mouse registrations (pc 1600-1920)
lines = list(walk_code(cp, code))
import re
for t in lines:
    m = re.match(r"\s*(\d+): ldc_w ldcw@(\d+)", t)
    if not m:
        continue
    pc, idx = int(m.group(1)), int(m.group(2))
    if 1580 <= pc <= 1930:
        try:
            s = utf(cp, idx)
        except Exception as e:
            s = f"<err {e}>"
        print(f"pc {pc}: {s!r}")

print()
# the logMouse law
for msig, (mcp, mbody) in sorted(d.items()):
    if "logMouse" not in msig:
        continue
    mclen = struct.unpack_from(">I", mbody, 4)[0]
    mcode = mbody[8 : 8 + mclen]
    print(f"=== Options.{msig}")
    for t in walk_code(mcp, mcode):
        print(f"  {t}")
