#!/usr/bin/env python3
"""autopush-322 — THE FMT REGION WALK (the 321 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits), the pre-existing
static noise the HEAD law confirms the non-mine hunks the honest
inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

REGIONS = {
    "crates/mc-client/src/world.rs": [
        # the law layer (the tiers + the validator + the formulas +
        # the AABB + the payment verdict the round banks)
        ("THE BEACON'S OWN EFFECT TIERS", "THE ANCHOR'S OWN CHARGE-LADDER MESH FOLD"),
    ],
    "crates/mc-client/src/gui.rs": [
        # the beacon sprite lane (the dynamic walk's own widening)
        ("THE BEACON BUTTONS' OWN LANE", "if !name.starts_with(\"mob_effect/\")"),
    ],
    "crates/mc-client/src/screens.rs": [
        # the bank/parse walk (the powers rows the round extends)
        ("THE BEACON BANK'S OWN WALK", "THE BED SPAWN'S OWN BANK"),
        # the screen state fields (the round's own pair wears)
        ("THE BEACON POWERS' OWN BANK", "THE PIG/COW VARIANTS' OWN SKINS"),
        # the ctor init rows (the fields' own zeros)
        ("beacon_stores: std::collections::BTreeSet::new(),",
         "chicken_warm_model: None,"),
        # the tick lane call (the effects beat's own seat)
        ("THE BEACON'S OWN EFFECTS BEAT (the autopush-322 round",
         "} else if self.health > 0.0 {"),
        # the open/close/click/confirm/quick-move/reapply block
        ("THE BEACON'S OWN OFFLINE OPEN", "THE STORE'S OWN FLUSH (the dev-66 round"),
        # the render fn
        ("THE BEACON'S OWN ROUND (the autopush-322 round", "THE DIMENSION SWAP"),
        # the door arm (the block lane's own seat — the javap ref the
        # comment alone carries)
        ("BeaconBlock.useWithoutItem pc 0-48", "// THE CAKE'S OWN BLOCK-USE ARM"),
        # the ScreenKind variant (the bare row the enum alone keeps)
        ("    Beacon,", "    Accessibility,"),
        # the press dispatch arms (the left + the right seats)
        ("THE BEACON'S OWN PRESS (the autopush-322 round",
         "if self.screen == ScreenKind::Inventory {"),
        ("THE BEACON'S OWN RIGHT PRESS (the autopush-322 round",
         "if self.screen == ScreenKind::Creative {"),
        # the render match arm
        ("THE BEACON'S OWN ROUND (the vanilla BeaconScreen over",
         "ScreenKind::Inventory => {"),
        # the load row ride
        ("let (cells, powers) = beacon_cells_from_nbt(&root);",
         "self.furnace_word_stores"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        # the 320 bank round-trip update (the powers the pairs keep)
        ("THE BANK'S OWN ROUND TRIP", "fn the_offline_beacon_the_placement_walk"),
        # the two REAL tests + the helper (the file tail the append wears)
        ("fn the_offline_beacon_the_gui_door_the_powers_the_payment_wear() {",
         "@@EOF@@"),
    ],
}


def fmt_lines(path):
    r = subprocess.run([RUSTFMT, "--emit", "stdout", path],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(f"FMT-ERR {path}: {r.stderr[:400]}")
        sys.exit(2)
    return r.stdout.split("\n")


def marker_ranges(lines, marks, path):
    out = []
    n = len(lines)
    for start, end in marks:
        s = None
        for i, l in enumerate(lines):
            if start in l:
                s = i
                break
        if s is None:
            print(f"MISSING START MARKER {path}:{start!r}")
            sys.exit(2)
        if end == "@@EOF@@":
            out.append((s, n))
            continue
        e = None
        for i in range(s + 1, n):
            if end in lines[i]:
                e = i
                break
        if e is None:
            print(f"MISSING END MARKER {path}:{end!r}")
            sys.exit(2)
        out.append((s, e + 1))
    return out


def main():
    bad = 0
    for path, marks in REGIONS.items():
        orig = open(path).read().split("\n")
        fmt = fmt_lines(path)
        sm = difflib.SequenceMatcher(None, orig, fmt, autojunk=False)
        regions = marker_ranges(orig, marks, path)
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag != "equal":
                hunks.append((i1, i2))
        dirty = [
            (i1, i2)
            for (i1, i2) in hunks
            if any(i1 < e and s < i2 for (s, e) in regions)
        ]
        if dirty:
            bad += 1
            print(f"DIRTY {path}: {len(dirty)} hunk(s) touch my regions")
            for h in dirty[:4]:
                print(f"    hunk [{h[0]}:{h[1]}]")
        else:
            noise = len(hunks)
            print(f"CLEAN {path} (my regions {len(regions)}, non-mine hunks {noise})")
    if bad:
        print(f"FMT REGION VERDICT: {bad} file(s) dirty")
        sys.exit(1)
    print("FMT REGION VERDICT: CLEAN")
    return 0


if __name__ == "__main__":
    sys.exit(main())
