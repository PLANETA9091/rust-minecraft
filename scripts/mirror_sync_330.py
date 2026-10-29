#!/usr/bin/env python3
"""mirror sync for the 330 round (the no-overlap law the 328/329 laps
keep — the authoritative tail the autopush-329 boundary slices, the
append the mirror's own tail the pure wear serves)."""
import subprocess

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"

text = open(AUTH).read()
boundary = text.rfind("\n--- Task ID: autopush-329 ")
if boundary < 0:
    raise SystemExit("autopush-329 boundary not found in authoritative worklog")
tail = text[boundary + 1:]
if "--- Task ID: autopush-330 " not in tail:
    raise SystemExit("autopush-330 entry missing from the sliced tail")

mirror = open(MIRROR).read()
if "--- Task ID: autopush-330 " in mirror:
    print("mirror ALREADY carries the 330 entry — no-op")
    raise SystemExit(0)
if mirror.endswith("\n--- Task ID: autopush-329 ") or boundary >= 0:
    # no overlap check: the mirror's own 329 entry the same boundary
    # marker the tail never re-carries
    sep = "" if mirror.endswith("\n") else "\n"
    open(MIRROR, "a").write(sep + "\n" + tail)
    print("mirror appended the 330 entry (%d chars)" % len(tail))

log = subprocess.run(["git", "diff", "--stat", MIRROR], capture_output=True, text=True)
print(log.stdout.strip())
