#!/usr/bin/env python3
"""autopush-316 — THE FMT DAMAGE RECOVERY v2 (the anchor splice): the
HEAD files restored; each of the round's own edits re-applied with the
CANONICAL region content extracted from the rustfmt'd backup (the
markers the round's own comment lines define). The non-region lines
stay the HEAD's own (the pre-existing static the HEAD law keeps).

Exit 0 iff every splice landed (the marker verify pass)."""

B = "/tmp/fmt316_backup/"

FILES = {
    "world": "crates/mc-client/src/world.rs",
    "mesher": "crates/mc-client/src/mesher.rs",
    "screens": "crates/mc-client/src/screens.rs",
    "part11": "crates/mc-client/src/screens/tests/part_11.rs",
    "nbt": "crates/mc-protocol/src/nbt.rs",
}


def backup_region(key, start_marker, end_marker):
    """The backup's lines [start_marker .. end_marker) — the canonical
    region content (the markers = the round's own comment/doc lines,
    rustfmt-identical in both)."""
    bl = open(B + FILES[key].rsplit("/", 1)[1]).read()
    s = bl.index(start_marker)
    e = bl.index(end_marker, s)
    assert s < e, f"{key}: bad region order"
    return bl[s:e]


def main():
    # ---------- world.rs region A (the pure fns block) ----------
    key = "world"
    path = FILES[key]
    text = open(path).read()
    old = (
        "    best.unwrap_or((cx, cy, cz))\n"
        "}\n"
        "\n"
        "/// THE CHORUS WALK'S OWN SAFE SEAT (javap LivingEntity.randomTeleport"
    )
    region = backup_region(
        key,
        "/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round",
        "/// THE CHORUS WALK'S OWN SAFE SEAT (javap LivingEntity.randomTeleport",
    )
    prefix = "    best.unwrap_or((cx, cy, cz))\n}\n\n"
    new = prefix + region
    assert old in text, "world A anchor missing"
    text = text.replace(old, new, 1)
    open(path, "w").write(text)

    # ---------- world.rs region B (the test block) ----------
    old = (
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"
    )
    # the anchor = the gateway-exit test's closing brace + the chorus
    # test header — locate via the exit test's last assert tail
    anchor = (
        "            \"the center cell the skip keeps — the neighbor the win wears\"\n"
        "        );\n"
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"
    )
    region = backup_region(
        key,
        "    #[test]\n    fn the_gateway_the_beam_the_starfield_the_laws_wear() {",
        "    #[test]\n    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {",
    )
    prefix = (
        "            \"the center cell the skip keeps — the neighbor the win wears\"\n"
        "        );\n"
        "    }\n"
        "\n"
    )
    new = prefix + region
    assert anchor in text, "world B anchor missing"
    text = text.replace(anchor, new, 1)
    open(path, "w").write(text)

    # ---------- mesher.rs region A (the TILE const) ----------
    key = "mesher"
    path = FILES[key]
    text = open(path).read()
    old = "pub const TILE_END_PORTAL: u16 = 65534;\n"
    region = backup_region(
        key,
        "/// THE END GATEWAY'S OWN SENTINEL (the autopush-316 round",
        "/// THE HEAD'S OWN MODEL-FACE TABLE",
    )
    assert text.count(old) == 1, "mesher A anchor not unique"
    text = text.replace(old, old + "\n" + region, 1)
    open(path, "w").write(text)

    # ---------- mesher.rs region B (the gateway cube arm) ----------
    old_start = "                // THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310"
    old_end = "                // THE BED'S OWN TWO-CELL LANE (the dev-194 round"
    s = text.index(old_start)
    e = text.index(old_end, s)
    region = backup_region(
        key,
        "                // THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310",
        "                // THE BED'S OWN TWO-CELL LANE (the dev-194 round",
    )
    text = text[:s] + region + text[e:]
    open(path, "w").write(text)

    # ---------- nbt.rs (the as_i64 insert) ----------
    key = "nbt"
    path = FILES[key]
    text = open(path).read()
    anchor = "    /// Float view (the timeline track values"
    region = backup_region(
        key,
        "    /// Long view (the BE clocks like the gateway's own `Age` word",
        "    /// Float view (the timeline track values",
    )
    assert text.count(anchor) == 1
    text = text.replace(anchor, region, 1)
    open(path, "w").write(text)

    print("world/mesher/nbt spliced")


if __name__ == "__main__":
    main()
