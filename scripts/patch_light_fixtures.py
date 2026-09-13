#!/usr/bin/env python3
"""Bulk-patch the DimensionTypeEntry wire fixtures for the four new
syncable visual attributes (block_light_tint / sky_light_color /
sky_light_factor / night_vision_color) — the line-pattern discipline:
walk EVERY file fully, count the hits, and only write to disk when the
counts match the expectation exactly.

Pattern A (multi-line initializers): a line `cloud_fog_end_distance:
<value>,` gets four same-indent None lines inserted after it.
Pattern B (single-line initializers): `... cloud_fog_end_distance: None }`
gets the four fields inline before the closing brace.

The four new fields are ALWAYS None in the existing fixtures — no
vanilla dimension states them; the custom-element wire test states them
deliberately by hand (not here).
"""
import re
import sys

BASE = "/home/z/my-project/rust-minecraft"

# (path, expected_pattern_A_hits, expected_pattern_B_hits)
TARGETS = [
    ("crates/mc-client/src/world_model.rs", 13, 2),
    ("crates/mc-client/src/world_view.rs", 3, 0),
]

INSERT_FIELDS = [
    "block_light_tint: None,",
    "sky_light_color: None,",
    "sky_light_factor: None,",
    "night_vision_color: None,",
]

RE_A = re.compile(r"^(\s*)cloud_fog_end_distance: (?:None|Some\(\d+(?:\.\d+)?\)),$")
RE_B = re.compile(r"^(.*cloud_fog_end_distance: None) \},$")


def patch(text: str, exp_a: int, exp_b: int, path: str) -> str:
    lines = text.split("\n")
    out = []
    hit_a = hit_b = 0
    for line in lines:
        if RE_B.match(line):
            out.append(RE_B.sub(r"\1, block_light_tint: None, sky_light_color: None, sky_light_factor: None, night_vision_color: None },", line))
            hit_b += 1
            continue
        out.append(line)
        if RE_A.match(line):
            indent = RE_A.match(line).group(1)
            for f in INSERT_FIELDS:
                out.append(f"{indent}{f}")
            hit_a += 1
    if (hit_a, hit_b) != (exp_a, exp_b):
        print(f"FAIL {path}: pattern A {hit_a}/{exp_a}, pattern B {hit_b}/{exp_b} — nothing written")
        sys.exit(1)
    print(f"OK   {path}: A={hit_a} B={hit_b}")
    return "\n".join(out)


def main() -> None:
    for path, exp_a, exp_b in TARGETS:
        full = f"{BASE}/{path}"
        with open(full, "r", encoding="utf-8") as fh:
            text = fh.read()
        patched = patch(text, exp_a, exp_b, path)
        with open(full, "w", encoding="utf-8") as fh:
            fh.write(patched)


if __name__ == "__main__":
    main()
