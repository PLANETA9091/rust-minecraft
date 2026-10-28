#!/usr/bin/env python3
"""autopush-314 — THE BED'S OWN RESPAWN ANCHOR KIND scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - RespawnAnchorBlock the class walk (the useWithoutItem the static
    use the clinit the CHARGE property the SHAPES lambda the setCharge
    the canSetSpawn the explode the findRespawnPositionAndUseSpawnBlock
    the charge ladder answers),
  - DimensionType.respawnAnchorWorks the read (the canSetSpawn gate the
    dimension pair answers),
  - ServerPlayer.findRespawnPositionAndUseSpawnBlock the ANCHOR ARM the
    313 round's honest gap the walk retires (the RespawnAnchorBlock
    kind the static call the above-pair scan serves),
  - the tag/data rows the explosion decay answer (block_explosion_decay
    the generated table's anchor row the 26.2 data banks).

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
    print("=== RespawnAnchorBlock: the use walk + the static core ===")
    dump(zf, 'net/minecraft/world/level/block/RespawnAnchorBlock', [
        'useWithoutItem', 'use', 'setCharge', 'canSetSpawn',
        'explode', 'findRespawnPositionAndUseSpawnBlock', '<clinit>',
    ])
    print("=== DimensionType: the anchor-works read ===")
    dump(zf, 'net/minecraft/world/level/dimension/DimensionType', [
        'respawnAnchorWorks',
    ])
    print("=== ServerPlayer: the ANCHOR ARM the 313 gap the walk retires ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'findRespawnPositionAndUseSpawnBlock', 'findRespawnPosition',
    ])
    print("=== the 26.2 data rows (explosion decay + the lang rows) ===")
    for rel in ('data/minecraft/tags/block/wither_immune.json',
                'data/minecraft/tags/block/beacon_base_blocks.json'):
        try:
            print(rel, zf.read(rel).decode()[:400])
        except KeyError:
            print(rel, "MISSING")
    # the lang rows the anchor's own words speak
    try:
        lang = json.loads(zf.read('assets/minecraft/lang/en_us.json'))
        for k, v in lang.items():
            if 'respawn_anchor' in k or 'block.minecraft.respawn' in k:
                print("LANG", k, "=", v)
    except KeyError:
        print("lang MISSING")
    # the generated table's own row (the repo's census the block_dig the
    # anchor row banks — the destroy-time the dig speed the resistance)
    print("=== RespawnAnchorBlock fields the charge carry ===")
    dump(zf, 'net/minecraft/world/level/block/RespawnAnchorBlock', [
        'createBlockStateDefinition', 'getShape', 'isPathfindable',
        'getLightEmission', 'tryUsePortal',
    ])

if __name__ == '__main__':
    main()
