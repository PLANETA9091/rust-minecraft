#!/usr/bin/env python3
"""autopush-125 — the redstone block's own kind pin.

The source block the comparator's side walk and the wire flood have
been waiting for. The registry tables (block_dig, block_resistances,
block_sounds, items) are NAME-keyed across the whole 1196-block
vanilla registry — the "redstone_block" rows they already carry — so
the pin rides the ATLAS alone:

  atlas.rs      TILE_911_RGBA (block/redstone_block.png) decoded from
                the runtime cache, appended at the POOL TAIL (every
                existing index holds — the goldens the same pixels
                render); TILES 912; N_KINDS 769 -> 770; KIND_NAMES +
                REGISTRY_NAMES + TILE_NAMES the "redstone_block"
                tails; FACE_TILES the [911;6] row (the full cube the
                default walk renders), TINTS the zero row,
                DOOR_MODEL_BY_ROW the empty ref, ENTITY_HEAD_FACES
                the TILE_NONE row.
  world.rs      NAME_OF the 770th entry (the palette resolver's row
                — the item lookup the placement lane reads).

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
    if have >= 770:
        print(f"  [{tag}] already pinned ({have} rows)")
        return text
    print(f"  [{tag}] pinned ({have} -> {have + 1} rows)")
    return text[:close] + "\n" + row.rstrip("\n") + text[close:]


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    atlas = edit_all(
        atlas,
        "pub const N_KINDS: usize = 769;",
        "pub const N_KINDS: usize = 770;",
        1,
        "n_kinds",
    )

    if "static TILE_911_RGBA" not in atlas:
        img = Image.open(os.path.join(TEX, "block/redstone_block.png")).convert("RGBA")
        if img.size != (16, 16):
            sys.exit(f"redstone_block.png: {img.size} != 16x16")
        data = list(img.tobytes())
        rows = []
        for i in range(0, len(data), 16):
            rows.append("    " + ",".join(str(b) for b in data[i : i + 16]) + ",")
        statics = (
            "\n/// THE REDSTONE BLOCK'S OWN TILE (the autopush-125 round —\n"
            "    /// the jar's block/redstone_block.png the pool tail wears,\n"
            "    /// the every-existing-index law the goldens keep).\n"
            "static TILE_911_RGBA: [u8; 1024] = [\n" + "\n".join(rows) + "\n];\n\n"
        )
        anchor = "pub static TILES: [&[u8]; 911] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile static")
    else:
        print("  [tile static] already pinned")

    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 911] = [",
        "pub static TILES: [&[u8]; 912] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_910_RGBA,];",
        ",&TILE_910_RGBA,&TILE_911_RGBA,];",
        1,
        "tiles tail",
    )

    atlas = edit_all(
        atlas,
        'pub static TILE_NAMES: [&str; 911] = [',
        'pub static TILE_NAMES: [&str; 912] = [',
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"comparator", "comparator_on"];',
        '"comparator", "comparator_on", "redstone_block"];',
        1,
        "tile_names tail",
    )

    atlas = edit_all(
        atlas,
        '"comparator"]',
        '"comparator",\n    "redstone_block"]',
        2,
        "kind/registry tails",
    )

    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [911, 911, 911, 911, 911, 911],",
        "face_tiles row",
    )
    atlas = insert_before_close(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS] = [",
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],",
        "tints row",
    )
    atlas = insert_before_close(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [",
        '    "",',
        "door_model row",
    )
    atlas = insert_before_close(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [",
        "    [65535, 65535, 65535, 65535, 65535, 65535],",
        "entity_head row",
    )

    open(ATLAS, "w").write(atlas)

    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 769] = [",
        "pub const NAME_OF: [&str; 770] = [",
        1,
        "name_of len",
    )
    if '"redstone_block"' in world.split("pub const NAME_OF")[1].split("\n];")[0]:
        print("  [name_of tail] already pinned")
    else:
        seg = world.split("pub const NAME_OF: [&str; 770] = [")
        head, tail = seg[0], seg[1]
        close = tail.index("\n];")
        tail = tail[:close] + '\n    "redstone_block",' + tail[close:]
        world = head + "pub const NAME_OF: [&str; 770] = [" + tail
        print("  [name_of tail] pinned")
    open(WORLD, "w").write(world)

    print("autopush125_redstone_block_pin: done")


if __name__ == "__main__":
    main()
