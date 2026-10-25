#!/usr/bin/env python3
"""THE TOGGLE KEYMAPPING'S OWN LAW PROBE (the autopush-216 round): the
four ToggleKeyMapping instances the Options ctor banks (keyShift pc 2497,
keySprint pc 2532, keyUse pc 2624, keyAttack pc 2660) — the class's own
isDown/setDown/release walk the toggle law speaks, the four
toggleCrouch/toggleSprint/toggleUse/toggleAttack OptionInstances the
processOptions rows load/save, the en_us rows the display words keep.

Usage: python3 scripts/jar_work/toggle_216.py [client.jar]
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

print("[1] ToggleKeyMapping methods:")
list_methods(z, "net.minecraft.client.ToggleKeyMapping")

print()
print("[2] ToggleKeyMapping full bodies:")
ms = methods_of(z, "net.minecraft.client.ToggleKeyMapping")
for sig, (cp, body) in ms.items():
    if not body:
        print(f"--- {sig}: (no code)")
        continue
    clen = struct_len = int.from_bytes(body[4:8], "big")
    code = body[8:8 + clen]
    print(f"--- ToggleKeyMapping.{sig}")
    try:
        for line in walk_code(cp, code):
            print(line)
    except Exception as ex:
        print(f"   !! desync: {ex}")
    print()

print("[3] Options utf8 words containing 'toggle':")
class_utf8_words(z, "net.minecraft.client.Options", ["toggle"])

print()
print("[4] Options methods mentioning toggle:")
ms = methods_of(z, "net.minecraft.client.Options")
for sig in ms:
    if "toggle" in sig.lower() or "Toggle" in sig:
        print(f"  {sig}")

print()
print("[5] Options.lambda methods (the indy impls) — dumping the small ones:")
for sig, (cp, body) in ms.items():
    if "lambda" in sig and body:
        clen = int.from_bytes(body[4:8], "big")
        code = body[8:8 + clen]
        lines = list(walk_code(cp, code))
        txt = " | ".join(str(l).strip() for l in lines)
        if "toggle" in txt or "getfield #7.toggle" in txt:
            print(f"--- Options.{sig}")
            for l in lines:
                print(l)
            print()

print("[6] processOptions walk (the toggle rows the decode keeps):")
dump_method(z, "net.minecraft.client.Options", "processOptions", max_pc=1400)

print()
print("[7] lang table rows (en_us.json) — toggle/durability keys:")
try:
    data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
    import json
    table = json.loads(data)
    for k in sorted(table):
        if "toggle" in k.lower() or "durability" in k.lower():
            print(f"  {k} = {table[k]!r}")
except Exception as ex:
    print(f"  !! {ex}")
