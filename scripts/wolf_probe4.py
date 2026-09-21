#!/usr/bin/env python3
"""Probe 4: resolve Wolf's BootstrapMethods entries (the lambda bindings
the invokedynamic sites wear) + list all declared method names, so the
prio-4 Player selector's implMethod reads verbatim."""
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

def mem_str(idx):
    e = cp[idx]
    if not (e and e[0] == "mem"):
        return f"?{idx}"
    kind = e[0]
    return f"{cls_name(e[1])}.{utf(e[2])}"

def mh_str(idx):
    e = cp[idx]
    if not (e and e[0] == "mh"):
        return f"?{idx}"
    kind, ref = e[1], e[2]
    kinds = {1:"getField",2:"getStatic",3:"putField",4:"putStatic",5:"invokeVirtual",6:"invokeStatic",7:"invokeSpecial",8:"newInvokeSpecial",9:"invokeInterface"}
    return f"{kinds.get(kind,'?')}:{mem_str(ref)}"

r.u2(); r.u2(); r.u2()
ifc = r.u2(); r.p += 2 * ifc
def skip_members():
    n = r.u2()
    for _ in range(n):
        r.u2(); r.u2(); r.u2()
        for _ in range(r.u2()):
            r.u2(); alen = r.u4(); r.p += alen
skip_members()

# METHODS: list names
mcount = r.u2()
names = []
for _ in range(mcount):
    r.u2(); name_i = r.u2(); desc_i = r.u2()
    names.append(f"{utf(name_i)}{utf(desc_i)}")
    for _ in range(r.u2()):
        r.u2(); alen = r.u4(); r.p += alen

print("=== ALL METHODS ===")
for n in names:
    print(" ", n)

# re-walk class attributes (BootstrapMethods) — need to re-parse; class attrs come after methods
r2 = R(data)
r2.u4(); r2.u2(); r2.u2()
cpcount2 = r2.u2()
i = 1
while i < cpcount2:
    tag = r2.u1()
    if tag == 1:
        ln = r2.u2(); r2.p += ln
    elif tag in (7, 8, 16, 19, 20):
        r2.u2()
    elif tag == 15:
        r2.u1(); r2.u2()
    elif tag in (3, 4):
        r2.u4()
    elif tag in (5, 6):
        r2.u4(); r2.u4(); i += 1
    else:
        r2.u2(); r2.u2()
    i += 1
r2.u2(); r2.u2(); r2.u2()
ifc = r2.u2(); r2.p += 2 * ifc
def skip_members2():
    n = r2.u2()
    for _ in range(n):
        r2.u2(); r2.u2(); r2.u2()
        for _ in range(r2.u2()):
            r2.u2(); alen = r2.u4(); r2.p += alen
skip_members2()
for _ in range(r2.u2()):
    r2.u2(); r2.u2(); r2.u2()
    for _ in range(r2.u2()):
        r2.u2(); alen = r2.u4(); r2.p += alen
acount = r2.u2()
for _ in range(acount):
    an_i = r2.u2(); alen = r2.u4(); ad = data[r2.p:r2.p+alen]; r2.p += alen
    if utf(an_i) == "BootstrapMethods":
        br = R(ad)
        count = br.u2()
        print(f"\n=== BOOTSTRAPMETHODS ({count}) ===")
        for k in range(count):
            bsm_mh = br.u2()
            argc = br.u2()
            args = [br.u2() for _ in range(argc)]
            print(f"  bsm#{k}: {mh_str(bsm_mh)}")
            for a in args:
                e = cp[a]
                if e and e[0] == "mh":
                    print(f"      arg#{a}: MethodHandle {mh_str(a)}")
                else:
                    print(f"      arg#{a}: {e if not (e and e[0]=='ref') else cls_name(a)}")
