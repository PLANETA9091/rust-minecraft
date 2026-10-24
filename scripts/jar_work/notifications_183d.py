#!/usr/bin/env python3
"""THE NOTIFICATION WALK v4 (autopush-183d): the bootstrap #55 the
indy@954 (caption) + #55/56 the indy@955 (xmap) lanes bind — the
exact lambda$ methods the OptionInstance ctor wears — the FULL
registration walk 1290-1425 uncut, the lang row the caption speaks.
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
d = z.read("net/minecraft/client/Options.class")

# the BootstrapMethods attribute: brute scan for utf8 'BootstrapMethods'
# cp index, then find u2(idx) occurrences that parse as an attribute head
bm_idx = None
for i, c in enumerate(cp):
    if c and c[0] == 1 and c[1] == b"BootstrapMethods":
        bm_idx = i
        break
print("BootstrapMethods utf8 idx:", bm_idx)

pat = struct.pack(">H", bm_idx)
boots = None
pos = 0
while True:
    pos = d.find(pat, pos + 1)
    if pos < 0:
        break
    ln = struct.unpack_from(">I", d, pos + 2)[0]
    if ln < 4 or pos + 6 + ln > len(d):
        continue
    n = struct.unpack_from(">H", d, pos + 6)[0]
    if 40 < n < 500:  # plausible bootstrap count
        body = d[pos + 6:pos + 6 + ln]
        q = 2
        boots = []
        for _ in range(n):
            mh, nargs = struct.unpack_from(">HH", body, q); q += 4
            args = struct.unpack_from(f">{nargs}H", body, q); q += 2 * nargs
            boots.append((mh, args))
        print(f"bootstrap table @ raw {pos}: {n} methods")
        break

def show_boot(i):
    mh, args = boots[i]
    print(f"bootstrap#{i}: handle={core['cp_ref'](cp, mh)} args={[core['cp_ref'](cp, a) for a in args]}")

for i in [54, 55, 56, 57]:
    show_boot(i)

# the full registration walk
b = bs.get(("<init>", "(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"))
clen = struct.unpack_from(">I", b, 4)[0]
code = b[8:8 + clen]
ls = core["disasm"](cp, code)
print("== Options.<init> pcs 1294-1425 uncut ==")
for pc, l in ls:
    if 1294 <= pc <= 1425:
        print(f"   {pc}: {l}")
