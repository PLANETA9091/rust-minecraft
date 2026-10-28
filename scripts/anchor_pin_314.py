#!/usr/bin/env python3
"""autopush-314 — THE BED'S OWN RESPAWN ANCHOR KIND pin.

The autopush-313 NEXT list's own first candidate: the respawn anchor
kind the client palette never carried — the 313 walk's honest gap the
round retires (the ANCHOR ARM the static findRespawnAndUseSpawnBlock
the palette's own absence kept). THE JAR LAW the live disasm re-pins
(scripts/anchor_scout{,2,3}_314.py + the anchor4 static-arm probe the
26.2 jar answers, rc=0 — scripts/jar_work/anchor{,3,4}_314.txt the
dumps serve): RespawnAnchorBlock.<clinit> the RESPAWN_ANCHOR_CHARGES
IntegerProperty the RESPAWN_HORIZONTAL_OFFSETS eight rows the
ring@0/ring@-1/ring@+1/above the 25-row RESPAWN_OFFSETS table speaks,
useItemOn the glowstone fuel the canBeCharged gate the charge walk
the consume-quirk (the stack the press consumes REGARDLESS of the
gate — pc 23-27 the un-gated consume wears), useWithoutItem the
charge-0 PASS the canSetSpawn gate the RespawnData.of(dimension, pos,
0.0f, 0.0f) the setRespawnPosition walk the SET_SPAWN voice, the
canSetSpawn-false arm the explode (the removeBlock + the horizontal
would-flow scan the above-water pair the $1 calculator the dampened
boom keeps, the 5.0F power the fire-true the BLOCK interaction the
cp#275 float speaks), findStandUpPosition the 25-row scan the
DismountHelper.findSafeDismountLocation verdict the strict->loose
two-pass fold the bed's own 313 law shares, the static arm the
(forced || charge > 0) gate the scan the consume (charge - 1, the
!forced && param3 pair gates) the RespawnPosAngle the map wears.
The pin:

  atlas.rs      TILE_941..948_RGBA the eight anchor masters the jar's
                block/respawn_anchor_{bottom,side0..4,top,top_off} the
                decode serves (the top the 16x512 flipbook strip the
                FIRST frame the static decode wears), appended at the
                POOL TAIL so every existing tile index holds (the
                goldens the same pixels render); TILE_NAMES the eight
                tails; the FACE_TILES the anchor row [942, 942, 948,
                941, 942, 942] (the atlas order [east, west, up, down,
                south, north] the law keeps — the charge-0 model the
                cube_bottom_top the top_off master the UP seat wears,
                the bottom the DOWN, the side0 the four sides);
                TINTS the no-tint row; DOOR_MODEL_BY_ROW the
                not-a-door empty row; ENTITY_HEAD_FACES the
                not-a-head 65535 row; CHEST_LOCK_TILES the TILE_NONE
                row; KIND_NAMES + REGISTRY_NAMES the anchor rows;
                N_KINDS 798 -> 799; the TILE_RESPAWN_ANCHOR_* named
                consts the tests read.

  world.rs      NAME_OF the respawn_anchor row the tail seat wears,
                the 798 -> 799 decl keeps.

  sweep         the kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 798 -> 799.

Idempotent: every section checks its own anchor first.
"""
import os
import sys

try:
    from PIL import Image
except ImportError:
    sys.exit("PIL missing — the png decode the tiles need")

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

OLD_KINDS = 798
NEW_KINDS = 799
OLD_TILES = 1055
NEW_TILES = 1063

# the pool tail's eight: the cube_bottom_top family's own masters —
# the bottom the DOWN seat, the top_off the charge-0 UP, the side0..4
# the charge ladder's side masters, the top the charged UP (the 16x512
# flipbook the FIRST frame the static decode wears).
TILES = [
    (941, "block/respawn_anchor_bottom.png", "respawn_anchor_bottom", "TILE_RESPAWN_ANCHOR_BOTTOM"),
    (942, "block/respawn_anchor_side0.png", "respawn_anchor_side0", "TILE_RESPAWN_ANCHOR_SIDE0"),
    (943, "block/respawn_anchor_side1.png", "respawn_anchor_side1", "TILE_RESPAWN_ANCHOR_SIDE1"),
    (944, "block/respawn_anchor_side2.png", "respawn_anchor_side2", "TILE_RESPAWN_ANCHOR_SIDE2"),
    (945, "block/respawn_anchor_side3.png", "respawn_anchor_side3", "TILE_RESPAWN_ANCHOR_SIDE3"),
    (946, "block/respawn_anchor_side4.png", "respawn_anchor_side4", "TILE_RESPAWN_ANCHOR_SIDE4"),
    (947, "block/respawn_anchor_top.png", "respawn_anchor_top", "TILE_RESPAWN_ANCHOR_TOP"),
    (948, "block/respawn_anchor_top_off.png", "respawn_anchor_top_off", "TILE_RESPAWN_ANCHOR_TOP_OFF"),
]


def tile_rgba(rel: str) -> list:
    img = Image.open(os.path.join(TEX, rel)).convert("RGBA")
    # THE FLIPBOOK'S FIRST FRAME (the 16x512 strip the animation rows
    # wear — the static decode the first 16x16 window the master
    # serves; the animation lane the BE/later round keeps)
    if img.size != (16, 16):
        assert img.size[0] == 16 and img.size[1] % 16 == 0, (
            f"{rel} the 16-wide strip the tile banks (got {img.size})"
        )
        img = img.crop((0, 0, 16, 16))
    px = img.load()
    tile = [0] * (16 * 16 * 4)
    for y in range(16):
        for x in range(16):
            r, g, b, a = px[x, y]
            i = (y * 16 + x) * 4
            tile[i : i + 4] = [r, g, b, a]
    return tile


def fmt_bytes(data: list) -> str:
    rows = []
    for i in range(0, len(data), 16):
        rows.append("    " + ",".join(str(b) for b in data[i : i + 16]) + ",")
    return "\n".join(rows)


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def append_table_row(text: str, decl: str, row: str, before: int, tag: str) -> str:
    """The decl-anchored tail append (the ROW-COUNT check the idempotence
    keeps — the identical tail rows the count the pinned state proves)."""
    start = text.find(decl)
    if start < 0:
        sys.exit(f"  [{tag}] decl anchor missing")
    close = text.find("\n];", start)
    body = text[start:close]
    rows = [l for l in body.split("\n") if l.rstrip().endswith(",")]
    if len(rows) == before + 1:
        print(f"  [{tag}] already pinned")
        return text
    if len(rows) != before:
        sys.exit(f"  [{tag}] row count {len(rows)} != {before}")
    print(f"  [{tag}] pinned")
    return text[:close] + "\n" + row + text[close:]


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the eight tile statics (the jar's own masters decoded).
    if "static TILE_941_RGBA" not in atlas:
        statics = (
            "\n/// THE RESPAWN ANCHOR'S OWN MASTERS (the autopush-314 round —\n"
            "    /// the kind mint the eight tiles the jar's block/\n"
            "    /// respawn_anchor_{bottom,side0..4,top,top_off} the\n"
            "    /// decode serves: the charge-0 model's #side/#top/#bottom\n"
            "    /// refs the cube_bottom_top family banks, the charged\n"
            "    /// ladder the side1..4 + the top masters keep, the top\n"
            "    /// the 16x512 flipbook strip the FIRST frame the static\n"
            "    /// decode wears).\n"
        )
        for idx, rel, _, _ in TILES:
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(tile_rgba(rel))
                + "\n];\n"
            )
        statics += "\n"
        anchor = f"pub static TILES: [&[u8]; {OLD_TILES}] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 2. the pool itself (the tail refs the every-existing-index law
    # keeps — the goldens the same pixels render).
    atlas = edit_all(
        atlas,
        f"pub static TILES: [&[u8]; {OLD_TILES}] = [",
        f"pub static TILES: [&[u8]; {NEW_TILES}] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        f"&TILE_{OLD_TILES - 1}_RGBA,];",
        f"&TILE_{OLD_TILES - 1}_RGBA,"
        + ",".join(f"&TILE_{idx}_RGBA" for idx, _, _, _ in TILES)
        + ",];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the eight ride.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"yellow_bed_foot_west"];',
        '"yellow_bed_foot_west", '
        + ", ".join(f'"{n}"' for _, _, n, _ in TILES)
        + "];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the anchor row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"yellow_bed", "end_gateway"];',
        '"yellow_bed", "end_gateway", "respawn_anchor"];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: the charge-0 model's own masters — the atlas
    # order [east, west, up, down, south, north] the law keeps (the
    # chest row's front-north slot the same order banks): the four
    # sides the side0 master, the UP the top_off master, the DOWN the
    # bottom master.
    atlas = append_table_row(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [942, 942, 948, 941, 942, 942],",
        OLD_KINDS,
        "face_tiles row",
    )
    atlas = append_table_row(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS] = [",
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],",
        OLD_KINDS,
        "tints row",
    )
    atlas = append_table_row(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [",
        '    "",',
        OLD_KINDS,
        "door row",
    )
    atlas = append_table_row(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [",
        "    [65535, 65535, 65535, 65535, 65535, 65535],",
        OLD_KINDS,
        "head faces row",
    )
    atlas = append_table_row(
        atlas,
        "pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [",
        "    65535,",
        OLD_KINDS,
        "chest_lock row",
    )

    # 5. the named consts the tests read.
    if "pub const TILE_RESPAWN_ANCHOR_BOTTOM:" not in atlas:
        consts = (
            "\n/// THE RESPAWN ANCHOR'S OWN NAMED SEATS (the pool tail's\n"
            "/// eight the mesher's generic-cube row + the charge ladder's\n"
            "/// next-mesh round the tests read).\n"
            "pub const TILE_RESPAWN_ANCHOR_BOTTOM: u16 = 941;\n"
            "pub const TILE_RESPAWN_ANCHOR_SIDE0: u16 = 942;\n"
            "pub const TILE_RESPAWN_ANCHOR_SIDE1: u16 = 943;\n"
            "pub const TILE_RESPAWN_ANCHOR_SIDE2: u16 = 944;\n"
            "pub const TILE_RESPAWN_ANCHOR_SIDE3: u16 = 945;\n"
            "pub const TILE_RESPAWN_ANCHOR_SIDE4: u16 = 946;\n"
            "pub const TILE_RESPAWN_ANCHOR_TOP: u16 = 947;\n"
            "pub const TILE_RESPAWN_ANCHOR_TOP_OFF: u16 = 948;\n"
        )
        anchor = "pub const TILE_RED_BED_FOOT_UP: u16 = 1039;"
        atlas = edit_all(atlas, anchor, anchor + consts, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 6. world.rs the NAME_OF row + decl.
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
        '    "end_gateway",\n];',
        '    "end_gateway",\n    "respawn_anchor",\n];',
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank).
    import glob
    import re

    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 798,"), None, "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 798,"), None, "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*798,"), None, "kinds multi"),
        (re.compile(r"atlas::N_KINDS, 798,"), None, "kinds bare"),
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
                txt = pat.sub(lambda m: m.group(0).replace("798", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            # the already-pinned probe: the same shape the NEW constant
            # wears (the re-run the skip verdict banks)
            probe = re.compile(pat.pattern.replace("798", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done > 0:
            print(f"  [{tag}] already pinned")
            continue
        if total == 0 and done == 0:
            print(f"  [{tag}] no sites (the count the tables' own row law carries)")
            continue
        print(f"  [{tag}] swept ({total} site(s))")

    print("anchor_pin_314: done")


if __name__ == "__main__":
    main()
