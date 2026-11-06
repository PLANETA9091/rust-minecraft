#!/usr/bin/env python3
"""dev-227 — THE FMT REGION WALK (the 372..autopush-481 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions:
  mob.rs      — the EnderPearl kind variant, the pearl consts trio, the
                pearl_throw_from ctor + the owner-gate fn.
  screens.rs  — the throw lane, the dose queue's decl, the tick walk's
                queue + the pre capture, the clip arm's gate, the mob
                arm's gate, the portal burst, the teleport walk, the
                dose tail, the render's pearl sprite arm + the kind
                split.
  part_11.rs  — the round's own tests (the throw + the teleport walk
                the tail wears, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MOB = [
    # the EnderPearl kind variant (the onHit walk the doc carries — the
    # egg body's doc the next seat keeps)
    (
        "    /// `ThrownEnderpearl` — the TELEPORT",
        "/// THE THROWN EGG'S OWN BODY",
    ),
    # the pearl consts trio (the throw + the teleport voices + the dose)
    (
        "/// THE PEARL'S OWN THROW VOICE",
        "impl EggProjectile {",
    ),
    # the pearl ctor + the owner gate (the impl's close the walk spans)
    (
        "    /// THE PEARL'S OWN THROW (javap",
        "/// THE SEGMENT'S OWN SLAB WALK",
    ),
]

SCREENS = [
    # the dose queue's decl (the world borrow's own head the walk rides)
    (
        "                    // THE PEARL'S OWN DOSE QUEUE",
        "                        // THE PEACEFUL'S OWN DESPAWN",
    ),
    # the tick walk's queue + the pre capture (the iter the walk opens)
    (
        "                        // THE PEARL'S OWN TELEPORT QUEUE (the pre-move",
        "                        for egg in self.eggs.iter_mut() {",
    ),
    # the clip arm's gate + the chick doc re-wear (the mob sweep the
    # next seat keeps)
    (
        "                                // THE PEARL'S OWN TELEPORT QUEUE (the",
        "                            // THE MOB SWEEP (the boxes",
    ),
    # the mob arm's gate (the dead write the next seat keeps)
    (
        "                                // THE CHICK'S OWN ROLLS (the same onHit",
        "                                egg.dead = true;",
    ),
    # the portal burst (the snowball's own arm the next seat keeps)
    (
        "                            if bkind == crate::mob::ThrownProjectileKind::EnderPearl {",
        "                            // THE SNOWBALL'S OWN CASE-3 (javap",
    ),
    # the teleport walk (the arrow flights the next seat keeps)
    (
        "                        // THE PEARL'S OWN TELEPORT WALK (javap onHit pcs",
        "                        for arrow in self.arrows.iter_mut() {",
    ),
    # the dose tail (the shot spawns the next seat keeps)
    (
        "                // THE PEARL'S OWN DOSE TAIL",
        "                // THE SHOT'S OWN SPAWNS",
    ),
    # the throw lane (the egg's own arm the sibling seat keeps)
    (
        "            // THE PEARL'S OWN THROW (javap",
        "            // THE LOADED CROSSBOW: the instant shot",
    ),
    # the render's pearl sprite arm (the camera basis the next seat keeps)
    (
        "        let pearl_parts: Vec<mc_registry::element_quads::Quad> =",
        "        // the camera basis",
    ),
    # the kind split's pearl arm (the sampled push the next seat keeps)
    (
        "            // THE KIND'S OWN SPRITE (the egg the throw lane mints, the",
        "            for q in parts {",
    ),
]

PART11 = [
    # the round's own tests (the throw + the teleport walk the tail
    # wears — the EOF the end the round keeps)
    (
        "/// THE ENDER PEARL'S OWN THROW",
        None,
    ),
]

REGIONS = {
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
