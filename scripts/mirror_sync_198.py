#!/usr/bin/env python3
"""mirror sync for the 198 round (the no-overlap law the 333..197 laps
keep — the authoritative tail the dev-198 boundary slices, the
append the mirror's own tail the pure wear serves)."""
import subprocess

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"

text = open(AUTH).read()
boundary = text.rfind("\n--- Task ID: dev-198 ")
if boundary < 0:
    raise SystemExit("dev-198 boundary not found in authoritative worklog")
tail = text[boundary + 1:]
if "--- Task ID: dev-198 " not in tail:
    raise SystemExit("dev-198 entry missing from the sliced tail")

mirror = open(MIRROR).read()
if "--- Task ID: dev-198 " in mirror:
    print("mirror ALREADY carries the 335 entry — no-op")
    raise SystemExit(0)
sep = "" if mirror.endswith("\n") else "\n"
open(MIRROR, "a").write(sep + "\n" + tail)
print("mirror appended the 335 entry (%d chars)" % len(tail))

log = subprocess.run(["git", "diff", "--stat", MIRROR], capture_output=True, text=True)
print(log.stdout.strip())
