#!/usr/bin/env python3
"""dev-98: append the vanilla detector_rail_on tile to the generated atlas.

The 26.2 jar's textures/block/detector_rail_on.png (present in
assets-cache/jar — the standing extract) decodes to 16x16 RGBA; the
pool tail seat (index 908) the append wears — every EXISTING index
stays put, the goldens the same pool reads never move. The named const
TILE_DETECTOR_RAIL_ON the mesher's powered swap lane reads — the
dev-96/97 rounds' own twin the detector kind serves (the MESHER side
only: the detector's WORLD law is the cart-detection its own lane
waits for, the neighbor-signal verdict its bit never arms).

Vanilla anchors (assets-cache/jar/assets/minecraft/):
  blockstates/detector_rail.json: powered=true,shape=north_south ->
    detector_rail_on y=0, east_west -> detector_rail_on y=90; the
    ascending arms the detector_rail_on_raised_ne/sw models speak.
  models/block/detector_rail_on.json: parent rail_flat — the SAME
    zero-thickness [0,1,0]->[16,1,16] element the flat lane draws,
    the texture ref the powered swap carries.

Every edit asserts its anchor count == 1; any drift aborts the run
with the file untouched (the atomic batch the dev-87 discipline keeps).
"""
import sys

PATH = "crates/mc-registry/src/generated/atlas.rs"
PNG = "assets-cache/jar/assets/minecraft/textures/block/detector_rail_on.png"
NEW_IDX = 908


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
                   "(16x16x4 bytes).\npub static TILES: [&[u8]; 908]")
    src = edit(
        src,
        anchor_pool,
        "\n\n" + render_tile_const(NEW_IDX, rgba) +
        "\n\n/// RGBA8 texel data, one entry per tile "
        "(16x16x4 bytes).\npub static TILES: [&[u8]; 909]",
        "E1 tile array + pool count",
    )

    # E2: the pool tail carries the new seat.
    src = edit(
        src,
        ",&TILE_907_RGBA,];",
        ",&TILE_907_RGBA,&TILE_908_RGBA,];",
        "E2 pool tail",
    )

    # E3: the name table grows with the pool.
    src = edit(
        src,
        "pub static TILE_NAMES: [&str; 908]",
        "pub static TILE_NAMES: [&str; 909]",
        "E3a names count",
    )
    src = edit(
        src,
        '"rail_corner", "powered_rail_on", "activator_rail_on"];',
        '"rail_corner", "powered_rail_on", "activator_rail_on", '
        '"detector_rail_on"];',
        "E3b names tail",
    )

    # E4: the named seat the mesher's powered lane reads (the named
    # block's own doc rhythm keeps).
    anchor_named = "     pub const TILE_ACTIVATOR_RAIL_ON: u16 = 907;"
    named = (
        "\n\n/// THE DETECTOR RAIL'S OWN ON SEAT (the dev-98 round — the\n"
        "     /// vanilla blockstates/detector_rail.json's powered=true rows\n"
        "     /// the model swap the seat resolves: the detector_rail_on\n"
        "     /// texture the POWERED bit's mesher lane wears, the\n"
        "     /// pool tail the index banks — the world law the cart\n"
        "     /// detection's own lane serves, the neighbor signal the\n"
        "     /// detector's verdict never arms).\n"
        f"     pub const TILE_DETECTOR_RAIL_ON: u16 = {NEW_IDX};"
    )
    src = edit(src, anchor_named, anchor_named + named, "E4 named seat")

    with open(PATH, "w", encoding="utf-8") as f:
        f.write(src)
    print(f"appended TILE_{NEW_IDX}_RGBA + detector_rail_on seat ({len(rgba)} texel bytes)")


if __name__ == "__main__":
    sys.exit(main())
