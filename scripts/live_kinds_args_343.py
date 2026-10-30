#!/usr/bin/env python3
"""autopush-343 — THE LIVE KINDS' OWN ARG WALK: the 4-arg
`live_inventory_updated` call sites the tests wear the FIFTH arg the
wire lane speaks — the NONE-kinds array the no-component law keeps
(the kindless stacks the honest silence), the same shape the
SnapshotView field wears. Balanced-paren walk to the call's close (the
string literals the walk skips — no call carries one); the insert seat
rides the LAST arg's own tail: the single-line call the `, ARG` inline
keeps, the multi-line the last arg's indent the ARG line re-wears and
the close paren's own whitespace the tail restores. `app.`/`off.` call
prefixes only — the definition and the window.rs 5-arg site untouched.
"""
import re
import sys

FILES = [
    "crates/mc-client/src/screens/tests/part_7.rs",
    "crates/mc-client/src/screens/tests/part_9.rs",
    "crates/mc-client/src/screens/tests/part_10.rs",
]
ARG = "[crate::brewing_sim::POTION_KIND_NONE; crate::world_view::INVENTORY_MENU_SLOTS]"
PAT = re.compile(r"\b(?:app|off)\.live_inventory_updated\(")

total = 0
for path in FILES:
    with open(path) as f:
        text = f.read()
    opens = [m.end() - 1 for m in PAT.finditer(text)]
    for op in reversed(opens):
        depth = 0
        i = op
        while True:
            c = text[i]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    break
            elif c == '"':
                i += 1
                while text[i] != '"':
                    if text[i] == "\\":
                        i += 1
                    i += 1
            i += 1
        close = i
        seg = text[:close]
        head = seg[op + 1 :].rstrip()
        single = "\n" not in head
        if single:
            text = seg + ", " + ARG + text[close:]
        else:
            # the last arg's final char (strip the close paren's own
            # leading whitespace — the trailing tail restores it)
            j = len(seg) - 1
            while seg[j] in " \t\n":
                j -= 1
            trailing = seg[j + 1 :]
            last_line_start = seg.rfind("\n", 0, j) + 1
            arg_indent = seg[last_line_start:j]
            # the trailing comma the multi-line calls wear: the
            # comma-terminated tail the ARG seat needs NO extra comma
            # (the `,,` the honest walk refuses), the comma-less tail
            # the one comma the insert speaks
            comma = "" if seg[j] == "," else ","
            text = (
                seg[: j + 1]
                + comma
                + "\n"
                + arg_indent
                + ARG
                + trailing
                + text[close:]
            )
        total += 1
    with open(path, "w") as f:
        f.write(text)
    print(f"{path}: {len(opens)} call sites")
print(f"total: {total}")
sys.exit(0 if total else 1)
