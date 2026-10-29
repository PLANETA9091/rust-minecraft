#!/usr/bin/env python3
"""mirror sync for the 332 round (the no-overlap law the 328..331 laps
keep — the authoritative tail the autopush-331 boundary slices, the
append the mirror's own tail the pure wear serves)."""
import subprocess

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"

text = open(AUTH).read()
boundary = text.rfind("\n--- Task ID: autopush-331 ")
if boundary < 0:
    raise SystemExit("autopush-331 boundary not found in authoritative worklog")
tail = text[boundary + 1:]
if "--- Task ID: autopush-332 " not in tail:
    raise SystemExit("autopush-332 entry missing from the sliced tail")

mirror = open(MIRROR).read()
if "--- Task ID: autopush-332 " in mirror:
    print("mirror ALREADY carries the 332 entry — no-op")
    raise SystemExit(0)
sep = "" if mirror.endswith("\n") else "\n"
open(MIRROR, "a").write(sep + "\n" + tail)
print("mirror appended the 332 entry (%d chars)" % len(tail))

log = subprocess.run(["git", "diff", "--stat", MIRROR], capture_output=True, text=True)
print(log.stdout.strip())
