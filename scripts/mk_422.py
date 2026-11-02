#!/usr/bin/env python3
"""autopush-422 — THE TRIO'S OWN GENERATOR (the 416 round's own mk-lane
the round re-wears): the 421 pair the rename + the insert the 422's own
walk answers — the directed filter the title's own lane the test wears,
the fmt regions the title read + the round's own test the markers key.
NO sed, NO grep: the pure-Python transforms the -byte trap refuses.
Usage: python3 scripts/mk_422.py"""

# --- directed_422.py: the 421 text, the rename + the head-insert ---
src = open("scripts/directed_421.py", encoding="utf-8").read()
src = src.replace("autopush-421", "autopush-422")
src = src.replace("scripts/directed_421.py", "scripts/directed_422.py")
src = src.replace(
    'FILTERS = [\n    "the_brewing_stand_the_wire_lane_wears",',
    'FILTERS = [\n    "the_brewing_stand_the_wire_title_wears",\n'
    '    "the_brewing_stand_the_wire_lane_wears",',
)
assert "the_brewing_stand_the_wire_title_wears" in src, "the insert the head wears"
open("scripts/directed_422.py", "w", encoding="utf-8").write(src)
print("directed_422.py: filters", src.count('",\n') + src.count('",\n    "'), "-> written")

# --- fmt_probe_422.py: the 421 shape, the regions the 422 lanes re-key ---
probe = open("scripts/fmt_probe_421.py", encoding="utf-8").read()
probe = probe.replace("autopush-421", "autopush-422")
probe = probe.replace("fmt_probe_421.py", "fmt_probe_422.py")
# the doc's own region list the 422 walk re-keys
old_doc = probe[probe.find("Regions:"):probe.find('"""', probe.find("Regions:"))]
new_doc = """Regions:
  screens.rs — the render's own title read (the live window's own
               name the furnace's own law wears — the 422 round's own
               edit).
  part_11.rs — the round's own test (the wire title's arms).
"""
probe = probe.replace(old_doc, new_doc)
# the SCREENS region list the title read the only lane keeps
old_screens = probe[probe.find("SCREENS = ["):probe.find("]\n\nPART11 = [") + 1]
new_screens = """SCREENS = [
    # the render's own title read (the live window's own name the
    # furnace's own law wears — the None the lang row the offline
    # canvas keeps)
    (
        "THE LIVE WINDOW'S OWN NAME (the furnace's own law the",
        "if let Some(t) = tr(\\"container.inventory\\")",
    ),
]"""
probe = probe.replace(old_screens, new_screens)
# the PART11 region list the round's own test the head keeps
old_p11 = probe[probe.find("PART11 = ["):probe.find("]\n\nREGIONS = {") + 1]
new_p11 = """PART11 = [
    # the round's own test (the wire title's arms)
    (
        "/// THE BREWING STAND'S OWN WIRE TITLE",
        None,
    ),
]"""
probe = probe.replace(old_p11, new_p11)
assert "THE LIVE WINDOW'S OWN NAME (the furnace's own law the" in probe
assert "/// THE BREWING STAND'S OWN WIRE TITLE" in probe
open("scripts/fmt_probe_422.py", "w", encoding="utf-8").write(probe)
print("fmt_probe_422.py: written")
