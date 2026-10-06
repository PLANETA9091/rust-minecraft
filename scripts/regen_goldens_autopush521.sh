#!/bin/bash
# THE GOLDEN REGEN (autopush-521) — the fluids' animate lane's own
# re-baseline: the SAME args verify_goldens.sh renders every scene
# with, the output the goldens/ dir itself receives. The render CHANGED
# this round (the water/lava surface quads the anim_tile remap repaints
# at the App's live tick — the vanilla-faithful animated stills the
# mcmeta's own frametime-2 cadence plays): the world-riding GUI scenes
# (ui-frame 30 -> the water frame the tick answers) re-pin the animated
# frame; the orbit scenes (the headless render's fire_anim_tick = 0 —
# the tick-0 still the frame-0 pixels wear) re-render byte-identical.
# The task protocol: regenerate goldens/ and explain it in the commit.
set -eu
cd "$(dirname "$0")/.."
BIN=target/release/mc-client
mkdir -p goldens

# the CPU reference world (procedural, orbit camera) — anim tick 0
"$BIN" --headless --width 320 --height 180 --frames 1 --seed 20260214 --out goldens/cpu_320x180.png
"$BIN" --headless --width 480 --height 270 --frames 1 --seed 1337 --out goldens/cpu_480x270.png

# the NETWORK wire world
"$BIN" --headless --net-world --width 320 --height 180 --frames 1 --seed 20260214 --out goldens/net_320x180.png
"$BIN" --headless --net-world --width 480 --height 270 --frames 1 --seed 20260214 --out goldens/net_480x270.png

# the GUI screens (bit-deterministic, max-rms 0)
"$BIN" --screen title --ui-frame 30 --seed 20260214 --width 320 --height 180 --out goldens/gui_title_320x180.png
"$BIN" --screen servers --ui-frame 60 --seed 20260214 --width 320 --height 180 --out goldens/gui_servers_320x180.png
"$BIN" --screen loading --ui-frame 15 --seed 20260214 --width 320 --height 180 --out goldens/gui_loading_320x180.png
"$BIN" --screen world --ui-frame 30 --seed 20260214 --width 320 --height 180 --out goldens/gui_world_320x180.png
"$BIN" --screen worldnight --ui-frame 30 --seed 20260214 --width 320 --height 180 --out goldens/gui_worldnight_320x180.png
"$BIN" --screen pause --ui-frame 30 --seed 20260214 --width 320 --height 180 --out goldens/gui_pause_320x180.png
"$BIN" --screen world --ui-frame 30 --seed 20260214 --held-item item/iron_sword --width 320 --height 180 --out goldens/gui_world_held_320x180.png
"$BIN" --screen world --ui-frame 30 --seed 20260214 --held-item item/iron_sword --swing-progress 0.5 --width 320 --height 180 --out goldens/gui_world_held_swing_320x180.png
"$BIN" --screen chest --ui-frame 30 --seed 20260214 --width 320 --height 180 --out goldens/gui_chest_320x180.png

echo "== regen complete (13 scenes) =="
