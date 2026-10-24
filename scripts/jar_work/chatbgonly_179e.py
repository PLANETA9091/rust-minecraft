#!/usr/bin/env python3
"""THE LAWS' OWN CALLERS (autopush-179e): the scan_jar walk the
getBackgroundColor / getBackgroundOpacity refs the classes answer — each
caller's method the window prints.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)

for needle in ("getBackgroundColor", "getBackgroundOpacity"):
    print("#####", needle)
    hits = core["scan_jar"](z, needle)
    for cls, status, refs in hits:
        if status != "ok":
            continue
        # skip Options itself (the definer)
        if cls == "net.minecraft.client.Options":
            continue
        try:
            cp, bs = core["method_bodies"](z, cls)
        except Exception as ex:
            print(cls, "bodies fail", ex)
            continue
        if bs is None:
            print(cls, "no bodies")
            continue
        for (nm, desc), b in sorted(bs.items()):
            if b is None:
                continue
            txt = "\n".join(l for _, l in dis(cp, b))
            if needle in txt:
                print("==", cls, "::", nm, desc)
                for pc, l in dis(cp, b):
                    print("   %4d: %s" % (pc, l))
                print()
