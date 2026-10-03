#!/usr/bin/env python3
"""cp_531.py — the brush-dust forensics (autopush-489: the
spawnDustParticles walk the client's dust lane this round wears).

1. Resolve BrushItem's own CP entries the dust walk loads: #206
   (ldc2_w double, the velocity multiplier d) and #256 (ldc_w float,
   the WEST/NORTH hit-location offset).
2. Resolve BrushItem$DustParticlesDelta's CP doubles #48/#50/#52 the
   fromDirection switch arms load.
3. Disassemble BrushItem$1.<clinit> — the synthetic $SwitchMap$
   net$minecraft$core$Direction map — to pin which Direction ordinal
   lands on which fromDirection switch case (1..6).
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402

JAR = "/home/z/my-project/client262.jar"
BRUSH = "net/minecraft/world/item/BrushItem.class"
DELTA = "net/minecraft/world/item/BrushItem$DustParticlesDelta.class"
SWITCH = "net/minecraft/world/item/BrushItem$1.class"


def cp_entry(cp, idx):
    return cp.get(idx)


def resolve_const(cp, idx):
    """Resolve a ldc/ldc_w/ldc2_w constant to a printable value."""
    e = cp.get(idx)
    if e is None:
        return f"#{idx}=<missing>"
    tag = e[0]
    if tag == "string":
        return f"#{idx}=str({e[1]!r})"
    if tag in (3, 4, 5, 6):
        bits = e[1] & 0xFFFFFFFFFFFFFFFF
        if tag == 6:
            val = struct.unpack(">d", struct.pack(">Q", bits))[0]
            return f"#{idx}=double={val!r} bits=0x{bits:016X}"
        if tag == 4:
            val = struct.unpack(">f", struct.pack(">I", bits & 0xFFFFFFFF))[0]
            return f"#{idx}=float={val!r} bits=0x{bits & 0xFFFFFFFF:08X}"
        return f"#{idx}=int={bits}"
    if tag == "method":
        return f"#{idx}=method {e[1]}"
    if tag == "field":
        return f"#{idx}=field {field_name(cp, idx)}"
    if tag == "iface":
        return f"#{idx}=iface {e[1]}"
    return f"#{idx}={e}"


def field_name(cp, ref_idx):
    """The Fieldref's class.name:descriptor (the member_name shape)."""
    e = cp.get(ref_idx)
    if not e or e[0] != "field":
        return f"#{ref_idx}"
    cls = utf8(cp, e[1][0])
    nt = cp.get(e[1][1])
    if nt and nt[0] == "name_type":
        return f"{cls}.{utf8(cp, nt[1][0])}"
    return f"{cls}#{e[1][1]}"


def load_cp(path):
    with zipfile.ZipFile(JAR) as jar:
        data = jar.read(path)
    cp, _ = parse_cp(data)
    return cp


brush_cp = load_cp(BRUSH)
delta_cp = load_cp(DELTA)

print("== BrushItem CP ==")
for idx in (206, 256):
    print(" ", resolve_const(brush_cp, idx))

print("== DustParticlesDelta CP ==")
for idx in (48, 50, 52):
    print(" ", resolve_const(delta_cp, idx))

print("== BrushItem$1 <clinit> (the $SwitchMap walk) ==")
with zipfile.ZipFile(JAR) as jar:
    data = jar.read(SWITCH)
cp, i = parse_cp(data)
i += 6  # access, this, super
nif = struct.unpack_from(">H", data, i)[0]
i += 2 + 2 * nif
nfields = struct.unpack_from(">H", data, i)[0]
i += 2
for _ in range(nfields):
    i += 6
    na = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(na):
        i += 2
        alen = struct.unpack_from(">I", data, i)[0]
        i += 4 + alen
nmethods = struct.unpack_from(">H", data, i)[0]
i += 2
for _ in range(nmethods):
    acc = struct.unpack_from(">H", data, i)[0]
    name = utf8(cp, struct.unpack_from(">H", data, i + 2)[0])
    desc = utf8(cp, struct.unpack_from(">H", data, i + 4)[0])
    i += 6
    na = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(na):
        aname = utf8(cp, struct.unpack_from(">H", data, i)[0])
        alen = struct.unpack_from(">I", data, i + 2)[0]
        body = data[i + 6:i + 6 + alen]
        if aname == "Code":
            print(f"=== {name}{desc} ===")
            code = body[8:]
            max_stack, max_locals, clen = struct.unpack_from(">HHI", body, 0)
            pc = 0
            pending_enum = None
            pending_val = None
            while pc < clen:
                op = code[pc]
                if op == 0xB2:  # getstatic
                    idx = struct.unpack_from(">H", code, pc + 1)[0]
                    fname = field_name(cp, idx)
                    if "Direction" in fname:
                        pending_enum = fname
                elif op == 0x04:  # iconst_1
                    pending_val = 1
                elif op == 0x05:  # iconst_2
                    pending_val = 2
                elif op == 0x06:
                    pending_val = 3
                elif op == 0x07:
                    pending_val = 4
                elif op == 0x08:
                    pending_val = 5
                elif op == 0x10:  # bipush
                    pending_val = struct.unpack_from(">b", code, pc + 1)[0]
                elif op == 0x11:  # sipush
                    pending_val = struct.unpack_from(">h", code, pc + 1)[0]
                elif op == 0x4F:  # iastore — the pair lands
                    if pending_enum is not None:
                        print(f"  {pending_enum} -> case {pending_val}")
                    pending_enum = None
                    pending_val = None
                pc += 1
        i += 6 + alen
