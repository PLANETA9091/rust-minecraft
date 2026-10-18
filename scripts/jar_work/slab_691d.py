#!/usr/bin/env python3
"""THE SLAB COLLECTION'S OWN INDY DECODE (the dev-63 round): the four indys
the CUT_COPPER_SLAB registerBlocks row wires — the bootstrap methods the
lambda targets speak (the ctor + properties + waxed lambdas the weathering
ladder banks).

Usage: python3 scripts/jar_work/slab_691d.py <client.jar>
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

d = z.read("net/minecraft/world/level/block/Blocks.class")
cp, p = parse_cp(d)

# locate the class's attributes to find BootstrapMethods + Code of <clinit>
# parse_cp left p right after the constant pool (it consumed magic+minor+major+cpcount)
# access_flags u2, this_class u2, super_class u2
p += 6
nif = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * nif
nf = struct.unpack_from(">H", d, p)[0]; p += 2
for _ in range(nf):
    p += 6  # access, name, desc
    na = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(na):
        p += 2
        ln = struct.unpack_from(">I", d, p)[0]; p += 4 + ln
nm = struct.unpack_from(">H", d, p)[0]; p += 2
clinit_code = None
methods = []
for _ in range(nm):
    acc, namei, desci = struct.unpack_from(">HHH", d, p); p += 6
    name = utf(cp, namei)
    na = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(na):
        aname = utf(cp, struct.unpack_from(">H", d, p)[0]); p += 2
        ln = struct.unpack_from(">I", d, p)[0]
        info = d[p + 4:p + 4 + ln]; p += 4 + ln
        if name == "<clinit>" and aname == "Code":
            clinit_code = info
    methods.append(name)

assert clinit_code is not None
# Code attribute: max_stack u2, max_locals u2, code_len u4, code...
clen = struct.unpack_from(">I", clinit_code, 4)[0]
code = clinit_code[8:8 + clen]

# bootstrap methods attribute: find it among class attributes
# continue class parse after methods
nc = struct.unpack_from(">H", d, p)[0]; p += 2
bootmethods = None
for _ in range(nc):
    aname = utf(cp, struct.unpack_from(">H", d, p)[0]); p += 2
    ln = struct.unpack_from(">I", d, p)[0]
    info = d[p + 4:p + 4 + ln]; p += 4 + ln
    if aname == "BootstrapMethods":
        bootmethods = info
assert bootmethods is not None

nbm = struct.unpack_from(">H", bootmethods, 0)[0]
boots = []
q = 2
for i in range(nbm):
    mh_ref, nargs = struct.unpack_from(">HH", bootmethods, q); q += 4
    args = struct.unpack_from(f">{nargs}H", bootmethods, q); q += 2 * nargs
    boots.append((mh_ref, args))

def mh_string(idx):
    """MethodHandle entry -> human string."""
    e = cp[idx]
    kind = e[1]
    refi = e[2]
    return f"kind={kind} {ref_name(cp, refi)}"

# walk clinit code, collect indys: pc -> bootstrap index
indys = {}
pc = 0
j = 0
while j < len(code):
    op = code[j]
    pc = j
    if op == 0xba:
        idx = struct.unpack_from(">H", code, j + 1)[0]
        e = cp[idx]  # tag 18: (18, bootstrap_attr_index, name_and_type)
        indys[pc] = e[1]
        j += 5
        continue
    # skip lengths per opcode (mirror walk_code's table minimally for our pcs)
    # use a simple table of instruction lengths
    LEN = {0x00:1,0x01:1,0x02:1,0x03:1,0x04:1,0x05:1,0x06:1,0x07:1,0x08:1,0x09:1,0x0a:1,
           0x0b:1,0x0c:1,0x0d:1,0x0e:1,0x0f:1,0x10:3,0x11:3,0x12:2,0x13:3,0x14:3,
           0x15:2,0x16:2,0x17:2,0x18:2,0x19:2,0x1a:1,0x1b:1,0x1c:1,0x1d:1,0x1e:1,
           0x1f:1,0x20:1,0x21:1,0x22:1,0x23:1,0x24:1,0x25:1,0x26:1,0x27:1,0x28:1,
           0x29:1,0x2a:1,0x2b:1,0x2c:1,0x2d:1,0x2e:1,0x2f:1,0x30:1,0x31:1,0x32:1,
           0x33:1,0x34:1,0x35:1,0x36:2,0x37:2,0x38:2,0x39:2,0x3a:2,0x3b:1,0x3c:1,
           0x3d:1,0x3e:1,0x3f:1,0x40:1,0x41:1,0x42:1,0x43:1,0x44:1,0x45:1,0x46:1,
           0x47:1,0x48:1,0x49:1,0x4a:1,0x4b:1,0x4c:1,0x4d:1,0x4e:1,0x4f:1,0x50:1,
           0x51:1,0x52:1,0x53:1,0x54:1,0x55:1,0x56:1,0x57:1,0x58:1,0x59:1,0x5a:1,
           0x5b:1,0x5c:1,0x5d:1,0x5e:1,0x5f:1,0x60:1,0x61:1,0x62:1,0x63:1,0x64:1,
           0x65:1,0x66:1,0x67:1,0x68:1,0x69:1,0x6a:1,0x6b:1,0x6c:1,0x6d:1,0x6e:1,
           0x6f:1,0x70:1,0x71:1,0x72:1,0x73:1,0x74:1,0x75:1,0x76:1,0x77:1,0x78:1,
           0x79:1,0x7a:1,0x7b:1,0x7c:1,0x7d:1,0x7e:1,0x7f:1,0x80:1,0x81:1,0x82:1,
           0x83:1,0x84:2,0x85:1,0x86:1,0x87:1,0x88:1,0x89:1,0x8a:1,0x8b:1,0x8c:1,
           0x8d:1,0x8e:1,0x8f:1,0x90:1,0x91:1,0x92:1,0x93:1,0x94:1,0x95:1,0x96:1,
           0x97:1,0x98:1,0x99:3,0x9a:3,0x9b:3,0x9c:3,0x9d:3,0x9e:3,0x9f:3,0xa0:3,
           0xa1:3,0xa2:3,0xa3:3,0xa4:3,0xa5:3,0xa6:3,0xa7:3,0xa8:1,0xa9:1,0xaa:-1,
           0xab:-2,0xac:1,0xad:1,0xae:1,0xaf:1,0xb0:1,0xb1:1,0xb2:3,0xb3:3,0xb4:3,
           0xb5:3,0xb6:3,0xb7:3,0xb8:3,0xb9:5,0xba:5,0xbb:3,0xbc:2,0xbd:3,0xbe:1,
           0xbf:1,0xc0:3,0xc1:3,0xc2:1,0xc3:1,0xc4:-3,0xc5:4,0xc6:3,0xc7:3,0xc8:5,
           0xc9:5,0xca:1,0xfb:2,0xfc:2,0xfd:2,0xfe:2,0xff:2}
    ln = LEN.get(op, 1)
    if ln == -1:  # tableswitch
        pad = (4 - ((j + 1) % 4)) % 4
        default, lo, hi = struct.unpack_from(">iii", code, j + 1 + pad)
        ln = 1 + pad + 12 + 4 * (hi - lo + 1)
    elif ln == -2:  # lookupswitch
        pad = (4 - ((j + 1) % 4)) % 4
        default, npairs = struct.unpack_from(">ii", code, j + 1 + pad)
        ln = 1 + pad + 8 + 8 * npairs
    elif ln == -3:  # wide
        wop = code[j + 1]
        ln = 6 if wop == 0x84 else 4
    j += ln

# the four collection pcs (from the clinit walk): the indys at 29305/29310/29315/29320
WANT = [29305, 29310, 29315, 29320]
for wpc in WANT:
    bi = indys.get(wpc)
    print(f"--- indy @ {wpc}: bootstrap #{bi}")
    if bi is None:
        print("    MISSING")
        continue
    mh_ref, args = boots[bi]
    print(f"    bootstrap: {mh_string(mh_ref)}")
    for a in args:
        e = cp[a]
        if e[0] == 16:  # MethodType
            print(f"    arg MethodType: {utf(cp, e[1])}")
        elif e[0] == 15:  # MethodHandle
            print(f"    arg MethodHandle: {mh_string(a)}")
        elif e[0] == 10:  # Methodref (impl method maybe)
            print(f"    arg Methodref: {ref_name(cp, a)}")
        else:
            print(f"    arg tag{e[0]}: {utf(cp, a) if e[0] == 1 else e}")

# ALSO: the stairs collection + bars + chains + lantern collections' pcs for cross-check?
print()
print("=== all indys whose bootstrap lambda target mentions SlabBlock (full census)")
for pc2, bi in sorted(indys.items()):
    mh_ref, args = boots[bi]
    for a in args:
        e = cp[a]
        if e[0] == 15:
            s = ref_name(cp, e[2])
            if "SlabBlock" in s or "lambda$static$" in s:
                pass  # too noisy; skip
print("(done)")
