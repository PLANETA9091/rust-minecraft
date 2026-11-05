#!/usr/bin/env python3
"""press_468 — the merchant's own negative-scroll press fold the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
TradeOfferButton's own press the lambda answers — the getIndex +
getfield scrollOff fold the postButtonClick serve, the NEGATIVE SEAT
the fold never guards [the -1 band the 467 census wore the shopItem
the -1 seat banks — the offers.get(-1) tail the 466 crash arm
speaks], the shopItem field the pressed row banks, the
setSelectionHint + tryMoveItems pair the client-local halves speak,
the ServerboundSelectTradePacket the send tail carries, the
mouseClicked the isActive gate the press opens with. Usage: python3
scripts/jar_work_parse_press_468.py > scripts/jar_work/press_468.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen$TradeOfferButton.class",
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
]
WANTED = {
    "mouseClicked",
    "onPress",
    "postButtonClick",
    "lambda",
    "getIndex",
}


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
                dump_class(data, lambda n, d: any(k in n for k in WANTED))
    text = buf.getvalue()
    print(text, end="")
    # the one lambda block the fold opens — the getIndex invoke the
    # slice anchors, the next METHOD header the block closes
    fold_start = text.find("invokevirtual net/minecraft/client/gui/screens/inventory/MerchantScreen$TradeOfferButton.getIndex:()I")
    fold = ""
    if fold_start >= 0:
        hdr = text.rfind("METHOD ", 0, fold_start)
        nxt = text.find("\n  METHOD ", fold_start)
        fold = text[hdr : nxt if nxt >= 0 else len(text)]
    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak)
    verdicts = [
        ("the TradeOfferButton class the census wears (the index getter)",
         "METHOD getIndex ()I" in text),
        ("the press the mouseClicked gate opens (isActive)",
         "METHOD mouseClicked" in text),
        ("the postButtonClick walk the lambda serves",
         "METHOD postButtonClick" in text),
        ("the getIndex invoke the fold speaks",
         "TradeOfferButton.getIndex:()I" in text),
        ("the scrollOff field the fold reads",
         "getfield net/minecraft/client/gui/screens/inventory/MerchantScreen.scrollOff:I" in fold),
        ("the fold the shopItem field banks (putfield)",
         "putfield net/minecraft/client/gui/screens/inventory/MerchantScreen.shopItem:I" in fold),
        ("the negative seat the fold never guards (no bounds word)",
         bool(fold) and "arraylength" not in fold and "size:()I" not in fold),
        ("the shopItem walk the client-local halves serve",
         "setSelectionHint:(I)V" in text and "tryMoveItems:(I)V" in text),
        ("the Select Trade packet the send tail carries",
         "ServerboundSelectTradePacket.<init>:(I)V" in text),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
