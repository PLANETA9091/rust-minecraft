#!/usr/bin/env python3
"""THE FIXTURES' OWN FIELD PATCH (the E0063 walk the new World field
serves) — every full `World { ... }` literal outside world.rs carries
the explicit field list; the new `has_skylight` member the overworld
default answers (true). The patch appends the row after each
`portal_axis: std::collections::HashMap::new(),` line — the LAST
member the literal's own order keeps — with the line's own indent.
Deterministic: re-runs are no-ops (the row's presence the skip wears)."""

import re
import sys

SITES = [
    "crates/mc-client/src/world_view.rs",
    "crates/mc-client/src/mesher.rs",
    "crates/mc-client/src/screens.rs",
]

ROW = re.compile(r"^(\s*)portal_axis: std::collections::HashMap::new\(\),(.*)$")


def main() -> int:
    changed = 0
    for path in SITES:
        with open(path, "r", encoding="utf-8") as f:
            lines = f.readlines()
        out = []
        n = 0
        for line in lines:
            m = ROW.match(line)
            out.append(line)
            if m:
                indent, tail = m.group(1), m.group(2)
                out.append(f"{indent}has_skylight: true,{tail}\n")
                n += 1
        if n:
            with open(path, "w", encoding="utf-8") as f:
                f.writelines(out)
            print(f"{path}: {n} site(s) patched")
            changed += n
    print(f"total: {changed}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
