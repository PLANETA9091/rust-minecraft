#!/usr/bin/env python3
"""autopush-416 helper maker — the 415 pair the 416 round re-keys:
directed_416.py (the new filter the head inserts, 60 filters) +
fmt_probe_416.py (the hoist's own regions the chain re-keys)."""
import re

# --- directed_416.py: copy + rename + insert the round's own filter ---
src = open("scripts/directed_415.py", encoding="utf-8").read()
src = src.replace("autopush-415", "autopush-416")
src = src.replace("scripts/directed_415.py", "scripts/directed_416.py")
src = src.replace(
    'FILTERS = [\n',
    'FILTERS = [\n    "the_dispenser_shulker_the_place_law_wears",\n',
    1,
)
open("scripts/directed_416.py", "w", encoding="utf-8").write(src)

n = len(re.findall(r'^\s{4}"the_', src, flags=re.M))
print(f"directed_416.py filters={n}")

# --- fmt_probe_416.py: copy + rename + re-key the two chain markers ---
p = open("scripts/fmt_probe_415.py", encoding="utf-8").read()
p = p.replace("autopush-415", "autopush-416")
p = p.replace(
    "Regions:\n"
    "  world.rs      — the shulker box item's own kind predicate.\n"
    "  screens.rs    — the spill's own mark + the beat's own place arm +\n"
    "                  the eject gate's own ride.\n"
    "  part_11.rs    — the round's own test (the spill's arms).",
    "Regions:\n"
    "  world.rs      — the shulker box item's own kind predicate.\n"
    "  screens.rs    — the spill's own mark + the front read the hoist\n"
    "                  wears + the item-keyed dispatch order + the beat's\n"
    "                  own place arm + the eject gate's own ride.\n"
    "  part_11.rs    — the round's own test (the dispenser's arms).",
)
p = p.replace(
    '    # the spill\'s own mark (the entity gate\'s own flag the chain sets)\n'
    '    (\n'
    '        "            // THE SPILL\'S OWN MARK",\n'
    '        "            let (dispensed, voice) = if kind == crate::world::DROPPER {",\n'
    '    ),',
    '    # the spill\'s own mark + the front read + the registry order\n'
    '    # (the hoist the 416 round wears)\n'
    '    (\n'
    '        "            // THE SPILL\'S OWN MARK",\n'
    '        "            let (dispensed, voice) = if crate::world::is_shulker_box_item(item) {",\n'
    '    ),',
)
p = p.replace(
    '    # the beat\'s own place arm (the item-keyed law the walk wears)\n'
    '    (\n'
    '        "                // THE SHULKER BOX ITEM\'S OWN SPILL (javap",\n'
    '        "                } else if crate::world::is_chest(front_kind)",\n'
    '    ),',
    '    # the beat\'s own place arm (the item-keyed law the walk wears)\n'
    '    (\n'
    '        "                // THE SHULKER BOX ITEM\'S OWN SPILL (javap",\n'
    '        "                if crate::world::is_chest(front_kind)",\n'
    '    ),',
)
p = p.replace(
    '    # the round\'s own test (the spill\'s arms)\n'
    '    (\n'
    '        "/// THE DROPPER\'S OWN SHULKER SPILL",\n'
    '        None,\n'
    '    ),',
    '    # the round\'s own test (the dispenser\'s arms)\n'
    '    (\n'
    '        "/// THE DISPENSER\'S OWN SHULKER SPILL",\n'
    '        None,\n'
    '    ),',
)
open("scripts/fmt_probe_416.py", "w", encoding="utf-8").write(p)
for probe in (
    '"            // THE SPILL\'S OWN MARK",',
    '"            // THE SHULKER BOX ITEM\'S OWN SPILL (javap",',
    '"                if crate::world::is_chest(front_kind)",',
    '"/// THE DISPENSER\'S OWN SHULKER SPILL",',
):
    print(f"fmt_probe_416 marker {probe.strip()} present={probe in p}")
assert "let (dispensed, voice) = if kind ==" not in p, "stale 415 chain marker"
print("mk_416 done")
