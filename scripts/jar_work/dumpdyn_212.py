#!/usr/bin/env python3
"""THE DUMP DYNAMIC TEXTURES' OWN PROBE (the autopush-212 round): the
TextureUtil.getDebugTexturePath body + the TextureManager.dumpAllSheets
body — the paths the lane writes, the sheets the walk keeps.
Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/dumpdyn_212.py
"""
import os, zipfile, struct

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

def dump_method(cls, frag):
    print("=" * 72)
    print(f"[{cls}] methods matching {frag!r}:")
    try:
        meths = methods_of(z, cls)
    except Exception as ex:
        print("  load failed:", ex)
        return
    for sig, (cp, body) in meths.items():
        if frag in sig and body is not None:
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            print(f"--- {sig} ---")
            print(walk_code(cp, code))

dump_method("com/mojang/blaze3d/platform/TextureUtil", "getDebugTexturePath")
dump_method("net/minecraft/client/renderer/texture/TextureManager", "dumpAllSheets")
dump_method("net/minecraft/client/renderer/texture/TextureAtlas", "dumpContents")

print("=" * 72)
print("[extra] dumpSpriteNames + writeAsPNG + toDebugFileName:")
dump_method("net/minecraft/client/renderer/texture/TextureAtlas", "dumpSpriteNames")
dump_method("net/minecraft/resources/Identifier", "toDebugFileName")
dump_method("com/mojang/blaze3d/platform/TextureUtil", "writeAsPNG")

print("=" * 72)
print("[extra2] KeyboardHandler.debugFeedbackComponent (the color law):")
dump_method("net/minecraft/client/KeyboardHandler", "debugFeedbackComponent")
