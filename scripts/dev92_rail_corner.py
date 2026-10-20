#!/usr/bin/env python3
"""dev-92: append the vanilla rail_corner tile to the generated atlas.

The 26.2 jar's textures/block/rail_corner.png (present in
assets-cache/jar — the standing extract) decodes to 16x16 RGBA; the
pool tail seat (index 905) the append wears — every EXISTING index
stays put, the goldens the same pool reads never move. The named const
TILE_RAIL_CORNER the mesher's corner lane reads.

Vanilla anchors (assets-cache/jar/assets/minecraft/):
  blockstates/rail.json: shape=south_east -> rail_corner y=0,
    south_west -> y=90, north_west -> y=180, north_east -> y=270.
  models/block/rail_corner.json: parent rail_curved — the SAME
    zero-thickness [0,1,0]->[16,1,16] element the flat lane draws,
    the texture ref the corner swap carries.
  texture edge touch (decoded): the bend reaches the EAST edge
    (u=15, rows 2..14) and the SOUTH edge (v=15, cols 2..14) — the
    y=0 = south_east verdict the pixels confirm.

Every edit asserts its anchor count == 1; any drift aborts the run
with the file untouched (the atomic batch the dev-87 discipline keeps).
"""
import sys

PATH = "crates/mc-registry/src/generated/atlas.rs"
PNG = "assets-cache/jar/assets/minecraft/textures/block/rail_corner.png"
NEW_IDX = 905


def decode_png_rgba(path: str) -> bytes:
    from PIL import Image
    im = Image.open(path).convert("RGBA")
    assert im.size == (16, 16), f"the 16x16 tile the size law keeps, got {im.size}"
    out = bytearray()
    for y in range(16):
        for x in range(16):
            out.extend(im.getpixel((x, y)))
    assert len(out) == 1024
    return bytes(out)


def render_tile_const(idx: int, rgba: bytes) -> str:
    # the row-major 16-texel lines the generated files wear
    lines = []
    for y in range(16):
        row = rgba[y * 64:(y + 1) * 64]
        texels = [str(b) for b in row]
        lines.append(",".join(texels) + ",")
    body = "\n".join(lines)
    return f"static TILE_{idx}_RGBA: [u8; 1024] = [\n{body}\n];"


def edit(src: str, old: str, new: str, tag: str) -> str:
    n = src.count(old)
    assert n == 1, f"anchor {tag}: count {n} != 1 — abort"
    print(f"  ok {tag}")
    return src.replace(old, new, 1)


def main() -> None:
    rgba = decode_png_rgba(PNG)
    with open(PATH, "r", encoding="utf-8") as f:
        src = f.read()
    if f"TILE_{NEW_IDX}_RGBA" in src:
        print("already appended — nothing to do")
        return

    # E1: the pixel array slots in BEFORE the pool comment; the pool's
    # own count the same edit bumps (existing indices stay stable).
    anchor_pool = ("\n\n/// RGBA8 texel data, one entry per tile "
                   "(16x16x4 bytes).\npub static TILES: [&[u8]; 905]")
    src = edit(
        src,
        anchor_pool,
        "\n\n" + render_tile_const(NEW_IDX, rgba) +
        "\n\n/// RGBA8 texel data, one entry per tile "
        "(16x16x4 bytes).\npub static TILES: [&[u8]; 906]",
        "E1 tile array + pool count",
    )

    # E2: the pool tail carries the new seat.
    src = edit(
        src,
        ",&TILE_904_RGBA,];",
        ",&TILE_904_RGBA,&TILE_905_RGBA,];",
        "E2 pool tail",
    )

    # E3: the name table grows with the pool.
    src = edit(
        src,
        "pub static TILE_NAMES: [&str; 905]",
        "pub static TILE_NAMES: [&str; 906]",
        "E3a names count",
    )
    src = edit(
        src,
        '"lava_flow_frame15"];',
        '"lava_flow_frame15", "rail_corner"];',
        "E3b names tail",
    )

    # E4: the named seat the mesher's corner lane reads (the named
    # block's own doc rhythm keeps).
    anchor_named = "     pub const TILE_FARMLAND_MOIST: u16 = 406;"
    named = (
        "\n\n/// THE RAIL'S OWN CORNER SEAT (the dev-92 round — the vanilla\n"
        "     /// blockstates/rail.json's corner rows the model rotation the\n"
        "     /// seat resolves: the rail_corner texture the SE/SW/NW/NE\n"
        "     /// bytes the y=0/90/180/270 arms wear, the pool tail the\n"
        "     /// index banks — the mesher's curve lane the const reads).\n"
        f"     pub const TILE_RAIL_CORNER: u16 = {NEW_IDX};"
    )
    src = edit(src, anchor_named, anchor_named + named, "E4 named seat")

    with open(PATH, "w", encoding="utf-8") as f:
        f.write(src)
    print(f"appended TILE_{NEW_IDX}_RGBA + rail_corner seat ({len(rgba)} texel bytes)")


if __name__ == "__main__":
    sys.exit(main())
