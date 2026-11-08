#!/usr/bin/env python3
"""autopush-526 — THE FMT REGION WALK (the 485..525 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the enchant wire walk's own regions (the wire free
  fn's doc's own first line .. the table result take's doc; the ENCHANT
  wire caller's own self-reference doc line .. the offline table's
  doc — the shared WIRE GATE first line the 14 twins wear, the
  [`wire_enchant_swap`] self-reference the discriminator keeps).
  part_11.rs — the round's own tests (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the inventory wire free fn's own region (the wire doc's own
    # first line .. the Q drop twin's doc)
    (
        "/// THE WINDOW-0 SWAP WIRE ROUND (the doClick SWAP table the offline",
        "/// THE Q KEY'S OWN WIRE WALK (javap 26.2",
    ),
    # the inventory wire caller's own region (the self-reference doc
    # line .. the hovered seat's doc)
    (
        "the table the [`wire_inventory_swap`] free fn rides",
        "/// THE HOVERED SEAT'S OWN WALK (the click lane's own order",
    ),
]

PART11 = [
    # the round's own tests (the tail wears)
    (
        "/// THE INVENTORY WIRE SWAP'S OWN KIND VERDICTS (the autopush-526 round",
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
