#!/usr/bin/env python3
"""dev-53 pin walk: the current-tail 442 rows the 444 seats the
round's growth re-banks. The historical docs (the terracotta kilns'
own displacement row, the javap pc numbers 373-425/405-425, the
autopush-425 round's own number, the cp@244/245 pitch floats) the
walk never touches — the every-form replacement the 678 precedent
wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 442,", "END_PORTAL, 444,"),
    ("assert_eq!(atlas::N_KINDS, 442,", "assert_eq!(atlas::N_KINDS, 444,"),
    ("atlas::N_KINDS, 442,", "atlas::N_KINDS, 444,"),
    ("assert_eq!(END_PORTAL, 442);", "assert_eq!(END_PORTAL, 444);"),
    ("assert_eq!(w::END_PORTAL, 442);", "assert_eq!(w::END_PORTAL, 444);"),
    ("NAME_OF.len(), 442,", "NAME_OF.len(), 444,"),
    # the current-tail comment lines the 678 precedent re-banks
    ("// 442 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 444 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 442 keeps)",
     "// family order banks, the END_PORTAL the displaced 444 keeps)"),
    ("// refuses, the table lengths the same 442 the one law keeps)",
     "// refuses, the table lengths the same 444 the one law keeps)"),
    ("// displacement keeps (the 442 the `id < N_KINDS` guards the",
     "// displacement keeps (the 444 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 442",
     "// stone shelf order banks, the END_PORTAL the displaced 444"),
    ("// the same 442 the kit's growth banks)",
     "// the same 444 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 442 keeps)",
     "// desert order banks, the END_PORTAL the displaced 444 keeps)"),
    ("// the same 442 the shelf's growth banks)",
     "// the same 444 the shelf's growth banks)"),
    ("// (the 442 the `id < N_KINDS` guards the bigger tail banks),",
     "// (the 444 the `id < N_KINDS` guards the bigger tail banks),"),
    ("// clinit order banks, the END_PORTAL the displaced 442 keeps)",
     "// clinit order banks, the END_PORTAL the displaced 444 keeps)"),
    ("// the same 442 the cloths' growth banks)",
     "// the same 444 the cloths' growth banks)"),
    ("// 442 the displacement law keeps).",
     "// 444 the displacement law keeps)."),
]

# screens.rs standalone palette pins: the multi-line palette-pin rows
# the `442,` form alone wears (the 678 diff's `-            442,` rows).
SCREENS_STANDALONE = re.compile(r"^(\s+)442,$", re.M)


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
    src, k = SCREENS_STANDALONE.subn(r"\g<1>444,", src)
    open(SCREENS, "w").write(src)
    print(f"{SCREENS}: {n} form + {k} standalone replacements")
    total += n + k
    print(f"total: {total}")


if __name__ == "__main__":
    main()
