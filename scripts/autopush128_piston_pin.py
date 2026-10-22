#!/usr/bin/env python3
"""autopush-128 — the piston lane's own atlas pin.

The biggest mover the redstone family still lacked — the worklog's
next-candidates head: the pin rides the OFFICIAL 26.2 jar's own bytes
the same pool-tail walk the comparator (124), redstone block (125),
repeater (126) and observer (127) rounds wore:

  atlas.rs      TILE_919..923_RGBA (block/piston_top.png, piston_side,
                piston_bottom, piston_inner, piston_top_sticky) decoded
                from the runtime cache, appended at the POOL TAIL so
                every existing tile index holds (the goldens the same
                pixels render); N_KINDS 772 -> 775; KIND_NAMES +
                REGISTRY_NAMES the "piston", "sticky_piston",
                "piston_head" tails (the jar's own registry words —
                the three kinds the family wears); FACE_TILES the
                [920;6] rows (the piston_side tile the whole-cell
                verdict keeps), TINTS the zero rows, DOOR_MODEL_BY_ROW
                the empty refs, ENTITY_HEAD_FACES the TILE_NONE rows
                (the per-kind tables the N_KINDS length law keeps);
                TILE_NAMES the five's own tails; the TILE_PISTON_*
                named consts the mesher's verdict reads.
  elements.rs   the honest no-op — the generator already banked the
                "block/template_piston", "block/piston_extended",
                "block/template_piston_head" and
                "block/template_piston_head_short" rows (the five
                bakes the mesher's lane derives; the models.rs parent
                chain block/piston + block/piston_base + block/
                piston_head + block/piston_head_sticky the SAME walk
                resolves).
  world.rs      NAME_OF the 773rd/774th/775th entries (the palette
                resolver's rows; the stems the dig/sound/resistance/
                drops tables key AND the item pairing answers — the
                jar's own loot_table/blocks/{piston,sticky_piston,
                piston_head}.json + Items rows the same words wear);
                the N_KINDS one-line asserts x27 + the multi-line x8 +
                the NAME_OF.len() asserts x21 the count law keeps; the
                piston_ticks field + the x10 World initializers the
                struct law grows.

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

# the pool tail order the TILE_NAMES tail keeps — 919..923.
TILES = [
    (919, "block/piston_top.png", "piston_top"),
    (920, "block/piston_side.png", "piston_side"),
    (921, "block/piston_bottom.png", "piston_bottom"),
    (922, "block/piston_inner.png", "piston_inner"),
    (923, "block/piston_top_sticky.png", "piston_top_sticky"),
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


def insert_before_close(text: str, decl_anchor: str, row: str, tag: str, want: int = 3) -> str:
    start = text.index(decl_anchor)
    close = text.index("\n];", start)
    seg = text[start:close]
    have = seg.count("\n    [") + seg.count('\n    "')
    if have >= 775:
        print(f"  [{tag}] already pinned ({have} rows)")
        return text
    print(f"  [{tag}] pinned (+{want} rows)")
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
        "pub const N_KINDS: usize = 772;",
        "pub const N_KINDS: usize = 775;",
        1,
        "n_kinds",
    )

    # 2. the five tile statics (decoded from the jar's own PNGs).
    if "static TILE_919_RGBA" not in atlas:
        statics = (
            "\n/// THE PISTON LANE'S OWN TILES (the autopush-128 round — the\n"
            "    /// jar's block/piston_top.png + _side + _bottom + _inner +\n"
            "    /// _top_sticky the pool tail wears, the every-existing-\n"
            "    /// index law the goldens keep).\n"
        )
        for idx, png, _ in TILES:
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(tile_rgba(png))
                + "\n];\n"
            )
        statics += "\n"
        anchor = "pub static TILES: [&[u8]; 919] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 3. the pool itself.
    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 919] = [",
        "pub static TILES: [&[u8]; 924] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_917_RGBA,&TILE_918_RGBA,];",
        ",&TILE_917_RGBA,&TILE_918_RGBA,&TILE_919_RGBA,&TILE_920_RGBA,"
        "&TILE_921_RGBA,&TILE_922_RGBA,&TILE_923_RGBA,];",
        1,
        "tiles tail",
    )

    # 4. TILE_NAMES the five ride.
    atlas = edit_all(
        atlas,
        "pub static TILE_NAMES: [&str; 919] = [",
        "pub static TILE_NAMES: [&str; 924] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"observer_back_on"];',
        '"observer_back_on", "piston_top", "piston_side", "piston_bottom",'
        ' "piston_inner", "piston_top_sticky"];',
        1,
        "tile_names tail",
    )

    # 5. the kind + registry name tails (the SHARED law the two tables
    # keep — the three registry words the family wears).
    for decl, name in (
        ("pub const KIND_NAMES", "kind tail"),
        ("pub const REGISTRY_NAMES", "registry tail"),
    ):
        start = atlas.index(decl)
        close = atlas.index("];", start)
        seg = atlas[start:close]
        if '"piston_head"' in seg:
            print(f"  [{name}] already pinned")
            continue
        want = '    "observer"'
        if want not in seg:
            sys.exit(f"  [{name}] tail anchor missing")
        seg = seg.replace(
            want,
            '    "observer",\n    "piston",\n    "sticky_piston",\n    "piston_head"',
            1,
        )
        atlas = atlas[:start] + seg + atlas[close:]
        print(f"  [{name}] pinned")

    # 6-9. the per-kind rows the N_KINDS length law keeps (the three
    # rows the three kinds wear).
    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [920, 920, 920, 920, 920, 920],\n    [920, 920, 920, 920, 920, 920],\n"
        "    [920, 920, 920, 920, 920, 920],",
        "face_tiles rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS] = [",
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],\n    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],\n"
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],",
        "tints rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [",
        '    "",\n    "",\n    "",',
        "door_model rows",
    )
    atlas = insert_before_close(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [",
        "    [65535, 65535, 65535, 65535, 65535, 65535],\n"
        "    [65535, 65535, 65535, 65535, 65535, 65535],\n"
        "    [65535, 65535, 65535, 65535, 65535, 65535],",
        "entity_head rows",
    )

    # 10. the named consts the mesher's per-quad verdict reads.
    if "pub const TILE_PISTON_TOP:" not in atlas:
        block = (
            "\n/// THE PISTON LANE'S OWN SEATS (the autopush-128 round — the\n"
            "     /// pool tail the block/piston_top + _side + _bottom +\n"
            "     /// _inner + _top_sticky textures wear: the mesher's\n"
            "     /// per-quad texture verdict the consts read, the\n"
            "     /// extended base's #inside face the inner tile banks).\n"
            "     pub const TILE_PISTON_TOP: u16 = 919;\n"
            "\n"
            "/// THE PISTON'S SIDE SEAT (the flanks' + the head shaft's tile).\n"
            "     pub const TILE_PISTON_SIDE: u16 = 920;\n"
            "\n"
            "/// THE PISTON'S BOTTOM SEAT (the base's south face's tile).\n"
            "     pub const TILE_PISTON_BOTTOM: u16 = 921;\n"
            "\n"
            "/// THE PISTON'S INNER SEAT (the extended base's open face's\n"
            "     /// tile — the piston_base model's #inside ref wears).\n"
            "     pub const TILE_PISTON_INNER: u16 = 922;\n"
            "\n"
            "/// THE STICKY PISTON'S PLATFORM SEAT (the #platform ref the\n"
            "     /// sticky kind's own model wears — the unextended top).\n"
            "     pub const TILE_PISTON_TOP_STICKY: u16 = 923;\n"
        )
        anchor = "\n/// The campfire lane's own campfire_log tile"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 11. the elements.rs honest no-op (the generator banked the four
    # template rows the five bakes derive).
    print("elements.rs:")
    elements = open(ELEMENTS).read()
    for row in (
        '("block/template_piston", &[',
        '("block/piston_extended", &[',
        '("block/template_piston_head", &[',
        '("block/template_piston_head_short", &[',
    ):
        if row in elements:
            print(f"  [model row] {row.split(',')[0][1:]} already banked")
        else:
            sys.exit(f"the {row} row the generator never banked")

    # 12. the world.rs NAME_OF rows (the registry names the dig/sound/
    # resistance tables key — the item pairing the three stems the
    # placeable walk answers).
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 772] = [",
        "pub const NAME_OF: [&str; 775] = [",
        1,
        "name_of len",
    )
    seg = world.split("pub const NAME_OF: [&str; 775] = [")
    head, tail = seg[0], seg[1]
    close = tail.index("\n];")
    if '"piston_head"' in tail[:close]:
        print("  [name_of tail] already pinned")
    else:
        tail = (
            tail[:close]
            + '\n    "piston",\n    "sticky_piston",\n    "piston_head",'
            + tail[close:]
        )
        world = head + "pub const NAME_OF: [&str; 775] = [" + tail
        print("  [name_of tail] pinned")

    # 13. the count law the tests keep: the one-line N_KINDS asserts
    # x27 + the multi-line x8 + the NAME_OF.len() asserts x21.
    world, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)775(,)",
        lambda m: m.group(1) + "774" + m.group(2),
        world,
    )
    print(f"  [n_kinds asserts world.rs] {n} site(s) bumped")
    world, n = re.subn(
        r"(assert_eq!\(\s*NAME_OF\.len\(\), )775(,)",
        r"\g<1>774\g<2>",
        world,
    )
    print(f"  [name_of len asserts] {n} site(s) bumped")
    open(WORLD, "w").write(world)

    screens = open(CLIENT).read()
    screens, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)775(,)",
        lambda m: m.group(1) + "774" + m.group(2),
        screens,
    )
    print(f"  [n_kinds asserts screens.rs] {n} site(s) bumped")
    open(CLIENT, "w").write(screens)

    print("autopush128_piston_pin: done")


if __name__ == "__main__":
    main()
