#!/usr/bin/env python3
"""autopush-504 — THE FMT REGION WALK (the 485..dev-234 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the table result's own doc tail, the table result's
  own pickup + merge arms, the craft result's own doc tail, the craft
  result's own pickup + merge arms (the honest cursor's kindless
  answer the ride banks).
  part_11.rs — the round's own test (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the table result's own doc tail
    (
        "    /// same reverse-order walk the inventory's batch speaks, folded to",
        "    fn click_table_result(&mut self, button: i8) {",
    ),
    # the table result's own pickup + merge arms
    (
        "        let (id, count) = self.table_result;",
        "    /// THE TABLE'S QUICK-MOVE BATCH",
    ),
    # the craft result's own doc tail
    (
        "    /// The result slot's own emptiness the carried-empty arm gates",
        "    fn click_craft_result(&mut self, button: i8) {",
    ),
    # the craft result's own pickup + merge arms
    (
        "        let (id, count) = self.craft_result;",
        "    /// THE QUICK_MOVE BATCH",
    ),
]

PART11 = [
    # the round's own test (the tail wears)
    (
        "/// THE RESULT SEAT'S OWN HONEST CURSOR (the autopush-504 ride — the",
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
