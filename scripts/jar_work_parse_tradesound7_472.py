#!/usr/bin/env python3
"""tradesound7_472 — the Level family's own sound walk the jar's
bytes speak: EVERY playSound/playLocalSound the Level + ClientLevel
classes wear, the empty bodies + the client overrides the delivery
names. Usage: python3 scripts/jar_work_parse_tradesound7_472.py >
scripts/jar_work/tradesound7_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/level/Level.class",
    "net/minecraft/client/multiplayer/ClientLevel.class",
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
                dump_class(data, lambda n, d: "Sound" in n or "sound" in n)
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
