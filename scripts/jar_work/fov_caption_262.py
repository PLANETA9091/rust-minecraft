#!/usr/bin/env python3
"""THE FOV EFFECT SCALE'S OWN CAPTION PROBE (autopush-176): the
UnitDouble record's own toString the slider's value word speaks, and
the Options clinit window the fovEffectScale registration wears (the
caption lambda the percent/OFF words keep).
Usage: python3 scripts/jar_work/fov_caption_262.py <client.jar>
"""
import struct, sys, zipfile
sys.path.insert(0, "scripts/jar_work")

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"

# reuse the walk helpers from wall_684b via exec of its shared core
import importlib.util
spec = importlib.util.spec_from_file_location("probe_core", "scripts/jar_work/fov_scale_tail_262.py")

z = zipfile.ZipFile(JAR)

# --- OptionInstance$UnitDouble: list methods via the access probe's helpers
core = {}
exec(compile(open("scripts/jar_work/access_screen_262.py").read().split("z = zipfile.ZipFile(JAR)")[0], "core", "exec"), core)

dump_method = core["dump_method"]
methods_of = core["methods_of"]

CLS = "net.minecraft.client.gui.screens.options.AccessibilityOptionsScreen"

print("=== OptionInstance$UnitDouble (the record's own value word)")
dump_method(z, "net.minecraft.client.gui.screens.OptionInstance$UnitDouble", "toString")
dump_method(z, "net.minecraft.client.gui.screens.OptionInstance$UnitDouble", "xmap")

print()
print("=== Options clinit: the fovEffectScale registration window")
ms = methods_of(z, "net.minecraft.client.Options")
found = False
for sig, (cp, body) in sorted(ms.items()):
    if not body or "<clinit>" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = core["walk_code"](cp, code)
    for i, line in enumerate(lines):
        if "fovEffectScale" in line:
            lo = max(0, i - 40)
            hi = min(len(lines), i + 45)
            print(f"--- Options.{sig} window pcs {lines[lo].split(':')[0]}..{lines[hi-1].split(':')[0]}")
            for l in lines[lo:hi]:
                print(l)
            found = True
if not found:
    print("!! no fovEffectScale window in clinit")
