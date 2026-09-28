#!/usr/bin/env python3
"""Dump the LevelEvent constants class rows by value (autopush-461).

The constants class (net/minecraft/world/level/block/LevelEvent or
the 26.2 equivalent) holds `public static final int` rows the
lookupswitch words name. Walk its static fields, resolve
ConstantValue attributes, print rows for the 1000-band sound words.
"""
import struct
import zipfile
import sys

JAR = "/home/z/my-project/client262.jar"

def find_constants_class(z):
    for n in z.namelist():
        if n.endswith("LevelEvent.class"):
            return n
    return None

def parse_class(raw):
    off = 8
    (cp_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    pool = {}
    i = 1
    while i < cp_count:
        tag = raw[off]
        if tag == 1:
            (ln,) = struct.unpack_from(">H", raw, off + 1)
            pool[i] = ("utf8", raw[off + 3 : off + 3 + ln])
            off += 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            pool[i] = ("ref", struct.unpack_from(">H", raw, off + 1)[0])
            off += 3
        elif tag == 15:
            pool[i] = ("mh", None)
            off += 4
        elif tag == 3:
            pool[i] = ("int", struct.unpack_from(">i", raw, off + 1)[0])
            off += 5
        elif tag == 4:
            pool[i] = ("float", None)
            off += 5
        elif tag in (5, 6):
            pool[i] = ("num8", None)
            off += 9
            i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            pool[i] = ("ref2", struct.unpack_from(">HH", raw, off + 1))
            off += 5
        else:
            raise SystemExit(f"tag {tag}")
        i += 1
    def utf8(idx):
        kind, val = pool[idx]
        if kind == "ref":
            return pool[val][1].decode("utf-8", "replace")
        return val.decode("utf-8", "replace")
    off += 6  # access + this + super
    (ifc_count,) = struct.unpack_from(">H", raw, off)
    off += 2 + 2 * ifc_count
    (fld_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    fields = []
    for _ in range(fld_count):
        f_acc, f_name, f_desc = struct.unpack_from(">HHH", raw, off)
        off += 6
        (attr_count,) = struct.unpack_from(">H", raw, off)
        off += 2
        const = None
        for _ in range(attr_count):
            (an, alen) = struct.unpack_from(">HI", raw, off)
            if utf8(an) == "ConstantValue":
                (ci,) = struct.unpack_from(">H", raw, off + 6)
                kind, val = pool[ci]
                if kind == "int":
                    const = val
            off += 6 + alen
        if f_acc & 0x0018 and const is not None:  # static final
            fields.append((utf8(f_name), const))
    return fields

def main():
    with zipfile.ZipFile(JAR) as z:
        name = find_constants_class(z)
        print("class:", name)
        fields = parse_class(z.read(name))
    for n, v in sorted(fields, key=lambda p: p[1]):
        if 1000 <= v <= 1060:
            print(f"    {v:5d} {n}")

if __name__ == "__main__":
    sys.exit(main())
