#!/usr/bin/env python3
"""dev-216 — THE FMT REGION WALK (the 372..autopush-417 record's
own law the round wears): the whole-file rustfmt the ORIGINAL text
the formatted stdout the diff answers — the MY-REGIONS verdict the
marker ranges keep (no hunk may touch the round's own edits).

Regions:
  part_11.rs    — the round's own test (the bridge the store law
                  wears, EOF the end the round keeps).
  clipboard.rs  — the round's own module (the whole file the bridge
                  owns).
  screens.rs    — the walk sites the bridge rides (the hunk check
                  the crate::clipboard lines the round keeps — no
                  rustfmt hunk may touch a wired site).
"""
import difflib
import os

import subprocess
import sys


RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

PART11 = [
    # the round's own test (the bridge the store law wears — the EOF
    # the end the round keeps)
    (
        "THE OS CLIPBOARD'S OWN BRIDGE",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def region_end(text: str, marker: str, start_at: int) -> int:
    if marker is None:
        return len(text)
    return find(text, marker, start_at)


def formatted_of(path: str) -> str:
    """the formatted text the stdout answers — the filename header this
    rustfmt wears the strip removes (the header the j-side only rides,
    the wired-site hunk math the i-side keeps)"""
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    # the header this rustfmt wears — the CANONICAL path the first line
    # speaks + the blank the second keeps (the relative arg the abs
    # prefix never matches — the endswith-colon shape the rule reads)
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def whole_file_clean(path: str) -> bool:
    """the module the round owns — the whole file the diff answers"""
    original = open(path, encoding="utf-8").read()
    sm = difflib.SequenceMatcher(
        None, original.splitlines(), formatted_of(path).splitlines())
    bad = [h for h in sm.get_opcodes() if h[0] != "equal"]
    if bad:
        print(f"{path}: WHOLE-FILE NOT FMT-CLEAN hunks={bad[:3]}")
        return False
    print(f"{path}: whole-file fmt-clean")
    return True


def wired_sites_clean(path: str) -> bool:
    """the walk sites the bridge rides — no rustfmt hunk may touch a
    crate::clipboard line"""
    original = open(path, encoding="utf-8").read()
    sm = difflib.SequenceMatcher(
        None, original.splitlines(), formatted_of(path).splitlines())
    wired = [
        i for i, line in enumerate(original.splitlines())
        if "crate::clipboard::" in line
    ]
    bad = [
        (i1, i2, tag) for tag, i1, i2, _, _ in sm.get_opcodes()
        if tag != "equal"
        for lo, hi in [(i1, i2)]
        if any(lo <= w < hi for w in wired)
    ]
    if bad:
        print(f"{path}: WIRED-SITE NOT FMT-CLEAN hunks={bad}")
        return False
    print(f"{path}: wired sites fmt-clean ({len(wired)} lines)")
    return True


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56].replace("\n", " | ")
            if bad:
                ok = False
                print(f"{path}: REGION NOT FMT-CLEAN [{label}] hunks={bad}")
            else:
                print(f"{path}: region fmt-clean [{label}]")
    ok = whole_file_clean("crates/mc-client/src/clipboard.rs") and ok
    ok = wired_sites_clean("crates/mc-client/src/screens.rs") and ok
    if ok:
        print("ALL-REGIONS-CLEAN")
    else:
        print("REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
