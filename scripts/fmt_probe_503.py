#!/usr/bin/env python3
"""autopush-503 — THE FMT REGION WALK (the 485..502 record's own law
the round wears): the whole-file rustfmt the ORIGINAL text the
formatted stdout the diff answers — the MY-REGIONS verdict the marker
ranges keep (no hunk may touch the round's own edits).

Regions:
  screens.rs — the ender window's own kind flag doc + field, the
  ctor's own init line, the scene reset's own block, the fresh
  session's own reset block, the is_ender's own arm branch, the
  shulker door's own reset block, the hopper door's own reset block,
  the wire decode's own reset block, the chest seat's own clone doc +
  the whole fn (the ender lane the round wakes).
  part_11.rs — the round's own test (the tail wears, EOF the end the
  round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the ender window's own kind flag doc + field
    (
        "    /// THE ENDER WINDOW'S OWN KIND FLAG (the offline clone lane's key",
        "    /// The last biome the network world's music situation resolved to",
    ),
    # the ctor's own init line
    (
        "            chest_is_live: false,",
        "            live_clicks: None,",
    ),
    # the scene reset's own block
    (
        "        // the shulker's kind row the chest's own scene never wears (the",
        "        // the deterministic scene the fresh grid keeps — the flush key",
    ),
    # the fresh session's own reset block
    (
        "        // the shulker's kind row the chest's own door never wears (the",
        "        // THE DOUBLE'S OWN COMBINE (the dev-71 round — javap 26.2 the",
    ),
    # the is_ender's own arm branch
    (
        "        // OWN SHARED LAW (javap 26.2 EnderChestBlock.useWithoutItem",
        "        self.container_open_cell = Some(pos);",
    ),
    # the shulker door's own reset block
    (
        "        // reads — the container.shulkerBox default the vanilla menu",
        "        // THE SHULKER'S OWN OPEN BYTE (the dev-193 round — the vanilla",
    ),
    # the hopper door's own reset block
    (
        "        // THE HOPPER'S OWN KIND ROW (the title + canvas lanes' flag the",
        "        // THE FAMILY'S OWN VOICE (the counter law the chest walk keeps",
    ),
    # the wire decode's own reset block
    (
        "                // HOPPER'S OWN CANVAS the flag wears (the 176x133",
        "                // THE WINDOW'S OWN NAME (the same decode ride — the",
    ),
    # the chest seat's own clone doc + the whole fn
    (
        "    /// THE CHEST SEAT'S OWN CLONE (the chest click walk's own live-only",
        "    /// THE FURNACE SEAT'S OWN CLONE (the furnace click walk's own",
    ),
]

PART11 = [
    # the round's own test (the tail wears)
    (
        "/// THE CLONE'S OWN ENDER RIDE (the 502 NEXT's own third lane the",
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


def main() -> int:
    ok = True
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        proc = subprocess.run(
            [RUSTFMT, "--emit", "stdout", path],
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            print(f"{path}: RUSTFMT FAILED\n{proc.stderr[:400]}")
            ok = False
            continue
        formatted = proc.stdout
        for (start_m, end_m) in regions:
            s = find(original, start_m)
            e = region_end(original, end_m, s + 1)
            orig_region = original[s:e]
            try:
                fs = find(formatted, start_m)
                fe = region_end(formatted, end_m, fs + 1)
            except SystemExit:
                print(f"{path}: REGION LOST in formatted output: {start_m[:60]!r}")
                ok = False
                continue
            fmt_region = formatted[fs:fe]
            if orig_region == fmt_region:
                print(f"{path}: region CLEAN [{start_m[:52]!r}...]")
            else:
                ok = False
                print(f"{path}: region DIRTY [{start_m[:52]!r}...] — the diff:")
                for line in list(
                    difflib.unified_diff(
                        orig_region.splitlines(),
                        fmt_region.splitlines(),
                        lineterm="",
                        n=1,
                    )
                )[:40]:
                    print("   " + line)
    print("ALL-REGIONS-CLEAN" if ok else "REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
