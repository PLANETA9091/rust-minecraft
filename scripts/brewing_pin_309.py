#!/usr/bin/env python3
"""autopush-309 — THE BREWING STAND'S OWN FIVE-SLOT BANK pin.

The autopush-308 NEXT list's own first candidate: the brewing stand
kind the container bank the client never seats — the BE's own
withSize(5) the five slots speak (the three bottle seats the
ingredient top the fuel tail the jar law banks). THE JAR LAW the live
disasm re-pins (scripts/brewing_feed_scout_309.py +
brewing_clinit_scout_309.py + the addVanillaMixes walk the 26.2 jar
answers, rc=0): BrewingStandBlockEntity.<init> the
`NonNullList.withSize(5, EMPTY)` the five slots speak, the <clinit>
the SLOTS_FOR_UP [3] / SLOTS_FOR_DOWN [0, 1, 2, 3] / SLOTS_FOR_SIDES
[0, 1, 2, 4] the face tables bank, canPlaceItem the slot-3 isIngredient
the slot-4 BREWING_FUEL tag the bottle trio the potion-family +
empty-slot glass-bottle fold wears, canTakeItemThroughFace the slot-3
the GLASS_BOTTLE take the only refuse keeps. The pin:

  atlas.rs      TILE_939..940_RGBA the two brewing masters the jar's
                block/brewing_stand + block/brewing_stand_base the
                decode serves, appended at the POOL TAIL so every
                existing tile index holds (the goldens the same
                pixels render); TILE_NAMES the two's tails; the
                FACE_TILES the brewing row [940, 939, 939, 939, 939,
                939] (the down face the base master the stand's own
                plate keeps, the up + the four sides the rod strip
                the block/brewing_stand model's #stand ref banks);
                TINTS the no-tint row; DOOR_MODEL_BY_ROW the
                not-a-door empty row; ENTITY_HEAD_FACES the not-a-head
                65535 row; CHEST_LOCK_TILES the TILE_NONE row;
                KIND_NAMES + REGISTRY_NAMES the brewing rows;
                N_KINDS 780 -> 781; the TILE_BREWING_* named consts
                the tests read.

  world.rs      NAME_OF the brewing_stand row the tail seat wears,
                the 780 -> 781 decl keeps.

  sweep         the 56 kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 780 -> 781.

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

OLD_KINDS = 780
NEW_KINDS = 781
OLD_TILES = 939
NEW_TILES = 941

# the pool tail's two: the block/brewing_stand model's own #stand/#base
# refs the masters serve.
TILES = [
    (939, "block/brewing_stand.png", "brewing_stand", "TILE_BREWING_STAND"),
    (940, "block/brewing_stand_base.png", "brewing_stand_base", "TILE_BREWING_STAND_BASE"),
]


def tile_rgba(rel: str) -> list:
    img = Image.open(os.path.join(TEX, rel)).convert("RGBA")
    assert img.size == (16, 16), f"{rel} the 16x16 master the tile banks"
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

    # 1. the two tile statics (the jar's own masters decoded).
    if "static TILE_939_RGBA" not in atlas:
        statics = (
            "\n/// THE BREWING STAND'S OWN MASTERS (the autopush-309 round —\n"
            "    /// the kind mint the two tiles the jar's block/\n"
            "    /// brewing_stand + block/brewing_stand_base the decode\n"
            "    /// serves: the block/brewing_stand model's #stand ref\n"
            "    /// the rod strip banks, the #base ref the stand's own\n"
            "    /// plate keeps).\n"
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
        f",&TILE_{OLD_TILES - 1}_RGBA,];",
        f",&TILE_{OLD_TILES - 1}_RGBA,"
        + ",".join(f"&TILE_{idx}_RGBA" for idx, _, _, _ in TILES)
        + ",];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the two ride.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"hopper_inside"];',
        '"hopper_inside", ' + ", ".join(f'"{n}"' for _, _, n, _ in TILES) + "];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the brewing row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"dispenser",\n    "dropper",\n    "shulker_box",\n    "hopper"];',
        '"dispenser",\n    "dropper",\n    "shulker_box",\n    "hopper",\n    "brewing_stand"];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: down=base(940), up=rod strip(939), the four sides
    # the same rod strip (the model-face order [down, up, west, north,
    # east, south] the Cube ctor's own polygon law banks).
    atlas = edit_all(
        atlas,
        "    [936, 937, 936, 936, 936, 936],\n];",
        "    [936, 937, 936, 936, 936, 936],\n"
        "    [940, 939, 939, 939, 939, 939],\n];",
        1,
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
    if "pub const TILE_BREWING_STAND:" not in atlas:
        consts = (
            "\n/// THE BREWING STAND'S OWN NAMED SEATS (the pool tail's pair\n"
            "/// the mesher's generic-cube row the tests read).\n"
            "pub const TILE_BREWING_STAND: u16 = 939;\n"
            "pub const TILE_BREWING_STAND_BASE: u16 = 940;\n"
        )
        anchor = "pub const TILE_HOPPER_INSIDE: u16 = 938;"
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
        '"shulker_box",\n    "hopper",\n];',
        '"shulker_box",\n    "hopper",\n    "brewing_stand",\n];',
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 7. the sweep (the 56 kind-count sites the verdicts bank).
    import glob
    import re

    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 780,"), 26, "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 780,"), 21, "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*780,"), 8, "kinds multi"),
        (re.compile(r"atlas::N_KINDS, 780,"), 1, "kinds bare"),
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
                txt = pat.sub(lambda m: m.group(0).replace("780", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            # the already-pinned probe: the same shape the NEW constant
            # wears (the re-run the skip verdict banks)
            probe = re.compile(pat.pattern.replace("780", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done >= want:
            print(f"  [{tag}] already pinned")
            continue
        if total != want:
            sys.exit(f"  [{tag}] sweep count {total} != {want}")
        print(f"  [{tag}] swept ({want} site(s))")

    print("brewing_pin_309: done")


if __name__ == "__main__":
    main()
