#!/usr/bin/env python3
"""autopush-126 — the repeater's own atlas/elements pin.

The DiodeBlock sibling the worklog's next-candidates named: the pin
rides the OFFICIAL 26.2 jar's own bytes the same pool-tail walk the
comparator (124) and redstone block (125) rounds wore:

  atlas.rs      TILE_912_RGBA (block/repeater.png) + TILE_913_RGBA
                (block/repeater_on.png) decoded from the runtime cache,
                appended at the POOL TAIL so every existing tile index
                holds (the goldens the same pixels render);
                N_KINDS 770 -> 771; KIND_NAMES the "repeater" tail +
                REGISTRY_NAMES the "redstone_repeater" tail (the jar's
                own registry the Blocks register speaks); FACE_TILES
                the [912;6] row, TINTS the zero row, DOOR_MODEL_BY_ROW
                the empty ref, ENTITY_HEAD_FACES the TILE_NONE row
                (the per-kind tables the N_KINDS length law keeps);
                TILE_NAMES the pair's own tails; the TILE_REPEATER/
                TILE_REPEATER_ON named consts the mesher's verdict
                reads.
  elements.rs   the TWELVE typed model rows the jar's own model JSONs
                speak — block/repeater_{1,2,3,4}tick, the _locked
                trio (the bedrock bar the LOCKED state wears), the
                _on trio (the lit torch + the twelve halo faces the
                glow star carries) — inserted in the alphabetical seat.
  world.rs      NAME_OF the 771st entry (the palette resolver's row;
                the "repeater" stem the dig/sound/resistance/drops
                tables key AND the item pairing answers — the jar's
                own loot_table/blocks/repeater.json + Items rows the
                short name wear, the REGISTRY_NAMES atlas row the
                "redstone_repeater" registry law keeps).

Idempotent: every section checks its own anchor first (the re-run the
"already pinned" answer keeps).
"""
import json
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
    if have >= 771:
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


def fmt_f(v) -> str:
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return f"{v}.0"
    return repr(float(v))


def emit_element_row(model: str, obj: dict) -> str:
    """One typed elements.rs row, the jar's own model JSON speaks it."""
    faces_out = []
    for el in obj["elements"]:
        faces = []
        for key in ["down", "up", "north", "south", "west", "east"]:
            f = el["faces"].get(key)
            if f is None:
                faces.append("None")
                continue
            uv = f.get("uv")
            uv_s = (
                "Some([" + ", ".join(fmt_f(v) for v in uv) + "])"
                if uv is not None
                else "None"
            )
            cf = f.get("cullface")
            cf_s = (
                f"Some(Cull::{cf.capitalize()})" if cf else "None"
            )
            faces.append(
                "Some(ElementFace { uv: " + uv_s
                + ", texture: \"" + f["texture"] + "\""
                + ", cullface: " + cf_s
                + ", tintindex: None, rotation: None })"
            )
        shade = el.get("shade", True)
        light = el.get("light_emission")
        rot = el.get("rotation")
        if rot is None:
            rot_s = "None"
        else:
            rot_s = (
                "Some(ElementRotation::Axis { angle: " + fmt_f(rot["angle"])
                + ", axis: Axis::" + rot["axis"].capitalize()
                + ", origin: [" + ", ".join(fmt_f(v) for v in rot["origin"]) + "]"
                + ", rescale: " + ("true" if rot.get("rescale") else "false") + " })"
            )
        faces_out.append(
            "Element { from: [" + ", ".join(fmt_f(v) for v in el["from"]) + "]"
            + ", to: [" + ", ".join(fmt_f(v) for v in el["to"]) + "]"
            + ", shade: " + ("true" if shade else "false")
            + ", light_emission: " + (f"Some({light}u8)" if light else "None")
            + ", rotation: " + rot_s
            + ", faces: [" + ", ".join(faces) + "]"
            + ", name: None, comment: None }"
        )
    return "    (\"" + model + "\", &[" + ",".join(faces_out) + "]),"


MODELS = (
    ["block/repeater_%dtick" % d for d in (1, 2, 3, 4)]
    + ["block/repeater_%dtick_locked" % d for d in (1, 2, 3, 4)]
    + ["block/repeater_%dtick_on" % d for d in (1, 2, 3, 4)]
)


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. N_KINDS.
    atlas = edit_all(
        atlas,
        "pub const N_KINDS: usize = 770;",
        "pub const N_KINDS: usize = 771;",
        1,
        "n_kinds",
    )

    # 2. the two tile statics (decoded from the jar's own PNGs).
    if "static TILE_912_RGBA" not in atlas:
        b912 = tile_rgba("block/repeater.png")
        b913 = tile_rgba("block/repeater_on.png")
        statics = (
            "\n/// THE REPEATER'S OWN TILES (the autopush-126 round — the\n"
            "    /// jar's block/repeater.png + block/repeater_on.png the\n"
            "    /// pool tail wears, the every-existing-index law the goldens\n"
            "    /// keep).\n"
            "static TILE_912_RGBA: [u8; 1024] = [\n" + fmt_bytes(b912) + "\n];\n"
            "static TILE_913_RGBA: [u8; 1024] = [\n" + fmt_bytes(b913) + "\n];\n\n"
        )
        anchor = "pub static TILES: [&[u8]; 912] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 3. the pool itself.
    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 912] = [",
        "pub static TILES: [&[u8]; 914] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_910_RGBA,&TILE_911_RGBA,];",
        ",&TILE_910_RGBA,&TILE_911_RGBA,&TILE_912_RGBA,&TILE_913_RGBA,];",
        1,
        "tiles tail",
    )

    # 4. TILE_NAMES the pair rides.
    atlas = edit_all(
        atlas,
        'pub static TILE_NAMES: [&str; 912] = [',
        'pub static TILE_NAMES: [&str; 914] = [',
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"comparator_on", "redstone_block"];',
        '"comparator_on", "redstone_block", "repeater", "repeater_on"];',
        1,
        "tile_names tail",
    )

    # 5. the kind + registry name tails (the SHARED law the two tables
    # keep — the KIND the item word wears, the REGISTRY the dig/drop/
    # sound lanes' word: the "repeater" stem both answer, the vanilla
    # register's own "redstone_repeater" the dig/sound/resist tables
    # never key [the jar's loot_table/blocks/repeater.json the stem
    # speaks]) — the per-table seat the shared tail line the
    # whole-text guard never answers.
    for decl, name in (("pub const KIND_NAMES", "kind tail"), ("pub const REGISTRY_NAMES", "registry tail")):
        start = atlas.index(decl)
        close = atlas.index("];", start)
        seg = atlas[start:close]
        if '"redstone_repeater"' in seg or '"repeater"]' in seg:
            print(f"  [{name}] already pinned")
            continue
        want = '    "redstone_block"'
        if want not in seg:
            sys.exit(f"  [{name}] tail anchor missing")
        // THE SHARED ROW LAW (the dig/drop/sound lanes' word the
        // REGISTRY_NAMES table itself carries — the "repeater" stem
        // the jar's own loot_table/blocks/repeater.json + the 1196-name
        // dig/sound/resist tables key): both tails the same row wears
        new_name = "repeater"
        seg = seg.replace(want, f'    "redstone_block",\n    "{new_name}"', 1)
        atlas = atlas[:start] + seg + atlas[close:]
        print(f"  [{name}] pinned")

    # 6-9. the per-kind rows the N_KINDS length law keeps.
    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [912, 912, 912, 912, 912, 912],",
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
    if "pub const TILE_REPEATER:" not in atlas:
        block = (
            "\n/// THE REPEATER'S OWN SEATS (the autopush-126 round — the\n"
            "     /// pool tail the block/repeater + block/repeater_on\n"
            "     /// textures wear: the mesher's per-quad texture verdict the\n"
            "     /// consts read, the #top swap the powered bit's own seat\n"
            "     /// banks).\n"
            "     pub const TILE_REPEATER: u16 = 912;\n"
            "\n"
            "/// THE REPEATER'S ON SEAT (the POWERED bit's own lit tile).\n"
            "     pub const TILE_REPEATER_ON: u16 = 913;\n"
        )
        anchor = "\n/// The campfire lane's own campfire_log tile"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 11. the elements.rs rows (the alphabetical seat the first row id
    # past the repeater family wears — the redstone_* rows the seat
    # before keeps).
    print("elements.rs:")
    elements = open(ELEMENTS).read()
    if '"block/repeater_1tick", &[' in elements:
        print("  [model rows] already pinned")
    else:
        rows = []
        for m in MODELS:
            obj = json.load(
                open(os.path.join(REPO, f"assets-cache/jar/assets/minecraft/models/{m}.json"))
            )
            rows.append(emit_element_row(m, obj))
        # the alphabetical seat: the first row key the repeater family's
        # own sort order passes — every existing index holds
        needle = None
        for m in sorted(MODELS, reverse=True):
            pass
        for candidate in re_seat_scan(elements):
            if candidate > "block/repeater_4tick_on":
                needle = candidate
                break
        if needle is None:
            sys.exit("the alphabetical seat the scan never answered")
        elements = elements.replace(
            f'("{needle}"', rows_block(rows) + f'("{needle}"', 1
        )
        print(f"  [model rows] pinned (12) — seat before {needle}")
    open(ELEMENTS, "w").write(elements)

    # 12. the world.rs NAME_OF row (the registry name the dig/sound/
    # resistance tables key — the item pairing the "repeater" arm the
    # placeable walk answers).
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 770] = [",
        "pub const NAME_OF: [&str; 771] = [",
        1,
        "name_of len",
    )
    if '"repeater"' in world.split("pub const NAME_OF")[1].split("\n];")[0]:
        print("  [name_of tail] already pinned")
    else:
        seg = world.split("pub const NAME_OF: [&str; 771] = [")
        head, tail = seg[0], seg[1]
        close = tail.index("\n];")
        tail = tail[:close] + '\n    "repeater",' + tail[close:]
        world = head + "pub const NAME_OF: [&str; 771] = [" + tail
        print("  [name_of tail] pinned")
    open(WORLD, "w").write(world)

    print("autopush126_repeater_pin: done")


def rows_block(rows: list) -> str:
    return "\n".join(rows) + "\n"


def re_seat_scan(elements: str):
    """The row keys the elements table speaks, in file order."""
    import re
    return re.findall(r'\("([^"]+)", &\[', elements)


if __name__ == "__main__":
    main()
