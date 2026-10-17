#!/usr/bin/env python3
"""autopush-681 pin walk: the current-tail 474 rows the 497 seats the
round's growth re-banks. The historical docs (the END_PORTAL chain's
own per-round sentences, the concrete/powder families' javap evidence
rows, the javap pc numbers 20979-21046/6275-6309/12912-26212, the pc
459-462 nether pairs, the round numbers the DIFFERENT contexts the
walk never touches) the walk never touches — the every-form
replacement the 680 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 475,", "END_PORTAL, 497,"),
    ("assert_eq!(atlas::N_KINDS, 474,", "assert_eq!(atlas::N_KINDS, 496,"),
    ("atlas::N_KINDS, 474,", "atlas::N_KINDS, 496,"),
    ("assert_eq!(END_PORTAL, 475);", "assert_eq!(END_PORTAL, 497);"),
    ("assert_eq!(w::END_PORTAL, 475);", "assert_eq!(w::END_PORTAL, 497);"),
    ("NAME_OF.len(), 474,", "NAME_OF.len(), 496,"),
    # the current-tail comment lines the 680 precedent re-banks
    ("// 474 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 496 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 475 keeps)",
     "// family order banks, the END_PORTAL the displaced 497 keeps)"),
    ("// refuses, the table lengths the same 474 the one law keeps)",
     "// refuses, the table lengths the same 496 the one law keeps)"),
    ("// displacement keeps (the 474 the `id < N_KINDS` guards the",
     "// displacement keeps (the 496 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 475",
     "// stone shelf order banks, the END_PORTAL the displaced 497"),
    ("// the same 474 the kit's growth banks)",
     "// the same 496 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 475 keeps)",
     "// desert order banks, the END_PORTAL the displaced 497 keeps)"),
    ("// the same 474 the shelf's growth banks)",
     "// the same 496 the shelf's growth banks)"),
    ("// (the 474 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 496 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// clinit order banks, the END_PORTAL the displaced 475 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 497 keeps)"),
    ("// the same 474 the cloths' growth banks)",
     "// the same 496 the cloths' growth banks)"),
    ("// 475 the displacement law keeps).",
     "// 497 the displacement law keeps)."),
    ("// 442/443, the END_PORTAL the beyond-table 475 the",
     "// 442/443, the END_PORTAL the beyond-table 497 the"),
    # the 680 round's own current-tail lines the 681 growth re-banks
    ("// early seat keeps, the END_PORTAL the displaced 475 keeps)",
     "// early seat keeps, the END_PORTAL the displaced 497 keeps)"),
    ("// displacement keeps (the 475 the `id < N_KINDS` guards the",
     "// displacement keeps (the 497 the `id < N_KINDS` guards the"),
    ("// the same 474 the kilns' growth banks)",
     "// the same 496 the kilns' growth banks)"),
    ("// the same 474 the rungs' growth banks)",
     "// the same 496 the rungs' growth banks)"),
    ("// serves — 444..458, the END_PORTAL the beyond-table 475 the",
     "// serves — 444..458, the END_PORTAL the beyond-table 497 the"),
]

# the multi-line palette-pin rows the `474,`/`475,` forms alone wear
STANDALONE_474 = re.compile(r"^(\s+)474,$", re.M)
STANDALONE_475 = re.compile(r"^(\s+)475,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_FORMS:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    c = len(STANDALONE_474.findall(src))
    if c:
        src = STANDALONE_474.sub(r"\g<1>496,", src)
        n += c
    c = len(STANDALONE_475.findall(src))
    if c:
        src = STANDALONE_475.sub(r"\g<1>497,", src)
        n += c
    open(path, "w").write(src)
    return n


total = walk(WORLD) + walk(SCREENS)
print(f"pins corrected: {total}")
