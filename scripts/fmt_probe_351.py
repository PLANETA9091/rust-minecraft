#!/usr/bin/env python3
"""autopush-350 — THE FMT REGION WALK (the 323 record's own law the round
wears): the whole-file rustfmt the ORIGINAL text the formatted stdout
the diff answers — the MY-REGIONS verdict the marker ranges keep (no
hunk may touch the round's own edits), the pre-existing static noise
the HEAD law confirms the non-mine hunks the honest inventory banks.

Exit 0 iff every marker range is fmt-clean (no diff hunk overlaps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

CONST_HEAD = (
    "/// THE DEBUG PROFILE'S OWN DATA VERSION (javap 26.2 the"
)
CONST_TAIL = "pub const DEBUG_PROFILE_DATA_VERSION: i64 = 4649;"
LOAD_HEAD = (
    '        // THE VERSION ROW\'S OWN READ (the wrapCodec decode pc 0-56 —'
)
LOAD_TAIL = '        match root.get("profile") {'
SAVE_HEAD = (
    '        // THE WRAPPED ENCODE\'S OWN VERSION ROW (the DataFixTypes'
)
SAVE_TAIL = '        if let Ok(text) = serde_json::to_string(&root) {'

REGIONS = {
    "crates/mc-client/src/screens.rs": [
        # the wrapped encode's version-row stamp
        (SAVE_HEAD, SAVE_TAIL),
        # the wrapped decode's version read + newer-throw + identity note
        (LOAD_HEAD, LOAD_TAIL),
        # the const's doc + declaration
        (CONST_HEAD, CONST_TAIL),
    ],
    "crates/mc-client/src/screens/tests/part_4.rs": [
        # the whole new datafixer round-trip test
        ("THE DEBUG PROFILE'S OWN DATAFIXER ROUND (javap 26.2 the", None),
    ],
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


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            ok = False
            continue
        formatted = proc.stdout
        sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag != "equal":
                hunks.append((i1, i2))
        for head_marker, tail_marker in regions:
            start = find(original, head_marker)
            end = region_end(original, tail_marker, start)
            start_line = original.count("\n", 0, start)
            end_line = original.count("\n", 0, end)
            overlap = [
                (a, b) for (a, b) in hunks
                if a < end_line and b > start_line
            ]
            if overlap:
                ok = False
                print(f"{path}: REGION DIRTY [{start_line},{end_line}) "
                      f"head={head_marker[:60]!r} hunks={overlap}")
            else:
                print(f"{path}: region clean [{start_line},{end_line}) "
                      f"head={head_marker[:48]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
