#!/usr/bin/env python3
"""autopush-456 — THE FMT REGION WALK (the 455 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the beacon's own wire lane block the decode arm
                  wears + the close/click/gather route arms the round
                  banks (no rustfmt hunk may touch them).
  world_model.rs — the MENU_TYPE_BEACON const the round banks.
  part_11.rs    — the round's own test (the beacon the menu-type-9
                  route wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the beacon's own live fields (the flag + the window identity)
    (
        "THE BEACON SCREEN'S OWN LIVE FLAG (the sibling lanes' own law",
        "pub beacon_slot_rects: Vec<(i32, i32, i16)>,",
    ),
    # the gather's own wire walk (the belief's own snapshot)
    (
        "THE BEACON GATHER'S OWN WIRE WALK (the anvil's own shape the",
        "/// THE BEACON GATHER'S OWN OFFLINE WALK (the doClick PICKUP_ALL",
    ),
    # the wire's own close (the live echo the close wears)
    (
        "THE WIRE'S OWN CLOSE (the anvil close's own shape",
        "// THE CARRIED'S OWN HOME RIDE (the super.removed walk",
    ),
    # the click's own wire echo (the prediction-free lane)
    (
        "THE BEACON CLICK'S OWN WIRE ECHO (the enchant's 419 shape the",
        "// THE QUICK-MOVE LANE (the shift-click — javap",
    ),
    # the live confirm's own close (the server's own law)
    (
        "THE LIVE CONFIRM'S OWN CLOSE (the anvil close's own law the",
        "let Some(pos) = self.beacon_open_cell else {",
    ),
    # the beacon's own wire lane (the decode arm the open lands)
    (
        "THE BEACON'S OWN WIRE LANE (the anvil's dev-202",
        "if c.menu_type_id == crate::world_model::MENU_TYPE_ENCHANTMENT {",
    ),
]

WORLD_MODEL = [
    # the MENU_TYPE_BEACON const (the census the round retires)
    (
        "`MenuType`'s beacon — the BeaconMenu's registry id",
        "pub const MENU_TYPE_BEACON: i32 = 9;",
    ),
]

PART11 = [
    # the round's own test (the beacon the menu-type-9 route
    # wears — EOF the end the round keeps)
    (
        "THE BEACON'S OWN WIRE ROUTE (the 455 NEXT's own lane the wheel",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/world_model.rs": WORLD_MODEL,
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


def formatted_of(path: str) -> str:
    """the formatted text the stdout answers — the filename header this
    rustfmt wears the strip removes"""
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
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
