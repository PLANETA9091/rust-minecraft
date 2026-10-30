#!/usr/bin/env python3
"""dev-202 — THE FMT REGION WALK (the 323 record's own law the round
wears): the whole-file rustfmt the ORIGINAL text the formatted stdout
the diff answers — the MY-REGIONS verdict the marker ranges keep (no
hunk may touch the round's own edits), the pre-existing static noise
the HEAD law confirms the non-mine hunks the honest inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# the screens.rs regions (the doc-head anchors the round's own edits
# keep — each block the dev-202 rounds' own doc words wear)
SCREENS = [
    # the ScreenKind::Anvil enum row
    ("THE ANVIL'S OWN REPAIR WALK (the dev-202 round — javap 26.2", "    Anvil,"),
    # the state fields
    ("THE ANVIL'S OWN TRANSIENT TRIO (the dev-202 round — the", "    pub anvil_pos: Option<(i32, i32, i32)>,"),
    # the open + close + ride-home + sync + seat-map block
    ("THE ANVIL'S OWN MIRROR SYNC (the enchanting's own law — the", "    /// THE ANVIL'S OWN CLICK LANE"),
    # the walk + the name law + the responder + the take + the hover
    ("THE COST WALK (javap 26.2 AnvilMenu.createResult pc 0-1077 the", "    /// THE ANVIL'S OWN THROW LANE"),
    # the click walk + the lanes
    ("THE ANVIL'S OWN CLICK LANE (the offline sim's own walk — the", "    /// THE ANVIL DRAG'S OWN ARM"),
    # the qc trio + the typing pair
    ("THE ANVIL DRAG'S OWN ARM (the same mouseClicked pcs 316-378", "    /// THE ANVIL FIELD'S OWN BACKSPACE"),
    ("THE ANVIL FIELD'S OWN CHARS (the charTyped lane the focused box", "    /// THE ANVIL FIELD'S OWN BACKSPACE"),
    # the door arm
    ("THE ANVIL'S OWN GUI DOOR (the dev-202 round — javap 26.2", "            self.open_anvil_for(hit.pos);"),
    # the live route arm
    ("THE ANVIL'S OWN WIRE LANE (the dev-202 round — the", "                if c.menu_type_id != crate::world_model::MENU_TYPE_CHEST {"),
    # the render walk (the region the NEXT fn's doc the tail keeps —
    # the pre-existing static noise the HEAD law banks beyond)
    ("THE ANVIL'S OWN ROUND (the dev-202 round — the vanilla", "    /// THE BEACON'S OWN ROUND"),
]
WORLD = [
    ("THE ANVIL'S OWN KIND SEAT (the dev-202 round — the 350 NEXT list's", "    id == ANVIL\n}"),
]
WORLD_VIEW = [
    ("THE ANVIL'S OWN RENAME WORD (the dev-202 round — the vanilla", "    },"),
]
WINDOW = [
    ("THE ANVIL'S OWN CHARS (the charTyped lane the focused", "            }"),
    ("THE ANVIL'S OWN BACKSPACE (the EditBox's own keyPressed", "            }"),
    ("THE ANVIL'S RENAME WORD (the vanilla", "                            session.rename_item(&name)\n                        }"),
]
PART11 = [
    ("THE ANVIL'S OWN DOOR SEAT (the dev-202 round — the 26.2", None),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/world.rs": WORLD,
    "crates/mc-client/src/world_view.rs": WORLD_VIEW,
    "crates/mc-client/src/window.rs": WINDOW,
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
            if tag != "equal":
                hunks.append((i1, i2))
        for head_marker, tail_marker in regions:
            start = find(original, head_marker)
            end = region_end(original, tail_marker, start)
            start_line = original.count("\n", 0, start)
            end_line = original.count("\n", 0, end)
            overlap = [
                (a, b) for (a, b) in hunks
                if a < end_line and b > start_line
            ]
            if overlap:
                ok = False
                print(f"{path}: REGION DIRTY [{start_line},{end_line}) "
                      f"head={head_marker[:60]!r} hunks={overlap}")
            else:
                print(f"{path}: region clean [{start_line},{end_line}) "
                      f"head={head_marker[:48]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
