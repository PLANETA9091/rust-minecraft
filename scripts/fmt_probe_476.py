#!/usr/bin/env python3
"""autopush-474 — THE FMT REGION WALK (the 473 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits). The 476 suffix
the 473's 475 wheel the round speaks.

Regions:
  avatar.rs  — the wanderer's own model section (the state + the sha1
               const + the model impl + the parts builder), the
               round's own three tests.
  screens.rs — the model field, the None init, the lazy load, the
               local binding, the render's own villager walk arm.
Usage: python3 scripts/fmt_probe_476.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

AVATAR = [
    # the wanderer's own model section (the banner to the chicken's)
    (
        "// THE WANDERING TRADER (the villager robe the talk arm's own body wears)",
        "// THE CHICKEN (the fifth drawn body — the avian's own chain)",
    ),
    # the round's own tests (to the module's close)
    (
        "    /// THE WANDERER'S OWN MESH WALK (the VillagerModel.createBodyModel",
        None,
    ),
]

SCREENS = [
    # the model field
    (
        "    /// THE WANDERER'S own villager robe (the VillagerModel mesh the",
        "    /// THE FOURTH MOB's own WOOL",
    ),
    # the None init
    (
        "            wandering_trader_model: None,",
        "            sheep_wool_model: None,",
    ),
    # the lazy load
    (
        "        // THE WANDERER'S own lazy load (the villager robe the talk",
        "        // THE VARIANT SKINS' OWN LAZY WALK (the cold/warm pairs the",
    ),
    # the local binding
    (
        "        let wandering_trader_model = self.wandering_trader_model.as_ref();",
        "        // the variants' honesty is ITS OWN lane: a failed variant skin",
    ),
    # the render's own villager walk arm
    (
        "                crate::mob::MobKind::WanderingTrader => {\n"
        "                    // THE WANDERER'S OWN VILLAGER WALK (javap",
        "                crate::mob::MobKind::Sheep => {",
    ),
]

REGIONS = {
    "crates/mc-client/src/avatar.rs": AVATAR,
    "crates/mc-client/src/screens.rs": SCREENS,
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
            inside = [
                (i1, i2, tag, j1, j2)
                for (i1, i2, tag, j1, j2) in hunks
                if i1 < e_line and i2 > s_line
            ]
            if inside:
                ok = False
                print(f"[MY-REGIONS DIRTY] {path} :: {start_marker[:60]!r}")
                for h in inside:
                    print(f"   hunk {h}")
            else:
                print(f"[MY-REGIONS CLEAN] {path} :: {start_marker[:60]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
