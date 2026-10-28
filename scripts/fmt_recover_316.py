#!/usr/bin/env python3
"""autopush-316 — THE FMT DAMAGE RECOVERY (the reverse splice): the
whole-file rustfmt the module tree walked (screens.rs's own mod tree
the part_4 noise fixed too) — the recovery keeps the backup's CANONICAL
region content (my round's edits, rustfmt-clean) and restores the HEAD's
lines EVERYWHERE ELSE (the pre-existing static the HEAD law keeps).

The regions = the marker ranges the round's own edits define. Exit 0 iff
every merged file's diff vs HEAD AND vs backup stays inside the regions.
"""
import difflib
import sys

B = "/tmp/fmt316_backup/"


def read(p):
    return open(p).read().split("\n")


def write(p, lines):
    open(p, "w").write("\n".join(lines))


def find_all(lines, needle):
    return [i for i, l in enumerate(lines) if needle in l]


def find_first(lines, needle, start=0):
    for i in range(start, len(lines)):
        if needle in lines[i]:
            return i
    return None


def regions_backup(bl, specs):
    """specs: list of (start_marker, end_marker) — the range =
    [start_line .. end_line) in the BACKUP lines."""
    out = []
    for start, end in specs:
        if end == "@@COUNT@@":  # (marker, closing-line marker) pair form
            starts = find_all(bl, start)
            for s in starts:
                e = find_first(bl, end, s + 1)
                assert e is not None, f"no close {end} after {s}"
                out.append((s, e + 1))
            continue
        s = find_first(bl, start)
        assert s is not None, f"no start {start!r}"
        e = find_first(bl, end, s + 1)
        assert e is not None, f"no end {end!r} after {s}"
        out.append((s, e))
    return out


def in_regions(i, regions):
    for (s, e) in regions:
        if s <= i < e:
            return True
    return False


def merge(hl, bl, regions, path):
    sm = difflib.SequenceMatcher(None, hl, bl, autojunk=False)
    out = []
    spill = 0
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            out.extend(hl[i1:i2])
            continue
        # the hunk = [i1..i2) head vs [j1..j2) backup — the region
        # verdict: ANY overlap with the backup's regions keeps the
        # BACKUP's lines, else the HEAD's (the pre-existing static)
        overlaps = any(
            (j1 < re_ and j2 > rs) or (i1 < re_ and i2 > rs)
            for (rs, re_) in regions
        )
        if overlaps:
            out.extend(bl[j1:j2])
        else:
            out.extend(hl[i1:i2])
            spill += (i2 - i1)
    return out, spill


PAIRS = {
    "crates/mc-client/src/world.rs": [
        ("/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round",
         "/// THE CHORUS WALK'S OWN SAFE SEAT"),
        ("    fn the_gateway_the_beam_the_starfield_the_laws_wear() {",
         "    fn the_chorus_the_walk_down_and_the_safe_verdicts_wear() {"),
    ],
    "crates/mc-client/src/mesher.rs": [
        ("/// THE END GATEWAY'S OWN SENTINEL (the autopush-316 round",
         "/// THE HEAD'S OWN MODEL-FACE TABLE"),
        ("// THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310",
         "// THE BED'S OWN TWO-CELL LANE"),
    ],
    "crates/mc-client/src/screens.rs": [
        ("/// THE END GATEWAY'S OWN STORE WORDS (the autopush-310 round",
         "/// THE BED SPAWN'S OWN BANK"),
        ("// THE GATEWAY CLOCK'S OWN DECAY (the autopush-316 round",
         "if !self.world_is_network && !self.drops.is_empty() {"),
        ("// THE GATEWAY'S OWN PUFFS (the autopush-316 round",
         "let animate = &mut self.ambient_rng;"),
        ("crate::world::SMOKER_LIT => {",
         "// THE PLAYLOCALSOUND WALK"),
        ("for (px, py, pz) in smoker_smokes {",
         "match self.screen {"),
        ("pub end_portal_sky: Option<(u32, u32, Vec<u8>)>,",
         "/// THE PIG/COW VARIANTS' OWN SKINS"),
        ("// THE END PORTAL'S OWN PASS (the BE-renderer lane",
         "// THE DROP LANE's entities"),
        ("/// THE GATEWAY BEAM'S OWN RENDER (the autopush-316 round",
         "/// THE PAIR'S OWN LOADER"),
    ],
    "crates/mc-client/src/screens/tests/part_11.rs": [
        ("crate::screens::GatewayStore {", ")", ),  # the pair form below
        ("/// THE GATEWAY'S OWN BEAM STAR LANE (the autopush-316 round",
         "/// THE BED'S OWN PLACE WALK"),
    ],
    "crates/mc-protocol/src/nbt.rs": [
        ("/// Long view (the BE clocks like the gateway's own `Age` word",
         "/// Float view (the timeline track values"),
    ],
}

# part_11's store-literal regions: EVERY `app.gateway_stores.insert(`
# occurrence through its closing `    );` line.
STORE_SPECS = ("app.gateway_stores.insert(", "@@COUNT@@")


def main():
    bad = 0
    for path, specs in PAIRS.items():
        hl = read(path)
        bl = read(B + path.rsplit("/", 1)[1])
        if "part_11" in path:
            regs = regions_backup(bl, [STORE_SPECS])
        else:
            regs = regions_backup(bl, specs)
        merged, spill = merge(hl, bl, regs, path)
        write(path, merged)
        # THE VERDICTS: the merged vs HEAD = in-region only; the merged
        # vs BACKUP = in-region only
        for other, name in ((hl, "HEAD"), (bl, "BACKUP")):
            sm = difflib.SequenceMatcher(None, merged, other, autojunk=False)
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag == "equal":
                    continue
                # the hunk must fall inside SOME region (either file's
                # numbering — approximate via the backup's regions for
                # the backup side, the head-side probe for HEAD)
                if name == "BACKUP":
                    ok = any(j1 < re_ and j2 > rs for (rs, re_) in regs)
                else:
                    ok = any(
                        any(
                            (rs <= i < re_)
                            for i in range(i1, i2)
                            for (rs, re_) in regs
                        )
                        or (i1 >= len(hl) and True)
                        for (rs, re_) in regs
                    )
                if not ok:
                    bad += 1
                    print(f"SPILL {path} vs {name}: [{i1}..{i2})/{[j1, j2]}")
                    for l in merged[max(0, i1 - 1):i2 + 1][:8]:
                        print(f"   | {l}")
        print(f"{path}: regions={len(regs)} merged, spill={spill}")
    if bad:
        print(f"RECOVERY VERDICT: {bad} spill(s)")
        sys.exit(1)
    print("RECOVERY VERDICT: all in-region")


if __name__ == "__main__":
    main()
