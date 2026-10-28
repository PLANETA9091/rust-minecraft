#!/usr/bin/env python3
"""autopush-310 — THE END GATEWAY'S OWN KIND SEAT pin.

The autopush-309 NEXT list's own first candidate: the teleport lane
round the chorus arm the gateway arm the client's own vocabulary
banks — the end_gateway kind the client never seats (the 797th the
vanilla's own INVISIBLE law the render retires). THE JAR LAW the live
disasm re-pins (scripts/teleport_scout_310.py the walk banks, rc=0 —
scripts/jar_work/teleport_310.txt the dumps serve):
EndGatewayBlock.entityInside the canUsePortal gate the
!isCoolingDown fold the setAsInsidePortal the triggerCooldown pair
speaks, TheEndGatewayBlockEntity.triggerCooldown the `bipush 40` the
cooldown value banks, getPortalDestination the arrival the
atBottomCenterOf(exit) the Vec3.ZERO velocity the transition banks,
getPortalPosition the exactTeleport fold the findExitPosition rescan
(exit.offset(0,2,0) radius 5 the center skip the .above() tail)
answers, findTallestBlock the column walk the isCollisionShapeFullBlock
verdict banks, TeleportRandomlyConsumeEffect.apply the 16-attempt loop
the three nextDouble draws the (d - 0.5) * diameter fold the
CHORUS_FRUIT_TELEPORT voice the resetFallDistance tail wears,
LivingEntity.randomTeleport the walk-down the blocksMotion verdict the
noCollision + containsAnyLiquid fold the restore arm keeps,
EndGatewayBlock.getRenderShape the INVISIBLE the render retires.
The pin:

  atlas.rs      KIND_NAMES + REGISTRY_NAMES the end_gateway tail row
                the every-existing-index law keeps (the goldens the
                same pixels render); N_KINDS 797 -> 798; the five
                N_KINDS tables the never-read rows the INVISIBLE law
                wears (FACE_TILES the TILE_NONE row, TINTS the
                no-tint row, DOOR_MODEL_BY_ROW the not-a-door empty
                row, ENTITY_HEAD_FACES the not-a-head row,
                CHEST_LOCK_TILES the TILE_NONE row).

  world.rs      NAME_OF the end_gateway row the tail seat wears, the
                decl keeps.

  sweep         the 56 kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 797 -> 798.

Idempotent: every section checks its own anchor first.
"""
import glob
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")

OLD_KINDS = 797
NEW_KINDS = 798


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def append_table_row(text: str, decl: str, row: str, old_kinds: int, new_kinds: int, tag: str) -> str:
    """The decl-anchored tail append (the ROW-COUNT check the idempotence
    keeps — the identical tail row the count the pinned state proves)."""
    start = text.find(decl)
    if start < 0:
        sys.exit(f"  [{tag}] decl anchor missing")
    close = text.find("\n];", start)
    body = text[start:close]
    rows = [l for l in body.split("\n") if l.rstrip().endswith(",")]
    if len(rows) == new_kinds:
        print(f"  [{tag}] already pinned")
        return text
    if len(rows) != old_kinds:
        sys.exit(f"  [{tag}] row count {len(rows)} != {old_kinds}")
    print(f"  [{tag}] pinned")
    return text[:close] + "\n" + row + text[close:]


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the kind count (the 798th seat the tail wears).
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )

    # 2. the names' tails (the kind + the registry rows the same law
    # wears — the vanilla registration order the batch's own member
    # keeps).
    atlas = edit_all(
        atlas,
        '"white_bed", "yellow_bed"];',
        '"white_bed", "yellow_bed", "end_gateway"];',
        2,
        "kind+registry names",
    )

    # 3. the five N_KINDS tables the never-read rows (the INVISIBLE
    # law the mesher's own early-retire lane serves — the generic
    # cube the gateway never wears, the faces the mesher never
    # reads).
    for decl, row, tag in [
        ("pub const FACE_TILES: [[u16; 6]; N_KINDS] = [", "    [65535, 65535, 65535, 65535, 65535, 65535],", "face_tiles row"),
        ("pub const TINTS: [[u32; 6]; N_KINDS] = [", "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],", "tints row"),
        ("pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [", '    "",', "door row"),
        ("pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [", "    [65535, 65535, 65535, 65535, 65535, 65535],", "head faces row"),
        ("pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [", "    65535,", "chest_lock row"),
    ]:
        atlas = append_table_row(atlas, decl, row, OLD_KINDS, NEW_KINDS, tag)

    open(ATLAS, "w").write(atlas)

    # 4. world.rs the NAME_OF row + decl.
    world = open(WORLD).read()
    print("world.rs:")
    world = edit_all(
        world,
        f"pub const NAME_OF: [&str; {OLD_KINDS}] = [",
        f"pub const NAME_OF: [&str; {NEW_KINDS}] = [",
        1,
        "name_of decl",
    )
    world = edit_all(
        world,
        '"yellow_bed",\n];',
        '"yellow_bed",\n    "end_gateway",\n];',
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 5. the sweep (the kind-count sites the verdicts bank).
    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 797,"), 26, "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 797,"), 21, "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*797,"), 8, "kinds multi"),
        (re.compile(r"atlas::N_KINDS, 797,"), 1, "kinds bare"),
    ]
    files = ["crates/mc-client/src/world.rs"] + sorted(
        glob.glob("crates/mc-client/src/screens/tests/*.rs")
    )
    for pat, want, tag in pats:
        total = 0
        done = 0
        for f in files:
            txt = open(f).read()
            n = len(pat.findall(txt))
            if n:
                total += n
                txt = pat.sub(lambda m: m.group(0).replace("797", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            probe = re.compile(pat.pattern.replace("797", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done >= want:
            print(f"  [{tag}] already pinned")
            continue
        if total != want:
            sys.exit(f"  [{tag}] sweep count {total} != {want}")
        print(f"  [{tag}] swept ({want} site(s))")

    print("gateway_pin_310: done")


if __name__ == "__main__":
    main()
