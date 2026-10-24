#!/usr/bin/env python3
"""THE DEBUG CLEAR CHAT'S OWN FUNCTIONAL LANE (autopush-205): the
hdk_262.txt pc 198-227 window the lane's call answers — the
`minecraft.gui.hud.getChat().clearMessages(false)` the iconst_0 arg
wears, the istore_3 the used flag banks. The walk the boolean's
own meaning reads: the ChatComponent.clearMessages(Z) body the
fields the arg gates (the sent-history arm the true branch keeps?
the display rows the unconditional clear serves?), the caller the
handleDebugKeys gate the screen-open question answers.

Usage: python3 scripts/jar_work/clearchat_lane_205.py <client.jar>
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


def dump(cls, sigwin):
    print(f"===== {cls} =====")
    try:
        d = dict(methods_of(z, cls))
    except Exception as e:
        print("  CLASS NOT FOUND:", e)
        return {}
    for sig, (cp, body) in sorted(d.items()):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        lines = list(walk_code(cp, code))
        print(f"  --- {sig} (max_locals={struct.unpack_from('>H', body, 2)[0]})")
        for t in lines:
            print(f"    {t}")
    return d


print("########## I. ChatComponent.clearMessages(Z)V — the boolean's own walk")
CC = "net.minecraft.client.gui.components.ChatComponent"
d = dict(methods_of(z, CC))
for sig, (cp, body) in sorted(d.items()):
    if sig.startswith("clearMessages"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"  --- {sig}")
        for t in walk_code(cp, code):
            print(f"    {t}")

print()
print("########## II. the clearMessages callers — who walks it")
for n in z.namelist():
    if not n.endswith(".class"):
        continue
    cls = n[:-6].replace("/", ".")
    if "$" in cls:
        continue
    try:
        dd = dict(methods_of(z, cls))
    except Exception:
        continue
    for sig, (cp, body) in dd.items():
        if body is None:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        if clen == 0 or clen > 60000:
            continue
        try:
            code = body[8 : 8 + clen]
            text = "\n".join(walk_code(cp, code))
        except Exception:
            continue
        if "clearMessages" in text and cls != CC:
            hits = [l for l in text.split("\n") if "clearMessages" in l]
            print(f"  {cls}.{sig}")
            for h in hits:
                print(f"      {h.strip()}")

print()
print("########## III. the handleDebugKeys caller — the gate chain")
for n in z.namelist():
    if not n.endswith(".class"):
        continue
    cls = n[:-6].replace("/", ".")
    if "$" in cls:
        continue
    try:
        dd = dict(methods_of(z, cls))
    except Exception:
        continue
    for sig, (cp, body) in dd.items():
        if body is None:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        if clen == 0 or clen > 60000:
            continue
        try:
            code = body[8 : 8 + clen]
            text = "\n".join(walk_code(cp, code))
        except Exception:
            continue
        if "handleDebugKeys" in text and "handleDebugKeys(" not in sig:
            lines = text.split("\n")
            hits = [i for i, l in enumerate(lines) if "handleDebugKeys" in l]
            print(f"  {cls}.{sig}")
            for i in hits:
                lo = max(0, i - 14)
                for l in lines[lo : i + 3]:
                    print(f"      {l.strip()}")
                print("      ...")
