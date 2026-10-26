#!/usr/bin/env bash
# THE WALL-GATE'S OWN BATCH WALKER (the 258 round's reusable lane —
# the per-window git restore the mid-write lesson keeps, the
# pre-flight check the corrupted ORIGINAL the future kills never
# inherit): usage `bash scripts/wallgate_batch.sh 1 2 3` — each arg
# one gate window (1..11|rest), the batch the full recompile the
# shared target dir amortizes, the MOD-RESTORED-CLEAN the batch's
# tail verifies.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.."
rc_total=0
for w in "$@"; do
  # THE PRE-FLIGHT (the mod.rs the dirty tree the walk never
  # inherits — the kill-window lesson the restore the head start)
  if ! git diff --quiet -- crates/mc-client/src/screens/tests/mod.rs; then
    git checkout -- crates/mc-client/src/screens/tests/mod.rs
    echo "[pre-flight] mod.rs restored"
  fi
  python3 scripts/gate_window_full.py "$w"
  rc=$?
  if [ $rc -ne 0 ]; then
    echo "[window $w FAILED rc=$rc]"
    rc_total=1
    break
  fi
  # THE RESTORE VERDICT (the gated walk the clean tree the tail
  # verifies — the next window the fresh ORIGINAL reads)
  if git diff --quiet -- crates/mc-client/src/screens/tests/mod.rs; then
    echo "MOD-RESTORED-CLEAN window $w"
  else
    git checkout -- crates/mc-client/src/screens/tests/mod.rs
    echo "MOD-RESTORED-FORCED window $w"
  fi
done
exit $rc_total
