#!/usr/bin/env python3
"""THE INDY RECIPE PROBE (autopush-168 round 7): the BootstrapMethods
attribute — which recipe constant the SoundSource-loop indy wears
('soundCategory_\\x01' vs 'soundCategory.\\x01' — the options.txt key's own
truth), + the OPTION_SPLITTER's ':' shape (the splitter utf8).
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
data = z.read("net/minecraft/client/Options.class")
cp, _ = parse_cp(data)

def cls_name(i):
    return utf(cp, cp[i][1])

def member(cpentry, kind="m"):
    # cpentry = (tag, class_idx, nat_idx); nat = (name_idx, desc_idx)
    _, ci, ni = cpentry
    name, desc = cp[ni][1], cp[ni][2]
    return f"{cls_name(ci)}.{name} {desc}"

# walk to the class attribute table: cp -> access -> this/super -> ifaces ->
# fields -> methods -> attributes
p = 10
cpcount = struct.unpack_from(">H", data, 8)[0]
# re-walk cp manually to find where it ends: parse_cp returns p already
_, cp_end = parse_cp(data)
p = cp_end
p += 6  # access, this, super
ifc = struct.unpack_from(">H", data, p)[0]; p += 2 + 2 * ifc

def skip_fields_or_methods(p):
    n = struct.unpack_from(">H", data, p)[0]; p += 2
    for _ in range(n):
        p += 6  # access, name, desc
        ac = struct.unpack_from(">H", data, p)[0]; p += 2
        for _ in range(ac):
            alen = struct.unpack_from(">I", data, p + 2)[0]
            p += 6 + alen
    return p

p = skip_fields_or_methods(p)  # fields
p = skip_fields_or_methods(p)  # methods

# class attributes
ac = struct.unpack_from(">H", data, p)[0]; p += 2
for _ in range(ac):
    namei = struct.unpack_from(">H", data, p)[0]
    alen = struct.unpack_from(">I", data, p + 2)[0]
    body = data[p + 6:p + 6 + alen]
    aname = utf(cp, namei)
    if aname == "BootstrapMethods":
        n = struct.unpack_from(">H", body, 0)[0]
        print(f"=== BootstrapMethods ({n} entries)")
        q = 2
        bsm = []
        for k in range(n):
            ref = struct.unpack_from(">H", body, q)[0]; q += 2
            argc = struct.unpack_from(">H", body, q)[0]; q += 2
            args = []
            for _ in range(argc):
                ai = struct.unpack_from(">H", body, q)[0]; q += 2
                args.append(ai)
            bsm.append((ref, args))
        for k, (ref, args) in enumerate(bsm):
            mhref = cp[ref]
            if mhref[0] == 15:
                mr = cp[mhref[2]]  # MethodHandle.kind, ref -> Methodref
                print(f"  BSM{k}: {member(mr)}")
            else:
                print(f"  BSM{k}: tag{mhref[0]} {mhref[1:]}")
            for ai in args:
                e = cp[ai]
                if e[0] == "u":
                    print(f"      arg: {e[1]!r}")
                elif e[0] == 8:
                    print(f"      arg(str): {utf(cp, e[1])!r}")
                else:
                    print(f"      arg: tag{e[0]}")
    p += 6 + alen
