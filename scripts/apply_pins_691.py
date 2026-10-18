#!/usr/bin/env python3
"""dev-63 pin walk: the slab family's SIXTY-EIGHT kinds the palette's
tail banks — the palette/NAME_OF lengths the 737 rows the growth
answers (the END_PORTAL the 510 seat the walk never moves — the slab
rows the tail 669..737 the displacement law never trips). The
historical docs the exact-form replacement the 690 precedent wears;
the audit lists every remaining 669 mention the manual read answers
(the OAK_SLAB constant + the autopush-669 doc mentions the walk never
touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (669 -> 737)
PIN_669 = [
    ("assert_eq!(atlas::N_KINDS, 669,", "assert_eq!(atlas::N_KINDS, 737,"),
    ("NAME_OF.len(), 669,", "NAME_OF.len(), 737,"),
    ("atlas::N_KINDS, 669,", "atlas::N_KINDS, 737,"),
]

STANDALONE_669 = re.compile(r"^(\s+)669,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_669:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_669.findall(src))
    if c:
        src = STANDALONE_669.sub(r"\g<1>737,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 669->737")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 669 mention the manual read wears
print("\n=== AUDIT: remaining 669 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])669(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
