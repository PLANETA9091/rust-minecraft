#!/usr/bin/env python3
"""tradeset_477 — the TradeSet + the VillagerTrade codecs the jar's own
bytes speak (the javap-less walk the JRE-only sandbox wears): the
TradeSet the amount/random_sequence/trades/allow_duplicates fields the
codec reads, the VillagerTrade the wants/gives/max_uses/reputation_
discount/modifiers the codec defaults answer. Usage: python3
scripts/jar_work_parse_tradeset_477.py > scripts/jar_work/tradeset_477.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/item/trading/TradeSet.class",
    "net/minecraft/world/item/trading/VillagerTrade.class",
    "net/minecraft/world/item/trading/ItemCost.class",
    "net/minecraft/world/item/trading/TradeCost.class",
]


def main():
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        with zipfile.ZipFile(JAR) as z:
            for cls in CLASSES:
                data = z.read(cls)
                dump_class(data, lambda name, desc: True)
    print(buf.getvalue())


if __name__ == "__main__":
    main()
