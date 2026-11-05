#!/usr/bin/env python3
"""talk2_474 — the Villager + the WanderingTrader's FULL method walk
the jar's own bytes speak: every declared method both classes wear,
the TALKED_TO_VILLAGER getstatic the award method's own bytes answer.
Usage: python3 scripts/jar_work_parse_talk2_474.py >
scripts/jar_work/talk2_474.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/entity/npc/villager/Villager.class",
    "net/minecraft/world/entity/npc/wanderingtrader/WanderingTrader.class",
]


def main():
    buf = io.StringIO()
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            data = z.read(cls)
            print(f"== {cls}")
            with contextlib.redirect_stdout(buf):
                dump_class(data, lambda n, d: True)
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
