#!/usr/bin/env python3
"""dev-213 — THE FMT REGION WALK (the 372..392 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the furnace trio's own kind bank (the store decls +
                  ctor + the NBT bank/parse + the load/save rows + the
                  switch/flush/spill rides + the helpers + the click
                  four-arm + the tick syncs + the wire wall + the
                  tooltip read + the paint/drag reads + the place
                  bank).
  part_11.rs    — the round's own test tail (the furnace trio kind
                  bank's eight arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the store decls
    (
        "/// THE FURNACE TRIO'S OWN KIND BANK (the dev-213 round — the",
        "pub furnace_is_live: bool,",
    ),
    (
        "/// THE FURNACE TRIO'S OWN LIVE WORDS",
        "pub furnace_sim: crate::furnace_sim::FurnaceSim,",
    ),
    # the NBT bank + parse
    ("pub fn furnace_kinds_bank_to_nbt", "/// THE BREWING WORDS' OWN BANK"),
    # the ctor
    (
        "furnace_kind_stores: std::collections::HashMap::new(),",
        "furnace_slots: vec![(0, 0); 39],",
    ),
    (
        "furnace_seat_potion_kinds: [crate::brewing_sim::POTION_KIND_NONE; 3],",
        "furnace_sim: crate::furnace_sim::FurnaceSim::default(),",
    ),
    # the live tick's own word sync
    (
        "// THE TRIO'S OWN WORD SYNC (the dev-213 round — the tick's",
        "if flipped {",
    ),
    # the banked tick's own word sync
    (
        "// THE TRIO'S OWN WORD SYNC (the dev-213 round — the\n                    // banked cell's tick",
        "self.furnace_stores.insert(cell, trio);",
    ),
    # the switch's word ride home
    (
        "// THE TRIO'S OWN WORD RIDE HOME (the dev-213 round —\n                    // the kind words the same switch walk re-banks",
        "// THE PREV CELL'S OWN FILL",
    ),
    # the open's own word load
    (
        "// THE TRIO'S OWN WORD LOAD (the dev-213 round — the banked",
        "// THE LOADED INPUT'S OWN PRIME",
    ),
    # the flush's word ride home
    (
        "// THE TRIO'S OWN WORD RIDE HOME (the dev-213 round — the\n            // kind words the same flush banks",
        "self.container_stores_file_save();",
    ),
    # the spill's own death ride
    ("// THE KINDS' OWN DEATH RIDE", "// THE FILL'S OWN DEATH RIDE"),
    # the helpers
    ("/// THE FURNACE TRIO'S OWN SEAT KIND READ", "/// THE FURNACE'S OWN TOOLTIP SEAT"),
    # the tooltip's own read
    (
        "// THE KIND'S OWN RIDE (the 356 round the dev-213 wall re-wears",
        "let mut rows = Self::item_tooltip_rows_kind(id, kind, self.advanced_item_tooltips, &ench);",
    ),
    # the paint's own helper read
    (
        "// the seat's own word the helper reads (the dev-213 round —",
        "if seat_id == carried.0 && qc_collect_kind_refuses(seat_kind, carried_kind) {",
    ),
    # the drag's own helper read
    (
        "// THE COMPONENT HALF (the autopush-383 round the dev-213",
        "if !quickcraft_furnace_seat_may_place(menu, carried.0, kind) {",
    ),
    # the drag's own place bank
    (
        "// THE PLACE'S OWN WORD BANK (the autopush-383 round the",
        "self.hotbar_slots = self.inventory.hotbar();",
    ),
    # the load-once insert
    (
        "// THE PROGRESS WORDS' OWN PARSE (the dev-70 round —",
        "// THE BREWING WORDS' OWN PARSE",
    ),
    # the save row
    ("// THE FURNACE KINDS' OWN ROW (the dev-213 round — the", "// THE BREWING WORDS' OWN ROW"),
    # the click's own four-arm ride
    (
        "// THE KIND'S OWN FOUR-ARM RIDE (the dev-204 law the\n            // furnace's own trio re-wears",
        "if (slot as usize) < self.furnace_slots.len() {",
    ),
    # the wire landing's own NONE wall
    ("// the trio's own word wall (the dev-213 round — the", "let mut slots = vec![(0i32, 0i32); 39];"),
]

PART11 = [
    # the round's own test tail
    ("/// THE FURNACE TRIO'S OWN KIND BANK (the dev-213 round —", None),
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
