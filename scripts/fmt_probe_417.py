#!/usr/bin/env python3
"""autopush-417 — THE FMT REGION WALK (the 372..autopush-416 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world_model.rs — the menu type 13 constant's own doc row.
  screens.rs     — the gather's release walk re-key + the wire half +
                   the honest gates + the close arm + the sync arm +
                   the None arm's own branch.
  part_11.rs     — the round's own test (the wire half's arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLD_MODEL = [
    # the menu type 13 constant's own doc row (the clinit order speaks)
    (
        "/// `MenuType`'s enchantment — the EnchantmentMenu's registry id",
        "pub const MENU_TYPE_FURNACE: i32 = 14;",
    ),
]

SCREENS = [
    # the gather's release walk (the re-key the wire half wears)
    (
        "    /// THE ENCHANT GATHER'S OWN RELEASE WALK",
        "    /// THE ENCHANT GATHER'S OWN OFFLINE WALK",
    ),
    # the wire half (the round's own fn)
    (
        "    /// THE ENCHANT GATHER'S OWN WIRE HALF",
        "    /// THE WIRE SPREAD'S OWN WALK",
    ),
    # the spell click's honest gate
    (
        "    pub fn click_enchant_option(&mut self, id: usize) -> bool {",
        "        let i = id + 1;",
    ),
    # the wire close's own arm
    (
        "    pub fn close_enchanting(&mut self, back: ScreenKind) {",
        "        // THE DOUBLE-CLICK PAIR'S OWN RESET (the close_inventory's own",
    ),
    # the drag arm's honest gate
    (
        "    fn qc_arm_enchanting(&mut self, mx: i32, my: i32, button: usize) {",
        "        if self.qc_armed {",
    ),
    # the pickup lane's honest gate
    (
        "    fn click_enchanting_slot(&mut self, mx: i32, my: i32, button: i8) {",
        "        if self.enchant_slot_rects.is_empty() {",
    ),
    # the sync arm (the menu type 13 lane)
    (
        "                if c.menu_type_id == crate::world_model::MENU_TYPE_ENCHANTMENT {",
        "                if c.menu_type_id != crate::world_model::MENU_TYPE_CHEST",
    ),
    # the None arm's own branch
    (
        "                // the store dropped: a LIVE enchant table closes back to",
        "                // the window's OWN rows die with it (the lifecycle the",
    ),
]

PART11 = [
    # the round's own test (the wire half's arms)
    (
        "/// THE ENCHANT GATHER'S OWN WIRE HALF",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/world_model.rs": WORLD_MODEL,
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
