#!/usr/bin/env python3
"""the enchanting table's own book float probe (the 26.2 jar the
client.jar serves — the EnchantTableRenderer.render's PoseStack
constants + the BookModel.createBodyLayer's box/rotation stream the
static open-book bake pins): the full constant pool the readable
table prints, every method the resolved-instruction walk speaks
(ldc -> the float/int the pool banks, field/invoke -> the name the
member ref carries — the rest pose the tickCount 0 + bookSpread 0 +
openCount 0 state answers)."""
import struct
import sys
import zipfile

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")


class CF:
    def __init__(self, name):
        self.data = z.read(name)
        d = self.data
        self.cp = {}
        self.cp_kind = {}

        def u2(o):
            return struct.unpack(">H", d[o : o + 2])[0]

        def u4(o):
            return struct.unpack(">I", d[o : o + 4])[0]

        self.u2, self.u4 = u2, u4
        cp_count = u2(8)
        o = 10
        i = 1
        while i < cp_count:
            tag = d[o]
            if tag == 1:
                ln = u2(o + 1)
                self.cp[i] = d[o + 3 : o + 3 + ln]
                self.cp_kind[i] = ("UTF8", self.cp[i].decode("utf-8", "replace"))
                o += 3 + ln
            elif tag == 3:
                self.cp[i] = ("INT", struct.unpack(">i", d[o + 1 : o + 5])[0])
                self.cp_kind[i] = self.cp[i]
                o += 5
            elif tag == 4:
                self.cp[i] = ("FLT", struct.unpack(">f", d[o + 1 : o + 5])[0])
                self.cp_kind[i] = self.cp[i]
                o += 5
            elif tag == 5:
                self.cp[i] = ("LNG", struct.unpack(">q", d[o + 1 : o + 9])[0])
                self.cp_kind[i] = self.cp[i]
                o += 9
                i += 1
            elif tag == 6:
                self.cp[i] = ("DBL", struct.unpack(">d", d[o + 1 : o + 9])[0])
                self.cp_kind[i] = self.cp[i]
                o += 9
                i += 1
            elif tag == 7:
                self.cp[i] = ("CLS", u2(o + 1))
                self.cp_kind[i] = ("CLS", None)
                o += 3
            elif tag == 8:
                self.cp[i] = ("STR", u2(o + 1))
                self.cp_kind[i] = ("STR", None)
                o += 3
            elif tag in (9, 10, 11):
                kinds = {9: "FLD", 10: "MTH", 11: "IFM"}
                self.cp[i] = (kinds[tag], u2(o + 1), u2(o + 3))
                self.cp_kind[i] = (kinds[tag], None, None)
                o += 5
            elif tag == 12:
                self.cp[i] = ("NAT", u2(o + 1), u2(o + 3))
                self.cp_kind[i] = ("NAT", None, None)
                o += 5
            elif tag == 15:
                self.cp[i] = ("MH", d[o + 1], u2(o + 2), u2(o + 4))
                self.cp_kind[i] = ("MH",)
                o += 4
            elif tag == 16:
                self.cp[i] = ("MT", u2(o + 1))
                self.cp_kind[i] = ("MT",)
                o += 3
            elif tag in (17, 18):
                self.cp[i] = ("CONDBS", u2(o + 1), u2(o + 3))
                self.cp_kind[i] = ("CONDBS",)
                o += 5
            elif tag in (19, 20):
                self.cp[i] = ("MOD", u2(o + 1))
                self.cp_kind[i] = ("MOD",)
                o += 3
            else:
                raise SystemExit(f"tag {tag}")
            i += 1
        # second pass resolve indirections
        for idx, e in self.cp.items():
            if e[0] in ("CLS", "STR", "MOD") and isinstance(e[1], int):
                tgt = self.cp.get(e[1])
                if isinstance(tgt, bytes):
                    self.cp_kind[idx] = (e[0], tgt.decode("utf-8", "replace"))
            elif e[0] in ("FLD", "MTH", "IFM", "NAT"):
                cls_e = self.cp.get(e[1])
                nat_e = self.cp.get(e[2])
                cls = (
                    cls_e.decode("utf-8", "replace")
                    if isinstance(cls_e, bytes)
                    else self.cp_kind.get(e[1], ("?",))[1]
                    if self.cp_kind.get(e[1], (None, None))[0] == "CLS"
                    else "?"
                )
                if isinstance(nat_e, tuple) and len(nat_e) == 3:
                    nm = self.cp[nat_e[1]].decode("utf-8", "replace")
                    dsc = self.cp[nat_e[2]].decode("utf-8", "replace")
                    self.cp_kind[idx] = (e[0], cls, nm, dsc)

    def members(self):
        d = self.data
        u2, u4 = self.u2, self.u4
        o = self._fields_start()
        res = []
        fc = u2(o)
        o += 2
        for f in range(fc):
            acc, ni, di, ac = u2(o), u2(o + 2), u2(o + 4), u2(o + 6)
            o += 8
            attrs = []
            for a in range(ac):
                an, alen = u2(o), u4(o + 2)
                attrs.append((an, o + 6, alen))
                o += 6 + alen
            res.append(("field", acc, self.cp[ni], self.cp[di], attrs))
        mc = u2(o)
        o += 2
        for m in range(mc):
            acc, ni, di, ac = u2(o), u2(o + 2), u2(o + 4), u2(o + 6)
            o += 8
            attrs = []
            for a in range(ac):
                an, alen = u2(o), u4(o + 2)
                attrs.append((an, o + 6, alen))
                o += 6 + alen
            res.append(("method", acc, self.cp[ni], self.cp[di], attrs))
        return res

    def _fields_start(self):
        d = self.data
        u2, u4 = self.u2, self.u4
        cp_count = u2(8)
        o = 10
        i = 1
        while i < cp_count:
            tag = d[o]
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
        return o + 8 + 2 * ifc

    def code_attr(self, attrs):
        d = self.data
        u4 = self.u4
        for an, off, alen in attrs:
            nm = self.cp.get(an)
            if nm == b"Code":
                cl = u4(off + 4)
                return d[off + 8 : off + 8 + cl]
        return None


LEN = [1] * 202
for op in (16, 18):
    LEN[op] = 2
LEN[17] = 3
for op in (19, 20):
    LEN[op] = 3
for op in (21, 22, 23, 24, 25, 54, 55, 56, 57, 58, 132, 169):
    LEN[op] = 2
for op in (47, 48, 49, 50):
    LEN[op] = 1
for op in (51, 52, 53) + tuple(range(59, 79)):
    LEN[op] = 1
for op in (153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168):
    LEN[op] = 3
for op in (178, 179, 180, 181, 182, 183, 184, 187, 188, 189, 190, 191, 192, 193, 195):
    LEN[op] = 3
LEN[185] = 5
LEN[186] = 5
LEN[196] = 4
LEN[197] = 4
LEN[200] = 5
LEN[201] = 5

MNEM = {
    0: "nop", 1: "aconst_null", 2: "iconst_m1", 3: "iconst_0", 4: "iconst_1",
    5: "iconst_2", 6: "iconst_3", 7: "iconst_4", 8: "iconst_5", 9: "lconst_0",
    26: "iload_0", 27: "iload_1", 28: "iload_2", 29: "iload_3",
    30: "lload_0", 31: "lload_1", 32: "lload_2", 33: "lload_3",
    34: "fload_0", 35: "fload_1", 36: "fload_2", 37: "fload_3",
    38: "dload_0", 39: "dload_1", 40: "dload_2", 41: "dload_3",
    42: "aload_0", 43: "aload_1", 44: "aload_2", 45: "aload_3",
    59: "istore_0", 60: "istore_1", 61: "istore_2", 62: "istore_3",
    63: "lstore_0", 64: "lstore_1", 65: "lstore_2", 66: "lstore_3",
    67: "fstore_0", 68: "fstore_1", 69: "fstore_2", 70: "fstore_3",
    71: "dstore_0", 72: "dstore_1", 73: "dstore_2", 74: "dstore_3",
    75: "astore_0", 76: "astore_1", 77: "astore_2", 78: "astore_3",
    79: "iastore", 80: "lastore", 81: "fastore", 82: "dastore",
    83: "aastore", 84: "bastore", 85: "castore", 86: "sastore",
    10: "lconst_1", 11: "fconst_0", 12: "fconst_1", 13: "fconst_2",
    14: "dconst_0", 15: "dconst_1", 16: "bipush", 17: "sipush", 18: "ldc",
    19: "ldc_w", 20: "ldc2_w", 21: "iload", 22: "lload", 23: "fload",
    24: "dload", 25: "aload", 46: "iaload", 47: "laload", 48: "faload",
    49: "daload", 50: "aaload", 51: "baload", 52: "caload", 53: "saload",
    54: "istore", 55: "lstore", 56: "fstore", 57: "dstore", 58: "astore",
    59: "istore_0", 83: "astore_3", 87: "pop", 88: "pop2", 89: "dup",
    90: "dup_x1", 91: "dup_x2", 92: "dup2", 93: "dup2_x1", 94: "dup2_x2",
    95: "swap", 96: "iadd", 97: "ladd", 98: "fadd", 99: "dadd", 100: "isub",
    101: "lsub", 102: "fsub", 103: "dsub", 104: "imul", 105: "lmul",
    106: "fmul", 107: "dmul", 108: "idiv", 109: "ldiv", 110: "fdiv",
    111: "ddiv", 112: "irem", 113: "lrem", 114: "frem", 115: "dneg",
    116: "ineg", 117: "lneg", 118: "fneg", 119: "dneg", 120: "ishl",
    121: "lshl", 122: "ishr", 123: "lshr", 124: "iushr", 125: "lushr",
    126: "iand", 127: "land", 128: "ior", 129: "lor", 130: "ixor",
    131: "lxor", 132: "iinc", 133: "i2l", 134: "i2f", 135: "i2d",
    136: "l2i", 137: "l2f", 138: "l2d", 139: "f2i", 140: "f2l", 141: "f2d",
    142: "d2i", 143: "d2l", 144: "d2f", 145: "i2b", 146: "i2c", 147: "i2s",
    148: "lcmp", 149: "fcmpl", 150: "fcmpg", 151: "dcmpl", 152: "dcmpg",
    153: "ifeq", 154: "ifne", 155: "iflt", 156: "ifge", 157: "ifgt",
    158: "ifle", 159: "if_icmpeq", 160: "if_icmpne", 161: "if_icmplt",
    162: "if_icmpge", 163: "if_icmpgt", 164: "if_icmple", 165: "if_acmpeq",
    166: "if_acmpne", 167: "goto", 168: "jsr", 169: "ret", 170: "tableswitch",
    171: "lookupswitch", 172: "ireturn", 173: "lreturn", 174: "freturn",
    175: "dreturn", 176: "areturn", 177: "return", 178: "getstatic",
    179: "putstatic", 180: "getfield", 181: "putfield", 182: "invokevirtual",
    183: "invokespecial", 184: "invokestatic", 185: "invokeinterface",
    186: "invokedynamic", 187: "new", 188: "newarray", 189: "anewarray",
    190: "arraylength", 191: "athrow", 192: "checkcast", 193: "instanceof",
    194: "monitorenter", 195: "monitorexit", 196: "wide", 197: "multianewarray",
    198: "ifnull", 199: "ifnonnull", 200: "goto_w", 201: "jsr_w",
}


def disasm(cf, code):
    out = []
    p = 0
    n = len(code)
    while p < n:
        op = code[p]
        if op >= 202:
            out.append(f"!! overrun {p} op {op}")
            break
        if op == 170:
            base = (p + 1 + 3) & ~3
            lo = struct.unpack(">i", code[base : base + 4])[0]
            hi = struct.unpack(">i", code[base + 4 : base + 8])[0]
            out.append(f"{p:4d} tableswitch {lo}..{hi}")
            p = base + 12 + (hi - lo + 1) * 4
            continue
        if op == 171:
            base = (p + 1 + 3) & ~3
            npairs = struct.unpack(">i", code[base + 4 : base + 8])[0]
            out.append(f"{p:4d} lookupswitch x{npairs}")
            p = base + 8 + npairs * 8
            continue
        if op == 196:
            nxt = code[p + 1]
            out.append(f"{p:4d} wide {MNEM.get(nxt, nxt)}")
            p += 6 if nxt == 132 else 4
            continue
        nm = MNEM.get(op, f"op{op}")
        if op in (16,):
            v = code[p + 1]
            out.append(f"{p:4d} bipush {v - 256 if v > 127 else v}")
        elif op == 17:
            out.append(f"{p:4d} sipush {struct.unpack('>h', code[p+1:p+3])[0]}")
        elif op in (18, 19, 20):
            idx = code[p + 1] if op == 18 else struct.unpack(">H", code[p + 1 : p + 3])[0]
            e = cf.cp_kind.get(idx)
            out.append(f"{p:4d} {nm} #{idx} {e}")
        elif op in (178, 179, 180, 181, 182, 183, 184, 187, 189, 192, 193):
            idx = struct.unpack(">H", code[p + 1 : p + 3])[0]
            e = cf.cp_kind.get(idx)
            out.append(f"{p:4d} {nm} {e}")
        elif op in (153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168):
            tgt = struct.unpack(">h", code[p + 1 : p + 3])[0]
            out.append(f"{p:4d} {nm} -> {p + tgt}")
        elif op in (21, 22, 23, 24, 25, 54, 55, 56, 57, 58, 132):
            out.append(f"{p:4d} {nm} {code[p+1]}" + (f" by {struct.unpack('>h', code[p+2:p+4])[0]}" if op == 132 else ""))
        else:
            out.append(f"{p:4d} {nm}")
        p += LEN[op]
    return out


target = sys.argv[1] if len(sys.argv) > 1 else "net/minecraft/client/renderer/blockentity/EnchantTableRenderer.class"
only = sys.argv[2] if len(sys.argv) > 2 else None
cf = CF(target)
print("=== " + target)
for kind, acc, ni, di, attrs in cf.members():
    nm = ni.decode("utf-8", "replace") if isinstance(ni, bytes) else "?"
    dsc = di.decode("utf-8", "replace") if isinstance(di, bytes) else "?"
    if only and only not in nm:
        continue
    code = cf.code_attr(attrs) if kind == "method" else None
    if code is not None:
        print(f"--- {kind} {nm} {dsc}")
        for line in disasm(cf, code):
            print("   ", line)
    else:
        if kind == "field":
            constval = None
            for an, off, alen in attrs:
                if cf.cp.get(an) == b"ConstantValue":
                    idx = struct.unpack(">H", cf.data[off + 6 : off + 8])[0]
                    constval = cf.cp_kind.get(idx)
            print(f"--- field {nm} {dsc} {constval if constval else ''}")
