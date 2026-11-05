#!/usr/bin/env python3
"""traded_471 — the stat's own traded lane the jar's own bytes speak
(the javap-less walk the JRE-only sandbox wears): the MerchantResultSlot's
onTake the awardStat TRADED_WITH_VILLAGER ride answers — the exact pcs
the census banks, the guard the success arm speaks, the Stats class the
custom stat's own id word wears. Usage: python3
scripts/jar_work_parse_traded_471.py > scripts/jar_work/traded_471.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/inventory/MerchantResultSlot.class",
    "net/minecraft/stats/Stats.class",
]
WANTED = {
    "MerchantResultSlot": {"onTake"},
    "Stats": {"<clinit>"},
}


def wanted_for(cls):
    keep = WANTED.get(cls.split("/")[-1].replace(".class", ""), set())

    def wanted(name, desc):
        if name.startswith("FIELD TRADED_WITH_VILLAGER"):
            return True
        return name in keep

    return wanted


def main():
    buf = io.StringIO()
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                return 1
            print(f"== {cls}")
            with contextlib.redirect_stdout(buf):
                dump_class(data, wanted_for(cls))
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
