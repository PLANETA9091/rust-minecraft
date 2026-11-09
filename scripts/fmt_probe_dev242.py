#!/usr/bin/env python3
"""dev-242 — THE FMT REGION WALK (the 535..549 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges keep
(no hunk may touch the round's own edits).

Regions: the round's own horse interact walk the lanes wear. Shapes:
the TIGHT BLOCK (the unique statement lines the round's own edits
bound), the EOF-fallback tail test (the body fn the round's own walk
wears, no fn after — the whole tail the region keeps). The touched
files the width audit answers (the 100-width law the audit pins) and
the count law (the identical insert lines the helper literals repeat,
the per-file count the walk pins).
"""
import difflib
import os
import re
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

WIDTH_LAW = 100
FN_LINE = re.compile(r"^    fn |^    pub fn |^fn |^pub fn ")

MOB_PATH = "crates/mc-client/src/mob.rs"
MOB_REGIONS = [
    # THE TAG + THE CEILING (the horse food rows + the temper clamp
    # the jar's own bytes bound)
    (
        "/// THE HORSE'S OWN FOOD TAG (the jar's",
        "/// `AgeableMob.getBabyStartAge`",
    ),
    # THE VERDICTS' OWN FOUR (the FedFood/MakeMad/HorseRide/
    # OpenInventory variants the Talk arm bounds)
    (
        "    /// THE HORSE'S OWN FED-FOOD ARM",
        "impl MobInteract {",
    ),
    # THE TEMPER FIELD (the doc the leash link bounds)
    (
        "    /// THE HORSE'S OWN TEMPER (javap",
        "    /// THE LEASH'S OWN LINK (javap",
    ),
    # THE WALK'S OWN SIGNATURE + ARM (the delegate + the sneak split
    # + the horse chain arm the shears arm bounds)
    (
        "    pub fn mob_interact(&mut self, held_item: &str, rng: &mut Lcg) -> MobInteract {",
        "            \"shears\" if self.kind == MobKind::Sheep",
    ),
    # THE VOICE PAIR (the eat + the angry the pitch walk bounds)
    (
        "    pub fn eat_voice(&self) -> Option<&'static str> {",
        "    /// `LivingEntity.getVoicePitch`",
    ),
    # THE HELPERS' OWN TRIO (the rows + the fedFood + the super chain
    # the tame roll bounds)
    (
        "    /// `AbstractHorse.handleEating`'s OWN ROWS",
        "    /// THE TAME WALK'S OWN ROLL",
    ),
    # THE WALK'S OWN TEST (the doc the spider walk bounds)
    (
        "    /// THE HORSE'S OWN INTERACT WALK THE CHAIN WEARS",
        "    /// THE SPIDER'S OWN REGISTRATION WORDS",
    ),
]

SCREENS_PATH = "crates/mc-client/src/screens.rs"
SCREENS_REGIONS = [
    # THE DISPATCH'S OWN SNEAK (the verdict line the stand-down
    # comment bounds)
    (
        "                    let mut verdict =",
        "                    // THE SECONDARY-USE STAND-DOWN",
    ),
    # THE FOUR ARMS' OWN WALK (the FedFood/MakeMad/HorseRide/
    # OpenInventory callers the Feed arm bounds)
    (
        "                    if let crate::mob::MobInteract::FedFood { ate, loved } = verdict {",
        "                    if let crate::mob::MobInteract::Feed { love, grow } = verdict {",
    ),
    # THE MOUNT'S OWN WALK (the horse mount the cart bounds)
    (
        "    /// THE HORSE'S OWN MOUNT (javap `AbstractHorse.doPlayerRide`",
        "    /// THE CART'S OWN MOUNT (javap `Minecart.interact`",
    ),
]

PART11_PATH = "crates/mc-client/src/screens/tests/part_11.rs"
PART11_REGIONS = [
    # THE VANILLA ROWS' OWN FOOD PINS (the carrot row the HORSE_FOOD
    # tag admits — the old two-row gate the dev-242 walk replaces)
    (
        "/// THE HORSE'S OWN VANILLA ROWS",
        "/// THE HORSE'S OWN Q WALK",
    ),
    # THE RIDE TEST'S OWN DOC (the doc the round's fn bounds — the
    # EOF fallback the tail keeps)
    (
        "/// THE HORSE'S OWN RIDE MOUNT (javap `AbstractHorse.doPlayerRide`",
        None,
    ),
    # THE RIDE TEST'S OWN BODY (the fn the EOF fallback keeps — the
    # walk the tail test wears)
    (
        "fn the_horse_the_interact_ride_the_mount_wears",
        None,
    ),
]


def region_bounds(lines, start, end, fn_line) -> tuple:
    si = None
    for i, line in enumerate(lines):
        if start in line:
            si = i
            break
    if si is None:
        raise SystemExit(f"START MARKER NOT FOUND: {start[:60]}")
    if end is None:
        # the EOF fallback the tail test wears — the next indented fn
        # after the start the region closes, the whole tail the
        # no-fn-after keeps
        for j in range(si + 1, len(lines)):
            if fn_line.match(lines[j]):
                return (si, j)
        return (si, len(lines))
    ei = None
    for j in range(si + 1, len(lines)):
        if end in lines[j]:
            ei = j
            break
    if ei is None:
        ei = len(lines)  # the EOF fallback the tail test wears
    return (si, ei)


def width_audit(path, needle) -> bool:
    ok = True
    for i, line in enumerate(open(path, encoding="utf-8"), 1):
        if needle in line and len(line.rstrip("\n")) > WIDTH_LAW:
            ok = False
            print(f"WIDTH  {path}:{i} ({len(line.rstrip())} cols) {line.strip()[:60]}")
    return ok


def count_audit(path, needle, want) -> bool:
    got = sum(1 for line in open(path, encoding="utf-8") if needle in line)
    if got != want:
        print(f"COUNT  {path}: '{needle}' {got} sites, the law wants {want}")
        return False
    print(f"COUNT  {path}: '{needle}' {got} sites the law wears")
    return True


def walk(path, regions, fn_line=FN_LINE) -> bool:
    ok = True
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path], capture_output=True, text=True
    )
    if proc.returncode != 0:
        raise SystemExit(f"RUSTFMT FAILED\n{proc.stderr[:400]}")
    mine_lines = original.split("\n")
    theirs_lines = proc.stdout.split("\n")
    for (start, end) in regions:
        ms, me = region_bounds(mine_lines, start, end, fn_line)
        ts, te = region_bounds(theirs_lines, start, end, fn_line)
        mine = "\n".join(mine_lines[ms:me])
        theirs = "\n".join(theirs_lines[ts:te])
        label = mine_lines[ms].strip()[:58]
        if mine == theirs:
            print(f"CLEAN  {label}")
        else:
            ok = False
            print(f"DIRTY  {label}")
            for line in difflib.unified_diff(
                mine.splitlines(), theirs.splitlines(), "mine", "rustfmt",
                lineterm="", n=1,
            ):
                print(line)
    return ok


def main() -> int:
    ok = walk(MOB_PATH, MOB_REGIONS)
    ok = walk(SCREENS_PATH, SCREENS_REGIONS) and ok
    ok = walk(PART11_PATH, PART11_REGIONS) and ok
    # THE COUNT LAW (the identical insert lines the literal sites
    # repeat)
    ok = count_audit(MOB_PATH, "pub const HORSE_FOOD", 1) and ok
    ok = count_audit(MOB_PATH, "pub const HORSE_MAX_TEMPER", 1) and ok
    ok = count_audit(MOB_PATH, "pub temper: i32,", 1) and ok
    ok = count_audit(MOB_PATH, "fn fed_food", 1) and ok
    ok = count_audit(MOB_PATH, "fn horse_super_chain", 1) and ok
    ok = count_audit(MOB_PATH, "fn mob_interact_with_sneak", 1) and ok
    ok = count_audit(SCREENS_PATH, "fn mount_offline_horse", 1) and ok
    ok = count_audit(PART11_PATH, "fn the_horse_the_interact_ride_the_mount_wears", 1) and ok
    # THE WIDTH LAW (the round's own words the 100 cols pin)
    for path in (MOB_PATH, SCREENS_PATH, PART11_PATH):
        ok = width_audit(path, "horse_variant") and ok
        ok = width_audit(path, "horse_markings") and ok
        ok = width_audit(path, "temper") and ok
        ok = width_audit(path, "HORSE_FOOD") and ok
        ok = width_audit(path, "HORSE_MAX_TEMPER") and ok
        ok = width_audit(path, "fed_food") and ok
        ok = width_audit(path, "horse_super_chain") and ok
        ok = width_audit(path, "mob_interact_with_sneak") and ok
        ok = width_audit(path, "mount_offline_horse") and ok
        ok = width_audit(path, "angry_voice") and ok
        ok = width_audit(path, "FedFood") and ok
        ok = width_audit(path, "HorseRide") and ok
        ok = width_audit(path, "OpenInventory") and ok
        ok = width_audit(path, "MakeMad") and ok
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
