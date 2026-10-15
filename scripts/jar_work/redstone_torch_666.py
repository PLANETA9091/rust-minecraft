#!/usr/bin/env python3
"""THE REDSTONE TORCH'S OWN PROBE (the autopush-666 round): dump the
RedstoneTorchBlock class walk — the getSignal/getDirectSignal arms, the
NOT-gate consult (hasNeighborSignal), the tick walk's own delay — plus
the TorchBlock parent's canSurvive/ctor, the Blocks clinit row (the
lightLevel lambda, noCollision, pushReaction, the WOOD sound), the Items
clinit BlockItem pairing, the blockstate + model + template rows (the
template_redstone_torch / template_torch_unlit elements the derive walk
wears), the two PNGs' dims, and the wood sound rows the dig/place lanes
carry. javap absent — the JRE's own bytecode the python walk reads (the
same law the weighted_plate_49 / redstone_wire_665 probes keep).

Usage: python3 scripts/jar_work/redstone_torch_666.py <client.jar>
"""
import json
import os
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"


def parse_cp(d):
    cpcount = struct.unpack_from(">H", d, 8)[0]
    cp = [None] * cpcount
    p = 10
    i = 1

    def u1():
        nonlocal p
        v = d[p]; p += 1; return v

    def u2():
        nonlocal p
        v = struct.unpack_from(">H", d, p)[0]; p += 2; return v

    def u4():
        nonlocal p
        return struct.unpack_from(">I", d, (p := p + 4) - 4)[0]

    while i < cpcount:
        tag = u1()
        if tag == 1:
            ln = u2(); cp[i] = ("u", d[p:p + ln].decode("utf-8", "replace")); p += ln
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, u2())
        elif tag == 15:
            cp[i] = (tag, u1(), u2())
        elif tag in (3, 4):
            cp[i] = (tag, u4())
        elif tag in (5, 6):
            p += 8; i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, u2(), u2())
        else:
            raise Exception(f"tag {tag} @ {i}")
        i += 1
    return cp, p


def utf(cp, i):
    e = cp[i]
    return e[1] if e and e[0] == "u" else f"?{i}"


def ref_name(cp, i):
    e = cp[i]
    cls = utf(cp, cp[e[1]][1])
    nt = cp[e[2]]
    return f"{cls}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"


MNEMONIC = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0",
    0x04: "iconst_1", 0x05: "iconst_2", 0x06: "iconst_3", 0x07: "iconst_4",
    0x08: "iconst_5", 0x0b: "fconst_0", 0x0c: "fconst_1", 0x0d: "fconst_2",
    0x10: "bipush", 0x11: "sipush", 0x12: "ldc", 0x13: "ldc2_w",
    0x15: "iload", 0x17: "fload", 0x19: "aload", 0x1a: "iload_0",
    0x1b: "iload_1", 0x1c: "iload_2", 0x1d: "iload_3", 0x22: "fload_0",
    0x23: "fload_1", 0x24: "fload_2", 0x25: "fload_3", 0x2a: "aload_0",
    0x2b: "aload_1", 0x2c: "aload_2", 0x2d: "aload_3", 0x32: "aaload",
    0x36: "istore", 0x38: "fstore", 0x3a: "astore", 0x3d: "istore_2",
    0x3e: "istore_3", 0x45: "fstore_0", 0x46: "fstore_1", 0x47: "fstore_2",
    0x48: "fstore_3", 0x4b: "astore_0", 0x4c: "astore_1", 0x4d: "astore_2",
    0x4e: "astore_3", 0x57: "pop", 0x59: "dup", 0x5f: "swap", 0x60: "iadd",
    0x62: "fadd", 0x64: "isub", 0x66: "fsub", 0x6c: "idiv", 0x6e: "fdiv",
    0x70: "irem", 0x78: "ishl", 0x7a: "ishr", 0x7c: "iushr", 0x80: "ior",
    0x84: "iinc", 0x85: "i2l", 0x86: "i2f", 0x87: "i2d", 0x88: "l2i",
    0x8d: "f2i", 0x8e: "d2i", 0x91: "i2b", 0x92: "i2c", 0x94: "lcmp",
    0x95: "fcmpl", 0x96: "fcmpg", 0x97: "dcmpl", 0x98: "dcmpg",
    0x99: "ifeq", 0x9a: "ifne", 0x9b: "iflt", 0x9c: "ifge", 0x9d: "ifgt",
    0x9e: "ifle", 0x9f: "if_icmpeq", 0xa0: "if_icmpne", 0xa1: "if_icmplt",
    0xa2: "if_icmpge", 0xa3: "if_icmpgt", 0xa4: "if_icmple",
    0xa5: "if_acmpeq", 0xa6: "if_acmpne", 0xa7: "goto", 0xa8: "jsr",
    0xac: "ireturn", 0xad: "lreturn", 0xae: "freturn", 0xb0: "areturn",
    0xb1: "return", 0xb2: "getstatic", 0xb3: "putstatic", 0xb4: "getfield",
    0xb5: "putfield", 0xb6: "invokevirtual", 0xb7: "invokespecial",
    0xb8: "invokestatic", 0xb9: "invokeinterface", 0xba: "invokedynamic",
    0xbb: "new", 0xbc: "newarray", 0xbd: "anewarray", 0xbe: "arraylength",
    0xbf: "athrow", 0xc0: "checkcast", 0xc1: "instanceof",
    0xc2: "monitorenter", 0xc3: "monitorexit", 0xc6: "ifnull",
    0xc7: "ifnonnull",
}


def walk_code(cp, code):
    j = 0
    out = []
    while j < len(code):
        pc = j
        op = code[j]; j += 1
        mnem = MNEMONIC.get(op, f"0x{op:02x}")
        detail = ""
        if op == 0x10:
            detail = str(struct.unpack_from(">b", code, j)[0]); j += 1
        elif op == 0x11:
            detail = str(struct.unpack_from(">h", code, j)[0]); j += 2
        elif op == 0x12:
            idx = code[j]; j += 1
            e = cp[idx]
            detail = f'"{utf(cp, e[1])}"' if e[0] == 8 else f"cp{idx}"
        elif op in (0x13, 0x14):
            # ldc_w / ldc2_w: the 2-byte CP index the stream carries
            j += 2
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19):
            # iload/lload/fload/dload/aload: the 1-byte local index
            j += 1
        elif op in (0x36, 0x37, 0x38, 0x39, 0x3a):
            # istore/lstore/fstore/dstore/astore: the 1-byte local index
            j += 1
        elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xbb, 0xbd, 0xc0, 0xc1,
                    0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3,
                    0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xc6, 0xc7):
            idx = struct.unpack_from(">H", code, j)[0]; j += 2
            detail = ref_name(cp, idx) if op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8) else f"cp{idx}"
        elif op in (0xc8, 0xc9):
            j += 4
        elif op == 0xb9:
            j += 4
        elif op == 0xba:
            idx = struct.unpack_from(">H", code, j)[0]; j += 4
            e = cp[idx]
            detail = f"bsm[{e[1]}]"
        elif op == 0x84:
            j += 2
        elif op == 0xbc:
            j += 1
        elif op == 0xc5:
            j += 3
        elif op == 0xc4:
            wop = code[j]
            j += 4 if wop == 0x84 else 3
        elif op in (0xaa, 0xab):
            pad = (4 - (j % 4)) % 4
            j += pad
            if op == 0xaa:
                j += 12
            else:
                npairs = struct.unpack_from(">i", code, j + 4)[0]
                j += 8 + 8 * npairs
        out.append((pc, mnem, detail))
    return out


def methods_of(d, cp, cpend):
    p = cpend
    p += 6
    ifc = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc
    nf = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nf):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _fa in range(na):
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    nm = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nm):
        struct.unpack_from(">H", d, p)[0]; p += 2
        mn = struct.unpack_from(">H", d, p)[0]; p += 2
        md = struct.unpack_from(">H", d, p)[0]; p += 2
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _a in range(na):
            an = struct.unpack_from(">H", d, p)[0]
            alen = struct.unpack_from(">I", d, p + 2)[0]
            body = d[p + 6:p + 6 + alen]
            yield utf(cp, mn), utf(cp, md), utf(cp, an), body
            p += 6 + alen


def class_attrs(d, cp, cpend):
    p = cpend
    p += 6
    ifc = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc
    nf = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nf):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _fa in range(na):
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    nm = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nm):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _a in range(na):
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    nc = struct.unpack_from(">H", d, p)[0]; p += 2
    for _a in range(nc):
        an = struct.unpack_from(">H", d, p)[0]
        alen = struct.unpack_from(">I", d, p + 2)[0]
        body = d[p + 6:p + 6 + alen]
        yield utf(cp, an), body
        p += 6 + alen


def bootstrap_methods(d, cp, cpend):
    for aname, body in class_attrs(d, cp, cpend):
        if aname != "BootstrapMethods":
            continue
        n = struct.unpack_from(">H", body, 0)[0]
        out = {}
        p = 2
        for i in range(n):
            mhref = struct.unpack_from(">H", body, p)[0]
            nargs = struct.unpack_from(">H", body, p + 2)[0]
            args = [struct.unpack_from(">H", body, p + 4 + 2 * k)[0] for k in range(nargs)]
            out[i] = (mhref, args)
            p += 4 + 2 * nargs
        return out
    return {}


def mh_target(cp, i):
    e = cp[i]
    return ref_name(cp, e[2])


def dump_class_methods(z, cls, only=None):
    d = z.read(cls + ".class")
    cp, cpend = parse_cp(d)
    print(f"===== {cls} =====")
    for name, desc, aname, body in methods_of(d, cp, cpend):
        if aname != "Code":
            continue
        if only and name not in only:
            continue
        mq = 4
        clen = struct.unpack_from(">I", body, mq)[0]; mq += 4
        code = body[mq:mq + clen]
        print(f"  --- {name} {desc} ---")
        for pc, mnem, detail in walk_code(cp, code):
            print(f"    {pc:5d}: {mnem} {detail}")
    return cp, cpend


def clinit_segments(z, cls):
    d = z.read(cls + ".class")
    cp, cpend = parse_cp(d)
    segments = {}
    cur = []
    for name, desc, aname, body in methods_of(d, cp, cpend):
        if aname != "Code" or name != "<clinit>":
            continue
        mq = 4
        clen = struct.unpack_from(">I", body, mq)[0]; mq += 4
        code = body[mq:mq + clen]
        for pc, mnem, detail in walk_code(cp, code):
            if mnem == "putstatic" and detail.startswith(cls + "."):
                field = detail.split(".")[-1].split(":")[0]
                segments[field] = cur
                cur = []
            else:
                cur.append((pc, mnem, detail))
    bsm = bootstrap_methods(d, cp, cpend)
    lambdas = {}
    for idx, (mhref, args) in bsm.items():
        if len(args) >= 2:
            tgt = mh_target(cp, args[1])
            lambdas[idx] = tgt.split(".")[-1].split(":")[0]
    lam_code = {}
    for name, desc, aname, body in methods_of(d, cp, cpend):
        if aname == "Code" and name.startswith("lambda$static$"):
            clen = struct.unpack_from(">I", body, 4)[0]
            lam_code[name] = body[8:8 + clen]
    return segments, lambdas, lam_code, cp


def print_segment(segments, lambdas, lam_code, cp, field):
    print(f"=== clinit segment: {field} ===")
    seg = segments.get(field)
    if seg is None:
        near = sorted(k for k in segments if "TORCH" in k or "REDSTONE" in k)
        print(f"  (no segment; TORCH/REDSTONE keys present: {near})")
        return
    lam_name = None
    for pc, mnem, detail in seg:
        extra = ""
        if mnem == "invokedynamic":
            idx = int(detail[4:-1])
            lam_name = lambdas.get(idx, "?")
            extra = f"  -> {lam_name}"
        print(f"  {pc:5d}: {mnem} {detail}{extra}")
    if lam_name and lam_name in lam_code:
        print(f"  --- {lam_name} body ---")
        for pc, mnem, detail in walk_code(cp, lam_code[lam_name]):
            print(f"    {pc:5d}: {mnem} {detail}")


def png_dims(raw):
    # the IHDR's width/height the bytes 16..24 carry (BE u32 pair)
    w, h = struct.unpack_from(">II", raw, 16)
    return w, h


def main():
    with zipfile.ZipFile(JAR) as z:
        # 1. the RedstoneTorchBlock's OWN walk — every method the class
        # banks (the getSignal/getDirectSignal arms, the NOT-gate
        # consult, the tick walk's delay, isSignalSource)
        dump_class_methods(z, "net/minecraft/world/level/block/RedstoneTorchBlock")

        # 2. the TorchBlock parent's canSurvive + ctor
        dump_class_methods(z, "net/minecraft/world/level/block/TorchBlock")

        # 3. the Blocks clinit row: the lightLevel lambda, noCollision,
        # pushReaction, the WOOD sound, the strength rows
        segments, lambdas, lam_code, cp = clinit_segments(
            z, "net/minecraft/world/level/block/Blocks")
        print_segment(segments, lambdas, lam_code, cp, "REDSTONE_TORCH")

        # 4. the Items clinit row: the BlockItem pairing
        iseg, ilam, ilamc, icp = clinit_segments(
            z, "net/minecraft/world/item/Items")
        print_segment(iseg, ilam, ilamc, icp, "REDSTONE_TORCH")

        # 5. the blockstate + model + template rows
        for path in (
            "assets/minecraft/blockstates/redstone_torch.json",
            "assets/minecraft/models/block/redstone_torch.json",
            "assets/minecraft/models/block/redstone_torch_off.json",
            "assets/minecraft/models/block/template_redstone_torch.json",
            "assets/minecraft/models/block/template_torch_unlit.json",
            "assets/minecraft/models/block/template_torch.json",
        ):
            print(f"=== {path} ===")
            try:
                print(z.read(path).decode())
            except KeyError:
                print("  (absent)")

        # 6. the two PNGs' dims
        for tex in ("redstone_torch", "redstone_torch_off"):
            raw = z.read(f"assets/minecraft/textures/block/{tex}.png")
            w, h = png_dims(raw)
            print(f"=== texture {tex}.png: {w}x{h}, {len(raw)} bytes ===")

    # 7. the sounds.json wood rows the dig/place lanes carry
    sj = None
    with zipfile.ZipFile(JAR) as z:
        try:
            sj = json.loads(z.read("assets/minecraft/sounds.json"))
        except KeyError:
            print("no sounds.json in jar")
            sj = None
    if sj:
        print("=== SOUNDS.JSON wood/torch ROWS ===")
        for key in sorted(sj):
            if key.startswith("block.wood.") and key.split(".")[-1] in ("break", "place", "step", "hit", "fall"):
                row = sj[key]
                if isinstance(row, dict):
                    snds = row.get("sounds")
                    n = len(snds) if isinstance(snds, list) else 1
                    print(f"  {key}: {n} sound rows, volume={row.get('volume')} pitch={row.get('pitch')}")


if __name__ == "__main__":
    main()
