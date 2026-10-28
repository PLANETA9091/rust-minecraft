#!/usr/bin/env python3
"""autopush-310 — THE HOPPER'S OWN CHORUS/END LANE scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - TeleportRandomlyConsumeEffect the apply walk (the diameter's fold
    the 16-attempt loop the x/y/z draw rows answer),
  - Entity.randomTeleport the safe-position law (the y-walk the
    collision verdicts the state broadcast answer),
  - EndGatewayBlockEntity the teleport walk (the cooldown gate the
    exit-portal read the entity kinds the cooldown set banks),
  - EndGatewayBlock the shape walk (the empty-shape law the
    entityInside gate answers).

Exit 0 iff the walks parse.
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

def dump(zf, path, wants):
    try:
        data = zf.read(path + '.class')
    except KeyError:
        print(f"!! {path} MISSING")
        return
    for name, desc, code, cp in parse_class(data, set(wants)):
        if code is None:
            continue
        print(f"== {path.rsplit('/',1)[-1]} :: {name} {desc} ==")
        for ln in disasm(code, cp):
            print(ln)

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== TeleportRandomlyConsumeEffect: the apply walk ===")
    dump(zf, 'net/minecraft/world/item/consume_effects/TeleportRandomlyConsumeEffect', [
        'apply', '<init>', '<clinit>', 'diameter',
    ])
    print("\n=== ChorusFruitItem: the finish walk (the legacy lane) ===")
    dump(zf, 'net/minecraft/world/item/ChorusFruitItem', [
        'finishUsingItem', 'apply',
    ])
    print("\n=== LivingEntity.randomTeleport: the safe-position law ===")
    dump(zf, 'net/minecraft/world/entity/LivingEntity', [
        'randomTeleport',
    ])
    print("\n=== TheEndGatewayBlockEntity: the teleport walk ===")
    dump(zf, 'net/minecraft/world/level/block/entity/TheEndGatewayBlockEntity', [
        'teleportEntity', 'isCoolingDown', 'setCoolDown', '<clinit>', '<init>',
        'entityInside', 'getExitPortal', 'findExitPosition', 'cooldown',
    ])
    print("\n=== EndGatewayBlock: the shape + the inside gate ===")
    dump(zf, 'net/minecraft/world/level/block/EndGatewayBlock', [
        'getShape', 'entityInside', '<clinit>',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())
