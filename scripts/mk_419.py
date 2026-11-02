#!/usr/bin/env python3
"""autopush-419 helper maker — the 418 pair the 419 round re-keys:
directed_419.py (the new filter the head inserts, 63 filters) +
fmt_probe_419.py (the echo's own regions the chain re-keys)."""
import re

# --- directed_419.py: copy + rename + insert the round's own filter ---
src = open("scripts/directed_418.py", encoding="utf-8").read()
src = src.replace("autopush-418", "autopush-419")
src = src.replace("scripts/directed_418.py", "scripts/directed_419.py")
src = src.replace(
    'FILTERS = [\n',
    'FILTERS = [\n    "the_enchant_pickup_the_wire_echo_wears",\n',
    1,
)
open("scripts/directed_419.py", "w", encoding="utf-8").write(src)

n = len(re.findall(r'^\s{4}"the_', src, flags=re.M))
print(f"directed_419.py filters={n}")

# --- fmt_probe_419.py: copy + rename + re-key the regions ---
p = open("scripts/fmt_probe_418.py", encoding="utf-8").read()
p = p.replace("autopush-418", "autopush-419")
p = p.replace(
    "Regions:\n"
    "  screens.rs — the wire sync arm's own fresh-state reset + the costs'\n"
    "               data-word walk + the clue rows' own pick.\n"
    "  part_11.rs — the round's own test (the data words' arms).",
    "Regions:\n"
    "  screens.rs — the click lane's own head the live gate re-keys +\n"
    "               the pickup's own wire echo (the anvil's dev-202 shape\n"
    "               the enchant menu re-wears).\n"
    "  part_11.rs — the round's own test (the echo's arms).",
)
p = p.replace(
    'SCREENS = [\n'
    '    # the wire sync arm\'s own data-word lane (the 418 round\'s walk)\n'
    '    (\n'
    '        "                    // THE FRESH STATE\'S OWN RESET (the open walk\'s own",\n'
    '        "                    // the window\'s identity rides in: the gather echoes",\n'
    '    ),\n'
    ']',
    'SCREENS = [\n'
    '    # the click lane\'s own head (the live gate the seat resolution\n'
    '    # re-keys — the 419 round\'s own walk)\n'
    '    (\n'
    '        "fn click_enchanting_slot(&mut self, mx: i32, my: i32, button: i8) {",\n'
    '        "        // THE ENCHANT PICKUP\'S OWN WIRE ECHO",\n'
    '    ),\n'
    '    # the pickup\'s own wire echo (the anvil\'s dev-202 shape the\n'
    '    # enchant menu re-wears)\n'
    '    (\n'
    '        "        // THE ENCHANT PICKUP\'S OWN WIRE ECHO",\n'
    '        "                    carried: self.enchant_carried,",\n'
    '    ),\n'
    ']',
)
p = p.replace(
    'PART11 = [\n'
    '    # the round\'s own test (the data words\' arms)\n'
    '    (\n'
    '        "/// THE ENCHANT COSTS\' OWN DATA WORDS",\n'
    '        None,\n'
    '    ),\n'
    ']',
    'PART11 = [\n'
    '    # the round\'s own test (the echo\'s arms)\n'
    '    (\n'
    '        "/// THE ENCHANT PICKUP\'S OWN WIRE ECHO",\n'
    '        None,\n'
    '    ),\n'
    ']',
)
open("scripts/fmt_probe_419.py", "w", encoding="utf-8").write(p)
for probe in (
    '"fn click_enchanting_slot(&mut self, mx: i32, my: i32, button: i8) {",',
    '"        // THE ENCHANT PICKUP\'S OWN WIRE ECHO",',
    '"                    carried: self.enchant_carried,",',
    '"/// THE ENCHANT PICKUP\'S OWN WIRE ECHO",',
):
    assert probe in p, f"probe marker missing: {probe}"
print("fmt_probe_419.py regions re-keyed (4 markers verified)")
assert "the_enchant_pickup_the_wire_echo_wears" in src, "filter missing"
assert '"the_enchant_costs_the_data_words_wear"' in src, "prior filter kept"
print("mk_419: both files the round's own shape wears")
