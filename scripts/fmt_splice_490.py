#!/usr/bin/env python3
"""autopush-490 — THE FMT SPLICE (the 488/489 record's own law the
round wears): the DIRTY regions the probe named the rustfmt bytes the
line-splice wears — the prefix the original keeps, the region's own
lines the formatted text's region lines answer, the suffix the
original resumes (the 490 round's own regions only — the pre-existing
drift the other areas the law never touches).

Regions:
  drops.rs    — the gated touch region (the wrapper + the gate body
                the 4-line Picked tail the rustfmt bytes collapse).
  part_11.rs  — the 489 third arm + the round's own tests (the tail
                the EOF keeps).
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

REGIONS = {
    "crates/mc-client/src/drops.rs": (
        "player_touch_gate(drop, inventory, selected, creative, &|_| true)",
        "/// THE DAMAGE BAR'S OWN STACK LAWS",
    ),
    "crates/mc-client/src/screens/tests/part_11.rs": (
        "// THE REFUSE-TO-MERGE'S OWN LAW",
        None,
    ),
}


def splice(path: str, start_marker: str, end_marker: str | None) -> None:
    original = open(path, encoding="utf-8").read()
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: rustfmt failed\n{proc.stderr}")
    formatted = proc.stdout
    o_lines = original.splitlines(keepends=True)
    f_lines = formatted.splitlines(keepends=True)

    def region_lines(lines, start_marker, end_marker):
        text = "".join(lines)
        s = text.find(start_marker)
        if s < 0:
            raise SystemExit(f"{path}: START MARKER NOT FOUND: {start_marker!r}")
        s_line = text[:s].count("\n")
        if end_marker is None:
            e_line = len(lines)
        else:
            e = text.find(end_marker, s)
            if e < 0:
                raise SystemExit(f"{path}: END MARKER NOT FOUND: {end_marker!r}")
            e_line = text[:e].count("\n")
        return s_line, e_line

    os_line, oe_line = region_lines(o_lines, start_marker, end_marker)
    fs_line, fe_line = region_lines(f_lines, start_marker, end_marker)
    new_lines = o_lines[:os_line] + f_lines[fs_line:fe_line] + o_lines[oe_line:]
    open(path, "w", encoding="utf-8").write("".join(new_lines))
    print(f"{path}: spliced [{os_line}:{oe_line}] -> [{fs_line}:{fe_line}]")


def main() -> None:
    for path, (sm, em) in REGIONS.items():
        splice(path, sm, em)


if __name__ == "__main__":
    main()
