#!/usr/bin/env python3
"""dev-54 pin walk: the note block's single seat the 681 gap-id fills —
the END_PORTAL the beyond-table 498 the displacement re-banks, the
palette/NAME_OF lengths the 497 rows the growth answers. The
historical docs (the END_PORTAL chain's own per-round sentences —
the 681 round's `beyond-table 497 the same law keeps` row the
world.rs 954 doc keeps, the javap pc numbers, the 3428 `table the
497 rows` row the round's own NAME_OF doc wears) the walk never
touches — the exact-form replacement the 681 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

# the END_PORTAL seat's own forms (497 -> 498) — the FIRST walk
# (the 496-length forms' product never re-walks)
PIN_497 = [
    ("END_PORTAL, 497,", "END_PORTAL, 498,"),
    ("assert_eq!(END_PORTAL, 497);", "assert_eq!(END_PORTAL, 498);"),
    ("assert_eq!(w::END_PORTAL, 497);", "assert_eq!(w::END_PORTAL, 498);"),
    # the current-tail comment lines the 681 precedent re-banks
    ("// 497 the displacement law keeps).",
     "// 498 the displacement law keeps)."),
    ("// 442/443, the END_PORTAL the beyond-table 497 the",
     "// 442/443, the END_PORTAL the beyond-table 498 the"),
    ("// serves — 444..458, the END_PORTAL the beyond-table 497 the",
     "// serves — 444..458, the END_PORTAL the beyond-table 498 the"),
    ("// beyond-table 497 the displacement law keeps).",
     "// beyond-table 498 the displacement law keeps)."),
    ("// family order banks, the END_PORTAL the displaced 497 keeps)",
     "// family order banks, the END_PORTAL the displaced 498 keeps)"),
    ("// stone shelf order banks, the END_PORTAL the displaced 497",
     "// stone shelf order banks, the END_PORTAL the displaced 498"),
    ("// desert order banks, the END_PORTAL the displaced 497 keeps)",
     "// desert order banks, the END_PORTAL the displaced 498 keeps)"),
    ("// clinit order banks, the END_PORTAL the displaced 497 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 498 keeps)"),
    ("// early seat keeps, the END_PORTAL the displaced 497 keeps)",
     "// early seat keeps, the END_PORTAL the displaced 498 keeps)"),
    ("// displacement keeps (the 497 the `id < N_KINDS` guards the",
     "// displacement keeps (the 498 the `id < N_KINDS` guards the"),
    ("// beyond-table seat the displacement keeps (the 497 the",
     "// beyond-table seat the displacement keeps (the 498 the"),
]

# the lengths' own forms (496 -> 497) — the SECOND walk
PIN_496 = [
    ("assert_eq!(atlas::N_KINDS, 496,", "assert_eq!(atlas::N_KINDS, 497,"),
    ("NAME_OF.len(), 496,", "NAME_OF.len(), 497,"),
    ("// 496 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 497 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// refuses, the table lengths the same 496 the one law keeps)",
     "// refuses, the table lengths the same 497 the one law keeps)"),
    ("// displacement keeps (the 496 the `id < N_KINDS` guards the",
     "// displacement keeps (the 497 the `id < N_KINDS` guards the"),
    ("// the same 496 the kit's growth banks)",
     "// the same 497 the kit's growth banks)"),
    ("// the same 496 the shelf's growth banks)",
     "// the same 497 the shelf's growth banks)"),
    ("// (the 496 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 497 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// the same 496 the cloths' growth banks)",
     "// the same 497 the cloths' growth banks)"),
    ("// the same 496 the kilns' growth banks)",
     "// the same 497 the kilns' growth banks)"),
    ("// the same 496 the rungs' growth banks)",
     "// the same 497 the rungs' growth banks)"),
    ("// the same 496 the ladder's growth banks)",
     "// the same 497 the ladder's growth banks)"),
]

STANDALONE_497 = re.compile(r"^(\s+)497,$", re.M)
STANDALONE_496 = re.compile(r"^(\s+)496,$", re.M)


def walk(path):
    src = open(path).read()
    n497 = n496 = 0
    for old, new in PIN_497:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n497 += c
    c = len(STANDALONE_497.findall(src))
    if c:
        src = STANDALONE_497.sub(r"\g<1>498,", src)
        n497 += c
    for old, new in PIN_496:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n496 += c
    c = len(STANDALONE_496.findall(src))
    if c:
        src = STANDALONE_496.sub(r"\g<1>497,", src)
        n496 += c
    open(path, "w").write(src)
    print(f"{path}: {n497} x 497->498 (END_PORTAL), {n496} x 496->497 (lengths)")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 496/497 mention the manual read wears
print("\n=== AUDIT: remaining 496/497 mentions (manual read) ===")
for path in (WORLD, SCREENS):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"\b49[67]\b", line):
            # the historical/docs the walk never touches — print for read
            print(f"{path}:{i}: {line.strip()[:110]}")
