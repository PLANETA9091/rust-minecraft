#!/usr/bin/env python3
"""talk_474 — the talked-to-villager stat's own award site the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
Villager + the WanderingTrader the TALKED_TO_VILLAGER getstatic the
award walk answers — the method the award speaks, the guard the
client-side branch keeps. Usage: python3
scripts/jar_work_parse_talk_474.py > scripts/jar_work/talk_474.txt"""
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
WANTED = {
    "Villager": {"interactMob", "receiveTradeOffers", "startTradingPlayer",
                 "setTradingPlayer", "updateSpecialIngredients"},
    "WanderingTrader": {"interactMob", "setTradingPlayer",
                        "receiveTradeOffers"},
}


def wanted_for(cls):
    keep = WANTED.get(cls.split("/")[-1].replace(".class", ""), set())

    def wanted(name, desc):
        return name in keep or name.startswith("FIELD TALKED")

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
