#!/usr/bin/env bash
# verify_goldens.sh — the FULL golden matrix in one command.
#
# Every golden in goldens/ is re-rendered here at its EXACT recorded
# flags (the same flags the README/worklog pins were verified with) and
# diffed through mc2rs-pixel against the checked-in reference:
#   - the two CPU orbit-camera scenes and the two NETWORK wire scenes
#     gate at max-rms 8 (the shared-float-raster tolerance),
#   - the eight GUI screens (six originals + the held-item scene + the
#     held-swing scene) gate at max-rms 0 — the GUI path is
#     bit-deterministic, so any pixel drift at all is a regression.
#
# A render that disagrees with its golden fails the script; NOTHING is
# regenerated automatically (regenerating a golden is a deliberate,
# documented act — see the pixel-goldens policy).
#
# Usage:  ./verify_goldens.sh            (from the repo root)
# Needs:  cargo (release build of mc-client + mc2rs-pixel), the
#         assets-cache/ runtime assets for the GUI screens.
set -euo pipefail

cd "$(dirname "$0")"

export PATH="$HOME/.cargo/bin:$PATH"

BIN_CLIENT=target/release/mc-client
BIN_PIXEL=target/release/mc2rs-pixel
WORK=target/golden_verify

echo "== building release binaries =="
cargo build --release -p mc-client -p mc2rs-pixel

mkdir -p "$WORK"

pass=0
fail=0

# check <name> <max_rms> <client flags...>
check() {
    local name="$1"; shift
    local max_rms="$1"; shift
    local golden="goldens/${name}.png"
    local render="${WORK}/${name}.png"

    if [[ ! -f "$golden" ]]; then
        echo "FAIL  ${name}  (golden missing: ${golden} — render and commit it deliberately)"
        fail=$((fail+1))
        return
    fi

    if ! "$BIN_CLIENT" --headless "$@" --out "$render" >/dev/null 2>&1; then
        echo "FAIL  ${name}  (render failed)"
        fail=$((fail+1))
        return
    fi

    local line
    line=$("$BIN_PIXEL" --a "$render" --b "$golden" --max-rms "$max_rms" 2>&1 | tail -2 | head -1)
    local rms
    rms=$(printf '%s' "$line" | sed -n 's/.*"rms":\([0-9.]*\),.*/\1/p')
    if printf '%s' "$line" | grep -q '"pass":true'; then
        echo "PASS  ${name}  rms ${rms} (max ${max_rms})"
        pass=$((pass+1))
    else
        echo "FAIL  ${name}  ${line:-no verdict}"
        fail=$((fail+1))
    fi
}

echo "== golden matrix =="
# --- the CPU reference world (procedural, orbit camera) ---
check cpu_320x180 8 --width 320 --height 180 --frames 1 --seed 20260214
check cpu_480x270 8 --width 480 --height 270 --frames 1 --seed 1337
# --- the NETWORK wire world (real packet bytes -> parse -> materialize) ---
check net_320x180 8 --net-world --width 320 --height 180 --frames 1 --seed 20260214
check net_480x270 8 --net-world --width 480 --height 270 --frames 1 --seed 20260214
# --- the GUI screens (bit-deterministic, max-rms 0) ---
check gui_title_320x180 0 --screen title --ui-frame 30 --seed 20260214 --width 320 --height 180
check gui_servers_320x180 0 --screen servers --ui-frame 60 --seed 20260214 --width 320 --height 180
check gui_loading_320x180 0 --screen loading --ui-frame 15 --seed 20260214 --width 320 --height 180
check gui_world_320x180 0 --screen world --ui-frame 30 --seed 20260214 --width 320 --height 180
check gui_worldnight_320x180 0 --screen worldnight --ui-frame 30 --seed 20260214 --width 320 --height 180
check gui_pause_320x180 0 --screen pause --ui-frame 30 --seed 20260214 --width 320 --height 180
# --- the held-item scene (the display arc's persistent pixel coverage) ---
check gui_world_held_320x180 0 --screen world --ui-frame 30 --seed 20260214 --held-item item/iron_sword --width 320 --height 180
# --- the held-swing scene (the swing chain pinned at a fixed progress) ---
check gui_world_held_swing_320x180 0 --screen world --ui-frame 30 --seed 20260214 --held-item item/iron_sword --swing-progress 0.5 --width 320 --height 180

echo "== ${pass} passed, ${fail} failed =="
if [[ "$fail" -gt 0 ]]; then
    echo "PIXEL TEST FAILED (see above; diffs land in ${WORK}/)"
    exit 1
fi
echo "ALL GOLDENS PASS"
