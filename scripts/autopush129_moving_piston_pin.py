#!/usr/bin/env python3
"""autopush-129 — the moving piston's own kind pin.

The worklog's next-candidates head: the mid-state the pushed walk
renders. The pin rides the SAME pool-tail law the comparator (124),
redstone block (125), repeater (126), observer (127) and piston (128)
rounds wore — but the HONEST no-tile kind: the jar's own moving_piston
model carries NO faces (one particle ref, block/piston_side — the tile
the pool already banks at 920), so the lane adds NO tiles, NO
TILE_NAMES, NO elements rows — only the kind seat:

  atlas.rs      N_KINDS 775 -> 776; KIND_NAMES + REGISTRY_NAMES the
                "moving_piston" tail (the jar's own registry word —
                minecraft:moving_piston, the block_ids table's own row
                the generated registry banks); FACE_TILES the [920;6]
                row (the particle tile the fallback verdict reads —
                the mesher's moving lane bypasses the normal bake),
                TINTS the zero row, DOOR_MODEL_BY_ROW the empty ref,
                ENTITY_HEAD_FACES the TILE_NONE row (the per-kind
                tables the N_KINDS length law keeps).
  world.rs      NAME_OF the 776th entry (the palette resolver's row —
                the stem the dig/sound/resistance/drops tables key AND
                the item pairing answers — the jar's own
                loot_table/blocks/moving_piston.json + Items rows the
                same words wear); the N_KINDS one-line asserts +
                the NAME_OF.len() asserts the count law keeps.

Idempotent: every section checks its own anchor first (the re-run the
"already pinned" answer keeps).
"""
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ATLAS = os.path.join(REPO, "crates/mc-registry/src/generated/atlas.rs")
WORLD = os.path.join(REPO, "crates/mc-client/src/world.rs")
CLIENT = os.path.join(REPO, "crates/mc-client/src/screens.rs")


def edit_all(text: str, old: str, new: str, want: int, tag: str) -> str:
    n = text.count(old)
    if n == 0 and new in text:
        print(f"  [{tag}] already pinned")
        return text
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    print(f"  [{tag}] pinned ({want} site(s))")
    return text.replace(old, new)


def append_table_row(text: str, decl: str, rows: str, tag: str) -> str:
    # THE TABLE'S OWN TAIL (the regex the type's own `];` dodge keeps —
    # `[[u16; 6]; N_KINDS]` the decl carries, the data's `],` rows the
    # `];` terminator never shares: the lazy body the decl's `= [` arm
    # opens, the FIRST `];` after it the table's own end speaks).
    pat = re.compile(re.escape(decl) + r" = \[(.*?)\];", re.S)
    m = pat.search(text)
    if m is None:
        sys.exit(f"  [{tag}] table not found")
    body = m.group(1)
    # the probe the ROW START anchors — line-start, never a mid-line
    # echo of an existing row's content.
    probe = rows.strip().splitlines()[0].strip()
    if re.search(r"^" + re.escape(probe), body, re.M):
        print(f"  [{tag}] already pinned")
        return text
    print(f"  [{tag}] pinned")
    new_body = body + rows + "\n"
    return text[: m.start(1)] + new_body + text[m.end(1) :]


def main() -> None:
    atlas = open(ATLAS).read()
    print("atlas.rs:")

    # 1. N_KINDS.
    atlas = edit_all(
        atlas,
        "pub const N_KINDS: usize = 775;",
        "pub const N_KINDS: usize = 776;",
        1,
        "n_kinds",
    )

    # 2. the kind + registry name tails (the SHARED law the two tables
    # keep — the one registry word the mid-state wears).
    for decl, name in (
        ("pub const KIND_NAMES", "kind tail"),
        ("pub const REGISTRY_NAMES", "registry tail"),
    ):
        start = atlas.index(decl)
        close = atlas.index("];", start)
        seg = atlas[start:close]
        if '"moving_piston"' in seg:
            print(f"  [{name}] already pinned")
            continue
        want = '    "piston_head"'
        if want not in seg:
            sys.exit(f"  [{name}] tail anchor missing")
        seg = seg.replace(want, '    "piston_head",\n    "moving_piston"', 1)
        atlas = atlas[:start] + seg + atlas[close:]
        print(f"  [{name}] pinned")

    # 3-6. the per-kind rows the N_KINDS length law keeps (the ONE row
    # the mid-state wears — the particle tile the fallback reads).
    atlas = append_table_row(
        atlas,
        "pub const FACE_TILES: [[u16; 6]; N_KINDS]",
        "    [920, 920, 920, 920, 920, 920],",
        "face_tiles row",
    )
    atlas = append_table_row(
        atlas,
        "pub const TINTS: [[u32; 6]; N_KINDS]",
        "    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],",
        "tints row",
    )
    atlas = append_table_row(
        atlas,
        "pub const DOOR_MODEL_BY_ROW: [&str; N_KINDS]",
        '    "",',
        "door_model row",
    )
    atlas = append_table_row(
        atlas,
        "pub const ENTITY_HEAD_FACES: [[u16; 6]; N_KINDS]",
        "    [65535, 65535, 65535, 65535, 65535, 65535],",
        "entity_head row",
    )

    open(ATLAS, "w").write(atlas)

    # 7. the world.rs NAME_OF row (the registry name the dig/sound/
    # resistance/drops tables key — the item pairing the stem answers).
    print("world.rs:")
    world = open(WORLD).read()
    world = edit_all(
        world,
        "pub const NAME_OF: [&str; 775] = [",
        "pub const NAME_OF: [&str; 776] = [",
        1,
        "name_of len",
    )
    seg = world.split("pub const NAME_OF: [&str; 776] = [")
    head, tail = seg[0], seg[1]
    close = tail.index("\n];")
    if '"moving_piston"' in tail[:close]:
        print("  [name_of tail] already pinned")
    else:
        tail = tail[:close] + '\n    "moving_piston",' + tail[close:]
        world = head + "pub const NAME_OF: [&str; 776] = [" + tail
        print("  [name_of tail] pinned")

    # 8. the count law the tests keep: the N_KINDS asserts + the
    # NAME_OF.len() asserts the same bump the kind seat wears.
    world, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)775(,)",
        r"\g<1>776\g<2>",
        world,
    )
    print(f"  [n_kinds asserts world.rs] {n} site(s) bumped")
    world, n = re.subn(
        r"(assert_eq!\(\s*NAME_OF\.len\(\), )775(,)",
        r"\g<1>776\g<2>",
        world,
    )
    print(f"  [name_of len asserts] {n} site(s) bumped")
    open(WORLD, "w").write(world)

    screens = open(CLIENT).read()
    screens, n = re.subn(
        r"(assert_eq!\(\s*atlas::N_KINDS,\s*)775(,)",
        r"\g<1>776\g<2>",
        screens,
    )
    print(f"  [n_kinds asserts screens.rs] {n} site(s) bumped")
    open(CLIENT, "w").write(screens)

    print("autopush129_moving_piston_pin: done")


if __name__ == "__main__":
    main()
