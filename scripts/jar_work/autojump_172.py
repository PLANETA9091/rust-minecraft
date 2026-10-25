#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE (the dev-172 round): the autoJump OptionInstance
the Options ctor banks (the default the ctor arm wears), the processOptions
file row the load speaks, the LocalPlayer.aiStep autoJump readsites the
movement law wears (the gates the bytecode keeps), the Controls array row
4 the re-verify banks, the en_us rows the display words keep.

Usage: python3 scripts/jar_work/autojump_172.py [client.jar]
"""
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)

parse_cp = NS["parse_cp"]
utf = NS["utf"]
walk_code = NS["walk_code"]
methods_of = NS["methods_of"]
dump_method = NS["dump_method"]
class_utf8_words = NS["class_utf8_words"]
list_methods = NS["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPTS = "net.minecraft.client.Options"


def dump_scan(z, cls, needle, max_pc, hits_only):
    """dump the method, optionally only the lines containing hits_only."""
    ms = methods_of(z, cls)
    if not ms:
        print(f"!! class {cls} not found")
        return
    for sig, (cp, body) in ms.items():
        if needle.split("(")[0] in sig.split("(")[0] and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            lines = walk_code(cp, code)
            print(f"--- {cls}.{sig} ({len(lines)} insns)")
            for ln in lines:
                if hits_only is None or hits_only in ln:
                    print(ln)


print("[1] Options.<init>: the autoJump instance (all lines w/ autoJump)")
dump_scan(z, OPTS, "<init>", 4000, "autoJump")

print()
print("[2] Options.processOptions: the autoJump file row")
dump_scan(z, OPTS, "processOptions", 4000, "autoJump")

print()
print("[3] LocalPlayer.aiStep: every autoJump reference (the law's readsites)")
dump_scan(z, "net.minecraft.client.player.LocalPlayer", "aiStep", 6000, "autoJump")

print()
print("[4] LocalPlayer.aiStep: horizontalCollision readsites")
dump_scan(z, "net.minecraft.client.player.LocalPlayer", "aiStep", 6000, "horizontalCollision")

print()
print("[5] ControlsScreen static: the array row 4 (re-verify)")
dump_scan(z, "net.minecraft.client.gui.screens.options.controls.ControlsScreen",
          "options", 900, None)

print()
print("[6] en_us utf8 words (autoJump / on / off)")
class_utf8_words(z, OPTS, ["autoJump"])
