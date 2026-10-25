#!/usr/bin/env python3
"""THE /VERSION SERVER-ARM'S OWN PROBE (the autopush-215 round): where
the /version command registers (the Brigadier tree the register walk
keeps), the client's own command dispatch walk (the ChatScreen submit
the sendCommand lane serves — the client-local commands any check
speaks), the VersionCommand class the F3+V lane already banks.

Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/vercmd_215.py
"""
import os, zipfile, json

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

names = z.namelist()

print("=" * 72)
print("[A] the VersionCommand class file(s):")
for n in names:
    if "VersionCommand" in n and n.endswith(".class"):
        print("  ", n)

print("=" * 72)
print("[B] any client command dispatcher candidates:")
for n in names:
    base = n.rsplit("/", 1)[-1]
    if n.endswith(".class") and (
        "ClientCommand" in base
        or "ClientSuggestion" in base
        or ("CommandDispatcher" in base and "client" in n)
    ):
        print("  ", n)

print("=" * 72)
print("[C] VersionCommand's own methods:")
VC = None
for n in names:
    if "VersionCommand" in n and n.endswith(".class"):
        VC = n[:-6]
        break
if VC:
    meths = methods_of(z, VC)
    for sig in sorted(meths):
        print("  ", sig)
    # walk every method body, print all (short)
    for sig, (cp, body) in sorted(meths.items()):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        print("-" * 72)
        print(f"[C.{sig}] body:")
        for ln in lines:
            print("  ", ln)

print("=" * 72)
print("[D] who cites VersionCommand — the register walk's own class:")
# scan likely registrars' constant pools for the VersionCommand ref
import io
def cp_class_names(cp):
    out = []
    for e in cp:
        if e is None:
            continue
        if e[0] == 7:  # CONSTANT_Class
            try:
                out.append(utf(cp, e[1]))
            except Exception:
                pass
    return out

for cand in [
    "net/minecraft/client/Minecraft",
    "net/minecraft/client/gui/screens/ChatScreen",
    "net/minecraft/client/multiplayer/ClientPacketListener",
    "net/minecraft/client/CommandSuggestions",
    "net/minecraft/commands/Commands",
]:
    try:
        m = methods_of(z, cand)
    except Exception as e:
        print(f"  !! {cand}: {e}")
        continue
    if not m:
        print(f"  !! {cand}: no methods (class absent)")
        continue
    hits = []
    for sig, (cp, body) in m.items():
        for cn in cp_class_names(cp):
            if "VersionCommand" in cn:
                hits.append((sig, cn))
    print(f"  {cand}: {len(hits)} cite(s)")
    for sig, cn in hits:
        print(f"     {sig} -> {cn}")
    # the register CALL SITE's own window — the boolean the iload_1 wears
    for sig, (cp, body) in m.items():
        try:
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            lines = walk_code(cp, code)
        except Exception:
            continue
        for idx, ln in enumerate(lines):
            if "VersionCommand" in str(ln) and "register" in str(ln):
                lo = max(0, idx - 14)
                print(f"  --- call-site window {sig}:")
                for j in range(lo, min(len(lines), idx + 3)):
                    print("    ", lines[j])

print("=" * 72)
print("[E] ChatScreen.handleChatInput / submit walk:")
CH = "net/minecraft/client/gui/screens/ChatScreen"
try:
    meths = methods_of(z, CH)
    for sig, (cp, body) in sorted(meths.items()):
        if "handleChatInput" in sig or "normalize" in sig:
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            print(f"--- {sig}:")
            for ln in walk_code(cp, code):
                print("  ", ln)
except Exception as e:
    print(f"  !! {e}")

print("=" * 72)
print("[F] ClientPacketListener.sendCommand walk (the dispatch arm):")
CP = "net/minecraft/client/multiplayer/ClientPacketListener"
try:
    meths = methods_of(z, CP)
    for sig, (cp, body) in sorted(meths.items()):
        if sig.startswith("sendCommand(") or sig.startswith("sendCommand "):
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            print(f"--- {sig}:")
            for ln in walk_code(cp, code):
                print("  ", ln)
except Exception as e:
    print(f"  !! {e}")

print("=" * 72)
print("[G] en_us rows the command answers:")
data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
lang = json.loads(data)
for k, v in lang.items():
    if k.startswith("commands.version"):
        print(f"  {k} = {v!r}")
