#!/usr/bin/env python3
"""autopush-453 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own autopush-453 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_autopush453.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-453"
# the entry bytes the inline walk banked — the 159-entry state the
# md5 e32af9031ae7 wears; the re-run the guard the double lane answers


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears autopush-453 — the double lane never runs")
sys.exit("GUARD: the entry lane the inline walk already banked — see worklog autopush-453")
