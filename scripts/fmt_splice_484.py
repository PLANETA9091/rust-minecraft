#!/usr/bin/env python3
"""autopush-484 — THE FMT SPLICE WALK (the 483 record's own law the
round re-wears): the dirty regions the rustfmt stdout's own bytes the
region borders splice — the prefix the original text keeps, the round's
own region the formatted bytes wear, the rest the untouched law keeps."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")


def formatted(path: str) -> str:
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"RUSTFMT FAILED {path}: {proc.stderr}")
    return proc.stdout


def splice_mid(path: str, start_marker: str, end_marker: str) -> None:
    """the mid-file region the two borders splice"""
    original = open(path, encoding="utf-8").read()
    fmt = formatted(path)
    s = original.find(start_marker)
    if s < 0:
        raise SystemExit(f"MARKER NOT FOUND in {path}: {start_marker!r}")
    e = original.find(end_marker, s)
    if e < 0:
        raise SystemExit(f"MARKER NOT FOUND in {path}: {end_marker!r}")
    fs = fmt.find(start_marker)
    if fs < 0:
        raise SystemExit(f"MARKER NOT FOUND in fmt {path}: {start_marker!r}")
    fe = fmt.find(end_marker, fs)
    if fe < 0:
        raise SystemExit(f"MARKER NOT FOUND in fmt {path}: {end_marker!r}")
    spliced = original[:s] + fmt[fs:fe] + original[e:]
    open(path, "w", encoding="utf-8").write(spliced)
    print(f"{path}: spliced [{start_marker.strip()[:48]} .. {end_marker.strip()[:48]}]")


def splice_tail(path: str, start_marker: str) -> None:
    """the tail region the EOF border splice"""
    original = open(path, encoding="utf-8").read()
    fmt = formatted(path)
    s = original.find(start_marker)
    if s < 0:
        raise SystemExit(f"MARKER NOT FOUND in {path}: {start_marker!r}")
    fs = fmt.find(start_marker)
    if fs < 0:
        raise SystemExit(f"MARKER NOT FOUND in fmt {path}: {start_marker!r}")
    spliced = original[:s] + fmt[fs:]
    open(path, "w", encoding="utf-8").write(spliced)
    print(f"{path}: spliced tail [{start_marker.strip()[:48]} .. EOF]")


def main() -> None:
    splice_mid(
        "crates/mc-client/src/mob.rs",
        "/// THE FISHING BOBBER'S OWN BODY",
        "/// THE SEGMENT'S OWN SLAB WALK",
    )
    splice_tail(
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE FISHING ROD'S OWN CAST",
    )
    print("SPLICE-DONE")


if __name__ == "__main__":
    main()
