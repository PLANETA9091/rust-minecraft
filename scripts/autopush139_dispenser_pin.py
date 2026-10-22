#!/usr/bin/env python3
"""autopush-139 — the dispenser/dropper lane's own atlas pin.

The worklog's next-candidates head (the 138 entry): the dropper/
dispenser shot lanes the offline arm never banked. The pin rides the
OFFICIAL 26.2 jar's own bytes the same pool-tail walk the comparator
(124), redstone block (125), repeater (126), observer (127), piston
(128) and moving-piston (129) rounds wore:

  atlas.rs      TILE_924..927_RGBA (block/dispenser_front.png,
                dispenser_front_vertical.png, dropper_front.png,
                dropper_front_vertical.png) decoded from the runtime
                cache, appended at the POOL TAIL so every existing
                tile index holds (the goldens the same pixels
                render); N_KINDS 776 -> 778; KIND_NAMES +
                REGISTRY_NAMES the "dispenser", "dropper" tails (the
                jar's own registry words); FACE_TILES the two rows the
                orientable parent model's own face map banks — the
                jar's block/dispenser.json + block/dropper.json walk:
                side=furnace_side, top=furnace_top (the orientable
                parent's bottom=#top fold the furnace row wears),
                north=the front tile — the TILES the generator's own
                pool already carries (the furnace family's seats),
                only the fronts the pool tail banks; TINTS the zero
                rows, DOOR_MODEL_BY_ROW the empty refs,
                ENTITY_HEAD_FACES the TILE_NONE rows (the per-kind
                tables the N_KINDS length law keeps); TILE_NAMES the
                four's own tails; the TILE_DISPENSER_*/TILE_DROPPER_*
                named consts the mesher's verdict reads.
  elements.rs   the honest no-op — the generator already banked the
                "block/orientable" + "block/orientable_vertical"
                parent rows the dispenser/dropper models the SAME walk
                resolves (the block/dispenser + block/dispenser_
                vertical + block/dropper + block/dropper_vertical rows
                the furnace family's own lane keeps).
  world.rs      NAME_OF the 777th/778th entries (the palette
                resolver's rows; the stems the dig/sound/resistance/
                drops tables key AND the item pairing answers — the
                jar's own loot_table/blocks/{dispenser,dropper}.json +
                Items rows the same words wear); the N_KINDS one-line
                asserts x27 + the NAME_OF.len() asserts x21 the count
                law keeps.
  screens.rs    the N_KINDS asserts x8 the same law keeps.

Idempotent: every section checks its own anchor first (the re-run the
"already pinned" answer keeps).
"""
import os
import re
import sys

try:
    from PIL import Image
except ImportError:
    sys.exit("PIL missing — the png decode the tiles need")

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
ELEMENTS = os.path.join(REPO, "crates/mc-registry/src/generated/elements.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")
CLIENT = os.path.join(REPO, "crates/mc-client/src/screens.rs")
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

# the pool tail order the TILE_NAMES tail keeps — 924..927.
TILES = [
    (924, "block/dispenser_front.png", "dispenser_front"),
    (925, "block/dispenser_front_vertical.png", "dispenser_front_vertical"),
    (926, "block/dropper_front.png", "dropper_front"),
    (927, "block/dropper_front_vertical.png", "dropper_front_vertical"),
]


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def insert_before_close(text: str, decl_anchor: str, row: str, tag: str) -> str:
    start = text.index(decl_anchor)
    close = text.index("\n];", start)
    seg = text[start:close]
    have = seg.count("\n    [") + seg.count('\n    "')
    if have >= 777:
        print(f"  [{tag}] already pinned ({have} rows)")
        return text
    print(f"  [{tag}] pinned (+2 rows)")
    return text[:close] + "\n" + row.rstrip("\n") + text[close:]


def tile_rgba(png_name: str) -> list:
    img = Image.open(os.path.join(TEX, png_name)).convert("RGBA")
    if img.size != (16, 16):
        sys.exit(f"{png_name}: {img.size} != 16x16")
    return list(img.tobytes())


def fmt_bytes(data: list) -> str:
    rows = []
    for i in range(0, len(data), 16):
        rows.append("    " + ",".join(str(b) for b in data[i : i + 16]) + ",")
    return "\n".join(rows)


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. N_KINDS.
    atlas = edit_all(
        atlas,
        "pub const N_KINDS: usize = 776;",
        "pub const N_KINDS: usize = 778;",
        1,
        "n_kinds",
    )

    # 2. the four tile statics (decoded from the jar's own PNGs).
    if "static TILE_924_RGBA" not in atlas:
        statics = (
            "\n/// THE DISPENSER/DROPPER LANE'S OWN TILES (the autopush-139\n"
            "    /// round — the jar's block/dispenser_front.png +\n"
            "    /// _front_vertical + dropper_front + dropper_front_\n"
            "    /// vertical the pool tail wears, the every-existing-\n"
            "    /// index law the goldens keep).\n"
        )
        for idx, png, _ in TILES:
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(tile_rgba(png))
                + "\n];\n"
            )
        statics += "\n"
        anchor = "pub static TILES: [&[u8]; 924] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 3. the pool itself.
    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 924] = [",
        "pub static TILES: [&[u8]; 928] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_923_RGBA,];",
        ",&TILE_923_RGBA,&TILE_924_RGBA,&TILE_925_RGBA,"
        "&TILE_926_RGBA,&TILE_927_RGBA,];",
        1,
        "tiles tail",
    )

    # 4. TILE_NAMES the four ride.
    atlas = edit_all(
        atlas,
        "pub static TILE_NAMES: [&str; 924] = [",
        "pub static TILE_NAMES: [&str; 928] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"piston_top_sticky"];',
        '"piston_top_sticky", "dispenser_front", "dispenser_front_vertical",'
        ' "dropper_front", "dropper_front_vertical"];',
        1,
        "tile_names tail",
    )

    # 5. the kind + registry name tails (the SHARED law the two tables
    # keep — the two registry words the family wears).
    for decl, name in (
        ("pub const KIND_NAMES", "kind tail"),
        ("pub const REGISTRY_NAMES", "registry tail"),
    ):
        start = atlas.index(decl)
        close = atlas.index("];", start)
        seg = atlas[start:close]
        if '"dropper"' in seg:
            print(f"  [{name}] already pinned")
            continue
        want = '    "moving_piston"'
        if want not in seg:
            sys.exit(f"  [{name}] tail anchor missing")
        seg = seg.replace(
            want,
            '    "moving_piston",\n    "dispenser",\n    "dropper"',
            1,
        )
        atlas = atlas[:start] + seg + atlas[close:]
        print(f"  [{name}] pinned")

    # 6-9. the per-kind rows the N_KINDS length law keeps (the two
    # rows the two kinds wear — the orientable parent's own face map:
    # east/west/south=furnace_side(79), up/down=furnace_top(80), the
    # north seat the front tile banks — the blockstate's y-turns the
    # mesher's own travel walk rotates).
    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [79, 79, 80, 80, 79, 924],\n    [79, 79, 80, 80, 79, 926],",
        "face_tiles rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS] = [",
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],\n    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],",
        "tints rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [",
        '    "",\n    "",',
        "door_model rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [",
        "    [65535, 65535, 65535, 65535, 65535, 65535],\n"
        "    [65535, 65535, 65535, 65535, 65535, 65535],",
        "entity_head rows",
    )

    # 10. the named consts the mesher's per-quad verdict reads.
    if "pub const TILE_DISPENSER_FRONT:" not in atlas:
        block = (
            "\n/// THE DISPENSER/DROPPER LANE'S OWN SEATS (the autopush-139\n"
            "/// round — the pool tail the block/dispenser_front +\n"
            "/// _front_vertical + dropper_front + dropper_front_vertical\n"
            "/// textures wear: the mesher's per-quad verdict the consts\n"
            "/// read, the vertical facing's front the *_vertical tiles\n"
            "/// bank).\n"
            "pub const TILE_DISPENSER_FRONT: u16 = 924;\n"
            "\n"
            "/// THE DISPENSER'S VERTICAL-FRONT SEAT (the up/down facing's\n"
            "/// front face's tile — the dispenser_vertical model's own\n"
            "/// #front ref wears).\n"
            "pub const TILE_DISPENSER_FRONT_VERTICAL: u16 = 925;\n"
            "\n"
            "/// THE DROPPER'S FRONT SEAT (the horizontal facing's front\n"
            "/// face's tile).\n"
            "pub const TILE_DROPPER_FRONT: u16 = 926;\n"
            "\n"
            "/// THE DROPPER'S VERTICAL-FRONT SEAT (the up/down facing's\n"
            "/// front face's tile — the dropper_vertical model's own\n"
            "/// #front ref wears).\n"
            "pub const TILE_DROPPER_FRONT_VERTICAL: u16 = 927;\n"
        )
        anchor = "pub const TILE_PISTON_TOP: u16 = 919;"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 11. the elements.rs honest no-op (the generator banked the
    # "block/cube" parent row the orientable chain resolves through —
    # the pure parent refs the furnace family's own lane keeps, the
    # FACE_TILES rows the one-texture-direction walk serves).
    print("elements.rs:")
    elements = open(ELEMENTS).read()
    for row in ('("block/cube", &[',):
        if row in elements:
            print(f"  parent row {row.split(',')[0][1:]} already banked")
        else:
            sys.exit(f"the {row} row the generator never banked")

    # 12. the world.rs NAME_OF rows (the registry names the dig/sound/
    # resistance/drops tables key — the item pairing the two stems the
    # placeable walk answers).
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 776] = [",
        "pub const NAME_OF: [&str; 778] = [",
        1,
        "name_of len",
    )
    seg = world.split("pub const NAME_OF: [&str; 778] = [")
    head, tail = seg[0], seg[1]
    close = tail.index("\n];")
    if '"dropper"' in tail[:close]:
        print("  [name_of tail] already pinned")
    else:
        tail = (
            tail[:close]
            + '\n    "dispenser",\n    "dropper",'
            + tail[close:]
        )
        world = head + "pub const NAME_OF: [&str; 778] = [" + tail
        print("  [name_of tail] pinned")

    # 13. the count law the tests keep: the one-line N_KINDS asserts
    # x27 + the NAME_OF.len() asserts x21.
    world, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)776(,)",
        r"\g<1>778\g<2>",
        world,
    )
    print(f"  [n_kinds asserts world.rs] {n} site(s) bumped")
    world, n = re.subn(
        r"(assert_eq!\(\s*NAME_OF\.len\(\), )776(,)",
        r"\g<1>778\g<2>",
        world,
    )
    print(f"  [name_of len asserts] {n} site(s) bumped")
    open(WORLD, "w").write(world)

    screens = open(CLIENT).read()
    screens, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)776(,)",
        r"\g<1>778\g<2>",
        screens,
    )
    print(f"  [n_kinds asserts screens.rs] {n} site(s) bumped")
    open(CLIENT, "w").write(screens)

    print("autopush139_dispenser_pin: done")


if __name__ == "__main__":
    main()
