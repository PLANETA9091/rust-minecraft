#!/usr/bin/env python3
"""autopush-443 — THE WORKLOG MIRROR SYNC (the one-shot walk): the
authoritative seat the mirror the same bytes wear, the counts the
assert banks, the tail marker the last entry answers. One-shot: the
re-run the guard refuses (the round the entry the mirror already
wears). Usage: python3 scripts/worklog_443.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-443"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

n_auth = auth.count("\n--- Task ID: autopush") + (
    1 if auth.startswith("--- Task ID: autopush") else 0)
n_mirror = mirror.count("\n--- Task ID: autopush") + (
    1 if mirror.startswith("--- Task ID: autopush") else 0)

if MARK in mirror:
    print(f"worklog_443: the mirror the 443 entry already wears — "
          f"the one-shot the re-run refuses")
    sys.exit(1)
if n_auth != n_mirror + 1:
    print(f"worklog_443: count mismatch auth={n_auth} mirror={n_mirror}")
    sys.exit(1)
if not auth.rstrip().endswith(
    "the seat the round's own law may re-wear)."
):
    print("worklog_443: the auth tail the 443 NEXT tail does not wear")
    sys.exit(1)

open(MIRROR, "w", encoding="utf-8").write(auth)
mirror2 = open(MIRROR, encoding="utf-8").read()
assert mirror2 == auth, "the mirror the write the same bytes keeps"
print(f"worklog_443: both files {n_auth} entries, equal, "
      f"tail autopush-443")
