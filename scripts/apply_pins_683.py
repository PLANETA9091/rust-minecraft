#!/usr/bin/env python3
"""dev-55 pin walk: the fence family's THIRTEEN seats the palette's
tail banks — the END_PORTAL the beyond-table 510 the displacement
re-banks, the palette/NAME_OF lengths the 510 rows the growth
answers. The historical docs (the END_PORTAL chain's own per-round
sentences — the 682 round's `beyond-table 498 the same law keeps`
row the world.rs 957 doc keeps, the javap pc numbers, the 3428
`table the 497 rows` row the round's own NAME_OF doc wears) the
walk never touches — the exact-form replacement the 682 precedent
wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"
MESHER = "crates/mc-client/src/mesher.rs"

# the END_PORTAL seat's own forms (498 -> 510) — the FIRST walk
# (the 497-length forms' product never re-walks)
PIN_498 = [
    ("END_PORTAL, 498,", "END_PORTAL, 510,"),
    ("assert_eq!(END_PORTAL, 498);", "assert_eq!(END_PORTAL, 510);"),
    ("assert_eq!(w::END_PORTAL, 498);", "assert_eq!(w::END_PORTAL, 510);"),
    # the current-tail comment lines the 682 precedent re-banks
    ("// 498 the displacement law keeps).",
     "// 510 the displacement law keeps)."),
    ("// 442/443, the END_PORTAL the beyond-table 498 the",
     "// 442/443, the END_PORTAL the beyond-table 510 the"),
    ("// serves — 444..458, the END_PORTAL the beyond-table 498 the",
     "// serves — 444..458, the END_PORTAL the beyond-table 510 the"),
    ("// beyond-table 498 the displacement law keeps).",
     "// beyond-table 510 the displacement law keeps)."),
    ("// family order banks, the END_PORTAL the displaced 498 keeps)",
     "// family order banks, the END_PORTAL the displaced 510 keeps)"),
    ("// stone shelf order banks, the END_PORTAL the displaced 498",
     "// stone shelf order banks, the END_PORTAL the displaced 510"),
    ("// desert order banks, the END_PORTAL the displaced 498 keeps)",
     "// desert order banks, the END_PORTAL the displaced 510 keeps)"),
    ("// clinit order banks, the END_PORTAL the displaced 498 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 510 keeps)"),
    ("// early seat keeps, the END_PORTAL the displaced 498 keeps)",
     "// early seat keeps, the END_PORTAL the displaced 510 keeps)"),
    ("// displacement keeps (the 498 the `id < N_KINDS` guards the",
     "// displacement keeps (the 510 the `id < N_KINDS` guards the"),
    ("// beyond-table seat the displacement keeps (the 498 the",
     "// beyond-table seat the displacement keeps (the 510 the"),
    ("// beyond-table 498 the displacement keeps, the dig row the",
     "// beyond-table 510 the displacement keeps, the dig row the"),
    ("// the kind's own seat the beyond-table 498 the same law keeps).",
     "// the kind's own seat the beyond-table 510 the same law keeps)."),
    ("/// beyond-table 498 the same displacement answers).",
     "/// beyond-table 510 the same displacement answers)."),
]

# the lengths' own forms (497 -> 510) — the SECOND walk
PIN_497 = [
    ("assert_eq!(atlas::N_KINDS, 497,", "assert_eq!(atlas::N_KINDS, 510,"),
    ("NAME_OF.len(), 497,", "NAME_OF.len(), 510,"),
    ("atlas::N_KINDS, 497,", "atlas::N_KINDS, 510,"),
    ("// 497 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 510 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// refuses, the table lengths the same 497 the one law keeps)",
     "// refuses, the table lengths the same 510 the one law keeps)"),
    ("// displacement keeps (the 497 the `id < N_KINDS` guards the",
     "// displacement keeps (the 510 the `id < N_KINDS` guards the"),
    ("// the same 497 the kit's growth banks)",
     "// the same 510 the kit's growth banks)"),
    ("// the same 497 the shelf's growth banks)",
     "// the same 510 the shelf's growth banks)"),
    ("// (the 497 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 510 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// the same 497 the cloths' growth banks)",
     "// the same 510 the cloths' growth banks)"),
    ("// the same 497 the kilns' growth banks)",
     "// the same 510 the kilns' growth banks)"),
    ("// the same 497 the rungs' growth banks)",
     "// the same 510 the rungs' growth banks)"),
    ("// the same 497 the ladder's growth banks)",
     "// the same 510 the ladder's growth banks)"),
    ("// the same 497 the note's growth banks)",
     "// the same 510 the note's growth banks)"),
]

STANDALONE_498 = re.compile(r"^(\s+)498,$", re.M)
STANDALONE_497 = re.compile(r"^(\s+)497,$", re.M)


def walk(path):
    src = open(path).read()
    n498 = n497 = 0
    for old, new in PIN_498:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n498 += c
    c = len(STANDALONE_498.findall(src))
    if c:
        src = STANDALONE_498.sub(r"\g<1>510,", src)
        n498 += c
    for old, new in PIN_497:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n497 += c
    c = len(STANDALONE_497.findall(src))
    if c:
        src = STANDALONE_497.sub(r"\g<1>510,", src)
        n497 += c
    open(path, "w").write(src)
    print(f"{path}: {n498} x 498->510 (END_PORTAL), {n497} x 497->510 (lengths)")


walk(WORLD)
walk(SCREENS)

# THE AUDIT: every remaining 497/498 mention the manual read wears
print("\n=== AUDIT: remaining 497/498 mentions (manual read) ===")
for path in (WORLD, SCREENS, MESHER):
    for i, line in enumerate(open(path).read().splitlines(), 1):
        if re.search(r"\b49[78]\b", line):
            # the historical/docs the walk never touches — print for read
            print(f"{path}:{i}: {line.strip()[:110]}")
