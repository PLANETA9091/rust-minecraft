#!/usr/bin/env python3
"""autopush-421 — THE TRIO'S OWN GENERATOR (the 416 round's own mk-lane
the round re-wears): the 420 pair the rename + the insert the 421's own
walk answers — the directed filter the stand's own lane the test wears,
the fmt regions the close + the click + the lane + the round's own test
the markers key. NO sed, NO grep: the pure-Python transforms the -byte
trap refuses. Usage: python3 scripts/mk_421.py"""

# --- directed_421.py: the 420 text, the rename + the head-insert ---
src = open("scripts/directed_420.py", encoding="utf-8").read()
src = src.replace("autopush-420", "autopush-421")
src = src.replace("scripts/directed_420.py", "scripts/directed_421.py")
src = src.replace(
    'FILTERS = [\n    "the_enchant_option_the_wire_click_wears",',
    'FILTERS = [\n    "the_brewing_stand_the_wire_lane_wears",\n'
    '    "the_enchant_option_the_wire_click_wears",',
)
assert "the_brewing_stand_the_wire_lane_wears" in src, "the insert the head wears"
open("scripts/directed_421.py", "w", encoding="utf-8").write(src)

# --- fmt_probe_421.py: the 420 shape, the regions the 421 lanes re-key ---
probe = open("scripts/fmt_probe_420.py", encoding="utf-8").read()
probe = probe.replace("autopush-420", "autopush-421")
probe = probe.replace("fmt_probe_420.py", "fmt_probe_421.py")
# the doc's own region list the 421 walk re-keys
probe = probe.replace(
    """Regions:
  screens.rs — the option lane's own walk (the live gate the echo
               the silence retires — the 420 round's own edit).
  part_11.rs — the round's own test (the option click's arms).
""",
    """Regions:
  screens.rs — the close's own retire + the click's own wire echo +
               the stand's own wire lane (the live gate the honest
               drop retires — the 421 round's own edits).
  part_11.rs — the round's own test (the stand lane's arms).
""",
)
# the SCREENS list the three lanes' own pairs wear
probe = probe.replace(
    """SCREENS = [
    # the option lane's own walk (the live gate the echo the silence
    # retires — the 420 round's own edit, the offline head the bound
    # keeps)
    (
        "pub fn click_enchant_option(&mut self, id: usize) -> bool {",
        "        let i = id + 1;",
    ),
]""",
    """SCREENS = [
    # the close's own retire (the live window's own close the flag
    # retires — the 421 round's own head, the offline ride the tail
    # keeps)
    (
        "pub fn close_brewing(&mut self, back: ScreenKind) {",
        "        if self.brewing_carried.1 > 0 {",
    ),
    # the click's own wire echo (the prediction-free lane the
    # enchant's own walk banks — the offline head the live arm
    # precedes)
    (
        "fn click_brewing_slot(&mut self, mx: i32, my: i32, button: i8) {",
        "        let menu = slot as usize;",
    ),
    # the stand's own wire lane (the menu type 11 the honest drop
    # retires — the projection the wholesale law wears)
    (
        "if c.menu_type_id == crate::world_model::MENU_TYPE_BREWING_STAND {",
        "if c.menu_type_id != crate::world_model::MENU_TYPE_CHEST",
    ),
]""",
)
assert "MENU_TYPE_BREWING_STAND {" in probe, "the lane's own pair the list wears"
# the PART11 list the round's own test the marker keys
probe = probe.replace(
    """PART11 = [
    # the round's own test (the option click's arms)
    (
        "/// THE ENCHANT OPTION'S OWN WIRE CLICK",
        None,
    ),
]""",
    """PART11 = [
    # the round's own test (the stand lane's arms)
    (
        "/// THE BREWING STAND'S OWN WIRE LANE",
        None,
    ),
]""",
)
assert "/// THE BREWING STAND'S OWN WIRE LANE" in probe, "the test's own marker the list wears"
open("scripts/fmt_probe_421.py", "w", encoding="utf-8").write(probe)
print("mk_421: directed_421 + fmt_probe_421 generated")
