#!/usr/bin/env python3
"""autopush-521 — THE FMT SPLICE (the dev-234/520 record's own law the
round wears): the round's own regions the rustfmt answer the ORIGINAL
files splice back — the marker start the region end the round keeps,
the pre-marker text the original file never touches."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

JOBS = [
    (
        "crates/mc-client/src/screens.rs",
        "/// THE CHEST SWAP WIRE ROUND (the doClick SWAP table",
        "/// THE ON-TAKE WALK OVER THE BELIEF",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "/// [`wire_chest_swap`] free fn answers): the window identity the",
        "/// THE CHEST'S OWN SWAP TABLE (the offline twin",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE CHEST WIRE SWAP'S OWN KIND VERDICTS (the autopush-521 round",
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
        raise SystemExit(f"START MARKER NOT FOUND in {path}: {start_m[:60]!r}")
    if end_m is None:
        e, fe = len(original), len(formatted)
    else:
        e = original.find(end_m, s + 1)
        fe = formatted.find(end_m, fs + 1)
        if e < 0 or fe < 0:
            raise SystemExit(f"END MARKER NOT FOUND in {path}: {end_m[:60]!r}")
    spliced = original[:s] + formatted[fs:fe] + original[e:]
    open(path, "w", encoding="utf-8").write(spliced)
    got = fe - fs
    had = e - s
    print(f"{path}: spliced {had} -> {got} chars")
print("SPLICE-DONE")
