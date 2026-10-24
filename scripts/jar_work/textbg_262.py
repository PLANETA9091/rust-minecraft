#!/usr/bin/env python3
"""THE TEXT BACKGROUND OPACITY'S OWN PROBE (the autopush-177 round):
the chat backgrounds' own option the accessibility screen's array
(index 4) wears — the Options.<init> registration window (the label
key + the codec/range the default 0.5 keeps), the
Options.getBackgroundColor(F)I composition the ChatScreen's input
band arg speaks, and the ChatComponent.render walk the consumer lanes
keep (which fills the textBackgroundOpacity rides, which the
chatOpacity's own 0.9/0.1 fold wears).

Usage: python3 scripts/jar_work/textbg_262.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"

# reuse the walked helpers from the access-screen probe (the
# MNEMONIC table + parse_cp + walk_code + methods_of + dump_method
# the javap-less lane keeps)
core = {}
exec(compile(open("scripts/jar_work/access_screen_262.py").read().split("z = zipfile.ZipFile(JAR)")[0], "core", "exec"), core)

walk_code = core["walk_code"]
methods_of = core["methods_of"]
dump_method = core["dump_method"]

z = zipfile.ZipFile(JAR)


def windows(z, cls, init_sig, needle, before=35, after=50):
    ms = methods_of(z, cls)
    for sig, (cp, body) in sorted(ms.items()):
        if init_sig not in sig or not body:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for i, line in enumerate(lines):
            if needle in line:
                lo = max(0, i - before)
                hi = min(len(lines), i + after)
                print(f"--- {cls}.{sig} window pcs {lines[lo].split(':')[0]}..{lines[hi-1].split(':')[0]}")
                for l in lines[lo:hi]:
                    print(l)
                print()
                return
    print(f"!! no `{needle}` window in {cls}{init_sig}")


print("=== 1. Options.<init>: the TEXT_BACKGROUND_OPACITY registration")
windows(z, "net.minecraft.client.Options", "<init>", "textBackgroundOpacity")

print("=== 2. Options.<init>: the chatOpacity registration (the text-alpha lane's own option)")
windows(z, "net.minecraft.client.Options", "<init>", "chatOpacity", before=10, after=40)

print("=== 3. Options.getBackgroundColor — the composition law")
dump_method(z, "net.minecraft.client.Options", "getBackgroundColor")

print("=== 4. ChatScreen — the input band's own arg")
for cls in ["net.minecraft.client.gui.screens.ChatScreen"]:
    ms = methods_of(z, cls)
    for sig, (cp, body) in sorted(ms.items()):
        if not body:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for i, line in enumerate(lines):
            if "getBackgroundColor" in line or "0x80000000" in line or "fill" in line.lower():
                lo = max(0, i - 8)
                hi = min(len(lines), i + 8)
                print(f"--- {cls}.{sig} window pcs {lines[lo].split(':')[0]}..{lines[hi-1].split(':')[0]}")
                for l in lines[lo:hi]:
                    print(l)
                print()
                break

print("=== 5. ChatComponent — the consumer walk (which lanes the options ride)")
CC = "net.minecraft.client.gui.components.ChatComponent"
ms = methods_of(z, CC)
for sig, (cp, body) in sorted(ms.items()):
    if not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = walk_code(cp, code)
    except Exception as e:
        print(f"--- {CC}.{sig}: walk failed ({e!r}) — raw scan instead")
        raw = code.hex()
        for word in [b"chatOpacity", b"textBackgroundOpacity", b"getBackgroundColor"]:
            if word.hex() in raw:
                print(f"    raw utf8 present: {word.decode()}")
        continue
    hits = [i for i, l in enumerate(lines)
            if "chatOpacity" in l or "textBackgroundOpacity" in l
            or "getBackgroundColor" in l or "chatLineSpacing" in l]
    if hits:
        print(f"--- {CC}.{sig}: {len(hits)} hit(s)")
        shown = set()
        for i in hits:
            lo = max(0, i - 14)
            hi = min(len(lines), i + 16)
            key = lo // 10
            if key in shown:
                continue
            shown.add(key)
            for l in lines[lo:hi]:
                print(l)
            print("   ...")
        print()
