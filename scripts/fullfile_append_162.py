#!/usr/bin/env python3
"""THE FULL-FILE LAWS' OWN APPEND (dev-162): the ten byte-exact
assertions the debugOptions row the END appends — the save walk's own
order (the switchGameMode row the debugOptions the row's AFTER banks)
— and the ends_with laws the row the file's own END wear.

Usage: python3 scripts/fullfile_append_162.py
"""
PATHS = [
    "/home/z/my-project/rust-minecraft/crates/mc-client/src/screens/tests/part_4.rs",
    "/home/z/my-project/rust-minecraft/crates/mc-client/src/options_file.rs",
]

OLD_FULL = 'key_key.debug.switchGameMode:key.keyboard.f4\\n",'
NEW_FULL = (
    "key_key.debug.switchGameMode:key.keyboard.f4\\n"
    'key_key.debug.debugOptions:key.keyboard.f6\\n",'
)
OLD_ENDS = '.ends_with("key_key.debug.switchGameMode:key.keyboard.f4")'
NEW_ENDS = '.ends_with("key_key.debug.debugOptions:key.keyboard.f6")'

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
print(f"TOTAL: full-file {total_full} (expect 10), ends_with {total_ends} (expect 6)")
