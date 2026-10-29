#!/usr/bin/env python3
"""mirror sync for the 328 round: the mirror's own tail the 327 entry
keeps — the authoritative tail's 328 entry the plain append serves (the
index-cut law the no-overlap state retires)."""
import subprocess

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"

auth = open(AUTH).read()
marker = "\n--- Task ID: autopush-328 "
idx = auth.find(marker)
assert idx > 0, "the 328 marker the authoritative tail keeps"
tail = auth[idx + 1:]  # the 328 entry the cut serves (marker line inclusive)
if not tail.endswith("\n"):
    tail += "\n"

mirror = open(MIRROR).read()
assert "\n--- Task ID: autopush-328 " not in mirror, "the 328 the mirror never pre-wears"
assert mirror.rstrip().endswith("the smaller round keeps)."), "the 327 tail the mirror keeps"
open(MIRROR, "a").write(tail)
print("mirror appended: the 328 entry the tail serves")
r = subprocess.run(["git", "diff", "--stat", "--", MIRROR], capture_output=True, text=True)
print(r.stdout.strip())
