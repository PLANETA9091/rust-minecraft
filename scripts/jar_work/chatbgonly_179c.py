#!/usr/bin/env python3
"""THE INPUT BAND'S OWN LAW PROBE (autopush-179c): the caption key #1129
the row speaks, the value keys 2562/2564 the lambda$new$76 pair wears, the
GameRenderer walk the `backgroundForChatOnly ? raw : ARGB.colorFromFloat`
pair the input band answers, the OptionsRenderState field the value rides.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)

def utf8_of(idx, cp):
    e = cp[idx] if idx < len(cp) else None
    if e and e[0] == 8:
        u = cp[e[1]]
        return u[1] if u and u[0] == 1 else e[1]
    return e

# ---- 1. the caption key #1129 + the value keys 2562/2564 ----
d = z.read("net/minecraft/client/Options.class")
cp, _p = core["parse_cp"](d)
for idx in (1129, 2562, 2564):
    print("Options CP#%d:" % idx, repr(utf8_of(idx, cp)))
print()

# ---- 2. the GameRenderer consumer: the method the ref wears ----
cp3, bs3 = core["method_bodies"](z, "net.minecraft.client.renderer.GameRenderer")
if bs3:
    for (nm, desc), b in sorted(bs3.items()):
        if b is None:
            continue
        txt = "\n".join(l for _, l in dis(cp3, b))
        if "backgroundForChatOnly" in txt or "textBackgroundOpacity" in txt:
            print("== GameRenderer ::", nm, desc)
            for pc, l in dis(cp3, b):
                print("   %4d: %s" % (pc, l))
            print()
else:
    print("!! GameRenderer bodies None")
