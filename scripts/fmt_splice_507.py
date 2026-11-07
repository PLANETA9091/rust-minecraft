#!/usr/bin/env python3
"""autopush-507 — THE FMT SPLICE (the dev-234/235 record's own law the
round wears): the two regions' own markers the rustfmt answer the
ORIGINAL files splice back — the pre-marker text the original files
never touch.

  screens.rs — the chest swap walk's own doc-tail marker start, the
  survival doc's marker end (the split + the twin the rustfmt answer
  re-lays).
  part_11.rs — the round's own tests' marker start, EOF the end the
  round keeps.
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

JOBS = [
    (
        "crates/mc-client/src/screens.rs",
        "    /// - the window gate: the WIRE lane the live session's own",
        "    /// THE SURVIVAL SCREEN'S OWN HOTBAR KEYS (javap 26.2 — the",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE CHEST SWAP'S OWN WORD RIDE (the autopush-507 round — the",
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
