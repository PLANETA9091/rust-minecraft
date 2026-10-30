#!/usr/bin/env python3
"""autopush-363 — THE FMT SPLICE WALK (the 358/360/361/362 record's own
law the round wears): the probe's dirty regions the formatted stdout's
own opcode clusters wear — the original's non-region text the splice
keeps, the bottom-up order the offsets the shifts never wear, the
re-probe the ALL-REGIONS-CLEAN verdict speaks.
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fmt_probe_363 as probe


def main() -> int:
    ok = True
    for path, regions in probe.REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True, text=True)
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
            ok = False
            continue
        formatted = proc.stdout
        orig_lines = original.splitlines(keepends=True)
        fmt_lines = formatted.splitlines(keepends=True)
        sm = difflib.SequenceMatcher(None, original.splitlines(), formatted.splitlines())
        opcodes = [
            (i1, i2, j1, j2)
            for tag, i1, i2, j1, j2 in sm.get_opcodes()
            if tag != "equal"
        ]
        # the hunks the round's own regions the overlap speaks
        mine = []
        for (start_marker, end_marker) in regions:
            s = probe.find(original, start_marker)
            e = probe.region_end(original, end_marker, s)
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            for h in opcodes:
                if h[0] < e_line and h[1] > s_line and h not in mine:
                    mine.append(h)
        if not mine:
            print(f"{path}: regions already clean")
            continue
        # the merged clusters the contiguous hunks wear, bottom-up
        mine.sort()
        clusters = []
        for h in mine:
            if clusters and h[0] <= clusters[-1][-1][1]:
                clusters[-1].append(h)
            else:
                clusters.append([h])
        new_lines = orig_lines[:]
        for cluster in reversed(clusters):
            i1 = cluster[0][0]
            i2 = cluster[-1][1]
            j1 = cluster[0][2]
            j2 = cluster[-1][3]
            new_lines = new_lines[:i1] + fmt_lines[j1:j2] + new_lines[i2:]
            print(f"{path}: spliced orig[{i1}:{i2}] <- fmt[{j1}:{j2}] ({len(cluster)} hunk(s))")
        open(path, "w", encoding="utf-8").writelines(new_lines)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
