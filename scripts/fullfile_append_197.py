#!/usr/bin/env python3
"""THE FULL-FILE LAWS' OWN APPEND (autopush-197): the nine byte-exact
assertions the copyRecreate row the END appends — the save walk's own
order (the advTooltips row the copyRecreate the row's AFTER banks).

Usage: python3 scripts/fullfile_append_197.py
"""
PATH = "/home/z/my-project/rust-minecraft/crates/mc-client/src/screens/tests/part_4.rs"

OLD = 'key_key.debug.showAdvancedTooltips:key.keyboard.h\\n",'
NEW = (
    "key_key.debug.showAdvancedTooltips:key.keyboard.h\\n"
    'key_key.debug.copyRecreateCommand:key.keyboard.i\\n",'
)

src = open(PATH).read()
n = src.count(OLD)
open(PATH, "w").write(src.replace(OLD, NEW))
print(f"replaced {n} full-file assertion ends")
