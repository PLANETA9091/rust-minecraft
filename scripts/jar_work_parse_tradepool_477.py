#!/usr/bin/env python3
"""tradepool_477 — the wanderer's own OFFER POOLS the jar's data files
speak (the trade_set + the villager_trade jsons the 26.2 jar carries):
the three sets the updateTrades walk answers [buying 2.0, uncommon 2.0,
common 5.0 — the allow_duplicates the codec's iconst_0 default keeps],
the tag's own entry order the pools bank, the codec defaults the absent
words wear [max_uses 4.0 the ldc #250 speaks, reputation_discount 0.0
the fconst_0 keeps, xp 1.0 the fconst_1 answers, count 1.0 both sides].
Usage: python3 scripts/jar_work_parse_tradepool_477.py >
scripts/jar_work/tradepool_477.txt (the census) —
--rust emits the Rust static tables the port wears."""
import json
import sys
import zipfile

JAR = ".mc2rs/classpath/client.jar"
SETS = [
    ("buying", 2),
    ("uncommon", 2),
    ("common", 5),
]


def load():
    z = zipfile.ZipFile(JAR)
    pools = {}
    for tag, count in SETS:
        d = json.loads(z.read(f"data/minecraft/trade_set/wandering_trader/{tag}.json"))
        values = json.loads(z.read(
            f"data/minecraft/tags/villager_trade/wandering_trader/{tag}.json"))["values"]
        rows = []
        for v in values:
            e = json.loads(z.read(f"data/minecraft/villager_trade/wandering_trader/"
                                  f"{v.split('/')[-1]}.json"))
            wants = e["wants"]
            gives = e["gives"]
            rows.append((
                wants["id"].replace("minecraft:", ""),
                int(wants.get("count", 1.0)),
                gives["id"].replace("minecraft:", ""),
                int(gives.get("count", 1.0)),
                int(e.get("max_uses", 4.0)),
                int(e.get("xp", 1.0)),
                float(e.get("reputation_discount", 0.0)),
            ))
        pools[tag] = {
            "amount": d["amount"],
            "random_sequence": d.get("random_sequence", ""),
            "allow_duplicates": d.get("allow_duplicates", False),
            "rows": rows,
        }
    return pools


def main():
    pools = load()
    if "--rust" in sys.argv:
        for tag, _count in SETS:
            p = pools[tag]
            name = "WANDER_" + tag.upper() + "_POOL"
            print(f"/// THE WANDERER'S OWN {tag.upper()} POOL (the "
                  f"trade_set/wandering_trader/{tag} the jar's data "
                  f"speaks — amount {p['amount']}, the allow_duplicates "
                  f"the codec's iconst_0 default keeps, the tag's own "
                  f"order the census banks): the (wants, wants_count, "
                  f"gives, gives_count, max_uses, xp) rows the "
                  f"villager_trade jsons wear, the codec defaults the "
                  f"absent words wear (max_uses 4, xp 1, count 1).")
            print(f"pub static {name}: [(&str, i32, &str, i32, i32, i32); "
                  f"{len(p['rows'])}] = [")
            for (w, wc, g, gc, mu, xp, _disc) in p["rows"]:
                print(f'    ("{w}", {wc}, "{g}", {gc}, {mu}, {xp}),')
            print("];")
            print()
        return
    for tag, count in SETS:
        p = pools[tag]
        print(f"=== trade_set/wandering_trader/{tag}.json — amount "
              f"{p['amount']}, allow_duplicates {p['allow_duplicates']}, "
              f"random_sequence {p['random_sequence']}")
        for i, (w, wc, g, gc, mu, xp, disc) in enumerate(p["rows"]):
            print(f"  [{i:2d}] wants {w} x{wc} -> gives {g} x{gc} "
                  f"max_uses {mu} xp {xp} discount {disc}")


if __name__ == "__main__":
    main()
