#!/usr/bin/env python3
"""THE CHAT TEXT'S OWN OPACITY PROBE (autopush-178): the ChatComponent$1
anonymous class the extractRenderState fstore-12 walk serves — the glyph
alpha the captured `chatOpacity * 0.9f + 0.1f` base the per-line fade
multiplies (the drawString lane the client's text_faded alpha keeps),
plus the Options.save key order the chatOpacity row's file seat wears.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

# 1) the anonymous inner class's own walk (the text draw the captured
#    fstore-12 base the per-line fade multiplies)
for cls in [
    "net.minecraft.client.gui.components.ChatComponent$1",
    "net.minecraft.client.gui.components.ChatComponent$1$1",
]:
    got = core["method_bodies"](z, cls)
    if got is None:
        print("!! no", cls)
        continue
    cp, bodies = got
    for (nm, desc), body in sorted(bodies.items()):
        if body is None:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = core["disasm"](cp, code)
        except Exception as e:
            print(cls, nm, "FAIL", repr(e))
            continue
        print("==", cls, "::", nm, desc)
        for _, l in lines:
            print("   ", l)
        print()

# 2) the Options.save key order (the chatOpacity row's file seat the
#    write walk wears — the ldc key constants the method speaks)
cp2, bodies2 = core["method_bodies"](z, "net.minecraft.client.Options")
body = bodies2.get(("save", "()V"))
if body is None:
    # fallback: any method whose name holds save
    for (nm, desc), b in sorted(bodies2.items()):
        if "save" in nm:
            print("== candidate:", nm, desc)
            body = b
            break
if body is not None:
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = core["disasm"](cp2, code)
    print("== Options.save ()V — the FULL walk (the getters + the indy recipes the row order wears) ==")
    for pc, l in lines:
        s = l.strip()
        if ("invokevirtual" in s and "Options." in s) or "indy" in s or "makeConcat" in s or "println" in s or "version" in s.lower():
            print("    ", s)

# 3) the processOptions visit order (the file rows the visitor wears —
#    the ldc key + the getter pairs in walk order)
for (nm, desc), body in sorted(bodies2.items()):
    if nm != "processOptions":
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = core["disasm"](cp2, code)
    print("== Options.processOptions — the visit order (the ldc keys + the getters) ==")
    hits = []
    for pc, l in lines:
        s = l.strip()
        if ("ldc" in s and "cp(8," in s) or ("invokevirtual" in s and "Options." in s):
            hits.append("    " + s)
    # the keys the utf8 constants hold: resolve cp(8, N) utf8s
    for h in hits:
        print(h)
    # resolve the utf8 values the ldc rows point to
    print("== the utf8 constants the ldc rows wear (the key strings) ==")
    import re
    for pc, l in lines:
        s = l.strip()
        m = re.search(r"cp\(8, (\d+)\)", s)
        if m and "ldc" in s:
            idx = int(m.group(1))
            e = cp2[idx]
            val = e[1] if e and e[0] == "u" else f"<{e}>"
            print("    ", pc, "ldc ->", repr(val)[:70])


