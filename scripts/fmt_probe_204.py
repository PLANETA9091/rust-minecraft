#!/usr/bin/env python3
"""dev-204 — THE FMT REGION WALK (the 323 record's own law the
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

SCREENS = [
    # the field decl (the word's own seat the inv_carried tail keeps)
    ("    pub inv_carried: (i32, i32),\n    /// THE OFFLINE WORD'S OWN KIND SEAT",
     "    /// The inventory panel's GUI origin the LAST render laid out —"),
    # the offhand ride (the predict block the result-slot doc keeps)
    ("        // THE KIND'S OWN OFFHAND RIDE (the dev-204 round — the store",
     "    /// The RESULT slot's click (the InventoryMenu's slot 0 — the"),
    # the close ride-home (the capture the close's own tail keeps)
    ("            // THE WORD'S OWN RIDE HOME (the dev-199 law the brewing",
     "    /// THE DOUBLE-CLICK'S OWN RELEASE WALK (the mouseReleased walk pcs"),
    # the fresh-open pair (the creative gate the close doc keeps)
    ("        // the fresh-open pair (the carried's own word the same silence",
     "    /// Close the survival inventory (the E key's second half and the"),
    # the table-open pair (the grid clear the furnace doc keeps)
    ("        self.inv_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE;\n        self.open(ScreenKind::Crafting);",
     "    /// THE FURNACE'S OWN OFFLINE OPEN (javap 26.2 AbstractFurnaceBlock."),
    # the store-row ride (the drop-outside comment the aimed-throw doc keeps)
    ("            // The press BETWEEN the slots (inside the panel) stays silent.",
     "    /// THE AIMED THROW (LivingEntity.createItemStackToDrop's own else"),
    # the throw word (the aimed drop the furnace-throw doc keeps)
    ("        // THE THROW'S OWN KIND WORD (the 345 round's live law the",
     "    /// THE FURNACE CARRIED'S OWN THROW (the 26.2 mouseClicked's own"),
    # the ghost read (the carried tint the count-write tail keeps)
    ("                    // THE KIND'S OWN CARRIED TINT (the autopush-344",
     "                    if carried_kind != crate::brewing_sim::POTION_KIND_NONE {"),
]
PART11 = [
    ("THE SURVIVAL STORE'S OWN CURSOR WORD (the dev-204 round", None),
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
