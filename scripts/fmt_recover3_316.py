#!/usr/bin/env python3
"""autopush-316 — THE FMT DAMAGE RECOVERY v3: screens.rs / nbt.rs /
part_11.rs (the three the timed-out merge never served). The anchor
splice: the HEAD text + the backup's canonical region content, the
markers the round's own comment/doc lines define."""

B = "/tmp/fmt316_backup/"

SRC_SCREEN = B + "screens.rs"
SRC_NBT = B + "nbt.rs"
SRC_P11 = B + "part_11.rs"


def span(src, start_marker, end_marker):
    s = src.index(start_marker)
    e = src.index(end_marker, s + 1)
    assert s < e, f"bad span {start_marker[:40]!r}"
    return src[s:e]


def main():
    # ---------- nbt.rs ----------
    head = open("crates/mc-protocol/src/nbt.rs").read()
    region = span(
        open(SRC_NBT).read(),
        "    /// Long view (the BE clocks like the gateway's own `Age` word",
        "    /// Float view (the timeline track values",
    )
    anchor = "    /// Float view (the timeline track values"
    assert head.count(anchor) == 1
    head = head.replace(anchor, region, 1)
    open("crates/mc-protocol/src/nbt.rs", "w").write(head)
    print("nbt spliced")

    # ---------- screens.rs ----------
    bl = open(SRC_SCREEN).read()
    text = open("crates/mc-client/src/screens.rs").read()

    # R1: the store words + the bank + the parse (the doc block through
    # the parse fn's tail — the seat before the BED SPAWN'S OWN BANK)
    region = span(
        bl,
        "/// THE END GATEWAY'S OWN STORE WORDS (the autopush-310 round",
        "/// THE BED SPAWN'S OWN BANK",
    )
    s = text.index("/// THE END GATEWAY'S OWN STORE WORDS (the autopush-310 round")
    e = text.index("/// THE BED SPAWN'S OWN BANK", s)
    text = text[:s] + region + text[e:]
    print("screens R1 (store words) spliced")

    # R2: the clock lane (the drop-tick comment .. the drops-gate if)
    region = span(
        bl,
        "// THE GATEWAY CLOCK'S OWN DECAY (the autopush-316 round",
        "if !self.world_is_network && !self.drops.is_empty() {",
    )
    s = text.index("if !self.world_is_network && !self.drops.is_empty() {")
    # the lane rides BEFORE the drops gate — insert before it
    text = text[:s] + region + text[s:]
    print("screens R2 (clock lane) spliced")

    # R3: the gateway_puffs vec (the three-decl block .. the let animate)
    region = span(
        bl,
        "// THE GATEWAY'S OWN PUFFS (the autopush-316 round",
        "let animate = &mut self.ambient_rng;",
    )
    s = text.index("                let animate = &mut self.ambient_rng;")
    # the vec rides before the `let animate` (the same indent family)
    text = text[:s] + region.replace(
        "// THE GATEWAY'S OWN PUFFS",
        "// THE GATEWAY'S OWN PUFFS",
        1,
    ) + text[s:]
    print("screens R3 (puff vec) spliced")

    # R4: the animate arm (the smoker arm's tail .. the PLAYLOCALSOUND
    # comment) — the backup's span carries the gateway arm inside the
    # match; the head's span lacks it
    region = span(
        bl,
        "crate::world::END_GATEWAY => {",
        "                        _ => {}",
    )
    s = text.index("                        _ => {}")
    text = text[:s] + region + text[s:]
    print("screens R4 (animate arm) spliced")

    # R5: the puff plant loop (the smoker plant's tail .. the match
    # self.screen)
    region = span(
        bl,
        "for puff in gateway_puffs {",
        "        match self.screen {",
    )
    s = text.index("        match self.screen {")
    text = text[:s] + region + text[s:]
    print("screens R5 (puff plant) spliced")

    # R6: the beam tex field (the end_portal pair .. the pig/cow doc)
    region = span(
        bl,
        "    /// THE GATEWAY BEAM'S OWN TEXTURE (the autopush-316 round",
        "    /// THE PIG/COW VARIANTS' OWN SKINS",
    )
    anchor = "    /// THE PIG/COW VARIANTS' OWN SKINS"
    assert text.count(anchor) == 1
    text = text.replace(anchor, region, 1)
    print("screens R6 (beam tex field) spliced")

    # R7: the init row
    old = "            end_portal_sky: None,\n            end_portal_noise: None,\n"
    new = old + "            gateway_beam_tex: None,\n"
    assert text.count(old) == 1
    text = text.replace(old, new, 1)
    print("screens R7 (init row) spliced")

    # R8: the render passes (the portal pass .. the drop lane comment)
    region = span(
        bl,
        "// THE END PORTAL'S OWN PASS (the BE-renderer lane",
        "// THE DROP LANE's entities",
    )
    s = text.index("// THE END PORTAL'S OWN PASS (the BE-renderer lane")
    e = text.index("// THE DROP LANE's entities", s)
    text = text[:s] + region + text[e:]
    print("screens R8 (render passes) spliced")

    # R9: the beams fn (the portal walk's tail .. the PAIR'S OWN LOADER)
    region = span(
        bl,
        "    /// THE GATEWAY BEAM'S OWN RENDER (the autopush-316 round",
        "    /// THE PAIR'S OWN LOADER",
    )
    anchor = "    /// THE PAIR'S OWN LOADER"
    assert text.count(anchor) == 1
    text = text.replace(anchor, region, 1)
    print("screens R9 (beams fn) spliced")

    open("crates/mc-client/src/screens.rs", "w").write(text)

    # ---------- part_11.rs ----------
    bl = open(SRC_P11).read()
    text = open("crates/mc-client/src/screens/tests/part_11.rs").read()

    # the two store literals (the age field) — the backup's canonical
    # blocks; each occurrence replaced in order
    old_lit = (
        "        crate::screens::GatewayStore {\n"
        "            exit: exit_cell,\n"
        "            exact: true,\n"
        "            cooldown: 0,\n"
        "        },\n"
    )
    new_lit = (
        "        crate::screens::GatewayStore {\n"
        "            exit: exit_cell,\n"
        "            exact: true,\n"
        "            cooldown: 0,\n"
        "            age: crate::world::GATEWAY_SPAWN_TICKS,\n"
        "        },\n"
    )
    assert text.count(old_lit) == 2, f"literals: {text.count(old_lit)}"
    text = text.replace(old_lit, new_lit, 2)
    print("part11 store literals spliced")

    # the big walk (the discard-keeps tail .. the BED'S OWN PLACE WALK)
    region = span(
        bl,
        "/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round — the",
        "/// THE BED'S OWN PLACE WALK",
    )
    s = text.index("/// THE BED'S OWN PLACE WALK")
    text = text[:s] + region + text[s:]
    print("part11 big walk spliced")

    open("crates/mc-client/src/screens/tests/part_11.rs", "w").write(text)
    print("ALL SPLICED")


if __name__ == "__main__":
    main()
