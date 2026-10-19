#!/usr/bin/env python3
"""Exact dump of Block.boxZ(DDDDD) + Block.box(DDDDDD) with precise
double-op length table — the wall skull collision box law."""
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
                if tag == 5: cp[i] = ("long", struct.unpack(">q", raw)[0])
                else: cp[i] = ("dbl", struct.unpack(">d", raw)[0])
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

    def ref(self, i):
        e = self.cp.get(i)
        if e and e[0] == "ref":
            cls_idx, nat_idx = e[1]
            nt = self.cp[nat_idx]
            try:
                name_idx, desc_idx = nt[1]
            except Exception:
                return f"ref<{nt}>"
            return f"{self.cls(cls_idx).split('/')[-1]}.{self.utf(name_idx)}"
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
            elif tag in (5,6): idx += 9; i += 1
            elif tag == 15: idx += 4
            elif tag == 16: idx += 3
            elif tag == 18: idx += 5
            elif tag in (19, 20): idx += 3
            else: raise SystemExit(f"tag {tag}")
            i += 1
        idx += 6
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
            name_i, desc_i = struct.unpack_from(">HH", d, idx+2)
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

# op -> (mnemonic, extra operand bytes) ; None extra = 0
OPS = {
    0x00:("nop",0),0x01:("aconst_null",0),0x02:("iconst_m1",0),0x03:("iconst_0",0),
    0x04:("iconst_1",0),0x05:("iconst_2",0),0x06:("iconst_3",0),0x07:("iconst_4",0),
    0x08:("iconst_5",0),0x09:("lconst_0",0),0x0a:("lconst_1",0),0x0b:("fconst_0",0),
    0x0c:("fconst_1",0),0x0d:("fconst_2",0),0x0e:("dconst_0",0),0x0f:("dconst_1",0),
    0x57:("pop",0),0x58:("pop2",0),0x59:("dup",0),0x5a:("dup_x1",0),0x5b:("dup2",0),
    0x5c:("dup2_x1",0),0x5d:("dup2_x2",0),0xac:("ireturn",0),0xad:("lreturn",0),
    0xae:("freturn",0),0xaf:("dreturn",0),0xb0:("areturn",0),0xb1:("return",0),
    0xbf:("athrow",0),0x60:("iadd",0),0x61:("ladd",0),0x62:("fadd",0),0x63:("dadd",0),
    0x64:("isub",0),0x65:("lsub",0),0x66:("fsub",0),0x67:("dsub",0),
    0x68:("imul",0),0x69:("lmul",0),0x6a:("fmul",0),0x6b:("dmul",0),
    0x6c:("idiv",0),0x6d:("ldiv",0),0x6e:("fdiv",0),0x6f:("ddiv",0),
    0x70:("irem",0),0x71:("lrem",0),0x72:("frem",0),0x73:("drem",0),
    0x74:("ineg",0),0x75:("lneg",0),0x76:("fneg",0),0x77:("dneg",0),
    0x8d:("i2f",0),0x8e:("i2d",0),0x8b:("f2i",0),0x8c:("f2l",0),0x8a:("d2i",0),
    0x8f:("d2l",0),0x88:("l2d",0),0x87:("l2f",0),0x85:("l2i",0),0x86:("i2l",0),
    0x94:("lcmp",0),0x95:("fcmpl",0),0x96:("fcmpg",0),0x97:("dcmpl",0),0x98:("dcmpg",0),
    0x10:("bipush",1),0x11:("sipush",2),
    0x12:("ldc1",1),0x13:("ldc2",2),0x14:("ldc2w",2),
    0x15:("iload",1),0x16:("lload",1),0x17:("fload",1),0x18:("dload",1),0x19:("aload",1),
    0x36:("istore",1),0x37:("lstore",1),0x38:("fstore",1),0x39:("dstore",1),0x3a:("astore",1),
    0xa7:("goto",2),
    0x99:("ifeq",2),0x9a:("ifne",2),0x9b:("iflt",2),0x9c:("ifge",2),0x9d:("ifgt",2),0x9e:("ifle",2),
    0x9f:("if_icmpeq",2),0xa0:("if_icmpne",2),0xa1:("if_icmplt",2),0xa2:("if_icmpge",2),
    0xa3:("if_icmpgt",2),0xa4:("if_icmple",2),0xa5:("if_acmpne",2),0xa6:("if_acmpeq",2),
    0xb2:("getstatic",2),0xb3:("putstatic",2),0xb4:("getfield",2),0xb5:("putfield",2),
    0xb6:("invokevirtual",2),0xb7:("invokespecial",2),0xb8:("invokestatic",2),
    0xb9:("invokeinterface",4),0xba:("invokedynamic",4),
    0xbb:("new",2),0xc0:("checkcast",2),0xc1:("instanceof",2),
    0xc6:("ifnull",2),0xc7:("ifnonnull",2),
    0xbc:("newarray",1),0xbd:("anewarray",2),0xc5:("multianewarray",3),
    0xbe:("arraylength",0),0xc2:("monitorenter",0),0xc3:("monitorexit",0),
    0xa9:("ret",1),0x84:("iinc",2),0xc4:("wide",0),
}

def disasm(cf, code):
    out, o = [], 0
    while o < len(code):
        op = code[o]; pc = o
        if op not in OPS:
            out.append((pc, f"op_{op:#04x}")); o += 1
            continue
        name, ext = OPS[op]
        if name == "ldc1":
            ci = code[o+1]
            e = cf.cp.get(ci)
            out.append((pc, f"ldc #{ci} {e}")); o += 2; continue
        if name in ("ldc2","ldc2w"):
            ci = struct.unpack_from(">H", code, o+1)[0]
            e = cf.cp.get(ci)
            out.append((pc, f"ldc_w #{ci} {e}")); o += 3; continue
        if name in ("getstatic","putstatic","getfield","putfield","invokevirtual",
                    "invokespecial","invokestatic"):
            r = cf.ref(struct.unpack_from(">H", code, o+1)[0])
            out.append((pc, f"{name} {r}")); o += 3; continue
        if name == "invokeinterface":
            r = cf.ref(struct.unpack_from(">H", code, o+1)[0])
            out.append((pc, f"{name} {r}")); o += 5; continue
        if name in ("new","checkcast","instanceof","anewarray"):
            c = cf.cls(struct.unpack_from(">H", code, o+1)[0])
            out.append((pc, f"{name} {c}")); o += 3; continue
        if name in ("ifeq","ifne","iflt","ifge","ifgt","ifle","if_icmpeq","if_icmpne",
                    "if_icmplt","if_icmpge","if_icmpgt","if_icmple","if_acmpne",
                    "if_acmpeq","ifnull","ifnonnull","goto","jsr"):
            delta = struct.unpack_from(">h", code, o+1)[0]
            out.append((pc, f"{name} -> {pc+3+delta}")); o += 3; continue
        if name == "invokedynamic":
            out.append((pc, f"invokedynamic #{struct.unpack_from('>H', code, o+1)[0]}")); o += 5; continue
        if name == "wide":
            out.append((pc, "wide ...")); o += 4; continue
        if name in ("iload","lload","fload","dload","aload","istore","lstore",
                    "fstore","dstore","astore","ret"):
            out.append((pc, f"{name} {code[o+1]}")); o += 2; continue
        if name == "iinc":
            out.append((pc, f"iinc {code[o+1]} {struct.unpack_from('>b', code, o+2)[0]}")); o += 3; continue
        out.append((pc, name)); o += 1 + ext
    return out

cf = CF(load("net/minecraft/world/level/block/Block"))
for want_desc in ("(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                  "(DDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                  "(DDDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                  "(DDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;"):
    for name, desc, code in cf.methods():
        if name in ("boxZ", "box") and desc == want_desc and code:
            print(f"--- Block.{name}{desc} ({len(code)}B) ---")
            for pc, txt in disasm(cf, code):
                print(f"  pc {pc:4d}: {txt}")
            print()
