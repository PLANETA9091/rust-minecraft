#!/usr/bin/env python3
"""dev-215 — THE TRIO'S OWN GENERATOR (the 416 round's own mk-lane the
round re-wears): the dev-214 pair the rename + the insert the dev-215's
own walk answers — the directed filter the hopper title's own lane the
test wears, the fmt regions the render's own read + the round's own
test the markers key (the window's own inserts the dev-214 regions the
retire drops — the chain the round never touches). NO sed, NO grep:
the pure-Python transforms the -byte trap refuses. Usage: python3
scripts/mk_dev215.py"""

# --- directed_dev215.py: the dev-214 text, the rename + the head-insert ---
src = open("scripts/directed_dev214.py", encoding="utf-8").read()
src = src.replace("dev-214", "dev-215")
src = src.replace("scripts/directed_dev214.py", "scripts/directed_dev215.py")
src = src.replace(
    'FILTERS = [\n    "the_brewing_stand_the_swap_key_wears",',
    'FILTERS = [\n    "the_hopper_the_wire_title_wears",\n'
    '    "the_brewing_stand_the_swap_key_wears",',
)
assert "the_hopper_the_wire_title_wears" in src, "the insert the head wears"
open("scripts/directed_dev215.py", "w", encoding="utf-8").write(src)
print("directed_dev215.py: written")

# --- fmt_probe_dev215.py: the dev-214 shape, the regions re-keyed ---
probe = open("scripts/fmt_probe_dev214.py", encoding="utf-8").read()
probe = probe.replace("dev-214", "dev-215")
probe = probe.replace("fmt_probe_dev214.py", "fmt_probe_dev215.py")
# the doc's own region list the dev-215 walk re-keys
old_doc = probe[probe.find("Regions:"):probe.find('"""', probe.find("Regions:"))]
new_doc = """Regions:
  screens.rs — the hopper's own title read (the live window's own
               name the chest's own law wears — the dev-215 round's
               own edit).
  part_11.rs — the round's own test (the hopper title's arms).
"""
probe = probe.replace(old_doc, new_doc)
# the SCREENS region list the hopper title read the only lane keeps
old_screens = probe[probe.find("SCREENS = ["):probe.find("]\n\nPART11 = [") + 1]
new_screens = """SCREENS = [
    # the hopper's own title read (the live window's own name the
    # chest's own law wears — the None the lang row the offline
    # canvas keeps)
    (
        "THE LIVE WINDOW'S OWN NAME (the chest's own law the",
        "if let Some(t) = tr(\\"container.inventory\\")",
    ),
]"""
probe = probe.replace(old_screens, new_screens)
# the PART11 region the head marker the EOF walk keeps (the round's
# own test the tail the same EOF region covers)
# the WINDOW array the dev-215 round never touches — the region the
# REGIONS map the screens + part_11 pair the lane keeps
old_regions = probe[probe.find("WINDOW = ["):probe.find("\n\n\ndef find")]
new_regions = """REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}"""
probe = probe.replace(old_regions, new_regions)
# the REGIONS map's window.rs entry the retire takes (the map the
# dev-214 lanes wrote — the hopper round the two-file map wears)
probe = probe.replace(
    '    "crates/mc-client/src/window.rs": WINDOW,\n', "", 1
)
assert "THE LIVE WINDOW'S OWN NAME (the chest's own law the" in probe
assert '"crates/mc-client/src/window.rs"' not in probe
open("scripts/fmt_probe_dev215.py", "w", encoding="utf-8").write(probe)
print("fmt_probe_dev215.py: written")
