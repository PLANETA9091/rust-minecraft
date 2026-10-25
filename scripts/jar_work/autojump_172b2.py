#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE b2 (the dev-172 round): pc-indexed context walks
+ defensive code walking (wide/0xc4 the simple walker desyncs — the try
gate the scan keeps). The autoJump instance's default the ctor keeps, the
LocalPlayer.<init>'s autoJumpEnabled arm, every LocalPlayer method body
the autoJump references bank.

Usage: python3 scripts/jar_work/autojump_172b2.py [client.jar]
"""
import os
import re
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


def safe_lines(cp, code):
    try:
        return walk_code(cp, code)
    except Exception as ex:
        return [f"   !! desync after: {ex}"]


def safe_methods(z, cls):
    ms = methods_of(z, cls)
    if not ms:
        return None
    out = {}
    for sig, (cp, body) in ms.items():
        if not body:
            continue
        clen = int.from_bytes(body[4:8], "big")
        code = body[8:8 + clen]
        out[sig] = (cp, safe_lines(cp, code))
    return out


def ctx_by_pc(z, cls, needle, hit_pc, span):
    ms = safe_methods(z, cls)
    if not ms:
        print(f"!! {cls} not found")
        return
    for sig, (cp, lines) in ms.items():
        if needle.split("(")[0] in sig.split("(")[0]:
            idx = next((i for i, ln in enumerate(lines)
                        if ln.startswith(f"{hit_pc}:")), None)
            if idx is None:
                continue
            lo, hi = max(0, idx - span), min(len(lines), idx + span)
            print(f"--- {cls}.{sig}")
            for ln in lines[lo:hi]:
                print(ln)
            return
    print(f"!! no {needle} in {cls}")


def all_method_scan(z, cls, word):
    ms = safe_methods(z, cls)
    if not ms:
        print(f"!! {cls} not found")
        return
    for sig, (cp, lines) in ms.items():
        hits = [ln for ln in lines if word in ln]
        if hits:
            print(f"--- {cls}.{sig}")
            for ln in hits:
                print("   " + ln)


print("[1] Options.<init> ctx around autoJump putfield pc 1726")
ctx_by_pc(z, OPTS, "<init>", 1726, 30)

print()
print("[2] LocalPlayer.<init> ctx around autoJumpEnabled pc 58")
ctx_by_pc(z, LP, "<init>", 58, 40)

print()
print("[3] LocalPlayer: every method body referencing 'autoJump'")
all_method_scan(z, LP, "autoJump")

print()
print("[4] en_us options.on/off words (generic boolean captions)")
class_utf8_words(z, OPTS, ["options.on", "options.off"])
