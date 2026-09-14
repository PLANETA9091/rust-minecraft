#!/bin/bash
# THE GOLDEN MATRIX — the EXACT args every golden was captured with
# (reconstructed and verified rms 0.000 in the sound round, v0.3.148).
# The screens animate: the loading screen's boot progress rides ui-frame
# 15, the world-family screens' sky/cloud cycle aligns at ui-frame 30,
# the title's panorama spin aligns at 30 too. The held variants hold the
# iron sword at swing progress 0.5 (the headless doc's own example).
set -u
cd /home/z/my-project/rust-minecraft
BIN=target/release/mc-client
PIX=target/release/mc2rs-pixel
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
pass=0; fail=0

check() { # check <golden> <rendered>
  local g="$1" r="$2"
  local line
  line=$("$PIX" --a "$r" --b "goldens/$g" --out "$OUT/d.png" 2>/dev/null | rg -o '"rms":[0-9.]+' | cut -d: -f2)
  if [ "${line:-1}" = "0.000" ]; then
    echo "PASS $g :: rms 0.000"
    pass=$((pass+1))
  else
    echo "FAIL $g :: rms ${line:-NO-READ}"
    fail=$((fail+1))
  fi
}

# the world scenes (the CI's own matrix: the seeds)
"$BIN" --headless --width 320 --height 180 --frames 1 --seed 20260214 --out "$OUT/r.png" >/dev/null 2>&1
check cpu_320x180.png "$OUT/r.png"
"$BIN" --headless --width 480 --height 270 --frames 1 --seed 1337 --out "$OUT/r.png" >/dev/null 2>&1
check cpu_480x270.png "$OUT/r.png"

# the net fixture scenes
"$BIN" --headless --net-world --width 320 --height 180 --frames 1 --out "$OUT/r.png" >/dev/null 2>&1
check net_320x180.png "$OUT/r.png"
"$BIN" --headless --net-world --width 480 --height 270 --frames 1 --out "$OUT/r.png" >/dev/null 2>&1
check net_480x270.png "$OUT/r.png"

# the GUI screens (the animated frames are part of the golden contract)
"$BIN" --screen loading    --width 320 --height 180 --ui-frame 15 --out "$OUT/r.png" >/dev/null 2>&1
check gui_loading_320x180.png "$OUT/r.png"
"$BIN" --screen title      --width 320 --height 180 --ui-frame 30 --out "$OUT/r.png" >/dev/null 2>&1
check gui_title_320x180.png "$OUT/r.png"
"$BIN" --screen servers    --width 320 --height 180 --out "$OUT/r.png" >/dev/null 2>&1
check gui_servers_320x180.png "$OUT/r.png"
"$BIN" --screen pause      --width 320 --height 180 --ui-frame 30 --out "$OUT/r.png" >/dev/null 2>&1
check gui_pause_320x180.png "$OUT/r.png"
"$BIN" --screen chest      --width 320 --height 180 --ui-frame 30 --out "$OUT/r.png" >/dev/null 2>&1
check gui_chest_320x180.png "$OUT/r.png"
"$BIN" --screen world      --width 320 --height 180 --ui-frame 30 --out "$OUT/r.png" >/dev/null 2>&1
check gui_world_320x180.png "$OUT/r.png"
"$BIN" --screen worldnight --width 320 --height 180 --ui-frame 30 --out "$OUT/r.png" >/dev/null 2>&1
check gui_worldnight_320x180.png "$OUT/r.png"

# the held variants (the iron sword, the swing frozen mid-arc)
"$BIN" --screen world --width 320 --height 180 --ui-frame 30 \
  --held-item item/iron_sword --out "$OUT/r.png" >/dev/null 2>&1
check gui_world_held_320x180.png "$OUT/r.png"
"$BIN" --screen world --width 320 --height 180 --ui-frame 30 \
  --held-item item/iron_sword --swing-progress 0.5 --out "$OUT/r.png" >/dev/null 2>&1
check gui_world_held_swing_320x180.png "$OUT/r.png"

echo "----"
echo "matrix: $pass passed / $fail failed / 13 total"
[ "$fail" = "0" ]
