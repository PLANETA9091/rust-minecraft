#!/usr/bin/env python3
"""autopush-472 — THE FMT REGION WALK (the 471 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits). The 474 suffix
the 469's 471-splice + the 470's 472 + the 471's 473 the wheel wears.

Regions:
  screens.rs  — the StatsCounter's own field (the talked counter the
                struct wears), the General tab's own row (the clinit
                order the talked the traded precedes), the wire merge's
                own docstrings + arm, the file save's own array + the
                load's own match (the nine words the lanes serve).
  part_11.rs  — the round's own test (the talked stat's own wire lane
                the EOF side keeps — the file's own tail the new pin
                wears).
Usage: python3 scripts/fmt_probe_474.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the StatsCounter's own field (the talked counter the struct wears)
    (
        "    pub traded_with_villager: i64,",
        "    pub talked_to_villager: i64,",
    ),
    # the General tab's own row (the clinit order the talked precedes)
    (
        "        if self.stats.talked_to_villager > 0 {",
        "        if self.stats.traded_with_villager > 0 {",
    ),
    # the wire merge's own docstring (the nine the tab serves)
    (
        "    /// category the nine own lanes set",
        "    pub fn merge_award_stats(&mut self, entries: &[",
    ),
    (
        '                    "talked_to_villager" => self.stats.'
        'talked_to_villager = e.count as i64,',
        "                // THE COMBAT PAIRS' OWN RESOLVE",
    ),
    # the file save's own array (the nine the counter's fields keep)
    (
        "        // THE CUSTOM CATEGORY (the nine the counter's own fields",
        "        for (word, count) in customs {",
    ),
    # the file load's own docstring + match (the nine words the lanes speak)
    (
        "    /// the wire merge wears, the custom the nine words the fields",
        "    pub fn stats_load_json(&mut self, v: &serde_json::Value) {",
    ),
    (
        '                match k.strip_prefix("minecraft:").unwrap_or(k) {',
        "            }\n        }\n    }",
    ),
]

PART11 = [
    # the round's own pin (the EOF side keeps — the file's own tail
    # the new test wears)
    (
        "fn the_talked_stat_the_wire_lane_wears() {",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-client/src/screens/tests/part_11.rs": PART11,
}


def find(text: str, marker: str, start: int = 0) -> int:
    i = text.find(marker, start)
    if i < 0:
        raise SystemExit(f"MARKER NOT FOUND: {marker!r}")
    return i


def region_end(text: str, marker: str, start_at: int) -> int:
    if marker is None:
        return len(text)
    return find(text, marker, start_at)


def formatted_of(path: str) -> str:
    proc = subprocess.run(
        [RUSTFMT, "--emit", "stdout", path],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise SystemExit(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
    out = proc.stdout
    lines = out.split("\n")
    if len(lines) >= 2 and lines[0].endswith(":") and lines[1] == "":
        out = "\n".join(lines[2:])
    return out


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(
            None, original.splitlines(), formatted.splitlines())
        hunks = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            hunks.append((i1, i2, tag, j1, j2))
        cursor = 0  # the sequential threading — the shared texts the order keys
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            inside = [
                (i1, i2, tag, j1, j2)
                for (i1, i2, tag, j1, j2) in hunks
                if i1 < e_line and i2 > s_line
            ]
            if inside:
                ok = False
                print(f"[MY-REGIONS DIRTY] {path} :: {start_marker[:60]!r}")
                for h in inside:
                    print(f"   hunk {h}")
            else:
                print(f"[MY-REGIONS CLEAN] {path} :: {start_marker[:60]!r}")
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
