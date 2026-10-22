#!/usr/bin/env python3
"""autopush-124 — the comparator's own atlas/elements pin.

The sandbox wipe ate the lost lane's pin; this re-pins the comparator's
registry seats from the official 26.2 jar's own bytes:

  atlas.rs      TILE_909_RGBA (block/comparator.png) + TILE_910_RGBA
                (block/comparator_on.png) decoded from the runtime cache,
                appended at the POOL TAIL so every existing tile index
                holds (the goldens the same pixels render);
                N_KINDS 768 -> 769; KIND_NAMES + REGISTRY_NAMES the
                "comparator" tail wears; FACE_TILES the [909;6] row,
                TINTS the zero row, DOOR_MODEL_BY_ROW the empty ref,
                ENTITY_HEAD_FACES the TILE_NONE row (the per-kind tables
                the N_KINDS length law keeps); TILE_NAMES the pair's own
                tails; the TILE_COMPARATOR/TILE_COMPARATOR_ON named
                consts the mesher's verdict reads.
  elements.rs   the four typed model rows (block/comparator, _on,
                _subtract, _on_subtract) the jar's own model JSONs
                speak, inserted in the alphabetical seat.
  world.rs      NAME_OF the 769th entry (the palette resolver's row).

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
    # the section-scoped seat check (the zero rows the file wears
    # elsewhere the whole-text guard never answers)
    have = seg.count("\n    [") + seg.count('\n    "')
    if have >= 769:
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


def fmt_f(v) -> str:
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return f"{v}.0"
    return repr(float(v))


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. N_KINDS.
    atlas = edit_all(
        atlas,
        "pub const N_KINDS: usize = 768;",
        "pub const N_KINDS: usize = 769;",
        1,
        "n_kinds",
    )

    # 2. the two tile statics (decoded from the jar's own PNGs).
    if "static TILE_909_RGBA" not in atlas:
        b909 = tile_rgba("block/comparator.png")
        b910 = tile_rgba("block/comparator_on.png")
        statics = (
            "\n/// THE COMPARATOR'S OWN TILES (the autopush-124 round — the\n"
            "    /// jar's block/comparator.png + block/comparator_on.png the\n"
            "    /// pool tail wears, the every-existing-index law the goldens\n"
            "    /// keep).\n"
            "static TILE_909_RGBA: [u8; 1024] = [\n" + fmt_bytes(b909) + "\n];\n"
            "static TILE_910_RGBA: [u8; 1024] = [\n" + fmt_bytes(b910) + "\n];\n\n"
        )
        anchor = "pub static TILES: [&[u8]; 909] = ["
        atlas = edit_all(atlas, anchor, statics + anchor, 1, "tile statics")
    else:
        print("  [tile statics] already pinned")

    # 3. the pool itself.
    atlas = edit_all(
        atlas,
        "pub static TILES: [&[u8]; 909] = [",
        "pub static TILES: [&[u8]; 911] = [",
        1,
        "tiles len",
    )
    atlas = edit_all(
        atlas,
        ",&TILE_908_RGBA,];",
        ",&TILE_908_RGBA,&TILE_909_RGBA,&TILE_910_RGBA,];",
        1,
        "tiles tail",
    )

    # 4. TILE_NAMES the pair rides.
    atlas = edit_all(
        atlas,
        'pub static TILE_NAMES: [&str; 909] = [',
        'pub static TILE_NAMES: [&str; 911] = [',
        1,
        "tile_names len",
    )
    atlas = edit_all(
        atlas,
        '"detector_rail_on"];',
        '"detector_rail_on", "comparator", "comparator_on"];',
        1,
        "tile_names tail",
    )

    # 5. the kind + registry name tails (the shared line answers both).
    atlas = edit_all(
        atlas,
        '"activator_rail"]',
        '"activator_rail",\n    "comparator"]',
        2,
        "kind/registry tails",
    )

    # 6-9. the per-kind rows the N_KINDS length law keeps.
    atlas = insert_before_close(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS] = [",
        "    [909, 909, 909, 909, 909, 909],",
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
    if "pub const TILE_COMPARATOR:" not in atlas:
        block = (
            "\n/// THE COMPARATOR'S OWN SEATS (the autopush-124 round — the\n"
            "     /// pool tail the block/comparator + block/comparator_on\n"
            "     /// textures wear: the mesher's per-quad texture verdict the\n"
            "     /// consts read, the #top swap the powered bit's own seat\n"
            "     /// banks).\n"
            "     pub const TILE_COMPARATOR: u16 = 909;\n"
            "\n"
            "/// THE COMPARATOR'S ON SEAT (the POWERED bit's own lit tile).\n"
            "     pub const TILE_COMPARATOR_ON: u16 = 910;\n"
        )
        anchor = "\n/// The campfire lane's own campfire_log tile"
        atlas = edit_all(atlas, anchor, block + anchor, 1, "named consts")
    else:
        print("  [named consts] already pinned")

    open(ATLAS, "w").write(atlas)

    # 11. the elements.rs rows (alphabetical seats).
    print("elements.rs:")
    elements = open(ELEMENTS).read()
    if '"block/comparator", &[' in elements:
        print("  [model rows] already pinned")
    else:
        rows = []
        for m in [
            "block/comparator",
            "block/comparator_on",
            "block/comparator_on_subtract",
            "block/comparator_subtract",
        ]:
            obj = json.load(
                open(os.path.join(TEX, f"../models/{m}.json".replace("textures/../models", "../models")))
            ) if False else json.load(
                open(os.path.join(REPO, f"assets-cache/jar/assets/minecraft/models/{m}.json"))
            )
            rows.append(emit_element_row(m, obj))
        # the alphabetical seat: the first row id past comparator_subtract
        needle = '("block/composter"'
        if needle not in elements:
            sys.exit("block/composter seat missing")
        block = "\n".join(rows) + "\n"
        elements = elements.replace(needle, block + needle, 1)
        print("  [model rows] pinned (4)")
    open(ELEMENTS, "w").write(elements)

    # 12. the world.rs NAME_OF row.
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 768] = [",
        "pub const NAME_OF: [&str; 769] = [",
        1,
        "name_of len",
    )
    # the table's own closing seat: the last entry before the ]; that ends it
    if '"comparator"' in world.split("pub const NAME_OF")[1].split("\n];")[0]:
        print("  [name_of tail] already pinned")
    else:
        seg = world.split("pub const NAME_OF: [&str; 769] = [")
        head, tail = seg[0], seg[1]
        close = tail.index("\n];")
        tail = tail[:close] + '\n    "comparator",' + tail[close:]
        world = head + "pub const NAME_OF: [&str; 769] = [" + tail
        print("  [name_of tail] pinned")
    open(WORLD, "w").write(world)

    print("autopush124_comparator_pin: done")


if __name__ == "__main__":
    main()
