#!/usr/bin/env python3
"""THE SILENT LANES' OWN PROBE (the autopush-212 round): the
handleDebugKeys windows the three candidates keep — pc 651 debugOptions,
pc 749 focusPause, pc 802 dumpDynamicTextures — the lane bodies the
walk answers, the cp strings, the en_us rows, the KeyMapping binds.
Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/silentlanes_212.py
"""
import sys, zipfile, json

import os
HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; ref_name = NS["ref_name"]
parse_cp = NS["parse_cp"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CLS = "net/minecraft/client/KeyboardHandler"
meths = methods_of(z, CLS)

def walk_window(lo, hi, tag):
    print("=" * 72)
    print(f"[{tag}] handleDebugKeys pc {lo}-{hi}:")
    for sig, (cp, body) in meths.items():
        if sig.startswith("handleDebugKeys"):
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            lines = walk_code(cp, code)
            for ln in lines:
                try:
                    pc = int(str(ln).split(":")[0].strip())
                except ValueError:
                    continue
                if lo <= pc <= hi:
                    print(ln)

walk_window(640, 770, "A-the debugOptions+focusPause window")
walk_window(759, 800, "A2-the focusPause tail (the state words)")
walk_window(790, 930, "B-the dumpDynamicTextures window")

print("=" * 72)
print("[C] KeyboardHandler cp strings (options/pause/texture/dump):")
cp_seen = False
for sig, (cp, body) in meths.items():
    if cp is None or cp_seen:
        continue
    for i in range(1, len(cp)):
        e = cp[i] if isinstance(cp, list) else cp.get(i)
        if not e or e[0] != 1:
            continue
        s = e[1]
        low = s.lower()
        if ("debug" in low and any(w in low for w in
            ("options", "pause", "texture", "dump", "dynamic", "focus"))) \
           or "pauseOnLostFocus" in s or "dumpTextures" in s:
            print(f"  cp#{i}: {s!r}")
    cp_seen = True

print("=" * 72)
print("[D] en_us rows (debug.options / debug.pause / dump_dynamic):")
data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
lang = json.loads(data)
for k, v in lang.items():
    if any(w in k for w in ("debug.options", "debug.pause", "dump_dynamic",
                            "debug.dump", "debug.focus")):
        print(f"  {k} = {v!r}")

print("=" * 72)
print("[E] Options keyDebug* fields (the binds the lanes answer):")
OPT = "net/minecraft/client/Options"
try:
    om = methods_of(z, OPT)
    for sig, (cp, body) in om.items():
        if "<clinit>" in sig or "KeyMapping" in sig:
            continue
    # field list via javap-like scan: walk the ctor for Fieldrefs
except Exception as ex:
    print("  Options walk skipped:", ex)
# scan ALL classes' cp for keyDebug field names is heavy — use the
# keyboard screen / Options ctor window instead: grep the class bytes.
raw = z.read(OPT + ".class")
import re
names = set(re.findall(rb"keyDebug[A-Za-z]+", raw))
print("  field-name hits in Options.class:", sorted(n.decode() for n in names))

print("=" * 72)
print("[F] the cp entries the lane windows cite (415/439/495 + neighbors):")
for sig, (cp, body) in meths.items():
    if not sig.startswith("handleDebugKeys"):
        continue
    for idx in (415, 437, 439, 495, 503):
        if 0 <= idx < len(cp):
            e = cp[idx]
            if e[0] == 8:  # CONSTANT_String -> two-level deref
                print(f"  cp[{idx}] str-> {utf(cp, e[1])!r}")
            elif e[0] == 1:
                print(f"  cp[{idx}] utf8-> {e[1]!r}")
            else:
                print(f"  cp[{idx}] tag={e[0]}")
    break

print("=" * 72)
print("[G] lang table — debug.options / debug.pause / debug.dump rows:")
for k, v in lang.items():
    if "debug." in k and any(w in k for w in ("option", "pause", "dump",
                                              "dynamic", "gamemode", "switch")):
        print(f"  {k} = {v!r}")
