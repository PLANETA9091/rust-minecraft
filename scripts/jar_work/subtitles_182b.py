#!/usr/bin/env python3
"""THE SUBTITLES' OWN ADJACENCY PROBE (autopush-182b): the showSubtitles
registration the Options ctor's pcs 2053-2067 window keeps, the
processOptions the ldc@149 visit seat the save row's order wears, the
Mth.clampedLerp (FFF) body the fade law's own signature answers, the
SoundEngine.play the range the listener's fload-6 arg keeps, the
SubtitleOverlay glyph constants (the `<`/` `/`>` utf8s), the en_us
lang rows.
"""
import struct
import sys
import zipfile
import json

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

cp, bs = core["method_bodies"](z, "net.minecraft.client.Options")
b = None
for (n, d), bb in bs.items():
    if n == "<init>":
        b = bb
clen = struct.unpack_from(">I", b, 4)[0]
print("== Options.<init> pcs 2050-2070 (the showSubtitles window) ==")
for pc, l in core["disasm"](cp, b[8:8 + clen]):
    if 2050 <= pc <= 2070:
        print(f"   {pc}: {l}")
e = cp[1121]
s = e[1] if e[0] == "u" else cp[e[1]][1]
print("caption #1121:", repr(s))
print()

b3 = None
for (n, d), bb in bs.items():
    if n == "processOptions":
        b3 = bb
clen3 = struct.unpack_from(">I", b3, 4)[0]
print("== Options.processOptions — our carried keys' visit pcs ==")
ours = {"mouseSensitivity", "pauseOnLostFocus", "showSubtitles",
        "chatOpacity", "chatLineSpacing", "textBackgroundOpacity",
        "backgroundForChatOnly"}
for pc, l in core["disasm"](cp, b3[8:8 + clen3]):
    if "cp(8" in l:
        import re
        for m in re.finditer(r"cp\(8, (\d+)\)", l):
            idx = int(m.group(1))
            e = cp[idx]
            s = e[1] if e[0] == "u" else (cp[e[1]][1] if e[0] == 8 else None)
            if s in ours:
                print(f"   {pc}: {s}")
print()

cpm, bsm = core["method_bodies"](z, "net.minecraft.util.Mth")
b = bsm.get(("clampedLerp", "(FFF)F"))
if b:
    clen = struct.unpack_from(">I", b, 4)[0]
    print("== Mth.clampedLerp (FFF)F ==")
    for pc, l in core["disasm"](cpm, b[8:8 + clen]):
        print(f"   {pc}: {l}")
print()

cpe, bse = core["method_bodies"](z, "net.minecraft.client.sounds.SoundEngine")
b = bse.get(("play", "(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/client/sounds/SoundEngine$PlayResult;"))
clen = struct.unpack_from(">I", b, 4)[0]
print("== SoundEngine.play pcs 154-176 (the range calc) + 230-252 (the listener arg) ==")
for pc, l in core["disasm"](cpe, b[8:8 + clen]):
    if 154 <= pc <= 176 or 230 <= pc <= 252:
        print(f"   {pc}: {l}")
print()

cps, bss = core["method_bodies"](z, "net.minecraft.client.gui.components.SubtitleOverlay")
for i in (158, 163, 165):
    e = cp[i]
    s = e[1] if e[0] == "u" else (cp[e[1]][1] if e[0] == 8 else "?")
    print(f"SubtitleOverlay utf8#{i}: {s!r}")
print()

raw = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
tbl = json.loads(raw)
for k in ["options.showSubtitles", "options.on", "options.off",
          "subtitles.block.generic.break", "subtitles.entity.item.pickup"]:
    print(f"lang {k} = {tbl.get(k)!r}")
# the sound table's own subtitle field (stone break) — the sounds.json
# the asset index cache keeps (the jar itself carries no sounds.json)
zj = json.loads(open("assets-cache/jar/assets/minecraft/sounds.json").read())
for k in ["block.stone.break", "block.stone.step", "block.stone.place", "block.stone.hit"]:
    row = zj.get(k)
    print(f"sounds.json {k}: subtitle={row.get('subtitle') if isinstance(row, dict) else row!r}")
