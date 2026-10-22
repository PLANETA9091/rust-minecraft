#!/usr/bin/env python3
"""autopush-127 — the observer's own atlas pin.

The DirectionalBlock sibling the worklog's next-candidates named: the
pin rides the OFFICIAL 26.2 jar's own bytes the same pool-tail walk the
comparator (124), redstone block (125) and repeater (126) rounds wore:

  atlas.rs      TILE_914..918_RGBA (block/observer_top.png, _side,
                _front, _back, _back_on) decoded from the runtime
                cache, appended at the POOL TAIL so every existing
                tile index holds (the goldens the same pixels render);
                N_KINDS 771 -> 772; KIND_NAMES the "observer" tail +
                REGISTRY_NAMES the same tail (the jar's own registry
                word the observer wears — the item name answers);
                FACE_TILES the [916;6] row (the model's own particle
                texture the front tile wears), TINTS the zero row,
                DOOR_MODEL_BY_ROW the empty ref, ENTITY_HEAD_FACES the
                TILE_NONE row (the per-kind tables the N_KINDS length
                law keeps); TILE_NAMES the five's own tails; the
                TILE_OBSERVER_* named consts the mesher's verdict
                reads.
  elements.rs   the honest no-op — the generator already banked the
                "block/observer" full-cube row (the one bake the
                mesher's lane derives; the powered swap the per-quad
                verdict's own #bottom-seat answers, the observer_on
                parent-override the verdict never needs).
  world.rs      NAME_OF the 772nd entry (the palette resolver's row;
                the "observer" stem the dig/sound/resistance/drops
                tables key AND the item pairing answers — the jar's
                own loot_table/blocks/observer.json + Items rows the
                same word wear).

Idempotent: every section checks its own anchor first (the re-run the
"already pinned" answer keeps).
"""
import os
import sys

try:
    from PIL import Image
except ImportError:
    sys.exit("PIL missing — the png decode the tiles need")

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
ELEMENTS = os.path.join(REPO, "crates/mc-registry/src/generated/elements.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

# the pool tail order the TILE_NAMES tail keeps — 914..918.
TILES = [
    (914, "block/observer_top.png", "observer_top"),
    (915, "block/observer_side.png", "observer_side"),
    (916, "block/observer_front.png", "observer_front"),
    (917, "block/observer_back.png", "observer_back"),
    (918, "block/observer_back_on.png", "observer_back_on"),
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
    if have >= 772:
        print(f"  [{tag}] already pinned ({have} rows)")
        return text
    print(f"  [{tag}] pinned ({have} -> {have + 1} rows)")
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
        "pub const N_KINDS: usize = 771;",
        "pub const N_KINDS: usize = 772;",
        1,
        "n_kinds",
    )

    # 2. the five tile statics (decoded from the jar's own PNGs).
    if "static TILE_914_RGBA" not in atlas:
        statics = (
            "\n/// THE OBSERVER'S OWN TILES (the autopush-127 round — the\n"
            "    /// jar's block/observer_top.png + _side + _front + _back\n"
            "    /// + _back_on the pool tail wears, the every-existing-\n"
            "    /// index law the goldens keep).\n"
        )
        for idx, png, _ in TILES:
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(tile_rgba(png))
                + "\n];\n"
            )
        statics += "\n"
        anchor = "pub static TILES: [&[u8]; 914] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 3. the pool itself.
    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 914] = [",
        "pub static TILES: [&[u8]; 919] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_912_RGBA,&TILE_913_RGBA,];",
        ",&TILE_912_RGBA,&TILE_913_RGBA,&TILE_914_RGBA,&TILE_915_RGBA,"
        "&TILE_916_RGBA,&TILE_917_RGBA,&TILE_918_RGBA,];",
        1,
        "tiles tail",
    )

    # 4. TILE_NAMES the five ride.
    atlas = edit_all(
        atlas,
        "pub static TILE_NAMES: [&str; 914] = [",
        "pub static TILE_NAMES: [&str; 919] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"repeater", "repeater_on"];',
        '"repeater", "repeater_on", "observer_top", "observer_side",'
        ' "observer_front", "observer_back", "observer_back_on"];',
        1,
        "tile_names tail",
    )

    # 5. the kind + registry name tails (the SHARED law the two tables
    # keep — the "observer" stem both wear: the jar's own registry the
    # Blocks register speaks, the item word the same answer keeps).
    for decl, name in (
        ("pub const KIND_NAMES", "kind tail"),
        ("pub const REGISTRY_NAMES", "registry tail"),
    ):
        start = atlas.index(decl)
        close = atlas.index("];", start)
        seg = atlas[start:close]
        if '"observer"' in seg:
            print(f"  [{name}] already pinned")
            continue
        want = '    "repeater"'
        if want not in seg:
            sys.exit(f"  [{name}] tail anchor missing")
        seg = seg.replace(want, f'{want},\n    "observer"', 1)
        atlas = atlas[:start] + seg + atlas[close:]
        print(f"  [{name}] pinned")

    # 6-9. the per-kind rows the N_KINDS length law keeps.
    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [916, 916, 916, 916, 916, 916],",
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

    # 10. the named consts the mesher's per-quad verdict reads.
    if "pub const TILE_OBSERVER_TOP:" not in atlas:
        block = (
            "\n/// THE OBSERVER'S OWN SEATS (the autopush-127 round — the\n"
            "     /// pool tail the block/observer_top + _side + _front +\n"
            "     /// _back + _back_on textures wear: the mesher's per-quad\n"
            "     /// texture verdict the consts read, the #bottom swap the\n"
            "     /// powered bit's own seat banks, the eye face the #front\n"
            "     /// tile the facing byte answers).\n"
            "     pub const TILE_OBSERVER_TOP: u16 = 914;\n"
            "\n"
            "/// THE OBSERVER'S SIDE SEAT (the four flank faces' tile).\n"
            "     pub const TILE_OBSERVER_SIDE: u16 = 915;\n"
            "\n"
            "/// THE OBSERVER'S EYE SEAT (the FACING face's own tile).\n"
            "     pub const TILE_OBSERVER_FRONT: u16 = 916;\n"
            "\n"
            "/// THE OBSERVER'S BACK SEAT (the output face's own tile).\n"
            "     pub const TILE_OBSERVER_BACK: u16 = 917;\n"
            "\n"
            "/// THE OBSERVER'S ON BACK SEAT (the POWERED bit's own lit\n"
            "     /// tile — the observer_on parent's own override the\n"
            "     /// verdict's #bottom seat swaps).\n"
            "     pub const TILE_OBSERVER_BACK_ON: u16 = 918;\n"
        )
        anchor = "\n/// The campfire lane's own campfire_log tile"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 11. the elements.rs honest no-op (the generator banked the
    # "block/observer" full-cube row the one bake derives).
    print("elements.rs:")
    elements = open(ELEMENTS).read()
    if '("block/observer", &[' in elements:
        print("  [model row] already banked (the honest no-op keeps)")
    else:
        sys.exit("the block/observer row the generator never banked")

    # 12. the world.rs NAME_OF row (the registry name the dig/sound/
    # resistance tables key — the item pairing the "observer" arm the
    # placeable walk answers).
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 771] = [",
        "pub const NAME_OF: [&str; 772] = [",
        1,
        "name_of len",
    )
    seg = world.split("pub const NAME_OF: [&str; 772] = [")
    head, tail = seg[0], seg[1]
    close = tail.index("\n];")
    if '"observer"' in tail[:close]:
        print("  [name_of tail] already pinned")
    else:
        tail = tail[:close] + '\n    "observer",' + tail[close:]
        world = head + "pub const NAME_OF: [&str; 772] = [" + tail
        print("  [name_of tail] pinned")
    open(WORLD, "w").write(world)

    print("autopush127_observer_pin: done")


if __name__ == "__main__":
    main()
