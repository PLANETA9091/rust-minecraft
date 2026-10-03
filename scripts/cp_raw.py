#!/usr/bin/env python3
"""Dump raw constant-pool entries for one class (raw tags + float decode).

usage: python3 scripts/cp_raw.py <jar> <net/minecraft/Class> <idx> [idx...]
"""
import struct
import sys
import zipfile


def main() -> None:
    jar_path, cls = sys.argv[1], sys.argv[2]
    idxs = [int(a) for a in sys.argv[3:]] or None
    data = zipfile.ZipFile(jar_path).read(f"{cls}.class")
    count = struct.unpack_from(">H", data, 8)[0]
    i = 10
    idx = 1
    while idx < count:
        tag = data[i]
        if tag == 1:
            ln = struct.unpack_from(">H", data, i + 1)[0]
            val = data[i + 3 : i + 3 + ln].decode("utf-8", "replace")
            size = 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            val = struct.unpack_from(">H", data, i + 1)[0]
            size = 3
        elif tag == 15:
            val = (data[i + 1], struct.unpack_from(">H", data, i + 2)[0])
            size = 4
        elif tag in (17, 18):
            val = struct.unpack_from(">HH", data, i + 1)
            size = 5
        elif tag == 3:
            val = struct.unpack(">i", struct.pack(">I", struct.unpack_from(">I", data, i + 1)[0]))[0]
            size = 5
        elif tag == 4:
            raw = struct.unpack_from(">I", data, i + 1)[0]
            val = ("float", struct.unpack(">f", struct.pack(">I", raw))[0])
            size = 5
        elif tag == 5:
            val = struct.unpack_from(">Q", data, i + 1)[0]
            size = 9
            idx += 1  # long/double take two slots
        elif tag == 6:
            raw = struct.unpack_from(">Q", data, i + 1)[0]
            val = ("double", struct.unpack(">d", struct.pack(">Q", raw))[0])
            size = 9
            idx += 1
        elif tag == 9:
            val = ("field",) + struct.unpack_from(">HH", data, i + 1)
            size = 5
        elif tag == 10:
            val = ("method",) + struct.unpack_from(">HH", data, i + 1)
            size = 5
        elif tag == 11:
            val = ("iface",) + struct.unpack_from(">HH", data, i + 1)
            size = 5
        elif tag == 12:
            val = ("name_type",) + struct.unpack_from(">HH", data, i + 1)
            size = 5
        else:
            raise ValueError(f"unknown cp tag {tag} at index {idx}")
        if idxs is None or idx in idxs:
            print(idx, tag, val)
        i += size
        idx += 1


if __name__ == "__main__":
    main()
