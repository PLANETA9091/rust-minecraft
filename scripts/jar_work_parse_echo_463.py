#!/usr/bin/env python3
"""echo_463 — the trade's own echo bank the jar's own bytes speak (the
javap-less walk the JRE-only sandbox wears): the handleMerchantOffers
walk (the offers refresh the screen's own act wears — the selection
the refresh keeps or the reset speaks), the MerchantScreen's own
updateOffers/refresh seat, the MerchantMenu's own setOffers + the
selectionHint bank. Usage:
python3 scripts/jar_work_parse_echo_463.py > scripts/jar_work/echo_463.txt"""
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/multiplayer/ClientPacketListener.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/world/inventory/MerchantMenu.class",
]


def main():
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                continue
            print(f"== {cls}")
            # the big classes: dump only the wanted methods' names first
            if "ClientPacketListener" in cls:
                dump_class(data, lambda n, d: "MerchantOffers" in n)
            elif "MerchantScreen" in cls:
                dump_class(data, lambda n, d: n in ("init", "updateOffers",
                                                    "refreshOffers") or "merchant" in n.lower())
            else:
                dump_class(data, lambda n, d: n in ("setOffers", "setSelectionHint",
                                                    "getSelectionHint", "setXp",
                                                    "setMerchantLevel", "setShowProgressBar",
                                                    "setCanRestock", "showProgressBar",
                                                    "getTraderLevel", "offerCount",
                                                    "getOffers"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
