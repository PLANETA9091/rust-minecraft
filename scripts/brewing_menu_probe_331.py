#!/usr/bin/env python3
"""the brewing menu's own geometry probe (the 26.2 jar the client.jar
serves — the BrewingStandMenu ctor's Code attribute the constants
stream bakes): the full JVM opcode-length table the walk wears, the
<init> the bipush/sipush stream prints — the slot x/y constants the
vanilla ctor speaks."""
import struct
import zipfile

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
data = z.read("net/minecraft/world/inventory/BrewingStandMenu.class")


def u2(o):
    return struct.unpack(">H", data[o : o + 2])[0]


def u4(o):
    return struct.unpack(">I", data[o : o + 4])[0]


cp_count = u2(8)
o = 10
cp = {}
i = 1
while i < cp_count:
    tag = data[o]
    if tag == 1:
        ln = u2(o + 1)
        cp[i] = data[o + 3 : o + 3 + ln]
        o += 3 + ln
    elif tag in (7, 8, 16, 19, 20):
        cp[i] = ("REF", u2(o + 1))
        o += 3
    elif tag == 3:
        cp[i] = ("INT", struct.unpack(">i", data[o + 1 : o + 5])[0])
        o += 5
    elif tag == 4:
        cp[i] = ("FLT",)
        o += 5
    elif tag in (5, 6):
        cp[i] = ("LNG",)
        o += 9
        i += 1
    elif tag in (9, 10, 11, 12, 17, 18):
        cp[i] = ("MEM", u2(o + 1), u2(o + 3))
        o += 5
    elif tag == 15:
        cp[i] = ("MH",)
        o += 4
    else:
        raise SystemExit("tag %d" % tag)
    i += 1

o = 10
i = 1
while i < cp_count:
    tag = data[o]
    if tag == 1:
        o += 3 + u2(o + 1)
    elif tag in (7, 8, 16, 19, 20):
        o += 3
    elif tag in (3, 4):
        o += 5
    elif tag in (5, 6):
        o += 9
        i += 1
    elif tag in (9, 10, 11, 12, 17, 18):
        o += 5
    elif tag == 15:
        o += 4
    i += 1

ifc = u2(o + 6)
o += 8 + 2 * ifc
fc = u2(o)
o += 2
for f in range(fc):
    o += 6
    ac = u2(o)
    o += 2
    for a in range(ac):
        o += 6 + u4(o + 2)
mc = u2(o)
o += 2

# THE FULL JVM OPCODE LENGTH TABLE (the JVMS the instruction set
# speaks): the default 1, the exceptions the table banks
LEN = [1] * 202
for op in (16, 17, 18):  # bipush sipush ldc
    LEN[op] = 2 if op != 17 else 3
for op in (19, 20):  # ldc_w ldc2_w
    LEN[op] = 3
for op in (21, 22, 23, 24, 25, 54, 55, 56, 57, 58, 132, 169):  # load/store/iinc/ret
    LEN[op] = 2
for op in (47, 48, 49, 50):  # iaload family
    LEN[op] = 1
for op in (
    51,
    52,
    53,
    59,
    60,
    61,
    62,
    63,
    64,
    65,
    66,
    67,
    68,
    69,
    70,
    71,
    72,
    73,
    74,
    75,
    76,
    77,
    78,
):
    LEN[op] = 1
for op in (179, 180, 181, 182, 183, 184, 187, 188, 189, 190, 191, 192, 193, 195):
    LEN[op] = 3
LEN[185] = 5  # invokeinterface
LEN[186] = 5  # invokedynamic
LEN[196] = 4  # wide (the operand the wide's own rule wears — iinc rides 6)
LEN[197] = 4  # multianewarray
LEN[200] = 5  # goto_w
LEN[201] = 5  # jsr_w


def scan(code):
    consts = []
    p = 0
    n = len(code)
    while p < n:
        op = code[p]
        if op >= 202:
            raise SystemExit("opcode overrun at %d (op %d) — the table desynced" % (p, op))
        if op == 170:  # tableswitch
            base = (p + 1 + 3) & ~3
            lo = struct.unpack(">i", code[base : base + 4])[0]
            hi = struct.unpack(">i", code[base + 4 : base + 8])[0]
            p = base + 12 + (hi - lo + 1) * 4
            continue
        if op == 171:  # lookupswitch
            base = (p + 1 + 3) & ~3
            npairs = struct.unpack(">i", code[base + 4 : base + 8])[0]
            p = base + 8 + npairs * 8
            continue
        if op == 196:  # wide
            nxt = code[p + 1]
            p += 6 if nxt == 132 else 4
            continue
        if op == 16:
            v = code[p + 1]
            consts.append(v - 256 if v > 127 else v)
        elif op == 17:
            consts.append(struct.unpack(">h", code[p + 1 : p + 3])[0])
        elif op == 18:
            idx = code[p + 1]
            e = cp.get(idx)
            if isinstance(e, tuple) and e[0] == "INT":
                consts.append(e[1])
        p += LEN[op]
    return consts


for m in range(mc):
    macc = u2(o)
    nm_i = u2(o + 2)
    o += 6
    nm = cp[nm_i] if isinstance(cp[nm_i], bytes) else b"?"
    ac = u2(o)
    o += 2
    is_init = nm == b"<init>"
    for a in range(ac):
        an = u2(o)
        alen = u4(o + 2)
        o += 6
        attr = cp[an] if an in cp else None
        if is_init and isinstance(attr, bytes) and attr == b"Code":
            cl = u4(o)
            print("<init> const stream:", scan(data[o : o + cl]))
        o += alen
