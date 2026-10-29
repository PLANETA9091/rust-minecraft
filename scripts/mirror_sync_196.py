#!/usr/bin/env python3
"""dev-196 — the worklog mirror's own sync (the logs/worklog.md the
authoritative file's tail the repo copy answers): the rfind boundary
slice the autopush-335 round's own law keeps — the mirror's LAST entry
boundary the append point answers, the no-op the already-synced copy
never double-rides."""
import os

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
src = "/home/z/my-project/worklog.md"
dst = os.path.join(root, "logs", "worklog.md")

authoritative = open(src).read()
mirror = open(dst).read()

# the mirror's own last-entry boundary
marker = "--- Task ID: dev-196 "
idx = mirror.rfind(marker)
if idx != -1:
    print("mirror already carries dev-196 — no-op")
else:
    # the append: the authoritative file's tail FROM the mirror's own
    # last boundary forward — the entries the mirror lacks
    own = "--- Task ID: autopush-337 "
    base = mirror.rfind(own)
    assert base != -1, "the mirror's own tail the boundary answers"
    addendum = authoritative[base:]
    with open(dst, "a") as f:
        f.write(addendum)
    print("appended", len(addendum), "chars to the mirror")
