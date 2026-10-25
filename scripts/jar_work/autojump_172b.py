#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE b (the dev-172 round): the ctor window around the
autoJump putfield (the default + the caption the walk keeps), the
processOptions row context, every LocalPlayer/Player method body the
autoJumpTime / options.autoJump references bank (the writer the law serves).

Usage: python3 scripts/jar_work/autojump_172b.py [client.jar]
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
class_utf8_words = NS["class_utf8_words"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPTS = "net.minecraft.client.Options"
LP = "net.minecraft.client.player.LocalPlayer"


def ctx_scan(z, cls, needle, around, span, max_pc):
    ms = methods_of(z, cls)
    for sig, (cp, body) in ms.items():
        if needle.split("(")[0] in sig.split("(")[0] and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            lines = walk_code(cp, code)[:max_pc]
            lo = max(0, around - span)
            hi = min(len(lines), around + span)
            print(f"--- {cls}.{sig} ctx pc {lo}..{hi}")
            for ln in lines[lo:hi]:
                print(ln)


print("[1] Options.<init> ctx around pc 1726 (the autoJump instance)")
ctx_scan(z, OPTS, "<init>", 1712, 40, 4000)

print()
print("[2] Options.processOptions ctx around pc 10")
ctx_scan(z, OPTS, "processOptions", 0, 30, 4000)


def all_method_scan(z, cls, word):
    ms = methods_of(z, cls)
    if not ms:
        print(f"!! {cls} not found")
        return
    for sig, (cp, body) in ms.items():
        if not body:
            continue
        clen = int.from_bytes(body[4:8], "big")
        code = body[8:8 + clen]
        hits = [ln for ln in walk_code(cp, code) if word in ln]
        if hits:
            print(f"--- {cls}.{sig}")
            for ln in hits:
                print("   " + ln)


print()
print("[3] LocalPlayer: every method body referencing 'autoJump'")
all_method_scan(z, LP, "autoJump")

print()
print("[4] Player (common): every method body referencing 'autoJump'")
all_method_scan(z, "net.minecraft.world.entity.player.Player", "autoJump")

print()
print("[5] en_us options.on/off words")
class_utf8_words(z, OPTS, ["options.on", "options.off"])
