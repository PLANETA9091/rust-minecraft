#!/usr/bin/env python3
"""autopush-362 — THE FMT REGION WALK (the 323/204/358/360/361 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits), the
pre-existing static noise the HEAD law confirms the non-mine hunks
the honest inventory banks.

Regions:
  screens.rs  — the furnace walk's two kind rides (the standard rows'
                four-arm block + the kindless seats' wall), the
                dispenser walk's four-arm ride, the two offline ghost
                tints, the two ride homes, the two throws' words, the
                two fresh-open doors, the two wire doors, the live
                close + window death, the two field docs.
  part_11.rs  — the round's own test tail (the two seat helpers + the
                furnace/dispenser carried word walks).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the furnace walk's standard rows' own cursor ride
    ("                // THE KIND'S OWN CURSOR RIDE (the dev-204 law the\n                // furnace's own walk re-wears",
     "                self.inventory.items[idx] = new_slot;"),
    # the furnace's kindless seats' own wall
    ("            // THE KINDLESS SEAT'S OWN WALL (the chest rows' own law the",
     "            if (slot as usize) < self.furnace_slots.len() {"),
    # the dispenser walk's own cursor ride
    ("        // THE KIND'S OWN CURSOR RIDE (the dev-204 law the dispenser's",
     "        if menu < 9 {\n            self.dispenser_slots[menu] = new_slot;"),
    # the furnace's offline ghost tint
    ("        // THE KIND'S OWN CARRIED TINT (the 358 law the furnace's own",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the dev-203 round"),
    # the dispenser's offline ghost tint
    ("        // render — the offline sim's local cursor the chest's own gate\n        // keeps, the headless scenes track no mouse; THE KIND'S OWN\n        // CARRIED TINT the 358 law the dispenser's own ghost re-wears",
     "        // THE ITEM TOOLTIP (the extractTooltip pass — the dev-203 round"),
    # the furnace close's ride home
    ("            // THE WORD'S OWN RIDE HOME (the chest's own close law the\n            // furnace's close re-wears",
     "        // THE STORE'S OWN FLUSH (the dev-69 round"),
    # the dispenser close's ride home
    ("            // THE WORD'S OWN RIDE HOME (the chest's own close law the\n            // dispenser's close re-wears",
     "        // THE STORE'S OWN FLUSH (the chest's own persist law"),
    # the furnace throw's own word
    ("        // the throw's own word (the emptied carried the NONE wears —\n        // the whole-stack throw the word drops, the half the split the\n        // same stack keeps)\n        if self.furnace_carried.1 <= 0 {",
     "/// THE CHEST CARRIED'S OWN THROW"),
    # the dispenser throw's own word
    ("        // the throw's own word (the emptied carried the NONE wears —\n        // the whole-stack throw the word drops, the half the split the\n        // same stack keeps)\n        if self.dispenser_carried.1 <= 0 {",
     "/// hit box the last render laid out (the vanilla Slot's own hit"),
    # the furnace's fresh-open door
    ("        // THE FRESH-OPEN'S OWN DOOR (the chest's own law — the stale\n        // word the open retires)\n        self.furnace_carried_potion_kind",
     "        self.open(ScreenKind::Furnace);"),
    # the dispenser's fresh-open door
    ("        // THE FRESH-OPEN'S OWN DOOR (the chest's own law — the stale\n        // word the open retires)\n        self.dispenser_carried_potion_kind",
     "        // THE STANDARD ROWS' OWN PROJECTION (the menu's"),
    # the furnace wire open's own word
    ("                    // the wire's own word (the live lane the honest NONE\n                    // wall keeps — the offline memory the open retires)",
     "                    let mut slots = vec![(0i32, 0i32); 39];"),
    # the dispenser wire open's own word
    ("                    // the wire's own word (the live lane the honest NONE\n                    // wall keeps — the correction the kind never carries)",
     "                    // the server's window the per-cell key never banks"),
    # the dispenser live close's own word
    ("            // the wire's own word (the live lane the honest NONE wall\n            // keeps — the close the word retires)",
     "            self.container_removed_rode = true;"),
    # the dispenser window death's own word
    ("                                                 // the window's death the word retires (the honest",
     "                self.bundle_contents.clear();"),
    # the furnace carried word's own field doc
    ("    /// THE OFFLINE CARRIED'S OWN KIND WORD (the chest's own field the\n    /// furnace's walk re-wears",
     "    /// THE OFFLINE SIM'S OWN MEMORY"),
    # the dispenser carried word's own field doc
    ("    /// THE OFFLINE CARRIED'S OWN KIND WORD (the chest's own field the\n    /// dispenser's walk re-wears",
     "    /// The dispenser's slot hit boxes the LAST render laid out"),
]
PART11 = [
    ("fn furnace_seat(app: &mut App, menu: i16) -> (i32, i32) {", None),
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
        sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker)
            e = region_end(original, end_marker, s)
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56]
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
