#!/usr/bin/env python3
"""autopush-492 — THE FMT SPLICE (the 485/487/488/489/491/229 record's own law
the round wears): the DIRTY region the probe spoke the rustfmt bytes
the region wears — the prefix the original text keeps byte-exact, the
region's own range the formatted text answers, the tail the original
resumes. The pre-existing drift the splice never touches (the outside
regions the file's own history banks)."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# the DIRTY region the probe's own verdict spoke (the part_11 tail the
# round's own tests the EOF keeps)
DIRTY = [
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE TABLE CELLS' OWN DATA SEATS (the autopush-492 round",
    ),
]


def main() -> int:
    for (path, start_marker) in DIRTY:
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            return 1
        formatted = proc.stdout
        s_orig = original.find(start_marker)
        if s_orig < 0:
            print(f"{path}: MARKER NOT FOUND: {start_marker!r}")
            return 1
        s_fmt = formatted.find(start_marker)
        if s_fmt < 0:
            print(f"{path}: FORMATTED MARKER NOT FOUND")
            return 1
        spliced = original[:s_orig] + formatted[s_fmt:]
        if spliced != original:
            open(path, "w", encoding="utf-8").write(spliced)
            print(f"{path}: spliced [{start_marker[:48]}] "
                  f"({len(original) - s_orig} -> {len(spliced) - s_orig} bytes)")
        else:
            print(f"{path}: already clean [{start_marker[:48]}]")
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
