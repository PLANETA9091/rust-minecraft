#!/usr/bin/env python3
"""autopush-320 — THE BEAM LAMBDAS' OWN BODIES probe (the walk the
318 dump's cut tail answers): lambda$submitBeaconBeam$0 (the CORE's
own face walk), lambda$submitBeaconBeam$1 (the HALO's), + the
extract/submit method refs the alpha vertex-color question wears.
Exit 0 iff the walks parse."""
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp

JAR = '.mc2rs/classpath/client.jar'
CLASSES = [
    'net/minecraft/client/renderer/blockentity/BeaconRenderer',
]
WANT = ('lambda$submitBeaconBeam$0', 'lambda$submitBeaconBeam$1', 'submit', 'extract')


def main():
    zf = zipfile.ZipFile(JAR)
    for cls in CLASSES:
        data = zf.read(cls + '.class')
        cp, _ = parse_cp(data)
        print(f"\n########## {cls}")
        for mname, mdesc, mcode, mcp in parse_class(data, None):
            if not any(w in mname for w in WANT):
                continue
            print(f"--- {mname} {mdesc}")
            print(disasm(mcode, mcp))
    print("\n## done")
    return 0


if __name__ == '__main__':
    sys.exit(main())
