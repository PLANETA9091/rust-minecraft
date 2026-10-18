#!/usr/bin/env python3
"""dev-64 pin walk: the chest family's ELEVEN kinds the palette's tail
banks — the palette/NAME_OF lengths the 748 rows the growth answers
(the END_PORTAL the 510 seat the walk never moves — the chest rows the
tail 737..748 the displacement law never trips). The historical docs
the exact-form replacement the 691 precedent wears; the audit lists
every remaining 737 mention the manual read answers (the CHEST constant
+ the autopush-737 doc mentions the walk never touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (737 -> 748)
PIN_737 = [
    ("assert_eq!(atlas::N_KINDS, 737,", "assert_eq!(atlas::N_KINDS, 748,"),
    ("NAME_OF.len(), 737,", "NAME_OF.len(), 748,"),
    ("atlas::N_KINDS, 737,", "atlas::N_KINDS, 748,"),
]

STANDALONE_737 = re.compile(r"^(\s+)737,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_737:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_737.findall(src))
    if c:
        src = STANDALONE_737.sub(r"\g<1>748,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 737->748")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 737 mention the manual read wears
print("\n=== AUDIT: remaining 737 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])737(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
