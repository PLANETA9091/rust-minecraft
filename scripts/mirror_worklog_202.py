#!/usr/bin/env python3
"""THE WORKLOG'S OWN APPEND (autopush-202): the entry the BOTH worklogs the
tail appends — the main the dev-164 slice's AFTER, the repo's own
logs/worklog.md the same entry the mirror wears. Append-only.

Usage: python3 scripts/mirror_worklog_202.py
"""
MAIN = "/home/z/my-project/worklog.md"
REPO = "/home/z/my-project/rust-minecraft/logs/worklog.md"
ENTRY = open("/home/z/my-project/scripts/worklog_entry_202.md").read()

for path in (MAIN, REPO):
    src = open(path).read()
    if "Task ID: autopush-202" in src:
        print(f"{path}: autopush-202 already present, skip")
        continue
    if not src.endswith("\n"):
        src += "\n"
    src += "---\n" + ENTRY
    open(path, "w").write(src)
    print(f"{path}: appended, new tail:")
    print(src[-200:])
