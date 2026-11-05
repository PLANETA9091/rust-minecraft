#!/usr/bin/env python3
"""autopush-470 — THE FMT REGION WALK (the 469 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the quick-move docstring's own re-answer (the
                dead-code silence the census truth wears, the stale
                no-audio-infra clause the bytes replace).
  part_11.rs  — the round's own test (the trade-sound silence the
                EOF side keeps — the file's own tail the new pin
                wears).
Usage: python3 scripts/fmt_probe_472.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the quick-move docstring (the dead-code clause the census wears
    # — the onTake tail the next keeps)
    (
        "THE RESULT SOURCE'S OWN QUICK-MOVE (javap 26.2 MerchantMenu.",
        "    fn merchant_result_quick_move(&mut self, button: i8) {",
    ),
]

PART11 = [
    # the round's own pin (the EOF side keeps — the file's own tail
    # the new test wears)
    (
        "fn the_merchant_trade_sound_the_dead_code_silence_wears() {",
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
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            inside = [
                (i1, i2, tag, j1, j2)
                for (i1, i2, tag, j1, j2) in hunks
                if i1 < e_line and i2 > s_line
            ]
            if inside:
                ok = False
                print(f"[MY-REGIONS DIRTY] {path} :: {start_marker[:60]!r}")
                for h in inside:
                    print(f"   hunk {h}")
            else:
                print(f"[MY-REGIONS CLEAN] {path} :: {start_marker[:60]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
