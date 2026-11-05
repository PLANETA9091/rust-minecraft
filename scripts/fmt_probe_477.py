#!/usr/bin/env python3
"""autopush-475 — THE FMT REGION WALK (the 474 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits). The 477 suffix
the 475's 476 wheel the round speaks.

Regions:
  mob.rs     — the pool tables + the offer struct + the generation fn
               + the projection fn, the Zombie field + the ctor init +
               the ensure helper, the natural-spawn call, the round's
               own four tests.
  screens.rs — the merchant_trader_id field + the init, the wire
               merge call, the talk window walk, the take re-bank,
               the two close resets.
  part_11.rs — the round's own test.
Usage: python3 scripts/fmt_probe_477.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MOB = [
    # the pool tables + the struct + the fns (the consts the anchor keeps)
    (
        "/// THE WANDERER'S OWN BUYING POOL (the trade_set/wandering_trader/buying",
        "/// `Creeper.createAttributes` — MOVEMENT_SPEED 0.25",
    ),
    # the Zombie struct field
    (
        "    pub farm_sound_variant: Option<String>,\n"
        "    /// THE WANDERER'S OWN OFFERS (the trade_set pools the ensure walk",
        "    /// THE GRAZE'S OWN CLOCK (Sheep.eatAnimationTick",
    ),
    # the ctor init
    (
        "            // THE WANDERER'S OWN OFFERS (the trade_set pools the kind\n"
        "            // the ensure helper arms",
        "            eat_animation_tick: 0,",
    ),
    # the ensure helper
    (
        "    /// THE WANDERER'S OWN OFFER ARM (the `updateTrades` walk the",
        "    /// The zombie's feet-anchored bbox (the registration's own dims).",
    ),
    # the natural-spawn call
    (
        "            // THE WANDERER'S OWN OFFERS (the updateTrades walk the\n"
        "            // spawn arms",
        "            // THE SKELETON'S OWN EQUIPMENT (the unconditional BOW",
    ),
    # the round's own tests (to the undercoat's gate)
    (
        "    /// THE WANDERER'S OWN OFFER POOL (the tradepool_477 census the",
        "    /// THE UNDERCOAT'S OWN GATE (javap SheepWoolUndercoatLayer.submit's",
    ),
]

SCREENS = [
    # the field
    (
        "    pub merchant_is_live: bool,\n"
        "    /// THE WANDERER'S OWN TRADER (the offline window's own entity",
        "    /// THE MERCHANT WINDOW'S OWN IDENTITY (the live window's",
    ),
    # the init
    (
        "            merchant_is_live: false,\n            merchant_trader_id: None,",
        "            merchant_live_window: None,",
    ),
    # the live close reset
    (
        "            self.merchant_is_live = false;\n"
        "            self.merchant_live_window = None;\n"
        "            self.merchant_trader_id = None;",
        "            self.merchant_carried = (0, 0);",
    ),
    # the offline close reset
    (
        "        self.merchant_trader_id = None; // the offline trader's disarm",
        "        self.open(back);\n    }",
    ),
    # the take re-bank
    (
        "            // THE WANDERER'S OWN RE-BANK (the offline window's own",
        "            // THE STAT'S OWN RIDE (pc 69-73 — the awardStat",
    ),
    # the talk window walk
    (
        "                        // THE WANDERER'S OWN TRADE WINDOW (the\n"
        "                        // openTradingScreen tail pc 75-87 the OFFLINE",
        "                        return true;\n                    }\n                    entity_pass = true;",
    ),
    # the wire merge call
    (
        "                // THE WANDERER'S OWN OFFERS (the updateTrades walk the\n"
        "                // merge arms",
        "                m.yaw = w.yaw;",
    ),
]

PART11 = [
    # the round's own test (to the file's close)
    (
        "/// THE WANDERER'S OWN WINDOW (javap `WanderingTrader.updateTrades` +",
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
