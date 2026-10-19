#!/usr/bin/env python3
"""dev-83 follow-up probe: the WallSkullBlock clinit double constants
(ldc2_w #179/#181) + the Block.boxZ descriptor."""
import struct, zipfile

JAR = "tools/client.jar"

def load(cls):
    with zipfile.ZipFile(JAR) as z:
        return z.read(cls + ".class")

class CF:
    def __init__(self, data):
        self.d = data
        self.cp = self._cp()

    def _cp(self):
        d = self.d
        n = struct.unpack_from(">H", d, 8)[0]
        idx, cp, i = 10, {}, 1
        while i < n:
            tag = d[idx]
            if tag == 1:
                ln = struct.unpack_from(">H", d, idx+1)[0]
                cp[i] = ("utf8", d[idx+3:idx+3+ln].decode("utf8", "replace")); idx += 3+ln
            elif tag == 7: cp[i] = ("class", struct.unpack_from(">H", d, idx+1)[0]); idx += 3
            elif tag in (9,10,11,12,17): cp[i] = ("ref", struct.unpack_from(">HH", d, idx+1)); idx += 5
            elif tag == 8: cp[i] = ("str", struct.unpack_from(">H", d, idx+1)[0]); idx += 3
            elif tag in (3,4): cp[i] = ("num", struct.unpack_from(">I", d, idx+1)[0]); idx += 5
            elif tag in (5,6):
                raw = d[idx+1:idx+9]
                if tag == 5:
                    cp[i] = ("long", struct.unpack(">q", raw)[0])
                else:
                    cp[i] = ("dbl", struct.unpack(">d", raw)[0])
                idx += 9; i += 1
            elif tag == 15: cp[i] = ("mh", d[idx+1], struct.unpack_from(">H", d, idx+2)[0]); idx += 4
            elif tag == 16: cp[i] = ("pt", struct.unpack_from(">H", d, idx+1)[0]); idx += 3
            elif tag == 18: cp[i] = ("indy", struct.unpack_from(">HH", d, idx+1)); idx += 5
            elif tag in (19, 20): cp[i] = ("modpkg", struct.unpack_from(">H", d, idx+1)[0]); idx += 3
            else: raise SystemExit(f"tag {tag}")
            i += 1
        return cp

    def utf(self, i):
        e = self.cp.get(i)
        return e[1] if e and e[0] == "utf8" else None

    def cls(self, i):
        e = self.cp.get(i)
        return self.utf(e[1]) if e and e[0] == "class" else None

cf = CF(load("net/minecraft/world/level/block/WallSkullBlock"))
print("cp 179:", cf.cp.get(179))
print("cp 181:", cf.cp.get(181))

for i, e in cf.cp.items():
    if e[0] == "ref":
        cls_idx, nat_idx = e[1]
        nt = cf.cp[nat_idx]
        try:
            name_idx, desc_idx = nt[1]
        except Exception:
            continue
        nm = cf.utf(name_idx)
        if nm in ("boxZ", "rotateHorizontal"):
            print(nm, "ref:", cf.cls(cls_idx), cf.utf(desc_idx))
