#!/usr/bin/env python3
"""dev-59 pin walk: the copper bars family's EIGHT oxidization rungs
the palette's tail banks — the palette/NAME_OF lengths the 578 rows
the growth answers (the END_PORTAL the 510 seat the walk never moves —
the copper rows the tail 578..585 the displacement law never trips).
The historical docs the exact-form replacement the 686 precedent
wears; the audit lists every remaining 578 mention the manual read
answers."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (578 -> 586)
PIN_578 = [
    ("assert_eq!(atlas::N_KINDS, 578,", "assert_eq!(atlas::N_KINDS, 586,"),
    ("NAME_OF.len(), 578,", "NAME_OF.len(), 586,"),
    ("atlas::N_KINDS, 578,", "atlas::N_KINDS, 586,"),
]

STANDALONE_578 = re.compile(r"^(\s+)578,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_578:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_578.findall(src))
    if c:
        src = STANDALONE_578.sub(r"\g<1>586,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 578->586")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 578 mention the manual read wears
print("\n=== AUDIT: remaining 578 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])578(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
