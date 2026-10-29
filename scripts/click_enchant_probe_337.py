#!/usr/bin/env python3
"""the enchanting menu's own click walk probe (the 26.2 jar the client.jar
serves — the EnchantmentMenu.clickMenuButton the Code attribute the
constants stream bakes): the full JVM opcode stream the method prints,
the ldc/bipush/sipush + the method refs the spend walk speaks."""
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
cp = {}
i = 1
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
        cp[i] = ("mh", data[o + 1], u2(o + 2), u2(o + 4))
        o += 4
    elif tag == 3:
        cp[i] = ("num", struct.unpack(">i", data[o + 1 : o + 5])[0])
        o += 5
    elif tag == 4:
        cp[i] = ("flt",)
        o += 5
    elif tag in (5, 6):
        cp[i] = ("num8", data[o + 1 : o + 9].hex())
        o += 9
        i += 1  # long/double take TWO cp slots
    elif tag in (9, 10, 11, 12):
        cp[i] = ("mem", u2(o + 1), u2(o + 3))
        o += 5
    else:
        raise ValueError(f"tag {tag} at {o}")
    i += 1


def utf(i):
    t, v = cp[i]
    assert t == "utf"
    return v


def name_of(i):
    t = cp.get(i)
    if not t:
        return f"?#{i}"
    if t[0] == "mem":
        cls_t, nat_i = t[1], t[2]
        cls = utf(cp[cls_t][1]) if cp.get(cls_t, ("",))[0] == "ref" else "?"
        nt = cp.get(nat_i)
        if nt and nt[0] == "utf":
            return f"{cls}.{nt[1]}"
        return f"?#{i}"
    return f"?#{i}"


# method table
o += 6  # skip access/this/super
iface_count = u2(o)
o += 2 + 2 * iface_count
field_count = u2(o)
o += 2
for _ in range(field_count):
    o += 6
    attrs = u2(o)
    o += 2
    for _a in range(attrs):
        o += 6 + u4(o + 2)
method_count = u2(o)
o += 2
for _m in range(method_count):
    acc = u2(o)
    nm = utf(u2(o + 2))
    dsc = utf(u2(o + 4))
    o += 6
    attrs = u2(o)
    o += 2
    for _a in range(attrs):
        an = utf(u2(o))
        ln = u4(o + 2)
        assert ln >= 0
        if nm == "clickMenuButton" and an == "Code":
            cl = u4(o + 10)
            print(f"=== {nm} {dsc} acc={acc:04x} codelen={cl} ===")
            code = data[o + 14 : o + 14 + cl]
            print("code head hex:", code[:48].hex())
            print("max_stack", u2(o + 14), "max_locals", u2(o + 16))
            # walk the raw opcode stream (all reads on the CODE slice)
            def c2(p):
                return struct.unpack(">H", code[p : p + 2])[0]

            def c4(p):
                return struct.unpack(">i", code[p : p + 4])[0]

            def c2s(p):
                return struct.unpack(">h", code[p : p + 2])[0]

            pc = 0
            names = {
                0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
                0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
                0x08: "iconst_5", 0x09: "lconst_0", 0x0A: "lconst_1", 0x0B: "fconst_0",
                0x0C: "fconst_1", 0x0D: "fconst_2", 0x0E: "dconst_0", 0x0F: "dconst_1",
                0x1A: "iload_0", 0x1B: "iload_1", 0x1C: "iload_2", 0x1D: "iload_3",
                0x2A: "aload_0", 0x2B: "aload_1", 0x2C: "aload_2", 0x2D: "aload_3",
                0x4B: "astore_0", 0x4C: "astore_1", 0x4D: "astore_2", 0x4E: "astore_3",
                0x59: "dup", 0x5F: "swap", 0x60: "iadd", 0x64: "isub", 0x68: "imul",
                0x70: "irem", 0x74: "ineg", 0x78: "ishl", 0x7A: "ishr", 0x7C: "iushr",
                0x80: "ior", 0x7E: "iand", 0x82: "ixor",
                0x83: "i2l", 0x85: "l2i", 0x86: "i2f", 0x88: "i2d",
                0xAC: "ireturn", 0xAD: "lreturn", 0xAE: "freturn", 0xB0: "areturn",
                0xB1: "return", 0xBE: "arraylength", 0xBF: "athrow", 0xC0: "checkcast",
                0xC1: "instanceof", 0x53: "aastore", 0x4F: "iastore",
                0x37: "lstore", 0x36: "istore", 0x3A: "astore", 0x15: "iload",
                0xAB: "lookupswitch", 0xAA: "tableswitch", 0xC4: "wide",
            }
            for i in range(0x1E, 0x2A):
                if i < 0x22:
                    names[i] = f"lload_{i-0x1E}"
                else:
                    names[i] = f"fload_{i-0x22}"
            for i in range(0x2E, 0x35):
                names[i] = f"iaload fam_{i-0x2E}"
            for i in range(0x2E, 0x4E):
                if i not in names:
                    arr = ["iaload","laload","faload","daload","aaload","baload","caload","saload"]
                    if i < 0x36:
                        names[i] = arr[i-0x2E]
                    else:
                        names[i] = f"store_{i}"
            for i in range(0x4F, 0x59):
                arr = ["iastore","lastore","fastore","dastore","aastore","bastore","castore","sastore","pop","pop2"]
                names[i] = arr[i-0x4F]
            for i in range(0x61, 0x83):
                base = ["iadd","isub","imul","idiv","irem","ineg","ishl","ishr","iushr","iand","ior","ixor"]
                # rough: l/i/f/d arithmetic family
                names.setdefault(i, f"arith_{i-0x61}")
            for i in range(0x87, 0x97):
                names.setdefault(i, f"conv_{i-0x87}")
            for i in range(0x98, 0xA7):
                names.setdefault(i, f"cmp_{i-0x98}")
            BR = {0x99: "ifeq", 0x9A: "ifne", 0x9B: "iflt", 0x9C: "ifge", 0x9D: "ifgt",
                  0x9E: "ifle", 0x9F: "if_icmpeq", 0xA0: "if_icmpne", 0xA1: "if_icmplt",
                  0xA2: "if_icmpge", 0xA3: "if_icmpgt", 0xA4: "if_icmple",
                  0xA5: "if_acmpeq", 0xA6: "if_acmpne", 0xC6: "ifnull", 0xC7: "ifnonnull",
                  0xA7: "goto", 0xC8: "goto_w"}
            while pc < len(code):
                op = code[pc]
                nmz = names.get(op, f"op_0x{op:02x}")
                if op in BR:
                    if op in (0xC8, 0xC9):
                        print(f"{pc:4d}: {nmz} -> {pc + c4(pc+1)}")
                        pc += 5
                    else:
                        print(f"{pc:4d}: {nmz} -> {pc + c2s(pc+1)}")
                        pc += 3
                elif op == 0x12:
                    j = code[pc+1]
                    t = cp.get(j)
                    print(f"{pc:4d}: ldc #{j} {t}")
                    pc += 2
                elif op in (0x13, 0x14):
                    j = c2(pc+1)
                    print(f"{pc:4d}: ldc_{ 'w' if op==0x13 else '2w'} #{j} {cp.get(j)}")
                    pc += 3
                elif op in (0x15, 0x36, 0x3A) or 0x1E <= op <= 0x2D or 0x2E <= op <= 0x35:
                    if op in (0x15, 0x36, 0x3A):
                        print(f"{pc:4d}: {nmz} {code[pc+1]}")
                        pc += 2
                    else:
                        print(f"{pc:4d}: {nmz}")
                        pc += 1
                elif 0x1E <= op <= 0x4E:
                    print(f"{pc:4d}: {nmz}")
                    pc += 1
                elif op in (0xB2, 0xB4, 0xB5):
                    j = c2(pc+1)
                    print(f"{pc:4d}: {'getstatic' if op==0xB2 else ('getfield' if op==0xB4 else 'putfield')} {name_of(j)}")
                    pc += 3
                elif op in (0xB6, 0xB7, 0xB8):
                    j = c2(pc+1)
                    kind = {0xB6: "invokevirtual", 0xB7: "invokespecial", 0xB8: "invokestatic"}[op]
                    print(f"{pc:4d}: {kind} {name_of(j)}")
                    pc += 3
                elif op == 0xB9 or op == 0xBA:
                    j = c2(pc+1)
                    print(f"{pc:4d}: {'invokeinterface' if op==0xB9 else 'invokedynamic'} {name_of(j)}")
                    pc += 5
                elif op == 0xBB:
                    j = c2(pc+1)
                    t = cp.get(j)
                    cls = utf(t[1]) if t and t[0] == "ref" else f"?#{j}"
                    print(f"{pc:4d}: new {cls}")
                    pc += 3
                elif op == 0xBD:
                    j = c2(pc+1)
                    print(f"{pc:4d}: anewarray {name_of(j)}")
                    pc += 3
                elif op == 0x10:
                    v = code[pc+1]
                    print(f"{pc:4d}: bipush {v-256 if v > 127 else v}")
                    pc += 2
                elif op == 0x11:
                    print(f"{pc:4d}: sipush {struct.unpack('>h', code[pc+1:pc+3])[0]}")
                    pc += 3
                elif op == 0x84:
                    print(f"{pc:4d}: iinc {code[pc+1]} {struct.unpack('>b', code[pc+2:pc+3])[0]}")
                    pc += 3
                elif op == 0xA9:
                    print(f"{pc:4d}: ret {code[pc+1]}")
                    pc += 2
                elif op == 0xC4:
                    w = code[pc+1]
                    if w == 0x84:
                        print(f"{pc:4d}: wide iinc {c2(pc+2)} {struct.unpack('>h', code[pc+4:pc+6])[0]}")
                        pc += 6
                    else:
                        print(f"{pc:4d}: wide {names.get(w, f'op_{w:02x}')} {c2(pc+2)}")
                        pc += 4
                elif op in (0xAA, 0xAB):
                    base = pc + (4 - pc % 4) % 4
                    df = c4(base)
                    if op == 0xAA:
                        lo = c4(base + 4)
                        hi = c4(base + 8)
                        print(f"{pc:4d}: tableswitch lo={lo} hi={hi} default={pc+df}")
                        n = hi - lo + 1
                        for k in range(n):
                            print(f"        [{lo+k} -> {pc + c4(base + 12 + 4*k)}]")
                        pc = base + 12 + 4 * n
                    else:
                        np = c4(base + 4)
                        print(f"{pc:4d}: lookupswitch n={np} default={pc+df}")
                        for k in range(np):
                            m = c4(base + 8 + 8*k)
                            off = c4(base + 12 + 8*k)
                            print(f"        [{m} -> {pc + off}]")
                        pc = base + 8 + 8 * np
                else:
                    print(f"{pc:4d}: {nmz}")
                    pc += 1
            print(f"code end pc={pc} cl={cl}")
        o += 6 + ln
