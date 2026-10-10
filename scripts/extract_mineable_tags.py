#!/usr/bin/env python3
"""Extract the block mineable tags (pickaxe/axe/shovel/hoe) from the
client.jar's data/minecraft/tags/block/mineable/*.json into a generated
Rust table: crates/mc-registry/src/generated/block_mineable.rs.

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


if __name__ == "__main__":
    main()
