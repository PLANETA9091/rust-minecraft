#!/usr/bin/env python3
"""Resolve the key strings the SoundEvents.<clinit> registers per field.

Autopush-461 audit, the honest walk: parse SoundEvents.class from the
26.2 jar, find <clinit>, scan the bytecode for the pattern
`ldc/ldc_w <str> -> invokestatic SoundEvent.register -> putstatic <FIELD>`
and resolve the string constant from the constant pool. No guessed
strides — the walk answers the exact key each field's own register
call carries.
"""
import struct
import zipfile
import sys

JAR = "/home/z/my-project/client262.jar"
CLASS = "net/minecraft/sounds/SoundEvents.class"

WANT = {
    "ANVIL_DESTROY", "ANVIL_LAND", "ANVIL_USE", "BOOK_PAGE_TURN",
    "GRINDSTONE_USE", "PHANTOM_BITE", "POINTED_DRIPSTONE_LAND",
    "POINTED_DRIPSTONE_DRIP_LAVA_INTO_CAULDRON",
    "POINTED_DRIPSTONE_DRIP_WATER_INTO_CAULDRON",
    "SMITHING_TABLE_USE", "SULFUR_SPIKE_LAND",
}

def parse_class(raw: bytes):
    off = 8
    (cp_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    pool = {}
    i = 1
    while i < cp_count:
        tag = raw[off]
        if tag == 1:
            (ln,) = struct.unpack_from(">H", raw, off + 1)
            pool[i] = ("utf8", raw[off + 3 : off + 3 + ln])
            off += 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            pool[i] = ("ref", struct.unpack_from(">H", raw, off + 1)[0])
            off += 3
        elif tag == 15:
            off += 4
            pool[i] = ("mh", None)
        elif tag in (3, 4):
            pool[i] = ("num", None)
            off += 5
        elif tag in (5, 6):
            pool[i] = ("num8", None)
            off += 9
            i += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            pool[i] = ("ref2", struct.unpack_from(">HH", raw, off + 1))
            off += 5
        else:
            raise SystemExit(f"unknown cp tag {tag} at index {i}")
        i += 1

    def utf8(idx):
        return pool[idx][1].decode("utf-8", "replace")

    def deref(idx):
        # tag 7/8/16/19/20 hold a Utf8 index
        return utf8(pool[idx][1])

    (access, this_c, super_c) = struct.unpack_from(">HHH", raw, off)
    off += 6
    (ifc_count,) = struct.unpack_from(">H", raw, off)
    off += 2 + 2 * ifc_count
    (fld_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    for _ in range(fld_count):
        off += 6
        (attr_count,) = struct.unpack_from(">H", raw, off)
        off += 2
        for _ in range(attr_count):
            (an, alen) = struct.unpack_from(">HI", raw, off)
            off += 6 + alen
    (mth_count,) = struct.unpack_from(">H", raw, off)
    off += 2
    methods = []
    for _ in range(mth_count):
        m_acc, m_name, m_desc = struct.unpack_from(">HHH", raw, off)
        off += 6
        (attr_count,) = struct.unpack_from(">H", raw, off)
        off += 2
        code = None
        for _ in range(attr_count):
            (an, alen) = struct.unpack_from(">HI", raw, off)
            name = utf8(an)
            body = raw[off + 6 : off + 6 + alen]
            if name == "Code":
                code = body
            off += 6 + alen
        methods.append((utf8(m_name), utf8(m_desc), code))
    return pool, utf8, deref, methods

def walk_clinit(code, pool, deref):
    (max_stack, max_locals, codelen) = struct.unpack_from(">HHI", code, 0)
    bc = code[8 : 8 + codelen]
    found = {}
    last_ldc = None
    saw_register = False
    i = 0
    while i < len(bc):
        op = bc[i]
        if op == 0x12:  # ldc 1-byte
            last_ldc = bc[i + 1]
            i += 2
        elif op == 0x13:  # ldc_w 2-byte
            last_ldc = struct.unpack_from(">H", bc, i + 1)[0]
            i += 3
        elif op == 0xB8:  # invokestatic
            saw_register = True
            i += 3
        elif op == 0xB3:  # putstatic
            cp = struct.unpack_from(">H", bc, i + 1)[0]
            # Fieldref -> (class, NameAndType) -> name
            if cp in pool and pool[cp][0] == "ref2":
                cls_i, nat_i = pool[cp][1]
                name_i = pool[nat_i][1][0]  # NameAndType = (name, desc)
                fname = pool[name_i][1].decode()
                if fname in WANT and saw_register and last_ldc is not None:
                    found[fname] = deref(last_ldc)
            saw_register = False
            last_ldc = None
            i += 3
        elif op in (0xC4,):  # wide
            nxt = bc[i + 1]
            i += 4 if nxt in (0x84, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0xA7) else 6
        elif op in (0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0xA7):  # branches
            i += 3
        elif op == 0xC6 or op == 0xC7:  # ifnull/ifnonnull
            i += 3
        else:
            i += 1
    return found

def main():
    with zipfile.ZipFile(JAR) as z:
        raw = z.read(CLASS)
    pool, utf8, deref, methods = parse_class(raw)
    for name, desc, code in methods:
        if name == "<clinit>" and code is not None:
            found = walk_clinit(code, pool, deref)
            for fname in sorted(WANT):
                print(f"{fname:45s} = {found.get(fname, '<NOT FOUND>')!r}")
            missing = WANT - set(found)
            if missing:
                print("MISSING:", missing, file=sys.stderr)
                return 1
    return 0

if __name__ == "__main__":
    sys.exit(main())
