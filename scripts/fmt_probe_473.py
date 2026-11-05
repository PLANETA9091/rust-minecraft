#!/usr/bin/env python3
"""autopush-471 — THE FMT REGION WALK (the 470 record's own law the
round re-wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits). The 473 suffix
the 469 record's own 471-splice lane the wheel wears.

Regions:
  screens.rs  — the StatsCounter's own field (the traded counter the
                struct wears), the onTake docstring's own re-answer +
                the stat ride block (the success arm the award speaks),
                the General tab's own row, the wire merge's own arm +
                docstrings, the file save's own array + the load's own
                match (the eight words the lanes serve).
  part_11.rs  — the round's own test (the traded stat's own ride the
                EOF side keeps — the file's own tail the new pin
                wears).
Usage: python3 scripts/fmt_probe_473.py"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the StatsCounter's own field (the traded counter the struct wears)
    (
        "    pub deaths: i64,",
        "    pub traded_with_villager: i64,",
    ),
    # the onTake docstring's own re-answer (the success arm the award
    # speaks — the later-round clause the census truth replaces)
    (
        "THE XP'S OWN REWARD the offer's existence",
        "    fn merchant_result_take(&mut self) {",
    ),
    # the stat ride block (the award the setItem walk-back wears before)
    (
        "            // THE STAT'S OWN RIDE (pc 69-73",
        "        // THE XP'S OWN REWARD (pc 96-114",
    ),
    # the General tab's own row (the traded word the nonzero count wears)
    (
        "        if self.stats.traded_with_villager > 0 {",
        "        rows\n    }",
    ),
    # the wire merge's own docstring + arm (the eight the tab serves)
    (
        "    /// category the eight own lanes set",
        "    pub fn merge_award_stats(&mut self, entries: &[",
    ),
    (
        '                    "traded_with_villager" => self.stats.'
        'traded_with_villager = e.count as i64,',
        "                // THE COMBAT PAIRS' OWN RESOLVE",
    ),
    # the file save's own array (the eight the counter's fields keep)
    (
        "        // THE CUSTOM CATEGORY (the eight the counter's own fields",
        "        for (word, count) in customs {",
    ),
    # the file load's own docstring + match (the eight words the lanes speak)
    (
        "    /// the wire merge wears, the custom the eight words the fields",
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
        "fn the_traded_stat_the_on_take_ride_wears() {",
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
