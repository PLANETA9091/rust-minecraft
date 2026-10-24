#!/usr/bin/env python3
"""THE DEBUG PROFILING CHART'S OWN WALK (dev-166): the FIVE-arg
overload the pc 3632-3652 window the `ldc_w cp#1506` the
`key.debug.profilingChart` name answers, the `KEYSYM, bipush 49` the
N default wears, the `iconst_1` the orderI arg the 5-arg ctor banks,
the `putfield keyDebugPofilingChart` the vanilla TYPO'D field the
seat speaks (the missing r the honest name keeps). THE ORDER LAW:
the KeyMapping fields the int the ctor stores — the readers the
KeyBindsList sort walk answers (the orderI the within-category
ordering the list wears, or the registration-order law the seat
keeps). The en_us lang row the display word keeps.

Usage: python3 scripts/jar_work/profilingchart_166.py <client.jar>
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

print("########## I. the KeyMapping 5-arg ctor (the orderI the field the store answers)")
d = dict(methods_of(z, "net.minecraft.client.KeyMapping"))
for sig, (cp, body) in d.items():
    if sig.startswith("<init>") and sig.count(";") >= 3 and "I)" in sig:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"--- {sig}")
        for t in walk_code(cp, code):
            print(f"  {t}")
        print()

print("########## II. the KeyMapping fields (the int the name wears)")
import re
data = z.read("net/minecraft/client/KeyMapping.class")
# minimal field walk: parse the class file's field_count
magic, minor, major = struct.unpack_from(">HHI", data, 0)
cpcount = struct.unpack_from(">H", data, 8)[0]
off = 10
cp = {}
i = 1
while i < cpcount:
    tag = data[off]
    if tag == 1:
        l = struct.unpack_from(">H", data, off + 1)[0]
        cp[i] = (tag, data[off + 3 : off + 3 + l])
        off += 3 + l
    elif tag in (7, 8, 16, 19, 20):
        cp[i] = (tag, struct.unpack_from(">H", data, off + 1)[0])
        off += 3
    elif tag == 15:
        cp[i] = (tag, struct.unpack_from(">HB", data, off + 1)[0])
        off += 4
    elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
        cp[i] = (tag, struct.unpack_from(">I", data, off + 1)[0])
        off += 5
    elif tag in (5, 6):
        cp[i] = (tag, struct.unpack_from(">Q", data, off + 1)[0])
        off += 9
        i += 1
    else:
        print(f"  tag {tag}?")
        break
    i += 1
access, this_c, super_c = struct.unpack_from(">HHH", data, off)
off += 6
icount = struct.unpack_from(">H", data, off)[0]
off += 2 + 2 * icount
fcount = struct.unpack_from(">H", data, off)[0]
off += 2

def u(cpi):
    return cp[cpi][1].decode("utf-8", "replace") if cp[cpi][0] == 1 else f"#{cpi}"

for _ in range(fcount):
    facc, fn, fd = struct.unpack_from(">HHH", data, off)
    off += 6
    acount = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _a in range(acount):
        alen = struct.unpack_from(">I", data, off)[0]
        off += 2 + alen
    if fd == "I":
        print(f"  int field: {u(fn)}")
print()

print("########## III. the KeyBindsList ctor (the sort the orderI the walk answers)")
d2 = dict(methods_of(z, "net.minecraft.client.gui.screens.options.controls.KeyBindsList"))
for sig, (cp, body) in d2.items():
    if sig.startswith("<init>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"--- {sig}")
        for t in walk_code(cp, code):
            if any(k in t for k in ("sort", "Sort", "orderI", "invoke", "field", "getstatic", "invokestatic", "ldc")):
                print(f"  {t}")
        print()
