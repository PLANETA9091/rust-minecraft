#!/usr/bin/env python3
"""Probe 3: dump the Wolf lambda$* synthetic methods (the target-selector
bodies the invokedynamic sites bind) + <clinit> (the PREY_SELECTOR walk).
Prints every instruction with cp-resolved calls, so the selector law reads
verbatim."""
import struct, zipfile

JAR = "tools/client.jar"
CLS = "net/minecraft/world/entity/animal/wolf/Wolf"

data = zipfile.ZipFile(JAR).read(CLS + ".class")

class R:
    def __init__(s, d): s.d = d; s.p = 0
    def u1(s): v = s.d[s.p]; s.p += 1; return v
    def u2(s): v = struct.unpack_from(">H", s.d, s.p)[0]; s.p += 2; return v
    def u4(s): v = struct.unpack_from(">I", s.d, s.p)[0]; s.p += 4; return v

r = R(data)
assert r.u4() == 0xCAFEBABE
r.u2(); r.u2()
cpcount = r.u2()
cp = [None] * cpcount
i = 1
while i < cpcount:
    tag = r.u1()
    if tag == 1:
        ln = r.u2(); cp[i] = ("Utf8", data[r.p:r.p+ln].decode("utf-8", "replace")); r.p += ln
    elif tag in (7, 8, 16, 19, 20):
        cp[i] = ("ref", r.u2())
    elif tag == 15:
        cp[i] = ("mh", r.u1(), r.u2())
    elif tag in (3, 4):
        cp[i] = ("num", r.u4())
    elif tag in (5, 6):
        cp[i] = ("num8", r.u4(), r.u4()); i += 1
    else:
        cp[i] = ("mem", r.u2(), r.u2())
    i += 1

def utf(idx):
    e = cp[idx]
    return e[1] if e and e[0] == "Utf8" else f"?{idx}"

def cls_name(idx):
    e = cp[idx]
    return utf(e[1]) if e and e[0] == "ref" else f"?{idx}"

NAMES = {0x00:"nop",0x01:"aconst_null",0x02:"iconst_m1",0x03:"iconst_0",0x04:"iconst_1",0x05:"iconst_2",
         0x06:"iconst_3",0x07:"iconst_4",0x08:"iconst_5",0x09:"lconst_0",0x0a:"lconst_1",
         0x0b:"fconst_0",0x0c:"fconst_1",0x0d:"fconst_2",0x0e:"dconst_0",0x0f:"dconst_1",
         0x10:"bipush",0x11:"sipush",0x12:"ldc",0x13:"ldc_w",0x14:"ldc2_w",
         0x15:"iload",0x19:"aload",0x1a:"iload_0",0x1b:"iload_1",0x1c:"iload_2",0x1d:"iload_3",
         0x2a:"aload_0",0x2b:"aload_1",0x2c:"aload_2",0x2d:"aload_3",
         0x36:"istore",0x3a:"astore",0x3b:"istore_0",0x3c:"istore_1",0x3d:"istore_2",0x3e:"istore_3",
         0x4b:"astore_0",0x4c:"astore_1",0x4d:"astore_2",0x4e:"astore_3",
         0x57:"pop",0x59:"dup",0x5f:"swap",0x60:"iadd",0x61:"ladd",0x62:"fadd",0x63:"dadd",
         0x64:"isub",0x67:"dsub",0x68:"imul",0x6b:"dmul",0x6c:"idiv",0x6f:"ddiv",
         0x70:"irem",0x77:"dneg",0x78:"ineg",0x7b:"lshl",0x7c:"lshr",0x7e:"iushr",0x7f:"land",
         0x80:"ior",0x82:"ixor",0x83:"i2l",0x85:"i2f",0x86:"i2d",0x87:"l2f",0x88:"l2d",
         0x89:"l2i",0x8b:"f2i",0x8d:"f2d",0x8e:"d2f",0x8f:"d2i",0x91:"i2b",0x92:"i2c",
         0x94:"lcmp",0x95:"fcmpl",0x96:"fcmpg",0x97:"dcmpl",0x98:"dcmpg",
         0x99:"ifeq",0x9a:"ifne",0x9b:"iflt",0x9c:"ifge",0x9d:"ifgt",0x9e:"ifle",
         0x9f:"if_icmpeq",0xa0:"if_icmpne",0xa1:"if_icmplt",0xa2:"if_icmpge",0xa3:"if_icmpgt",0xa4:"if_icmple",
         0xa5:"if_acmpeq",0xa6:"if_acmpne",0xa7:"goto",0xc6:"ifnull",0xc7:"ifnonnull",
         0xbb:"new",0xbd:"anewarray",0xbe:"arraylength",0xbf:"athrow",0xc0:"checkcast",0xc1:"instanceof",
         0xb1:"return",0xac:"ireturn",0xad:"lreturn",0xae:"freturn",0xb0:"areturn",
         0x32:"aaload",0x53:"aastore",0x35:"iaload",0x4f:"iastore",
         0xc2:"monitorenter",0xc3:"monitorexit",0xfa:"athrow_wide"}

def decode(code, cp):
    p = 0
    out = []
    while p < len(code):
        op = code[p]
        nm = NAMES.get(op, f"op_{op:02x}")
        try:
            if op in (0xb4, 0xb2, 0xb5):
                idx = struct.unpack_from(">H", code, p+1)[0]
                e = cp[idx]
                if e and e[0] == "mem":
                    out.append(f"{p:4d} {nm} {cls_name(e[1])}.{utf(e[2])}")
                else:
                    out.append(f"{p:4d} {nm} #{idx}")
                p += 3
            elif op in (0xb6, 0xb7, 0xb8):
                idx = struct.unpack_from(">H", code, p+1)[0]
                e = cp[idx]
                if e and e[0] == "mem":
                    out.append(f"{p:4d} {nm} {cls_name(e[1])}.{utf(e[2])}")
                else:
                    out.append(f"{p:4d} {nm} #{idx}")
                p += 3
            elif op == 0xba:
                idx = struct.unpack_from(">H", code, p+1)[0]
                e = cp[idx]
                nm2 = utf(e[2]) if e and e[0] == "mem" else "?"
                out.append(f"{p:4d} invokedynamic {nm2}")
                p += 5
            elif op in (0x12,):
                idx = code[p+1]
                e = cp[idx]
                v = cls_name(idx) if e and e[0] == "ref" else (e[1] if e and e[0] == "Utf8" else e)
                out.append(f"{p:4d} ldc -> {v}")
                p += 2
            elif op in (0x13, 0x14):
                idx = struct.unpack_from(">H", code, p+1)[0]
                e = cp[idx]
                v = cls_name(idx) if e and e[0] == "ref" else e
                out.append(f"{p:4d} {nm} -> {v}")
                p += 3
            elif op == 0x10:
                out.append(f"{p:4d} bipush {struct.unpack_from('>b', code, p+1)[0]}")
                p += 2
            elif op == 0x11:
                out.append(f"{p:4d} sipush {struct.unpack_from('>h', code, p+1)[0]}")
                p += 3
            elif op in (0x99,0x9a,0x9b,0x9c,0x9d,0x9e,0x9f,0xa0,0xa1,0xa2,0xa3,0xa4,0xa5,0xa6,0xa7,0xc6,0xc7):
                off = struct.unpack_from(">h", code, p+1)[0]
                out.append(f"{p:4d} {nm} -> {p+off}")
                p += 3
            elif op in (0x15, 0x19, 0x36, 0x3a, 0xc0, 0xc1, 0xbb, 0xbd):
                idx = struct.unpack_from(">H", code, p+1)[0] if op in (0xc0, 0xc1, 0xbb, 0xbd) else code[p+1]
                extra = f" {cls_name(idx)}" if op in (0xc1, 0xc0) else (f" -> {cls_name(idx)}" if op in (0xbb, 0xbd) else "")
                out.append(f"{p:4d} {nm}{extra}")
                p += 3 if op in (0xc0, 0xc1, 0xbb, 0xbd) else 2
            elif op in (0x14,):
                out.append(f"{p:4d} ldc2_w")
                p += 3
            else:
                out.append(f"{p:4d} {nm}")
                p += 1
        except IndexError:
            out.append(f"{p:4d} TRUNCATED")
            break
    return out

r.u2(); r.u2(); r.u2()
ifc = r.u2(); r.p += 2 * ifc
def skip_members():
    n = r.u2()
    for _ in range(n):
        r.u2(); r.u2(); r.u2()
        for _ in range(r.u2()):
            r.u2(); alen = r.u4(); r.p += alen
skip_members()
mcount = r.u2()
TARGETS = ("lambda$", "<clinit>")
for _ in range(mcount):
    r.u2(); name_i = r.u2(); desc_i = r.u2()
    name = utf(name_i); desc = utf(desc_i)
    for _ in range(r.u2()):
        an_i = r.u2(); alen = r.u4(); ad = data[r.p:r.p+alen]; r.p += alen
        if utf(an_i) == "Code" and (name.startswith("lambda$") or name == "<clinit>"):
            cr = R(ad)
            cr.u2(); cr.u2(); clen = cr.u4()
            code = ad[cr.p:cr.p+clen]
            print(f"=== {name}{desc} ({clen} bytes) ===")
            print("\n".join(decode(code, cp)))
            print()
