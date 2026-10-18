#!/usr/bin/env python3
"""dev-60 pin walk: the chain family's NINE links the palette's tail
banks — the palette/NAME_OF lengths the 586 rows the growth answers
(the END_PORTAL the 510 seat the walk never moves — the chain rows the
tail 586..595 the displacement law never trips). The historical docs
the exact-form replacement the 687 precedent wears; the audit lists
every remaining 586 mention the manual read answers (the IRON_CHAIN
constant + the bytecode pc mentions the walk never touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (586 -> 595)
PIN_586 = [
    ("assert_eq!(atlas::N_KINDS, 586,", "assert_eq!(atlas::N_KINDS, 595,"),
    ("NAME_OF.len(), 586,", "NAME_OF.len(), 595,"),
    ("atlas::N_KINDS, 586,", "atlas::N_KINDS, 595,"),
]

STANDALONE_586 = re.compile(r"^(\s+)586,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_586:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_586.findall(src))
    if c:
        src = STANDALONE_586.sub(r"\g<1>595,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 586->595")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 586 mention the manual read wears
print("\n=== AUDIT: remaining 586 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])586(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
