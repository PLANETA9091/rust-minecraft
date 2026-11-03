#!/usr/bin/env python3
"""autopush-438 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_438.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

with open(AUTH, encoding="utf-8") as f:
    auth = f.read()
i = auth.index("--- Task ID: autopush-438 ")
ENTRY = auth[i:]

MARK = "--- Task ID: autopush-438 "

mirror = open(MIRROR, encoding="utf-8").read()
assert MARK not in mirror, "the mirror the round's own marker already wears"
assert auth.count(MARK) == 1, "the authoritative file the marker once wears"
assert auth.endswith(ENTRY), "the authoritative tail the entry wears"
assert mirror.endswith("\n"), "the mirror's own tail the newline keeps"
head_mirror = mirror[:200]
head_auth = auth[:200]

new_mirror = mirror + ENTRY
open(MIRROR, "w", encoding="utf-8").write(new_mirror)

check_auth = open(AUTH, encoding="utf-8").read()
check_mirror = open(MIRROR, encoding="utf-8").read()
assert check_auth.startswith(head_auth), "the authoritative head the sync never moves"
assert check_mirror.startswith(head_mirror), "the mirror's own head the sync never moves"
assert check_auth.endswith(ENTRY), "the authoritative tail the entry wears"
assert check_mirror.endswith(ENTRY), "the mirror's own tail the entry wears"

n_auth = check_auth.count("\n--- Task ID: autopush-")
n_mirror = check_mirror.count("\n--- Task ID: autopush-")
assert n_auth == n_mirror == 118, f"the entry count the 118 wears ({n_auth}/{n_mirror})"
print(f"worklog_438: both files at {n_auth} entries, head/tail verified")
