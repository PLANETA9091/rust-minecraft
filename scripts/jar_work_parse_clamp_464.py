#!/usr/bin/env python3
"""clamp_464 — the merchant's own selection clamp the jar's own bytes
speak (the javap-less walk the JRE-only sandbox wears): the
TradeOfferButton's own onClick walk (the hint set the press answers,
the range the press never gaited), the MerchantScreen's own init +
updateOffers + tick (the SEVEN buttons the creation loop lays, the
visible/active rows the bank re-wears — the out-of-bank press the
widget gates or the walk answers), the MerchantMenu's own clicked +
tryMoveItems (the trade execution's own guard the slotId range wears),
the setSelectionHint/getSelectionHint pair the hint bank keeps.
Usage: python3 scripts/jar_work_parse_clamp_464.py > scripts/jar_work/clamp_464.txt"""
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$TradeOfferButton.class",
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
            if "MerchantScreen$TradeOfferButton" in cls:
                # the press walk + the visual state the bank re-wears
                dump_class(data, lambda n, d: n in (
                    "onClick", "onPress", "updateVisuals", "renderWidget",
                    "mouseMoved", "isHovering",
                ) or n == "<init>")
            elif "MerchantScreen" in cls:
                # the button creation loop + the per-bank row updates
                dump_class(data, lambda n, d: n in (
                    "init", "updateOffers", "refreshOffers", "tick",
                    "mouseClicked", "containerChanged",
                ) or n == "<init>")
            else:
                # the hint bank + the trade execution's own range guard
                dump_class(data, lambda n, d: n in (
                    "setSelectionHint", "getSelectionHint", "clicked",
                    "tryMoveItems", "canTakeItemForPickAll", "quickMoveStack",
                ) or n == "<init>")
    return 0


if __name__ == "__main__":
    sys.exit(main())
