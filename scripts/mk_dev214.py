#!/usr/bin/env python3
"""dev-214 — THE TRIO'S OWN GENERATOR (the 416 round's own mk-lane the
round re-wears): the 422 pair the rename + the insert the dev-214's own
walk answers — the directed filter the swap key's own lane the test
wears, the fmt regions the swap key fn + the window's own inserts + the
round's own test the markers key. NO sed, NO grep: the pure-Python
transforms the -byte trap refuses. Usage: python3 scripts/mk_dev214.py"""

# --- directed_dev214.py: the 422 text, the rename + the head-insert ---
src = open("scripts/directed_422.py", encoding="utf-8").read()
src = src.replace("autopush-422", "dev-214")
src = src.replace("scripts/directed_422.py", "scripts/directed_dev214.py")
src = src.replace(
    'FILTERS = [\n    "the_brewing_stand_the_wire_title_wears",',
    'FILTERS = [\n    "the_brewing_stand_the_swap_key_wears",\n'
    '    "the_brewing_stand_the_wire_title_wears",',
)
assert "the_brewing_stand_the_swap_key_wears" in src, "the insert the head wears"
open("scripts/directed_dev214.py", "w", encoding="utf-8").write(src)
print("directed_dev214.py: written")

# --- fmt_probe_dev214.py: the 422 shape, the regions the dev-214 lanes re-key ---
probe = open("scripts/fmt_probe_422.py", encoding="utf-8").read()
probe = probe.replace("autopush-422", "dev-214")
probe = probe.replace("fmt_probe_422.py", "fmt_probe_dev214.py")
# the doc's own region list the dev-214 walk re-keys
old_doc = probe[probe.find("Regions:"):probe.find('"""', probe.find("Regions:"))]
new_doc = """Regions:
  screens.rs — the stand's own swap key fn (the prediction-free
               packet lane the checkHotbarKeyPressed gates wear — the
               dev-214 round's own edit).
  window.rs — the two inserts (the offhand 40 + the digits the swap
              chain the brewing arm joins).
  part_11.rs — the round's own test (the swap key's arms).
"""
probe = probe.replace(old_doc, new_doc)
# the SCREENS region list the swap key fn the only lane keeps
old_screens = probe[probe.find("SCREENS = ["):probe.find("]\n\nPART11 = [") + 1]
new_screens = """SCREENS = [
    # the stand's own swap key fn (the prediction-free packet lane
    # the checkHotbarKeyPressed gates wear)
    (
        "/// THE BREWING STAND'S OWN SWAP KEY (javap 26.2",
        "/// THE BREWING STAND'S OWN CLICK LANE (the offline sim's own walk",
    ),
]"""
probe = probe.replace(old_screens, new_screens)
# the WINDOW region list the two inserts the chain keys
old_regions = probe[probe.find("REGIONS = {"):probe.find("\n\n\ndef find")]
new_regions = """REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/window.rs": WINDOW,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}"""
probe = probe.replace(old_regions, new_regions)
# the WINDOW array the REGIONS map precedes — the inserts the markers key
window_block = """WINDOW = [
    # the offhand 40 insert (the swap chain the brewing arm joins)
    (
        "if !app.on_inventory_swap_key(40)",
        "app.on_swap_offhand_key();",
    ),
    # the digits insert (the same chain the digit arm joins)
    (
        "if !app.on_inventory_swap_key(i)",
        "&& !app.on_enchant_swap_key(i)",
    ),
]

"""
probe = probe.replace("REGIONS = {", window_block + "REGIONS = {", 1)
assert "WINDOW = [" in probe
assert "on_inventory_swap_key(i)" in probe
assert "on_inventory_swap_key(40)" in probe
open("scripts/fmt_probe_dev214.py", "w", encoding="utf-8").write(probe)
print("fmt_probe_dev214.py: written")
