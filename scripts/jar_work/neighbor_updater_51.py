#!/usr/bin/env python3
"""THE NOT-GATE'S OWN CASCADE PROBE (the dev-51 round): dump the
neighbor-update chain's own walk — Level.blockUpdated/neighborsChanged
(the setBlock's flag-3 beat the broadcast serves), the
CollectingNeighborUpdater's own classes (the updateNeighborsAtExceptFrom
Facing walk the six-neighbors loop keeps, the FullNeighborUpdate the
neighborChanged call serves, the runUpdates depth the chain's own stack
banks), the InstantNeighborUpdater's direct walk, and the
RedstoneTorchBlock tick walk's own setBlock flag row (the levelEvent and
the scheduleTick args the burnout arms bank). javap absent — the JRE's
own bytecode the python walk reads (the same law the redstone_torch_666
probe keeps).

Usage: python3 scripts/jar_work/neighbor_updater_51.py <client.jar>
"""
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
        v = struct.unpack_from(">I", d, p)[0]; p += 4; return v

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
        try:
            if op == 0x10:
                detail = str(struct.unpack_from(">b", code, j)[0]); j += 1
            elif op == 0x11:
                detail = str(struct.unpack_from(">h", code, j)[0]); j += 2
            elif op == 0x12:
                idx = code[j]; j += 1
                e = cp[idx]
                detail = utf(cp, e[1]) if e[0] == "u" else f"ldc#{idx}"
                if e[0] in (3, 4):
                    detail = str(struct.unpack_from(">i" if e[0] == 3 else ">f", struct.pack(">I", e[1]))[0])
            elif op == 0x13:
                idx = struct.unpack_from(">H", code, j)[0]; j += 2
                detail = f"#{idx}"
            elif op in (0x15, 0x17, 0x19, 0x36, 0x38, 0x3a):
                detail = str(code[j]); j += 1
            elif op == 0x84:
                a, b = struct.unpack_from(">bb", code, j); j += 2
                detail = f"{a} {b}"
            elif op in (0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0,
                        0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xc6, 0xc7):
                off = struct.unpack_from(">h", code, j)[0]; j += 2
                detail = f"-> {pc + off}"
            elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8):
                idx = struct.unpack_from(">H", code, j)[0]; j += 2
                detail = ref_name(cp, idx)
            elif op == 0xb9:
                idx = struct.unpack_from(">H", code, j)[0]; j += 4
                detail = ref_name(cp, idx)
            elif op == 0xba:
                idx = struct.unpack_from(">H", code, j)[0]; j += 2
                detail = f"indy#{idx}"
            elif op in (0xbb, 0xbd, 0xc0, 0xc1):
                idx = struct.unpack_from(">H", code, j)[0]; j += 2
                e = cp[idx]
                detail = utf(cp, cp[e[1]][1])
            elif op == 0xc5:
                idx = struct.unpack_from(">H", code, j)[0]; j += 4
                detail = f"#{idx}"
            elif op in (0xbc, 0xa9):
                j += 1
            elif op in (0xc4, 0xc8, 0xc9):
                j += 2 if op == 0xc4 else 4
        except Exception as ex:
            detail = f"?{ex}"
        out.append(f"  pc {pc}: {mnem} {detail}".rstrip())
    return out


def dump_class(zf, name, want_methods=None):
    if not name.endswith(".class"):
        name += ".class"
    d = zf.read(name)
    cp, p = parse_cp(d)
    flags, this_cls, super_cls = struct.unpack_from(">HHH", d, p)
    p += 6
    # skip interfaces
    icount = struct.unpack_from(">H", d, p)[0]; p += 2
    p += 2 * icount
    # skip fields
    fcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(fcount):
        p += 6  # access, name idx, desc idx
        acount = struct.unpack_from(">H", d, p)[0]; p += 2
        for _ in range(acount):
            p += 2  # attribute name idx
            alen = struct.unpack_from(">I", d, p)[0]; p += 4
            p += alen
    lines = [f"=== {name} (this {utf(cp, this_cls)} super {utf(cp, super_cls)}) ==="]
    mcount = struct.unpack_from(">H", d, p)[0]; p += 2
    for _ in range(mcount):
        mflags, mname_i, mdesc_i = struct.unpack_from(">HHH", d, p)
        p += 6
        mname = utf(cp, mname_i)
        mdesc = utf(cp, mdesc_i)
        acount = struct.unpack_from(">H", d, p)[0]; p += 2
        code_body = None
        for _ in range(acount):
            aname_i = struct.unpack_from(">H", d, p)[0]
            p += 2
            alen = struct.unpack_from(">I", d, p)[0]
            p += 4
            adata = d[p:p + alen]
            p += alen
            if utf(cp, aname_i) == "Code":
                # the Code attribute's own head: no sub-pool — the max
                # stack + locals + len the outer cp's refs the walk reads
                max_stack, max_locals, clen = struct.unpack_from(">HHI", adata, 0)
                code_body = adata[8:8 + clen]
        if want_methods and mname not in want_methods:
            continue
        lines.append(f"--- {mname} {mdesc} flags 0x{mflags:04x} ---")
        if code_body is not None:
            lines.extend(walk_code(cp, code_body))
        else:
            lines.append("  (abstract/native/no code)")
    return "\n".join(lines)


def main():
    zf = zipfile.ZipFile(JAR)
    out = []
    # 1. THE LEVEL'S OWN BROADCAST (the blockUpdated walk the setBlock's
    # flag-3 beat serves, the neighborsChanged/updateNeighborsAt* the
    # six-neighbors loop keeps)
    lvl = dump_class(zf, "net/minecraft/world/level/Level",
                     {"blockUpdated", "neighborsChanged", "updateNeighborsAt",
                      "updateNeighborsAtExceptFromFacing", "neighborChanged",
                      "updateNeighbourShapes", "blockChanged"})
    out.append(lvl)
    # 2. THE COLLECTING UPDATER'S OWN QUEUE (the depth the stack banks,
    # the FullNeighborUpdate the neighborChanged call serves, the
    # runUpdates walk the six arms answer)
    out.append(dump_class(zf, "net/minecraft/world/level/redstone/CollectingNeighborUpdater",
                          {"updateNeighborsAtExceptFromFacing", "neighborChanged",
                           "shapeChanged", "runUpdates", "addAndRun", "countMerges"}))
    out.append(dump_class(zf, "net/minecraft/world/level/redstone/CollectingNeighborUpdater$FullNeighborUpdate"))
    # 3. THE INSTANT UPDATER'S OWN DIRECT WALK
    out.append(dump_class(zf, "net/minecraft/world/level/redstone/InstantNeighborUpdater",
                          {"updateNeighborsAtExceptFromFacing", "neighborChanged",
                           "shapeChanged"}))
    # 2b. THE MULTI NEIGHBOR UPDATE'S OWN SIX-LOOP (the runNext walk the
    # Direction.values() order serves — the cascade's own radius the
    # loop's arms answer) + THE INTERFACE'S OWN EXECUTE (the shape
    # updates + the neighborChanged the same walk serves)
    out.append(dump_class(zf, "net/minecraft/world/level/redstone/CollectingNeighborUpdater$MultiNeighborUpdate"))
    out.append(dump_class(zf, "net/minecraft/world/level/redstone/NeighborUpdater",
                          {"executeUpdate"}))
    # 4. THE TORCH'S OWN TICK + NEIGHBORCHANGED (the setBlock flag row
    # the scheduleTick args the burnout arms bank — the 666 probe's own
    # rows the same walk re-reads)
    out.append(dump_class(zf, "net/minecraft/world/level/block/RedstoneTorchBlock",
                          {"tick", "neighborChanged", "hasNeighborSignal",
                           "onPlace", "setLit"}))
    # 5. THE STATE BASE'S OWN NEIGHBORCHANGED DEFAULT (the empty law the
    # stone blocks wear — the cascade's own chain the verdict answers)
    out.append(dump_class(zf, "net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase",
                          {"neighborChanged"}))
    print("\n".join(out))


if __name__ == "__main__":
    main()
