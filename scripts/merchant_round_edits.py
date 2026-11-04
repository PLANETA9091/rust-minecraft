#!/usr/bin/env python3
"""dev-222 — THE MERCHANT'S OWN FIELD RIDE (the LiveContainer literal
walker): every test-side `LiveContainer {` literal gains the round's
own `merchant: None,` field — the projection's SOME/None lane the
struct's new field wears, the literals the compile the honest None
keeps. The world_view builder's own site (oc.title_nbt.clone()) the
walker NEVER touches (the merchant ride the round already banks).
Usage: python3 scripts/merchant_round_edits.py"""
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
FILES = [
    "crates/mc-client/src/screens/tests/part_4.rs",
    "crates/mc-client/src/screens/tests/part_7.rs",
    "crates/mc-client/src/screens/tests/part_9.rs",
    "crates/mc-client/src/screens/tests/part_10.rs",
    "crates/mc-client/src/screens/tests/part_11.rs",
]

total = 0
for rel in FILES:
    p = ROOT / rel
    lines = p.read_text().split("\n")
    out = []
    in_literal = False
    depth = 0
    lit_indent = ""
    patched = 0
    for line in lines:
        if not in_literal and "LiveContainer {" in line:
            in_literal = True
            depth = line.count("{") - line.count("}")
            lit_indent = line[: len(line) - len(line.lstrip())]
            out.append(line)
            continue
        if in_literal:
            depth += line.count("{") - line.count("}")
            if depth <= 0:
                # the literal's closing line — insert the field BEFORE it
                # (the house field order the title lane's own tail keeps)
                out.append(f"{lit_indent}    merchant: None,")
                patched += 1
                in_literal = False
                out.append(line)
                continue
            out.append(line)
            continue
        out.append(line)
    if patched:
        p.write_text("\n".join(out))
    total += patched
    print(f"{rel}: {patched} literals patched")
print(f"TOTAL: {total}")
