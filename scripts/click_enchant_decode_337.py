#!/usr/bin/env python3
"""the clickMenuButton code the exact byte-level decode wears (the 0x19/0x15/
0x36/0x3A the 2-byte local ops the javap answer keeps) — the spend walk the
final third the 83..151 region speaks."""
import struct
import zipfile

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
data = z.read("net/minecraft/world/inventory/EnchantmentMenu.class")


def u2(o):
    return struct.unpack(">H", data[o : o + 2])[0]


def u4(o):
    return struct.unpack(">I", data[o : o + 4])[0]


cp_count = u2(8)
o = 10
i = 1
cp = {}
while i < cp_count:
    tag = data[o]
    if tag == 1:
        ln = u2(o + 1)
        cp[i] = ("utf", data[o + 3 : o + 3 + ln].decode("utf-8", "replace"))
        o += 3 + ln
    elif tag in (7, 8, 16, 19, 20):
        cp[i] = ("ref", u2(o + 1))
        o += 3
    elif tag in (17, 18):
        cp[i] = ("dyn", u2(o + 1), u2(o + 3))
        o += 5
    elif tag == 15:
        cp[i] = ("mh",)
        o += 4
    elif tag == 3:
        cp[i] = ("int", struct.unpack(">i", data[o + 1 : o + 5])[0])
        o += 5
    elif tag == 4:
        cp[i] = ("flt",)
        o += 5
    elif tag in (5, 6):
        cp[i] = ("lng",)
        o += 9
        i += 1
    elif tag == 12:
        cp[i] = ("nt", u2(o + 1), u2(o + 3))
        o += 5
    elif tag in (9, 10, 11):
        cp[i] = ("mem", u2(o + 1), u2(o + 3))
        o += 5
    else:
        raise SystemExit(f"tag {tag}")
    i += 1


def utf(i):
    return cp[i][1]


def mem_name(idx):
    cls_i, nat_i = cp[idx][1], cp[idx][2]
    cls = utf(cp[cls_i][1]).split("/")[-1]
    nt = cp[nat_i]
    return f"{cls}.{utf(nt[1])}{utf(nt[2])}"


# locate clickMenuButton Code attr (reuse walk from earlier probe)
o2 = o + 6
ifc = u2(o2)
o2 += 2 + 2 * ifc
fc = u2(o2)
o2 += 2
for _ in range(fc):
    o2 += 6
    ac = u2(o2)
    o2 += 2
    for _a in range(ac):
        o2 += 6 + u4(o2 + 2)
mc = u2(o2)
o2 += 2
code = None
for _m in range(mc):
    nm_i = u2(o2 + 2)
    o2 += 6
    ac = u2(o2)
    o2 += 2
    name = utf(nm_i)
    for _a in range(ac):
        an_i = u2(o2)
        alen = u4(o2 + 2)
        o2 += 6
        if name == "clickMenuButton" and cp[an_i][1] == "Code":
            cl = u4(o2 + 4)
            code = data[o2 + 8 : o2 + 8 + cl]
        o2 += alen
assert code is not None


def c2(p):
    return struct.unpack(">H", code[p : p + 2])[0]


def c2s(p):
    return struct.unpack(">h", code[p : p + 2])[0]


BR = {
    0x99: "ifeq", 0x9A: "ifne", 0x9B: "iflt", 0x9C: "ifge", 0x9D: "ifgt",
    0x9E: "ifle", 0x9F: "if_icmpeq", 0xA0: "if_icmpne", 0xA1: "if_icmplt",
    0xA2: "if_icmpge", 0xA3: "if_icmpgt", 0xA4: "if_icmple",
    0xA5: "if_acmpeq", 0xA6: "if_acmpne", 0xC6: "ifnull", 0xC7: "ifnonnull",
    0xA7: "goto",
}
pc = 0
while pc < len(code):
    op = code[pc]
    if op in BR:
        print(f"{pc:4d}: {BR[op]:10s} -> {pc + c2s(pc+1)}")
        pc += 3
    elif op == 0xB2:
        print(f"{pc:4d}: getstatic  {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB4:
        print(f"{pc:4d}: getfield  {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB5:
        print(f"{pc:4d}: putfield  {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB6:
        print(f"{pc:4d}: invokevirtual {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB7:
        print(f"{pc:4d}: invokespecial {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB8:
        print(f"{pc:4d}: invokestatic {mem_name(c2(pc+1))}")
        pc += 3
    elif op == 0xB9:
        print(f"{pc:4d}: invokeinterface {mem_name(c2(pc+1))}")
        pc += 5
    elif op == 0xBA:
        j = c2(pc + 1)
        nt = cp[j]
        print(f"{pc:4d}: invokedynamic {utf(nt[1])}{utf(nt[2])}")
        pc += 5
    elif op == 0x12:
        j = code[pc + 1]
        e = cp.get(j)
        print(f"{pc:4d}: ldc #{j} {e}")
        pc += 2
    elif op == 0x13:
        j = c2(pc + 1)
        e = cp.get(j)
        print(f"{pc:4d}: ldc_w #{j} {e}")
        pc += 3
    elif op == 0x10:
        v = code[pc + 1]
        print(f"{pc:4d}: bipush {v - 256 if v > 127 else v}")
        pc += 2
    elif op == 0x11:
        print(f"{pc:4d}: sipush {struct.unpack('>h', code[pc+1:pc+3])[0]}")
        pc += 3
    elif op in (0x15, 0x36, 0x3A, 0x19):
        nmz = {0x15: "iload", 0x36: "istore", 0x3A: "astore", 0x19: "aload"}[op]
        print(f"{pc:4d}: {nmz} {code[pc+1]}")
        pc += 2
    elif op == 0x84:
        print(f"{pc:4d}: iinc {code[pc+1]} {struct.unpack('>b', code[pc+2:pc+3])[0]}")
        pc += 3
    elif op == 0xBB:
        j = c2(pc + 1)
        print(f"{pc:4d}: new {utf(cp[j][1]).split('/')[-1]}")
        pc += 3
    elif op == 0xC0:
        j = c2(pc + 1)
        print(f"{pc:4d}: checkcast {utf(cp[j][1]).split('/')[-1]}")
        pc += 3
    elif op in (0xAA, 0xAB):
        base = pc + (4 - pc % 4) % 4
        df = struct.unpack(">i", code[base : base + 4])[0]
        if op == 0xAA:
            lo = struct.unpack(">i", code[base+4:base+8])[0]
            hi = struct.unpack(">i", code[base+8:base+12])[0]
            print(f"{pc:4d}: tableswitch lo={lo} hi={hi} default={pc+df}")
            n = hi - lo + 1
            for k in range(n):
                off = struct.unpack(">i", code[base+12+4*k:base+16+4*k])[0]
                print(f"       [{lo+k} -> {pc+off}]")
            pc = base + 12 + 4 * n
        else:
            np_ = struct.unpack(">i", code[base+4:base+8])[0]
            print(f"{pc:4d}: lookupswitch n={np_} default={pc+df}")
            for k in range(np_):
                m = struct.unpack(">i", code[base+8+8*k:base+12+8*k])[0]
                off = struct.unpack(">i", code[base+12+8*k:base+16+8*k])[0]
                print(f"       [{m} -> {pc+off}]")
            pc = base + 8 + 8 * np_
    else:
        simple = {
            0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
            0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
            0x08: "iconst_5", 0x1A: "iload_0", 0x1B: "iload_1", 0x1C: "iload_2",
            0x1D: "iload_3", 0x2A: "aload_0", 0x2B: "aload_1", 0x2C: "aload_2",
            0x2D: "aload_3", 0x4B: "astore_0", 0x4C: "astore_1", 0x4D: "astore_2",
            0x4E: "astore_3", 0x59: "dup", 0x57: "pop", 0x60: "iadd", 0x64: "isub",
            0x68: "imul", 0x70: "irem", 0x74: "ineg", 0x78: "ishl", 0x7A: "ishr",
            0x7C: "iushr", 0x7E: "iand", 0x80: "ior", 0x82: "ixor", 0x83: "i2l",
            0x85: "l2i", 0x86: "i2f", 0x88: "i2d", 0xAC: "ireturn", 0xB0: "areturn",
            0xB1: "return", 0xBE: "arraylength", 0xBF: "athrow", 0xC1: "instanceof",
            0x53: "aastore", 0x4F: "iastore", 0x5F: "swap",
        }
        print(f"{pc:4d}: {simple.get(op, f'op_0x{op:02x}')}")
        pc += 1
