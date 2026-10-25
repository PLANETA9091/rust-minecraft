#!/usr/bin/env bash
# THE AUTOPUSH-218 WALK (the dev-166 twelve-window answer wears): the
# mc-client test bank one window per call, the fast crates' own trio,
# the golden matrix the verify_goldens.sh walk serves. rc sums per
# stage; the log the tail prints.
set -u
cd /home/z/my-project/rust-minecraft
export PATH="$HOME/.cargo/bin:$PATH"

rc_all=0
for w in part_1 part_2 part_3 part_4 part_5 part_6 part_7 part_8 part_9 part_10 part_11; do
  echo "=== WINDOW $w ==="
  if ! cargo test -p mc-client --bin mc-client "screens::tests::$w" >"/tmp/walk_$w.log" 2>&1; then
    echo "WINDOW $w FAILED — tail:"
    tail -30 "/tmp/walk_$w.log"
    rc_all=1
  else
    rg -o "test result: ok. [0-9]+ passed" "/tmp/walk_$w.log" | tail -1
  fi
done

echo "=== WINDOW rest ==="
if ! cargo test -p mc-client --bin mc-client -- --skip "screens::tests::part_" >"/tmp/walk_rest.log" 2>&1; then
  echo "WINDOW rest FAILED — tail:"
  tail -30 "/tmp/walk_rest.log"
  rc_all=1
else
  rg -o "test result: ok. [0-9]+ passed" "/tmp/walk_rest.log" | tail -1
fi

echo "=== FAST CRATES ==="
for c in mc-protocol mc-registry mc2rs-gen; do
  if ! cargo test -p "$c" >"/tmp/walk_$c.log" 2>&1; then
    echo "CRATE $c FAILED — tail:"
    tail -20 "/tmp/walk_$c.log"
    rc_all=1
  else
    echo "$c: $(rg -o 'test result: ok. [0-9]+ passed' /tmp/walk_$c.log | tail -1)"
  fi
done

echo "=== GOLDENS ==="
if ! bash scripts/verify_goldens.sh >"/tmp/walk_goldens.log" 2>&1; then
  echo "GOLDENS FAILED — tail:"
  tail -40 "/tmp/walk_goldens.log"
  rc_all=1
else
  tail -5 "/tmp/walk_goldens.log"
fi

echo "=== WALK rc=$rc_all ==="
exit $rc_all
