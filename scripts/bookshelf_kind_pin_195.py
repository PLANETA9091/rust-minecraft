#!/usr/bin/env python3
"""dev-195 — THE BOOKSHELF KIND MINT pin (the enchant_pin_333 shape
the idempotent sections keep).

The spell walk's own foundation: the bookshelf kind the client never
seats — the power scan's provider block the same batch wears. THE JAR
LAW the probes bank (scripts/jar_work/enchant_power_195.txt +
enchant_clinit_195.txt + enchant_isvalid_195.txt + enchant_offsets_195.txt
+ enchant_cost_195.txt + enchant_screen_render_195.txt +
enchant_screen_all_195.txt + enchant_sprites_195.txt, rc=0): the
EnchantingTableBlock clinit the BOOKSHELF_OFFSETS betweenClosedStream
(-2,0,-2, 2,1,2) the lambda$static$0 abs==2 filter keeps, the
isValidBookShelf the provider/transmitter tag pair the midpoint
(x/2, y, z/2) gap answers, the block/bookshelf.json the cube_column
end/side split the FACE_TILES row banks (up/down the oak_planks tile
43, the four sides the bookshelf master).

The pin:

  atlas.rs      TILE_1074_RGBA the jar's block/bookshelf master the
                decode serves, appended at the POOL TAIL so every
                existing tile index holds (the goldens the same
                pixels render); TILE_NAMES the tail; the FACE_TILES
                row [43, 43, 1074, 1074, 1074, 1074] (down/up the
                planks end the cube_column model banks, the four
                sides the bookshelf strip); TINTS the no-tint row;
                DOOR_MODEL_BY_ROW the not-a-door empty row;
                ENTITY_HEAD_FACES the not-a-head 65535 row;
                CHEST_LOCK_TILES the TILE_NONE row; KIND_NAMES +
                REGISTRY_NAMES the bookshelf rows; N_KINDS 806 ->
                807; the TILE_BOOKSHELF named const the tests read.

  world.rs      NAME_OF the bookshelf row the tail seat wears, the
                806 -> 807 decl keeps; the BOOKSHELF kind const +
                the is_bookshelf helper the scan's provider arm and
                the power tests read.

  sweep         the kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 806 -> 807.

Idempotent: every section checks its own anchor first.
"""
import glob
import os
import re
import sys

try:
    from PIL import Image
except ImportError:
    sys.exit("PIL missing — the png decode the tiles need")

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

OLD_KINDS = 806
NEW_KINDS = 807
OLD_TILES = 1074
NEW_TILES = 1075

# the pool tail's one: the block/bookshelf model's own #side ref the
# master serves (the #end the oak_planks tile 43 the pool already
# banks — the cube_column model's own split the FACE_TILES row wears).
TILES = [(1074, "block/bookshelf.png", "bookshelf", "TILE_BOOKSHELF")]


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

    # 1. the tile static (the jar's own master decoded).
    if "static TILE_1074_RGBA" not in atlas:
        statics = (
            "\n/// THE BOOKSHELF'S OWN MASTER (the dev-195 round —\n"
            "/// the kind mint the tile the jar's block/bookshelf the\n"
            "/// decode serves: the block/bookshelf model's own #side\n"
            "/// ref the four sides the FACE_TILES row seats, the #end\n"
            "/// the oak_planks tile 43 the pool already banks).\n"
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
        "    &TILE_1073_RGBA,\n];",
        "    &TILE_1073_RGBA,\n"
        + "\n".join(f"    &TILE_{idx}_RGBA," for idx, _, _, _ in TILES)
        + "\n];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the tail rides.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"enchanting_table_book_pages_right",\n];',
        '"enchanting_table_book_pages_right",\n    '
        + ",\n    ".join(f'"{n}"' for _, _, n, _ in TILES)
        + ",\n];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the bookshelf row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"enchanting_table",\n];',
        '"enchanting_table",\n    "bookshelf",\n];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: down/up the planks end (43), the four sides the
    # bookshelf strip (1074) — the cube_column model's own split the
    # model-face order [down, up, west, north, east, south] banks.
    atlas = edit_all(
        atlas,
        "    [1067, 1068, 1069, 1069, 1069, 1069],\n];",
        "    [1067, 1068, 1069, 1069, 1069, 1069],\n"
        "    [43, 43, 1074, 1074, 1074, 1074],\n];",
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
    # chest_lock: the PACKED table (13 items/line the generator's own
    # wrap wears) — the ITEM count the idempotence keeps, the tail
    # line the +1 item the append serves.
    start = atlas.find("pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [")
    if start < 0:
        sys.exit("  [chest_lock row] decl anchor missing")
    close = atlas.find("\n];", start)
    body = atlas[start:close]
    body_items = body[body.find("[") + 1 :]
    items = body_items.count(",")
    if items == NEW_KINDS:
        print("  [chest_lock row] already pinned")
    elif items == OLD_KINDS:
        atlas = atlas[:close].rstrip()[:-1].rstrip() + ", 65535," + atlas[close:]
        print("  [chest_lock row] pinned")
    else:
        sys.exit(f"  [chest_lock row] item count {items} != {OLD_KINDS}")

    # 5. the named const the tests read.
    if "pub const TILE_BOOKSHELF:" not in atlas:
        consts = (
            "\n/// THE BOOKSHELF'S OWN NAMED SEAT (the pool tail's one\n"
            "/// the mesher's generic-cube row the tests read).\n"
            "pub const TILE_BOOKSHELF: u16 = 1074;\n"
        )
        anchor = "pub const TILE_ENCHANTING_TABLE_BOOK_PAGES_RIGHT: u16 = 1073;"
        atlas = edit_all(atlas, anchor, anchor + consts, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 6. world.rs the NAME_OF row + decl + the kind const + helper.
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
        '"enchanting_table",\n];',
        '"enchanting_table",\n    "bookshelf",\n];',
        1,
        "name_of tail",
    )
    if "pub const BOOKSHELF: u16 = 806;" not in world:
        const_block = (
            "\n/// THE POWER PROVIDER'S OWN KIND (the dev-195 round — the\n"
            "/// spell walk's own foundation: the EnchantingTableBlock\n"
            "/// isValidBookShelf's #enchantment_power_provider tag the\n"
            "/// [minecraft:bookshelf] row the provider arm reads, the\n"
            "/// block/bookshelf.json cube_column model the FACE_TILES\n"
            "/// row banks [the #end the oak_planks tile 43 the down/up\n"
            "/// faces seat, the #side the bookshelf master the four\n"
            "/// horizontals keep]).\n"
            "pub const BOOKSHELF: u16 = 806;\n"
            "\n#[inline]\n"
            "pub fn is_bookshelf(id: u16) -> bool {\n"
            "    id == BOOKSHELF\n"
            "}\n"
        )
        anchor = (
            "pub fn is_enchanting_table(id: u16) -> bool {\n"
            "    id == ENCHANTING_TABLE\n"
            "}\n"
        )
        world = edit_all(world, anchor, anchor + const_block, 1, "kind const")
    else:
        print("  [kind const] already pinned")
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank — the
    # dynamic count the re-run the already-pinned verdict keeps).
    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 806,"), "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 806,"), "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*806,"), "kinds multi"),
    ]
    files = ["crates/mc-client/src/world.rs"] + sorted(
        glob.glob("crates/mc-client/src/screens/tests/*.rs")
    )
    for pat, tag in pats:
        total = 0
        done = 0
        for f in files:
            txt = open(f).read()
            n = len(pat.findall(txt))
            if n:
                total += n
                txt = pat.sub(lambda m: m.group(0).replace("806", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            probe = re.compile(pat.pattern.replace("806", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done > 0:
            print(f"  [{tag}] already pinned ({done} site(s))")
            continue
        if total:
            print(f"  [{tag}] pinned ({total} site(s))")
            continue
        print(f"  [{tag}] no sites (the count the tail append already serves)")

    print("bookshelf kind pin COMPLETE")


if __name__ == "__main__":
    main()
