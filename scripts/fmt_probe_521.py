#!/usr/bin/env python3
"""autopush-521 — THE FMT REGION WALK (the 485..520 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the chest wire walk's own regions (the wire free fn's
  doc's own first line .. the on-take walk's doc; the wire caller's
  own fn-name doc line .. the offline table's doc).
  part_11.rs — the round's own tests (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the chest wire free fn's own region (the wire doc's own first
    # line .. the on-take walk's doc)
    (
        "/// THE CHEST SWAP WIRE ROUND (the doClick SWAP table",
        "/// THE ON-TAKE WALK OVER THE BELIEF",
    ),
    # the chest wire caller's own region (the fn-name doc line ..
    # the offline table's doc)
    (
        "/// [`wire_chest_swap`] free fn answers): the window identity the",
        "/// THE CHEST'S OWN SWAP TABLE (the offline twin",
    ),
]

PART11 = [
    # the round's own tests (the tail wears)
    (
        "/// THE CHEST WIRE SWAP'S OWN KIND VERDICTS (the autopush-521 round",
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
            raise SystemExit(f"RUSTFMT FAILED for {path}\n{proc.stderr[:400]}")
        formatted = proc.stdout
        for start_m, end_m in regions:
            s = find(original, start_m)
            e = region_end(original, end_m, s + 1)
            fs = find(formatted, start_m)
            fe = region_end(formatted, end_m, fs + 1)
            my_orig = original[s:e]
            my_fmt = formatted[fs:fe]
            if my_orig == my_fmt:
                print(f"{path}: region CLEAN [{start_m[:60]!r}...]")
                continue
            ok = False
            print(f"{path}: region DIRTY [{start_m[:60]!r}...]")
            for line in difflib.unified_diff(
                my_orig.splitlines(),
                my_fmt.splitlines(),
                lineterm="",
                n=1,
            ):
                print(line)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
