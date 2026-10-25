#!/usr/bin/env python3
"""THE ADVANCED TOOLTIPS RENDER PROBE (the autopush-210 round): the
AbstractContainerScreen render tooltip branch (the hoveredSlot.hasItem
gate the render walk keeps) + the ItemStack.getTooltipLines advanced
rows (the ID row the DARK_GRAY wears, the item.durability row the
MAX_DAMAGE gate speaks) + the Item.toString law + the en_us lang rows.
Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/advtooltip_210.py
"""
import os, sys, zipfile, json

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; struct = NS["struct"]

JAR = os.path.join(os.path.dirname(HERE), "..", ".mc2rs", "classpath", "client.jar")
JAR = os.path.normpath(JAR)
z = zipfile.ZipFile(JAR)


def dump(cls, sig_prefix, lo=None, hi=None, label=""):
    meths = methods_of(z, cls)
    found = False
    for sig, (cp, body) in meths.items():
        if not sig.startswith(sig_prefix):
            continue
        found = True
        print(f"--- {cls} :: {sig} {label}")
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for ln in lines:
            try:
                pc = int(str(ln).split(":")[0].strip())
            except ValueError:
                print("   ", ln)
                continue
            if lo is not None and pc < lo:
                continue
            if hi is not None and pc > hi:
                continue
            print(f"    {ln}")
    if not found:
        print(f"--- {cls} :: NO method starting {sig_prefix}")


print("=" * 72)
print("[1] AbstractContainerScreen.render — the tooltip branch (the")
print("    hoveredSlot.hasItem gate + the renderTooltip invoke):")
meths = methods_of(z, "net/minecraft/client/gui/screens/inventory/AbstractContainerScreen")
for sig, (cp, body) in meths.items():
    if sig.startswith("render(Lnet/minecraft/client/gui/GuiGraphics;I"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        pcs = []
        for ln in lines:
            try:
                pc = int(str(ln).split(":")[0].strip())
            except ValueError:
                continue
            pcs.append((pc, str(ln)))
        total = pcs[-1][0] if pcs else 0
        print(f"    ({sig}, code_len_pc={total})")
        # find the tooltip-relevant rows
        for pc, s in pcs:
            if any(k in s for k in ("tooltip", "Tooltip", "hoveredSlot", "hasItem",
                                    "invokevirtual #", "renderTooltip")):
                print(f"    {pc}: {s.split(': ', 1)[1] if ': ' in s else s}")
print()
print("[2] AbstractContainerScreen.renderTooltip override? +")
print("    supertip helpers (the method list the class owns):")
for sig in sorted(meths):
    if "ooltip" in sig or "ovedSlot" in sig:
        print(f"    {sig}")
print()
print("[3] ItemStack.getTooltipLines — the ADVANCED rows (the flag")
print("    gate, the ID row, the durability row):")
meths = methods_of(z, "net/minecraft/world/item/ItemStack")
for sig, (cp, body) in meths.items():
    if sig.startswith("getTooltipLines"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        print(f"    ({sig})")
        for ln in lines:
            print("    ", ln)
print()
print("[4] Item.toString — the registry-name law the ID row reads:")
meths = methods_of(z, "net/minecraft/world/item/Item")
for sig, (cp, body) in meths.items():
    if sig.startswith("toString()"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"    ({sig})")
        for ln in walk_code(cp, code):
            print("    ", ln)
print()
print("[5] the cp strings the getTooltipLines walk reads (the lang keys):")
for sig, (cp, body) in meths.items():
    if sig.startswith("getTooltipLines"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        seen = set()
        for ln in walk_code(cp, code):
            s = str(ln)
            if "ldc" in s:
                # resolve the cp index through the walk's own text
                idx_txt = s.split("#")[-1].strip().split(" ")[0]
                try:
                    idx = int(idx_txt)
                except ValueError:
                    continue
                try:
                    row = utf(cp, cp[idx][1])
                    if row not in seen:
                        seen.add(row)
                        print(f"    str@{idx}: {row!r}")
                except Exception:
                    pass
print()
print("[6] the en_us lang rows the tooltip family reads:")
lang = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
tbl = json.loads(lang)
for k in sorted(tbl):
    if "durability" in k or k.startswith("item.durability"):
        print(f"    {k} = {tbl[k]}")
print()
print("[7] TooltipFlag — the ADVANCED constant the flag gate reads:")
meths = methods_of(z, "net/minecraft/world/item/TooltipFlag")
for sig, (cp, body) in meths.items():
    if sig.startswith("isAdvanced"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"    ({sig})")
        for ln in walk_code(cp, code):
            print("    ", ln)
