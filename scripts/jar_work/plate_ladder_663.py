#!/usr/bin/env python3
"""THE PLATE FAMILY'S FULL LADDER probe (the autopush-663 round): dump the
Blocks clinit factory rows for the TWELVE remaining pressure plates (the
spruce/birch/jungle/acacia/dark_oak/mangrove/cherry/pale_oak/bamboo/crimson/
warped woods + polished_blackstone) — the ctor shapes, the sensitivity args,
the BlockSetType voice rows, the indy bootstrap numbering — plus the cache's
sounds.json rows the five click-voice sets carry. javap absent — the JRE's
own bytecode the python walk reads (the same law the tame_probe keeps).

Usage: python3 scripts/jar_work/plate_ladder_663.py <client.jar>
"""
import json
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"

PLATES = [
    "spruce_pressure_plate", "birch_pressure_plate", "jungle_pressure_plate",
    "acacia_pressure_plate", "dark_oak_pressure_plate", "mangrove_pressure_plate",
    "cherry_pressure_plate", "pale_oak_pressure_plate", "bamboo_pressure_plate",
    "crimson_pressure_plate", "warped_pressure_plate",
    "polished_blackstone_pressure_plate",
]


def parse_cp(d):
    cpcount = struct.unpack_from(">H", d, 8)[0]  # magic(4) minor(2) major(2) then cpcount
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
    """Fieldref/Methodref/InterfaceMethodref -> Class.name.nt"""
    e = cp[i]
    cls = utf(cp, cp[e[1]][1])
    nt = cp[e[2]]
    return f"{cls}.{utf(cp, nt[1])}:{utf(cp, nt[2])}"


def mh_target(cp, i):
    """MethodHandle -> the Methodref's Class.name:desc"""
    e = cp[i]
    return ref_name(cp, e[2])


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
    0xa5: "if_acmpeq", 0xa6: "if_acmpne", 0xa7: "goto", 0xac: "ireturn",
    0xae: "freturn", 0xb0: "areturn", 0xb1: "return", 0xb2: "getstatic",
    0xb3: "putstatic", 0xb4: "getfield", 0xb5: "putfield",
    0xb6: "invokevirtual", 0xb7: "invokespecial", 0xb8: "invokestatic",
    0xb9: "invokeinterface", 0xba: "invokedynamic", 0xbb: "new",
    0xbc: "newarray", 0xbd: "anewarray", 0xbe: "arraylength", 0xbf: "athrow",
    0xc0: "checkcast", 0xc1: "instanceof", 0xc2: "monitorenter",
    0xc3: "monitorexit", 0xc6: "ifnull", 0xc7: "ifnonnull",
}

LEN1 = {0x10, 0x15, 0x17, 0x19, 0x36, 0x38, 0x3a, 0x84, 0xbc}
LEN2 = {0x11, 0x13, 0x84, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0,
        0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xb2, 0xb3, 0xb4, 0xb5,
        0xb6, 0xb7, 0xb8, 0xba, 0xbb, 0xbd, 0xc0, 0xc1, 0xc6, 0xc7}


def walk_code(cp, code):
    """Linear bytecode walk — yield (pc, mnemonic, detail)."""
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
        elif op == 0x13:
            j += 2
        elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xbb, 0xbd, 0xc0, 0xc1,
                    0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3,
                    0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xc6, 0xc7):  # refs + 2-byte branches
            idx = struct.unpack_from(">H", code, j)[0]; j += 2
            detail = ref_name(cp, idx) if op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8) else f"cp{idx}"
        elif op in (0xc8, 0xc9):  # goto_w / jsr_w
            j += 4
        elif op == 0xb9:
            j += 4
        elif op == 0xba:
            idx = struct.unpack_from(">H", code, j)[0]; j += 4
            e = cp[idx]
            bsm_idx = e[1]
            detail = f"bsm[{bsm_idx}]"
        elif op == 0x84:
            j += 2
        elif op == 0xbc:  # newarray atype
            j += 1
        elif op == 0xc5:  # multianewarray
            j += 3
        elif op == 0xc4:  # wide: wop(1) + index(2) [+ const(2) for iinc]
            wop = code[j]
            j += 4 if wop == 0x84 else 3
        elif op in (0xaa, 0xab):  # tableswitch / lookupswitch — skip rest
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
    p += 6  # access/this/super
    ifc = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc
    nf = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nf):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _fa in range(na):  # each attr = name(2) + len(4) + body
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


def main():
    with zipfile.ZipFile(JAR) as z:
        blocks = z.read("net/minecraft/world/level/block/Blocks.class")
        cp, cpend = parse_cp(blocks)
        # the class attribute table walk for clinit's Code
        for name, desc, aname, body in methods_of(blocks, cp, cpend):
            if aname != "Code":
                continue
            if name != "<clinit>":
                continue
            mq = 4
            clen = struct.unpack_from(">I", body, mq)[0]; mq += 4
            code = body[mq:mq + clen]
            stream = walk_code(cp, code)
            # segment the clinit by putstatic — the window BEFORE each
            # putstatic is that field's own factory setup (the register
            # call + the indy to the lambda factory)
            segments = {}
            cur = []
            for pc, mnem, detail in stream:
                if mnem == "putstatic" and detail.startswith("net/minecraft/world/level/block/Blocks."):
                    field = detail.split(".")[-1].split(":")[0]
                    segments[field] = cur
                    cur = []
                elif segments is not None:
                    cur.append((pc, mnem, detail))
            # the bootstrap table (a CLASS attribute, not a method attr)
            bsm = bootstrap_methods(blocks, cp, cpend)
            # the lambda bodies: bsm index -> implMethod -> lambda$static$N
            lambdas = {}
            for idx, (mhref, args) in bsm.items():
                if len(args) >= 2:
                    tgt = mh_target(cp, args[1])
                    lam = tgt.split(".")[-1].split(":")[0]
                    lambdas[idx] = lam
            # collect the lambda method bodies
            lam_code = {}
            for name, desc, aname, body in methods_of(blocks, cp, cpend):
                if aname == "Code" and name.startswith("lambda$static$"):
                    clen = struct.unpack_from(">I", body, 4)[0]
                    lam_code[name] = body[8:8 + clen]
            for plate in PLATES:
                seg = segments.get(plate.upper())
                print(f"=== {plate} ===")
                if seg is None:
                    print("  (no clinit segment!)")
                    continue
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
                        # resolve ldc strings inside the lambda
                        if mnem == "ldc" and detail.startswith("cp"):
                            pass
                        print(f"    {pc:5d}: {mnem} {detail}")
            print("=== BOOTSTRAP MAP (indy -> lambda) ===")
            for idx in sorted(lambdas):
                print(f"  bsm[{idx}]: {lambdas[idx]}")

    # the sounds.json rows (the cache's jar layout the restore lane
    # keeps — the client jar itself may not carry the file)
    import os
    sj_path = os.path.join(os.path.dirname(JAR), "assets/minecraft/sounds.json")
    sj = None
    for cand in (sj_path,):
        if os.path.isfile(cand):
            sj = json.load(open(cand))
            break
    if sj is None:
        with zipfile.ZipFile(JAR) as z:
            try:
                sj = json.loads(z.read("assets/minecraft/sounds.json"))
            except KeyError:
                print("no sounds.json in cache or jar")
                return
    print("=== SOUNDS.JSON PLATE ROWS ===")
    for key in sorted(sj):
        if "pressure_plate" in key:
            row = sj[key]
            if isinstance(row, dict):
                print(f"  {key}: sounds={row.get('sounds')} volume={row.get('volume')} pitch={row.get('pitch')}")
            else:
                print(f"  {key}: {row}")


def class_attrs(d, cp, cpend):
    p = cpend
    p += 6
    ifc = struct.unpack_from(">H", d, p)[0]; p += 2 + 2 * ifc
    nf = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nf):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _fa in range(na):  # each attr = name(2) + len(4) + body
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    nm = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(nm):
        p += 6
        na = struct.unpack_from(">H", d, p)[0]; p += 2
        for _a in range(na):  # skip the methods' own attributes
            alen = struct.unpack_from(">I", d, p + 2)[0]
            p += 6 + alen
    # the CLASS attribute table (BootstrapMethods lives here)
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


if __name__ == "__main__":
    main()
