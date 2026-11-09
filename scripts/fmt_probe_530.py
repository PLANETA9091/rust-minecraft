#!/usr/bin/env python3
"""autopush-530 — THE FMT REGION WALK (the 485..529 + dev-236/237
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict
the marker ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the on_chest_drop_key walk's own doc..wire-doc region
  (the round's kind retires the body wears — the chest Q's own doc
  the start marker keeps, the wire twin's doc the end answers).
  part_11.rs — the round's own survey test (the chest Q kind
  retire's own record the take-key census's chest lane pins — the
  test's own doc the start marker keeps, the beacon twin's doc the
  end answers).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the on_chest_drop_key walk's own region (the doc the start
    # marker keeps, the wire twin's doc the end answers)
    (
        "    /// THE CHEST'S OWN Q (the SAME keyPressed keyDrop arm — the",
        "    /// THE WIRE GATE'S OWN WALK (the offline walk's twin — the same",
    ),
]

PART11 = [
    # the round's own survey test (the chest Q kind retire's own
    # record .. the beacon twin's doc)
    (
        "/// THE CHEST DROP KEY'S OWN KIND RETIRE (the take-key census's own",
        "/// THE BEACON WIRE SWAP'S OWN KIND VERDICTS (the autopush-527 round",
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
            raise SystemExit(f"RUSTFMT FAILED for {path}\n{proc.stderr[:400]}")
        formatted = proc.stdout
        for start_m, end_m in regions:
            s = find(original, start_m)
            e = region_end(original, end_m, s + len(start_m))
            mine = original[s:e]
            fs = find(formatted, start_m)
            fe = region_end(formatted, end_m, fs + len(start_m))
            theirs = formatted[fs:fe]
            if mine == theirs:
                print(f"CLEAN  {path} :: {start_m[:60]!r}")
            else:
                ok = False
                print(f"DIRTY  {path} :: {start_m[:60]!r}")
                for line in difflib.unified_diff(
                    mine.splitlines(),
                    theirs.splitlines(),
                    "mine",
                    "rustfmt",
                    lineterm="",
                    n=1,
                ):
                    print(line)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
