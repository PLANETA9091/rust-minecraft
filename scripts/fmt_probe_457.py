#!/usr/bin/env python3
"""autopush-457 — THE FMT REGION WALK (the 456 record's own law the
round wears): the whole-file rustfmt the ORIGINAL text the formatted
stdout the diff answers — the MY-REGIONS verdict the marker ranges
keep (no hunk may touch the round's own edits).

Regions:
  screens.rs    — the powers' own data-words block the decode arm
                  wears + the live confirm's own wire block the round
                  banks (no rustfmt hunk may touch them).
  play.rs       — the set_beacon writer + the session method the
                  round banks.
  world_view.rs — the SetBeacon variant the round banks.
  window.rs     — the SetBeacon consumer arm the round banks.
  part_11.rs    — the round's own test (the powers' wire words wear,
                  EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the live confirm's own wire (the Set Beacon + the close)
    (
        "THE LIVE CONFIRM'S OWN WIRE (the BeaconConfirmButton.onPress",
        "let Some(pos) = self.beacon_open_cell else {",
    ),
    # the powers' own data words (the decode arm the window bank
    # re-lays) — the whole 457-edited decode tail
    (
        "THE POS-KEYED BANK the live lane never wears (the",
        "if c.menu_type_id == crate::world_model::MENU_TYPE_ENCHANTMENT {",
    ),
]

PLAY = [
    # the set_beacon writer (the Optional pair the bytes wear)
    (
        "The beacon's OWN effects word (the vanilla",
        "pub fn encode_set_beacon(primary: Option<i32>, secondary: Option<i32>) -> Vec<u8> {",
    ),
    (
        "pub fn encode_set_beacon(primary: Option<i32>, secondary: Option<i32>) -> Vec<u8> {",
        "/// The right-mouse USE (the vanilla `MultiPlayerGameMode.useItem`'s",
    ),
    # the session method (the send lane the confirm rides)
    (
        "The beacon's OWN effects word (the vanilla",
        "pub fn set_beacon(&mut self, primary: Option<i32>, secondary: Option<i32>) -> Result<()> {",
    ),
]

WORLD_VIEW = [
    # the SetBeacon variant (the command the screen emits)
    (
        "THE BEACON EFFECTS' OWN WIRE (the vanilla",
        "    /// The use RELEASE (the vanilla gameMode.releaseUsingItem's own",
    ),
]

WINDOW = [
    # the SetBeacon consumer arm (the session the worker speaks)
    (
        "crate::world_view::ContainerCommand::SetBeacon {",
        "crate::world_view::ContainerCommand::PickBlock { pos, include_data } => {",
    ),
]

PART11 = [
    # the round's own test (the powers' wire words wear — EOF the end
    # the round keeps)
    (
        "THE BEACON POWERS' OWN WIRE WORDS (the 456 NEXT's own lane the wheel",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-protocol/src/play.rs": PLAY,
    "crates/mc-client/src/world_view.rs": WORLD_VIEW,
    "crates/mc-client/src/window.rs": WINDOW,
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
    """the formatted text the stdout answers — the filename header this
    rustfmt wears the strip removes"""
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
            bad = [h for h in hunks if h[0] < e_line and h[1] > s_line]
            label = start_marker.strip()[:56].replace("\n", " | ")
            if bad:
                ok = False
                print(f"{path}: REGION NOT FMT-CLEAN [{label}] hunks={bad}")
            else:
                print(f"{path}: region fmt-clean [{label}]")
    if ok:
        print("ALL-REGIONS-CLEAN")
    else:
        print("REGIONS-DIRTY")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
