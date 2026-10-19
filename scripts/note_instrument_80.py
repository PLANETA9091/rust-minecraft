#!/usr/bin/env python3
"""javap-подобный дизасм NoteBlockInstrument.class из 26.2 jar (строки/поля)."""
import struct, sys, zipfile

JAR = "tools/client.jar"
CLS = "net/minecraft/world/level/block/state/properties/NoteBlockInstrument.class"

with zipfile.ZipFile(JAR) as z:
    data = z.read(CLS)

def u2(o): return struct.unpack_from(">H", data, o)[0]
def u4(o): return struct.unpack_from(">I", data, o)[0]

magic = u4(0)
minor, major = u2(4), u2(6)
cp_count = u2(8)
# parse constant pool at offset 8, entries are 1-indexed
idx = 10
cp = {}
i = 1
while i < cp_count:
    tag = data[idx]
    if tag == 1:  # Utf8
        ln = u2(idx+1)
        cp[i] = ("utf8", data[idx+3:idx+3+ln].decode("utf8", "replace"))
        idx += 3 + ln
    elif tag == 7:
        cp[i] = ("class", u2(idx+1)); idx += 3
    elif tag == 9 or tag == 10 or tag == 11 or tag == 12 or tag == 17:
        cp[i] = ("ref", u2(idx+1), u2(idx+3)); idx += 5
    elif tag == 8:
        cp[i] = ("str", u2(idx+1)); idx += 3
    elif tag == 3 or tag == 4:
        cp[i] = ("num", u4(idx+1)); idx += 5
    elif tag == 5 or tag == 6:
        cp[i] = ("long", None); idx += 9; i += 1  # takes two slots
    elif tag == 15:
        cp[i] = ("mh", data[idx+1], u2(idx+2)); idx += 4
    elif tag == 16:
        cp[i] = ("pt", u2(idx+1)); idx += 3
    elif tag == 18:
        cp[i] = ("indy", u2(idx+1), u2(idx+3)); idx += 5
    else:
        raise SystemExit(f"tag {tag} @ {idx}")
    i += 1

def utf(i):
    return cp[i][1] if cp.get(i) and cp[i][0] == "utf8" else None

acc = u2(idx); idx += 2
this_c, sup_c = u2(idx), u2(idx+2); idx += 4
ifc = u2(idx); idx += 2
for _ in range(ifc):
    idx += 2
fc = u2(idx); idx += 2
print(f"major={major} minor={minor} fields={fc}")
print("--- FIELDS ---")
for _ in range(fc):
    f_acc, f_name, f_desc = u2(idx), u2(idx+2), u2(idx+4)
    idx += 6
    attr_c = u2(idx); idx += 2
    print(f"  {'0x{f_acc:04x}'.format(f_acc=f_acc)} {utf(f_name)} : {utf(f_desc)}")
    for _ in range(attr_c):
        an, alen = u2(idx), u4(idx+2)
        idx += 6 + alen
mc = u2(idx); idx += 2
print("--- METHODS ---")
for _ in range(mc):
    m_acc, m_name, m_desc = u2(idx), u2(idx+2), u2(idx+4)
    idx += 6
    attr_c = u2(idx); idx += 2
    print(f"  {utf(m_name)}{utf(m_desc)}")
    for _ in range(attr_c):
        an, alen = u2(idx), u4(idx+2)
        if utf(an) == "Code":
            # dump LDC strings from the code: scan for ldc opcodes is overkill;
            # just note the code length
            print(f"    code_len={u4(idx+4)}")
        idx += 6 + alen
ac = u2(idx); idx += 2
print("--- CLASS ATTRS ---")
for _ in range(ac):
    an, alen = u2(idx), u4(idx+2)
    print(f"  {utf(an)} len={alen}")
    idx += 6 + alen

print("--- ALL UTF8 STRINGS (order) ---")
for k, v in cp.items():
    if v[0] == "utf8" and ("imitate" in v[1] or "note_block" in v[1] or v[1].isupper() and len(v[1]) > 3):
        print(f"  #{k}: {v[1]}")
