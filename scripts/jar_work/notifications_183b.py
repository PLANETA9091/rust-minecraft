#!/usr/bin/env python3
"""THE NOTIFICATION LAMBDAS' OWN WALK v2 (autopush-183b): the cp#950
key utf8 the registration wears, the cp#240/#984 visit-seat utf8s the
adjacency keeps, the @954 caption + @955 xmap lambda bodies the
bootstrap walk serves, the lang row the caption speaks.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

# 1) the cp utf8s the Options class keeps (the notification/percent rows)
cp, _ = core["method_bodies"](z, "net.minecraft.client.Options")
for idx in [240, 950, 954, 955, 984, 959]:
    try:
        kind, val = core["cp_get"](cp, idx) if "cp_get" in core else (None, None)
    except Exception:
        kind, val = "?", f"<no cp_get helper: raw={cp[idx] if idx < len(cp) else 'oob'}>"
    print(f"cp#{idx}: {kind} = {val}")

print()
# any utf8 mentioning notification
for i, c in enumerate(cp):
    if c and c[0] == 1:
        s = c[1]
        try:
            t = s.decode("utf-8", "replace")
        except Exception:
            continue
        if "notification" in t.lower() or t in ("options.percent", "options.generic_value"):
            print(f"utf8#{i}: {t!r}")

print()
# 2) the lambda bodies: every Options method whose disasm wears ddiv /
#    10.0 (the xmap) or the percent/genericValueLabel call (the caption)
got = core["method_bodies"](z, "net.minecraft.client.Options")
cp2, bs2 = got
for (nm, desc), b in sorted(bs2.items()):
    if b is None or not nm.startswith("lambda"):
        continue
    clen = struct.unpack_from(">I", b, 4)[0]
    code = b[8:8 + clen]
    try:
        ls = core["disasm"](cp2, code)
    except Exception:
        continue
    joined = [l for _, l in ls]
    if any("10.0" in l or "ddiv" in l or "percent" in l or "genericValueLabel" in l
           or "seconds" in l for l in joined) or "notification" in str(joined):
        print(f"== Options::{nm} {desc} ==")
        for pc, l in ls:
            print(f"   {pc}: {l}")
        print()

# 3) the lang row: the en_us json the caches keep
import glob, json, os
cands = glob.glob(".mc2rs/**/en_us.json", recursive=True) + glob.glob("assets/**/en_us.json", recursive=True)
print("== lang caches ==", cands[:5])
for path in cands[:2]:
    data = json.load(open(path, encoding="utf-8"))
    for k, v in data.items():
        if "notification" in k.lower() or "Notification" in v:
            print(f"  {k} = {v!r}")
