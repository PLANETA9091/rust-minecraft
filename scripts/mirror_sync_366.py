#!/usr/bin/env python3
"""autopush-366 — THE MIRROR SYNC (the 358/363 record's own law the
worklog's dual files wear): the authoritative entry the mirror's tail
appends — the rfind slice the last entry's head the anchor keeps, the
pure append the older bytes never touch.
"""
AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

anchor = "\n--- Task ID: autopush-365 ("
i = auth.rfind(anchor)
assert i >= 0, "the 365 anchor the authority never wears"
addition = auth[i:]
assert "autopush-366" in addition, "the 366 entry the slice wears"

mirror += addition
open(MIRROR, "w", encoding="utf-8").write(mirror)
print(f"MIRROR SYNCED +{len(addition)} bytes; entries now {mirror.count(chr(10)+'--- Task ID: autopush-')}+1")
