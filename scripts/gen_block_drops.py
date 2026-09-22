#!/usr/bin/env python3
"""mc2rs-gen helper — emits crates/mc-registry/src/generated/block_drops.rs
from the official jar's data/minecraft/loot_table/blocks/*.json.

THE CLIENT'S HAND LANE: the survival break with the bare hand — no shears,
no silk touch, no fortune, no explosion. The parse resolves the loot tree
under exactly those terms (the vanilla LootTable eval's own branches,
javap-pinned):

  - pool conditions: survives_explosion (pass), inverted(any_of(match_tool
    shears/silk)) (pass for the hand), block_state_property (resolved
    against the block's DEFAULT state — the runtime probe's output)
  - pools are INDEPENDENT (each fires its own rolls — a leaves break can
    yield the sapling AND a stick AND an apple)
  - within a pool, the entries walk IN ORDER (the vanilla's own eval
    order): a DEFINITE pass wins the pool (the walk stops); a ROLL-valued
    condition (table_bonus at fortune 0, random_chance) emits its row and
    the walk CONTINUES (the next sibling is the fallback — the gravel's
    flint@0.10 then the gravel itself); a definite fail skips
  - the alternatives tree and the any_of terms walk the same way (the
    first DEFINITE child wins; the roll children emit and continue)
  - count: the uniform's INCLUSIVE int law — javap Mth.nextInt:
    `min + nextInt(max - min + 1)` over the int-cast constants; the
    absolute count rides the int cast alone
  - explosion_decay is a no-op (no explosion); apply_bonus at fortune 0
    returns the base count verbatim (javap ApplyBonusCount$OreDrops)

The TOOL GATE is NOT here: `requiresCorrectToolForDrops` rides the
generated block_dig table (the dig lane's own flag) — the drop call
refuses everything when the hand cannot harvest. The rows carry only the
loot table's own answer.

The RNG stays the CLIENT's: the rolls use the drop lane's own LCG (the
audio lane's precedent) — the emitted table is pure data.

Default states (the runtime probe's own output, ProbeDefaultStates.java):
    stone|, dirt|, grass_block|snowy=false, oak_log|axis=y,
    oak_leaves|distance=7,persistent=false,waterlogged=false, water|level=0,
    sand|, granite|, diorite|, andesite|, deepslate|axis=y,
    cobbled_deepslate|, gravel|, bedrock|, coal_ore|, copper_ore|,
    iron_ore|, gold_ore|, redstone_ore|lit=false, lapis_ore|, diamond_ore|,
    emerald_ore|, netherrack|, pale_oak_leaves|distance=7,persistent=false,
    waterlogged=false, leaf_litter|facing=north,segment_amount=1
"""

import json
import sys
import zipfile
from pathlib import Path

JAR = Path(".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785")
OUT = Path("crates/mc-registry/src/generated/block_drops.rs")

# the client palette's default state property maps (the probe's own lines)
DEFAULT_STATES = {
    "grass_block": {"snowy": "false"},
    "oak_log": {"axis": "y"},
    "oak_leaves": {"distance": "7", "persistent": "false", "waterlogged": "false"},
    "water": {"level": "0"},
    "deepslate": {"axis": "y"},
    "redstone_ore": {"lit": "false"},
    "pale_oak_leaves": {"distance": "7", "persistent": "false", "waterlogged": "false"},
    "leaf_litter": {"facing": "north", "segment_amount": "1"},
    # the primed-TNT round's own read (javap TntBlock.<init> pc 5-23:
    # `registerDefaultState(this.stateDefinition.any().setValue(UNSTABLE,
    # false))` — the placed block's own state the loot condition's
    # `unstable: false` row answers, the mined TNT the item the pool
    # drops)
    "tnt": {"unstable": "false"},
    # the double-plant round's own read (javap DoublePlantBlock.<init>
    # over the 26.2 jar: `registerDefaultState(this.stateDefinition.any()
    # .setValue(HALF, DoubleBlockHalf.LOWER))` — the ctor's own lower
    # half the register walk banks; the worldgen jsons' simple_state_
    # providers the same `half=lower` state speaks)
    "tall_grass": {"half": "lower"},
    "large_fern": {"half": "lower"},
}

MISSING = object()


def base(cond):
    return cond["condition"].split(":")[-1]


def cond_roll(c, defaults, table_name=""):
    """A condition's hand-lane answer: True (definite pass), False
    (definite fail), or the roll chance the LCG must beat."""
    k = base(c)
    if k == "survives_explosion":
        return True  # no explosion speaks here
    if k == "match_tool":
        # the predicate's items/enchantments — the bare hand answers no
        return False
    if k == "table_bonus":
        return c["chances"][0]  # the fortune-0 chance
    if k in ("random_chance", "random_chance_with_looting"):
        return c["chance"]  # the looting-0 base
    if k == "location_check":
        # THE DOUBLE PLANT'S OWN PAIR GATE (the 26.2 jar: the tall_grass
        # and large_fern tables the ONLY block-loot location_check rows
        # speak — each pool's offset predicate names the block's OWN
        # pair mate: half=upper at offsetY +1, half=lower at -1). The
        # dig lane's own truth: the pair STANDS at any diggable moment
        # (the updateShape walk the lone half never leaves — the other
        # half's removal breaks this one before a ray can target it),
        # so the predicate naming the table's own block answers the
        # definite pass. A location_check naming any OTHER block stays
        # unresolvable (the honest False the lanes never speak).
        pred = c.get("predicate") or {}
        blocks = (pred.get("block") or {}).get("blocks", "")
        if isinstance(blocks, str) and blocks.split(":")[-1] == table_name:
            return True
        return False
    if k == "any_of":
        # the OR answers in the BOOL domain only: a definite term wins, a
        # roll term is un-representable inside an AND (MISSING)
        for t in c["terms"]:
            r = cond_roll(t, defaults, table_name)
            if r is True:
                return True
            if r is False or r is MISSING:
                continue
            return MISSING
        return False
    if k == "all_of":
        chance = 1.0
        for t in c["terms"]:
            r = cond_roll(t, defaults, table_name)
            if r is MISSING or r is False:
                return r
            if r is True:
                continue
            chance *= r
        return chance
    if k == "inverted":
        inner = cond_roll(c["term"], defaults, table_name)
        if inner is MISSING:
            return MISSING
        return not bool(inner)
    if k == "block_state_property":
        props = c.get("properties", {})
        want = c.get("block", "").split(":")[-1]
        # THE DEFAULT STATE'S OWN RESOLUTION (the doc's own law — the
        # condition names the block the table speaks, the props the
        # DEFAULT_STATES row wears: `defaults.get(want)` here read the
        # per-block props map by BLOCK NAME and always refused — the
        # module table the honest lookup keeps, the placed TNT's
        # `unstable: false` row the gate now answers)
        d = DEFAULT_STATES.get(want, {})
        return all(d.get(pk) == pv for pk, pv in props.items())
    if k in ("weather_check", "time_check", "location_check", "entity_properties",
             "killed_by_player", "random_difficulty_chance"):
        # the block-break lanes never speak these in our vocabulary
        return False
    return MISSING


def conditions_chance(conds, defaults, table_name=""):
    """The AND of the conditions: None = the walk failed; (chance, won)
    where chance < 1.0 rides a roll."""
    chance = 1.0
    for c in conds:
        r = cond_roll(c, defaults, table_name)
        if r is False or r is MISSING:
            return None
        if r is True:
            continue
        chance *= r
    return chance


def count_of_unused_removed():
    pass


def count_of(entry):
    """The set_count function's (min, max) inclusive int span."""
    for fn in entry.get("functions", []):
        k = fn["function"].split(":")[-1]
        if k != "set_count":
            continue
        fcon = fn.get("conditions", [])
        if fcon and conditions_chance(fcon, DEFAULT_STATES.get("__self__", {})) is None:
            continue  # the caller resolves the gated chain separately
        cnt = fn["count"]
        if isinstance(cnt, (int, float)):
            return (int(cnt), int(cnt))
        if cnt.get("type", "").endswith("uniform"):
            return (int(cnt["min"]), int(cnt["max"]))
    return (1, 1)


def collect_entry(e, defaults, rows, table_name=""):
    """One entry into the pool's row list. Returns True when the walk is
    DEFINITELY done (a definite pass ended it)."""
    t = e.get("type", "")
    if t.endswith("alternatives"):
        # the case chain: the children walk IN ORDER — a definite pass
        # ends the walk, a roll child emits its row and the walk
        # continues (the next child is the fallback — the gravel's
        # flint@0.1 then the gravel), a definite fail skips; the nested
        # alternatives recurses with the same law
        done = False
        for child in e.get("children", []):
            ct = child.get("type", "")
            if ct.endswith("alternatives"):
                if collect_entry(child, defaults, rows, table_name):
                    done = True
                    break
                continue
            if not ct.endswith("item"):
                continue
            ch = conditions_chance(child.get("conditions", []), defaults, table_name)
            if ch is None:
                continue
            if ch >= 1.0:
                rows.append((child["name"].split(":")[-1], count_gated(child, defaults), 1.0))
                done = True
                break
            rows.append((child["name"].split(":")[-1], count_gated(child, defaults), ch))
        return done
    if not t.endswith("item"):
        return False
    ch = conditions_chance(e.get("conditions", []), defaults, table_name)
    if ch is None:
        return False
    if ch >= 1.0:
        rows.append((e["name"].split(":")[-1], count_gated(e, defaults), 1.0))
        return True
    rows.append((e["name"].split(":")[-1], count_gated(e, defaults), ch))
    return False


def count_gated(entry, defaults):
    """The set_count chain with block_state_property gates resolved
    against the block's own default state (leaf_litter's law)."""
    for fn in entry.get("functions", []):
        k = fn["function"].split(":")[-1]
        if k != "set_count":
            continue
        fcon = fn.get("conditions", [])
        if fcon:
            ok = conditions_chance(fcon, defaults)
            if ok is None or ok is False or ok is MISSING:
                continue  # the gate refuses this branch
            if ok < 1.0:
                continue  # a roll-gated count: not in the vocabulary
        cnt = fn["count"]
        if isinstance(cnt, (int, float)):
            return (int(cnt), int(cnt))
        if cnt.get("type", "").endswith("uniform"):
            return (int(cnt["min"]), int(cnt["max"]))
    return (1, 1)


def parse_table(doc, defaults, table_name=""):
    pools = []
    for pool in doc.get("pools", []):
        pconds = pool.get("conditions", [])
        if pconds:
            ok = conditions_chance(pconds, defaults, table_name)
            if ok is None or ok is False or ok is MISSING:
                continue
            if ok < 1.0:
                # a roll-gated POOL: not in the palette's vocabulary
                print(f"  ! roll-gated pool skipped ({pool})", file=sys.stderr)
                continue
        entries = pool.get("entries", [])
        if len(entries) > 1:
            # the weighted-pick shape: not in the palette's vocabulary —
            # the honest empty beats a wrong guess
            print("  ! multi-entry pool emitted empty", file=sys.stderr)
            continue
        rows = []
        for e in entries:
            if collect_entry(e, defaults, rows, table_name):
                break
        if rows:
            pools.append(rows)
    return pools


def main():
    z = zipfile.ZipFile(JAR)
    tables = sorted(
        n for n in z.namelist()
        if n.startswith("data/minecraft/loot_table/blocks/") and n.endswith(".json")
    )
    rows = []
    for n in tables:
        name = n.rsplit("/", 1)[1][:-5]
        doc = json.loads(z.read(n))
        pools = parse_table(doc, DEFAULT_STATES.get(name, {}), name)
        rows.append((name, pools))
    rows.sort(key=lambda r: r[0])

    lines = []
    lines.append("// @generated by scripts/gen_block_drops.py from data/minecraft/loot_table/blocks/")
    lines.append("// of the official jar (clean-room JSON parse, the client's hand lane: no")
    lines.append("// shears, no silk touch, no fortune, no explosion; the tool gate rides the")
    lines.append("// generated block_dig table's requiresCorrectToolForDrops flag).")
    lines.append("//")
    lines.append("// THE EVAL LAW (javap LootTable): the pools are independent (each rolls");
    lines.append("// its own chain); within a pool the entries walk IN ORDER — a definite")
    lines.append("// pass wins the pool, a roll row (chance < 1) emits and the walk")
    lines.append("// continues (the next row is the fallback — the gravel's flint@0.1")
    lines.append("// then the gravel), a fail skips. The count rides the INCLUSIVE int")
    lines.append("// law (javap Mth.nextInt: min + nextInt(max - min + 1)).")
    lines.append("")
    lines.append("/// One drop case: the item's bare name, the INCLUSIVE count span, and")
    lines.append("/// the chance the LCG must beat (1.0 = the definite pass).")
    lines.append("pub struct DropEntry {")
    lines.append("    pub item: &'static str,")
    lines.append("    pub count_min: i32,")
    lines.append("    pub count_max: i32,")
    lines.append("    pub chance: f32,")
    lines.append("}")
    lines.append("")
    lines.append("/// One pool's case chain (the walk order preserved).")
    lines.append("pub struct DropPool {")
    lines.append("    pub entries: &'static [DropEntry],")
    lines.append("}")
    lines.append("")
    named = []
    for name, pools in rows:
        if not pools:
            continue
        sym = f"DROPS_{name.upper()}"
        named.append((name, sym, len(pools)))
        lines.append(f"pub static {sym}: [DropPool; {len(pools)}] = [")
        for pool in pools:
            entries = ", ".join(
                f'DropEntry {{ item: "{it}", count_min: {cmin}, count_max: {cmax}, chance: {ch} }}'
                for (it, (cmin, cmax), ch) in pool
            )
            lines.append(f"    DropPool {{ entries: &[{entries}] }},")
        lines.append("];")
        lines.append("")
    lines.append("/// The block's hand-lane drop rows, sorted by name (the binary search's")
    lines.append("/// own demand). Blocks with no pools (the tool-gated table, the missing")
    lines.append("/// file — bedrock) drop nothing: the absence IS the vanilla's own.")
    lines.append(f"pub static BLOCK_DROPS: [(&str, &[DropPool]); {len(named)}] = [")
    for name, sym, n in named:
        lines.append(f'    ("{name}", &{sym}),')
    lines.append("];")
    lines.append("")
    lines.append("pub fn drops(name: &str) -> Option<&'static [DropPool]> {")
    lines.append("    BLOCK_DROPS")
    lines.append("        .binary_search_by(|probe| probe.0.cmp(name))")
    lines.append("        .ok()")
    lines.append("        .map(|i| BLOCK_DROPS[i].1)")
    lines.append("}")
    lines.append("")
    OUT.write_text("\n".join(lines))
    print(f"emitted {OUT} — {len(named)} tables with drops")
    pal = "stone dirt grass_block oak_log oak_leaves water sand granite diorite andesite deepslate cobbled_deepslate gravel bedrock coal_ore copper_ore iron_ore gold_ore redstone_ore lapis_ore diamond_ore emerald_ore netherrack pale_oak_leaves leaf_litter".split()
    d = dict(rows)
    for p in pal:
        v = d.get(p)
        print(f"  {p:22s} {v if v else 'NO DROPS'}")


if __name__ == "__main__":
    main()
