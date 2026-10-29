#!/usr/bin/env python3
"""mirror sync for the dev-195 round (the no-overlap law the earlier
laps keep — the authoritative tail the dev-195 boundary slices, the
append the mirror's own tail the pure wear serves)."""
import subprocess

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"

text = open(AUTH).read()
boundary = text.rfind("\n--- Task ID: dev-195 ")
if boundary < 0:
    raise SystemExit("dev-195 boundary not found in authoritative worklog")
tail = text[boundary + 1:]
if "--- Task ID: dev-195 " not in tail:
    raise SystemExit("dev-195 entry missing from the sliced tail")

mirror = open(MIRROR).read()
if "--- Task ID: dev-195 " in mirror:
    print("mirror ALREADY carries the dev-195 entry — no-op")
    raise SystemExit(0)
sep = "" if mirror.endswith("\n") else "\n"
open(MIRROR, "a").write(sep + "\n" + tail)
print("mirror appended the dev-195 entry (%d chars)" % len(tail))

log = subprocess.run(["git", "diff", "--stat", MIRROR], capture_output=True, text=True)
print(log.stdout.strip())
