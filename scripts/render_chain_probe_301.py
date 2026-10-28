#!/usr/bin/env python3
"""THE LATCH'S OWN U-LAW WALK probe (autopush-301 round).

Q1: ChestRenderer.render — the pose chain (the translate/mulPose/scale
    calls the model wears: the X-flip the y-down model space answers).
Q2: the methods ChestRenderer carries (the render entry names).
"""
import struct, sys, zipfile

JAR = "/home/z/my-project/rust-minecraft/.mc2rs/classpath/client.jar"
CLS = "net/minecraft/client/renderer/blockentity/ChestRenderer"


def utf8(cp, i):
    v = cp[i]
    if v[0] == 'Utf8':
        return v[1]
    if v[0] in ('Class', 'Str', 'MethodType'):
        return utf8(cp, v[1])
    return str(v)


def parse_class(data):
    r = __import__('roperator') if False else None


# reuse raw_disasm's parser by import
sys.path.insert(0, "/home/z/my-project/rust-minecraft/scripts")
import raw_disasm as rd

data = zipfile.ZipFile(JAR).read(f"{CLS}.class")

# Q2: enumerate methods (parse with want=set() would yield none; walk manually)
r = rd.R(data)
n = r.u2()
cp = [None] * n
i = 1
while i < n:
    tag = r.u1()
    if tag == 1:
        ln = r.u2(); cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
    elif tag == 3: cp[i] = ('Int', r.s4())
    elif tag == 4: cp[i] = ('Float', r.f4())
    elif tag == 5: cp[i] = ('Long', struct.unpack_from('>q', r.raw(8))[0]); i += 1
    elif tag == 6: cp[i] = ('Double', r.f8()); i += 1
    elif tag == 7: cp[i] = ('Class', r.u2())
    elif tag == 8: cp[i] = ('Str', r.u2())
    elif tag in (9, 10, 11): cp[i] = ('Ref', r.u2(), r.u2())
    elif tag == 12: cp[i] = ('Name', r.u2(), r.u2())
    elif tag == 15: cp[i] = ('MethodHandle', r.u1(), r.u2())
    elif tag == 16: cp[i] = ('MethodType', r.u2())
    elif tag == 17: cp[i] = ('Dynamic', r.u2(), r.u2())
    elif tag == 18: cp[i] = ('InvokeDynamic', r.u2(), r.u2())
    elif tag in (19, 20): cp[i] = ('Mod', r.u2())
    else: cp[i] = ('Raw', tag, r.u4())  # unknown/newer tag: skip 4 payload bytes
    i += 1

flags = r.u2(); r.u2(); r.u2()
nif = r.u2()
for _ in range(nif): r.u2()
nf = r.u2()
for _ in range(nf):
    r.u2(); r.u2(); r.u2()
    na = r.u2()
    for _ in range(na):
        r.u2(); alen = r.u4(); r.raw(alen)
nm = r.u2()
print("== methods with code ==")
names = []
for _ in range(nm):
    r.u2(); name_i = r.u2(); desc_i = r.u2()
    name = utf8(cp, name_i); desc = utf8(cp, desc_i)
    na = r.u2()
    has_code = False
    for _ in range(na):
        a_name = utf8(cp, r.u2())
        alen = r.u4(); adata = r.raw(alen)
        if a_name == 'Code':
            has_code = True
    if has_code and not name.startswith('<'):
        names.append(name)
        print(f"  {name} {desc}")
