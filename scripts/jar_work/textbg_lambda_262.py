#!/usr/bin/env python3
"""THE PER-LINE BG COMPOSITION PROBE (autopush-177): the forEachLine
lambda the extractRenderState indy@287 serves — the textbg the fade
multiplier's own composition the per-line fill keeps (the ChatComponent
lambda$ methods the ARGB.black / opacity rides wear).
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
cp, bodies = core["method_bodies"](z, "net.minecraft.client.gui.components.ChatComponent")
for (nm, desc), body in sorted(bodies.items()):
    if "lambda" not in nm or body is None:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = core["disasm"](cp, code)
        text = "\n".join(l for _, l in lines)
    except Exception as e:
        print(nm, desc, "FAIL", repr(e))
        continue
    if "black" in text or "Opacity" in text or "opacity" in text or "AlphaCalculator" in text:
        print("==", nm, desc)
        for _, l in lines:
            print("   ", l)
        print()

# the AlphaCalculator's own record (the fade law the client's
# chat_line_alpha keeps — the FULLY_VISIBLE / timeBased pair)
for cls in ["net.minecraft.client.gui.components.ChatComponent$AlphaCalculator"]:
    cp2, bodies2 = core["method_bodies"](z, cls)
    if bodies2 is None:
        print("!! no AlphaCalculator")
        continue
    for (nm, desc), body in sorted(bodies2.items()):
        if body is None:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = core["disasm"](cp2, code)
        except Exception as e:
            print(cls, nm, "FAIL", repr(e))
            continue
        print("==", cls, "::", nm, desc)
        for _, l in lines:
            print("   ", l)
        print()
