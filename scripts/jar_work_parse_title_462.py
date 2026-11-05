#!/usr/bin/env python3
"""title_462 — the merchant title's own arg row the jar's own bytes
speak (the javap-less walk the JRE-only sandbox wears): the
MerchantScreen's own label walk (the title + the level word the render
wears), the VillagerData's own level seat (the displayName the tier
names), the MerchantMenu's own title source (the getDisplayName the
menu title rides). Usage:
python3 scripts/jar_work_parse_title_462.py > scripts/jar_work/title_462.txt"""
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/world/entity/npc/VillagerData.class",
    "net/minecraft/world/inventory/MerchantMenu.class",
]


def main():
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                continue
            print(f"== {cls}")
            dump_class(data, lambda n, d: True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
