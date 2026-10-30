#!/usr/bin/env python3
"""dev-202 — THE ENCHANTMENT ANVIL COST'S OWN COLUMN pin.

The anvil round's combine-cost lane the defs table wears: the jar's own
`data/minecraft/enchantment/*.json` rows the `anvil_cost` int each
speaks (the AnvilMenu.createResult walk's `Enchantment.getAnvilCost`
read the combine loop's cost arm answers — scripts/jar_work/
anvil_walk_202.txt pc 682-711 the `j += anvilCost * newLevel` the walk
banks), the generated ENCHANTMENTS table the column the client never
carried.

The pin (the anvil_pin_202 shape the idempotent sections keep):

  enchantments.rs  the EnchDef struct the `anvil_cost: i32` field the
                   doc row keeps; the 43 rows the `anvil_cost: N` the
                   tail seat wears (the name the row's own key).

Idempotent: every section checks its own anchor first.
"""
import json
import os
import re
import sys
import zipfile

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ENCH = os.path.join(REPO, "crates/mc-registry/src/generated/enchantments.rs")
JAR = os.path.join(REPO, ".mc2rs/classpath/client.jar")

FIELD_DOC = (
    "    /// The definition's `anvil_cost` (the AnvilMenu combine\n"
    "    /// loop's own per-level arm — the dev-202 pin the json\n"
    "    /// rows' own ints the walk banks).\n"
    "    pub anvil_cost: i32,\n"
)
FIELD_ANCHOR = (
    "    /// The `exclusive_set` tag's family name (None = no exclusivity).\n"
    "    pub exclusive: Option<&'static str>,\n"
)
NAME_PAT = re.compile(r'EnchDef \{ name: "([a-z_]+)",')
PINNED_PAT = re.compile(r", anvil_cost: \d+ \},")


def main() -> None:
    z = zipfile.ZipFile(JAR)
    costs = {}
    for n in z.namelist():
        if n.startswith("data/minecraft/enchantment/") and n.endswith(".json"):
            name = n.split("/")[-1][: -len(".json")]
            d = json.loads(z.read(n))
            v = d.get("anvil_cost")
            if v is None:
                sys.exit(f"  [jar row] {name} the anvil_cost the json never speaks")
            costs[name] = int(v)

    ench = open(ENCH).read()

    # 1. the struct field (the doc row the javap law keeps).
    if "pub anvil_cost: i32," in ench:
        print("  [struct field] already pinned")
    else:
        if FIELD_ANCHOR not in ench:
            sys.exit("  [struct field] anchor missing")
        ench = ench.replace(FIELD_ANCHOR, FIELD_DOC + FIELD_ANCHOR, 1)
        print("  [struct field] pinned")

    # 2. the 43 rows (the name the row's own key — the anvil_cost the
    # tail seat wears, the comma the insert's own tail keeps).
    rows = NAME_PAT.findall(ench)
    if len(rows) == 0:
        sys.exit("  [rows] ZERO rows — the pattern the drift never answers")
    missing = [n for n in rows if n not in costs]
    if missing:
        sys.exit(f"  [rows] the costs the names never answer: {missing[:5]}")
    pinned = len(PINNED_PAT.findall(ench))
    if pinned == len(rows):
        print(f"  [rows] already pinned ({len(rows)} row(s))")
    elif pinned == 0:
        for n in rows:
            row_pat = re.compile(
                r'(EnchDef \{ name: "' + n + r'",[^\n]*) \},'
            )
            ench, c = row_pat.subn(
                lambda m: m.group(1) + ", anvil_cost: " + str(costs[n]) + " },", ench, count=1
            )
            if c != 1:
                sys.exit(f"  [rows] {n} the row the sub never answers")
        print(f"  [rows] pinned ({len(rows)} row(s))")
    else:
        sys.exit(f"  [rows] partial pin {pinned}/{len(rows)} — repair by hand")

    open(ENCH, "w").write(ench)
    print("anvil_cost_pin_202: done")


if __name__ == "__main__":
    main()
