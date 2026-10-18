#!/usr/bin/env python3
"""dev-65 pin walk: the BARREL kind the palette's tail banks — the
palette/NAME_OF lengths the 749 rows the growth answers (the END_PORTAL
the 510 seat the walk never moves). The historical docs the exact-form
replacement the 692 precedent wears; the audit lists every remaining 748
mention the manual read answers (the BARREL constant + the autopush-748
doc mentions the walk never touches)."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (748 -> 749)
PIN_748 = [
    ("assert_eq!(atlas::N_KINDS, 748,", "assert_eq!(atlas::N_KINDS, 749,"),
    ("NAME_OF.len(), 748,", "NAME_OF.len(), 749,"),
    ("atlas::N_KINDS, 748,", "atlas::N_KINDS, 749,"),
]

STANDALONE_748 = re.compile(r"^(\s+)748,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_748:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_748.findall(src))
    if c:
        src = STANDALONE_748.sub(r"\g<1>749,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 748->749")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 748 mention the manual read wears
print("\n=== AUDIT: remaining 748 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])748(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
