#!/usr/bin/env python3
"""dev-217 — THE FMT REGION FIX (the probe's own walk the splice
answers): the whole-file rustfmt the ORIGINAL text the formatted stdout
the hunks speak — the hunks the MY-REGIONS intersect the formatted
replacement the file re-wears (the regions' own text the rustfmt
verdict serves, the rest of the file the walk never touches).

Regions: the same map scripts/fmt_probe_217.py keeps.
"""
import difflib
import os

import subprocess


RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    (
        "// the store dropped: a LIVE brewing stand closes back to",
        "self.live_brewing_title = None; // the title dies with the window",
    ),
    (
        "// the store dropped: a LIVE anvil closes back to the",
        "self.live_anvil_title = None; // the title dies with the window",
    ),
]

PART11 = [
    (
        "THE WINDOW-DEATH'S OWN RETIRE (the dev-217 round",
        "the drop the window dies",
    ),
    (
        "THE WINDOW-DEATH'S OWN BREWING-ANVIL RETIRE",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
    "crates/mc-client/src/screens.rs": SCREENS,
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
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    fixed_any = False
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, j1, j2))
        cursor = 0
        touched = []
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            for (i1, i2, j1, j2) in hunks:
                if i1 < e_line and i2 > s_line and (i1, i2) not in touched:
                    touched.append((i1, i2, j1, j2))
        if not touched:
            print(f"{path}: region already fmt-clean, no splice")
            continue
        # the splice the reverse order wears — the offsets the tail
        # first the shift never wears
        new_text = original
        for (i1, i2, j1, j2) in sorted(touched, reverse=True):
            replacement = "\n".join(formatted.splitlines()[j1:j2])
            lines = new_text.split("\n")
            new_text = "\n".join(lines[:i1] + formatted.splitlines()[j1:j2] + lines[i2:])
        open(path, "w", encoding="utf-8").write(new_text)
        fixed_any = True
        print(f"{path}: spliced {len(touched)} region hunk(s)")
    if fixed_any:
        print("SPLICE-DONE")
    else:
        print("NOTHING-TO-FIX")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
