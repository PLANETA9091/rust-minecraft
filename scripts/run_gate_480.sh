#!/usr/bin/env bash
# run_gate_480.sh — the autopush-480 round's own gate walk (the 468
# record's own law the round re-wears): the twelve-window WALL-GATE,
# the FULL workspace run, the fast trio, the release build + the
# golden matrix. The log the /tmp/gate480.log the tail wears.
set -uo pipefail
cd /home/z/my-project/rust-minecraft
LOG=/tmp/gate480.log
: > "$LOG"
export PATH="$HOME/.cargo/bin:$PATH"

echo "== WINDOWED WALL-GATE ==" | tee -a "$LOG"
for w in 1 2 3 4 5 6 7 8 9 10 11 rest; do
  python3 scripts/gate_window_full.py "$w" 2>&1 | tee -a "$LOG"
done

echo "== FULL WORKSPACE ==" | tee -a "$LOG"
cargo test --workspace 2>&1 | grep -E "test result|running |error" | tail -16 | tee -a "$LOG"

echo "== FAST TRIO ==" | tee -a "$LOG"
cargo test -p mc-protocol 2>&1 | grep "test result" | tee -a "$LOG"
cargo test -p mc-registry 2>&1 | grep "test result" | tee -a "$LOG"
cargo test -p mc2rs-gen 2>&1 | grep "test result" | tee -a "$LOG"

echo "== RELEASE + GOLDENS ==" | tee -a "$LOG"
cargo build --release -p mc-client -p mc2rs-pixel 2>&1 | tail -2 | tee -a "$LOG"
bash scripts/verify_goldens.sh 2>&1 | tail -24 | tee -a "$LOG"

echo "== DRIVER DONE ==" | tee -a "$LOG"
