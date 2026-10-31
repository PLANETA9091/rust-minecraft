#!/usr/bin/env python3
"""dev-205 — THE MIRROR SYNC (the 358/366 record's own law the worklog's
dual files wear): the authoritative dev-205 entry the mirror's tail
appends — the rfind anchor the dev-204 head keeps, the slice the pure
append wears (the mirror the 366 mirror-commits the head state keeps —
the dev entries the mirror already wears through dev-204, the 366
autopush entry the mirror's own tail banks first).
"""
AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

# the mirror's last entry head the anchor keeps — the authority's tail
# beyond the anchor's OWN entry end the slice wears. The dev-205 entry
# the authority banks AFTER the autopush-366 entry; the mirror's tail
# (through autopush-366) the anchor speaks. Slice from the authority's
# OWN mirror-tail alignment: find the mirror's last entry head in the
# authority, take everything AFTER that entry's full body — i.e. from
# the dev-205 head on.
head = "\n--- Task ID: dev-205 ("
i = auth.find(head)
assert i >= 0, "the dev-205 head the authority wears"
addition = auth[i:]

# the mirror must not already wear the dev-205 entry
assert head not in mirror, "the dev-205 head the mirror already wears"

mirror += addition
open(MIRROR, "w", encoding="utf-8").write(mirror)
print(f"MIRROR SYNCED +{len(addition)} bytes")
