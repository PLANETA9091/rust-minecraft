#!/usr/bin/env python3
"""dev-222 — THE FMT REGION WALK (the 457 record's own law the round
wears): the whole-file rustfmt the ORIGINAL text the formatted stdout
the diff answers — the MY-REGIONS verdict the marker ranges keep (no
hunk may touch the round's own edits).

Regions:
  screens.rs    — the merchant fields + the decode arm + the close
                  walk + the widget press + the render round the
                  round banks (no rustfmt hunk may touch them).
  play.rs       — the merchant wire walk + the parse + the select
                  writer + the session method the round banks.
  world_model.rs — the MENU_TYPE_MERCHANT const + the MerchantBank +
                  the apply arm the round banks.
  world_view.rs — the LiveContainer merchant ride + the view structs +
                  the SelectTrade variant the round banks.
  window.rs     — the SelectTrade consumer arm the round banks.
  part_11.rs    — the round's own test (the merchant offers wire
                  words wear, EOF the end the round keeps).
"""
import difflib
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

SCREENS = [
    # the merchant fields (the state the route wears)
    (
        "THE MERCHANT SCREEN'S OWN LIVE FLAG (the beacon route's own law",
        "pub merchant_row_rects: Vec<(i32, i32, usize)>,",
    ),
    # the close walk (the removed() law the window serves)
    (
        "THE MERCHANT'S OWN CLOSE (javap of `MerchantMenu.removed`",
        "/// THE BEACON'S OWN SCREEN CLICK (the autopush-322 round",
    ),
    # the widget press + the render round (the row lanes the press serves)
    (
        "THE MERCHANT'S OWN WIDGET PRESS (javap of the TradeOfferButton",
        "/// THE BEACON'S OWN HOVER-TOOLTIP SEAT (the 324 round",
    ),
    # the decode arm (the wire lane the menu type 18 opens)
    (
        "if c.menu_type_id == crate::world_model::MENU_TYPE_MERCHANT {",
        "if c.menu_type_id == crate::world_model::MENU_TYPE_ENCHANTMENT {",
    ),
]

PLAY = [
    # the merchant wire walk (the structs + the parse the round banks)
    (
        "THE MERCHANT'S OWN COST WORD (javap of `ItemCost`",
        "THE TRADE'S OWN SELECT WIRE (javap of",
    ),
    # the select writer (the bytes the postButtonClick wears)
    (
        "THE TRADE'S OWN SELECT WIRE (javap of",
        "/// Set Held Slot (clientbound play, generated id 105",
    ),
    # the session method (the send lane the press rides)
    (
        "The trade's OWN select word (the vanilla",
        "/// The player's right-mouse USE (the vanilla gameMode.useItem's own",
    ),
]

WORLD_MODEL = [
    # the const + the bank (the type word the route wears)
    (
        "`MenuType`'s merchant — the VillagerMenu's registry id",
        "pub const MENU_TYPE_MERCHANT: i32 = 18;",
    ),
    (
        "THE MERCHANT'S OWN BANK (the handleMerchantOffers walk's five",
        "The merchant window's banked word set (the `MerchantMenu`'s own",
    ),
    (
        "The merchant window's banked word set (the `MerchantMenu`'s own",
        "`InventoryMenu.isHotbarSlot`'s audited shape",
    ),
    # the apply arm (the five setters the store mirrors)
    (
        "PlayEvent::MerchantOffers(o) => {",
        "PlayEvent::ContainerDataSet(s) => {",
    ),
]

WORLD_VIEW = [
    # the view structs (the projection words the round banks)
    (
        "THE MERCHANT'S OWN BANK (the store's MerchantBank the",
        "The merchant window's projected bank (the store's MerchantBank",
    ),
    (
        "The merchant window's projected bank (the store's MerchantBank",
        "/// The click's input modes, the UI-facing mirror of the wire's own",
    ),
    # the SelectTrade variant (the command the screen emits)
    (
        "THE TRADE'S OWN SELECT WIRE (the vanilla",
        "/// The use RELEASE (the vanilla gameMode.releaseUsingItem's own",
    ),
    # the merchant ride (the builder the projection feeds)
    (
        "THE MERCHANT'S OWN RIDE (the store's bank the projection",
        "/// The player's OWN inventory window's full slot shape",
    ),
]

WINDOW = [
    # the SelectTrade consumer arm (the session the worker speaks)
    (
        "crate::world_view::ContainerCommand::SelectTrade { item } => {",
        "crate::world_view::ContainerCommand::PickBlock { pos, include_data } => {",
    ),
]

PART11 = [
    # the round's own test (the merchant offers wire words wear — EOF
    # the end the round keeps)
    (
        "THE MERCHANT'S OWN OFFERS WIRE (javap the 26.2 jar",
        None,
    ),
]

REGIONS = {
    "crates/mc-client/src/screens.rs": SCREENS,
    "crates/mc-protocol/src/play.rs": PLAY,
    "crates/mc-client/src/world_model.rs": WORLD_MODEL,
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
