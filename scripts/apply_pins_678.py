#!/usr/bin/env python3
"""autopush-678 pin walk: the current-tail 425 rows the 442 seats the
round's growth re-banks. The historical docs (the wool cloths' own
displacement row, the javap pc numbers 373-425/405-425/4250-4262, the
autopush-425 round's own number) the walk never touches — the
every-form replacement the 677 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 425,", "END_PORTAL, 442,"),
    ("assert_eq!(atlas::N_KINDS, 425,", "assert_eq!(atlas::N_KINDS, 442,"),
    ("atlas::N_KINDS, 425,", "atlas::N_KINDS, 442,"),
    ("assert_eq!(END_PORTAL, 425);", "assert_eq!(END_PORTAL, 442);"),
    ("assert_eq!(w::END_PORTAL, 425);", "assert_eq!(w::END_PORTAL, 442);"),
    ("NAME_OF.len(), 425,", "NAME_OF.len(), 442,"),
    # the current-tail comment lines the 677 precedent re-banks
    ("// 425 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 442 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 425 keeps)",
     "// family order banks, the END_PORTAL the displaced 442 keeps)"),
    ("// refuses, the table lengths the same 425 the one law keeps)",
     "// refuses, the table lengths the same 442 the one law keeps)"),
    ("// displacement keeps (the 425 the `id < N_KINDS` guards the",
     "// displacement keeps (the 442 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 425",
     "// stone shelf order banks, the END_PORTAL the displaced 442"),
    ("// the same 425 the kit's growth banks)",
     "// the same 442 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 425 keeps)",
     "// desert order banks, the END_PORTAL the displaced 442 keeps)"),
    ("// the same 425 the shelf's growth banks)",
     "// the same 442 the shelf's growth banks)"),
    ("// (the 425 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 442 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// clinit order banks, the END_PORTAL the displaced 425 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 442 keeps)"),
    ("// the same 425 the cloths' growth banks)",
     "// the same 442 the cloths' growth banks)"),
    ("// 425 the displacement law keeps).",
     "// 442 the displacement law keeps)."),
]

# screens.rs standalone palette pins: the multi-line palette-pin rows
# the `425,` form alone wears (the 677 diff's `-            425,` rows).
SCREENS_STANDALONE = re.compile(r"^(\s+)425,$", re.M)


def walk(path):
    src = open(path).read()
    n = 0
    for old, new in PIN_FORMS:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    return src, n


def main():
    total = 0
    src, n = walk(WORLD)
    open(WORLD, "w").write(src)
    print(f"{WORLD}: {n} replacements")
    total += n

    src, n = walk(SCREENS)
    src, k = SCREENS_STANDALONE.subn(r"\g<1>442,", src)
    open(SCREENS, "w").write(src)
    print(f"{SCREENS}: {n} form + {k} standalone replacements")
    total += n + k
    print(f"total: {total}")


if __name__ == "__main__":
    main()
