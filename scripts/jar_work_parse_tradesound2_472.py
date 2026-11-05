#!/usr/bin/env python3
"""tradesound2_472 — the MerchantScreen's FULL method list the jar's
own bytes speak: every declared method the class wears, the callers
the trade sound's own lane names. Usage: python3
scripts/jar_work_parse_tradesound2_472.py >
scripts/jar_work/tradesound2_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$1.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$2.class",
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
                dump_class(data, lambda n, d: True)
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
