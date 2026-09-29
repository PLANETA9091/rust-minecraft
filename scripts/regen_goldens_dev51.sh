#!/bin/bash
# THE GOLDEN REGEN (dev-51) — the offhand lane's own re-baseline: the
# SAME args golden_matrix.sh renders every scene with, the output the
# goldens/ dir itself receives. The render CHANGED this round (the
# first-person OFFHAND lane — the bare LEFT arm the empty offhand
# renders, the FIRST_PERSON_LEFT_HAND item the filled one; the hands
# the screen==World gate hides under the container screens) — the task
# protocol: regenerate goldens/ and explain it in the commit.
# The NEW dual-wield golden (sword main + shield off) rides the same
# args the held variants speak.
set -eu
cd /home/z/my-project/rust-minecraft
BIN=target/release/mc-client

# the world scenes (the CI's own matrix: the seeds)
"$BIN" --headless --width 320 --height 180 --frames 1 --seed 20260214 --out goldens/cpu_320x180.png
"$BIN" --headless --width 480 --height 270 --frames 1 --seed 1337 --out goldens/cpu_480x270.png

# the net fixture scenes
"$BIN" --headless --net-world --width 320 --height 180 --frames 1 --out goldens/net_320x180.png
"$BIN" --headless --net-world --width 480 --height 270 --frames 1 --out goldens/net_480x270.png

# the GUI screens (the animated frames are part of the golden contract)
"$BIN" --screen loading    --width 320 --height 180 --ui-frame 15 --out goldens/gui_loading_320x180.png
"$BIN" --screen title      --width 320 --height 180 --ui-frame 30 --out goldens/gui_title_320x180.png
"$BIN" --screen servers    --width 320 --height 180 --out goldens/gui_servers_320x180.png
"$BIN" --screen pause      --width 320 --height 180 --ui-frame 30 --out goldens/gui_pause_320x180.png
"$BIN" --screen chest      --width 320 --height 180 --ui-frame 30 --out goldens/gui_chest_320x180.png
"$BIN" --screen world      --width 320 --height 180 --ui-frame 30 --out goldens/gui_world_320x180.png
"$BIN" --screen worldnight --width 320 --height 180 --ui-frame 30 --out goldens/gui_worldnight_320x180.png

# the held variants (the iron sword, the swing frozen mid-arc)
"$BIN" --screen world --width 320 --height 180 --ui-frame 30 \
  --held-item item/iron_sword --out goldens/gui_world_held_320x180.png
"$BIN" --screen world --width 320 --height 180 --ui-frame 30 \
  --held-item item/iron_sword --swing-progress 0.5 --out goldens/gui_world_held_swing_320x180.png

# THE NEW DUAL-WIELD GOLDEN (dev-51): the sword in the main hand, the
# shield in the offhand — the submitArmWithItem walk's two lanes the
# one frame wears (the F swap's own visual payoff)
"$BIN" --screen world --width 320 --height 180 --ui-frame 30 \
  --held-item item/iron_sword --offhand-item item/shield \
  --out goldens/gui_world_dual_320x180.png

echo "regen done: 14 goldens written"
