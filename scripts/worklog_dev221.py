#!/usr/bin/env python3
"""dev-221 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own dev-221 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_dev221.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: dev-221"
# the entry the inline walk banked — the idempotent re-run the guard
# answers (the entry bytes the md5 d03e9bfb9946 the 158-entry state wears)
ENTRY = None


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears dev-221 — the double lane never runs")
sys.exit("GUARD: the entry lane the inline walk already banked — see worklog dev-221")
