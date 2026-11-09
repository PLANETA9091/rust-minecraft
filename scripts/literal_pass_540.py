#!/usr/bin/env python3
"""autopush-540 — the MerchantOfferView literals the result_kind word wears.

THE WALK: every `MerchantOfferView {` literal in the tests the field
gains (`result_kind: crate::brewing_sim::POTION_KIND_NONE,` — the
kindless default the honest wall keeps), the insertion the literal's
own `result:` line follows. The one-line literals the pass refuses
(the anchor the same line never serves — the loud fail the honest
check keeps).
"""
import re
import sys

PATH = "crates/mc-client/src/screens/tests/part_11.rs"

src = open(PATH).read()
lines = src.splitlines(keepends=True)
out = []
i = 0
n = 0
while i < len(lines):
    line = lines[i]
    out.append(line)
    if "MerchantOfferView {" in line and "}" in line:
        # the one-line literal the anchor never serves
        sys.exit(f"ONE-LINE LITERAL at line {i+1}: {line.strip()}")
    if "MerchantOfferView {" in line:
        depth = line.count("{") - line.count("}")
        j = i + 1
        inserted = False
        while j < len(lines) and depth > 0:
            l = lines[j]
            if not inserted and re.match(r"\s*result:\s", l):
                out.append(l)
                indent = re.match(r"\s*", l).group(0)
                out.append(
                    indent + "result_kind: crate::brewing_sim::POTION_KIND_NONE,\n"
                )
                inserted = True
                n += 1
                depth += l.count("{") - l.count("}")
                j += 1
                continue
            depth += l.count("{") - l.count("}")
            out.append(l)
            j += 1
        if not inserted:
            sys.exit(f"LITERAL WITHOUT result: opened at line {i+1}")
        i = j
        continue
    i += 1

open(PATH, "w").write("".join(out))
print(f"inserted result_kind into {n} literals")
