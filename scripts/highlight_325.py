#!/usr/bin/env python3
"""highlight_325.py — the 325 round's own jar probe (the 26.2 client.jar).

THE QUESTION the round answers: the hovered slot's own highlight the
WHICH law wears — the sprites, the colors, the gates, the geometry?

THE WALK:
  1. AbstractContainerScreen method table — the
     renderSlotHighlightBack/Front + setTooltipForNextFrame the
     sprite seats pin;
  2. javap -c the renderSlotHighlightBack body — the sprite the
     blit serves (the x-1/y-1/18x18 geometry + the color the
     ldc wears);
  3. javap -c the renderSlotHighlightFront body — the same law the
     front seat answers;
  4. javap -c the render body — the hovered-slot walk the gates
     speak (the isHovering + isActive + the carried-empty verdict
     the highlight keys, the back BEFORE the item + the front
     AFTER the order the loop wears);
  5. the constant pool the sprite Identifier rows the -p table
     banks (the widget/slot highlight paths the loader reads).

rc=0 the probe banks; the stdout the evidence the scripts/jar_work/
highlight_325.txt dump serves.
"""
import subprocess
import sys
import os

JDK = "/home/z/my-project/jdk-25.0.4.1+1/bin/javap"
REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAR = os.path.join(REPO, ".mc2rs/classpath/client.jar")
DEST = os.path.join(REPO, "scripts/jar_work/highlight_325.txt")
CLS = "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen"


def sections():
    return [
        ("AbstractContainerScreen method table (highlight seats)", [
            "-p", "-cp", JAR, CLS,
        ]),
        ("AbstractContainerScreen.renderSlotHighlightBack body", [
            "-c", "-p", "-cp", JAR, CLS,
        ]),
        ("AbstractContainerScreen.renderSlotHighlightFront body", [
            "-c", "-p", "-cp", JAR, CLS,
        ]),
        ("AbstractContainerScreen.render body (the hovered walk)", [
            "-c", "-p", "-cp", JAR, CLS,
        ]),
    ]


def main() -> int:
    if not os.path.exists(JAR):
        print(f"MISSING JAR: {JAR}", file=sys.stderr)
        return 2
    out = []
    rc_all = 0
    for title, cmd in sections():
        out.append("=" * 72)
        out.append(f"## {title}")
        out.append("$ javap " + " ".join(cmd))
        try:
            proc = subprocess.run(
                [JDK] + cmd, capture_output=True, text=True, timeout=120
            )
            rc = proc.returncode
            out.append(proc.stdout)
            if proc.stderr.strip():
                out.append("[stderr] " + proc.stderr.strip()[:2000])
            if rc != 0:
                rc_all = rc
        except Exception as exc:  # noqa: BLE001
            out.append(f"[error] {exc}")
            rc_all = 3
    text = "\n".join(out)
    dest = DEST
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    with open(dest, "w") as fh:
        fh.write(text)
    print(f"renderSlotHighlightBack present: {'renderSlotHighlightBack' in text}")
    print(f"renderSlotHighlightFront present: {'renderSlotHighlightFront' in text}")
    print(f"SLOT_HIGHLIGHT sprites: {text.count('SLOT_HIGHLIGHT')}")
    print(f"dump written: {dest} ({len(text)} bytes)")
    print(f"rc_all={rc_all}")
    return 0 if rc_all == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
