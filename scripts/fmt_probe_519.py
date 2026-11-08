#!/usr/bin/env python3
"""autopush-519 — THE FMT REGION WALK (the 485..518 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the brewing wire walk's own regions (the wire twin's
  doc's own first line .. the enchant add's doc; the wire caller's
  doc's own self-reference line .. the offline swap table's doc).
  part_11.rs — the stand's re-worn doc the tail wears + the round's
  own tests (EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the brewing wire twin's own region (the brewing wire doc's own
    # first line .. the enchant add's doc)
    (
        "/// THE SWAP TABLE OVER THE BREWING'S OWN BELIEF (javap 26.2 — the",
        "/// THE ENCHANT BELIEF'S OWN INVENTORY ADD (the Inventory.add walk",
    ),
    # the brewing wire caller's own region (the doc's own
    # self-reference line .. the offline table's doc)
    (
        "on_brewing_swap_key_wire`], the body the",
        "/// THE BREWING'S OWN SWAP TABLE (the offline twin — the autopush-",
    ),
]

PART11 = [
    # the stand's re-worn doc (the brewing stand's doc .. the hopper
    # title test's doc)
    (
        "/// THE BREWING STAND'S OWN SWAP KEY (the 214 dev round — the",
        "/// THE HOPPER'S OWN WIRE TITLE (the 215 dev round",
    ),
    # the round's own tests (the tail wears)
    (
        "/// THE BREWING WIRE SWAP'S OWN KIND VERDICTS (the autopush-519 round",
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
