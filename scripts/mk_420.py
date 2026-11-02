#!/usr/bin/env python3
"""autopush-420 — THE TRIO'S OWN GENERATOR (the 416 round's own mk-lane
the round re-wears): the 419 pair the rename + the insert the 420's own
walk answers — the directed filter the option click's own test wears,
the fmt region the option lane + the round's own test the markers key.
NO sed, NO grep: the pure-Python transforms the [m-byte trap refuses.
Usage: python3 scripts/mk_420.py"""

import re

# --- directed_420.py: the 419 text, the rename + the head-insert ---
src = open("scripts/directed_419.py", encoding="utf-8").read()
src = src.replace("autopush-419", "autopush-420")
src = src.replace("scripts/directed_419.py", "scripts/directed_420.py")
src = src.replace(
    'FILTERS = [\n    "the_enchant_pickup_the_wire_echo_wears",',
    'FILTERS = [\n    "the_enchant_option_the_wire_click_wears",\n'
    '    "the_enchant_pickup_the_wire_echo_wears",',
)
assert "the_enchant_option_the_wire_click_wears" in src, "the insert the head wears"
open("scripts/directed_420.py", "w", encoding="utf-8").write(src)

# --- fmt_probe_420.py: the 419 shape, the regions the 420 lane re-keys ---
probe = open("scripts/fmt_probe_419.py", encoding="utf-8").read()
probe = probe.replace("autopush-419", "autopush-420")
probe = probe.replace("fmt_probe_419.py", "fmt_probe_420.py")
# the doc's own region list the 420 walk re-keys
probe = probe.replace(
    """Regions:
  screens.rs — the click lane's own head the live gate re-keys +
               the pickup's own wire echo (the anvil's dev-202 shape
               the enchant menu re-wears).
  part_11.rs — the round's own test (the echo's arms).
""",
    """Regions:
  screens.rs — the option lane's own walk (the live gate the echo
               the silence retires — the 420 round's own edit).
  part_11.rs — the round's own test (the option click's arms).
""",
)
# the SCREENS list the option lane's own pair wears
probe = probe.replace(
    """SCREENS = [
    # the click lane's own head (the live gate the seat resolution
    # re-keys — the 419 round's own walk)
    (
        "fn click_enchanting_slot(&mut self, mx: i32, my: i32, button: i8) {",
        "        // THE ENCHANT PICKUP'S OWN WIRE ECHO",
    ),
    # the pickup's own wire echo (the anvil's dev-202 shape the
    # enchant menu re-wears)
    (
        "        // THE ENCHANT PICKUP'S OWN WIRE ECHO",
        "                    carried: self.enchant_carried,",
    ),
]""",
    """SCREENS = [
    # the option lane's own walk (the live gate the echo the silence
    # retires — the 420 round's own edit, the offline head the bound
    # keeps)
    (
        "pub fn click_enchant_option(&mut self, id: usize) -> bool {",
        "        let i = id + 1;",
    ),
]""",
)
# the PART11 list the round's own test wears
probe = probe.replace(
    """PART11 = [
    # the round's own test (the echo's arms)
    (
        "/// THE ENCHANT PICKUP'S OWN WIRE ECHO",
        None,
    ),
]""",
    """PART11 = [
    # the round's own test (the option click's arms)
    (
        "/// THE ENCHANT OPTION'S OWN WIRE CLICK",
        None,
    ),
]""",
)
assert "click_enchant_option" in probe, "the screens region the lane wears"
assert "THE ENCHANT OPTION'S OWN WIRE CLICK" in probe, "the test region wears"
open("scripts/fmt_probe_420.py", "w", encoding="utf-8").write(probe)

# --- the sanity walk: the generated pair the fresh files read ---
d = open("scripts/directed_420.py", encoding="utf-8").read()
n = len(re.findall(r'^    "[a-z0-9_]+",$', d, re.M))
print(f"mk_420: directed_420.py {n} filters, fmt_probe_420.py regions re-keyed")
