#!/usr/bin/env python3
"""autopush-320 — THE REGION FMT SPLICE (the whole-file rustfmt the
MY-REGIONS text the splice serves: the formatted file the marker
ranges' text donates, the original the non-mine bytes the byte-level
law keeps — the my-regions-only verdict the splice banks).

Exit 0 iff the splice lands fmt-clean for every region file.
"""
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fmt_region_320 as R

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")


def fmt_text(path):
    r = subprocess.run([RUSTFMT, "--emit", "stdout", path],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(f"FMT-ERR {path}: {r.stderr[:400]}")
        sys.exit(2)
    return r.stdout


def main():
    for path, marks in R.REGIONS.items():
        orig = open(path).read()
        lines = orig.split("\n")
        regions = R.marker_ranges(lines, marks, path)
        fmt = fmt_text(path).split("\n")
        # THE MARKER WALK'S OWN RE-RUN (the formatted text the same
        # markers the span answers — the splice the formatted regions
        # the byte law wears)
        fmt_regions = R.marker_ranges(fmt, marks, path)
        if len(fmt_regions) != len(regions):
            print(f"MARKER MISMATCH {path}")
            sys.exit(2)
        out = list(lines)
        for (s, e), (fs, fe) in zip(
            sorted(regions, reverse=True), sorted(fmt_regions, reverse=True)
        ):
            out[s:e] = fmt[fs:fe]
        open(path, "w").write("\n".join(out))
        print(f"SPLICED {path}: {len(regions)} region(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
