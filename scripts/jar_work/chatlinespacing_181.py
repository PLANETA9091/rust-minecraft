#!/usr/bin/env python3
"""THE CHAT LINE SPACING'S OWN ROW PROBE (autopush-181): the
`chatLineSpacing` option the Options ctor's registration walk hosts —
the accessibility array's index-7 seat the 178 record banks, the line
height the `9 + round(4.0 * spacing)` fold the ChatComponent walk
speaks. The probe walks: 1) the Options.<init> registration (the
caption indy the lang key wears, the value set the UnitDouble law
answers, the default the dconst_0 banks), 2) the ChatComponent's own
fold walk (the ldc rows the pitch keeps), 3) the processOptions visit
seat (the @493 ldc the adjacency banks), 4) the en_us lang row.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")


def bodies_of(cls):
    cp, bs = core["method_bodies"](z, cls)
    return cp, bs


def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)


def find(bs, nm):
    for (n, d), b in bs.items():
        if n == nm:
            return b
    return None


# ---- 1. the Options.<init> registration window (the chatOpacity row
#         the pcs 741-818 bank keeps — the lineSpacing row the NEXT
#         window the same ctor walk wears) ----
cp, bs = bodies_of("net.minecraft.client.Options")
body = find(bs, "<init>")
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8:8 + clen]
lines = dis(cp, code)
print("== Options.<init> — the pcs 800-1000 window (the chatOpacity row's AFTER the lineSpacing registration wears) ==")
for pc, l in lines:
    if 800 <= pc <= 1000:
        print(f"   {pc}: {l}")
print()

# the caption indy resolution: the indy rows in the window — the
# bootstrap args the lang key utf8 wears. The disasm's indy detail is
# bare, so the cp walk answers: find the indy cp entries the window's
# pcs reference. Cheaper: the utf8 scan — the lang keys near the
# chat.opacity row the table keeps.
print("== the utf8 constants the ctor speaks (the chat.* lang keys) ==")
for i, e in enumerate(cp):
    if e and e[0] == "u":
        s = e[1]
        if "options.chat." in s:
            print(f"   #{i}: {s!r}")
print()

# ---- 2. the ChatComponent's own fold walk (the `9 + round(4.0 *
#         spacing)` rows the pitch keeps) ----
cpc, bsc = bodies_of("net.minecraft.client.gui.components.ChatComponent")
for (nm, desc), b in sorted(bsc.items()):
    if b is None:
        continue
    clen2 = struct.unpack_from(">I", b, 4)[0]
    code2 = b[8:8 + clen2]
    ls = dis(cpc, code2)
    joined = [l for _, l in ls]
    if any("float 4.0" in l or "double 4.0" in l for l in joined):
        print(f"== ChatComponent::{nm} {desc} — the 4.0 row the fold wears ==")
        for pc, l in ls:
            print(f"   {pc}: {l}")
        print()

# ---- 3. the processOptions visit seat (the ldc@493 walk the 178
#         record banks — the chatLineSpacing row the getter the seat
#         wears) ----
b3 = find(bs, "processOptions")
clen3 = struct.unpack_from(">I", b3, 4)[0]
code3 = b3[8:8 + clen3]
print("== Options.processOptions — the pcs 460-560 window (the chatOpacity@480 / lineSpacing@493 / textbg@506 / bgOnly@523 ladder) ==")
for pc, l in dis(cp, code3):
    if 460 <= pc <= 560:
        print(f"   {pc}: {l}")
print()

# ---- 4. the en_us lang row ----
try:
    raw = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
    import json
    tbl = json.loads(raw)
    for k in ["options.chat.line_spacing", "options.chat.opacity",
              "options.chat.visibility"]:
        print(f"   {k} = {tbl.get(k)!r}")
except Exception as e:
    print("lang fail:", repr(e))
