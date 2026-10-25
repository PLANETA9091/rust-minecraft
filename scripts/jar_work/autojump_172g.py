#!/usr/bin/env python3
"""THE AUTOJUMP GATES PROBE g (dev-172): find isStayingOnGroundSurface +
makeJump across the client+entity classes (cp word scan the home answers).

Usage: python3 scripts/jar_work/autojump_172g.py [client.jar]
"""
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)

utf = NS["utf"]
walk_code = NS["walk_code"]
methods_of = NS["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CANDS = [
    "net.minecraft.world.entity.Entity",
    "net.minecraft.world.entity.LivingEntity",
    "net.minecraft.world.entity.player.Player",
    "net.minecraft.client.player.LocalPlayer",
    "net.minecraft.client.player.ClientInput",
    "net.minecraft.world.entity.player.Input",
]


def cp_words(z, cls):
    d = z.read(f"{cls.replace('.', '/')}.class")
    cp, _ = NS["parse_cp"](d)
    words = []
    for i, e in enumerate(cp):
        if e and e[0] == "u":
            words.append((i, utf(cp, i)))
    return words


for cls in CANDS:
    try:
        ws = cp_words(z, cls)
    except KeyError:
        print(f"!! {cls} not in jar")
        continue
    hits = [(i, w) for i, w in ws if "StayingOnGround" in w or "makeJump" in w or w == "makeJump"]
    if hits:
        print(f"=== {cls}")
        for i, w in hits:
            print(f"  cp#{i}: {w!r}")

# dump whatever bodies matched
def dump_named(z, cls, name_sub):
    ms = methods_of(z, cls)
    for sig, (cp, body) in ms.items():
        if name_sub in sig.split("(")[0] and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            try:
                lines = walk_code(cp, code)
            except Exception as ex:
                lines = [f"!! desync {ex}"]
            print(f"--- {cls}.{sig}")
            for ln in lines[:40]:
                print(ln)
            print()


for cls in CANDS:
    try:
        ws = cp_words(z, cls)
    except KeyError:
        continue
    names = {w for _, w in ws if "makeJump" in w}
    for nm in names:
        dump_named(z, cls, nm)
