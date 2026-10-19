#!/usr/bin/env python3
"""TintedGlassBlock 26.2 probe: methods table + full bytecode of
getLightBlock/skipRendering (the same-kind cull + the light-block law),
against GlassBlock's skipRendering. Utf8/ref args resolved per pc."""
import struct, zipfile, sys

JAR = "tools/client.jar"

def load(cls):
    with zipfile.ZipFile(JAR) as z:
        return z.read(cls + ".class")

class CF:
    def __init__(self, data):
        self.d = data
        self.cp = self._cp()

    def u1(self, o): return self.d[o]
    def u2(self, o): return struct.unpack_from(">H", self.d, o)[0]
    def u4(self, o): return struct.unpack_from(">I", self.d, o)[0]

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
            elif tag in (5,6): cp[i] = ("long", None); idx += 8; i += 1
            elif tag == 15: cp[i] = ("mh", d[idx+1], struct.unpack_from(">H", d, idx+2)[0]); idx += 4
            elif tag == 16: cp[i] = ("pt", struct.unpack_from(">H", d, idx+1)[0]); idx += 3
            elif tag == 18: cp[i] = ("indy", struct.unpack_from(">HH", d, idx+1)); idx += 5
            else: raise SystemExit(f"tag {tag}")
            i += 1
        return cp

    def utf(self, i):
        e = self.cp.get(i)
        if e and e[0] == "utf8": return e[1]
        return None

    def cls(self, i):
        e = self.cp.get(i)
        if e and e[0] == "class": return self.utf(e[1])
        return None

    def ref(self, i):
        e = self.cp.get(i)
        if e and e[0] == "ref":
            cls_idx, nat_idx = e[1]
            nt = self.cp[nat_idx]
            name_idx, desc_idx = nt[1]
            return f"{self.cls(cls_idx)}.{self.utf(name_idx)}:{self.utf(desc_idx)}"
        return None

    def methods(self):
        d = self.d
        n = struct.unpack_from(">H", d, 8)[0]
        idx, i = 10, 1
        while i < n:
            tag = d[idx]
            if tag == 1: idx += 3 + struct.unpack_from(">H", d, idx+1)[0]
            elif tag == 7: idx += 3
            elif tag in (9,10,11,12,17): idx += 5
            elif tag == 8: idx += 3
            elif tag in (3,4): idx += 5
            elif tag in (5,6): idx += 8; i += 1
            elif tag == 15: idx += 4
            elif tag == 16: idx += 3
            elif tag == 18: idx += 5
            else: raise SystemExit(f"tag {tag}")
            i += 1
        acc = struct.unpack_from(">H", d, idx)[0]; idx += 2
        idx += 2  # this_class
        idx += 2  # super_class
        ifc = struct.unpack_from(">H", d, idx)[0]; idx += 2 + 2*ifc
        fcount = struct.unpack_from(">H", d, idx)[0]; idx += 2
        for _ in range(fcount):
            idx += 6
            acount = struct.unpack_from(">H", d, idx)[0]; idx += 2
            for _ in range(acount):
                idx += 2
                alen = struct.unpack_from(">I", d, idx)[0]; idx += 4 + alen
        mcount = struct.unpack_from(">H", d, idx)[0]; idx += 2
        out = []
        for _ in range(mcount):
            macc, name_i, desc_i = struct.unpack_from(">HHH", d, idx)
            idx += 6
            acount = struct.unpack_from(">H", d, idx)[0]; idx += 2
            code = None
            for _ in range(acount):
                aname_i = struct.unpack_from(">H", d, idx)[0]; idx += 2
                alen = struct.unpack_from(">I", d, idx)[0]; idx += 4
                body = d[idx:idx+alen]
                if self.utf(aname_i) == "Code":
                    clen = struct.unpack_from(">I", body, 4)[0]
                    code = body[8:8+clen]
                idx += alen
            out.append((self.utf(name_i), self.utf(desc_i), code))
        return out

def disasm(cf, code):
    out = []
    o = 0
    while o < len(code):
        op = code[o]
        pc = o
        if op == 0x02: out.append((pc, "iconst_m1")); o += 1
        elif op == 0x03: out.append((pc, "iconst_0")); o += 1
        elif op == 0x04: out.append((pc, "iconst_1")); o += 1
        elif op == 0x05: out.append((pc, "iconst_2")); o += 1
        elif op == 0x06: out.append((pc, "iconst_3")); o += 1
        elif op == 0x07: out.append((pc, "iconst_4")); o += 1
        elif op == 0x08: out.append((pc, "iconst_5")); o += 1
        elif op == 0x10: out.append((pc, f"bipush {struct.unpack_from('>b', code, o+1)[0]}")); o += 2
        elif op == 0x11: out.append((pc, f"sipush {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0x12: out.append((pc, f"ldc #{struct.unpack_from('>B', code, o+1)[0]}")); o += 2
        elif op == 0x13: out.append((pc, f"ldc_w #{struct.unpack_from('>H', code, o+1)[0]}")); o += 3
        elif op in (0x15,0x16,0x17,0x19): out.append((pc, f"load {op-0x15}")); o += 2
        elif 0x1a <= op <= 0x1d: out.append((pc, f"iload_{op-0x1a}")); o += 1
        elif 0x2a <= op <= 0x2d: out.append((pc, f"aload_{op-0x2a}")); o += 1
        elif op in (0x36,0x37,0x38,0x3a): out.append((pc, f"store {op-0x36}")); o += 2
        elif 0x3b <= op <= 0x3e: out.append((pc, f"istore_{op-0x3b}")); o += 1
        elif 0x4b <= op <= 0x4e: out.append((pc, f"astore_{op-0x4b}")); o += 1
        elif op == 0xa7: out.append((pc, f"goto {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op in (0x99,0x9a,0x9b,0x9c,0x9d,0x9e):
            names = {0x99:"ifeq",0x9a:"ifne",0x9b:"iflt",0x9c:"ifge",0x9d:"ifgt",0x9e:"ifle"}
            out.append((pc, f"{names[op]} {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op in (0x9f,0xa0,0xa1,0xa2,0xa3,0xa4):
            names = {0x9f:"if_icmpeq",0xa0:"if_icmpne",0xa1:"if_icmplt",0xa2:"if_icmpge",0xa3:"if_icmpgt",0xa4:"if_icmple"}
            out.append((pc, f"{names[op]} {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0xac: out.append((pc, "ireturn")); o += 1
        elif op == 0xb0: out.append((pc, "areturn")); o += 1
        elif op == 0xb1: out.append((pc, "return")); o += 1
        elif op in (0xb2,0xb3): out.append((pc, f"{'getstatic' if op==0xb2 else 'putstatic'} {cf.ref(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op in (0xb4,0xb5,0xb6,0xb7,0xb8):
            names = {0xb4:"getfield",0xb5:"putfield",0xb6:"invokevirtual",0xb7:"invokespecial",0xb8:"invokestatic"}
            r = cf.ref(struct.unpack_from('>H', code, o+1)[0])
            out.append((pc, f"{names[op]} {r}")); o += 3
        elif op == 0xbb: out.append((pc, f"new {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0x01: out.append((pc, "aconst_null")); o += 1
        elif op == 0x57: out.append((pc, "pop")); o += 1
        elif op == 0x58: out.append((pc, "pop2")); o += 1
        elif op == 0xbf: out.append((pc, "athrow")); o += 1
        elif op == 0xc0: out.append((pc, f"checkcast {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xc1: out.append((pc, f"instanceof {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xc6: out.append((pc, f"ifnull {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0xc7: out.append((pc, f"ifnonnull {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        else: out.append((pc, f"op_{op:#04x}")); o += 1
    return out

import zipfile as _zf
with _zf.ZipFile(JAR) as _z:
    _names = [n[:-6] for n in _z.namelist()
              if n.endswith(".class") and "Glass" in n and "$" not in n]
for cls_name in ["net/minecraft/world/level/block/TintedGlassBlock"] + sorted(_names):
    print(f"=== {cls_name} ===")
    cf = CF(load(cls_name))
    for name, desc, code in cf.methods():
        if code is None:
            print(f"  {name}{desc}  (abstract/native)")
            continue
        if name in ("<init>", "skipRendering", "getLightBlock", "getLightDampening",
                    "propagatesSkylightDown", "getCloneItemStack") and len(code) <= 32:
            print(f"  --- {name}{desc} ---")
            for pc, txt in disasm(cf, code):
                print(f"    pc {pc:4d}: {txt}")
        else:
            print(f"  {name}{desc}  (code {len(code)} bytes, skipped)")
    print()
