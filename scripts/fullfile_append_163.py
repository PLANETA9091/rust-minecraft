#!/usr/bin/env python3
"""THE FULL-FILE LAWS' OWN APPEND (dev-163): the ten byte-exact
assertions the dumpDynamicTextures row the END appends — the save
walk's own order (the focusPause row the dumpDynamicTextures the
row's AFTER banks) — and the ends_with laws the row the file's own
END wear.

Usage: python3 scripts/fullfile_append_163.py
"""
PATHS = [
    "/home/z/my-project/rust-minecraft/crates/mc-client/src/screens/tests/part_4.rs",
    "/home/z/my-project/rust-minecraft/crates/mc-client/src/options_file.rs",
]

OLD_FULL = 'key_key.debug.focusPause:key.keyboard.p\\n",'
NEW_FULL = (
    "key_key.debug.focusPause:key.keyboard.p\\n"
    'key_key.debug.dumpDynamicTextures:key.keyboard.s\\n",'
)
OLD_ENDS = '.ends_with("key_key.debug.focusPause:key.keyboard.p")'
NEW_ENDS = '.ends_with("key_key.debug.dumpDynamicTextures:key.keyboard.s")'

total_full = 0
total_ends = 0
for path in PATHS:
    src = open(path).read()
    n_full = src.count(OLD_FULL)
    n_ends = src.count(OLD_ENDS)
    src = src.replace(OLD_FULL, NEW_FULL).replace(OLD_ENDS, NEW_ENDS)
    open(path, "w").write(src)
    total_full += n_full
    total_ends += n_ends
    print(f"{path}: full-file {n_full}, ends_with {n_ends}")
print(f"TOTAL: full-file {total_full} (expect 10), ends_with {total_ends} (expect 5)")
