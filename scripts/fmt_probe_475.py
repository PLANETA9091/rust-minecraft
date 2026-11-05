#!/usr/bin/env python3
"""autopush-473 — THE FMT REGION WALK (the 472 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits). The 475 suffix
the 472's 474 wheel the round speaks.

Regions:
  mob.rs     — the enum's own variant + the category's own arm, the
               wire round trip (from_wire_name + wire_name), the six
               per-kind tables (eye/health/panic/move/stroll + the
               voice/step rows), the consts pair, the box's own arm,
               the mob_interact's own talk arm, the food/dandelion/
               loot arms, the round's own test.
  screens.rs — the entity id's own arm, the talk award's own walk,
               the render's own undrawn arm, the flame bbox's own
               pair.
Usage: python3 scripts/fmt_probe_475.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

MOB = [
    # the enum's own variant (the census docstring the body wears)
    (
        "    /// THE WANDERER'S OWN BODY (javap",
        "/// THE MOB CATEGORY'S OWN CENSUS SLICE",
    ),
    # the category's own arm
    (
        "            MobKind::WanderingTrader => MobCategory::Creature,",
        "    pub fn from_wire_name(name: &str) -> Self {",
    ),
    # the from_wire admission
    (
        "        } else if name == \"minecraft:wandering_trader\" {",
        "        } else if name == \"minecraft:cow\" {",
    ),
    # the wire_name arm
    (
        '            MobKind::WanderingTrader => "minecraft:wandering_trader",',
        "    /// THE MONSTER CHAIN'S OWN WORD",
    ),
    # eye_height arm
    (
        "            MobKind::WanderingTrader => 1.6200000047683716,",
        "    pub fn max_health(self) -> f32 {",
    ),
    # max_health arm
    (
        "            MobKind::WanderingTrader => 20.0,",
        "    pub fn panic_speed(self) -> Option<f64> {",
    ),
    # panic arm
    (
        "            MobKind::WanderingTrader => Some(0.5),",
        "    pub fn movement_speed(self) -> f64 {",
    ),
    # movement arm
    (
        "            MobKind::WanderingTrader => 0.5,",
        "    pub fn stroll_speed(self) -> f64 {",
    ),
    # stroll arm
    (
        "            MobKind::WanderingTrader => 0.35,",
        "    pub fn voice_event(self, slot: VoiceSlot) -> &'static str {",
    ),
    # voice rows
    (
        '            (MobKind::WanderingTrader, VoiceSlot::Ambient) => "entity.wandering_trader.ambient",',
        "    /// THE STEP VOICE (the playStepSound law the kinds split",
    ),
    # step arm
    (
        "            MobKind::WanderingTrader => {\n                use mc_registry::generated::block_sounds as bs;",
        "pub fn farm_sound_variant_word(",
    ),
    # consts pair
    (
        "pub const WANDERING_TRADER_WIDTH: f64 = 0.6;",
        "pub const CREEPER_MOVEMENT_SPEED: f64 = 0.25;",
    ),
    # box_for arm
    (
        "            MobKind::WanderingTrader => (WANDERING_TRADER_WIDTH, WANDERING_TRADER_HEIGHT),",
        "        // THE BABY'S OWN DIMENSIONS",
    ),
    # the talk arm (mob_interact)
    (
        "            _ if self.kind == MobKind::WanderingTrader && held_item != \"villager_spawn_egg\" => {",
        "    /// THE TAME WALK'S OWN ROLL",
    ),
    # the food refusal
    (
        "            // THE WANDERER'S OWN REFUSAL (the AbstractVillager chain",
        "    /// THE SET-IN-LOVE WALK",
    ),
    # the dandelion always-pass
    (
        "            // THE WANDERER'S OWN ALWAYS-PASS (javap the jar's own",
        "    /// THE GOLDEN-DANDELION'S OWN WALK",
    ),
    # the loot empty law
    (
        "            MobKind::WanderingTrader => {\n                // THE WANDERER'S OWN EMPTY LAW",
        "            MobKind::Creeper => {",
    ),
    # the round's own test
    (
        "    fn the_wanderer_talk_arm_the_wire_kind_wears() {",
        "    /// THE UNDERCOAT'S OWN GATE",
    ),
]

SCREENS = [
    # the entity id's own arm
    (
        "        // THE WANDERER'S OWN ID (the talk arm the kind rides",
        "}\n\n/// THE ENTITY NAME'S OWN STATIC ID",
    ),
    # the talk award's own walk (the caller's own arm)
    (
        "                    if let crate::mob::MobInteract::Talk = verdict {",
        "                    entity_pass = true;",
    ),
    # the render's own undrawn arm
    (
        "                crate::mob::MobKind::WanderingTrader => {\n                    // THE WANDERER'S OWN UNDRAWN LAW",
        "                crate::mob::MobKind::Sheep => {",
    ),
    # the flame bbox's own pair
    (
        "                // THE WANDERER'S OWN SIZED PAIR (javap the EntityTypes",
        "            let (bbox_w, bbox_h) = if mob.is_baby() {",
    ),
]

REGIONS = {
    "crates/mc-client/src/mob.rs": MOB,
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
