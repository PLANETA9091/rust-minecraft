#!/usr/bin/env python3
"""dev-209 — THE FMT REGION WALK (the 372..384 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the wire table's own kind sync (the carried word's
                  field, the store/cell/result rides, the throw's
                  retire, the four resets, the 384 wall's own voice,
                  the ride's own count-keyed merge/swap arms).
  part_11.rs    — the round's own test tail (the wire table).

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the field's own declaration
    ("THE WIRE TABLE'S OWN CARRIED WORD (the dev-209 round",
     "pub creative_source: Vec<(i32, i32)>"),
    # the init
    ("live_table_carried: (0, 0),\n            live_table_carried_potion_kind",
     "creative_source: Vec::new()"),
    # the close walk's own reset
    ("the window's death the word retires (the chest's own law",
     "self.container_removed_rode = true"),
    # the result-take caller
    ("wire_table_result_take(\n                    sink,",
     "        for cell in 0..9 {"),
    # the cell-pickup caller
    ("wire_table_cell_pickup(\n                        sink,",
     "        let Some(store) = (0..36)"),
    # the throw's own kind ride
    ("THE THROW'S OWN KIND RIDE (the 345 round's law the",
     "input: crate::world_view::ContainerInput::Throw"),
    # the store row's own kind seat
    ("THE STORE ROW'S OWN KIND SEAT (the caller's own map the",
     "    /// The recompute law (the InventoryMenu.slotsChanged walk"),
    # the 384 wall's own voice (the distribute caller re-key)
    ("/// wears; the dev-209 round the 384 wall the field's own word\n    /// voices)",
     "fn qc_distribute_table_wire(&mut self) {\n        let Some(sink) = self.live_clicks.as_ref() else {\n            return; // no live session — the command dies on the frame thread\n        };\n        let Some((window_id, state_id)) = self.table_live_window else {\n            return; // the stand-in: no window, no echo\n        };\n        let qc_type = self.qc_type;\n        let qc_slots = self.qc_slots.clone();\n        let items_kinds = &self.items_potion_kinds;\n        let carried_kind = self.live_table_carried_potion_kind;\n        let seat_kinds = |menu: usize| match menu {\n            10..=36 => items_kinds[menu - 1],\n            37..=45 => items_kinds[menu - 37],\n            _ => crate::brewing_sim::POTION_KIND_NONE,\n        };"),
    # the collect's own wall
    ("THE COLLECT'S OWN WALL (the cell's stack the kind the",
     "let _ = sink.send(crate::world_view::ContainerCommand::Click("),
    # the wholesale sync's own reset
    ("self.live_table_carried = c.carried;\n                    // the wire's own word",
     "if self.screen == ScreenKind::World {\n                        self.open(ScreenKind::Crafting);"),
    # the no-window reset
    ("self.live_table_carried = (0, 0); // no window, no held stack\n                self.live_table_carried_potion_kind",
     "self.live_table_title = None; // the title dies with the window"),
    # the ride's own count-keyed arms
    ("the kind the SEAT wears after the click (the merge the count's",
     "/// THE WINDOW-0 PICKUP ROUND (the doClick PICKUP table the"),
    # the result take's own crafted wall (doc + signature + body)
    ("THE CRAFTED WORD'S OWN\n/// WALL (the dev-209 round",
     "/// THE TABLE'S OWN QUICK-MOVE BATCH (doClick's QUICK_MOVE arm over"),
    # the cell pickup's own wall ride (doc + signature + body)
    ("THE CURSOR'S OWN WALL\n/// RIDE (the dev-209 round",
     "/// THE TABLE'S STORE-ROW PICKUP (the doClick PICKUP arm over the"),
    # the store pickup's own rides (doc + signature + body)
    ("THE KIND'S OWN RIDE (the dev-209 round\n/// — the wire table's own word the store rows the real rides serve)",
     "/// THE PACK LAW (javap 26.2 CraftingInput.ofPositioned"),
]

PART11 = [
    # the round's own test tail
    ("/// THE WIRE TABLE'S OWN KIND SYNC (dev-209 — the table lane the",
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
