#!/usr/bin/env python3
"""autopush-313 scout4 — the dismount verdict's own vocabulary.

Dumps from the 26.2 jar:
  - EntityType.isBlockDangerous (the dangerous rows),
  - DismountHelper.nonClimbableShape + isBlockFloorValid + getBlockFloorHeight
    (the floor walk),
  - the INVALID_SPAWN_INSIDE tag rows (data/minecraft/tags/block),
  - the SURVIVES_EXPLOSION-ish... no — the fire/lava/magma rows the
    dangerous verdict rides.

Exit 0 iff the walks parse.
"""
import sys, zipfile, json

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
    print("=== EntityType: the dangerous verdict ===")
    dump(zf, 'net/minecraft/world/entity/EntityType', [
        'isBlockDangerous',
    ])
    print("=== DismountHelper: the floor walk ===")
    dump(zf, 'net/minecraft/world/entity/vehicle/DismountHelper', [
        'nonClimbableShape', 'isBlockFloorValid', 'getBlockFloorHeight',
    ])
    print("=== the INVALID_SPAWN_INSIDE tag rows ===")
    try:
        data = json.loads(zf.read('data/minecraft/tags/block/invalid_spawn_inside.json'))
        print(json.dumps(data, indent=1))
    except KeyError:
        print("!! tag file MISSING (new pack format?)")
        # try the newer directory layout
        for cand in ('data/minecraft/tags/blocks/invalid_spawn_inside.json',
                     'data/minecraft/tags/block_type/invalid_spawn_inside.json'):
            try:
                data = json.loads(zf.read(cand))
                print(cand)
                print(json.dumps(data, indent=1))
                break
            except KeyError:
                pass
    print("=== the fire / magma kinds (the dangerous vocabulary) ===")
    data = json.loads(zf.read('assets/minecraft/lang/en_us.json'))
    for k in sorted(data):
        if k in ('block.minecraft.fire', 'block.minecraft.magma_block', 'block.minecraft.lava',
                 'block.minecraft.campfire', 'block.minecraft.soul_campfire'):
            print(f"{k} = {data[k]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())
