#!/usr/bin/env python3
"""THE NOTIFICATION DISPLAY TIME'S OWN WALK PROBE (autopush-183): the
`notificationDisplayTime` OptionInstance the accessibility array's
index-9 seat hosts — the registration the Options.<init> walk keeps
(the IntRange the 1-60 the candidate banks), the lang row the caption
wears, the visit seat the save's own order banks, the SubtitleOverlay
the displayTime the fold reads (the 3000.0 * displayTime window the
overlay's own walk keeps).
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

# 1) the registration walk: Options.<init> the notificationDisplayTime
#    window the ldc/int rows keep
cp, bs = core["method_bodies"](z, "net.minecraft.client.Options")
print("== Options.<init> the notificationDisplayTime window ==")
for (nm, desc), b in bs.items():
    if nm != "<init>" or b is None:
        continue
    clen = struct.unpack_from(">I", b, 4)[0]
    code = b[8:8 + clen]
    ls = core["disasm"](cp, code)
    for pc, l in ls:
        if "notification" in l.lower() or "Notification" in l:
            lo = max(0, pc - 30)
            for p, line in ls:
                if lo <= p <= pc + 24:
                    print(f"   {p}: {line}")
            break

# 2) every class the notificationDisplayTime utf8 the constant pool
#    carries (the reference kinds the read sites speak)
print("== the notificationDisplayTime utf8's own classes ==")
hits = []
for n in z.namelist():
    if not n.endswith(".class"):
        continue
    d = z.read(n)
    if b"notificationDisplayTime" in d:
        hits.append(n)
for h in sorted(hits):
    print("  ", h)

# 3) the read sites' own walks (each method the utf8 near wears)
for n in sorted(hits):
    cls = n[:-6].replace("/", ".")
    got = core["method_bodies"](z, cls)
    if got is None:
        continue
    cp2, bs2 = got
    for (nm, desc), b in sorted(bs2.items()):
        if b is None:
            continue
        clen = struct.unpack_from(">I", b, 4)[0]
        code = b[8:8 + clen]
        try:
            ls = core["disasm"](cp2, code)
        except Exception:
            continue
        window = [(p, l) for p, l in ls if "notificationDisplayTime" in l]
        if not window:
            continue
        print(f"== {cls}::{nm} {desc} ==")
        for pc, l in window:
            lo = max(0, pc - 26)
            for p, line in ls:
                if lo <= p <= pc + 30:
                    print(f"   {p}: {line}")
            print("   ...")
