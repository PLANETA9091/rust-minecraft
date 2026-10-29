#!/usr/bin/env python3
"""mirror_sync_339.py — the autopush-339 round's worklog append + the
mirror's own repair: the authoritative entry rides the file tail, the
logs/worklog.md the EXACT COPY the invariant speaks (the prior rounds'
duplicate autopush-337 the byte-identical twin the resync removes)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = open(
    "/home/z/my-project/rust-minecraft/scripts/worklog_entry_339.txt", encoding="utf-8"
).read()

t = open(AUTH, encoding="utf-8").read()
if "Task ID: autopush-339" in t:
    print("ALREADY-APPENDED")
    sys.exit(0)
if not t.endswith("\n"):
    t += "\n"
t += ENTRY
open(AUTH, "w", encoding="utf-8").write(t)

# THE MIRROR'S OWN REPAIR: the exact copy the invariant wears (the
# duplicate autopush-337 the repair removes, the git history the
# prior state preserves)
open(MIRROR, "w", encoding="utf-8").write(t)
print("APPENDED+MIRROR-REPAIRED", len(t))
