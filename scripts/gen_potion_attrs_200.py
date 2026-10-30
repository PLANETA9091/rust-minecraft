#!/usr/bin/env python3
"""THE ATTRIBUTE ROWS' OWN GENERATOR (dev-200 — the potion-kind's own
attribute-rows round). Parses the 26.2 jar's javap banks:

  scripts/jar_work/mob_effects_200.txt   — the MobEffects bootstrap: the
      addAttributeModifier(attr, id, amount, operation) calls the
      (effect, [(attr, op, amount)]) rows wear;
  scripts/jar_work/attributes_200.txt    — the Attributes bootstrap: the
      `attribute.name.<short>` description-id strings the ctor wears;
  scripts/jar_work/en_us_200.json        — the lang table's own rows the
      display names the plus/take templates the whenDrank header serve.

Emits the Rust tables into crates/mc-client/src/brewing_sim.rs
APPEND-ONLY (the file's own tail, after the end-341 marker), and prints
the shape facts the worklog banks. Never edits existing lines.
"""
import json
import re

ROOT = "/home/z/my-project/rust-minecraft"
EFFECTS_TXT = f"{ROOT}/scripts/jar_work/mob_effects_200.txt"
ATTRS_TXT = f"{ROOT}/scripts/jar_work/attributes_200.txt"
LANG_JSON = f"{ROOT}/scripts/jar_work/en_us_200.json"
BREWING = f"{ROOT}/crates/mc-client/src/brewing_sim.rs"

# javap renders the float-promoted doubles (the ldc2_w rows) — the
# float->double widening the JVM's own ldc serves. dconst_1 rows the
# 1.0 amount the pc pairing answers.


def parse_attr_registrations():
    """(effect, attr_field, amount_str, op) rows from the MobEffects
    bootstrap clinit, in file order. The String ldc rows wear BOTH
    shapes (the register name "speed" AND the modifier id "effect.
    speed") — the owner the most recent NON-"effect." string the walk
    answers; the attr/amount/op the trailing state the same walk keeps.
    """
    txt = open(EFFECTS_TXT).read()
    clinit = txt[txt.rindex("  static {};") :]
    out = []
    pending = None
    cur_attr = None
    cur_amt = None
    cur_op = None
    for L in clinit.splitlines():
        s = L.strip()
        m = re.match(r"\d+: (?:ldc|ldc_w)\s+#\S+\s+// String (\S+)", s)
        if m:
            if not m.group(1).startswith("effect."):
                pending = m.group(1)
            continue
        m = re.match(r"\d+: getstatic\s+#\d+\s+// Field net/minecraft/world/entity/ai/attributes/Attributes\.(\w+)",
            s,
        )
        if m:
            cur_attr = m.group(1)
            continue
        m = re.match(r"\d+: ldc2_w\s+#\d+\s+// double (\S+?)d?$", s)
        if m:
            cur_amt = m.group(1)
            continue
        if re.match(r"\d+: dconst_1\b", s):
            cur_amt = "1.0"
            continue
        m = re.match(
            r"\d+: getstatic\s+#\d+\s+// Field net/minecraft/world/entity/ai/attributes/AttributeModifier\$Operation\.(\w+)",
            s,
        )
        if m:
            cur_op = m.group(1)
            continue
        if "addAttributeModifier" in s:
            assert pending is not None and cur_attr is not None and cur_amt is not None and cur_op is not None, (
                f"the incomplete registration {pending} {cur_attr} {cur_amt} {cur_op}"
            )
            out.append((pending, cur_attr, cur_amt, cur_op))
    return out


def parse_attr_descriptions():
    """attr field -> short name (the description id's own tail) from the
    Attributes bootstrap: the `attribute.name.<short>` ldc strings the
    RangedAttribute ctor wears, the putstatic field the pairing answers."""
    txt = open(ATTRS_TXT).read()
    clinit = txt[txt.rindex("  static {};") :]
    pairs = []
    cur_field = None
    for L in clinit.splitlines():
        s = L.strip()
        m = re.match(r"\d+: ldc(?:_w)?\s+#\d+\s+// String attribute\.name\.([a-z_]+)$", s)
        if m:
            cur_desc = m.group(1)
            continue
        m = re.match(r"\d+: putstatic\s+#\d+\s+// Field ([A-Z_]+):Lnet/minecraft/core/Holder;", s)
        if m:
            pairs.append((m.group(1), cur_desc))
            cur_field = None
    seen = {}
    for field, desc in pairs:
        seen.setdefault(field, desc)
    return seen


WANT_FIELDS = [
    "MOVEMENT_SPEED",
    "ATTACK_SPEED",
    "ATTACK_DAMAGE",
    "SAFE_FALL_DISTANCE",
    "WAYPOINT_TRANSMIT_RANGE",
    "MAX_HEALTH",
    "MAX_ABSORPTION",
    "LUCK",
]


def main():
    regs = parse_attr_registrations()
    descs = parse_attr_descriptions()
    lang = json.load(open(LANG_JSON))
    print(f"registrations parsed: {len(regs)}")
    for r in regs:
        print("  ", r)
    # THE FIELD->SHORT MAP the fields the registrations wear
    field_short = {f: descs[f] for f in WANT_FIELDS}
    assert len(field_short) == len(WANT_FIELDS), f"the fields the parse misses: {set(WANT_FIELDS) - set(field_short)}"
    # THE DISPLAY NAMES the lang rows wear
    display = {}
    for f in WANT_FIELDS:
        short = field_short[f]
        key = f"attribute.name.{short}"
        assert key in lang, f"the lang key the table misses: {key}"
        display[short] = lang[key]
    # THE GROUP the rows the effect the (short, op, amount) triples wear
    OP_ID = {"ADD_VALUE": 0, "ADD_MULTIPLIED_BASE": 1, "ADD_MULTIPLIED_TOTAL": 2}
    grouped = {}
    for owner, attr_field, amt, op in regs:
        short = field_short[attr_field]
        grouped.setdefault(owner, []).append((short, OP_ID[op], amt))
    print(f"effects with modifiers: {len(grouped)}")
    for k, v in grouped.items():
        print("  ", k, "->", v)
    # THE LANG TEMPLATES the rows' own keys the cross-verify wears
    for k in ("potion.whenDrank", "attribute.modifier.plus.0", "attribute.modifier.take.2"):
        assert k in lang, k
    # THE EMIT (append-only after the end marker)
    txt = open(BREWING).read()
    marker = "// ===== end autopush-341 tables ====="
    assert marker in txt, "the 341 marker the append point misses"
    lines = []
    lines.append("")
    lines.append("// ===== dev-200 tables (scripts/gen_potion_attrs_200.py appends —")
    lines.append("// never edits existing lines) =====")
    lines.append("")
    lines.append("/// THE EFFECT'S OWN ATTRIBUTE-MODIFIER ROWS (javap 26.2 MobEffects.")
    lines.append("/// <clinit> — the `addAttributeModifier(attr, id, amount, operation)`")
    lines.append("/// calls the (effect, [(attr short, op id, amount)]) rows wear; the")
    lines.append("/// amount the ldc2_w double the float-promoted value serves [the")
    lines.append("/// (double)(float) widening the JVM's ldc wears], the dconst_1 rows")
    lines.append("/// the 1.0 the pc pairing answers. The display the Template.create")
    lines.append("/// walk the amount*(amp+1) law scales [MobEffect$AttributeTemplate")
    lines.append("/// create pcs 8-17], the 0-amount rows the walk's own silence keeps.")
    lines.append("pub const MOB_EFFECT_ATTRIBUTE_MODIFIERS: &[(&str, &[(&str, i32, f64)])] = &[")
    for owner, mods in grouped.items():
        inner = ", ".join(f'("{s}", {o}, {a})' for s, o, a in mods)
        lines.append(f'    ("{owner}", &[{inner}]),')
    lines.append("];")
    lines.append("")
    lines.append("/// THE ATTRIBUTE'S OWN DISPLAY NAMES (javap 26.2 Attributes.<clinit>")
    lines.append("/// — the `attribute.name.<short>` description-id strings the")
    lines.append("/// RangedAttribute ctor wears, the en_us table's own rows the")
    lines.append("/// display answers; the generic.* aliases the same 8 the table")
    lines.append("/// carries, the getDescriptionId the short keys serve).")
    lines.append("pub const ATTRIBUTE_NAMES: &[(&str, &str)] = &[")
    for f in WANT_FIELDS:
        short = field_short[f]
        lines.append(f'    ("{short}", "{display[short]}"),')
    lines.append("];")
    lines.append("")
    block = "\n".join(lines)
    txt = txt.replace(marker, marker + "\n" + block, 1)
    open(BREWING, "w").write(txt)
    print(f"emitted: {len(grouped)} modifier rows + {len(WANT_FIELDS)} name rows -> brewing_sim.rs")
    # THE DISPLAY SPOTS the worklog banks (the #.## fold the ×100 lane)
    def fmt(v):
        s = f"{v:.2f}"
        if s.endswith("00"):
            s = s[:-3]
        elif s.endswith("0"):
            s = s[:-1]
        return s

    for owner, mods in sorted(grouped.items()):
        for short, op, amt in mods:
            a = float(amt)
            scaled = a * 100.0 if op in (1, 2) else a
            sign = "+" if a > 0 else "-"
            tpl = lang[f"attribute.modifier.{'plus' if a > 0 else 'take'}.{op}"]
            row = tpl.replace("%s", fmt(abs(scaled)), 1).replace("%s", display[short], 1)
            print(f"  spot {owner}: [{sign}] {row!r} (base {amt} op {op})")


if __name__ == "__main__":
    main()
