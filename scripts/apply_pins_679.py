#!/usr/bin/env python3
"""autopush-679 pin walk: the current-tail 444 rows the 460 seats the
round's growth re-banks. The historical docs (the fence gate's own
displacement row, the javap pc numbers 12870-12912/8354-8363/20979-
21046/6275-6292, the dev-53 round's own number) the walk never
touches — the every-form replacement the 678 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 444,", "END_PORTAL, 460,"),
    ("assert_eq!(atlas::N_KINDS, 444,", "assert_eq!(atlas::N_KINDS, 459,"),
    ("atlas::N_KINDS, 444,", "atlas::N_KINDS, 459,"),
    ("assert_eq!(END_PORTAL, 444);", "assert_eq!(END_PORTAL, 460);"),
    ("assert_eq!(w::END_PORTAL, 444);", "assert_eq!(w::END_PORTAL, 460);"),
    ("NAME_OF.len(), 444,", "NAME_OF.len(), 459,"),
    # the current-tail comment lines the 678 precedent re-banks
    ("// 444 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 459 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 444 keeps)",
     "// family order banks, the END_PORTAL the displaced 460 keeps)"),
    ("// refuses, the table lengths the same 444 the one law keeps)",
     "// refuses, the table lengths the same 459 the one law keeps)"),
    ("// displacement keeps (the 444 the `id < N_KINDS` guards the",
     "// displacement keeps (the 459 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 444",
     "// stone shelf order banks, the END_PORTAL the displaced 460"),
    ("// the same 444 the kit's growth banks)",
     "// the same 459 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 444 keeps)",
     "// desert order banks, the END_PORTAL the displaced 460 keeps)"),
    ("// the same 444 the shelf's growth banks)",
     "// the same 459 the shelf's growth banks)"),
    ("// (the 444 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 459 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// clinit order banks, the END_PORTAL the displaced 444 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 460 keeps)"),
    ("// the same 444 the cloths' growth banks)",
     "// the same 459 the cloths' growth banks)"),
    ("// 444 the displacement law keeps).",
     "// 460 the displacement law keeps)."),
    ("// 442/443, the END_PORTAL the beyond-table 444 the",
     "// 442/443, the END_PORTAL the beyond-table 460 the"),
]

# the multi-line palette-pin rows the `444,` form alone wears
STANDALONE = re.compile(r"^(\s+)444,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_FORMS:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE.findall(src))
    if c:
        src = STANDALONE.sub(r"\g<1>459,", src)
        n += c
    open(path, "w").write(src)
    return n


total = walk(WORLD) + walk(SCREENS)
print(f"pins corrected: {total}")
