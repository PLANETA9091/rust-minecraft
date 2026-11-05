#!/usr/bin/env python3
"""trades_477 — the wanderer's own OFFER POOL the jar's own bytes speak
(the javap-less walk the JRE-only sandbox wears): the WanderingTrader +
the AbstractVillager the trade lanes answer — the updateTrades walk the
pool generation keeps, the receiveTradeOffers/getOffers the api speaks,
the VillagerTrades$... entries the trades fields the clinit names. The
476 suffix the 474's 475 wheel the round speaks. Usage: python3
scripts/jar_work_parse_trades_477.py > scripts/jar_work/trades_477.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"

# THE METHOD LIST (the class's own names the method table speaks) —
# the trade lanes the wanderer the api carries
CLASSES = [
    "net/minecraft/world/entity/npc/wanderingtrader/WanderingTrader.class",
    "net/minecraft/world/entity/npc/villager/AbstractVillager.class",
]


def main():
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        with zipfile.ZipFile(JAR) as z:
            for cls in CLASSES:
                data = z.read(cls)
                # the FULL dump (no method filter) — the method table +
                # the fields the trade lanes reveal
                dump_class(data, lambda name, desc: True)
    out = buf.getvalue()
    # the method/field signature lines only — the walk the lanes name
    for line in out.splitlines():
        print(line)


if __name__ == "__main__":
    main()
