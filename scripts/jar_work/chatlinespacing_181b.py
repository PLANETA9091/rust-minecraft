#!/usr/bin/env python3
"""THE CHAT LINE SPACING'S OWN ADJACENCY PROBE (autopush-181b): the
processOptions visit seat the @493 ldc keeps, the en_us lang row the
caption speaks, the indy@622 shared caption lambda the UnitDouble rows
wear (chatOpacity + lineSpacing + textbg the same call site keeps).
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

cp, bs = core["method_bodies"](z, "net.minecraft.client.Options")


def find(bs, nm):
    for (n, d), b in bs.items():
        if n == nm:
            return b
    return None


# ---- 1. the processOptions visit seat (the @480/@493/@506/@523
#         ladder the save row's order wears) ----
b3 = find(bs, "processOptions")
clen3 = struct.unpack_from(">I", b3, 4)[0]
print("== Options.processOptions — pcs 460-545 (the chat ladder) ==")
for pc, l in core["disasm"](cp, b3[8:8 + clen3]):
    if 460 <= pc <= 545:
        print(f"   {pc}: {l}")
print()

# ---- 2. the indy@622 bootstrap (the shared caption lambda) ----
e = cp[622]
print(f"indy@622 tag={e[0]} bsm={e[1]} nat={e[2]}")
nt = cp[e[2]]
print(f"  name={cp[nt[1]][1]!r} desc={cp[nt[2]][1]!r}")
# the bootstrap method: the class's BootstrapMethods attribute — the
# bsm index the attr keeps; the MethodHandles metafactory the handle
# wears. The impl method the CP the lambda body speaks: the Options
# class's private synthetic lambda$new$N the indy@622 targets.
# Resolve via the MethodHandle chain: bsm index -> BootstrapMethods.
# Cheaper path: scan the Options methods for the synthetic lambda
# whose descriptor matches the caption law (Object,Object)->Object
# and whose body calls the UnitDouble label walk.
for (nm, desc), b in sorted(bs.items()):
    if "lambda" not in nm or b is None:
        continue
    clen = struct.unpack_from(">I", b, 4)[0]
    code = b[8:8 + clen]
    ls = [l for _, l in core["disasm"](cp, code)]
    if any("UnitDouble" in l or "Math.round" in l or "ireturn" in l for l in ls):
        print(f"== {nm} {desc} ==")
        for l in ls[:40]:
            print("   ", l)
        print()
print()

# ---- 3. the en_us lang rows ----
import json
raw = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
tbl = json.loads(raw)
for k in ["options.chat.line_spacing", "options.chat.opacity",
          "options.chat.visibility", "options.chat.height", "options.chat.width"]:
    print(f"   {k} = {tbl.get(k)!r}")
