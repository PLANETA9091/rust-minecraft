#!/usr/bin/env python3
"""SkullBlock wall-mount probe 26.2 (dev-83): the wall SHAPE per facing,
the canSurvive law, the neighborChanged pop law, the blockstate json +
the wall model box, the SkullBlockRenderer wall transformation."""
import struct, zipfile, json

JAR = "tools/client.jar"

def load(cls):
    with zipfile.ZipFile(JAR) as z:
        return z.read(cls + ".class")

def jar_bytes(name):
    with zipfile.ZipFile(JAR) as z:
        try:
            return z.read(name)
        except KeyError:
            return None

class CF:
    def __init__(self, data):
        self.d = data
        self.cp = self._cp()

    def u2(self, o): return struct.unpack_from(">H", self.d, o)[0]

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
            elif tag in (5,6): cp[i] = ("long", None); idx += 9; i += 1
            elif tag == 15: cp[i] = ("mh", d[idx+1], struct.unpack_from(">H", d, idx+2)[0]); idx += 4
            elif tag == 16: cp[i] = ("pt", struct.unpack_from(">H", d, idx+2)[0]); idx += 3
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

    def num(self, i):
        e = self.cp.get(i)
        if e and e[0] == "num":
            raw = e[1]
            iv = struct.unpack(">i", struct.pack(">I", raw))[0]
            fv = struct.unpack(">f", struct.pack(">I", raw))[0]
            return (iv, fv)
        if e and e[0] == "str":
            return ("str", self.utf(e[1]))
        return None

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

    def scan_to_methods(self):
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
        fields = []
        for _ in range(fcount):
            fi = struct.unpack_from(">HH", d, idx)
            fields.append((self.utf(fi[0]), self.utf(fi[1])))
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
        return fields, out

    def fields(self):
        return self.scan_to_methods()[0]

    def methods(self):
        return self.scan_to_methods()[1]

def disasm(cf, code):
    out = []
    o = 0
    while o < len(code):
        op = code[o]; pc = o
        simple = {0x00:"nop",0x01:"aconst_null",0x02:"iconst_m1",0x03:"iconst_0",0x04:"iconst_1",
                  0x05:"iconst_2",0x06:"iconst_3",0x07:"iconst_4",0x08:"iconst_5",0x57:"pop",
                  0x58:"pop2",0xac:"ireturn",0xb0:"areturn",0xb1:"return",0xbf:"athrow",
                  0x64:"isub",0x68:"imul",0x60:"iadd",0x6f:"idiv",0x70:"irem",0x77:"ineg",
                  0x8d:"i2f",0x8e:"i2d",0x8b:"f2i",0x8c:"f2l",0x8a:"d2i",
                  0x94:"lcmp",0x95:"fcmpl",0x96:"fcmpg",0x97:"dcmpl",0x98:"dcmpg"}
        if op in simple:
            out.append((pc, simple[op])); o += 1
        elif op == 0x10: out.append((pc, f"bipush {struct.unpack_from('>b', code, o+1)[0]}")); o += 2
        elif op == 0x11: out.append((pc, f"sipush {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op in (0x12,0x13):
            ci = struct.unpack_from('>B' if op==0x12 else '>H', code, o+1)[0]
            v = cf.num(ci)
            out.append((pc, f"ldc #{ci} {v}")); o += 2 if op==0x12 else 3
        elif op == 0x14:
            ci = struct.unpack_from('>H', code, o+1)[0]
            out.append((pc, f"ldc2_w #{ci}")); o += 3
        elif op in (0x15,0x16,0x17,0x19): out.append((pc, f"xload {struct.unpack_from('>B', code, o+1)[0]}")); o += 2
        elif 0x1a <= op <= 0x1d: out.append((pc, f"iload_{op-0x1a}")); o += 1
        elif 0x1e <= op <= 0x21: out.append((pc, f"lload_{op-0x1e}")); o += 1
        elif 0x22 <= op <= 0x25: out.append((pc, f"fload_{op-0x22}")); o += 1
        elif 0x26 <= op <= 0x29: out.append((pc, f"dload_{op-0x26}")); o += 1
        elif 0x2a <= op <= 0x2d: out.append((pc, f"aload_{op-0x2a}")); o += 1
        elif op in (0x36,0x37,0x38,0x3a): out.append((pc, f"xstore {struct.unpack_from('>B', code, o+1)[0]}")); o += 2
        elif 0x3b <= op <= 0x3e: out.append((pc, f"istore_{op-0x3b}")); o += 1
        elif 0x43 <= op <= 0x46: out.append((pc, f"fstore_{op-0x43}")); o += 1
        elif 0x47 <= op <= 0x4a: out.append((pc, f"dstore_{op-0x47}")); o += 1
        elif 0x4b <= op <= 0x4e: out.append((pc, f"astore_{op-0x4b}")); o += 1
        elif op == 0xa7: out.append((pc, f"goto {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op in (0x99,0x9a,0x9b,0x9c,0x9d,0x9e):
            names = {0x99:"ifeq",0x9a:"ifne",0x9b:"iflt",0x9c:"ifge",0x9d:"ifgt",0x9e:"ifle"}
            out.append((pc, f"{names[op]} {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op in (0x9f,0xa0,0xa1,0xa2,0xa3,0xa4):
            names = {0x9f:"if_icmpeq",0xa0:"if_icmpne",0xa1:"if_icmplt",0xa2:"if_icmpge",0xa3:"if_icmpgt",0xa4:"if_icmple"}
            out.append((pc, f"{names[op]} {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0x6d: out.append((pc, "fmul")); o += 1
        elif op == 0x66: out.append((pc, "fsub")); o += 1
        elif op == 0x62: out.append((pc, "fadd")); o += 1
        elif op == 0x6e: out.append((pc, "fdiv")); o += 1
        elif op == 0x76: out.append((pc, "fneg")); o += 1
        elif op == 0x71: out.append((pc, "dmul")); o += 1
        elif op == 0x77: out.append((pc, "dneg")); o += 1
        elif op == 0x6b: out.append((pc, "dsub")); o += 1
        elif op == 0x63: out.append((pc, "dadd")); o += 1
        elif op == 0x6f: out.append((pc, "ddiv")); o += 1
        elif op in (0xb2,0xb3):
            r = cf.ref(struct.unpack_from('>H', code, o+1)[0])
            out.append((pc, f"{'getstatic' if op==0xb2 else 'putstatic'} {r}")); o += 3
        elif op in (0xb4,0xb5,0xb6,0xb7,0xb8):
            names = {0xb4:"getfield",0xb5:"putfield",0xb6:"invokevirtual",0xb7:"invokespecial",0xb8:"invokestatic"}
            r = cf.ref(struct.unpack_from('>H', code, o+1)[0])
            out.append((pc, f"{names[op]} {r}")); o += 3
        elif op == 0xb9:
            r = cf.ref(struct.unpack_from('>H', code, o+1)[0])
            out.append((pc, f"invokeinterface {r}")); o += 5
        elif op == 0xbb: out.append((pc, f"new {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xc0: out.append((pc, f"checkcast {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xc1: out.append((pc, f"instanceof {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xc6: out.append((pc, f"ifnull {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0xc7: out.append((pc, f"ifnonnull {struct.unpack_from('>h', code, o+1)[0]}")); o += 3
        elif op == 0xa6: out.append((pc, "if_acmpeq")); o += 3
        elif op == 0xa5: out.append((pc, "if_acmpne")); o += 3
        elif op == 0xba:
            out.append((pc, f"invokedynamic #{struct.unpack_from('>H', code, o+1)[0]}")); o += 5
        elif op == 0x84: out.append((pc, f"iinc {code[o+1]} {struct.unpack_from('>b', code, o+2)[0]}")); o += 3
        elif op == 0xc5: out.append((pc, f"multianewarray #{struct.unpack_from('>H', code, o+1)[0]}")); o += 4
        elif op == 0xbd: out.append((pc, f"anewarray {cf.cls(struct.unpack_from('>H', code, o+1)[0])}")); o += 3
        elif op == 0xbc: out.append((pc, f"newarray {code[o+1]}")); o += 2
        elif op in (0x32,0x33,0x34,0x35): out.append((pc, ["aaload","laload","faload","daload"][op-0x32])); o += 1
        elif op in (0x4f,0x50,0x51,0x52,0x53,0x54,0x55,0x56): out.append((pc, ["iastore","lastore","fastore","dastore","aastore","bastore","castore","sastore"][op-0x4f])); o += 1
        elif op in (0xbe,0xc2,0xc3): out.append((pc, ["arraylength","monitorenter","monitorexit"][op-0xbe])); o += 1
        else: out.append((pc, f"op_{op:#04x}")); o += 1
    return out

def show(cls_name, want, fields=False):
    print(f"=== {cls_name} ===")
    cf = CF(load(cls_name))
    if fields:
        for fn, fd in cf.fields():
            print(f"  FIELD {fn} : {fd}")
    for name, desc, code in cf.methods():
        if code is None:
            continue
        if name in want:
            print(f"--- {name}{desc} (code {len(code)}B) ---")
            for pc, txt in disasm(cf, code):
                print(f"  pc {pc:4d}: {txt}")
    print()

SKULL = "net/minecraft/world/level/block/SkullBlock"
ABS = "net/minecraft/world/level/block/AbstractSkullBlock"
REN = "net/minecraft/client/renderer/blockentity/SkullBlockRenderer"
ENT = "net/minecraft/world/level/block/entity/SkullBlockEntity"

def census(cls_name):
    print(f"=== {cls_name} census ===")
    cf = CF(load(cls_name))
    for fn, fd in cf.fields():
        print(f"  FIELD {fn} : {fd}")
    for name, desc, code in cf.methods():
        print(f"  {name} {desc} code={0 if code is None else len(code)}")
    print()

census(ABS)
census(ENT)

def probe_jar_class(name):
    try:
        return load(name)
    except KeyError:
        return None

for extra in ["net/minecraft/world/level/block/WallSkullBlock",
              "net/minecraft/client/renderer/blockentity/WallAndGroundTransformations"]:
    if probe_jar_class(extra):
        census(extra)

show(ABS, ["<init>", "getStateForPlacement", "canSurvive", "setPlacementData"])
show(ENT, ["setPlacementData", "getPlacementData", "getOrientation",
           "getStateForPlacement", "<init>", "loadWithComponents", "saveWithoutMetadata",
           "getUpdateTag", "getUpdatePacket"])
show("net/minecraft/world/level/block/WallSkullBlock",
     ["<clinit>", "<init>", "getStateForPlacement", "getShape", "canSurvive",
      "createBlockStateDefinition", "updateShape", "neighborChanged"], fields=True)
show("net/minecraft/world/item/StandingAndWallBlockItem",
     ["<clinit>", "<init>", "place", "updatePlacementContext", "getWallBlock",
      "canPlace", "getPlacementState"], fields=True)
show(SKULL, ["createBlockStateDefinition"])
show(REN, ["createWallTransformation", "createGroundTransformation",
           "extractRenderState", "submit", "submitSkull"])
show("net/minecraft/client/renderer/blockentity/WallAndGroundTransformations",
     ["wallTransformation", "freeTransformations", "createWallTransformation",
      "createGroundTransformation", "<clinit>"])

with zipfile.ZipFile(JAR) as z:
    wall_states = [n for n in z.namelist()
                   if n.startswith("assets/minecraft/blockstates/") and "skull" in n or "head" in n]
print("=== blockstates skull/head files ===")
for n in sorted(wall_states):
    print(" ", n)

# Blockstate + model jsons
for bs in ["skeleton_skull", "zombie_head", "player_head", "creeper_head"]:
    raw = jar_bytes(f"assets/minecraft/blockstates/{bs}.json")
    if raw:
        print(f"=== blockstates/{bs}.json (first 600B) ===")
        print(raw[:600].decode("utf8", "replace"))
        print()
raw = jar_bytes("assets/minecraft/models/block/skull.json")
if raw:
    print("=== models/block/skull.json ===")
    print(raw.decode("utf8", "replace"))
