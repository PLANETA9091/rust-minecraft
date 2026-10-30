#!/usr/bin/env python3
"""dev-201 — THE FMT SPLICE (the rustfmt re-lays the round's own
regions wear): the offline swap arm's one-liner, the test asserts'
joined pairs, the long assert's multiline form — the probe's dirty
hunks the exact replacements answer, the pre-existing static hunks
the HEAD law never touches."""

SWAP_ONE_LINER = (
    "            self.items_potion_kinds\n"
    "                .swap(sel, OFFHAND_KIND_SEAT);"
)
SWAP_JOINED = "            self.items_potion_kinds.swap(sel, OFFHAND_KIND_SEAT);"

PAIRS = [
    # screens.rs — the offline arm's one-liner
    (
        "crates/mc-client/src/screens.rs",
        SWAP_ONE_LINER,
        SWAP_JOINED,
        True,
    ),
    # part_11.rs — the [OFFHAND_KIND_SEAT] assert pairs the joined
    # form wears (the none pair twice, the kind_a pair once)
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "        app.items_potion_kinds[OFFHAND_KIND_SEAT],\n        none,",
        "        app.items_potion_kinds[OFFHAND_KIND_SEAT], none,",
        True,
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "        app.items_potion_kinds[OFFHAND_KIND_SEAT],\n        kind_a,",
        "        app.items_potion_kinds[OFFHAND_KIND_SEAT], kind_a,",
        True,
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "        live.items_potion_kinds[OFFHAND_KIND_SEAT],\n        none,",
        "        live.items_potion_kinds[OFFHAND_KIND_SEAT], none,",
        True,
    ),
    # the long assert the multiline form wears
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        '    assert_eq!(app.inventory.items[6], (dirt, 6), "the one off the seat keeps");',
        "    assert_eq!(\n"
        "        app.inventory.items[6],\n"
        "        (dirt, 6),\n"
        '        "the one off the seat keeps"\n'
        "    );",
        False,
    ),
]

for path, old, new, replace_all in PAIRS:
    text = open(path).read()
    n = text.count(old)
    if n == 0:
        raise SystemExit(f"{path}: pattern not found: {old[:60]!r}")
    if not replace_all and n > 1:
        raise SystemExit(f"{path}: pattern ambiguous ({n}x): {old[:60]!r}")
    open(path, "w").write(text.replace(old, new))
    print(f"{path}: {n}x spliced")
print("ALL SPLICES DONE")
