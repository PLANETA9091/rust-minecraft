#!/usr/bin/env bash
# verify_goldens.sh — the FULL golden matrix in one command: ALL 19
# goldens, the runner the repo itself owns (the pre-137 rounds ran it
# from the sandbox copy; the golden_matrix.sh companion keeps the 14
# core rows for a quick pass).
#
# Every golden in goldens/ is re-rendered here at its EXACT recorded
# flags (the same flags the README/worklog pins were verified with) and
# diffed through mc2rs-pixel against the checked-in reference:
#   - the two CPU orbit-camera scenes and the two NETWORK wire scenes
#     gate at max-rms 8 (the shared-float-raster tolerance),
#   - the fifteen GUI screens (the six originals + the held-item,
#     held-swing and dual-wield world scenes + the chest, double-chest,
#     placed-pair, furnace, lit-furnace and creative-picker canvases)
#     gate at max-rms 0 — the GUI path is bit-deterministic, so any
#     pixel drift at all is a regression.
#
# A render that disagrees with its golden fails the script; NOTHING is
# regenerated automatically (regenerating a golden is a deliberate,
# documented act — see the pixel-goldens policy).
#
# Usage:  scripts/verify_goldens.sh      (from the repo root; the
#         root ./verify_goldens.sh shim execs this same file)
# Needs:  cargo (release build of mc-client + mc2rs-pixel), the
#         assets-cache/ runtime assets for the GUI screens.
set -euo pipefail

cd "$(dirname "$0")/.."

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
# --- the dual-wield scene (dev-51: the offhand lane's own golden — the
# --- sword the main hand wears, the shield the offhand carries, the two
# --- submitArmWithItem lanes the one frame renders) ---
check gui_world_dual_320x180 0 --screen world --ui-frame 30 --held-item item/iron_sword --offhand-item item/shield --width 320 --height 180
# --- the chest screen (the container GUI's 9x3 grid round) ---
check gui_chest_320x180 0 --screen chest --ui-frame 30 --seed 20260214 --width 320 --height 180
# --- the double-chest screen (the dev-71 compound's own 9x6 canvas, the partner's rows the head keeps) ---
check gui_chest_double_320x180 0 --screen chest_double --ui-frame 30 --seed 20260214 --width 320 --height 180
# --- the lit furnace (the dev-72 world walk's own pin: the stocked furnace the settle ticks ignite, the LIT flip the emission-13 light the embers wear) ---
check gui_furnace_lit_320x180 0 --screen furnace_lit --body --body-yaw 0 --body-pitch=-12 --ui-frame 30 --settle-ticks 15 --seed 20260214 --width 320 --height 180
# --- the furnace menu (the dev-69 FurnaceMenu's own 176x166 canvas, the trio the slots speak, the 50% flame the arrow the mid-arc words wear) ---
check gui_furnace_320x180 0 --screen furnace --ui-frame 30 --seed 20260214 --width 320 --height 180
# --- the placed chest pair (the dev-78 world-side pin: the LEFT/RIGHT pair the mesher's double arms render, the joined body the two latches the plateau edge wears) ---
check gui_chest_pair_320x180 0 --screen chest_pair --body --body-yaw 0 --body-pitch=-12 --ui-frame 30 --seed 20260214 --width 320 --height 180
# --- the creative picker (the dev-79 pin: the dev-30 CreativeModeInventoryScreen's own canvas — the 195x136 panel the 14-tab strip wears, the 45-slot palette the building-blocks tab speaks, the hotbar row the demo loadout paints) ---
check gui_creative_320x180 0 --screen creative --ui-frame 30 --seed 20260214 --width 320 --height 180

echo "== ${pass} passed, ${fail} failed =="
if [[ "$fail" -gt 0 ]]; then
    echo "PIXEL TEST FAILED (see above; diffs land in ${WORK}/)"
    exit 1
fi
echo "ALL GOLDENS PASS"
