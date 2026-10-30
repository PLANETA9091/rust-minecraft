#!/usr/bin/env python3
"""autopush-363 — the worklog mirror sync (the 358/359/360/361/362
record's own law the round wears): the authoritative
/home/z/my-project/worklog.md the logs/worklog.md mirror the rfind
slice answers — the pure append the round's own marker speaks, the
head the mirror never rewrites."""
AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"
MARKER = "\n--- Task ID: autopush-363 ("

auth = open(AUTH, encoding="utf-8").read()
cut = auth.rfind(MARKER)
assert cut >= 0, "the round's own marker the head lacks"
tail = auth[cut:]

mirror = open(MIRROR, encoding="utf-8").read()
assert MARKER.strip()[:40] not in mirror, "the mirror the round already wears"
if not mirror.endswith("\n"):
    mirror += "\n"
open(MIRROR, "w", encoding="utf-8").write(mirror + tail)
print(f"mirror synced: {len(tail)} bytes appended (the pure append the marker keeps)")
