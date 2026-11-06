#!/usr/bin/env python3
"""leash_knot_498 — the fence knot's own body the jar's own bytes speak
(the JRE-only walk the sandbox wears): the Entity.interact(Player,
InteractionHand, Vec3) code=491 walk the pc 197-295 SHEARS arm banks —
the Leashable checkcast + the shearOffAllLeashConnections call + the
hurtAndBreak durability lane + the SUCCESS verdict the walk answers,
the Leashable.shearOffAllLeashConnections default the
dropLeash(wasLeashed, true, false) walk serves. THE DUMPER'S OWN WALK:
the shared 461 module the switch opcodes never wore — the
tableswitch/lookupswitch/wide/multianewarray lanes THIS copy banks
(the shared artifact the untouched bytes keep). Usage: python3
scripts/jar_work_parse_leash_knot_498.py > scripts/jar_work/leash_knot_498.txt"""
import contextlib
import io
import struct
import sys
import zipfile

sys.path.insert(0, "scripts")
import jar_work_parse_sprites_461 as shared  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/entity/Entity.class",
]
WANTED = {
    "getRopeHoldPosition",
    "getEyeHeight",
    "getPassengerRidingPosition",
    "getKnot",
    "createKnot",
    "playPlacementSound",
    "interact",
    "tick",
    "dropAllLeashConnections",
    "removeAllLeashConnections",
    "isPickable",
    "hurt",
    "playAmbientSound",
    "recalculateBoundingBox",
    "setPos",
    "makeBoundingBox",
    "readAdditionalSaveData",
    "defineSynchedData",
}


def disasm(code, pool, indent="    "):
    """THE FULL WALK — the shared 461 disasm the switch lanes this copy
    banks (tableswitch 0xAA the 4-byte pad + the lo/hi + the (hi-lo+3)
    offsets wear, the lookupswitch 0xAB the pairs serve, the wide 0xC4
    the 2-byte-local/3-byte-iinc arms keep, the multianewarray 0xC5 the
    3-byte lane answers)."""
    out = []
    off = 0
    while off < len(code):
        pc = off
        op = code[off]
        name = shared.NAMES.get(op, f"op_{op:02x}")
        off += 1
        if op == 0x10:  # bipush
            v = struct.unpack_from(">b", code, off)[0]
            off += 1
            out.append(f"{indent}{pc}: {name} {v}")
        elif op == 0x11:  # sipush
            v = struct.unpack_from(">h", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {v}")
        elif op == 0x12:  # ldc
            i = code[off]
            off += 1
            out.append(f"{indent}{pc}: {name} {shared.resolve(pool, i)}")
        elif op in (0x13, 0x14):  # ldc_w, ldc2_w
            i = struct.unpack_from(">H", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {shared.resolve(pool, i)}")
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3A):
            idx = code[off]
            off += 1
            out.append(f"{indent}{pc}: {name} {idx}")
        elif op == 0x84:  # iinc
            idx, delta = struct.unpack_from(">Bb", code, off)
            off += 2
            out.append(f"{indent}{pc}: {name} {idx} {delta}")
        elif op in (0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1,
                    0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xC6, 0xC7):
            br = struct.unpack_from(">h", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {pc + br}")
        elif op == 0xAA:  # tableswitch
            pad = (4 - (off % 4)) % 4
            off += pad
            dflt, lo, hi = struct.unpack_from(">iii", code, off)
            off += 12
            n = hi - lo + 1
            offs = struct.unpack_from(f">{n}i", code, off)
            off += 4 * n
            targets = ", ".join(f"{lo + k}->{pc + v}" for k, v in enumerate(offs))
            out.append(f"{indent}{pc}: {name} lo={lo} hi={hi} dflt->{pc + dflt} [{targets}]")
        elif op == 0xAB:  # lookupswitch
            pad = (4 - (off % 4)) % 4
            off += pad
            dflt, n = struct.unpack_from(">ii", code, off)
            off += 8
            pairs = []
            for _ in range(n):
                match, tgt = struct.unpack_from(">ii", code, off)
                off += 8
                pairs.append(f"{match}->{pc + tgt}")
            out.append(f"{indent}{pc}: {name} dflt->{pc + dflt} [{', '.join(pairs)}]")
        elif op == 0xC4:  # wide
            wop = code[off]
            if wop == 0x84:  # wide iinc
                idx, delta = struct.unpack_from(">Hh", code, off + 1)
                off += 5
                out.append(f"{indent}{pc}: wide iinc {idx} {delta}")
            else:
                idx = struct.unpack_from(">H", code, off + 1)[0]
                off += 3
                out.append(f"{indent}{pc}: wide {shared.NAMES.get(wop, hex(wop))} {idx}")
        elif op == 0xC5:  # multianewarray
            i, dims = struct.unpack_from(">HB", code, off)
            off += 3
            out.append(f"{indent}{pc}: {name} {shared.resolve(pool, i)} {dims}")
        elif op == 0xBA:  # invokedynamic
            i = struct.unpack_from(">H", code, off)[0]
            off += 4
            out.append(f"{indent}{pc}: invokedynamic {shared.resolve(pool, i)}")
        elif op in (0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xBB, 0xBD, 0xC0, 0xC1):
            i = struct.unpack_from(">H", code, off)[0]
            off += 2
            out.append(f"{indent}{pc}: {name} {shared.resolve(pool, i)}")
        elif op == 0xB9:  # invokeinterface
            i = struct.unpack_from(">H", code, off)[0]
            off += 4
            out.append(f"{indent}{pc}: {name} {shared.resolve(pool, i)}")
        elif op == 0xBC:  # newarray
            off += 1
            out.append(f"{indent}{pc}: {name}")
        else:
            out.append(f"{indent}{pc}: {name}")
    return out


def dump_class(data, wanted):
    pool = shared.read_const_pool(data)
    cp_count = struct.unpack_from(">H", data, 8)[0]
    off = 10
    i = 1
    while i < cp_count:
        tag = data[off]
        if tag == shared.CONSTANT_Utf8:
            ln = struct.unpack_from(">H", data, off + 1)[0]
            off += 3 + ln
        elif tag in (shared.CONSTANT_Long, shared.CONSTANT_Double):
            off += 9
            i += 1
        elif tag in (shared.CONSTANT_Field, shared.CONSTANT_Method,
                     shared.CONSTANT_Iface, shared.CONSTANT_NameAndType,
                     shared.CONSTANT_Integer, shared.CONSTANT_Float,
                     shared.CONSTANT_Dynamic, shared.CONSTANT_InvokeDynamic):
            off += 5
        elif tag == shared.CONSTANT_MethodHandle:
            off += 4
        else:
            off += 3
        i += 1
    off += 6  # access_flags + this_class + super_class
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc
    fs = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(fs):
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(ac):
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6 + alen
    ms = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(ms):
        mname, mdesc = struct.unpack_from(">HH", data, off + 2)
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        mname_s = shared.utf8(pool, mname)
        mdesc_s = shared.utf8(pool, mdesc)
        for _ in range(ac):
            aname_i = struct.unpack_from(">H", data, off)[0]
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6
            aname = shared.utf8(pool, aname_i)
            if aname == "Code" and wanted(mname_s, mdesc_s):
                clen2 = struct.unpack_from(">I", data, off + 4)[0]
                body = data[off + 8: off + 8 + clen2]
                print(f"  METHOD {mname_s} {mdesc_s}  (code={clen2})")
                for line in disasm(body, pool):
                    print(line)
            off += alen
    print()


def main():
    parts = []
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                parts.append(f"== {cls}: ABSENT")
                continue
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                dump_class(data, lambda n, d: n in WANTED)
            parts.append(f"== {cls}\n{buf.getvalue()}")

    out = "\n\n".join(parts)
    print(out, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
