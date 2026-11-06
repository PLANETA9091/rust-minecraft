#!/usr/bin/env python3
"""autopush-487 — THE FMT SPLICE (the 485 record's own law the round
wears): the DIRTY regions the probe spoke the rustfmt bytes the
region wears — the prefix the original text keeps byte-exact, the
region's own range the formatted text answers, the tail the original
resumes. The pre-existing drift the splice never touches (the
outside regions the file's own history banks)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# (path, start_marker, end_marker-or-None-EOF) — the DIRTY regions the
# probe's own verdict spoke (screens draw strip + the part_11 tail).
DIRTY = [
    (
        "crates/mc-client/src/screens.rs",
        "// THE DAMAGE BAR'S OWN STRIP",
        "if count > 1 {",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE DAMAGE BAR'S OWN LAWS",
        None,
    ),
]


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def main() -> int:
    for (path, start_marker, end_marker) in DIRTY:
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            return 1
        formatted = proc.stdout
        s_orig = find(original, start_marker)
        e_orig = (
            len(original)
            if end_marker is None
            else find(original, end_marker, s_orig)
        )
        s_fmt = find(formatted, start_marker)
        e_fmt = (
            len(formatted)
            if end_marker is None
            else find(formatted, end_marker, s_fmt)
        )
        spliced = original[:s_orig] + formatted[s_fmt:e_fmt] + original[e_orig:]
        if spliced != original:
            open(path, "w", encoding="utf-8").write(spliced)
            print(f"{path}: spliced [{start_marker[:48]}] "
                  f"({e_orig - s_orig} -> {e_fmt - s_fmt} bytes)")
        else:
            print(f"{path}: already clean [{start_marker[:48]}]")
    return 0


if __name__ == "__main__":
    sys.exit(main())
