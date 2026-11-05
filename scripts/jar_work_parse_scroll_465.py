#!/usr/bin/env python3
"""scroll_465 — the merchant's own stale-scroll retire the jar's own
bytes speak (the javap-less walk the JRE-only sandbox wears): the
extractContents window walk (the canScroll verdict the item walk opens
with, the `i10 < scrollOff` skip + the `i10 < 7 + scrollOff` draw pair
the >7 bank wears, the EVERY-offer walk the ≤7 bank answers — the
pc 126 ifeq the window never wears, the y walker the drawn rows
advance), the canScroll `size > 7` verdict, the mouseScrolled gate
[the clamp the scrollable bank alone answers — the stale word the ≤7
bank keeps], the mouseDragged max fold, the release's own disarm.
Usage: python3 scripts/jar_work_parse_scroll_465.py >
scripts/jar_work/scroll_465.txt"""
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
WANTED = (
    "extractContents",
    "canScroll",
    "mouseScrolled",
    "mouseDragged",
    "extractScroller",
    "mouseReleased",
)


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
    verdicts = [
        ("the window walk the extractContents census wears",
         "METHOD extractContents" in text),
        ("the window skip pair the >7 bank wears (if_icmplt)",
         "if_icmplt" in text),
        ("the window draw pair the scrolled seat banks (7 + scrollOff)",
         "bipush 7" in text and "iadd" in text),
        ("the ≤7 bank the window never wears (the ifeq tail)",
         "ifeq" in text),
        ("the y walker the drawn rows advance (iinc 8 20)",
         "iinc 8 20" in text),
        ("the canScroll `size > 7` verdict the census wears",
         "METHOD canScroll" in text and "if_icmple" in text),
        ("the mouseScrolled clamp the scrollable bank alone answers",
         "METHOD mouseScrolled" in text and "Mth.clamp" in text),
        ("the drag's max fold the live size wears (isub)",
         "METHOD mouseDragged" in text and "isub" in text),
        ("the release's own disarm the iconst_0 speaks",
         "METHOD mouseReleased" in text and "iconst_0" in text),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
