#!/usr/bin/env python3
"""inventory_highlight_326.py — the 326 round's own jar probe.

THE QUESTION: the survival inventory's slot highlight the WHICH law
wears — the InventoryScreen's own override or the
AbstractContainerScreen.extractSlotHighlightBack/Front the
inheritance keeps? (The 325 probe pinned the shared walk; this probe
pins the SUBCLASS absence.)

rc=0 the probe banks; the dump scripts/jar_work/inventory_highlight_326.txt
serves.
"""
import os
import subprocess
import sys

JDK = "/home/z/my-project/jdk-25.0.4.1+1/bin/javap"
REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAR = os.path.join(REPO, ".mc2rs/classpath/client.jar")
DEST = os.path.join(REPO, "scripts/jar_work/inventory_highlight_326.txt")

CLASSES = [
    "net.minecraft.client.gui.screens.inventory.InventoryScreen",
    "net.minecraft.client.gui.screens.inventory.AbstractInventoryScreen",
]


def main() -> int:
    if not os.path.exists(JAR):
        print(f"MISSING JAR: {JAR}", file=sys.stderr)
        return 2
    out, rc_all = [], 0
    for cls in CLASSES:
        out.append("=" * 72)
        out.append(f"## {cls} method table")
        cmd = [JDK, "-p", "-cp", JAR, cls]
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        out.append(proc.stdout)
        if proc.stderr.strip():
            out.append("[stderr] " + proc.stderr.strip()[:1000])
        if proc.returncode != 0:
            rc_all = proc.returncode
    text = "\n".join(out)
    os.makedirs(os.path.dirname(DEST), exist_ok=True)
    with open(DEST, "w") as fh:
        fh.write(text)
    print(f"InventoryScreen present: {'InventoryScreen' in text}")
    print(f"Highlight override in subclass tables: {'Highlight' in text.split('##')[2] if text.count('##') >= 2 else 'n/a'}")
    print(f"dump written: {DEST} ({len(text)} bytes)")
    print(f"rc_all={rc_all}")
    return 0 if rc_all == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
