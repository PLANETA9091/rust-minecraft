#!/usr/bin/env python3
"""autopush-457 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own autopush-457 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_autopush457.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-457"


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK not in auth:
    sys.exit("GUARD: AUTH never wore the autopush-457 entry — the inline lane must bank it first")
if open(MIRROR, encoding="utf-8").read() != auth:
    sys.exit("GUARD: the MIRROR the AUTH's bytes never wears — the copy lane must run first")
if md5(AUTH) != md5(MIRROR):
    sys.exit("GUARD: the md5 the pair never wears")
print("autopush-457 mirror sync: the AUTH the entry wears, the MIRROR the same bytes the copy answers")
print(f"AUTH  md5 {md5(AUTH)}")
print(f"MIRROR md5 {md5(MIRROR)}")
