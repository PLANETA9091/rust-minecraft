#!/usr/bin/env python3
"""dev-235 — THE FMT SPLICE (the dev-234 record's own law the round
wears): the two regions' own markers the rustfmt answer the ORIGINAL
files splice back — the pre-marker text the original files never
touch.

  screens.rs — the doc tail's marker start, the hovered-stack doc's
  marker end (the ride the tables the rustfmt answer re-lays).
  part_11.rs — the round's own tests' marker start, EOF the end the
  round keeps.
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

JOBS = [
    (
        "crates/mc-client/src/screens.rs",
        "    /// THE RIDE'S OWN TAIL (the dev-235 round — the kind + data rows",
        "    /// The hovered seat's own stack (the menu's slot list the walk",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE SWAP TABLE'S OWN WORD RIDE (the dev-235 round — the number-key",
        None,
    ),
]

for path, start_m, end_m in JOBS:
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path], capture_output=True, text=True
    )
    if proc.returncode != 0:
        raise SystemExit(f"RUSTFMT FAILED for {path}\n{proc.stderr[:400]}")
    formatted = proc.stdout
    s = original.find(start_m)
    fs = formatted.find(start_m)
    if s < 0 or fs < 0:
        raise SystemExit(f"START MARKER NOT FOUND in {path}")
    if end_m is None:
        spliced = original[:s] + formatted[fs:]
        print(f"{path}: SPLICED tail {len(original) - s} -> {len(formatted) - fs} bytes")
    else:
        e = original.find(end_m, s + 1)
        fe = formatted.find(end_m, fs + 1)
        if e < 0 or fe < 0:
            raise SystemExit(f"END MARKER NOT FOUND in {path}")
        spliced = original[:s] + formatted[fs:fe] + original[e:]
        print(f"{path}: SPLICED region {e - s} -> {fe - fs} bytes")
    open(path, "w", encoding="utf-8").write(spliced)
print("SPLICE DONE")
