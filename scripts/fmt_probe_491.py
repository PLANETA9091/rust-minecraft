#!/usr/bin/env python3
"""autopush-491 — THE FMT REGION WALK (the 485..489 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions (screens.rs, file order):
  the fields doc + the ctor rows (the cursor's own seats),
  the open walk + the close ride-home (the inventory's own lanes),
  the creative wrap's shift lane + the palette arms + the wrapped
  bottom row + the picker's close ride-home,
  the open-table reset, the table's close ride-home, the table's
  shift lane + store walk, the survival's shift lane + store walk,
  the throw's own ride, the quickcraft releases' banks + tails,
  the two free helpers (the ride fns the walks share).

part_11.rs — the round's own tests (the three new swap tests the
tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the fields doc (the cursor's own seats the round banks)
    (
        "    /// THE OFFLINE CURSOR'S OWN WEAR SEAT (the 491 round — the carried",
        "    pub inv_carried_enchants: Vec<(i32, i32)>,",
    ),
    # the ctor rows
    (
        "            inv_carried_damage_frac: 0.0,",
        "            inv_panel: (0, 0),",
    ),
    # the open walk's fresh-open pair
    (
        "        // the fresh-open pair (the carried's own word the same silence",
        "        self.open(ScreenKind::Inventory);",
    ),
    # the close ride-home (the banks + the floor ride + the resets)
    (
        "            // THE DATA'S OWN RIDE HOME (the 491 round — the cursor's own",
        "            self.hotbar_slots = self.inventory.hotbar();",
    ),
    # the creative wrap's shift lane data ride
    (
        "                    // THE QUICK-MOVE'S OWN DATA RIDE (the 491 round — the",
        "                } else {",
    ),
    # the palette arms (the emptied/destroyed/taken carried the resets)
    (
        "                    // the emptied carried the data seats (the 491 round",
        "    /// THE BOTTOM ROW'S STORE WALK (the generic menu.clicked walk the",
    ),
    # the wrapped bottom row's data ride
    (
        "        // THE SWAP WALK'S OWN DATA RIDE (the 491 round — the wrapped",
        "        // THE LIVE WORD (the listener law the swap lane speaks — the",
    ),
    # the picker's close ride-home
    (
        "            // THE DATA'S OWN RIDE HOME (the 491 round — the cursor's own",
        "            self.creative_swap_live_words(&words);",
    ),
    # the open-table reset
    (
        "        // the fresh open's own data seats (the 491 round — no carried",
        "        self.open(ScreenKind::Crafting);",
    ),
    # the table's close ride-home
    (
        "            // THE DATA'S OWN RIDE HOME (the 491 round — the cursor's own",
        "        // the grid's own clearContainer walk (each cell rides home):",
    ),
    # the table's shift lane data ride
    (
        "            // THE QUICK-MOVE'S OWN DATA RIDE (the 491 round — the table's",
        "            return;",
    ),
    # the table's store walk data ride
    (
        "        // the table's own store lanes the same seats the survival walk",
        "        self.inventory.items[store] = new_slot;",
    ),
    # the survival's shift lane data ride
    (
        "            // THE QUICK-MOVE'S OWN DATA RIDE (the 491 round — the moved",
        "            return;",
    ),
    # the survival's store walk data ride
    (
        "        // the 487 wear row + the 489 ench pairs the cursor's own seats",
        "        self.inventory.items[store] = new_slot;",
    ),
    # the throw's own data ride
    (
        "            // THE THROW'S OWN DATA RIDE (the 491 round — the whole",
        "    /// THE FURNACE CARRIED'S OWN THROW (the 26.2 mouseClicked's own",
    ),
    # the quickcraft release A's bank
    (
        "                        // THE DRAG'S OWN DATA BANK (the 491 round — the",
        "                    _ => {}",
    ),
    # the quickcraft release A's emptied tail
    (
        "            // the drag's own data seats (the 491 round — the emptied",
        "        self.hotbar_slots = self.inventory.hotbar();",
    ),
    # the quickcraft release B's bank
    (
        "                // THE DRAG'S OWN DATA BANK (the 491 round — the placed",
        "            }",
    ),
    # the quickcraft release B's emptied tail
    (
        "            // the drag's own data seats (the 491 round — the emptied",
        "        self.hotbar_slots = self.inventory.hotbar();",
    ),
    # the two free helpers (the ride fns the walks share)
    (
        "/// THE SWAP WALK'S OWN DATA RIDE (the 491 round — the two parallel",
        "/// THE OFFHAND'S OWN KIND SEAT (the autopush-346 round — the bank's",
    ),
]

PART11 = [
    # the round's own tests (the three new swap tests the tail wears)
    (
        "/// THE SWAP WALK'S OWN DATA RIDE (the 491 round — the two parallel",
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
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            ok = False
            continue
        formatted = proc.stdout
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
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56].replace("\n", " | ")
            if bad:
                ok = False
                print(f"{path}: REGION NOT FMT-CLEAN [{label}] hunks={bad}")
            else:
                print(f"{path}: region fmt-clean [{label}]")
    if ok:
        print("ALL-REGIONS-CLEAN")
    else:
        print("REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
