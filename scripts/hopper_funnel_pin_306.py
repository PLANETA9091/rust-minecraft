#!/usr/bin/env python3
"""autopush-306 — THE HOPPER'S OWN FUNNEL MESH pin (the inside master).

The autopush-305 NEXT list's own first candidate: the hopper's generic
cube the block/hopper.json seven elements the funnel shape retires.
The jar law (assets/minecraft/models/block/hopper.json +
block/hopper_side.json the 26.2 jar serves, the ELEMENTS table the
typed rows carries): the block/hopper model's #inside ref the
block/hopper_outside master the funnel round's third tile seat banks.
The pin:

  atlas.rs      TILE_938_RGBA the jar's block/hopper_inside the decode
                serves, appended at the POOL TAIL so every existing
                tile index holds (the goldens the same pixels render);
                TILE_NAMES the inside tail; the TILE_HOPPER_INSIDE
                named const the tests read. The hopper's FACE_TILES
                row the 305 round's generic-cube row keeps (the kind's
                own fallback the flat lanes wear).

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

OLD_TILES = 938
NEW_TILES = 939

# the pool tail's third: the block/hopper model's #inside ref the
# master serves (the funnel's interior skin the two 305 masters
# complete).
TILES = [
    (938, "block/hopper_inside.png", "hopper_inside", "TILE_HOPPER_INSIDE"),
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


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the inside static (the jar's own master decoded).
    if "static TILE_938_RGBA" not in atlas:
        statics = (
            "\n/// THE HOPPER'S OWN INSIDE MASTER (the autopush-306\n"
            "    /// round — the funnel mesh the third tile the jar's\n"
            "    /// block/hopper_inside the decode serves: the\n"
            "    /// block/hopper model's #inside ref the funnel's\n"
            "    /// interior skin banks — the top plate's down/up pair\n"
            "    /// + the funnel body's down + the nozzle's down the\n"
            "    /// four wears).\n"
            "static TILE_938_RGBA: [u8; 1024] = [\n"
            + fmt_bytes(tile_rgba(TILES[0][1]))
            + "\n];\n\n"
        )
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

    # 3. TILE_NAMES the inside rides.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"hopper_outside", "hopper_top"];',
        '"hopper_outside", "hopper_top", '
        + ", ".join(f'"{n}"' for _, _, n, _ in TILES)
        + "];",
        1,
        "tile_names tail",
    )

    # 4. the named const the tests read (the 305 pair's third). The
    # const's own name the anchor probes — the identical line the
    # pinned state proves (the replace the anchor never removes).
    for idx, _, _, c in TILES:
        line = f"pub const {c}: u16 = {idx};"
        if line in atlas:
            print(f"  [named consts] already pinned")
            continue
        atlas = edit_all(
            atlas,
            "pub const TILE_HOPPER_TOP: u16 = 937;",
            "pub const TILE_HOPPER_TOP: u16 = 937;\n" + line,
            1,
            "named consts",
        )

    open(ATLAS, "w").write(atlas)
    print("hopper_funnel_pin_306: done")


if __name__ == "__main__":
    main()
