#!/usr/bin/env python3
"""autopush-333 — THE ENCHANTING TABLE'S OWN KIND MINT pin.

The 332 NEXT list's own first candidate's foundation: the enchanting
table kind the client never seats — the GUI door + the highlight seat
the same batch wears, the block the mint banks FIRST (the brewing_pin
309 shape the same walk wears). THE JAR LAW the probes bank
(scripts/enchant_menu_probe_333.py + enchant_screen_probe_333.py +
enchant_renderbg_disasm_333.py, rc=0): EnchantmentMenu.<init> the
bipush stream [15, 47, 35, 47, 8, 84] the seats speak — the item slot
(15, 47), the lapis slot (35, 47), addStandardInventorySlots(8, 84)
the 36 mirror cells; the EnchantmentScreen the plain
AbstractContainerScreen extension the subclass absence banks (the
INHERITED imageWidth/imageHeight the 176x166 standard canvas keeps —
NO putfield the ctor never re-seats), the extractBackground the
blit(pipeline, ENCHANTING_TABLE_LOCATION, left, top, 0f, 0f,
imageWidth, imageHeight, 256, 256) the top-left 176x166 region serves
(the PNG's own opaque bbox (0,0)-(175,165) the decode pins — the book
the texture bakes, the extractBook the animated glyphs the offline
canvas never draws), the container.enchant lang row the title wears.

The pin (the brewing_pin_309 shape the idempotent sections keep):

  atlas.rs      TILE_1067..1069_RGBA the three masters the jar's
                block/enchanting_table_bottom + _top + _side the
                decode serves, appended at the POOL TAIL so every
                existing tile index holds (the goldens the same
                pixels render); TILE_NAMES the three's tails; the
                FACE_TILES row [1067, 1068, 1069, 1069, 1069, 1069]
                (down the bottom master, up the top, the four sides
                the side strip — the model-face order [down, up,
                west, north, east, south] the brewing row's law
                keeps); TINTS the no-tint row; DOOR_MODEL_BY_ROW the
                not-a-door empty row; ENTITY_HEAD_FACES the
                not-a-head 65535 row; CHEST_LOCK_TILES the TILE_NONE
                row; KIND_NAMES + REGISTRY_NAMES the enchanting rows;
                N_KINDS 805 -> 806; the TILE_ENCHANTING_* named
                consts the tests read.

  world.rs      NAME_OF the enchanting_table row the tail seat wears,
                the 805 -> 806 decl keeps.

  sweep         the kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 805 -> 806.

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

OLD_KINDS = 805
NEW_KINDS = 806
OLD_TILES = 1067
NEW_TILES = 1070

# the pool tail's three: the block/enchanting_table model's own
# #bottom/#top/#side refs the masters serve.
TILES = [
    (
        1067,
        "block/enchanting_table_bottom.png",
        "enchanting_table_bottom",
        "TILE_ENCHANTING_TABLE_BOTTOM",
    ),
    (
        1068,
        "block/enchanting_table_top.png",
        "enchanting_table_top",
        "TILE_ENCHANTING_TABLE_TOP",
    ),
    (
        1069,
        "block/enchanting_table_side.png",
        "enchanting_table_side",
        "TILE_ENCHANTING_TABLE_SIDE",
    ),
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

    # 1. the three tile statics (the jar's own masters decoded).
    if "static TILE_1067_RGBA" not in atlas:
        statics = (
            "\n/// THE ENCHANTING TABLE'S OWN MASTERS (the autopush-333 round —\n"
            "/// the kind mint the three tiles the jar's block/\n"
            "/// enchanting_table_bottom + _top + _side the decode serves:\n"
            "/// the block/enchanting_table model's own texture refs the\n"
            "/// generic-cube row the faces seat).\n"
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
        "    &TILE_1066_RGBA,\n];",
        "    &TILE_1066_RGBA,\n"
        + "\n".join(f"    &TILE_{idx}_RGBA," for idx, _, _, _ in TILES)
        + "\n];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the three ride.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"netherite_block",\n];',
        '"netherite_block",\n    '
        + ",\n    ".join(f'"{n}"' for _, _, n, _ in TILES)
        + ",\n];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the enchanting row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"netherite_block",\n    "beacon",\n];',
        '"netherite_block",\n    "beacon",\n    "enchanting_table",\n];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: down=bottom(1067), up=top(1068), the four sides
    # the side strip (1069) — the model-face order [down, up, west,
    # north, east, south] the brewing row's law banks.
    atlas = edit_all(
        atlas,
        "    [1063, 1063, 1063, 1063, 1063, 1063],\n];",
        "    [1063, 1063, 1063, 1063, 1063, 1063],\n"
        "    [1067, 1068, 1069, 1069, 1069, 1069],\n];",
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
    # the ITEM count the commas speak (the real tile ids the some
    # rows wear — the 65535-token count the TILE-NONE rows alone
    # spoke, the undercount the packed lines wore)
    body_items = body[body.find("[") + 1 :]
    items = body_items.count(",")
    if items == NEW_KINDS:
        print("  [chest_lock row] already pinned")
    elif items == OLD_KINDS:
        atlas = atlas[:close].rstrip()[:-1].rstrip() + ", 65535," + atlas[close:]
        print("  [chest_lock row] pinned")
    else:
        sys.exit(f"  [chest_lock row] item count {items} != {OLD_KINDS}")

    # 5. the named consts the tests read.
    if "pub const TILE_ENCHANTING_TABLE_BOTTOM:" not in atlas:
        consts = (
            "\n/// THE ENCHANTING TABLE'S OWN NAMED SEATS (the pool tail's\n"
            "/// trio the mesher's generic-cube row the tests read).\n"
            "pub const TILE_ENCHANTING_TABLE_BOTTOM: u16 = 1067;\n"
            "pub const TILE_ENCHANTING_TABLE_TOP: u16 = 1068;\n"
            "pub const TILE_ENCHANTING_TABLE_SIDE: u16 = 1069;\n"
        )
        anchor = "pub const TILE_NETHERITE_BLOCK: u16 = 1066;"
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
        '"netherite_block",\n    "beacon",\n];',
        '"netherite_block",\n    "beacon",\n    "enchanting_table",\n];',
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank — the
    # dynamic count the re-run the already-pinned verdict keeps).
    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 805,"), "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 805,"), "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*805,"), "kinds multi"),
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
                txt = pat.sub(lambda m: m.group(0).replace("805", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            probe = re.compile(pat.pattern.replace("805", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done > 0:
            print(f"  [{tag}] already pinned ({done} site(s))")
            continue
        if total == 0:
            print(f"  [{tag}] ZERO sites — check the pattern")
            continue
        print(f"  [{tag}] swept ({total} site(s))")

    print("enchant_pin_333: done")


if __name__ == "__main__":
    main()
