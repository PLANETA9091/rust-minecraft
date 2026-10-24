#!/usr/bin/env python3
"""THE NOTIFICATION WALK v3 (autopush-183c): the FULL registration
window (1290-1420 uncut), the indy bootstrap linkage (@954 caption /
@955 xmap the exact lambda$ binds), the cp#252 save-key utf8 the
visit seat wears, the cp#2819/#2821 caption rows the lambda speaks.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
CLS = "net.minecraft.client.Options"
cp, bs = core["method_bodies"](z, CLS)

def utf8(i):
    c = cp[i] if i < len(cp) else None
    if c and c[0] == 1:
        return c[1].decode("utf-8", "replace")
    return repr(c)

print("== the cp rows ==")
for i in [240, 252, 2819, 2821, 445, 629]:
    print(f"cp#{i}: {utf8(i) if cp[i] and cp[i][0]==1 else repr(cp[i])}")

# the bootstrap methods: BootstrapMethods attribute
d = z.read("net/minecraft/client/Options.class")

# walk the class attributes for BootstrapMethods (tag 23 = 0x17)
# crude scan: find the attribute by name utf8 'BootstrapMethods'
# The cp carries the utf8; locate the attribute in the class attribute table.
# Simpler: reuse core's cp walk? do a manual class-attribute parse.
def u2(p): return struct.unpack_from(">H", d, p)[0]
def u4(p): return struct.unpack_from(">I", d, p)[0]

# class structure: magic(4) minor(2) major(2) cpcount(2) cp... access(2) this(2) super(2) ifaces(2+2n) fields(2)... 
cpcount = u2(8)
p = 10
i = 1
while i < cpcount:
    tag = d[p]; p += 1
    if tag == 1:
        ln = u2(p); p += 2 + ln
    elif tag in (7, 8, 16, 19, 20):
        p += 2
    elif tag == 15:
        p += 3
    elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
        p += 4
    elif tag in (5, 6):
        p += 8; i += 1
    else:
        raise SystemExit(f"cp tag {tag} @ {p}")
access = u2(p); p += 2
this_cls = u2(p); p += 2
sup = u2(p); p += 2
ic = u2(p); p += 2 + 2 * ic
# fields
fc = u2(p); p += 2
for _ in range(fc):
    p += 6
    ac = u2(p); p += 2
    for _ in range(ac):
        p += 2; ln = u4(p); p += 4 + ln
# methods
mc = u2(p); p += 2
for _ in range(mc):
    p += 6
    ac = u2(p); p += 2
    for _ in range(ac):
        p += 2; ln = u4(p); p += 4 + ln
# class attributes
ac = u2(p); p += 2
for _ in range(ac):
    name_i = u2(p); p += 2
    ln = u4(p); p += 4
    body = d[p:p + ln]; p += ln
    if utf8(name_i) == "BootstrapMethods":
        q = 0
        n = struct.unpack_from(">H", body, 0)[0]; q = 2
        for bi in range(n):
            bsm = struct.unpack_from(">H", body, q)[0]; q += 2
            argc = struct.unpack_from(">H", body, q)[0]; q += 2
            args = []
            for _ in range(argc):
                ai = struct.unpack_from(">H", body, q)[0]; q += 2
                args.append(ai)
            if bi in (54, 55, 56, 57):
                print(f"bootstrap#{bi}: method_ref={bsm} args={[utf8(a) if cp[a] and cp[a][0]==1 else repr(cp[a]) for a in args]}")

# 3) the FULL registration walk 1290-1420
b = bs.get(("<init>", "(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"))
if b:
    clen = struct.unpack_from(">I", b, 4)[0]
    code = b[8:8 + clen]
    ls = core["disasm"](cp, code)
    print("== Options.<init> pcs 1290-1425 uncut ==")
    for pc, l in ls:
        if 1290 <= pc <= 1425:
            print(f"   {pc}: {l}")
