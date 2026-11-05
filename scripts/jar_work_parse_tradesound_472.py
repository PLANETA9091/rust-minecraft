#!/usr/bin/env python3
"""tradesound_472 — the merchant's own trade sound the jar's own bytes
speak (the javap-less walk the JRE-only sandbox wears): the
MerchantScreen's postButtonClick + playTradeSound + the button-press
walk the trade arm serves — the VILLAGER_TRADE event the forUI lane
plays, the volume/pitch constants the bytecode's ldc pairs wear, the
call sites the quick-move tail answers. Usage: python3
scripts/jar_work_parse_tradesound_472.py >
scripts/jar_work/tradesound_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
]
WANTED = {"postButtonClick", "playTradeSound", "onPress"}


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
                dump_class(data, lambda n, d: n in WANTED)
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
