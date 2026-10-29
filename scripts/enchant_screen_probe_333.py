#!/usr/bin/env python3
"""the enchanting SCREEN's own probe (the 26.2 jar serves): the
EnchantmentScreen chain the superclass field speaks, the ctor's const
stream the imageWidth/imageHeight/title coords bake, the bg PNG's own
dimensions the blit wears."""
import struct
import zipfile

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")


def class_info(name):
    data = z.read(name + ".class")
    u2 = lambda o: struct.unpack(">H", data[o : o + 2])[0]
    u4 = lambda o: struct.unpack(">I", data[o : o + 4])[0]
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

    # walk to class level
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
    this_idx = u2(o + 2)
    sup_idx = u2(o + 4)
    print(f"{name}:\n  extends {utf(sup_idx)}")
    # ctor const stream
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
    for _ in range(mc):
        nm_i = u2(o2 + 2)
        o2 += 6
        ac = u2(o2)
        o2 += 2
        is_init = utf(nm_i) == "<init>"
        for _ in range(ac):
            an_i = u2(o2)
            alen = u4(o2 + 2)
            o2 += 6
            if is_init and cp.get(an_i) == b"Code":
                cl = u4(o2 + 4)
                code = data[o2 + 8 : o2 + 8 + cl]
                consts = []
                p = 0
                while p < len(code):
                    op = code[p]
                    if op == 0x10:
                        v = code[p + 1]
                        consts.append(v - 256 if v > 127 else v)
                        p += 2
                    elif op == 0x11:
                        consts.append(struct.unpack(">h", code[p + 1 : p + 3])[0])
                        p += 3
                    elif op == 0x12:
                        e = cp.get(code[p + 1])
                        if isinstance(e, tuple) and e[0] == "MEM":
                            pass
                        p += 2
                    elif op in (16, 17, 18):
                        p += 3 if op == 17 else 2
                    elif op in (21, 22, 23, 24, 25, 54, 55, 56, 57, 58, 132, 169, 188, 189, 192, 193):
                        p += 2
                    elif op in (
                        179, 180, 181, 182, 183, 184, 185, 186, 187, 190, 191, 192, 193, 195, 198, 199, 200, 201,
                    ):
                        p += 3
                    elif op == 196:
                        p += 6 if code[p + 1] == 132 else 4
                    elif op == 197:
                        p += 4
                    elif op == 170:
                        base = (p + 1 + 3) & ~3
                        lo = struct.unpack(">i", code[base : base + 4])[0]
                        hi = struct.unpack(">i", code[base + 4 : base + 8])[0]
                        p = base + 12 + (hi - lo + 1) * 4
                    elif op == 171:
                        base = (p + 1 + 3) & ~3
                        np = struct.unpack(">i", code[base + 4 : base + 8])[0]
                        p = base + 8 + np * 8
                    else:
                        p += 1
                    if p > len(code):
                        break
                print(f"  <init> consts: {consts}")
            o2 += alen


class_info("net/minecraft/client/gui/screens/inventory/EnchantmentScreen")

png = z.read("assets/minecraft/textures/gui/container/enchanting_table.png")
w, h = struct.unpack(">II", png[16:24])
print(f"enchanting_table.png {w}x{h}")
