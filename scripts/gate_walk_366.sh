#!/usr/bin/env bash
# autopush-366 — THE TWELVE-WINDOW WALL-GATE WALK (the 205/302/358 law
# the runbook keeps): each window timeout 570 the rc 0 verdict wears,
# the counts the log banks.
set -u
cd /home/z/my-project/rust-minecraft
export PATH="$HOME/.cargo/bin:$PATH"
LOG=/tmp/gate366.log
: > "$LOG"
for w in 1 2 3 4 5 6 7 8 9 10 11 rest; do
  timeout 570 python3 scripts/gate_window_full.py "$w" > /tmp/gate_w_"$w".txt 2>&1
  rc=$?
  count=$(grep -oE '[0-9]+ passed' /tmp/gate_w_"$w".txt | head -1)
  echo "window=$w rc=$rc $count" >> "$LOG"
  if [ $rc -ne 0 ]; then
    echo "WINDOW FAILED rc=$rc — aborting walk" >> "$LOG"
    exit 1
  fi
done
echo "ALL-WINDOWS-GREEN" >> "$LOG"
