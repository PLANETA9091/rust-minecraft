#!/usr/bin/env python3
"""dev-194 — THE BED'S OWN MESH WALK pin.

The dev-193 NEXT list's own successor round (the dev lane's audit
re-verdicts the shulker/chest/hopper NEXT items the autopush rounds
consumed — the bed the sleep-walk's own render foundation the
interaction gap the vanilla core keeps). THE JAR LAW the live assets
re-pin (the 26.2 jar the blockstates/red_bed.json + the models
block/red_bed_head.json + block/red_bed_foot.json + the two
block/template_bed_* the derive walk banks — the probe rc=0 the
15-quad bakes the cross-level #ref merge answers): the blockstate the
eight variants the facing y-rotation wears (north 0 / east 90 /
south 180 / west 270 — the hopper lane's own CW quarter-turn law),
the part the head/foot pair the model split serves; the template's
three elements the plate (0,3,0)->(16,9,16) the six-face bake + the
two legs (0,0,0)->(3,3,3) and (13,0,0)->(16,3,3) the cullface rows
keep; the textures the per-color seven the model rows pass
(#east/#up/#west the head's trio, #east/#south/#up/#west the foot's
quartet) + the two shared masters (block/bed_down the plate's
underside, block/bed_head_north the head's front strip).

The pin:

  atlas.rs      TILE_941..1054_RGBA the 114 bed masters the jar's
                block/<color>_bed_{head,foot}_*.png + the two shared
                the decode serves, appended at the POOL TAIL so every
                existing tile index holds (the goldens the same
                pixels render); TILE_NAMES the 114's tails; the
                TILE_BED_DOWN / TILE_BED_HEAD_NORTH / TILE_RED_BED_*
                named consts the tests read.

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

OLD_TILES = 941
NEW_TILES = 1055
OLD_KINDS = 781
NEW_KINDS = 797

# the vanilla registration order the blocks table keeps.
COLORS = [
    "black", "blue", "brown", "cyan", "gray", "green", "light_blue",
    "light_gray", "lime", "magenta", "orange", "pink", "purple", "red",
    "white", "yellow",
]

# the shared pair first, then the per-color seven (the head's trio the
# model's #east/#up/#west rows pass, the foot's quartet the
# #east/#south/#up/#west rows keep — the north strip the head template
# inlines, the down the plate's underside the both parts wear).
TILE_SPECS: list = [("bed_down", "TILE_BED_DOWN"), ("bed_head_north", "TILE_BED_HEAD_NORTH")]
for c in COLORS:
    TILE_SPECS.append((f"{c}_bed_head_east", None))
    TILE_SPECS.append((f"{c}_bed_head_up", "TILE_RED_BED_HEAD_UP" if c == "red" else None))
    TILE_SPECS.append((f"{c}_bed_head_west", None))
    TILE_SPECS.append((f"{c}_bed_foot_east", None))
    TILE_SPECS.append((f"{c}_bed_foot_south", "TILE_RED_BED_FOOT_SOUTH" if c == "red" else None))
    TILE_SPECS.append((f"{c}_bed_foot_up", "TILE_RED_BED_FOOT_UP" if c == "red" else None))
    TILE_SPECS.append((f"{c}_bed_foot_west", None))
assert len(TILE_SPECS) == 114, f"the 114 the spec table keeps, got {len(TILE_SPECS)}"


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

    # 1. the 114 tile statics (the jar's own masters decoded).
    if "static TILE_941_RGBA" not in atlas:
        statics = (
            "\n/// THE BED'S OWN MASTERS (the dev-194 round — the 114\n"
            "    /// tiles the jar's block/<color>_bed_{head,foot}_*.png\n"
            "    /// the decode serves + the two shared (bed_down the\n"
            "    /// plate's underside, bed_head_north the head's front\n"
            "    /// strip) — the template_bed_* elements the refs the\n"
            "    /// per-color model rows pass, the pool-tail append the\n"
            "    /// every-existing-index law keeps).\n"
        )
        for idx, (name, _) in enumerate(TILE_SPECS, start=OLD_TILES):
            statics += (
                f"static TILE_{idx}_RGBA: [u8; 1024] = [\n"
                + fmt_bytes(tile_rgba(f"block/{name}.png"))
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
        f",&TILE_{OLD_TILES - 1}_RGBA,];",
        f",&TILE_{OLD_TILES - 1}_RGBA,"
        + ",".join(f"&TILE_{OLD_TILES + i}_RGBA" for i in range(114))
        + ",];",
        1,
        "tiles tail",
    )

    # 3. TILE_NAMES the 114 ride.
    names = ", ".join(f'"{n}"' for n, _ in TILE_SPECS)
    atlas = edit_all(
        atlas,
        f"pub static TILE_NAMES: [&str; {OLD_TILES}] = [",
        f"pub static TILE_NAMES: [&str; {NEW_TILES}] = [",
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"brewing_stand_base"];',
        '"brewing_stand_base", ' + names + "];",
        1,
        "tile_names tail",
    )

    # 4. the named consts (the tests' seats — the shared pair + the
    # red seven the color lane's own anchor keeps).
    if "pub const TILE_BED_DOWN" not in atlas:
        consts = "\n/// THE BED'S OWN NAMED SEATS (the pool tail's 114 — the shared\n"
        consts += "/// pair + the red seven the tests and the lane read).\n"
        for idx, (_, const) in enumerate(TILE_SPECS, start=OLD_TILES):
            if const:
                consts += f"pub const {const}: u16 = {idx};\n"
        anchor = "pub const FACE_TILES: [[u16; 6]; N_KINDS] = ["
        atlas = edit_all(atlas, anchor, consts + "\n" + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    # 5. the sixteen kind seats (the vanilla registration order the
    # ladder keeps — the kind ids 781..796 the client's own space
    # wears; the FACE_TILES the TILE_NONE rows the bed the mesher's
    # own lane serves [the generic cube the bed never wears], the
    # TINTS the no-tint row, the door/head/chest-lock tables the
    # not-a-door/not-a-head/empty rows keep).
    atlas = edit_all(
        atlas,
        f"pub const N_KINDS: usize = {OLD_KINDS};",
        f"pub const N_KINDS: usize = {NEW_KINDS};",
        1,
        "n_kinds",
    )
    bed_names = ", ".join(f'"{c}_bed"' for c in COLORS)
    atlas = edit_all(
        atlas,
        '"shulker_box",\n    "hopper",\n    "brewing_stand"];',
        '"shulker_box",\n    "hopper",\n    "brewing_stand",\n    ' + bed_names + "];",
        2,
        "kind+registry names",
    )
    # the six N_KINDS tables the sixteen kind rows wear (the
    # 16-row append the kind batch's own shape keeps — the row-count
    # check the idempotence proves).
    for decl, row, tag in [
        ("pub const FACE_TILES: [[u16; 6]; N_KINDS] = [", "    [65535, 65535, 65535, 65535, 65535, 65535],", "face_tiles rows"),
        ("pub const TINTS: [[u32; 6]; N_KINDS] = [", "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],", "tints rows"),
        ("pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS] = [", '    "",', "door rows"),
        ("pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS] = [", "    [65535, 65535, 65535, 65535, 65535, 65535],", "head faces rows"),
        ("pub const CHEST_LOCK_TILES: [u16; N_KINDS] = [", "    65535,", "chest_lock rows"),
    ]:
        start = atlas.find(decl)
        if start < 0:
            sys.exit(f"  [{tag}] decl anchor missing")
        close = atlas.find("\n];", start)
        body = atlas[start:close]
        rows = [l for l in body.split("\n") if l.rstrip().endswith(",")]
        if len(rows) == NEW_KINDS:
            print(f"  [{tag}] already pinned")
        elif len(rows) == OLD_KINDS:
            atlas = atlas[:close] + "\n" + row * 16 + atlas[close:]
            print(f"  [{tag}] pinned (16 rows)")
        else:
            sys.exit(f"  [{tag}] row count {len(rows)} != {OLD_KINDS}/{NEW_KINDS}")

    open(ATLAS, "w").write(atlas)

    # 6. world.rs the NAME_OF rows + decl.
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
        '"shulker_box",\n    "hopper",\n    "brewing_stand",\n];',
        '"shulker_box",\n    "hopper",\n    "brewing_stand",\n    ' + bed_names + ",\n];",
        1,
        "name_of tail",
    )
    open(WORLD, "w").write(world)

    # 7. the sweep (the kind-count sites the verdicts bank).
    import glob
    import re

    pats = [
        (re.compile(r"assert_eq!\(atlas::N_KINDS, 781,"), 26, "kinds assert"),
        (re.compile(r"assert_eq!\(NAME_OF\.len\(\), 781,"), 21, "name assert"),
        (re.compile(r"atlas::N_KINDS,\s*\n\s*781,"), 8, "kinds multi"),
        (re.compile(r"atlas::N_KINDS, 781,"), 1, "kinds bare"),
    ]
    files = ["crates/mc-client/src/world.rs"] + sorted(
        glob.glob("crates/mc-client/src/screens/tests/*.rs")
    )
    for pat, want, tag in pats:
        total = 0
        done = 0
        for f in files:
            txt = open(f).read()
            n = len(pat.findall(txt))
            if n:
                total += n
                txt = pat.sub(lambda m: m.group(0).replace("781", str(NEW_KINDS)), txt)
                open(f, "w").write(txt)
            # the already-pinned probe: the same shape the NEW constant
            # wears (the re-run the skip verdict banks)
            probe = re.compile(pat.pattern.replace("781", str(NEW_KINDS)))
            done += len(probe.findall(txt))
        if total == 0 and done >= want:
            print(f"  [{tag}] already pinned")
            continue
        if total != want:
            sys.exit(f"  [{tag}] sweep count {total} != {want}")
        print(f"  [{tag}] swept ({want} site(s))")

    print("bed_pin_194: done")


if __name__ == "__main__":
    main()
