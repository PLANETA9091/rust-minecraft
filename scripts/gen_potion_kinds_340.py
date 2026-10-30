#!/usr/bin/env python3
"""THE GENERATOR'S OWN WALK (autopush-340 — the potion-kind's own tooltip +
tint round). Parses the 26.2 jar's javap banks:

  scripts/jar_work/mob_effects_340.txt   — the MobEffects bootstrap: the
      register(name, new MobEffect/InstantMobEffect(CATEGORY, COLOR)) rows,
      the color the ctor's ldc int wears;
  scripts/jar_work/potions_clinit_340.txt — the Potions bootstrap: the
      register(key, new Potion("base_name", new MobEffectInstance(EFFECT,
      duration[, amplifier])...)) rows, the putstatic order IS the registry
      id order (the 339 round's own law).

Emits the Rust tables + the blend walk into
crates/mc-client/src/brewing_sim.rs APPEND-ONLY (the file's own tail), and
prints the shape facts the worklog banks. Never edits existing lines.
"""
import re
import sys

ROOT = "/home/z/my-project/rust-minecraft"
EFFECTS_TXT = f"{ROOT}/scripts/jar_work/mob_effects_340.txt"
POTIONS_TXT = f"{ROOT}/scripts/jar_work/potions_clinit_340.txt"
BREWING = f"{ROOT}/crates/mc-client/src/brewing_sim.rs"


def parse_effect_colors():
    """(name, color) rows from the MobEffects bootstrap, file order."""
    txt = open(EFFECTS_TXT).read()
    rows = []
    # each registration block: ldc "name" ... ldc int COLOR ... invokespecial MobEffect."<init>"
    for m in re.finditer(
        r'(?:ldc|ldc_w)\s+\S+\s+// String (\w+)\n(.*?)invokespecial\s+\S+\s+// Method net/minecraft/world/effect/\w*MobEffect\."<init>"',
        txt,
        re.S,
    ):
        name = m.group(1)
        body = m.group(2)
        c = re.search(r"(?:ldc|ldc_w)\s+\S+\s+// int (\d+)", body)
        if not c:
            continue
        rows.append((name, int(c.group(1))))
    # dedupe (the register call sites are the only ctor shapes) keeping order
    seen = set()
    out = []
    for name, color in rows:
        if name in seen:
            continue
        seen.add(name)
        out.append((name, color))
    return out


def parse_potions():
    """Potions bootstrap rows in putstatic order.

    Each block: getstatic PotionIds.X | (first row: the ldc key lane)
                new Potion, dup, ldc "base_name", iconst_N, anewarray,
                then N x (getstatic MobEffects.E; duration ldc/bipush/sipush;
                [amplifier ldc/bipush/sipush/iconst]; invokespecial (Holder,I|II))
                then invokespecial Potion."<init>", register, putstatic FIELD.
    Returns list of (field, base_name, [(effect, amplifier), ...]).
    """
    txt = open(POTIONS_TXT).read()
    rows = []
    # split on the Potion ctor invocation: each registration ends with
    #   invokespecial Potion."<init>":(Ljava/lang/String;[Lnet/minecraft/world/effect/MobEffectInstance;)V
    blocks = re.split(r'invokespecial\s+\S+\s+// Method net/minecraft/world/item/alchemy/Potion\."<init>":\(Ljava/lang/String;\[Lnet/minecraft/world/effect/MobEffectInstance;\)V', txt)
    for block in blocks[:-1]:
        base = re.search(r'(?:ldc|ldc_w)\s+\S+\s+// String (\w+)\n\s+\d+: iconst_\d+\n\s+\d+: anewarray\s+\S+\s+// class net/minecraft/world/effect/MobEffectInstance', block)
        if not base:
            continue
        base_name = base.group(1)
        body = block[base.end():]
        insts = []
        for im in re.finditer(
            r'getstatic\s+\S+\s+// Field net/minecraft/world/effect/MobEffects\.(\w+):.*?invokespecial\s+\S+\s+// Method net/minecraft/world/effect/MobEffectInstance\."<init>":\(Lnet/minecraft/core/Holder;I(?:I)?\)V',
            body,
            re.S,
        ):
            seg = im.group(0)
            insts.append(seg)
        # amplifier: the (Holder,II) ctor shape carries the second int
        # (iconst_0..5 / bipush / sipush / ldc) immediately before the ctor
        effects = []
        for seg in insts:
            effect = re.search(r"MobEffects\.(\w+):", seg).group(1)
            # find the ctor method desc to learn the arg count
            two_arg = "Holder;II)V" in seg
            if two_arg:
                # the amplifier is the int push on the LINE immediately
                # before the invokespecial (the ctor order: duration,
                # amplifier, invoke) — the walk the bytecode lays
                lines = seg.strip().splitlines()
                idx = next(
                    i for i, ln in enumerate(lines)
                    if "invokespecial" in ln and "MobEffectInstance" in ln
                )
                prev = lines[idx - 1]
                pm = (
                    re.search(r"iconst_(\d)\s*$", prev)
                    or re.search(r"bipush\s+(\d+)\s*$", prev)
                    or re.search(r"sipush\s+(\d+)\s*$", prev)
                    or re.search(r"// int (\d+)\s*$", prev)
                )
                amp = int(pm.group(1)) if pm else -1
            else:
                amp = 0
            effects.append((effect.lower(), amp))
        rows.append((base_name, effects))
    return rows


def main():
    colors = parse_effect_colors()
    potions = parse_potions()
    if not colors or not potions:
        print("PARSE FAIL", len(colors), len(potions))
        return 1
    # THE ORDER LAW: the putstatic-derived rows must match the kind names
    # (the 339 round's evidence: the registration order IS the registry id
    # order; the base_name here is the Potion.name FIELD — the strip law).
    src = open(BREWING).read()
    kind_names = re.search(
        r"pub const POTION_KIND_NAMES: \[&str; 46\] = \[(.*?)\];", src, re.S
    ).group(1)
    kind_names = re.findall(r'"(\w+)"', kind_names)
    assert len(potions) == 46, f"expected 46 potions, got {len(potions)}"
    assert len(colors) >= 40, f"expected >= 40 effects, got {len(colors)}"
    # the display base name strip: registry id "long_night_vision" ->
    # the ctor's base "night_vision". The potion_ids order the kind order
    # wears — the parse order the same walk serves.
    for i, (base, effects) in enumerate(potions):
        reg = kind_names[i]
        strip = re.sub(r"^(long|strong)_", "", reg)
        if base != strip:
            print(f"MISMATCH row {i}: registry {reg} -> ctor base {base} (strip {strip})")
            return 1
    print("ORDER LAW: 46 rows, every ctor base = the registry strip")

    # emit the tables
    color_rows = ",\n    ".join(f'("{n}", 0x{c:06X})' for n, c in colors)
    eff_rows = []
    for base, effects in potions:
        rows = ", ".join(f'("{e}", {a})' for e, a in effects)
        eff_rows.append(f"&[{rows}]" if effects else "&[]")
    eff_table = ",\n    ".join(eff_rows)
    base_table = ",\n    ".join(f'"{b}"' for b, _ in potions)

    rust = f'''
// ===== autopush-340 THE POTION-KIND'S OWN TINT + NAME TABLES =====
// THE JAR LAW the 26.2 client.jar's own bytecode banks (scripts/jar_work/
// mob_effects_340.txt + potions_clinit_340.txt, both rc=0): the
// MobEffects bootstrap the register(name, new MobEffect(CATEGORY, COLOR))
// rows the ctor's ldc int wears, the Potions bootstrap the
// register(key, new Potion("base_name", new MobEffectInstance(EFFECT,
// duration[, amplifier])...)) rows the putstatic order IS the registry id
// order the 339 round's own law keeps. GENERATED by
// scripts/gen_potion_kinds_340.py — the tables the parser emits, the
// names the strip law answers.

/// THE MOB EFFECT'S OWN COLOR ROWS (javap 26.2 MobEffects.<clinit> — the
/// `new MobEffect(category, color)` / `new InstantMobEffect(category,
/// color)` ctor args the ldc int wears, the {len(colors)} rows the
/// bootstrap registers). The blend the getColorOptional walk reads.
pub const MOB_EFFECT_COLORS: &[(&str, u32)] = &[
    {color_rows},
];

/// THE KIND'S OWN BASE NAME (javap 26.2 Potions.<clinit> — the
/// `Potion.name` field the ctor's FIRST arg wears; the long_/strong_
/// registry rows the BASE name the same arg answers — pc 95/133 both
/// "night_vision", the 26.2 lang table's own 26-key shape the strip law
/// explains). The display key `item.minecraft.potion.effect.<base>` the
/// getName walk's makeConcat pairs.
pub const POTION_KIND_BASE_NAMES: [&str; 46] = [
    {base_table},
];

/// THE KIND'S OWN EFFECT ROWS ((effect, amplifier) the ctor pairs — the
/// `(Holder, I)` ctor the amplifier-0 default wears, the `(Holder, II)`
/// the explicit second int; the water/mundane/thick/awkward rows the
/// EMPTY slice keeps). The visibility the short ctors' own default
/// answers (ambient false, visible true — every row the blend reads).
pub const POTION_KIND_EFFECTS: [&[(&str, i32)]; 46] = [
    {eff_table},
];

/// THE BLEND'S OWN DEFAULT (javap 26.2 PotionContents.getColor pc 1 —
/// the `getColorOr(-13083194)` ldc the walk wears; -13083194 as u32 =
/// 0xFF385DC6, the RGB the water blue answers). The no-effect and the
/// unknown kinds the default keeps.
pub const POTION_DEFAULT_COLOR: u32 = 0x385DC6;

/// THE KIND'S OWN LIQUID COLOR (javap 26.2 PotionContents.getColorOr
/// pcs 0-35 — the customColor the present arm returns [the registry
/// potions the absent lane wears], the getAllEffects the
/// getColorOptional(Iterable) walk serves: the VISIBLE instances the
/// `(amplifier + 1)`-weighted ARGB channel sums the divisor blends
/// [pc 63-115], the zero-weight the OptionalInt.empty the default
/// arm answers [pc 120-128 + orElse(pc 31-32)]; the
/// `ARGB.color(r/w, g/w, b/w)` the channel walk wears). The client's
/// registry potions carry no custom colors — the kind's effects the
/// whole walk serves, the empty rows the default returns.
pub fn potion_kind_liquid_color(kind: i32) -> u32 {{
    if kind < 0 || kind as usize >= POTION_KIND_NAMES.len() {{
        return POTION_DEFAULT_COLOR;
    }}
    let rows = POTION_KIND_EFFECTS[kind as usize];
    if rows.is_empty() {{
        return POTION_DEFAULT_COLOR;
    }}
    let (mut r, mut g, mut b, mut w) = (0i64, 0i64, 0i64, 0i64);
    for &(effect, amp) in rows {{
        let Some(&(_, color)) =
            MOB_EFFECT_COLORS.iter().find(|(n, _)| *n == effect)
        else {{
            continue; // the registry's own miss the skip keeps
        }};
        let k = (amp + 1) as i64;
        r += k * ((color >> 16) & 0xFF) as i64;
        g += k * ((color >> 8) & 0xFF) as i64;
        b += k * (color & 0xFF) as i64;
        w += k;
    }}
    if w == 0 {{
        return POTION_DEFAULT_COLOR;
    }}
    (((r / w) as u32) << 16) | (((g / w) as u32) << 8) | ((b / w) as u32)
}}

/// THE KIND'S OWN DISPLAY KEY TAIL (the Potion.name field the base name
/// returns — the caller the `item.minecraft.potion.effect.` prefix the
/// lang key pairs). The unknown kind the "empty" tail answers [the
/// getName walk pc 13-18 the orElse("empty") the Uncraftable key wears].
pub fn potion_kind_base_name(kind: i32) -> &'static str {{
    if kind < 0 || kind as usize >= POTION_KIND_BASE_NAMES.len() {{
        return "empty";
    }}
    POTION_KIND_BASE_NAMES[kind as usize]
}}

// ===== end autopush-340 tables =====
'''
    with open(BREWING, "a") as f:
        f.write(rust)
    print(f"APPENDED: {len(colors)} effect colors, 46 base names, 46 effect rows")
    # the spot facts the worklog banks
    water = potions[0]
    print("water rows:", water)
    for i, (name, effects) in enumerate(zip(potions, potions)):
        pass
    for i, (base, effects) in enumerate(potions):
        if base == "turtle_master":
            print(f"row {i} turtle_master effects: {effects}")
        if base == "healing":
            print(f"row {i} healing effects: {effects}")
        if base == "night_vision":
            print(f"row {i} night_vision effects: {effects}")
    print("colors head:", colors[:4])
    nv = next(c for n, c in colors if n == "night_vision")
    ih = next(c for n, c in colors if n == "instant_health")
    print(f"night_vision color 0x{nv:06X}, instant_health 0x{ih:06X}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
