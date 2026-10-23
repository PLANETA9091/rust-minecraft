#!/usr/bin/env python3
"""THE INDY RECIPE PROBE (final): which BSM the SoundSource-loop indy wears —
the options.txt volume key's own recipe ('soundCategory_\\x01' vs the dot one).
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
data = z.read("net/minecraft/client/Options.class")
cp, cp_end = parse_cp(data)

# --- BootstrapMethods (fixed walker)
p = cp_end + 6
ifc = struct.unpack_from(">H", data, p)[0]; p += 2 + 2 * ifc

def skip_fom(p):
    n = struct.unpack_from(">H", data, p)[0]; p += 2
    for _ in range(n):
        p += 6
        ac = struct.unpack_from(">H", data, p)[0]; p += 2
        for _ in range(ac):
            alen = struct.unpack_from(">I", data, p + 2)[0]
            p += 6 + alen
    return p

p = skip_fom(skip_fom(p))
ac = struct.unpack_from(">H", data, p)[0]; p += 2
bsms = []
for _ in range(ac):
    namei = struct.unpack_from(">H", data, p)[0]
    alen = struct.unpack_from(">I", data, p + 2)[0]
    body = data[p + 6:p + 6 + alen]
    if utf(cp, namei) == "BootstrapMethods":
        n = struct.unpack_from(">H", body, 0)[0]
        q = 2
        for k in range(n):
            ref = struct.unpack_from(">H", body, q)[0]; q += 2
            argc = struct.unpack_from(">H", body, q)[0]; q += 2
            args = [struct.unpack_from(">H", body, q + 2 * i)[0] for i in range(argc)]
            q += 2 * argc
            bsms.append((ref, args))
    p += 6 + alen

def bsm_desc(k):
    ref, args = bsms[k]
    mh = cp[ref]
    try:
        mr = cp[mh[2]]
        cls = utf(cp, cp[mr[1]][1])
        ni = mr[2]
        nm = cp[ni]
        # name via NameAndType (tag 12): (tag, name_idx, desc_idx)
        if nm[0] == 12:
            name = utf(cp, nm[1])
        else:
            name = str(nm)
    except Exception as e:
        cls, name = "?", str(e)
    recip = ""
    for ai in args:
        e = cp[ai]
        if e[0] == "u" and ("\\x01" in repr(e[1]) or "\\u0001" in repr(e[1]) or "Category" in e[1] or "olume" in e[1]):
            recip += f" recipe={e[1]!r}"
        elif e[0] == "u" and "concat" in cls:
            recip += f" recipe={e[1]!r}"
    return f"{cls}.{name}{recip}"

print("=== concat BSMs (StringConcatFactory + recipe)")
for k in range(len(bsms)):
    d = bsm_desc(k)
    if "Concat" in d or "recipe=" in d:
        print(f"  BSM{k}: {d}")

# --- indy bootstrap indices in processOptions
ms = methods_of(z, "net.minecraft.client.Options")
for s, (cp2, body) in ms.items():
    if "processOptions" in s:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        j = 0
        print()
        print("=== indy ops in processOptions (pc -> BSM)")
        while j < len(code):
            if code[j] == 0xBA:
                bidx, nat = struct.unpack_from(">HH", code, j + 1)
                print(f"  pc{j}: BSM{bidx} = {bsm_desc(bidx)}")
                j += 4
            else:
                op = code[j]
                if op in (0x13, 0x14): j += 2
                elif op in (0x15, 0x17, 0x19, 0x21, 0x23, 0x25, 0x36, 0x38, 0x3a, 0x42, 0x44, 0x46, 0xbc): j += 1
                elif op in (0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1,
                            0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xc6, 0xc7, 0x10, 0x12): j += 2
                elif op == 0x11: j += 2
                elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xbb,
                            0xbd, 0xc0, 0xc1): j += 2
                elif op == 0xc5: j += 3
                elif op == 0xba: j += 4
                elif op == 0xab:
                    npairs = struct.unpack_from(">i", code, j + 4)[0]
                    j += 8 + 8 * npairs
                elif op == 0xaa:
                    default, lo, hi = struct.unpack_from(">iii", code, j + 1)
                    j += 12 + 4 * (hi - lo + 1)
            j += 1
        break
