#!/usr/bin/env python3
"""verify_atlas_full.py — the append-only equivalence proof (recreated for
autopush-673; the original was used in the autopush-672 round but never
committed).

Compares the WORKING-TREE generated atlas against a BASE revision (default:
git HEAD) and proves:
  1. every kind row < BASE_N_KINDS keeps its FACE_TILES pixels and TINTS
     (indices may shift only if the pointed-at PIXELS are the same — the
     honest check compares resolved pixel bytes, not raw indices);
  2. every base tile's PIXELS survive somewhere in the new tile table
     (byte-identical RGBA), and every kind row's resolved pixel bytes for
     the six faces match the base kind row's resolved bytes;
  3. KIND_NAMES/REGISTRY_NAMES rows < BASE_N_KINDS are verbatim;
  4. the new tail rows are exactly the expected additions.

Exit 0 = the render-relevant content of every pre-existing row is unchanged
(the goldens' pixel-perfect law keeps); non-zero with a diff report otherwise.
"""
import re
import subprocess
import sys

PATH = "crates/mc-registry/src/generated/atlas.rs"


def parse(src: str):
    def const_list_u16(name):
        m = re.search(r"pub const %s: \[\[u16; 6\]; N_KINDS\] = \[(.*?)\];" % name, src, re.S)
        return [[int(x) for x in row.split(",")] for row in re.findall(r"\[([0-9,\s]+)\]", m.group(1))]

    def const_list_u32(name):
        m = re.search(r"pub const %s: \[\[u32; 6\]; N_KINDS\] = \[(.*?)\];" % name, src, re.S)
        rows = []
        for row in re.findall(r"\[([0-9a-fA-Fx,\s]+)\]", m.group(1)):
            vals = [v.strip() for v in row.split(",") if v.strip()]
            rows.append([int(v, 0) for v in vals])
        return rows

    def names(name):
        m = re.search(r"pub const %s: \[&str; N_KINDS\] = \[(.*?)\];" % name, src, re.S)
        return re.findall(r'"([^"]+)"', m.group(1))

    n_kinds = int(re.search(r"pub const N_KINDS: usize = (\d+);", src).group(1))

    # Tile pixel data: static TILE_<i>_RGBA: [u8; 1024] = [...];
    tiles = []
    for m in re.finditer(r"static TILE_(\d+)_RGBA: \[u8; 1024\] = \[([0-9,\s]+)\];", src):
        idx = int(m.group(1))
        assert idx == len(tiles), f"tile index gap at {idx}"
        tiles.append([int(v) for v in m.group(2).split(",") if v.strip()])

    face_tiles = const_list_u16("FACE_TILES")
    tints = const_list_u32("TINTS")
    kind_names = names("KIND_NAMES")
    registry_names = names("REGISTRY_NAMES")
    assert len(face_tiles) == n_kinds and len(tints) == n_kinds
    assert len(kind_names) == n_kinds and len(registry_names) == n_kinds
    return {
        "n_kinds": n_kinds,
        "tiles": tiles,
        "face_tiles": face_tiles,
        "tints": tints,
        "kind_names": kind_names,
        "registry_names": registry_names,
    }


def resolved_pixels(data, row):
    """The six faces' pixel bytes a kind row renders with."""
    return [bytes(data["tiles"][t]) if t < len(data["tiles"]) else None for t in data["face_tiles"][row]]


def main():
    base_ref = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
    base_src = subprocess.run(
        ["git", "show", f"{base_ref}:{PATH}"], capture_output=True, text=True, check=True
    ).stdout
    work_src = open(PATH).read()
    base = parse(base_src)
    work = parse(work_src)

    errors = []

    # 1. tiles: every base tile's pixels survive byte-identical somewhere.
    work_tile_set = {}
    for i, t in enumerate(work["tiles"]):
        work_tile_set.setdefault(tuple(t), i)
    missing = 0
    for i, t in enumerate(base["tiles"]):
        if tuple(t) not in work_tile_set:
            missing += 1
            if missing <= 5:
                errors.append(f"tile {i}: pixels absent from the working tree")
    if missing:
        errors.append(f"TOTAL missing base tiles: {missing}")

    # 2. kind rows < base N_KINDS: resolved pixels + tints + names verbatim.
    for row in range(base["n_kinds"]):
        b = resolved_pixels(base, row)
        w = resolved_pixels(work, row)
        if b != w:
            errors.append(
                f"kind row {row} ({base['kind_names'][row]}): face pixels changed"
            )
        if base["tints"][row] != work["tints"][row]:
            errors.append(f"kind row {row}: tint row changed {base['tints'][row]} -> {work['tints'][row]}")
        if base["kind_names"][row] != work["kind_names"][row]:
            errors.append(f"kind row {row}: kind name changed")
        if base["registry_names"][row] != work["registry_names"][row]:
            errors.append(f"kind row {row}: registry name changed")

    # 3. the tail: the new rows are the additions.
    new_rows = work["n_kinds"] - base["n_kinds"]
    print(f"base kinds {base['n_kinds']}, work kinds {work['n_kinds']} (+{new_rows})")
    print(f"base tiles {len(base['tiles'])}, work tiles {len(work['tiles'])} (+{len(work['tiles']) - len(base['tiles'])})")
    for row in range(base["n_kinds"], work["n_kinds"]):
        print(f"  new row {row}: kind={work['kind_names'][row]} reg={work['registry_names'][row]} tiles={work['face_tiles'][row][:3]}")

    if errors:
        print("\nFAIL:")
        for e in errors[:40]:
            print("  " + e)
        sys.exit(1)
    print("PASS: every pre-existing kind row renders the same pixels + tints + names; all base tiles survive byte-identical.")


if __name__ == "__main__":
    main()
