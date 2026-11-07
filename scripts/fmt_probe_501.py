#!/usr/bin/env python3
"""autopush-501 — THE FMT REGION WALK (the 485..dev233 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the barrel window's own kind flag doc + field, the
  chest-grid seat's own kind read doc + fn (the dev-233 hopper read
  the barrel door the round widens), the chest seat's own clone doc +
  the whole fn (the barrel lane the round wakes), the shulker door's
  own kind-row reset block (the stale flag the family reset retires).
  part_11.rs — the round's own test (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the barrel window's own kind flag doc + field
    (
        "    /// THE BARREL WINDOW'S OWN KIND FLAG (the offline clone lane's key",
        "    /// The last biome the network world's music situation resolved to",
    ),
    # the chest-grid seat's own kind read doc + fn
    (
        "    /// THE CHEST-GRID SEAT'S OWN KIND READ (the 500 brewing read's own",
        "    /// THE HELD WORD (the wire order's own resolution — the item id",
    ),
    # the chest seat's own clone doc + the whole fn
    (
        "    /// THE CHEST SEAT'S OWN CLONE (the chest click walk's own live-only",
        "    /// THE FURNACE SEAT'S OWN CLONE (the furnace click walk's own",
    ),
    # the shulker door's own kind-row reset block
    (
        "        // THE SHULKER'S OWN KIND ROW (the title lane's flag the draw",
        "        // THE SHULKER'S OWN OPEN BYTE (the dev-193 round — the vanilla",
    ),
]

PART11 = [
    # the round's own test (the tail wears)
    (
        "/// THE CLONE'S OWN BARREL RIDE (the dev-233 NEXT's own first lane",
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
                        n=1,
                    )
                )[:40]:
                    print("   " + line)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
