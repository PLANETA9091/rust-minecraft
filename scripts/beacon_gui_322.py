#!/usr/bin/env python3
"""autopush-322 — THE BEACON'S OWN GUI DOOR probe.

Banks the full 26.2 disasm of the beacon GUI lane's classes:
  - BeaconBlock            (the useWithoutItem walk the door speaks)
  - BeaconMenu (+PaymentSlot) (the slot law, the DataSlots, VALID_EFFECTS)
  - BeaconScreen (+inner buttons) (the layout, the confirm/cancel walks)
  - BeaconBlockEntity      (applyEffects + the save/load pair)
  - ServerboundSetBeaconPacket (the wire lane)

Dumps to scripts/jar_work/beacon_gui_322.txt. Exit 0 iff every class parses.
"""
import io
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8, disasm

JAR = '.mc2rs/classpath/client.jar'
OUT = 'scripts/jar_work/beacon_gui_322.txt'

# (class, needle) — '' = every method; a needle = matching methods only
TARGETS = [
    ('net/minecraft/world/level/block/BeaconBlock', ''),
    ('net/minecraft/world/inventory/BeaconMenu', ''),
    ('net/minecraft/world/inventory/BeaconMenu$PaymentSlot', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconScreenButton', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconSpriteScreenButton', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconPowerButton', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconUpgradePowerButton', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconConfirmButton', ''),
    ('net/minecraft/client/gui/screens/inventory/BeaconScreen$BeaconCancelButton', ''),
    ('net/minecraft/world/level/block/entity/BeaconBlockEntity', 'applyEffects'),
    ('net/minecraft/world/level/block/entity/BeaconBlockEntity', 'save'),
    ('net/minecraft/world/level/block/entity/BeaconBlockEntity', 'load'),
    ('net/minecraft/world/level/block/entity/BeaconBlockEntity', 'setPrimaryEffect'),
    ('net/minecraft/world/level/block/entity/BeaconBlockEntity', 'setSecondaryEffect'),
    ('net/minecraft/network/protocol/game/ServerboundSetBeaconPacket', ''),
]


def dump_class(zf, cls, needle, out):
    data = zf.read(f'{cls}.class')
    cp, n = parse_cp(data)
    out.write(f"\n#### {cls} (cp {n}) needle={needle!r}\n")
    i = n + 6  # access_flags, this_class, super_class
    nif = struct.unpack_from('>H', data, i)[0]
    i += 2 + 2 * nif
    nfields = struct.unpack_from('>H', data, i)[0]
    i += 2
    for _ in range(nfields):
        fname = utf8(cp, struct.unpack_from('>H', data, i + 2)[0])
        fdesc = utf8(cp, struct.unpack_from('>H', data, i + 4)[0])
        i += 6
        attrs = struct.unpack_from('>H', data, i)[0]
        i += 2
        for _ in range(attrs):
            i += 2  # the attr's name_idx the walk skips
            alen = struct.unpack_from('>I', data, i)[0]
            i += 4 + alen
        out.write(f"  field {fname}: {fdesc}\n")
    nmethods = struct.unpack_from('>H', data, i)[0]
    i += 2
    for _ in range(nmethods):
        name = utf8(cp, struct.unpack_from('>H', data, i + 2)[0])
        desc = utf8(cp, struct.unpack_from('>H', data, i + 4)[0])
        i += 6
        attrs = struct.unpack_from('>H', data, i)[0]
        i += 2
        for _ in range(attrs):
            aname = utf8(cp, struct.unpack_from('>H', data, i)[0])
            alen = struct.unpack_from('>I', data, i + 2)[0]
            body = data[i + 6:i + 6 + alen]
            if aname == 'Code' and (needle in name or needle in f'{name}{desc}'):
                max_stack, max_locals, clen = struct.unpack_from('>HHI', body, 0)
                code = body[8:8 + clen]
                out.write(f"\n=== {name}{desc} max_stack={max_stack} max_locals={max_locals} ===\n")
                buf = io.StringIO()
                for start, mn, args in disasm(code, cp):
                    buf.write(f"  {start:5d}: {mn} {args}\n")
                out.write(buf.getvalue())
                out.write("\n")
            i += 6 + alen


def main():
    zf = zipfile.ZipFile(JAR)
    out = open(OUT, 'w')
    for cls, needle in TARGETS:
        dump_class(zf, cls, needle, out)
    out.close()
    print(f"banked {OUT}")
    return 0


if __name__ == '__main__':
    sys.exit(main())
