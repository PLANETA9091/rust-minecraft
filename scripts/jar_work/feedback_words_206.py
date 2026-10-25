#!/usr/bin/env python3
"""THE DEBUG FEEDBACK'S OWN WORDS (autopush-206): the
handleDebugKeys walk the debugFeedbackTranslated lanes the strings
read — the hitboxes@193 / chunkBorders@292 / focusPause@796 the on/off
pairs the cp indexes the utf8 answers, the lang rows the display
words keep. The charts/dump/clearChat lanes the feedback the walk
never serves (the silent toggles the Rust verdict already keeps).

Usage: python3 scripts/jar_work/feedback_words_206.py <client.jar>
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

KH = "net.minecraft.client.KeyboardHandler"
d = dict(methods_of(z, KH))
cp, body = d["handleDebugKeys(Lnet/minecraft/client/input/KeyEvent;)Z"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
lines = list(walk_code(cp, code))

WINDOWS = [
    ("reloadChunk", 101, 130),
    ("showHitboxes", 156, 198),
    ("showChunkBorders", 228, 298),
    ("spectate", 421, 469),
    ("switchGameMode", 557, 651),
    ("focusPause", 749, 802),
    ("reloadResourcePacks", 905, 933),
    ("copyLocation", 1004, 1044),
    ("dumpVersion", 1164, 1201),
]
import re
for name, lo, hi in WINDOWS:
    print(f"=== {name} (pc {lo}..{hi})")
    for t in lines:
        m = re.match(r"\s*(\d+):", t)
        if not m:
            continue
        pc = int(m.group(1))
        if lo <= pc <= hi and ("ldc" in t or "debugFeedback" in t or "istore" in t):
            print(f"    {t.strip()}")
            m2 = re.search(r"ldcw@(\d+)|ldc_w #(\d+)|ldc #(\d+)", t)
            if m2:
                idx = int(m2.group(1) or m2.group(2) or m2.group(3))
                try:
                    e = cp[idx]
                    word = utf(cp, e[1]) if e[0] == 8 else str(e)
                    print(f"        -> utf8: {word!r}")
                except Exception as e:
                    print(f"        -> utf8 read failed: {e}")

print()
print("=== the lang rows (debug.*)")
import json
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        table = json.loads(z.read(n).decode("utf-8"))
        for k in sorted(table):
            if k.startswith("debug.") and ("hitbox" in k or "border" in k
                                           or "pause_focus" in k or "chart" in k
                                           or "lightmap" in k or "profiler" in k
                                           or "reload_chunks" in k):
                print(f"  {k} = {table[k]}")
        break
