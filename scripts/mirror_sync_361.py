#!/usr/bin/env python3
"""autopush-361 — the worklog mirror's own pure-append sync (the 358
law the rfind slice keeps): the authoritative tail the marker's last
wearing answers, the mirror the same slice grows, the foreign body
the diff never touches."""
AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "\n--- Task ID: autopush-361 ("
auth = open(AUTH, encoding="utf-8").read()
i = auth.rfind(MARK)
assert i >= 0, "marker not found in the authoritative worklog"
tail = auth[i:]
mir = open(MIRROR, encoding="utf-8").read()
if MARK in mir:
    print("mirror already wears the entry — no-op")
else:
    if not mir.endswith("\n"):
        mir += "\n"
    open(MIRROR, "w", encoding="utf-8").write(mir + tail.lstrip("\n"))
    print(f"mirror grew by {len(tail)} chars")
