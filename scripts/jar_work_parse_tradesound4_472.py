#!/usr/bin/env python3
"""tradesound4_472 — the ENTITY layer's own notifyTrade the jar's
bytes speak: AbstractVillager.notifyTrade + Villager.notifyTrade —
the sound the trade's own completion wears. Usage: python3
scripts/jar_work_parse_tradesound4_472.py >
scripts/jar_work/tradesound4_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/entity/npc/villager/AbstractVillager.class",
    "net/minecraft/world/entity/npc/villager/Villager.class",
]


def main():
    buf = io.StringIO()
    with zipfile.ZipFile(JAR) as z:
        names = set(z.namelist())
        for cls in CLASSES:
            if cls not in names:
                print(f"== {cls}: ABSENT")
                continue
            data = z.read(cls)
            print(f"== {cls}")
            with contextlib.redirect_stdout(buf):
                dump_class(data, lambda n, d: n in {
                    "notifyTrade", "overrideXp", "getVillagerXp",
                })
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
