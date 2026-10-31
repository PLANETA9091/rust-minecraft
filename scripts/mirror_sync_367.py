#!/usr/bin/env python3
"""autopush-367 — THE MIRROR SYNC (the 358/366/367 record's own law the
worklog's dual files wear): the authoritative autopush-367 entry the
mirror's tail appends — the find anchor the 367 head keeps (the
dedup lesson the 366 wore: the mirror already wears the 365 entry, the
367 head the anchor the tail alone slices).
"""
AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

head = "\n--- Task ID: autopush-367 ("
i = auth.find(head)
assert i >= 0, "the 367 head the authority wears"
addition = auth[i:]
assert head not in mirror, "the 367 head the mirror already wears"

mirror += addition
open(MIRROR, "w", encoding="utf-8").write(mirror)
print(f"MIRROR SYNCED +{len(addition)} bytes")
