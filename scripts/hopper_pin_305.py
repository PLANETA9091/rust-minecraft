#!/usr/bin/env python3
"""autopush-305 — THE HOPPER'S OWN FIVE-SLOT BANK pin.

The autopush-304 NEXT list's own first candidate: the hopper kind the
container_stores bank the client never seats. THE JAR LAW the live
disasm re-pins (scripts/hopper_scout_305.py the 26.2 jar walk answers):
HopperBlockEntity.<init> the `NonNullList.withSize(5, EMPTY)` the five
slots speak, HopperMenu the checkContainerSize(container, 5) + the
Slot(container, i, 44 + i*18, 20) loop the five-seat row banks,
MenuType.HOPPER the window kind names, HopperBlockEntity.getDefaultName
the container.hopper DEFAULT_NAME row banks, HopperBlock.getStateForPlacement
the `clicked.getOpposite()` + the Y-axis-DOWN fold the facing stamp
answers. The pin:

  atlas.rs      TILE_936..937_RGBA the two hopper masters the jar's
                block/hopper_outside + block/hopper_top the decode
                serves, appended at the POOL TAIL so every existing
                tile index holds (the goldens the same pixels render);
                TILE_NAMES the two's tails; the FACE_TILES the hopper
                row [936, 937, 936, 936, 936, 936] (the down face the
                outside master the funnel's outer skin keeps, the up
                face the rim's own top tile, the four sides the same
                outside the block/hopper model's #side ref banks);
                TINTS the no-tint row; DOOR_MODEL_BY_ROW the not-a-door
                empty row; ENTITY_HEAD_FACES the not-a-head 65535 row;
                CHEST_LOCK_TILES the TILE_NONE row; KIND_NAMES +
                REGISTRY_NAMES the hopper rows; N_KINDS 779 -> 780; the
                TILE_HOPPER_* named consts the tests read.

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
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

OLD_KINDS = 779
NEW_KINDS = 780
OLD_TILES = 936
NEW_TILES = 938

# the pool tail's two: the block/hopper model's own #side/#top refs the
# masters serve (the #inside ref the funnel round's tile seat keeps).
TILES = [
    (936, "block/hopper_outside.png", "hopper_outside", "TILE_HOPPER_OUTSIDE"),
    (937, "block/hopper_top.png", "hopper_top", "TILE_HOPPER_TOP"),
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


def append_table_row(text: str, decl: str, row: str, before: int, tag: str) -> str:
    """The decl-anchored tail append (the ROW-COUNT check the idempotence
    keeps — the identical tail rows the count the pinned state proves:
    the unpinned table the `before` rows wears, the pinned the +1)."""
    start = text.find(decl)
    if start < 0:
        sys.exit(f"  [{tag}] decl anchor missing")
    # the type annotation's own `];` the \n-anchored close skips
    close = text.find("\n];", start)
    body = text[start:close]
    rows = [l for l in body.split("\n") if l.rstrip().endswith(",")]
    if len(rows) == before + 1:
        print(f"  [{tag}] already pinned")
        return text
    if len(rows) != before:
        sys.exit(f"  [{tag}] row count {len(rows)} != {before}")
    print(f"  [{tag}] pinned")
    # close rides the \n before ]; — the row the own line the join wears
    return text[:close] + "\n" + row + text[close:]


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the two tile statics (the jar's own masters decoded).
    if "static TILE_936_RGBA" not in atlas:
        statics = (
            "\n/// THE HOPPER'S OWN MASTERS (the autopush-305 round —\n"
            "    /// the kind mint the two tiles the jar's block/\n"
            "    /// hopper_outside + block/hopper_top the decode\n"
            "    /// serves: the block/hopper model's #side ref the\n"
            "    /// outside master the funnel's outer skin banks, the\n"
            "    /// #top ref the rim's own tile the opening wears,\n"
            "    /// the #inside ref the funnel round's tile seat\n"
            "    /// keeps).\n"
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
        '"weathered_copper_chest_lock", "oxidized_copper_chest_lock"];',
        '"weathered_copper_chest_lock", "oxidized_copper_chest_lock", '
        + ", ".join(f'"{n}"' for _, _, n, _ in TILES)
        + "];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the hopper row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"dispenser",\n    "dropper",\n    "shulker_box"];',
        '"dispenser",\n    "dropper",\n    "shulker_box",\n    "hopper"];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: down=outside(936), up=top(937), the four sides
    # the outside master (the model-face order [down, up, west, north,
    # east, south] the Cube ctor's own polygon law banks).
    atlas = edit_all(
        atlas,
        "    [928, 928, 928, 928, 928, 928],\n];",
        "    [928, 928, 928, 928, 928, 928],\n"
        "    [936, 937, 936, 936, 936, 936],\n];",
        1,
        "face_tiles row",
    )
    # TINTS row: the no-tint law the untinted family keeps. DOOR
    # MODEL BY ROW: the not-a-door empty row. ENTITY_HEAD_FACES: the
    # not-a-head TILE_NONE row. THE THREE the decl-anchored append
    # walk wears (the rows the per-table tail the last-row check
    # pins — the duplicate rows the blind anchor never adds).
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
    # CHEST_LOCK_TILES: the TILE_NONE row (the lock lane never reads
    # — the count check the identical 65535 tail the state proves).
    atlas = append_table_row(
        atlas,
        "pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [",
        "    65535,",
        OLD_KINDS,
        "chest_lock row",
    )

    # 5. the named consts the tests read.
    if "pub const TILE_HOPPER_OUTSIDE:" not in atlas:
        consts = (
            "\n/// THE HOPPER'S OWN NAMED SEATS (the pool tail's pair the\n"
            "/// mesher's generic-cube row the tests read).\n"
            "pub const TILE_HOPPER_OUTSIDE: u16 = 936;\n"
            "pub const TILE_HOPPER_TOP: u16 = 937;\n"
        )
        anchor = "pub const TILE_PISTON_TOP: u16 = 919;"
        atlas = edit_all(atlas, anchor, consts + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)
    print("hopper_pin_305: done")


if __name__ == "__main__":
    main()
