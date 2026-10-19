#!/usr/bin/env python3
"""Декод <clinit> NoteBlockInstrument 26.2: ldc-строки + method refs по pc."""
import struct, zipfile

JAR = "tools/client.jar"
CLS = "net/minecraft/world/level/block/state/properties/NoteBlockInstrument.class"

with zipfile.ZipFile(JAR) as z:
    data = z.read(CLS)

def u1(o): return data[o]
def u2(o): return struct.unpack_from(">H", data, o)[0]
def u4(o): return struct.unpack_from(">I", data, o)[0]

cp_count = u2(8)
idx = 10
cp = {}
i = 1
while i < cp_count:
    tag = data[idx]
    if tag == 1:
        ln = u2(idx + 1)
        cp[i] = ("utf8", data[idx + 3:idx + 3 + ln].decode("utf8", "replace"))
        idx += 3 + ln
    elif tag == 7:
        cp[i] = ("class", u2(idx + 1)); idx += 3
    elif tag in (9, 10, 11, 12, 17):
        cp[i] = ("ref", u2(idx + 1), u2(idx + 3)); idx += 5
    elif tag == 8:
        cp[i] = ("str", u2(idx + 1)); idx += 3
    elif tag in (3, 4):
        cp[i] = ("num", u4(idx + 1)); idx += 5
    elif tag in (5, 6):
        cp[i] = ("long", None); idx += 9; i += 1
    elif tag == 15:
        cp[i] = ("mh", data[idx + 1], u2(idx + 2)); idx += 4
    elif tag == 16:
        cp[i] = ("pt", u2(idx + 1)); idx += 3
    elif tag == 18:
        cp[i] = ("indy", u2(idx + 1), u2(idx + 3)); idx += 5
    else:
        raise SystemExit(f"tag {tag}")
    i += 1

def utf(i): return cp[i][1] if cp.get(i) and cp[i][0] == "utf8" else None

# find methods
acc = u2(idx); idx += 2
idx += 4
ifc = u2(idx); idx += 2 + 2 * ifc
fc = u2(idx); idx += 2
for _ in range(fc):
    idx += 6
    ac2 = u2(idx); idx += 2
    for _ in range(ac2):
        al = u4(idx + 2); idx += 6 + al
mc = u2(idx); idx += 2
clinit_code = None
for _ in range(mc):
    m_name, m_desc = u2(idx), u2(idx + 2)
    idx += 6
    ac2 = u2(idx); idx += 2
    for _ in range(ac2):
        an, al = u2(idx), u4(idx + 2)
        if utf(an) == "Code" and utf(m_name) == "<clinit>":
            # attribute_info: name(2) len(4) body — body starts at idx+6
            max_stack, max_locals = u2(idx + 6), u2(idx + 8)
            c_len = u4(idx + 10)
            clinit_code = data[idx + 14:idx + 14 + c_len]
        idx += 6 + al

print(f"clinit code len={len(clinit_code)}")

# constant pool walk: decode ldc/ldc_w (op 0x12/0x13) and method/invoke refs
pc = 0
out = []
code = clinit_code
def cp_entry(i): return cp.get(i)

while pc < len(code):
    op = code[pc]
    if op == 0x12:  # ldc
        k = code[pc + 1]
        e = cp_entry(k)
        out.append((pc, "ldc", e))
        pc += 2
    elif op == 0x13:  # ldc_w
        k = u2_at(code, pc + 1)
        e = cp_entry(k)
        out.append((pc, "ldc_w", e))
        pc += 3
    elif op in (0xb2,):  # getstatic
        k = u2_at(code, pc + 1)
        e = cp_entry(k)
        out.append((pc, "getstatic", e))
        pc += 3
    elif op in (0xb3,):  # putstatic
        k = u2_at(code, pc + 1)
        e = cp_entry(k)
        out.append((pc, "putstatic", e))
        pc += 3
    elif op in (0xb6, 0xb7, 0xb8):  # invokevirtual/special/static
        k = u2_at(code, pc + 1)
        e = cp_entry(k)
        out.append((pc, {0xb6: "inv_v", 0xb7: "inv_s", 0xb8: "inv_st"}[op], e))
        pc += 3
    elif op == 0xba:  # invokedynamic
        pc += 5
    elif op in (0x10, 0x11):
        pc += 2 if op == 0x10 else 3
    elif op in (0xa7, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xc6, 0xc7):
        pc += 3
    elif op == 0xc5:
        pc += 4
    elif op in (0x84, 0x15, 0x16, 0x17, 0x18, 0x19, 0x3a, 0xa9):
        pc += 2
    else:
        pc += 1

def u2_at(b, o): return struct.unpack_from(">H", b, o)[0]

def refname(e):
    if not e: return "?"
    if e[0] == "ref":
        cls = cp_entry(e[1]); nt = cp_entry(e[2])
        cname = cp_entry(cls[1])[1] if cls and cls[0] == "class" else "?"
        name, desc = cp_entry(nt[1])[1], cp_entry(nt[2])[1]
        return f"{cname}.{name}{desc}"
    if e[0] == "utf8":
        return e[1]
    if e[0] == "str":
        s = cp_entry(e[1])
        return f'"{s[1]}"' if s else "?"
    if e[0] == "class":
        return cp_entry(e[1])[1]
    if e[0] == "indy":
        return f"indy#{e[1]}"
    if e[0] == "num":
        return f"num={e[1]}"
    return str(e[0])

for pc, kind, e in out:
    print(f"  {pc:5d} {kind:10s} {refname(e)}")
