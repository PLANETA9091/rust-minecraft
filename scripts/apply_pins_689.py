#!/usr/bin/env python3
"""dev-61 pin walk: the lantern family's TEN kinds the palette's tail
banks — the palette/NAME_OF lengths the 595 rows the growth answers
(the END_PORTAL the 510 seat the walk never moves — the lantern rows
the tail 595..605 the displacement law never trips). The historical
docs the exact-form replacement the 688 precedent wears; the audit
lists every remaining 595 mention the manual read answers (the
LANTERN constant + the bytecode pc mentions the walk never touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (595 -> 605)
PIN_595 = [
    ("assert_eq!(atlas::N_KINDS, 595,", "assert_eq!(atlas::N_KINDS, 605,"),
    ("NAME_OF.len(), 595,", "NAME_OF.len(), 605,"),
    ("atlas::N_KINDS, 595,", "atlas::N_KINDS, 605,"),
]

STANDALONE_595 = re.compile(r"^(\s+)595,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_595:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_595.findall(src))
    if c:
        src = STANDALONE_595.sub(r"\g<1>605,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 595->605")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 595 mention the manual read wears
print("\n=== AUDIT: remaining 595 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])595(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
