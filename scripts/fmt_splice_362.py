#!/usr/bin/env python3
"""autopush-362 — THE FMT SPLICE WALK (the 358/360/361 record's own law
the round wears): the dirty regions the probe named the formatted
stdout the splice answers — the original[:s] + formatted[fs:fe] +
original[e:] the marker pairs speak, the bottom-up order the offsets
the shifts never wear. The re-probe the verdict re-speaks.
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # (start_marker, end_marker) — the probe's dirty regions only
    ("        // THE KIND'S OWN CARRIED TINT (the 358 law the furnace's own",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the dev-203 round"),
    ("        // render — the offline sim's local cursor the chest's own gate\n        // keeps, the headless scenes track no mouse; THE KIND'S OWN\n        // CARRIED TINT the 358 law the dispenser's own ghost re-wears",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the dev-203 round"),
    ("                                                 // the window's death the word retires (the honest",
     "                self.bundle_contents.clear();"),
]
PART11 = [
    ("fn furnace_seat(app: &mut App, menu: i16) -> (i32, i32) {", None),
]


def splice(path: str, regions) -> None:
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    assert proc.returncode == 0, proc.stderr
    formatted = proc.stdout
    jobs = []
    for start_marker, end_marker in regions:
        s = original.find(start_marker)
        assert s >= 0, f"orig start missing: {start_marker[:40]!r}"
        e = len(original) if end_marker is None else original.find(end_marker, s)
        assert e > s, f"orig end missing: {end_marker!r}"
        fs = formatted.find(start_marker)
        assert fs >= 0, f"fmt start missing: {start_marker[:40]!r}"
        fe = len(formatted) if end_marker is None else formatted.find(end_marker, fs)
        assert fe > fs, f"fmt end missing: {end_marker!r}"
        jobs.append((s, e, formatted[fs:fe]))
    # bottom-up: the later offsets the earlier splices never shift
    jobs.sort(key=lambda j: j[0], reverse=True)
    for s, e, replacement in jobs:
        original = original[:s] + replacement + original[e:]
    open(path, "w", encoding="utf-8").write(original)
    print(f"{path}: {len(jobs)} region(s) spliced")


def main() -> None:
    splice("crates/mc-client/src/screens.rs", SCREENS)
    splice("crates/mc-client/src/screens/tests/part_11.rs", PART11)
    print("SPLICE-DONE")


if __name__ == "__main__":
    main()
