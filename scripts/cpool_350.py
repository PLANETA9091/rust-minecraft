#!/usr/bin/env python3
"""Parse a .class file constant pool + fields + static int values.

Reads the constant pool entries (utf8/ints) and the declared fields of a
javac-compiled class, enough to verify enum constant names and integer
literals (the DataFixTypes.DEBUG_PROFILE = 4649 evidence lane).
"""
import struct
import sys


def parse(path):
    data = open(path, "rb").read()
    off = 8  # skip magic + version
    count = struct.unpack_from(">H", data, off)[0]
    off += 2
    pool = [None] * count  # 1-indexed
    i = 1
    while i < count:
        tag = data[off]
        off += 1
        if tag == 1:  # utf8
            ln = struct.unpack_from(">H", data, off)[0]
            off += 2
            pool[i] = ("utf8", data[off : off + ln].decode("utf-8", "replace"))
            off += ln
        elif tag in (7, 8, 16, 19, 20):  # class/string/method type/module/pkg
            pool[i] = ("ref", struct.unpack_from(">H", data, off)[0])
            off += 2
        elif tag == 15:  # method handle
            pool[i] = ("mh", data[off], struct.unpack_from(">H", data, off + 1)[0])
            off += 3
        elif tag in (3, 4):  # int/float
            pool[i] = ("num", struct.unpack_from(">I", data, off)[0])
            off += 4
        elif tag in (5, 6):  # long/double
            pool[i] = ("num8", struct.unpack_from(">Q", data, off)[0])
            off += 8
            i += 1  # takes two slots
        elif tag in (9, 10, 11, 12, 17, 18):  # member refs
            pool[i] = ("ref2",) + struct.unpack_from(">HH", data, off)
            off += 4
        else:
            raise ValueError(f"tag {tag} at {i}")
        i += 1

    def utf(idx):
        e = pool[idx]
        return e[1] if e and e[0] == "utf8" else "?"

    def resolve(idx, depth=0):
        e = pool[idx] if 0 < idx < len(pool) else None
        if e is None:
            return "?"
        if e[0] == "utf8":
            return e[1]
        if e[0] == "ref":
            return resolve(e[1], depth + 1) if depth < 4 else "..."
        if e[0] == "ref2":
            a = resolve(e[1], depth + 1)
            b = resolve(e[2], depth + 1)
            return f"{a}.{b}"
        if e[0] == "mh":
            return f"mh({e[1]},{resolve(e[2])})"
        return str(e)

    # access flags, this, super
    off += 6
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2
    interfaces = []
    for _ in range(ifc):
        interfaces.append(utf(struct.unpack_from(">H", data, off)[0]))
        off += 2

    def fields_or_methods():
        nonlocal off
        n = struct.unpack_from(">H", data, off)[0]
        off += 2
        out = []
        for _ in range(n):
            acc, name, desc = struct.unpack_from(">HHH", data, off)
            off += 6
            attrs = struct.unpack_from(">H", data, off)[0]
            off += 2
            const_val = None
            code_len = 0
            for _ in range(attrs):
                if off + 2 > len(data):
                    raise ValueError(f"attr name OOB at member {name} off {off}")
                aname = utf(struct.unpack_from(">H", data, off)[0])
                off += 2
                alen = struct.unpack_from(">I", data, off)[0]
                off += 4
                body = data[off : off + alen]
                off += alen
                if aname == "ConstantValue":
                    const_val = resolve(struct.unpack_from(">H", body, 0)[0])
                if aname == "Code":
                    code_len = alen
            out.append((acc, name, desc, const_val, code_len))
        return out

    fields = fields_or_methods()
    return interfaces, fields, pool


if __name__ == "__main__":
    interfaces, fields, pool = parse(sys.argv[1])

    def utf(idx):
        e = pool[idx] if 0 < idx < len(pool) else None
        return e[1] if e and e[0] == "utf8" else f"?{idx}"

    print("interfaces:", interfaces)
    print("fields:")
    for acc, name, desc, cv, code_len in fields:
        flags = []
        if acc & 0x0008:
            flags.append("static")
        if acc & 0x0010:
            flags.append("final")
        cv_s = f" = {cv}" if cv is not None else ""
        print(f"  {' '.join(flags)} {utf(name)}: {utf(desc)}{cv_s}")
