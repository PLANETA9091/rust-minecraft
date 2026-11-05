#!/usr/bin/env python3
"""clamp_464b — the shopItem's own write walk: every method name the
MerchantScreen + TradeOfferButton classes wear (the synthetic lambdas
the press answers, the scroll walk the shopItem seat re-keys, the
updateVisuals/renderWidget rows the bank re-wears). Usage:
python3 scripts/jar_work_parse_clamp_464b.py > scripts/jar_work/clamp_464b.txt"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$TradeOfferButton.class",
]

# the method LIST walk: name + desc + code size, nothing disassembled


def list_methods(data):
    pool_methods = []

    def wanted(n, d):
        pool_methods.append((n, d))
        return False

    dump_class(data, wanted)
    return pool_methods


def main():
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                continue
            print(f"== {cls}")
            dump_class(data, lambda n, d: "lambda" in n or "onScroll" in n
                       or "scroll" in n.lower() or "updateVisuals" in n)
    return 0


if __name__ == "__main__":
    sys.exit(main())
