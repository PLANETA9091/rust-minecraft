#!/usr/bin/env python3
"""THE OPT-SEAM PATCH'S OWN WALK (the autopush-218 round): the
operatorItemsTab:false row the save walk's own seat inserts — the
processOptions visit order the full-equality literals wear (the
autoJump@5-13 row's AFTER the rotateWithMinecart@18 row the client's
store skips, the operatorItemsTab@31-39 the autoSuggestions row's
BEFORE keeps), the `autoJump:false\\ntoggleCrouch` seam the ten
full-equality sites answer, the contains-assert the autoJump:true
round-trip wears the same row's own word keeps.

The patch the ONE pattern replaces:
  autoJump:false\\ntoggleCrouch  ->  autoJump:false\\noperatorItemsTab:false\\ntoggleCrouch
and the ONE contains-assert:
  pauseOnLostFocus:true\\nautoJump:true\\ntoggleCrouch:  ->
  pauseOnLostFocus:true\\nautoJump:true\\noperatorItemsTab:false\\ntoggleCrouch:
(the Rust string-escape forms the literals carry — the backslash-n
the source bytes speak).
"""
import re

FILES = [
    "crates/mc-client/src/options_file.rs",
    "crates/mc-client/src/screens/tests/part_4.rs",
]

SEAM = r"autoJump:false\ntoggleCrouch"
REPL = r"autoJump:false\noperatorItemsTab:false\ntoggleCrouch"
CONTAINS_OLD = r"pauseOnLostFocus:true\nautoJump:true\ntoggleCrouch:"
CONTAINS_NEW = r"pauseOnLostFocus:true\nautoJump:true\noperatorItemsTab:false\ntoggleCrouch:"

for path in FILES:
    text = open(path).read()
    n1 = text.count(SEAM)
    n2 = text.count(CONTAINS_OLD)
    text = text.replace(SEAM, REPL)
    text = text.replace(CONTAINS_OLD, CONTAINS_NEW)
    open(path, "w").write(text)
    print(f"{path}: seam {n1} site(s), contains {n2} site(s)")
