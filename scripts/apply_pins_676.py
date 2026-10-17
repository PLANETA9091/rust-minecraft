#!/usr/bin/env python3
"""autopush-676 pin walk: the current-tail 402 rows the 410 seats the
round's growth re-banks. The historical docs (the stone kit's own
displacement row, the new consts' self-referential rows) the walk never
touches — the every-form replacement the 675 precedent wears."""

import re

WORLD = "crates/mc-client/src/world.rs"
SCREENS = "crates/mc-client/src/screens.rs"

PIN_FORMS = [
    # (old, new) — the whole-line forms the tests' tail assertions wear
    ("END_PORTAL, 402,", "END_PORTAL, 410,"),
    ("assert_eq!(atlas::N_KINDS, 402,", "assert_eq!(atlas::N_KINDS, 410,"),
    ("atlas::N_KINDS, 402,", "atlas::N_KINDS, 410,"),
    ("assert_eq!(END_PORTAL, 402);", "assert_eq!(END_PORTAL, 410);"),
    ("assert_eq!(w::END_PORTAL, 402);", "assert_eq!(w::END_PORTAL, 410);"),
    ("NAME_OF.len(), 402,", "NAME_OF.len(), 410,"),
    # the current-tail comment lines the 675 precedent re-banks
    ("// 402 the `id < N_KINDS` guards the bigger tail banks), the",
     "// 410 the `id < N_KINDS` guards the bigger tail banks), the"),
    ("// family order banks, the END_PORTAL the displaced 402 keeps)",
     "// family order banks, the END_PORTAL the displaced 410 keeps)"),
    ("// refuses, the table lengths the same 402 the one law keeps)",
     "// refuses, the table lengths the same 410 the one law keeps)"),
    ("// displacement keeps (the 402 the `id < N_KINDS` guards the",
     "// displacement keeps (the 410 the `id < N_KINDS` guards the"),
    ("// stone shelf order banks, the END_PORTAL the displaced 402",
     "// stone shelf order banks, the END_PORTAL the displaced 410"),
    ("// the same 402 the kit's growth banks)",
     "// the same 410 the kit's growth banks)"),
]

# screens.rs standalone palette pins: the multi-line palette-pin rows
# the `402,` form alone wears (the 675 diff's `-            396,` rows).
SCREENS_STANDALONE = re.compile(r"^(\s+)402,$", re.M)


def walk(path, extra_forms=True):
    src = open(path).read()
    n = 0
    for old, new in PIN_FORMS:
        c = src.count(old)
        if c:
            src = src.replace(old, new)
            n += c
    if not extra_forms:
        pass
    return src, n


def main():
    total = 0
    src, n = walk(WORLD)
    open(WORLD, "w").write(src)
    print(f"{WORLD}: {n} replacements")
    total += n

    src, n = walk(SCREENS, extra_forms=False)
    src, k = SCREENS_STANDALONE.subn(r"\g<1>410,", src)
    open(SCREENS, "w").write(src)
    print(f"{SCREENS}: {n} form + {k} standalone replacements")
    total += n + k
    print(f"total: {total}")


if __name__ == "__main__":
    main()
