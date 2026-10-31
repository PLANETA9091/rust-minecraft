#!/usr/bin/env python3
"""autopush-368 — THE MIRROR SYNC (the 366 record's own rfind-anchor
trap the walk retires): the authority's own tail entry the mirror
appends — the ENTRY-LIST verdict the both files wear (the counts the
IDs the order the one law answers), the already-present guard the
duplicate never mints.
"""
import re

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()

pat = re.compile(r"^--- Task ID: ([A-Za-z0-9-]+)", re.M)
auth_ids = pat.findall(auth)
mirror_ids = pat.findall(mirror)

# my entry's header = the authority's LAST entry
my_header = "--- Task ID: " + auth_ids[-1]
assert my_header.startswith("--- Task ID: autopush-368"), f"unexpected tail: {auth_ids[-1]}"

if my_header in mirror:
    raise SystemExit("MIRROR ALREADY HAS THE ENTRY — nothing to sync")

# the mirror must be exactly the authority minus my tail entry
if mirror_ids != auth_ids[:-1]:
    raise SystemExit(
        f"MIRROR DIVERGED: mirror {len(mirror_ids)} entries vs authority {len(auth_ids)-1} expected; "
        f"first diff at {next((i for i,(a,b) in enumerate(zip(auth_ids, mirror_ids+[None]*99)) if a != b), '?')}"
    )

# extract my entry text: from my header to EOF (strip the leading blank line)
s = auth.find("\n" + my_header)
entry = auth[s + 1:]  # keep from the header line on
if not entry.endswith("\n"):
    entry += "\n"

new_mirror = mirror
if not new_mirror.endswith("\n"):
    new_mirror += "\n"
new_mirror += "\n" + entry if not mirror.endswith("\n---") else entry

open(MIRROR, "w", encoding="utf-8").write(new_mirror)

# verify: the entry lists now agree
mirror2 = open(MIRROR, encoding="utf-8").read()
mirror_ids2 = pat.findall(mirror2)
assert mirror_ids2 == auth_ids, "post-sync entry lists diverge"
print(f"SYNCED: mirror now {len(mirror_ids2)} entries, tail = {mirror_ids2[-1]}")
