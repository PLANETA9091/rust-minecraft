#!/usr/bin/env python3
"""autopush-313 scout5 — the respawn walk's own last rows.

Dumps the 26.2 bytecode for:
  - BlockBehaviour.isPossibleToRespawnInThis (the generic arm's
    verdict + the VALID_SPAWN tag read),
  - BedBlock.getShape + getCollisionShape (the 9-px collision the
    stand-up's above lane rides),
  - the VALID_SPAWN tag rows.

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
    print("=== BlockBehaviour: the respawn-possible verdict ===")
    dump(zf, 'net/minecraft/world/level/block/state/BlockBehaviour', [
        'isPossibleToRespawnInThis',
    ])
    print("=== Block: the override ===")
    dump(zf, 'net/minecraft/world/level/block/Block', [
        'isPossibleToRespawnInThis',
    ])
    print("=== BedBlock: the shape walk ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'getShape', 'getCollisionShape',
    ])
    print("=== the VALID_SPAWN tag rows ===")
    for cand in ('data/minecraft/tags/block/valid_spawn.json',
                 'data/minecraft/tags/blocks/valid_spawn.json'):
        try:
            data = json.loads(zf.read(cand))
            print(cand)
            print(json.dumps(data, indent=1))
            break
        except KeyError:
            pass
    return 0

if __name__ == '__main__':
    sys.exit(main())
