#!/usr/bin/env python3
"""autopush-319 — THE BEACON'S OWN KIND SEATS — the six-seat model lane pin.

The autopush-318 NEXT list's own first candidate: the beacon kind the
client palette never carried (the 318 law layer the is_beacon predicate
a stand-in kind kept — the RESPAWN_ANCHOR seat the tests' own ladder
wore) + the five base blocks the pyramid lane banks. THE JAR FACTS the
scout banks (scripts/jar_work/beacon_kind_scout_319.txt, rc=0): the
block/beacon.json the THREE elements speak (the glass shell
[0,0,0]->[16,16,16] the #glass faces wear, the obsidian base
[2,0.1,2]->[14,3,14] the #obsidian rows keep, the inner core
[3,3,3]->[13,14,13] the #beacon texture serves — the 18 quads the
derive walk banks), the four masters the decode wears (block/beacon +
block/diamond_block + block/emerald_block + block/netherite_block —
16x16 colortype-3 opaque tiles), the glass/obsidian masters the pool's
own rows keep (tiles 273/93 — the shell's alpha the {0,255} cutout map
speaks). The clinit rows the banked dump
scripts/jar_work/blocks_clinit_262.txt pins: Blocks.BEACON the
mapColor(DIAMOND).instrument(HAT).strength(F).lightLevel(..)
.noOcclusion().isRedstoneConductor(..) chain speaks (the noOcclusion
the is_transparent row banks), the five base blocks the
requiresCorrectToolForDrops().strength(FF).sound(METAL/IRON/NETHERITE)
opaque full-cube rows keep (the default is_transparent verdict the
generic path wears).

The pin (the 314 anchor precedent — the POOL TAIL append the every-
existing-index law keeps, the goldens the same pixels render):

  atlas.rs      TILE_1063..1066_RGBA the four masters the jar's
                block/{beacon,diamond_block,emerald_block,
                netherite_block} the decode serves, appended at the
                pool tail; TILE_NAMES the four tails; N_KINDS
                799 -> 805; KIND_NAMES + REGISTRY_NAMES the six rows
                (iron_block, gold_block, diamond_block, emerald_block,
                netherite_block, beacon — the pyramid lane's own
                order the tail banks); the FACE_TILES the six rows
                (the five full-cube rows the own-tile six speak, the
                beacon row the namesake tile the shape law banks —
                the special arm the generic row never reads); the
                TINTS/DOOR_MODEL_BY_ROW/ENTITY_HEAD_FACES/
                CHEST_LOCK_TILES the six no-op rows; the
                TILE_BEACON/TILE_DIAMOND_BLOCK/TILE_EMERALD_BLOCK/
                TILE_NETHERITE_BLOCK named consts the tests read.

  world.rs      NAME_OF the six rows the tail seats wear, the
                799 -> 805 decl keeps; the six kind consts the
                IRON_BLOCK 799 .. BEACON 804 ladder mints (the
                is_beacon predicate the 318 law layer's stand-in
                retires, the BASE_KINDS ladder the pyramid walk
                banks).

  sweep         the kind-count sites (the N_KINDS/NAME_OF.len assert
                rows the count verdicts bank) 799 -> 805.

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

OLD_KINDS = 799
NEW_KINDS = 805
OLD_TILES = 1063
NEW_TILES = 1067

# the pool tail's four: the beacon's own namesake master + the three
# base masters the pool's iron/gold rows already keep (145/144).
TILES = [
    (1063, "block/beacon.png", "beacon", "TILE_BEACON"),
    (1064, "block/diamond_block.png", "diamond_block", "TILE_DIAMOND_BLOCK"),
    (1065, "block/emerald_block.png", "emerald_block", "TILE_EMERALD_BLOCK"),
    (1066, "block/netherite_block.png", "netherite_block", "TILE_NETHERITE_BLOCK"),
]

# the six kind rows (the pyramid lane's own order): the five base
# blocks the value ladder keeps, the beacon the tail seat wears.
KIND_ROWS = [
    ("iron_block", 145),
    ("gold_block", 144),
    ("diamond_block", 1064),
    ("emerald_block", 1065),
    ("netherite_block", 1066),
    ("beacon", 1063),
]


def tile_rgba(rel: str) -> list:
    img = Image.open(os.path.join(TEX, rel)).convert("RGBA")
    if img.size != (16, 16):
        sys.exit(f"{rel}: expected 16x16, got {img.size}")
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


def append_table_row(text: str, decl: str, rows: list, before: int, tag: str) -> str:
    """The decl-anchored tail append (the ROW-COUNT check the idempotence
    keeps — the identical tail rows the count the pinned state proves)."""
    start = text.find(decl)
    if start < 0:
        sys.exit(f"  [{tag}] decl anchor missing")
    close = text.find("\n];", start)
    body = text[start:close]
    # THE REAL-ROW COUNT (the regex the line-shape law reads — the
    # hex-tint rows [0x0, ..] the comma-tail line count miscounts, the
    # element rows [a, b, c, d, e, f] / \"\" / 65535 the shapes the
    # walk covers)
    import re as _re
    body_clean = "\n".join(
        l.split("//")[0].rstrip() for l in body.split("\n")
    )
    existing = _re.findall(
        r"^\s+(?:\[[^\]]*\]|\"[^\"]*\"|\d+)\s*,?\s*$", body_clean, _re.M
    )
    if len(existing) == before + len(rows):
        print(f"  [{tag}] already pinned")
        return text
    if len(existing) != before:
        sys.exit(f"  [{tag}] row count {len(existing)} != {before}")
    print(f"  [{tag}] pinned ({len(rows)} row(s))")
    return text[:close] + "\n" + "\n".join(rows) + text[close:]


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the four tile statics (the jar's own masters decoded).
    if "static TILE_1063_RGBA" not in atlas:
        statics = (
            "\n/// THE BEACON'S OWN MASTERS (the autopush-319 round —\n"
            "    /// the kind mint the four tiles the jar's block/\n"
            "    /// {beacon,diamond_block,emerald_block,netherite_block}\n"
            "    /// the decode serves: the beacon core the model's\n"
            "    /// #beacon ref banks, the three base masters the\n"
            "    /// pyramid lane's own cube_all rows keep [the pool's\n"
            "    /// iron 145 / gold 144 rows the first two seats\n"
            "    /// already wear]).\n"
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
        "&TILE_948_RGBA,];",
        "&TILE_948_RGBA,"
        + ",".join(f"&TILE_{idx}_RGBA" for idx, _, _, _ in TILES)
        + ",];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the four ride.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"respawn_anchor_top_off"];',
        '"respawn_anchor_top_off", '
        + ", ".join(f'"{n}"' for _, _, n, _ in TILES)
        + "];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the rows wear.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"yellow_bed", "end_gateway", "respawn_anchor"];',
        '"yellow_bed", "end_gateway", "respawn_anchor", '
        + ", ".join(f'"{n}"' for n, _ in KIND_ROWS)
        + "];",
        2,
        "kind+registry names",
    )
    # FACE_TILES rows: the five full-cube rows the own-tile six speak,
    # the beacon row the namesake tile the table-shape law banks (the
    # three-element model the mesher's special arm the generic row
    # never reads — the 314 cube_bottom_top row the generic path wore).
    atlas = append_table_row(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        [f"    [{t}, {t}, {t}, {t}, {t}, {t}]," for n, t in KIND_ROWS],
        OLD_KINDS,
        "face_tiles rows",
    )
    atlas = append_table_row(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS] = [",
        ["    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],"] * len(KIND_ROWS),
        OLD_KINDS,
        "tints rows",
    )
    atlas = append_table_row(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [",
        ['    "",'] * len(KIND_ROWS),
        OLD_KINDS,
        "door rows",
    )
    atlas = append_table_row(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [",
        ["    [65535, 65535, 65535, 65535, 65535, 65535],"] * len(KIND_ROWS),
        OLD_KINDS,
        "head faces rows",
    )
    atlas = append_table_row(
        atlas,
        "pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [",
        ["    65535,"] * len(KIND_ROWS),
        OLD_KINDS,
        "chest_lock rows",
    )

    # 5. the named consts the tests read.
    if "pub const TILE_BEACON:" not in atlas:
        consts = (
            "\n/// THE BEACON'S OWN NAMED SEATS (the pool tail's four\n"
            "/// the mesher's model arm + the five-seat pyramid lane's\n"
            "/// next-mesh round the tests read).\n"
            "pub const TILE_BEACON: u16 = 1063;\n"
            "pub const TILE_DIAMOND_BLOCK: u16 = 1064;\n"
            "pub const TILE_EMERALD_BLOCK: u16 = 1065;\n"
            "pub const TILE_NETHERITE_BLOCK: u16 = 1066;\n"
        )
        anchor = "pub const TILE_RESPAWN_ANCHOR_TOP_OFF: u16 = 948;"
        atlas = edit_all(atlas, anchor, anchor + consts, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 6. world.rs the NAME_OF rows + decl + the kind consts.
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
        '    "respawn_anchor",\n];',
        '    "respawn_anchor",\n'
        + "\n".join(f'    "{n}",' for n, _ in KIND_ROWS)
        + "\n];",
        1,
        "name_of tail",
    )
    # the six kind consts the pyramid lane's own ladder mints (the
    # is_beacon predicate the 318 law layer's stand-in retires).
    if "pub const BEACON: u16 = 804;" not in world:
        seat = '''
/// THE PYRAMID'S OWN SIX SEATS (the autopush-319 round — the 318 NEXT
/// list's own first candidate the kind mint wears: the five base
/// blocks the updateBase scan's BEACON_BASE_BLOCKS tag banks [the tag
/// the data/minecraft/tags/block/beacon_base_blocks.json the five
/// members names: netherite, emerald, diamond, gold, iron — the 318
/// round's walk banks] + the beacon the tail seat keeps). The clinit
/// rows the banked dump scripts/jar_work/blocks_clinit_262.txt pins:
/// the five base blocks the requiresCorrectToolForDrops()
/// .strength(FF).sound(METAL/IRON/NETHERITE) opaque full-cube rows
/// keep [GOLD the METAL sound, IRON the IRON_XYLOPHONE instrument +
/// IRON sound, DIAMOND the METAL sound, EMERALD the METAL sound,
/// NETHERITE the NETHERITE_BLOCK sound], the beacon the
/// mapColor(DIAMOND).instrument(HAT).strength(F).lightLevel(..)
/// .noOcclusion().isRedstoneConductor(..) chain speaks — the
/// noOcclusion the is_transparent row banks, the lightLevel 15 the
/// light engine's own lane keeps. The NAME_OF rows the
/// dig/drop/sound/item lanes wire (the generated tables' own rows
/// the mint lights), the FACE_TILES rows the five full cubes the
/// generic path wears + the beacon row the table shape keeps (the
/// three-element model the mesher's special arm the generic row
/// never reads — block/beacon.json the glass shell [0,0,0]->
/// [16,16,16] + the obsidian base [2,0.1,2]->[14,3,14] + the inner
/// core [3,3,3]->[13,14,13] the 18 quads the derive walk banks).
pub const IRON_BLOCK: u16 = 799;
pub const GOLD_BLOCK: u16 = 800;
pub const DIAMOND_BLOCK: u16 = 801;
pub const EMERALD_BLOCK: u16 = 802;
pub const NETHERITE_BLOCK: u16 = 803;
pub const BEACON: u16 = 804;

/// THE BASE LADDER'S OWN ROWS (the vanilla tag order the walk banks —
/// the pyramid scan the is_base predicate the ladder reads).
pub const BEACON_BASE_KINDS: [u16; 5] = [
    IRON_BLOCK, GOLD_BLOCK, DIAMOND_BLOCK, EMERALD_BLOCK, NETHERITE_BLOCK,
];

#[inline]
pub fn is_iron_block(id: u16) -> bool {
    id == IRON_BLOCK
}

#[inline]
pub fn is_gold_block(id: u16) -> bool {
    id == GOLD_BLOCK
}

#[inline]
pub fn is_diamond_block(id: u16) -> bool {
    id == DIAMOND_BLOCK
}

#[inline]
pub fn is_emerald_block(id: u16) -> bool {
    id == EMERALD_BLOCK
}

#[inline]
pub fn is_netherite_block(id: u16) -> bool {
    id == NETHERITE_BLOCK
}

/// THE BEACON BLOCK'S OWN PREDICATE (the 318 law layer's parameter
/// the kind seat answers — the stand-in the RESPAWN_ANCHOR rows the
/// tests wore the mint retires).
#[inline]
pub fn is_beacon(id: u16) -> bool {
    id == BEACON
}

/// THE PYRAMID BASE'S OWN VERDICT (the BEACON_BASE_KINDS ladder the
/// updateBase scan's tag check wears).
#[inline]
pub fn is_beacon_base(id: u16) -> bool {
    BEACON_BASE_KINDS.contains(&id)
}
'''
        anchor = """#[inline]
pub fn anchor_charge_variant(c: u8) -> u8 {
    c.min(4)
}
"""
        world = edit_all(world, anchor, anchor + seat, 1, "kind consts")
    else:
        print("  [kind consts] already pinned")
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank).
    import glob
    import re

    pats = [
        (re.compile(r"N_KINDS, 799"), None, "kinds bare"),
        (re.compile(r"N_KINDS,\s*\n(\s*)799,"), None, "kinds multiline"),
        (re.compile(r"NAME_OF\.len\(\), 799"), None, "name assert"),
        (re.compile(r"NAME_OF\.len\(\),\s*\n\s*799,"), None, "name multiline"),
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
                txt = pat.sub(lambda m: m.group(0).replace("799", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            probe = re.compile(pat.pattern.replace("799", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done > 0:
            print(f"  [{tag}] already pinned")
            continue
        if total == 0 and done == 0:
            print(f"  [{tag}] no sites (the count the tables' own row law carries)")
            continue
        print(f"  [{tag}] swept ({total} site(s))")

    # 8. the verdict (the six rows the tables' own law asserts).
    atlas2 = open(ATLAS).read()
    world2 = open(WORLD).read()
    import re as _re

    kinds = _re.search(r"pub const N_KINDS: usize = (\d+);", atlas2).group(1)
    names = _re.search(r"pub const NAME_OF: \[&str; (\d+)\] = \[", world2).group(1)
    assert kinds == str(NEW_KINDS), f"n_kinds {kinds}"
    assert names == str(NEW_KINDS), f"name_of {names}"
    tail_names = _re.findall(
        r'"yellow_bed", "end_gateway", "respawn_anchor"(.*?)\];', atlas2
    )
    for t in tail_names:
        rows = _re.findall(r'"([^"]+)"', t)
        assert rows == [n for n, _ in KIND_ROWS], f"tail rows {rows}"
    print(f"  [verdict] N_KINDS={kinds} NAME_OF={names} the six rows ride")


if __name__ == "__main__":
    main()
