#!/usr/bin/env python3
"""THE FMT SEAT'S OWN HUNK FILTER b (the dev-172 round): the same walk
the a-file's blocked-write mystery keeps — the unified patch's hunks the
NEW-code ranges alone wear, the HEAD's own drift the skip keeps.
"""
import re

RANGES = {
    "/tmp/body.patch": [(1987, 2280), (2840, 2900), (3160, 3200), (4520, 4720)],
    "/tmp/p4.patch": [(16580, 16640)],
}


def hunk_range(h):
    m = re.match(r"@@ -(\d+)(?:,(\d+))? \+", h.splitlines()[0])
    if not m:
        return None
    start = int(m.group(1))
    ln = int(m.group(2)) if m.group(2) else 1
    return start, start + max(ln, 1)


def keep(h, ranges):
    r = hunk_range(h)
    if r is None:
        return False
    s, e = r
    return any(s < hi and e > lo for lo, hi in ranges)


for patch_path, ranges in RANGES.items():
    text = open(patch_path).read()
    head, *hunks = re.split(r"(?m)^@@ ", text)
    kept = [h for h in hunks if keep(h, ranges)]
    out = head + "".join("@@ " + h for h in kept)
    dest = patch_path.replace(".patch", ".filtered.patch")
    open(dest, "w").write(out)
    print(f"{patch_path}: {len(hunks)} hunks total, {len(kept)} kept -> {dest}")
