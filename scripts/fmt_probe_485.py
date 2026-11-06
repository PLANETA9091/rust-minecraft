#!/usr/bin/env python3
"""autopush-485 — THE FMT REGION WALK (the 484 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  drops.rs    — the Lcg fn pair (the next_f32_between the round's
                own fn banks), the ItemDrop field (the damage_frac
                word the struct carries), the four ctor inserts (the
                pristine 0.0 the spawn law wears).
  mob.rs      — the set_damage block (the roll fn the round's own
                walk banks).
  screens.rs  — the retrieve fn (the set_damage word the drop
                carries).
  part_11.rs  — the round's own tests (the two new set_damage tests
                the tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

DROPS = [
    # the Lcg fn pair (the next_f32_between the round's own fn banks)
    (
        "    /// `Mth.nextDouble(random, min, max)` (javap: `min + nextDouble() *",
        "    /// `RandomSource.triangle(mode, delta)` (javap the default method",
    ),
    # the ItemDrop field (the damage_frac word the struct carries)
    (
        "    /// The potion's own kind word (the vanilla ItemStack's NBT",
        "    pub x: f64,",
    ),
    # the four ctor inserts (the pristine 0.0 the spawn law wears —
    # the whole impl's ctor span the four lines keep)
    (
        "    /// The popResource spawn (javap Block.popResource)",
        "    /// The entity's bounding box",
    ),
]

MOB = [
    # the set_damage block (the roll fn the round's own walk banks)
    (
        "/// THE SET_DAMAGE'S OWN ROLL",
        "/// THE CLOCKS' OWN ROLLS",
    ),
]

SCREENS = [
    # the retrieve fn (the crossbow shot the next seat keeps)
    (
        "    /// THE BOBBER'S OWN RETRIEVE (javap",
        "    fn crossbow_shot_offline(&mut self) {",
    ),
]

PART11 = [
    # the round's own tests (the tail the EOF keeps)
    (
        "/// THE SET_DAMAGE'S OWN ROLL",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/drops.rs": DROPS,
    "crates/mc-client/src/mob.rs": MOB,
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
