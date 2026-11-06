#!/usr/bin/env python3
"""loot_507 — the fishing loot's own JSON the jar's own bytes speak
(the autopush-483 round, the JRE-free walk the data tables wear):
the gameplay/fishing.json the MAIN band the weight walk stores [the
junk 10 / the treasure 5 the quality pairs wear, the open-water
condition the treasure's own gate keeps, the fish 85 the default row],
the fishing/fish.json the pool the cod 60 / salmon 25 / pufferfish 13
/ tropical_fish 2 rolls, the fishing/junk.json + treasure.json the
pools the port's NEXT round the honest silence retired — the entries'
weight/quality/functions the verbatim bytes + the parsed walk both
bank. Usage: python3 scripts/jar_work_parse_loot_507.py >
scripts/jar_work/loot_507.txt"""
import json
import sys
import zipfile

JAR = ".mc2rs/classpath/client.jar"
TABLES = [
    "data/minecraft/loot_table/gameplay/fishing.json",
    "data/minecraft/loot_table/gameplay/fishing/fish.json",
    "data/minecraft/loot_table/gameplay/fishing/junk.json",
    "data/minecraft/loot_table/gameplay/fishing/treasure.json",
]


def walk_entry(path, e, out):
    """the one entry the weight/quality/condition/function rows the walk banks"""
    kind = e.get("type", "?")
    out.append(f"  entry type={kind}")
    if "weight" in e:
        out.append(f"    weight={e['weight']}")
    if "quality" in e:
        out.append(f"    quality={e['quality']}")
    if "name" in e:
        out.append(f"    name={e['name']}")
    for c in e.get("conditions", []):
        out.append(f"    condition={c.get('condition','?')}")
        for k, v in sorted(c.items()):
            if k != "condition":
                out.append(f"      {k}={json.dumps(v, sort_keys=True)}")
    for f in e.get("functions", []):
        out.append(f"    function={f.get('function','?')}")
        for k, v in sorted(f.items()):
            if k != "function":
                out.append(f"      {k}={json.dumps(v, sort_keys=True)}")
    for child in e.get("children", []):
        walk_entry(path + "/child", child, out)


def main():
    z = zipfile.ZipFile(JAR)
    print("=== loot_507: the fishing loot tables' own bytes ===")
    for name in TABLES:
        raw = z.read(name)
        print(f"\n### {name} ({len(raw)} bytes)")
        print("--- verbatim ---")
        print(raw.decode("utf-8"))
        print("--- parsed walk ---")
        doc = json.loads(raw)
        pool = doc["pools"][0]
        print(f"pool rolls={pool.get('rolls')} bonus_rolls={pool.get('bonus_rolls')}")
        for e in pool["entries"]:
            walk_entry(name, e, out := [])
            print("\n".join(out))


if __name__ == "__main__":
    sys.exit(main())
