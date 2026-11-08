#!/usr/bin/env python3
"""autopush-513 — THE FMT SPLICE (the dev-234/512 record's own law the
round wears): the round's own regions the rustfmt answer the ORIGINAL
files splice back — the marker start the region end the round keeps,
the pre-marker text the original file never touches. The screens.rs
region the marker-to-marker range splices, the part_11 tail the
marker-to-EOF range wears.
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

JOBS = [
    (
        "crates/mc-client/src/screens.rs",
        "    /// THE BEACON'S OWN SWAP KEY (javap 26.2 — the SAME",
        "    /// THE DISPENSER'S OWN SWAP KEY (javap 26.2 — the SAME",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE BEACON SWAP'S OWN PAYMENT BANK (the autopush-513 round — the",
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
