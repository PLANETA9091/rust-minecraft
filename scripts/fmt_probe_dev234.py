#!/usr/bin/env python3
"""dev-234 — THE FMT REGION WALK (the 485..503 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the table take's own emptied-cell arm (the kind row's
  own retire), the craft take's own emptied-cell arm (the twin), the
  table take's own doc tail, the craft take's own doc tail.
  part_11.rs — the round's own test (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the table take's own emptied-cell arm (the kind row's own retire)
    (
        "                        // THE EMPTIED CELL'S OWN RESET (the autopush-492",
        "                    apply_remainder_cell(cell, *rem)",
    ),
    # the craft take's own emptied-cell arm (the twin)
    (
        "                        // THE EMPTIED CELL'S OWN RESET (the dev-229",
        "                    apply_remainder_cell(cell, *rem)",
    ),
    # the table take's own doc tail
    (
        "    /// back. The HUD row the store's every mutation syncs. THE",
        "    fn table_on_take(&mut self) {",
    ),
    # the craft take's own doc tail
    (
        "    /// reads). THE EMPTIED CELL'S OWN RESET the third seat word",
        "    fn craft_on_take(&mut self) {",
    ),
]

PART11 = [
    # the round's own test (the tail wears)
    (
        "/// THE TAKE WALK'S OWN KIND RETIRE (the dev-234 ride — the onTake's",
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


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr[:400]}")
            ok = False
            continue
        formatted = proc.stdout
        for (start_m, end_m) in regions:
            s = find(original, start_m)
            e = region_end(original, end_m, s + 1)
            orig_region = original[s:e]
            try:
                fs = find(formatted, start_m)
                fe = region_end(formatted, end_m, fs + 1)
            except SystemExit:
                print(f"{path}: REGION LOST in formatted output: {start_m[:60]!r}")
                ok = False
                continue
            fmt_region = formatted[fs:fe]
            if orig_region == fmt_region:
                print(f"{path}: region CLEAN [{start_m[:52]!r}...]")
            else:
                ok = False
                print(f"{path}: region DIRTY [{start_m[:52]!r}...] — the diff:")
                for line in list(
                    difflib.unified_diff(
                        orig_region.splitlines(),
                        fmt_region.splitlines(),
                        lineterm="",
                    )
                )[:30]:
                    print(line)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
