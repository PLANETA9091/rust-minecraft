#!/usr/bin/env python3
"""fmt_fix_461 — the surgical region formatter (the probe's own law the
fixer serves): the whole-file rustfmt output the REGION-restricted
hunks the original text applies — the round's own regions the fix
wears, the OTHER regions the drift never touches.
Usage: python3 scripts/fmt_fix_461.py"""
import difflib
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

REGIONS = {
    "crates/mc-client/src/screens.rs": [
        (
            "THE ACTIVE OFFER'S OWN BANK (javap 26.2 MerchantContainer",
            "THE WINDOW'S OWN TITLE (the OpenScreen word's decode",
        ),
        (
            "THE MERCHANT'S OWN CLOSE (javap of `MerchantMenu.removed`",
            "THE MERCHANT'S OWN HOVER SEAT (the chest family's own shape",
        ),
        (
            "THE MERCHANT'S OWN SLOT WALK (javap 26.2 AbstractContainerMenu.",
            "/// THE BEACON'S OWN SCREEN CLICK (the autopush-322 round",
        ),
    ],
    "crates/mc-client/src/world_view.rs": [
        (
            "pub struct MerchantOfferView {",
            "/// The merchant window's projected bank",
        ),
        (
            "THE ACTIVE OFFER'S OWN SEARCH (javap 26.2 MerchantContainer.",
            "/// The quickMoveStack walk over the player's own store (javap 26.2",
        ),
        (
            ".map(|o| MerchantOfferView {",
            "level: oc.merchant.level,",
        ),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        (
            "offers: vec![crate::world_view::MerchantOfferView {",
            "level: 2,",
        ),
        (
            "THE RESULT SEAT'S OWN TAKE (the 460 round's own law",
            "THE STANDARD ROW'S OWN WRITE-THROUGH",
        ),
        (
            "the RESULT source the rows the quick-move serves",
            "THE GATHER'S OWN REFUSAL",
        ),
        (
            "THE TRADE'S OWN TRY-MOVE (javap 26.2 MerchantMenu.",
            "THE RESULT SEAT'S OWN TAKE (javap 26.2 MerchantResultSlot",
        ),
        (
            "THE RESULT SEAT'S OWN TAKE (javap 26.2 MerchantResultSlot",
            None,
        ),
    ],
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
    for path, regions in REGIONS.items():
        original = open(path, encoding="utf-8").read()
        orig_lines = original.split("\n")
        formatted = formatted_of(path)
        sm = difflib.SequenceMatcher(None, orig_lines, formatted.split("\n"))
        spans = []
        cursor = 0
        for (start_marker, end_marker) in regions:
            s = find(original, start_marker, cursor)
            e = region_end(original, end_marker, s)
            cursor = e
            s_line = original[:s].count("\n")
            e_line = original[:e].count("\n")
            spans.append((s_line, e_line))
        # walk the opcodes BACKWARD so the line indices stay valid; apply
        # only the hunks that INTERSECT a region
        ops = [op for op in sm.get_opcodes() if op[0] != "equal"]
        replaced = 0
        for tag, i1, i2, j1, j2 in reversed(ops):
            if any(i1 < e and i2 > s for (s, e) in spans):
                orig_lines[i1:i2] = formatted.split("\n")[j1:j2]
                replaced += 1
        open(path, "w", encoding="utf-8").write("\n".join(orig_lines))
        print(f"{path}: {replaced} region hunks applied")
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
