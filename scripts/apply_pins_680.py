#!/usr/bin/env python3
"""autopush-680 pin walk: the current-tail 459 rows the 475 seats the
round's growth re-banks. The historical docs (the END_PORTAL chain's
own per-round sentences, the concrete family's javap evidence row,
the javap pc numbers 20979-21046/6275-6309, the pc 459-462 nether
pairs, the dev-53 round's own number) the walk never touches — the
every-form replacement the 679 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 460,", "END_PORTAL, 475,"),
    ("assert_eq!(atlas::N_KINDS, 459,", "assert_eq!(atlas::N_KINDS, 474,"),
    ("atlas::N_KINDS, 459,", "atlas::N_KINDS, 474,"),
    ("assert_eq!(END_PORTAL, 460);", "assert_eq!(END_PORTAL, 475);"),
    ("assert_eq!(w::END_PORTAL, 460);", "assert_eq!(w::END_PORTAL, 475);"),
    ("NAME_OF.len(), 459,", "NAME_OF.len(), 474,"),
    # the current-tail comment lines the 679 precedent re-banks
    ("// 459 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 474 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 460 keeps)",
     "// family order banks, the END_PORTAL the displaced 475 keeps)"),
    ("// refuses, the table lengths the same 459 the one law keeps)",
     "// refuses, the table lengths the same 474 the one law keeps)"),
    ("// displacement keeps (the 459 the `id < N_KINDS` guards the",
     "// displacement keeps (the 474 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 460",
     "// stone shelf order banks, the END_PORTAL the displaced 475"),
    ("// the same 459 the kit's growth banks)",
     "// the same 474 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 460 keeps)",
     "// desert order banks, the END_PORTAL the displaced 475 keeps)"),
    ("// the same 459 the shelf's growth banks)",
     "// the same 474 the shelf's growth banks)"),
    ("// (the 459 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 474 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// clinit order banks, the END_PORTAL the displaced 460 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 475 keeps)"),
    ("// the same 459 the cloths' growth banks)",
     "// the same 474 the cloths' growth banks)"),
    ("// 460 the displacement law keeps).",
     "// 475 the displacement law keeps)."),
    ("// 442/443, the END_PORTAL the beyond-table 460 the",
     "// 442/443, the END_PORTAL the beyond-table 475 the"),
    # the 679 round's own current-tail lines the 680 growth re-banks
    ("// early seat keeps, the END_PORTAL the displaced 460 keeps)",
     "// early seat keeps, the END_PORTAL the displaced 475 keeps)"),
    ("// displacement keeps (the 460 the `id < N_KINDS` guards the",
     "// displacement keeps (the 475 the `id < N_KINDS` guards the"),
    ("// the same 459 the kilns' growth banks)",
     "// the same 474 the kilns' growth banks)"),
    ("// the same 459 the rungs' growth banks)",
     "// the same 474 the rungs' growth banks)"),
    ("// serves — 444..458, the END_PORTAL the beyond-table 460 the",
     "// serves — 444..458, the END_PORTAL the beyond-table 475 the"),
]

# the multi-line palette-pin rows the `459,`/`460,` forms alone wear
STANDALONE_459 = re.compile(r"^(\s+)459,$", re.M)
STANDALONE_460 = re.compile(r"^(\s+)460,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_FORMS:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_459.findall(src))
    if c:
        src = STANDALONE_459.sub(r"\g<1>474,", src)
        n += c
    c = len(STANDALONE_460.findall(src))
    if c:
        src = STANDALONE_460.sub(r"\g<1>475,", src)
        n += c
    open(path, "w").write(src)
    return n


total = walk(WORLD) + walk(SCREENS)
print(f"pins corrected: {total}")
