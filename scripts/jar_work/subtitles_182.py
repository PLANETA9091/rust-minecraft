#!/usr/bin/env python3
"""THE SUBTITLE OVERLAY'S OWN WALK PROBE (autopush-182): the
`SubtitleOverlay` class the extractRenderState lane serves — the 0.8F
band the getBackgroundColor(F)I overload wears (the 180 record banks),
the 9px rows the list layout keeps, the queue the sound play feeds.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

names = [n for n in z.namelist() if "Subtitle" in n and n.endswith(".class")]
print("== classes ==", names)

for cls in [n[:-6].replace("/", ".") for n in names]:
    got = core["method_bodies"](z, cls)
    if got is None:
        continue
    cp, bs = got
    for (nm, desc), b in sorted(bs.items()):
        if b is None:
            continue
        clen = struct.unpack_from(">I", b, 4)[0]
        code = b[8:8 + clen]
        try:
            ls = core["disasm"](cp, code)
        except Exception as e:
            print(f"!! {cls}::{nm}{desc}: {e!r}")
            continue
        joined = [l for _, l in ls]
        if any("0.8" in l or "getBackgroundColor" in l or "renderBackground" in l
               for l in joined) or nm == "<init>":
            print(f"== {cls}::{nm} {desc} ==")
            for pc, l in ls:
                print(f"   {pc}: {l}")
            print()
