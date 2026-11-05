#!/usr/bin/env python3
"""drag_467 — the merchant's own mid-drag shrink max fold the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
mouseDragged armed lane — the isDragging gate the walk opens with, the
LIVE size the getOffers().size() pair re-reads, the `size - 7` MAX
FOLD the canScroll gate NEVER guards [the isub the armed arm computes
unguarded — the shrunken bank the negative max wears], the Mth.clamp
(III) the [0, size-7] band wears [the max arm the clamp's own
if_icmpgt speaks — the negative band the weird verdict answers: the
clamp(0, 0, -1) the max arm returns], the putfield scrollOff the drag
tail banks. Usage: python3 scripts/jar_work_parse_drag_467.py >
scripts/jar_work/drag_467.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/util/Mth.class",
]
WANTED = {"mouseDragged", "clamp"}


def method_block(text: str, marker: str) -> str:
    """the one METHOD block the marker line opens — the next METHOD
    header the block closes"""
    start = text.find(marker)
    if start < 0:
        return ""
    nxt = text.find("\n  METHOD ", start + 1)
    return text[start : nxt if nxt >= 0 else len(text)]


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
    drag = method_block(text, "METHOD mouseDragged")
    clamp3 = method_block(text, "METHOD clamp (III)I")
    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak)
    verdicts = [
        ("the drag arm the mouseDragged census wears",
         "METHOD mouseDragged" in text),
        ("the armed gate the isDragging field speaks",
         "isDragging:Z" in drag and "ifeq" in drag),
        ("the live size the walk re-reads",
         "getOffers:()" in drag and "MerchantOffers.size:()I" in drag),
        ("the max fold the armed arm computes (isub 7)",
         "isub" in drag and "bipush 7" in drag),
        ("the canScroll gate the drag arm never wears",
         bool(drag) and "canScroll" not in drag),
        ("the clamp the [0, size-7] band wears (Mth.clamp:(III))",
         "Mth.clamp:(III)I" in drag),
        ("the int clamp the (III)I overload the census wears",
         "METHOD clamp (III)I" in text),
        ("the max/min fold the clamp wears (Math.max + Math.min)",
         "Math.max:(II)I" in clamp3 and "Math.min:(II)I" in clamp3),
        ("the branchless fold the negative band answers (no if_)",
         bool(clamp3) and "if_" not in clamp3),
        ("the scrollOff the drag tail banks (putfield)",
         "putfield" in drag and "scrollOff:I" in drag),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
