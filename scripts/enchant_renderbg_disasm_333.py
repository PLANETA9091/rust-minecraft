#!/usr/bin/env python3
"""the EnchantmentScreen's renderBg disasm (the 26.2 jar serves): the
method the 10345 sipush pair rides — the blit's own arg stack the walk
prints, the CP names resolved."""
import struct
import zipfile

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
data = z.read("net/minecraft/client/gui/screens/inventory/EnchantmentScreen.class")


def u2(o):
    return struct.unpack(">H", data[o : o + 2])[0]


def u4(o):
    return struct.unpack(">I", data[o : o + 4])[0]


cp_count = u2(8)
o = 10
i = 1
cp = {}
while i < cp_count:
    tag = data[o]
    if tag == 1:
        ln = u2(o + 1)
        cp[i] = data[o + 3 : o + 3 + ln]
        o += 3 + ln
    elif tag in (7, 8, 16, 19, 20):
        cp[i] = ("REF", u2(o + 1))
        o += 3
    elif tag in (3, 4):
        o += 5
    elif tag in (5, 6):
        o += 9
        i += 1
    elif tag in (9, 10, 11, 12, 17, 18):
        cp[i] = ("MEM", u2(o + 1), u2(o + 3))
        o += 5
    elif tag == 15:
        o += 4
    else:
        raise SystemExit("tag %d" % tag)
    i += 1


def utf(ridx):
    r = cp[ridx]
    if isinstance(r, bytes):
        return r.decode("utf-8", "replace")
    if r[0] == "REF":
        return utf(r[1])
    return "?"


def mem(midx):
    r = cp[midx]  # (MEM, class_idx, nat_idx)
    cls = utf(r[1])
    nat = cp[r[2]]  # (MEM, name_idx, desc_idx) — reuse MEM shape
    return cls, utf(nat[1]), utf(nat[2])


# walk to methods
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
o2 = o + 6
ifc = u2(o2)
o2 += 2 + 2 * ifc
fc = u2(o2)
o2 += 2
for _ in range(fc):
    o2 += 6
    ac = u2(o2)
    o2 += 2
    for _ in range(ac):
        o2 += 6 + u4(o2 + 2)
mc = u2(o2)
o2 += 2
TARGET = 10345
for _ in range(mc):
    nm_i = u2(o2 + 2)
    o2 += 6
    ac = u2(o2)
    o2 += 2
    name = utf(nm_i)
    for _ in range(ac):
        an_i = u2(o2)
        alen = u4(o2 + 2)
        o2 += 6
        if cp.get(an_i) == b"Code" and o2 <= TARGET < o2 + alen:
            cl = u4(o2 + 4)
            print(f"method {name} code {cl} bytes")
            p = o2 + 8
            end = o2 + 8 + cl
            while p < end:
                op = data[p]
                off = p - (o2 + 8)
                if op in (0xB2, 0xB4, 0xB5):
                    idx = u2(p + 1)
                    kind = {0xB2: "getstatic", 0xB4: "getfield", 0xB5: "putfield"}[op]
                    r = cp[idx]
                    print(f"  {off:5d} {kind} {utf(r[1])}.{utf(cp[r[2]][1])}")
                    p += 3
                elif op in (0xB6, 0xB7, 0xB8):
                    idx = u2(p + 1)
                    kind = {0xB6: "invokevirtual", 0xB7: "invokespecial", 0xB8: "invokestatic"}[op]
                    c, n, d = mem(idx)
                    print(f"  {off:5d} {kind} {c}.{n}{d}")
                    p += 3
                elif op == 0x11:
                    v = struct.unpack(">h", data[p + 1 : p + 3])[0]
                    print(f"  {off:5d} sipush {v}")
                    p += 3
                elif op == 0x10:
                    v = data[p + 1]
                    v = v - 256 if v > 127 else v
                    print(f"  {off:5d} bipush {v}")
                    p += 2
                elif op in (0x1A, 0x1B, 0x1C, 0x1D):
                    print(f"  {off:5d} iload_{op-0x1A}")
                    p += 1
                elif op in (0x15,):
                    print(f"  {off:5d} iload {data[p+1]}")
                    p += 2
                elif op in (0x2A, 0x2B, 0x2C, 0x2D):
                    print(f"  {off:5d} aload_{op-0x2A}")
                    p += 1
                elif op in (0x4B, 0x4C, 0x4D, 0x4E):
                    print(f"  {off:5d} astore_{op-0x4B}")
                    p += 1
                elif op == 0x36:
                    print(f"  {off:5d} istore {data[p+1]}")
                    p += 2
                elif op in (0x03, 0x04, 0x05, 0x06, 0x07, 0x08):
                    print(f"  {off:5d} iconst_{op-0x03}")
                    p += 1
                elif op == 0x59:
                    print(f"  {off:5d} dup")
                    p += 1
                elif op == 0xB0:
                    print(f"  {off:5d} areturn")
                    p += 1
                elif op == 0xB1:
                    print(f"  {off:5d} return")
                    p += 1
                elif op == 0xA7:
                    print(f"  {off:5d} goto {struct.unpack('>h', data[p+1:p+3])[0]}")
                    p += 3
                else:
                    print(f"  {off:5d} op 0x{op:02x}")
                    p += 1
            break
        o2 += alen
    else:
        continue
    break
