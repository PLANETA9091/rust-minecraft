#!/usr/bin/env python3
"""dev-58 pin walk: the pane family's EIGHTEEN windows the palette's
tail banks — the palette/NAME_OF lengths the 560 rows the growth
answers (the END_PORTAL the 510 seat the walk never moves — the pane
rows the tail 560..577 the displacement law never trips). The
historical docs the exact-form replacement the 685 precedent wears;
the audit lists every remaining 560 mention the manual read answers."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (560 -> 578)
PIN_560 = [
    ("assert_eq!(atlas::N_KINDS, 560,", "assert_eq!(atlas::N_KINDS, 578,"),
    ("NAME_OF.len(), 560,", "NAME_OF.len(), 578,"),
    ("atlas::N_KINDS, 560,", "atlas::N_KINDS, 578,"),
]

STANDALONE_560 = re.compile(r"^(\s+)560,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_560:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_560.findall(src))
    if c:
        src = STANDALONE_560.sub(r"\g<1>578,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 560->578")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 560 mention the manual read wears
print("\n=== AUDIT: remaining 560 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])560(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
