#!/usr/bin/env python3
"""autopush-117: replace attach_wire_voice's inline twelve-cell scan
with the shared redstone_door_lane_flips helper (the same verdicts
the drain seat's own consult wears). Line-range splice with boundary
assertions so a stale range fails loudly instead of corrupting."""

PATH = "/home/z/my-project/rust-minecraft/crates/mc-client/src/screens.rs"

START = 38108  # 1-indexed: "            // THE TWELVE-CELL SCAN (javap ..."
END = 38278    # 1-indexed: gate apply close "            }"

REPLACEMENT = '''            // THE TWELVE-CELL SCAN (javap the updateNeighbours walk —
            // the changed cell's own six + the support cell's own six
            // the Direction.values() loop serves; the FLOOR face's own
            // DOWN arm the connected-direction law speaks. THIS ROUND
            // the scan's own verdicts the [redstone_door_lane_flips]
            // helper wears — the same seeds the scan's loops bank [the
            // changed cell's own six + the support cell's own six + the
            // wire flips' own cells' six] the helper's per-cell arms
            // answer, the trapdoor/gate/door consults the same `seen`
            // dedup the inline closure kept)
            let lane = {
                let read = &*world;
                let mut door_seeds: Vec<(i32, i32, i32)> =
                    vec![(lx, ly, lz), (lx, ly - 1, lz)];
                door_seeds.extend(wire_cells.iter().copied());
                crate::world::redstone_door_lane_flips(read, &door_seeds)
            };
            door_flips = lane.doors;
            trapdoor_flips = lane.trapdoors;
            gate_flips = lane.gates;
            for ((dx, dy, dz), flip) in door_flips.iter() {
                world.set(*dx, *dy, *dz, *flip);
            }
            // THE TRAPDOOR'S OWN APPLY (the OPEN-sync verdicts the
            // setBlock walk banks — the neighborChanged law the
            // single-cell flip serves)
            for ((dx, dy, dz), flip) in trapdoor_flips.iter() {
                world.set(*dx, *dy, *dz, *flip);
            }
            // THE FENCE GATE'S OWN APPLY (the OPEN-sync verdicts the
            // setBlock walk banks — the neighborChanged law the
            // single-cell flip serves)
            for ((dx, dy, dz), flip) in gate_flips.iter() {
                world.set(*dx, *dy, *dz, *flip);
            }
'''

with open(PATH, "r", encoding="utf-8") as f:
    lines = f.readlines()

# boundary assertions (1-indexed -> 0-indexed)
assert lines[START - 1].startswith(
    "            // THE TWELVE-CELL SCAN"
), f"bad start boundary: {lines[START - 1]!r}"
assert lines[END - 1].rstrip() == "            }", (
    f"bad end boundary: {lines[END - 1]!r}"
)
assert lines[END].startswith(
    "            // THE LAMP'S OWN CONSUMER ARM"
), f"bad post boundary: {lines[END]!r}"
assert lines[START - 2].rstrip() == "            }", (
    f"bad pre boundary: {lines[START - 2]!r}"
)

new_lines = lines[: START - 1] + [REPLACEMENT] + lines[END:]

with open(PATH, "w", encoding="utf-8") as f:
    f.writelines(new_lines)

print(f"spliced lines {START}-{END}: {END - START + 1} lines -> "
      f"{REPLACEMENT.count(chr(10))} lines")
