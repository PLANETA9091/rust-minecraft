#!/usr/bin/env python3
"""dev-202 — THE ANVIL'S OWN KIND MINT pin.

The 350 NEXT list's own first candidate's foundation: the anvil kind
the client never seats — the GUI door + the repair walk the same batch
wears, the block the mint banks FIRST (the enchant_pin_333 shape the
same walk wears). THE JAR LAW the dumps bank (scripts/jar_work/
anvil_menu_202.txt + anvil_icm_202.txt + anvil_screen_202.txt +
anvil_ics_202.txt + anvil_walk_202.txt, rc=0 — the FIXED-ALIGNMENT
disassembler scripts/dump_method_352.py serves, the 350 tool's
astore/dload/ldc gaps the re-walk repairs): AnvilMenu
.createInputSlotDefinitions the seat stream [27, 47, 76, 47, 134, 47]
the seats speak — input1 (27, 47), input2 (76, 47), the result (134,
47), addStandardInventorySlots(8, 84) the 36 mirror cells; the
ItemCombinerScreen the menuResource blit the 176x166 canvas keeps
(left/top, 0f, 0f, imageWidth, imageHeight, 256, 256), the
textures/gui/container/anvil.png the ANVIL_LOCATION clinit row banks;
the block/anvil model the #body + #top texture refs the generic-cube
row the faces seat.

The pin (the enchant_pin_333 shape the idempotent sections keep):

  atlas.rs      TILE_1075..1076_RGBA the two masters the jar's
                block/anvil + block/anvil_top the decode serves,
                appended at the POOL TAIL so every existing tile index
                holds (the goldens the same pixels render); TILE_NAMES
                the two's tails; the FACE_TILES row [1075, 1076, 1075,
                1075, 1075, 1075] (down the body master, up the top —
                the template_anvil model's own split the model-face
                order [down, up, west, north, east, south] the brewing
                row's law keeps); TINTS the no-tint row;
                DOOR_MODEL_BY_ROW the not-a-door empty row;
                ENTITY_HEAD_FACES the not-a-head 65535 row;
                CHEST_LOCK_TILES the TILE_NONE item; KIND_NAMES +
                REGISTRY_NAMES the anvil rows; N_KINDS 807 -> 808; the
                TILE_ANVIL* named consts the tests read.

  world.rs      NAME_OF the anvil row the tail seat wears, the
                807 -> 808 decl keeps.

  sweep         the kind-count sites (the assert_eq! rows the
                N_KINDS/NAME_OF.len verdicts bank) 807 -> 808.

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

OLD_KINDS = 807
NEW_KINDS = 808
OLD_TILES = 1075
NEW_TILES = 1077

# the pool tail's two: the block/anvil model's own #body/#top refs the
# masters serve.
TILES = [
    (1075, "block/anvil.png", "anvil", "TILE_ANVIL"),
    (1076, "block/anvil_top.png", "anvil_top", "TILE_ANVIL_TOP"),
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
    if "static TILE_1075_RGBA" not in atlas:
        statics = (
            "\n/// THE ANVIL'S OWN MASTERS (the dev-202 round —\n"
            "/// the kind mint the two tiles the jar's block/anvil +\n"
            "/// block/anvil_top the decode serves: the block/anvil\n"
            "/// model's own #body/#top texture refs the generic-cube\n"
            "/// row the faces seat).\n"
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
        "    &TILE_1074_RGBA,\n];",
        "    &TILE_1074_RGBA,\n"
        + "\n".join(f"    &TILE_{idx}_RGBA," for idx, _, _, _ in TILES)
        + "\n];",
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
        '"enchanting_table_book_pages_right",\n    "bookshelf",\n];',
        '"enchanting_table_book_pages_right",\n    "bookshelf",\n    '
        + ",\n    ".join(f'"{n}"' for _, _, n, _ in TILES)
        + ",\n];",
        1,
        "tile_names tail",
    )

    # 4. the six kind-keyed tables the anvil row wears.
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    atlas = edit_all(
        atlas,
        '"enchanting_table",\n    "bookshelf",\n];',
        '"enchanting_table",\n    "bookshelf",\n    "anvil",\n];',
        2,
        "kind+registry names",
    )
    # FACE_TILES row: down=body(1075), up=top(1076), the four sides the
    # body strip — the model-face order [down, up, west, north, east,
    # south] the brewing row's law banks (the template_anvil model's
    # own #body/#top split the sides' rotations the flat row never
    # carries — the cross-section refine the NEXT list keeps).
    atlas = edit_all(
        atlas,
        "    [43, 43, 1074, 1074, 1074, 1074],\n];",
        "    [43, 43, 1074, 1074, 1074, 1074],\n"
        "    [1075, 1076, 1075, 1075, 1075, 1075],\n];",
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
    # item the +1 the append serves.
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

    # 5. the named consts the tests read.
    if "pub const TILE_ANVIL:" not in atlas:
        consts = (
            "\n/// THE ANVIL'S OWN NAMED SEATS (the pool tail's pair the\n"
            "/// mesher's generic-cube row the tests read).\n"
            "pub const TILE_ANVIL: u16 = 1075;\n"
            "pub const TILE_ANVIL_TOP: u16 = 1076;\n"
        )
        anchor = "pub const TILE_BOOKSHELF: u16 = 1074;"
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
        '"enchanting_table",\n    "bookshelf",\n];',
        '"enchanting_table",\n    "bookshelf",\n    "anvil",\n];',
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank — the
    # dynamic count the re-run the already-pinned verdict keeps).
    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 807,"), "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 807,"), "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*807,"), "kinds multi"),
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
                txt = pat.sub(lambda m: m.group(0).replace("807", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            probe = re.compile(pat.pattern.replace("807", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done > 0:
            print(f"  [{tag}] already pinned ({done} site(s))")
            continue
        if total == 0:
            print(f"  [{tag}] ZERO sites — check the pattern")
            continue
        print(f"  [{tag}] swept ({total} site(s))")

    print("anvil_pin_202: done")


if __name__ == "__main__":
    main()
