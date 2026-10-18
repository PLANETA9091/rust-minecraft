#!/usr/bin/env python3
"""dev-62 pin walk: the stairs family's SIXTY-FOUR kinds the palette's
tail banks — the palette/NAME_OF lengths the 669 rows the growth
answers (the END_PORTAL the 510 seat the walk never moves — the stairs
rows the tail 605..669 the displacement law never trips). The
historical docs the exact-form replacement the 689 precedent wears;
the audit lists every remaining 605 mention the manual read answers
(the OAK_STAIRS constant + the bytecode pc mentions the walk never
touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (605 -> 669)
PIN_605 = [
    ("assert_eq!(atlas::N_KINDS, 605,", "assert_eq!(atlas::N_KINDS, 669,"),
    ("NAME_OF.len(), 605,", "NAME_OF.len(), 669,"),
    ("atlas::N_KINDS, 605,", "atlas::N_KINDS, 669,"),
]

STANDALONE_605 = re.compile(r"^(\s+)605,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_605:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_605.findall(src))
    if c:
        src = STANDALONE_605.sub(r"\g<1>669,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 605->669")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 605 mention the manual read wears
print("\n=== AUDIT: remaining 605 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])605(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
