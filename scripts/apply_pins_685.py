#!/usr/bin/env python3
"""dev-57 pin walk: the glass family's SEVENTEEN panes the palette's
tail banks — the palette/NAME_OF lengths the 543 rows the growth
answers (the END_PORTAL the 510 seat the walk never moves — the glass
rows the tail 543..559 the displacement law never trips). The
historical docs (the per-round growth sentences the earlier rounds
wore — the javap pc numbers, the `table the 543 rows` rows) the exact-
form replacement the 683 precedent wears; the audit lists every
remaining 543 mention the manual read answers."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the lengths' own forms (543 -> 560)
PIN_543 = [
    ("assert_eq!(atlas::N_KINDS, 543,", "assert_eq!(atlas::N_KINDS, 560,"),
    ("NAME_OF.len(), 543,", "NAME_OF.len(), 560,"),
    ("atlas::N_KINDS, 543,", "atlas::N_KINDS, 560,"),
    ("// 543 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 560 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// the same 543 the kit's growth banks)",
     "// the same 560 the kit's growth banks)"),
    ("// (the 543 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 560 the `id < N_KINDS` guards the bigger tail banks),"),
]

STANDALONE_543 = re.compile(r"^(\s+)543,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_543:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_543.findall(src))
    if c:
        src = STANDALONE_543.sub(r"\g<1>560,", src)
        n += c
    open(path, "w").write(src)
    print(f"{path}: {n} pins 543->560")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 543 mention the manual read wears
print("\n=== AUDIT: remaining 543 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"(?<![\dxXbBoO_])543(?![0-9])", line):
            print(f"{path}:{i}: {line.strip()[:150]}")
