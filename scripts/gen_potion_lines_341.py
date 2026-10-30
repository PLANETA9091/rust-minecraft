#!/usr/bin/env python3
"""THE EFFECT-LINES' OWN GENERATOR (autopush-341 — the potion-kind's own
effect-lines round). Parses the 26.2 jar's javap banks:

  scripts/jar_work/mob_effects_340.txt   — the MobEffects bootstrap: the
      getstatic MobEffectCategory.X the ctor's FIRST arg wears (the
      category the tooltip ink answers);
  scripts/jar_work/potions_clinit_340.txt — the Potions bootstrap: the
      MobEffectInstance ctor's int pushes — the (Holder,I) rows the
      single push the DURATION wears, the (Holder,II) rows the first
      push the duration the second the amplifier.

Emits the Rust tables into crates/mc-client/src/brewing_sim.rs
APPEND-ONLY (the file's own tail), cross-verifies the amplifier rows
against the 340 round's POTION_KIND_EFFECTS table, and prints the shape
facts the worklog banks. Never edits existing lines.
"""
import re
import sys

ROOT = "/home/z/my-project/rust-minecraft"
EFFECTS_TXT = f"{ROOT}/scripts/jar_work/mob_effects_340.txt"
POTIONS_TXT = f"{ROOT}/scripts/jar_work/potions_clinit_340.txt"
BREWING = f"{ROOT}/crates/mc-client/src/brewing_sim.rs"

CAT_ID = {"BENEFICIAL": 0, "HARMFUL": 1, "NEUTRAL": 2}


def parse_effect_categories():
    """(name, category) rows from the MobEffects bootstrap, file order."""
    txt = open(EFFECTS_TXT).read()
    rows = []
    for m in re.finditer(
        r'(?:ldc|ldc_w)\s+\S+\s+// String (\w+)\n(.*?)invokespecial\s+\S+\s+// Method net/minecraft/world/effect/\w*MobEffect\."<init>"',
        txt,
        re.S,
    ):
        name = m.group(1)
        body = m.group(2)
        c = re.search(r"getstatic\s+\S+\s+// Field net/minecraft/world/effect/MobEffectCategory\.(\w+)", body)
        if not c:
            continue
        rows.append((name, CAT_ID[c.group(1)]))
    seen = set()
    out = []
    for name, cat in rows:
        if name in seen:
            continue
        seen.add(name)
        out.append((name, cat))
    return out


def parse_potion_durations():
    """Potions bootstrap rows in putstatic order — (base_name,
    [(effect, duration, amplifier), ...]) per potion."""
    txt = open(POTIONS_TXT).read()
    blocks = re.split(
        r'invokespecial\s+\S+\s+// Method net/minecraft/world/item/alchemy/Potion\."<init>":\(Ljava/lang/String;\[Lnet/minecraft/world/effect/MobEffectInstance;\)V',
        txt,
    )
    rows = []
    for block in blocks[:-1]:
        base = re.search(
            r'(?:ldc|ldc_w)\s+\S+\s+// String (\w+)\n\s+\d+: iconst_\d+\n\s+\d+: anewarray\s+\S+\s+// class net/minecraft/world/effect/MobEffectInstance',
            block,
        )
        if not base:
            continue
        base_name = base.group(1)
        body = block[base.end():]
        insts = re.finditer(
            r'getstatic\s+\S+\s+// Field net/minecraft/world/effect/MobEffects\.(\w+):.*?invokespecial\s+\S+\s+// Method net/minecraft/world/effect/MobEffectInstance\."<init>":\(Lnet/minecraft/core/Holder;(I{1,2})\)V',
            body,
            re.S,
        )
        insts_rows = []
        for im in insts:
            seg = im.group(0)
            effect = re.search(r"MobEffects\.(\w+):", seg).group(1).lower()
            two_arg = im.group(2) == "II"
            pushes = re.findall(
                r'(?:iconst_(\d)\s*\n|bipush\s+(-?\d+)\s*\n|sipush\s+(-?\d+)\s*\n|(?:ldc|ldc_w)\s+\S+\s+// int (-?\d+)\s*\n)',
                seg,
            )
            vals = [next(v for v in p if v != "") for p in pushes]
            if two_arg:
                dur, amp = int(vals[0]), int(vals[1])
            else:
                dur, amp = int(vals[0]) if vals else -1, 0
            insts_rows.append((effect, dur, amp))
        rows.append((base_name, insts_rows))
    return rows


def main():
    cats = parse_effect_categories()
    potions = parse_potion_durations()
    if not cats or not potions:
        print("PARSE FAIL", len(cats), len(potions))
        return 1
    assert len(potions) == 46, f"expected 46 potions, got {len(potions)}"
    assert len(cats) >= 40, f"expected >= 40 effects, got {len(cats)}"

    src = open(BREWING).read()
    effects_tbl = re.search(
        r"pub const POTION_KIND_EFFECTS: \[&\[\(&str, i32\)\]; 46\] = \[(.*?)\n\];",
        src, re.S,
    )
    assert effects_tbl, "the 340 effects table not found"
    kind_rows = re.findall(r"&\[(.*?)\],", effects_tbl.group(1), re.S)
    kind_effects = []
    for row in kind_rows:
        pairs = re.findall(r'\("(\w+)", (-?\d+)\)', row)
        kind_effects.append([(e, int(a)) for e, a in pairs])
    assert len(kind_effects) == 46, f"effects rows {len(kind_effects)}"

    # THE CROSS-VERIFY: the durations parse the SAME (effect, amplifier)
    # pairs the 340 table wears, the base names the strip law's rows
    # match (the long_/strong_ rows the SAME base name answer).
    names_tbl = re.search(
        r"pub const POTION_KIND_BASE_NAMES: \[&str; 46\] = \[(.*?)\];", src, re.S
    )
    kind_names = re.findall(r'"(\w+)"', names_tbl.group(1))
    cat_names = {n for n, _ in cats}
    for i, (base, insts) in enumerate(potions):
        assert base == kind_names[i], f"row {i}: base {base} != {kind_names[i]}"
        parsed_pairs = [(e, a) for e, _d, a in insts]
        assert parsed_pairs == kind_effects[i], (
            f"row {i}: pairs {parsed_pairs} != {kind_effects[i]}"
        )
        for e, _d, _a in insts:
            assert e in cat_names, f"row {i}: effect {e} has no category row"
    print("CROSS-VERIFY OK: 46 rows, the pairs the 340 table wears, the bases the strip law answers")

    cat_tbl = "\n".join(
        f'    ("{n}", {c}),  // {"BENEFICIAL" if c == 0 else "HARMFUL" if c == 1 else "NEUTRAL"}'
        for n, c in cats
    )
    dur_tbl = "\n".join(
        "    &[" + ", ".join(str(d) for _e, d, _a in insts) + "],"
        for _base, insts in potions
    )
    append = f"""
// ===== autopush-341 effect-lines tables =====
// THE JAR LAW the live disasm banks (scripts/jar_work/mob_effects_340.txt
// + potions_clinit_340.txt the 26.2 client.jar the evidence serves):
// the categories the MobEffects ctor's first arg wears, the durations
// the MobEffectInstance ctor's int pushes answer. GENERATED by
// scripts/gen_potion_lines_341.py — the parser emits, the 340 pairs
// the cross-verify keeps. Never edits existing lines.

/// THE EFFECT'S OWN CATEGORY ROWS (javap 26.2 MobEffects.<clinit> — the
/// `getstatic MobEffectCategory.X` the ctor's FIRST arg wears, the 40
/// rows the bootstrap registers). The tooltip ink the
/// MobEffectCategory.<clinit> pc 0-45 answers: BENEFICIAL the BLUE,
/// HARMFUL the RED, the NEUTRAL the BLUE the same enum ctor wears.
pub const MOB_EFFECT_CATEGORIES: &[(&str, u8)] = &[
{cat_tbl}
];

/// THE KIND'S OWN DURATION ROWS (javap 26.2 Potions.<clinit> — the
/// MobEffectInstance ctor's int pushes: the `(Holder,I)` rows the
/// single push the DURATION wears, the `(Holder,II)` rows the first
/// push the duration the second the amplifier; the same row order the
/// POTION_KIND_EFFECTS table keeps [the cross-verify the gen run
/// wears]). The instant rows the duration-1 the `endsWithin(20)` gate's
/// own silence answers [the addPotionTooltip pc 86-93 the no-suffix
/// lane keeps].
pub const POTION_KIND_DURATIONS: [&[i32]; 46] = [
{dur_tbl}
];

// ===== end autopush-341 tables =====
"""
    with open(BREWING, "a") as f:
        f.write(append)
    # the shape facts the worklog banks
    spots = {
        "night_vision": None,
        "long_night_vision": None,
        "swiftness": None,
        "strong_swiftness": None,
        "turtle_master": None,
        "healing": None,
        "luck": None,
    }
    for i, (base, insts) in enumerate(potions):
        key = (
            "night_vision" if base == "night_vision" and i == 4
            else "long_night_vision" if i == 5
            else "swiftness" if i == 13
            else "strong_swiftness" if i == 15
            else "turtle_master" if i == 19
            else "healing" if i == 24
            else "luck" if i == 39
            else None
        )
        if key:
            spots[key] = insts
    for k, v in spots.items():
        print(f"  {k}: {v}")
    print(f"TABLES: MOB_EFFECT_CATEGORIES {len(cats)} rows + POTION_KIND_DURATIONS 46 rows appended")
    return 0


if __name__ == "__main__":
    sys.exit(main())
