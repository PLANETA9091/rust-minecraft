#!/usr/bin/env python3
"""fmt_fix_464 — the surgical region formatter (the probe's own law the
fixer serves): the whole-file rustfmt output the REGION-restricted
hunks the original text applies — the round's own regions the fix
wears, the OTHER regions the drift never touches.

Regions:
  screens.rs  — the widget press's own visible gate (the raw row the
                selection clamp reads, the doc the law wears).
  part_11.rs  — the round's own test (the press clamp walk wears, EOF
                the end the round keeps).
Usage: python3 scripts/fmt_fix_464.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the press's own visible gate (the raw row the clamp reads)
    (
        "THE MERCHANT'S OWN WIDGET PRESS (javap of the TradeOfferButton",
        "THE MERCHANT'S OWN ROUND (javap 26.2 MerchantScreen",
    ),
]

PART11 = [
    # the round's own test (the press clamp walk wears — EOF the end keeps)
    (
        "THE MERCHANT'S OWN SELECTION CLAMP (autopush-464",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def region_end(text: str, marker: str, start_at: int) -> int:
    if marker is None:
        return len(text)
    return find(text, marker, start_at)


def formatted_of(path: str) -> str:
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        orig_lines = original.split("\n")
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(None, orig_lines, formatted.split("\n"))
        spans = []
        cursor = 0
        for (start_marker, end_marker) in regions:
            s_ = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s_)
            cursor = e
            s_line = original[:s_].count("\n")
            e_line = original[:e].count("\n")
            spans.append((s_line, e_line))
        # walk the opcodes BACKWARD so the line indices stay valid; apply
        # only the hunks that INTERSECT a region
        ops = [op for op in sm.get_opcodes() if op[0] != "equal"]
        replaced = 0
        for tag, i1, i2, j1, j2 in reversed(ops):
            if any(i1 < e and i2 > s_ for (s_, e) in spans):
                orig_lines[i1:i2] = formatted.split("\n")[j1:j2]
                replaced += 1
        open(path, "w", encoding="utf-8").write("\n".join(orig_lines))
        print(f"{path}: {replaced} region hunks applied")
    return 0


if __name__ == "__main__":
    sys.exit(main())
