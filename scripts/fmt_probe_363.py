#!/usr/bin/env python3
"""autopush-363 — THE FMT REGION WALK (the 323/204/358/360/361/362
record's own law the round wears): the whole-file rustfmt the ORIGINAL
text the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs  — the row bank's own fields + constructor sentinels, the
                cell-load helper, the four open doors' composes, the
                live open's NONE wall, the shift quick-move's kind
                read + ride, the click lane's bank read/write, the
                flush's kinds split, the spill's death ride + demote,
                the render's row tint, the normalized read helper.
  part_11.rs  — the 361 test's own row-lane update + the round's own
                test tail.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the row bank's own fields
    ("    /// THE CONTAINER ROWS' OWN KIND BANK (the autopush-363 round — the",
     "    /// THE BARREL LID'S OWN CELL (the dev-65 round"),
    # the constructor's own sentinels
    ("            container_stores_potion_kinds: std::collections::HashMap::new(),",
     "            container_stores_file: None,"),
    # the cell-load helper
    ("    /// THE ROW BANK'S OWN CELL LOAD (the kinds map the store's own",
     "    pub fn open_chest_for(&mut self, pos: (i32, i32, i32)) {"),
    # the chest door's own compose
    ("        // THE ROW BANK'S OWN COMPOSE (the autopush-363 round — the",
     "        // the fresh-open pair (the stale word the chest door retires)"),
    # the shulker door's own compose
    ("        // THE ROW BANK'S OWN COMPOSE (the chest's own law the shulker's",
     "        self.chest_carried = (0, 0);"),
    # the hopper door's own compose
    ("        // THE ROW BANK'S OWN COMPOSE (the chest's own law the hopper's",
     "        self.chest_carried = (0, 0);"),
    # the deterministic scene's own fresh pair
    ("        // the row bank's fresh pair (the deterministic scene the NONE",
     "        self.open(ScreenKind::Chest);"),
    # the live open's own NONE wall
    ("                // the live lane's own rows the honest NONE wall keeps",
     "                self.chest_mirror = mirror;"),
    # the shift quick-move's own kind read + ride
    ("            let src_menu = slot as usize;",
     "            // THE TRANSFER'S OWN LIVE FILL RIDE (the autopush-295"),
    # the click lane's own bank read + write
    ("        // THE KIND'S OWN CURSOR RIDE (the dev-204 round's law the\n        // chest's own walk re-wears",
     "        if menu < n {\n            self.chest_slots[menu] = new_slot;"),
    # the flush's own kinds split
    ("                // THE KINDS' OWN SPLIT (the 363 round — the bank the",
     "            // THE DISK BANK'S OWN RIDE HOME (the dev-68 round"),
    # the spill's own kinds death ride
    ("        // THE KINDS' OWN DEATH RIDE (the 363 round — the bank the same",
     "        // THE FILL'S OWN DEATH RIDE (the comparator's container lane"),
    # the spill's own demote truncation
    ("            // the demoted grid's own kinds the same truncation keeps",
     "        // THE DISK BANK'S OWN DEATH RIDE (the dev-68 round"),
    # the render's own row tint
    ("            // THE ROW BANK'S OWN TINT (the 340 law the stored rows",
     "        // the live mirror's contents (the networked store's rows 27..63):"),
    # the normalized read helper
    ("    /// THE ROW BANK'S OWN READ (the autopush-363 round — the same",
     "    /// THE HELD WORD (the wire order's own resolution"),
]

PART11 = [
    # the 361 test's own row-lane update
    ("    // THE CHEST ROW'S OWN BANK (the 363 round — the place the row's",
     "    // THE CARRIED THE CHEST ROW HOLDS BACK"),
    # the round's own test tail
    ("/// THE CHEST ROWS' OWN KIND BANK (the autopush-363 round — the dev-204",
     None),
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
