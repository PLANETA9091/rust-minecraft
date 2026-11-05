#!/usr/bin/env python3
"""stale_466 — the merchant's own stale-selection crash arm the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
extractContents tail the row loop's ifeq answers — the `getfield
shopItem` + the `iload 11` pair the MerchantOffers.get:(I) the walk
feeds, the ARRAYLENGTH BOUND the tail never wears — the stale
selection the shrunken bank wears the vanilla RENDER CRASH speaks
(the IndexOutOfBoundsException the render thread dies of), the
progress-bar + deprecated-tooltip walks the unguarded offer rides.
Usage: python3 scripts/jar_work_parse_stale_466.py >
scripts/jar_work/stale_466.txt"""
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
WANTED = {"extractContents"}


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
    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak)
    shop_item = text.find("getfield net/minecraft/client/gui/screens/inventory/MerchantScreen.shopItem:I")
    unguarded_get = text.find("invokevirtual net/minecraft/world/item/trading/MerchantOffers.get:(I)")
    between = text[shop_item:unguarded_get] if 0 <= shop_item < unguarded_get else ""
    verdicts = [
        ("the item walk the extractContents census wears",
         "METHOD extractContents" in text),
        ("the row loop's own end the tail follows (ifeq 299)",
         "ifeq 299" in text),
        ("the selected seat the tail reads (getfield shopItem)",
         shop_item >= 0),
        ("the get the walk feeds (MerchantOffers.get:(I))",
         unguarded_get >= 0),
        ("the bounds check the tail never wears (no arraylength between)",
         bool(between) and "arraylength" not in between),
        ("the progress bar the unguarded offer rides",
         "extractProgressBar" in text),
        ("the deprecated tooltip the unguarded offer rides",
         "DEPRECATED_TOOLTIP" in text),
        ("the out-of-stock verdict the selected offer answers",
         "isOutOfStock" in text),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
