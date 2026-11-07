#!/usr/bin/env python3
"""autopush-494 — THE FMT REGION WALK (the 485..493 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the ench store's own 37TH seat field, the swap's own
  ride, the drag's own pair bank, the tooltip's own wall re-key (the
  survival + the creative wrap arms), the offhand click's own shift
  ride + word ride, the data-ride helper's own re-key + the seat
  const's own doc.
  part_11.rs — the round's own test (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the ench store's own field (the doc + the shape)
    (
        "    /// THE INVENTORY'S OWN ENCHANTMENT STORE (the ride-home lane —",
        "    /// THE PLAYER ROWS' OWN POTION-KIND BANK (the dev-199 round — the",
    ),
    # the swap's own ride (the offline lane's tail)
    (
        "            // THE KIND'S OWN RIDE (the dev-201 round — the stack the",
        "            let id = self.held_id();",
    ),
    # the drag's own pair bank (the Offhand arm)
    (
        "                    InventoryHover::Offhand => {",
        "                    _ => {}",
    ),
    # the tooltip's own wall re-key (the survival arm's ench match)
    (
        "        // THE ENCHANTMENT STORE'S OWN RIDE (the dev-198 round — the",
        "        if count <= 0 {",
    ),
    # the creative wrap's own arm (the pairs' own seat)
    (
        "                    // THE OFFHAND'S OWN SEAT (the autopush-348 round — the",
        "                    // THE STORE SEATS (the main/hotbar rows the REAL",
    ),
    # the offhand click's own walk (the shift ride + the word ride)
    (
        "    fn click_offhand_slot(&mut self, button: i8) {",
        "    /// The RESULT slot's click (the InventoryMenu's slot 0 — the",
    ),
    # the helper's own re-key + the const's own doc (the second line
    # the marker wears — the 491 test doc the same head the tail file
    # carries, the unique row the collision retires)
    (
        "/// own wear + ench pairs the lanes ride: the drained source the",
        "/// THE KIND BANK'S OWN SEAT MAP (the 343 remap the ride re-keys): the",
    ),
]

PART11 = [
    # the round's own test (the tail wears)
    (
        "/// THE OFFHAND'S OWN ENCHANT SEAT (the 494 round — the 37-row shape",
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
            # the same region the formatted text wears (the marker the
            # formatted text carries — the find the first occurrence
            # the original keeps)
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
