#!/usr/bin/env python3
"""autopush-442 — THE WORKLOG MIRROR SYNC (the one-shot walk): the
authoritative seat the mirror the same bytes wear, the counts the
assert banks, the tail marker the last entry answers. One-shot: the
re-run the guard refuses (the round the entry the mirror already
wears). Usage: python3 scripts/worklog_442.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-442"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

n_auth = auth.count("\n--- Task ID: autopush") + (
    1 if auth.startswith("--- Task ID: autopush") else 0)
n_mirror = mirror.count("\n--- Task ID: autopush") + (
    1 if mirror.startswith("--- Task ID: autopush") else 0)

if MARK in mirror:
    print(f"worklog_442: the mirror the 442 entry already wears — "
          f"the one-shot the re-run refuses")
    sys.exit(1)
if n_auth != n_mirror + 1:
    print(f"worklog_442: count mismatch auth={n_auth} mirror={n_mirror}")
    sys.exit(1)
if not auth.rstrip().endswith(
    "the headless build risk the lane first answers)."
):
    print("worklog_442: the auth tail the 442 NEXT tail does not wear")
    sys.exit(1)

open(MIRROR, "w", encoding="utf-8").write(auth)
mirror2 = open(MIRROR, encoding="utf-8").read()
assert mirror2 == auth, "the mirror the write the same bytes keeps"
print(f"worklog_442: both files {n_auth} entries, equal, "
      f"tail autopush-442")
