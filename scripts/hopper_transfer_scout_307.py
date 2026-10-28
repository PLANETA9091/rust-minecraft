#!/usr/bin/env python3
"""autopush-307 — THE HOPPER'S OWN TRANSFER TICK scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - HopperBlockEntity the tick walk (serverTick the tryMoveItems serve),
  - the push law (the attached container the facing direction answers),
  - the suck law (the above container + the item entities),
  - the cooldown (setCooldown the 8-tick the success gate banks).

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
        print()

def main():
    zf = zipfile.ZipFile(JAR)
    dump(zf, 'net/minecraft/world/level/block/entity/HopperBlockEntity',
         {'serverTick', 'tryMoveItems', 'getAttachedContainer',
          'hopperPushItems', 'hopperInsert', 'suckInItems',
          'setCooldown', 'isOnCooldown', 'getCooldown',
          'ejectItems', 'tryTakeInItemFromSlot', 'getSlots',
          'getSourceContainer', 'addItem', 'getItemsAtAndAbove',
          'inventoryFull', 'isEmpty', 'tick'})
    dump(zf, 'net/minecraft/world/level/block/entity/Hopper',
         {'setCooldown', 'isOnCooldown', 'getCooldown'})
    dump(zf, 'net/minecraft/world/level/block/entity/BlockEntity',
         {'setChanged'})
    return 0

if __name__ == '__main__':
    sys.exit(main())
