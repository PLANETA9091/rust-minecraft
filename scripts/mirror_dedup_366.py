#!/usr/bin/env python3
"""autopush-366 — THE MIRROR DEDUP (the duplicate 365 entry the slice
anchor wore: the rfind("autopush-365") the already-present entry re-
appended): the SECOND 365 copy the 366 head the boundary keeps — the
copy the between them deletes, the mirror the authority's own count
wears (46 the both files speak).
"""
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

text = open(MIRROR, encoding="utf-8").read()

h365 = "\n--- Task ID: autopush-365 ("
h366 = "\n--- Task ID: autopush-366 ("

first = text.find(h365)
second = text.find(h365, first + 1)
dup = second >= 0
assert dup, "no duplicate 365 head the mirror wears"
tail366 = text.find(h366, second)
assert tail366 > second, "the 366 head the duplicate's tail wears"

before = text.count("\n--- Task ID: autopush-")
fixed = text[:second] + text[tail366:]
after = fixed.count("\n--- Task ID: autopush-")
open(MIRROR, "w", encoding="utf-8").write(fixed)
print(f"dedup: entries {before} -> {after} (the second 365 copy retired)")
