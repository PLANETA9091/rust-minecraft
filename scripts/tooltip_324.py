#!/usr/bin/env python3
"""tooltip_324.py — the 324 round's own jar probe (the 26.2 client.jar).

THE QUESTION the round answers: the beacon screen's hover tooltip the
WHICH law wears — the BeaconScreen's own override or the
AbstractContainerScreen.renderTooltip the inheritance keeps?

THE WALK:
  1. javap -p net.minecraft.client.gui.screens.inventory.BeaconScreen
     — the method table the override absence pins (no renderTooltip,
     no getTooltipFromItem — the inherited lane speaks);
  2. javap -p net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
     — renderTooltip(GuiGraphics,int,int) + isHovering(Slot,double,double)
     + getTooltipFromItem the shared law's own seats pin;
  3. javap -c the renderTooltip body — the carried-empty +
     hoveredSlot.hasItem() gates the walk disassembles (the exact
     verdict the offline lane re-keys);
  4. javap -c the isHovering body — the 16x16 inner cell the
     `mouseX >= slot.x && mouseX < slot.x + 16` shape keeps.

rc=0 the probe banks; the stdout the evidence the scripts/jar_work/
tooltip_324.txt dump serves.
"""
import subprocess
import sys
import os

JDK = "/home/z/my-project/jdk-25.0.4.1+1/bin/javap"
CP = "target/classes"

def sections(jar_cp: str):
    return [
        ("BeaconScreen method table", [
            "-p", "-cp", jar_cp,
            "net.minecraft.client.gui.screens.inventory.BeaconScreen",
        ]),
        ("AbstractContainerScreen method table (tooltip seats)", [
            "-p", "-cp", jar_cp,
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen",
        ]),
        ("AbstractContainerScreen.renderTooltip body", [
            "-c", "-p", "-cp", jar_cp,
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen",
            "renderTooltip",
        ]),
        ("AbstractContainerScreen.isHovering body", [
            "-c", "-p", "-cp", jar_cp,
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen",
            "isHovering",
        ]),
        ("AbstractContainerScreen.getTooltipFromItem body", [
            "-c", "-p", "-cp", jar_cp,
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen",
            "getTooltipFromItem",
        ]),
    ]


def main() -> int:
    jar = ".mc2rs/classpath/client.jar"
    if not os.path.exists(jar):
        print(f"MISSING JAR: {jar}", file=sys.stderr)
        return 2
    out = []
    rc_all = 0
    for title, cmd in sections(jar):
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
    dest = "scripts/jar_work/tooltip_324.txt"
    os.makedirs("scripts/jar_work", exist_ok=True)
    with open(dest, "w") as fh:
        fh.write(text)
    # THE VERDICTS the round's own law keeps
    beacon_has_override = "renderTooltip" in (
        text.split("## AbstractContainerScreen method table")[0]
    )
    print(f"beacon renderTooltip override present: {beacon_has_override}")
    print(f"abstract renderTooltip present: {'renderTooltip' in text}")
    print(f"dump written: {dest} ({len(text)} bytes)")
    print(f"rc_all={rc_all}")
    return 0 if rc_all == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
