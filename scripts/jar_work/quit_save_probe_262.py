#!/usr/bin/env python3
"""THE QUIT LANE'S OWN SAVE RECON (autopush-170): the vanilla
Minecraft.stop()/close()/destroy() walk — which shutdown method the
`Options.save()` call answers (the quit button's `stop()` the law rides),
the options.txt's own final write the shutdown speaks. Every shutdown-
named method's bytecode is flagged for `save` references.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

ms = methods_of(z, "net.minecraft.client.Minecraft")
shutdown = [sig for sig in ms if sig.split("(")[0] in
            ("stop", "close", "destroy", "unload", "emergencySave", "halt", "destroyOrClose")]
print("=== shutdown-named Minecraft methods:", shutdown)

for sig in shutdown:
    cp, body = ms[sig]
    if body is None:
        print(f"--- Minecraft.{sig}: abstract/native")
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = walk_code(cp, code)
    save_hits = [l for l in lines if "save" in l.lower()]
    print(f"\n=== Minecraft.{sig} ({len(lines)} instrs)")
    if save_hits:
        print("  SAVE HITS:")
        for h in save_hits:
            print("   ", h)
    else:
        print("  (no save refs)")

# ALSO: who calls stop()? the TitleScreen's quit button's onPress —
# dump TitleScreen's lambda/arm bytecode flags for stop refs.
print("\n\n######## TitleScreen stop refs")
tms = methods_of(z, "net.minecraft.client.gui.screens.TitleScreen")
for sig, (cp, body) in tms.items():
    if body is None:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = walk_code(cp, code)
    except Exception:
        continue
    hits = [l for l in lines if "stop" in l.lower()]
    if hits:
        print(f"--- TitleScreen.{sig}:")
        for h in hits:
            print("   ", h)
