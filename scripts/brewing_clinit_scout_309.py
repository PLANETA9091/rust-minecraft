#!/usr/bin/env python3
"""autopush-309 — the PotionBrewing clinit walk (the ingredient rows).

Dumps PotionBrewing <clinit> (the Builder adds the ingredient set
speaks) + the isPotionIngredient / isContainerIngredient verdicts.
Exit 0 iff parses.
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

def dump(zf, path, wants, max_lines=400):
    try:
        data = zf.read(path + '.class')
    except KeyError:
        print(f"!! {path} MISSING")
        return
    for name, desc, code, cp in parse_class(data, set(wants)):
        if code is None:
            continue
        print(f"== {path.rsplit('/',1)[-1]} :: {name} {desc} ==")
        lines = list(disasm(code, cp))
        for ln in lines[:max_lines]:
            print(ln)
        if len(lines) > max_lines:
            print(f"  ... ({len(lines) - max_lines} more)")

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== PotionBrewing: the clinit the ingredient rows speak ===")
    dump(zf, 'net/minecraft/world/item/alchemy/PotionBrewing', ['<clinit>'])
    print("\n=== PotionBrewing: the verdict pairs ===")
    dump(zf, 'net/minecraft/world/item/alchemy/PotionBrewing', [
        'isPotionIngredient', 'isContainerIngredient', 'isContainer',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())
