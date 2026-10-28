#!/usr/bin/env python3
"""autopush-303 — THE WHOLE-CHEST'S OWN V-FOLD pin.

The 302 verdict's whole-chest extension: the compose_face's body band
the tile rows 5..14 the DIRECT stack wears (the band's first row the
face's bottom edge samples — the jar's Polygon remap the 302 round
banked, the vfold_probe_303 the box constants re-pins). The pin
rewrites the generated atlas.rs's chest family FRONT/SIDE tile bytes
to the new compose (the generator source the same walk wears):

  rows 0..4   the lid strip (UNCHANGED: sheet (14, lid_y..lid_y+5))
  rows 5..14  the body band: tile[1+fx][fy] <- sheet[body_x+fx][33+(fy-5)]
              (the OLD window body_y+1..+10 + the 15th-row dup retires)

Seven sheets (the waxed twins the bare family's tiles re-wear — no
waxed sheet the jar keeps), 14 tiles total. The top/bottom/lock tiles
the round never touches. Idempotent: the tile whose bytes already
match the new compose the pass skips; the current-content guard the
old compose's bytes the mismatch reports.
"""
import os
import sys

try:
    from PIL import Image
except ImportError:
    sys.exit("PIL missing")

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

# (prefix, sheet path) — the seven kinds' rows the 300 pin's order keeps
SHEETS = [
    ("chest", "entity/chest/normal.png"),
    ("trapped_chest", "entity/chest/trapped.png"),
    ("ender_chest", "entity/chest/ender.png"),
    ("copper_chest", "entity/chest/copper.png"),
    ("exposed_copper_chest", "entity/chest/copper_exposed.png"),
    ("weathered_copper_chest", "entity/chest/copper_weathered.png"),
    ("oxidized_copper_chest", "entity/chest/copper_oxidized.png"),
]


def compose_face(rgba, w, lid_y, body_x):
    """the NEW compose_face (the generator source's own law)"""
    tile = [0] * (16 * 16 * 4)

    def put(tx, ty, sx, sy):
        i = (ty * 16 + tx) * 4
        j = (sy * w + sx) * 4
        tile[i:i + 4] = rgba[j:j + 4]

    lid_x = 14
    for fy in range(0, 5):
        for fx in range(0, 14):
            put(1 + fx, fy, lid_x + fx, lid_y + fy)
    body_y = 33
    for fy in range(5, 15):
        for fx in range(0, 14):
            put(1 + fx, fy, body_x + fx, body_y + (fy - 5))
    return bytes(tile)


def old_compose_face(rgba, w, lid_y, body_x):
    """the OLD compose (the guard the current bytes must match)"""
    tile = [0] * (16 * 16 * 4)

    def put(tx, ty, sx, sy):
        i = (ty * 16 + tx) * 4
        j = (sy * w + sx) * 4
        tile[i:i + 4] = rgba[j:j + 4]

    lid_x = 14
    for fy in range(0, 5):
        for fx in range(0, 14):
            put(1 + fx, fy, lid_x + fx, lid_y + fy)
    body_y = 33
    for fy in range(5, 14):
        for fx in range(0, 14):
            put(1 + fx, fy, body_x + fx, body_y + 1 + (fy - 5))
    for fx in range(0, 14):
        put(1 + fx, 14, body_x + fx, body_y + 9)
    return bytes(tile)


def main():
    src = open(ATLAS).read()

    # the TILE_NAMES row the index lookup wears
    import re
    m = re.search(r"pub static TILE_NAMES: \[&str; \d+\] = \[(.*?)\];", src, re.S)
    assert m, "TILE_NAMES not found"
    names = re.findall(r'"([^"]+)"', m.group(1))
    print(f"tile names: {len(names)}")

    n_new = n_skip = 0
    for prefix, sheet in SHEETS:
        img = Image.open(os.path.join(TEX, sheet)).convert("RGBA")
        w, h = img.size
        rgba = img.tobytes()
        for stem, lid_y, body_x in (
            (f"{prefix}_front", 14, 14),
            (f"{prefix}_side", 28, 28),
        ):
            idx = names.index(stem)
            new_bytes = compose_face(rgba, w, lid_y, body_x)
            old_bytes = old_compose_face(rgba, w, lid_y, body_x)

            # locate the tile's static: static TILE_{idx}_RGBA: [u8; 1024] = [...];
            decl = f"static TILE_{idx}_RGBA: [u8; 1024] = ["
            i = src.index(decl)
            j = src.index("];", i)
            body_txt = src[i + len(decl):j]
            cur = bytes(int(x) for x in re.findall(r"\d+", body_txt))
            assert len(cur) == 1024, f"{stem}: {len(cur)} bytes"

            if cur == new_bytes:
                n_skip += 1
                print(f"  SKIP {stem} (tile {idx}) — the new compose already wears")
                continue
            if cur != old_bytes:
                print(f"  WARN {stem} (tile {idx}) — neither the old nor the new compose matches "
                      f"(the first diff at byte "
                      f"{next(k for k in range(1024) if cur[k] != new_bytes[k])})")
            new_txt = ",".join(str(b) for b in new_bytes)
            src = src[:i + len(decl)] + new_txt + src[j:]
            n_new += 1
            print(f"  PIN  {stem} (tile {idx}) — the body band the direct stack wears")

    open(ATLAS, "w").write(src)
    print(f"pinned {n_new}, skipped {n_skip}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
