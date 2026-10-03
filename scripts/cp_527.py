#!/usr/bin/env python3
"""cp_527.py — the ArmPose clinit forensic (the THROW_TRIDENT row the
case-4 lane this round wears). Two jobs:

1. Walk HumanoidModel$ArmPose.<clinit> and print every enum constant's
   (ordinal, twoHanded, affectsOffhandPose) triple — the full_disasm
   tool crashes on this class's ldc shapes, so the loop here decodes
   the enum-construction bytecodes directly.
2. Resolve the HumanoidModel CP floats the case-4 (pc 133 ldc_w) and
   case-9 (pc 308 ldc_w) bodies load — the exact f32 bits the client's
   THROW_TRIDENT fold must carry.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp  # noqa: E402

JAR = "/home/z/my-project/client262.jar"
CLS = "net/minecraft/client/model/HumanoidModel$ArmPose.class"
HUMANOID = "net/minecraft/client/model/HumanoidModel.class"

with zipfile.ZipFile(JAR) as jar:
    data = jar.read(CLS)
    hdata = jar.read(HUMANOID)

cp, coff = parse_cp(data)   # coff = end of ArmPose's own constant pool
hcp, _ = parse_cp(hdata)


def u1(b, o):
    return b[o]


def u2(b, o):
    return (b[o] << 8) | b[o + 1]


def u4(b, o):
    return (b[o] << 24) | (b[o + 1] << 16) | (b[o + 2] << 8) | b[o + 3]


def code_attr(data, cp, coff):
    """Find the <clinit> method's Code attribute offset. Returns (bytes, start)."""
    # locate the utf8 '<clinit>' cp index
    clinit_idx = None
    for i, e in cp.items():
        if e[0] == "utf8" and e[1] == "<clinit>":
            clinit_idx = i
            break
    assert clinit_idx, "no <clinit> utf8"
    # class file: magic u4, minor u2, major u2, cpcount u2, cp...,
    # access u2, this u2, super u2, ifaces u2+..., fields u2+..., methods u2+...
    o = coff
    o += 6  # access, this, super
    ic = u2(data, o); o += 2 + 2 * ic
    fc = u2(data, o); o += 2
    for _ in range(fc):
        o += 6
        ac = u2(data, o); o += 2
        for _ in range(ac):
            ln = u4(data, o + 2)
            o += 6 + ln
    mc = u2(data, o); o += 2
    for _ in range(mc):
        m_acc = u2(data, o); m_name = u2(data, o + 2); m_desc = u2(data, o + 4)
        o += 6
        ac = u2(data, o); o += 2
        for _ in range(ac):
            a_name = u2(data, o); a_len = u4(data, o + 2)
            a_body = o + 6
            name_e = cp.get(m_name)
            if a_name and cp.get(a_name) and cp[a_name][1] == "Code" and \
               name_e and name_e[1] == "<clinit>":
                return data, a_body
            o += 6 + a_len
    raise SystemExit("clinit Code not found")


blob, cstart = code_attr(data, cp, coff)
# Code attribute: max_stack u2, max_locals u2, code_len u4, code...
clen = u4(blob, cstart + 4)
code = blob[cstart + 8: cstart + 8 + clen]

# walk the code, track the last ldc'd name + the iconst pushes
stack = []  # symbolic
cur_name = None
print("== ArmPose clinit enum constructions ==")
o = 0
pending_name = None
pending_ord = None
pending_flags = []
while o < len(code):
    op = code[o]
    if op == 0x01:  # aconst_null
        stack.append("null"); o += 1
    elif op == 0x02 or op == 0x03 or op == 0x04:  # iconst_m1/0/1
        stack.append({0x02: -1, 0x03: 0, 0x04: 1}[op]); o += 1
    elif op == 0x05 or op == 0x06:  # iconst_2/3
        stack.append({0x05: 2, 0x06: 3}[op]); o += 1
    elif op == 0x07 or op == 0x08:
        stack.append({0x07: 4, 0x08: 5}[op]); o += 1
    elif op in (0x10,):  # bipush
        v = code[o + 1]
        if v >= 128:
            v -= 256
        stack.append(v); o += 2
    elif op == 0x11:  # sipush
        v = (code[o + 1] << 8) | code[o + 2]
        if v >= 32768:
            v -= 65536
        stack.append(v); o += 3
    elif op in (0x12, 0x13, 0x14):  # ldc, ldc_w, ldc2_w
        if op == 0x12:
            idx = code[o + 1]; o += 2
        else:
            idx = u2(code, o + 1); o += 3
        e = cp.get(idx)
        rep = e[1] if e else "#0"
        if e and e[0] == "string":
            rep = f'"{e[1]}"'
        stack.append(rep)
    elif op == 0x59:  # dup
        stack.append(stack[-1] if stack else "dup!"); o += 1
    elif op == 0xbb:  # new
        idx = u2(code, o + 1)
        stack.append(f"new#{idx}")
        o += 3
    elif op == 0xb7:  # invokespecial
        idx = u2(code, o + 1)
        e = cp.get(idx)
        args = stack[-6:] if len(stack) >= 6 else stack[:]
        print(f"  invokespecial #{idx} {e[1] if e else ''}  args={args}")
        stack = stack[:-6] if len(stack) >= 6 else []
        o += 3
    elif op == 0xb3:  # putstatic
        idx = u2(code, o + 1)
        e = cp.get(idx)
        val = stack[-1] if stack else "?"
        print(f"  putstatic {e[1] if e else ''} = {val}")
        if stack:
            stack.pop()
        o += 3
    elif op == 0xb8:  # invokestatic
        idx = u2(code, o + 1)
        stack.append(f"static#{idx}")
        o += 3
    elif op == 0xbf:  # athrow
        o += 1
    elif op == 0xb1:  # return
        o += 1
    else:
        o += 1  # skip unknown shapes (getstatic pops handled loosely)

print()
print("== HumanoidModel CP floats (case-4 pc133 / case-9 pc308 / case-1 pc101) ==")
for i, e in hcp.items():
    if e[0] == 4:  # CONSTANT_Float stored as raw u32 bits
        bits = e[1]
        val = struct.unpack(">f", struct.pack(">I", bits))[0]
        print(f"  #{i} float 0x{bits:08X} {val!r}")
