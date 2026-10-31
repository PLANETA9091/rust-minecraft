#!/usr/bin/env python3
"""dev-212 — THE FMT REGION WALK (the 372..autopush-390 record's own
law the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  world_model.rs — the hopper's own registry const (the doc tail +
                   the MENU_TYPE_HOPPER the 16 id wears).
  screens.rs     — the wire decode's own hopper lane (the 41-36 five
                   the shape speaks, the canvas the flag wears), the
                   chest wire distribute's own five-split (the caller
                   + the free fn), the swap walk's own five-split.
  tests/mod.rs   — the hopper rects the helper lays.
  part_11.rs     — the drag-tails arm's own chest shape restore + the
                   round's own test tail (the six arms).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WORLD_MODEL = [
    ("/// `MenuType`'s hopper — the HopperMenu's registry id",
     "pub const MENU_TYPE_HOPPER: i32 = 16;"),
]

SCREENS = [
    # the chest wire distribute's own five-split (the caller's own
    # before-counts + closure + after-walk + the drag's own retire)
    ("THE FIVE'S OWN SPLIT (the chest family's own law the hopper's",
     "if self.chest_carried.1 <= 0 {\n            self.chest_carried_potion_kind = crate::brewing_sim::POTION_KIND_NONE;\n        }\n    }"),
    # the wire decode's own hopper lane (the five the shape speaks,
    # the canvas the flag wears)
    ("THE HOPPER'S OWN SHAPE (the menu type the five names:",
     "self.chest_window_hopper = hopper;"),
    # the free fn's own five-split (the seat read + the place write
    # the glen boundary names)
    ("THE FIVE'S OWN BOUNDARY (the chest family's own law the",
     "row.copy_from_slice(&mirror[27..36]);\n        *hotbar = row; // the HUD row the mirror's hotbar rows speak"),
    # the swap walk's own five-split (the row seat + the cases the
    # glen names, the HUD tail the mirror's own rows keep)
    ("THE FIVE'S OWN BOUNDARY (the chest family's own law the",
     "*hotbar = row;\n    }"),
]

MOD = [
    ("/// The hopper rects the render lays (the five seats' 0..5, the",
     "(sx + 1, sy + 1, (5 + i) as i16)\n        }))\n        .collect()\n}"),
]

PART11 = [
    # the drag-tails arm's own chest shape restore
    ("// THE CHEST'S OWN SHAPE (the seat vec the 27-grid + the mirror",
     "app.chest_mirror_sync_from_inventory();"),
    # the round's own test tail (the six arms)
    ("/// THE WIRE HOPPER'S OWN DOOR THE FIVE WEARS (the dev-212 round —",
     None),
]

REGIONS = {
    "crates/mc-client/src/world_model.rs": WORLD_MODEL,
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/mod.rs": MOD,
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
