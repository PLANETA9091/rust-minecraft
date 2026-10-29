#!/usr/bin/env python3
"""mirror_sync_199.py — the dev-199 round's worklog append + the
mirror's exact-copy resync (the 339 law — the authoritative entry
rides the file tail, the logs/worklog.md the EXACT COPY the
invariant speaks)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = open(
    "/home/z/my-project/rust-minecraft/scripts/worklog_entry_199.txt", encoding="utf-8"
).read()

t = open(AUTH, encoding="utf-8").read()
if "Task ID: dev-199" in t:
    print("ALREADY-APPENDED")
    sys.exit(0)
if not t.endswith("\n"):
    t += "\n"
t += ENTRY
open(AUTH, "w", encoding="utf-8").write(t)

# THE MIRROR'S OWN RESYNC: the exact copy the invariant wears
open(MIRROR, "w", encoding="utf-8").write(t)
print("APPENDED+MIRROR-REPAIRED", len(t))
