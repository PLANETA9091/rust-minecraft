#!/usr/bin/env python3
"""autopush-677 pin walk: the current-tail 410 rows the 425 seats the
round's growth re-banks. The historical docs (the sandstone shelf's own
displacement row, the new consts' self-referential rows, the TNT round's
own number, the bw_sample_color gradient probe) the walk never touches —
the every-form replacement the 676 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 410,", "END_PORTAL, 425,"),
    ("assert_eq!(atlas::N_KINDS, 410,", "assert_eq!(atlas::N_KINDS, 425,"),
    ("atlas::N_KINDS, 410,", "atlas::N_KINDS, 425,"),
    ("assert_eq!(END_PORTAL, 410);", "assert_eq!(END_PORTAL, 425);"),
    ("assert_eq!(w::END_PORTAL, 410);", "assert_eq!(w::END_PORTAL, 425);"),
    ("NAME_OF.len(), 410,", "NAME_OF.len(), 425,"),
    # the current-tail comment lines the 676 precedent re-banks
    ("// 410 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 425 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 410 keeps)",
     "// family order banks, the END_PORTAL the displaced 425 keeps)"),
    ("// refuses, the table lengths the same 410 the one law keeps)",
     "// refuses, the table lengths the same 425 the one law keeps)"),
    ("// displacement keeps (the 410 the `id < N_KINDS` guards the",
     "// displacement keeps (the 425 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 410",
     "// stone shelf order banks, the END_PORTAL the displaced 425"),
    ("// the same 410 the kit's growth banks)",
     "// the same 425 the kit's growth banks)"),
    ("// desert order banks, the END_PORTAL the displaced 410 keeps)",
     "// desert order banks, the END_PORTAL the displaced 425 keeps)"),
    ("// the same 410 the shelf's growth banks)",
     "// the same 425 the shelf's growth banks)"),
]

# screens.rs standalone palette pins: the multi-line palette-pin rows
# the `410,` form alone wears (the 676 diff's `-            410,` rows).
SCREENS_STANDALONE = re.compile(r"^(\s+)410,$", re.M)


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
    src, k = SCREENS_STANDALONE.subn(r"\g<1>425,", src)
    open(SCREENS, "w").write(src)
    print(f"{SCREENS}: {n} form + {k} standalone replacements")
    total += n + k
    print(f"total: {total}")


if __name__ == "__main__":
    main()
