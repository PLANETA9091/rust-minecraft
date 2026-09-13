#!/usr/bin/env python3
"""Add the `cloud_fog_end_distance: None,` field to every DimensionTypeEntry
fixture initializer that lacks it.

Line-pattern bulk patch, the ambient/skybox/fog rounds' discipline: walk the
WHOLE file, collect every planned insertion, and write only after every
pattern landed — a partial walk never reaches the disk.

The pattern: any `    fog_end_distance: ...,` field line inside a fixture
initializer whose NEXT line is not already the new field (the rounds that
came before hand-added a few). The accessor use sites (`d.fog_end_distance`,
no colon) and the field-assignment statements (`.fog_end_distance = ...`)
do not match the regex by construction.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
FILES = [
    "crates/mc-client/src/world_model.rs",
    "crates/mc-client/src/world_view.rs",
]
FIELD = re.compile(r"^(\s*)fog_end_distance: .*,$")
NEW_FIELD = "cloud_fog_end_distance: None,"

def main() -> int:
    total = 0
    for rel in FILES:
        path = ROOT / rel
        lines = path.read_text().splitlines(keepends=True)
        plan = []
        for i, line in enumerate(lines):
            m = FIELD.match(line.rstrip("\n"))
            if m is None:
                continue
            nxt = lines[i + 1].strip() if i + 1 < len(lines) else ""
            if nxt.startswith("cloud_fog_end_distance"):
                continue  # the hand-added site — already done
            plan.append((i + 1, m.group(1)))
        if not plan:
            print(f"{rel}: no insertion sites — nothing to do")
            continue
        print(f"{rel}: {len(plan)} insertion site(s)")
        for idx, indent in reversed(plan):
            lines.insert(idx, f"{indent}{NEW_FIELD}\n")
        path.write_text("".join(lines))
        total += len(plan)
    print(f"total: {total} field(s) inserted")
    return 0

if __name__ == "__main__":
    sys.exit(main())
