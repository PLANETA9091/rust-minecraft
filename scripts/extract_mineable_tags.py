#!/usr/bin/env python3
"""Extract the block mineable tags (pickaxe/axe/shovel/hoe) from the
client.jar's data/minecraft/tags/block/mineable/*.json into a generated
Rust table: crates/mc-registry/src/generated/block_mineable.rs — and the
SHEARS rule rows (javap ShearsItem.createToolProperties: the cobweb
minesAndDrops(15.0) direct rule + the #minecraft:leaves 15.0 /
#minecraft:wool 5.0 / glow_lichen+vine 2.0 overrideSpeed rules) into
crates/mc-registry/src/generated/block_shears.rs.

The tag values carry nested `#minecraft:<tag>` references (walls, logs,
leaves, ...) — resolved recursively from the jar's own
data/minecraft/tags/block/<name>.json (cycle-guarded).

Usage: extract_mineable_tags.py [client.jar]
       (default /tmp/mc-verify/client.jar — re-downloadable per the
       RESUME block in worklog.md)
"""
import json
import sys
import zipfile
from pathlib import Path

JAR = Path(sys.argv[1] if len(sys.argv) > 1 else "/tmp/mc-verify/client.jar")
OUT = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/block_mineable.rs"
OUT_SHEARS = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/block_shears.rs"

# the class bytes the dig lane reads (the TOOL component's rule order)
CLASSES = [("pickaxe", 0), ("axe", 1), ("shovel", 2), ("hoe", 3)]


def load_tag(zf: zipfile.ZipFile, name: str, cache: dict, stack: tuple) -> list[str]:
    """data/minecraft/tags/block/<name>.json -> flat values (refs resolved)."""
    if name in cache:
        return cache[name]
    if name in stack:
        raise RuntimeError(f"tag cycle: {' -> '.join(stack + (name,))}")
    path = f"data/minecraft/tags/block/{name}.json"
    raw = json.loads(zf.read(path))
    out: list[str] = []
    for v in raw.get("values", []):
        if v.startswith("#"):
            out.extend(load_tag(zf, v[1:].removeprefix("minecraft:"), cache, stack + (name,)))
        else:
            out.append(v.removeprefix("minecraft:"))
    cache[name] = out
    return out


def main() -> None:
    with zipfile.ZipFile(JAR) as zf:
        cache: dict = {}
        table: dict[str, int] = {}
        for tag, cls in CLASSES:
            for block in load_tag(zf, f"mineable/{tag}", cache, ()):
                prev = table.get(block)
                if prev is not None and prev != cls:
                    print(f"  note: {block} in multiple mineable tags, keeping {prev}")
                    continue
                table[block] = cls

    names = sorted(table)
    idx = {n: i for i, n in enumerate(names)}

    lines: list[str] = []
    a = lines.append
    a("// GENERATED from the client.jar's data/minecraft/tags/block/mineable/")
    a("// {pickaxe,axe,shovel,hoe}.json (nested #minecraft:<tag> refs resolved")
    a("// from the jar's own tags) by scripts/extract_mineable_tags.py — the")
    a("// block->tool-class rows the dig lane's TOOL component walk reads")
    a("// (javap Item.getDestroySpeed -> Tool.getMiningSpeed: the first")
    a("// matching rule's speed, else defaultMiningSpeed 1.0).")
    a(f"pub const MINEABLE_BLOCK_COUNT: usize = {len(names)};")
    a("")
    a("/// Sorted for the binary search the lookup fn keeps (the index the")
    a("/// MINEABLE_SORTED permutation speaks — MINEABLE_NAMES itself stays")
    a("/// sorted).")
    a("pub const MINEABLE_NAMES: [&str; MINEABLE_BLOCK_COUNT] = [")
    for n in names:
        a(f'    "{n}",')
    a("];")
    a("")
    a("/// The tool class each name mines with (0 pickaxe, 1 axe, 2 shovel,")
    a("/// 3 hoe — the class bytes the dig lane's own consts speak).")
    a("pub const MINEABLE_CLASS: [u8; MINEABLE_BLOCK_COUNT] = [")
    a("    " + ", ".join(str(table[n]) for n in names))
    a("];")
    a("")
    a("/// The identity permutation (the names are already sorted — the")
    a("/// mirror the block_dig table's own shape keeps).")
    a("pub const MINEABLE_SORTED: [u16; MINEABLE_BLOCK_COUNT] = [")
    a("    " + ", ".join(str(i) for i in range(len(names))))
    a("];")
    a("")
    a("/// The block's mineable tool class (the None the undemanding block")
    a("/// the default rule rides).")
    a("pub fn mineable_class(name: &str) -> Option<u8> {")
    a("    let pos = MINEABLE_SORTED")
    a("        .binary_search_by(|probe| MINEABLE_NAMES[*probe as usize].cmp(name))")
    a("        .ok()? as usize;")
    a("    Some(MINEABLE_CLASS[MINEABLE_SORTED[pos] as usize])")
    a("}")
    OUT.write_text("\n".join(lines) + "\n")
    counts = {t: sum(1 for n in names if table[n] == c) for t, c in CLASSES}
    print(f"wrote {OUT} ({len(names)} blocks: {counts})")

    # ---- THE SHEARS' OWN RULES (javap ShearsItem.createToolProperties
    # pcs 7-82 — the rules the Tool ctor's List.of keeps IN ORDER: the
    # cobweb minesAndDrops(15.0) direct rule (speed 15.0, drops TRUE),
    # the #minecraft:leaves overrideSpeed(15.0), the
    # #minecraft:wool overrideSpeed(5.0), the
    # glow_lichen+vine overrideSpeed(2.0); the defaultMiningSpeed the
    # fconst_1 1.0 keeps). overrideSpeed carries NO drop verdict — the
    # cobweb's TRUE the minesAndDrops arm alone keeps.
    with zipfile.ZipFile(JAR) as zf2:
        cache2: dict = {}
        extreme = set(load_tag(zf2, "leaves", cache2, ()))
        major = set(load_tag(zf2, "wool", cache2, ()))
    minor = {"glow_lichen", "vine"}
    shears: dict[str, (float, bool)] = {"cobweb": (15.0, True)}
    for n in extreme:
        shears[n] = (15.0, False)
    for n in major:
        if n in shears and shears[n][1]:
            continue  # the first matching rule the walk keeps
        shears[n] = (5.0, False)
    for n in minor:
        if n in shears and shears[n][1]:
            continue
        shears[n] = (2.0, False)
    snames = sorted(shears)

    lines = []
    a = lines.append
    a("// GENERATED from the client.jar's tags (#minecraft:leaves,")
    a("// #minecraft:wool, the glow_lichen+vine minors, the cobweb direct")
    a("// rule javap ShearsItem.createToolProperties keeps) by")
    a("// scripts/extract_mineable_tags.py — the SHEARS TOOL component's")
    a("// rule rows the dig lane's getMiningSpeed walk reads: (speed,")
    a("// correct_for_drops) — the cobweb's TRUE the minesAndDrops arm")
    a("// keeps, the overrideSpeed rules the no-verdict False wear, the")
    a("// unmatched block the defaultMiningSpeed 1.0 default rides.")
    a(f"pub const SHEARS_BLOCK_COUNT: usize = {len(snames)};")
    a("")
    a("pub const SHEARS_NAMES: [&str; SHEARS_BLOCK_COUNT] = [")
    for n in snames:
        a(f'    "{n}",')
    a("];")
    a("")
    a("/// (speed, correct_for_drops) per name — the first matching rule")
    a("/// the component walk keeps.")
    a("pub const SHEARS_SPEED: [f32; SHEARS_BLOCK_COUNT] = [")
    a("    " + ", ".join(f"{shears[n][0]}" for n in snames))
    a("];")
    a("")
    a("pub const SHEARS_CORRECT: [bool; SHEARS_BLOCK_COUNT] = [")
    a("    " + ", ".join(str(shears[n][1]).lower() for n in snames))
    a("];")
    a("")
    a("/// The shears' own rule row (the None the unmatched block the")
    a("/// defaultMiningSpeed 1.0 default rides).")
    a("pub fn shears_rule(name: &str) -> Option<(f32, bool)> {")
    a("    let pos = SHEARS_NAMES.binary_search(&name).ok()?;")
    a("    Some((SHEARS_SPEED[pos], SHEARS_CORRECT[pos]))")
    a("}")
    OUT_SHEARS.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT_SHEARS} ({len(snames)} blocks)")

    # ---- THE SWORD'S OWN RULES (javap ToolMaterial.applySwordProperties
    # pcs 0-106 — the rules the Tool ctor's List.of keeps IN ORDER: the
    # cobweb minesAndDrops(15.0) direct rule (speed 15.0, drops TRUE),
    # the #minecraft:sword_instantly_mines overrideSpeed(3.4028235E38 —
    # the f32::MAX the ldc wears), the #minecraft:sword_efficient
    # overrideSpeed(1.5); the defaultMiningSpeed the fconst_1 1.0 keeps,
    # the ctor's last arg the iconst_0 false wears — the no-rule
    # fallback the Tool.isCorrectForDrops's own pc-67 iconst_0 answers).
    # overrideSpeed carries NO drop verdict — the cobweb's TRUE the
    # minesAndDrops arm alone keeps.
    with zipfile.ZipFile(JAR) as zf3:
        cache3: dict = {}
        instant = set(load_tag(zf3, "sword_instantly_mines", cache3, ()))
        efficient = set(load_tag(zf3, "sword_efficient", cache3, ()))
    sword: dict[str, (float, bool)] = {"cobweb": (15.0, True)}
    for n in instant:
        if n in sword and sword[n][1]:
            continue  # the first matching rule the walk keeps
        sword[n] = (3.4028235e38, False)
    for n in efficient:
        if n in sword and sword[n][1]:
            continue
        sword[n] = (1.5, False)
    snames2 = sorted(sword)

    lines = []
    a = lines.append
    a("// GENERATED from the client.jar's tags (#minecraft:sword_instantly_mines,")
    a("// #minecraft:sword_efficient, the cobweb direct rule javap")
    a("// ToolMaterial.applySwordProperties keeps) by")
    a("// scripts/extract_mineable_tags.py — the SWORD TOOL component's")
    a("// rule rows the dig lane's getMiningSpeed walk reads: (speed,")
    a("// correct_for_drops) — the cobweb's TRUE the minesAndDrops arm")
    a("// keeps, the overrideSpeed rules the no-verdict False wear, the")
    a("// unmatched block the defaultMiningSpeed 1.0 default rides.")
    a(f"pub const SWORD_BLOCK_COUNT: usize = {len(snames2)};")
    a("")
    a("pub const SWORD_NAMES: [&str; SWORD_BLOCK_COUNT] = [")
    for n in snames2:
        a(f'    "{n}",')
    a("];")
    a("")
    a("/// (speed, correct_for_drops) per name — the first matching rule")
    a("/// the component walk keeps.")
    a("pub const SWORD_SPEED: [f32; SWORD_BLOCK_COUNT] = [")
    a("    " + ", ".join(f"{sword[n][0]}" for n in snames2))
    a("];")
    a("")
    a("pub const SWORD_CORRECT: [bool; SWORD_BLOCK_COUNT] = [")
    a("    " + ", ".join(str(sword[n][1]).lower() for n in snames2))
    a("];")
    a("")
    a("/// The sword's own rule row (the None the unmatched block the")
    a("/// defaultMiningSpeed 1.0 default rides).")
    a("pub fn sword_rule(name: &str) -> Option<(f32, bool)> {")
    a("    let pos = SWORD_NAMES.binary_search(&name).ok()?;")
    a("    Some((SWORD_SPEED[pos], SWORD_CORRECT[pos]))")
    a("}")
    OUT_SWORD = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/block_sword.rs"
    OUT_SWORD.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT_SWORD} ({len(snames2)} blocks)")


if __name__ == "__main__":
    main()
