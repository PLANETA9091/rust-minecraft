#!/usr/bin/env python3
"""autopush-493 — THE FMT SPLICE (the 485/487/488/489/491/229/492
record's own law the round wears): the DIRTY regions the probe spoke
the rustfmt bytes the region wears — the prefix the original text
keeps byte-exact, the region's own range the formatted text answers,
the tail the original resumes. The pre-existing drift the splice
never touches (the outside regions the file's own history banks)."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# the DIRTY regions the probe's own verdict spoke — (path, start, end)
# the end marker None the EOF keeps
DIRTY = [
    (
        "crates/mc-client/src/screens.rs",
        "    /// The table cell's click (the doClick PICKUP/QUICK_MOVE walk's",
        "        let believed = self.table_grid[cell];",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "            // THE GRID'S OWN FILL ARM (the 510 census — CraftingMenu.",
        "            let changed = crate::world_view::predict_quick_move_inventory(",
    ),
    (
        "crates/mc-client/src/screens.rs",
        "    /// The craft cell's click (the doClick PICKUP/QUICK_MOVE walk's",
        "        let believed = self.craft_grid[cell];",
    ),
    (
        "crates/mc-client/src/screens/tests/part_11.rs",
        "/// THE GRID'S OWN SHIFT RIDE (the autopush-493 round",
        None,
    ),
]


def main() -> int:
    # the per-path formatted renders (the whole-file rustfmt the splices
    # share — the regions the marker pairs the formatted text answers)
    formatted_cache = {}
    for (path, start_marker, end_marker) in DIRTY:
        if path not in formatted_cache:
            proc = subprocess.run(
                [RUSTFMT, "--emit", "stdout", path],
                capture_output=True, text=True)
            if proc.returncode != 0:
                print(f"{path}: RUSTFMT FAILED\n{proc.stderr}")
                return 1
            formatted_cache[path] = proc.stdout
        original = open(path, encoding="utf-8").read()
        formatted = formatted_cache[path]
        s_orig = original.find(start_marker)
        if s_orig < 0:
            print(f"{path}: MARKER NOT FOUND: {start_marker!r}")
            return 1
        s_fmt = formatted.find(start_marker)
        if s_fmt < 0:
            print(f"{path}: FORMATTED MARKER NOT FOUND")
            return 1
        if end_marker is None:
            # the tail region — the formatted text the EOF the splice wears
            spliced = original[:s_orig] + formatted[s_fmt:]
        else:
            e_fmt = formatted.find(end_marker, s_fmt)
            if e_fmt < 0:
                print(f"{path}: FORMATTED END NOT FOUND: {end_marker!r}")
                return 1
            spliced = original[:s_orig] + formatted[s_fmt:e_fmt] + original[s_orig:]
            # the ORIGINAL tail the region's own end marker re-anchors —
            # the suffix the first occurrence AFTER the start keeps
            tail_at = original.find(end_marker, s_orig)
            spliced = (original[:s_orig] + formatted[s_fmt:e_fmt]
                       + original[tail_at:])
        if spliced != original:
            open(path, "w", encoding="utf-8").write(spliced)
            print(f"{path}: spliced [{start_marker.strip()[:48]}] "
                  f"({len(original)} -> {len(spliced)} bytes)")
        else:
            print(f"{path}: already clean [{start_marker.strip()[:48]}]")
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
