#!/usr/bin/env python3
"""clipboard_470 — the debug search box's own clipboard trio the jar's
own bytes speak (the javap-less walk the JRE-only sandbox wears): the
EditBox's keyPressed the KeyEvent walk the 26.2 build wears — the
isActive/isFocused gate the walk opens with, the isCopy/isPaste/isCut
arms the keyboardHandler store the set/getClipboard walks serve, the
getHighlighted the substring law the copy/cut pair answers, the
insertText the budget fold the replace(lo, hi, s) walk wears. Usage:
python3 scripts/jar_work_parse_clipboard_470.py >
scripts/jar_work/clipboard_470.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
import jar_work_parse_sprites_461 as _dj  # noqa: E402

# the None cp slots the long/double shift wears — the shared resolver
# the guarded wrapper the census keeps (the parser file the lane never
# touches)
_orig_resolve = _dj.resolve


def _safe_resolve(pool, i):
    if i < 0 or i >= len(pool) or pool[i] is None:
        return "<cp-unresolved>"
    return _orig_resolve(pool, i)


_dj.resolve = _safe_resolve
dump_class = _dj.dump_class

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/components/EditBox.class",
]
WANTED = {
    "keyPressed",
    "charTyped",
    "canConsumeInput",
    "deleteText",
    "insertText",
    "getValue",
    "setValue",
    "getHighlighted",
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

    # the per-method blocks the fold slices read — the header the
    # slice anchors, the next METHOD header the block closes
    def method_block(sig):
        start = text.find(f"METHOD {sig}")
        if start < 0:
            return ""
        nxt = text.find("\n  METHOD ", start + 1)
        return text[start : nxt if nxt >= 0 else len(text)]

    kp = method_block("keyPressed (Lnet/minecraft/client/input/KeyEvent;)Z")
    it = method_block("insertText (Ljava/lang/String;)V")
    gh = method_block("getHighlighted ()Ljava/lang/String;")
    gate = method_block("canConsumeInput ()Z")

    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each
    # the dump's own bytes must speak; the 26.2 walk the KeyEvent
    # single-arg shape wears, the isCopy/isPaste/isCut arms the
    # keyboardHandler set/getClipboard walks serve, the getHighlighted
    # the substring the [min, max) pair answers, the insertText the
    # budget fold the replace(lo, hi, s) walk wears — the whole-value
    # copy fold the standing family law the bytes NEVER speak, the
    # empty-store paste guard the bytes NEVER speak)
    verdicts = [
        ("the keyPressed walk the census wears (the KeyEvent shape)",
         "METHOD keyPressed (Lnet/minecraft/client/input/KeyEvent;)Z" in text),
        ("the keyPressed the isActive/isFocused gate opens with",
         "isActive:()Z" in kp and "isFocused:()Z" in kp),
        ("the isCopy arm the store the unguarded write serves",
         "isCopy:()Z" in kp
         and "getHighlighted:()Ljava/lang/String;" in kp
         and "setClipboard:(Ljava/lang/String;)V" in kp),
        ("the isPaste arm the isEditable gate the insertText serves",
         "isPaste:()Z" in kp
         and "isEditable:()Z" in kp
         and "getClipboard:()Ljava/lang/String;" in kp
         and "insertText:(Ljava/lang/String;)V" in kp),
        ("the isCut arm the store write + the empty insert serves",
         "isCut:()Z" in kp
         and "setClipboard:(Ljava/lang/String;)V" in kp
         and "insertText:(Ljava/lang/String;)V" in kp),
        ("the isSelectAll arm the end park + the zero highlight wears",
         "isSelectAll:()Z" in kp
         and "moveCursorToEnd:(Z)V" in kp
         and "setHighlightPos:(I)V" in kp),
        ("the getHighlighted the substring law the pair wears",
         "Math.min:(II)I" in gh
         and "Math.max:(II)I" in gh
         and "substring:(II)Ljava/lang/String;" in gh),
        ("the insertText the budget fold the walk opens with",
         "maxLength" in it and "ifgt" in it and "return" in it),
        ("the insertText the replace(lo, hi, s) walk the value wears",
         "StringBuilder.replace:(IILjava/lang/String;)" in it),
        ("the insertText the setHighlightPos collapse the tail wears",
         "setHighlightPos:(I)V" in it),
        ("the insertText the onValueChange responder the tail wears",
         "onValueChange:(Ljava/lang/String;)V" in it),
        ("the gate the focused box the walk keeps (canConsumeInput)",
         bool(gate)),
    ]
    bad = 0
    for label, ok in verdicts:
        print(f"[{'OK' if ok else 'MISS'}] {label}")
        if not ok:
            bad += 1
    print(f"VERDICT: {len(verdicts) - bad}/{len(verdicts)} ok")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
