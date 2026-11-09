#!/usr/bin/env python3
"""autopush-531 — THE FMT REGION WALK (the 485..530 + dev-236/237
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict
the marker ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the on_brewing_drop_key walks' own doc..swap-table-doc
  region (the round's offline + wire twins the body wears — the Q's
  own doc the start marker keeps, the swap table's doc the end
  answers).
  window.rs — the drop-key chain's own brewing arm (the round's
  dispatch the chain wears — the arm's own comment the start marker
  keeps, the call the end answers).
  part_11.rs — the round's own two survey tests (the offline + wire
  kind retire's own records the Q family's stand lane pins — the
  tests' own doc the start marker keeps, the beacon twin's doc the
  end answers).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the on_brewing_drop_key walks' own region (the Q doc the start
    # marker keeps, the swap table's doc the end answers)
    (
        "    /// THE BREWING STAND'S OWN Q (the SAME keyPressed keyDrop arm —",
        "    /// THE BREWING'S OWN SWAP TABLE (the offline twin — the autopush-",
    ),
]

WINDOW = [
    # the drop-key chain's own brewing arm (the dispatch the chain
    # wears — the call the end answers)
    (
        "                // THE BREWING STAND'S OWN Q (the SAME arm — the",
        "app.on_brewing_drop_key(q_ctrl);",
    ),
]

PART11 = [
    # the round's own two survey tests (the offline + wire kind
    # retire's own records .. the beacon twin's doc)
    (
        "/// THE BREWING DROP KEY'S OWN KIND RETIRE (the autopush-531 round —",
        "/// THE BEACON WIRE SWAP'S OWN KIND VERDICTS (the autopush-527 round",
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/window.rs": WINDOW,
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
