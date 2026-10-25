#!/usr/bin/env python3
"""THE PROFILES' OWN CLINIT ROUND (autopush-234): the DebugScreenProfile
clinit the PROFILES map banks — the DEFAULT/PERFORMANCE presets the
vanilla rows wear. + the SerializedOptions the codec the fields speak
(the 985 JSON lane the vanilla truth verifies).

Usage: python3 scripts/jar_work/debugprofile_234.py <client.jar>
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

print("=========== I. DebugScreenProfile clinit (the PROFILES map)")
dump("net/minecraft/client/gui/components/debug/DebugScreenProfile")

print("=========== II. the SerializedOptions fields (the 985 lane's names)")
dump("net/minecraft/client/gui/components/debug/DebugScreenEntryList$SerializedOptions")

print("=========== III. DebugScreenEntryList load/save (the file lane)")
dump("net/minecraft/client/gui/components/debug/DebugScreenEntryList",
     only=["load", "save", "Options"])
