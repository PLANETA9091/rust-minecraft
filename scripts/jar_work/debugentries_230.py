#!/usr/bin/env python3
"""THE DEBUG OPTIONS' OWN ENTRIES ROUND (autopush-230): the tri-state
store the vanilla law banks — the DebugScreenEntryStatus enum the
three arms speak, the DebugScreenEntries clinit the 40 entry ids the
store keys, the DebugScreenEntryList.toggleStatus the chord's cycle
answers, the OptionEntry's CycleButton trio the pills' words wear.

Usage: python3 scripts/jar_work/debugentries_230.py <client.jar>
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

def dump(cls, only=None, maxpc=None):
    print(f"########## {cls}")
    try:
        d = dict(methods_of(z, cls))
    except Exception as e:
        print(f"  !! {e}")
        return
    for name, (cp, body) in sorted(d.items()):
        if only and not any(o in name for o in only):
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"  --- {name}")
        try:
            for t in walk_code(cp, code):
                pc = t.split(":", 1)[0]
                try:
                    p = int(pc)
                except ValueError:
                    continue
                if maxpc and p > maxpc:
                    continue
                print(f"    {t}")
        except Exception as e:
            print(f"    !! walk skipped: {e}")

print("=========== I. the STATUS enum (values + the ordinal law)")
dump("net.minecraft.client.gui.components.debug.DebugScreenEntryStatus")

print()
print("=========== II. the ENTRIES clinit (the id bank)")
dump("net.minecraft.client.gui.components.debug.DebugScreenEntries")

print()
print("=========== III. the LIST store (toggleStatus + the gates)")
dump("net.minecraft.client.gui.components.debug.DebugScreenEntryList",
     only=("toggleStatus", "isCurrentlyEnabled", "getStatus", "setStatus", "reset"))

print()
print("=========== IV. the lang rows (debug.entry.*)")
import json
lang = None
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        lang = n
        break
if lang:
    table = json.loads(z.read(lang).decode("utf-8"))
    for k in sorted(table):
        if k.startswith("debug.entry") or k.startswith("debug.options"):
            print(f"  {k} = {table[k]}")
