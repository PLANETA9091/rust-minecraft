#!/usr/bin/env python3
"""autopush-300 — THE LATCH'S OWN DEDICATED REGION pin.

The dev-193 -> autopush-299 NEXT list's own candidate the 3-row bend
retires: the vanilla lock the 4-TALL dims the live disasm re-pins
(ChestModel.createSingleBodyLayer the `lock` child texOffs(0,0)
.addBox(7,-2,14,2,4,1) PartPose.offset(0,9,1) the SAME pivot the lid
wears — the box the render y 7..11 band the lid-body seam straddles),
the sheet's own lock region (1,1)-(3,5) the 2x4 art the pixel probe
banks (the fourth row a5a5a5|FFFFFF the light flange speaks — the
compose's 3-row copy the row never wore). The pin:

  atlas.rs      TILE_929..935_RGBA the seven chest kinds' own lock
                tiles (the sheet's box-UV corner (0,0)-(6,6) the
                verbatim mirror wears: the up/down caps row 0, the
                west/north/east/south faces rows 1..5) appended at
                the POOL TAIL so every existing tile index holds
                (the goldens the same pixels render); TILE_NAMES the
                seven's tails; CHEST_LOCK_TILES the [u16; N_KINDS]
                table the seven rows 929..935 wear (TILE_NONE the
                rest); the TILE_*_LOCK named consts the tests read.

The compose_chest_tiles generator source the SAME walk wears (the
5-tuple + the front tile's 3-row band the dedicated region retires —
the open lid/body faces' flat ghost the pure lid+body art retires).

Idempotent: every section checks its own anchor first.
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
TEX = os.path.join(REPO, "assets-cache/jar/assets/minecraft/textures")

# the pool tail order the seven kinds' rows keep — 929..935 (the
# world.rs kind seats 737..743 the same order the palette wears).
KINDS = [
    (737, 929, "entity/chest/normal.png", "chest_lock", "TILE_CHEST_LOCK"),
    (738, 930, "entity/chest/trapped.png", "trapped_chest_lock", "TILE_TRAPPED_CHEST_LOCK"),
    (739, 931, "entity/chest/ender.png", "ender_chest_lock", "TILE_ENDER_CHEST_LOCK"),
    (740, 932, "entity/chest/copper.png", "copper_chest_lock", "TILE_COPPER_CHEST_LOCK"),
    (741, 933, "entity/chest/copper_exposed.png", "exposed_copper_chest_lock", "TILE_EXPOSED_COPPER_CHEST_LOCK"),
    (742, 934, "entity/chest/copper_weathered.png", "weathered_copper_chest_lock", "TILE_WEATHERED_COPPER_CHEST_LOCK"),
    (743, 935, "entity/chest/copper_oxidized.png", "oxidized_copper_chest_lock", "TILE_OXIDIZED_COPPER_CHEST_LOCK"),
]

# THE WAXED TWINS the bare family's own lock tile re-wear (the
# chest_family_sheet law: no waxed sheet the jar keeps — the waxed
# kinds the bare copper master's tiles wear, the lock seat the same
# row answers).
WAXED_KIND_SEATS = [
    (744, 932),  # waxed_copper_chest <- copper
    (745, 933),  # waxed_exposed_copper_chest <- copper_exposed
    (746, 934),  # waxed_weathered_copper_chest <- copper_weathered
    (747, 935),  # waxed_oxidized_copper_chest <- copper_oxidized
]

OLD_TILES = 929
NEW_TILES = 936


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def lock_tile_rgba(sheet_rel: str) -> list:
    """The sheet's box-UV corner (0,0)-(6,6) the verbatim mirror wears
    (the up/down caps the row 0 the box layout banks, the four face
    regions the rows 1..5 keep — the rest the blank 16x16 keeps)."""
    img = Image.open(os.path.join(TEX, sheet_rel)).convert("RGBA")
    px = img.load()
    tile = [0] * (16 * 16 * 4)
    for ty in range(6):
        for tx in range(6):
            r, g, b, a = px[tx, ty]
            i = (ty * 16 + tx) * 4
            tile[i : i + 4] = [r, g, b, a]
    return tile


def fmt_bytes(data: list) -> str:
    rows = []
    for i in range(0, len(data), 16):
        rows.append("    " + ",".join(str(b) for b in data[i : i + 16]) + ",")
    return "\n".join(rows)


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. the seven tile statics (the sheets' own corners decoded).
    if "static TILE_929_RGBA" not in atlas:
        statics = (
            "\n/// THE LATCH'S OWN DEDICATED REGION (the autopush-300\n"
            "    /// round — the seven chest kinds' own lock tiles the\n"
            "    /// sheets' box-UV corner (0,0)-(6,6) the verbatim mirror\n"
            "    /// wears: the 26.2 jar the live disasm re-pins the lock\n"
            "    /// child texOffs(0,0).addBox(7,-2,14,2,4,1) PartPose.\n"
            "    /// offset(0,9,1) the SAME pivot the lid wears — the\n"
            "    /// 2x4 front art the sheet's (1,1)-(3,5) region banks\n"
            "    /// [the fourth row a5a5a5|FFFFFF the light flange the\n"
            "    /// compose's 3-row copy never wore], the up/down caps\n"
            "    /// the row 0 the box layout keeps, the west/east sides\n"
            "    /// the rows 1..5 dark column wear).\n"
        )
        for _, idx, sheet, _, _ in KINDS:
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(lock_tile_rgba(sheet))
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
    tail_old = f",&TILE_{OLD_TILES - 1}_RGBA,];"
    tail_new = (
        f",&TILE_{OLD_TILES - 1}_RGBA,"
        + ",".join(f"&TILE_{idx}_RGBA" for _, idx, _, _, _ in KINDS)
        + ",];"
    )
    atlas = edit_all(atlas, tail_old, tail_new, 1, "tiles tail")

    # 3. TILE_NAMES the seven ride.
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"dropper_front_vertical", "shulker_box"];',
        '"dropper_front_vertical", "shulker_box", '
        + ", ".join(f'"{n}"' for _, _, _, n, _ in KINDS)
        + "];",
        1,
        "tile_names tail",
    )

    # 4. CHEST_LOCK_TILES the seven rows wear (TILE_NONE the rest —
    # the non-chest kinds the lane never reads; the WAXED twins the
    # bare family's own lock row re-wear).
    if "pub const CHEST_LOCK_TILES" not in atlas:
        rows = ["    65535,"] * 779
        for kind, idx, _, _, _ in KINDS:
            rows[kind] = f"    {idx},"
        for kind, idx in WAXED_KIND_SEATS:
            rows[kind] = f"    {idx},"
        block = (
            "\n/// THE LATCH'S OWN KIND SEATS (the autopush-300 round —\n"
            "/// the seven chest kinds the lock tile's own slot wear,\n"
            "/// TILE_NONE the rest the lane never reads; the WAXED\n"
            "/// twins the bare family's own lock row re-wear).\n"
            "pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [\n"
            + "\n".join(rows)
            + "\n];\n"
        )
        anchor = "pub const FACE_TILES: [[u16; 6]; N_KINDS] = ["
        atlas = edit_all(atlas, anchor, block + "\n" + anchor, 1, "lock tiles table")
    else:
        print("  [lock tiles table] already pinned")
        # the waxed seats the first pin pass never wore — the idempotent
        # fixup the bare family's law keeps
        start = atlas.find("pub const CHEST_LOCK_TILES")
        close = atlas.find("];", start)
        lines = atlas[start:close].split("\n")
        changed = False
        for kind, idx in WAXED_KIND_SEATS:
            row_no = 1 + kind  # line 0 = the decl, row k = line k+1
            if lines[row_no].strip().rstrip(",") != str(idx):
                lines[row_no] = f"    {idx},"
                changed = True
                print(f"  [waxed seat {kind} <- {idx}] fixed")
            else:
                print(f"  [waxed seat {kind} <- {idx}] already pinned")
        if changed:
            atlas = atlas[:start] + "\n".join(lines) + atlas[close:]

    # 5. the named consts the tests read.
    if "pub const TILE_CHEST_LOCK:" not in atlas:
        consts = []
        for _, idx, _, _, const in KINDS:
            consts.append(f"pub const {const}: u16 = {idx};")
        block = (
            "\n/// THE LATCH'S OWN NAMED SEATS (the pool tail's seven the\n"
            "/// mesher's verdict the tests read).\n"
            + "\n".join(consts)
            + "\n"
        )
        anchor = "pub const TILE_PISTON_TOP: u16 = 919;"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)
    print("autopush300_lock_pin: done")


if __name__ == "__main__":
    main()
