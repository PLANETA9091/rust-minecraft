#!/usr/bin/env python3
"""tradesound5_472 — the sound's own delivery the jar's bytes speak:
Level.playLocalSound + AbstractVillager.getNotifyTradeSound + Villager
overrides — the volume/pitch constants + the event the trade wears.
Usage: python3 scripts/jar_work_parse_tradesound5_472.py >
scripts/jar_work/tradesound5_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/level/Level.class",
    "net/minecraft/world/entity/npc/villager/AbstractVillager.class",
    "net/minecraft/world/entity/npc/villager/Villager.class",
    "net/minecraft/world/entity/npc/WanderingTrader.class",
]
WANTED = {
    "playLocalSound", "getNotifyTradeSound", "notifyTrade",
}


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
                dump_class(data, lambda n, d: n in WANTED)
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
