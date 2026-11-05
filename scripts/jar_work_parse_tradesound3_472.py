#!/usr/bin/env python3
"""tradesound3_472 — the MENU layer's own trade tail the jar's bytes
speak: MerchantMenu.tryMoveItems + MerchantResultSlot.onTake — the
sound the trade's own completion wears (the playSound callback /
SimpleSoundInstance the walk names). Usage: python3
scripts/jar_work_parse_tradesound3_472.py >
scripts/jar_work/tradesound3_472.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/inventory/MerchantMenu.class",
    "net/minecraft/world/inventory/MerchantResultSlot.class",
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
                    "tryMoveItems", "onTake", "onQuickCraft", "moveItemStackTo",
                })
    text = buf.getvalue()
    print(text, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
