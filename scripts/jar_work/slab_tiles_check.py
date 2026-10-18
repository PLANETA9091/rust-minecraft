#!/usr/bin/env python3
"""Check which texture stems the 68 slab kinds need vs the current TILE_NAMES."""
import json
import re
import zipfile

JAR = ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

# current TILE_NAMES from generated atlas.rs
src = open("crates/mc-registry/src/generated/atlas.rs").read()
m = re.search(r"pub static TILE_NAMES: \[&str; (\d+)\] = \[(.*?)\];", src, re.S)
tiles = set(re.findall(r'"([^"]+)"', m.group(2)))
print(f"current tiles: {len(tiles)}")

names = sorted(n.split("/")[-1][:-5] for n in z.namelist()
               if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
               and n.split("/")[-1][:-5].endswith("_slab"))
needed = {}
missing = set()
for name in names:
    bs = json.loads(z.read(f"assets/minecraft/blockstates/{name}.json"))
    vs = bs["variants"]
    for k in ("type=bottom", "type=top", "type=double"):
        v = vs[k]
        if isinstance(v, list):
            v = v[0]
        model = v["model"].replace("minecraft:", "")
        # walk the model chain
        seen = 0
        while seen < 10:
            d = json.loads(z.read(f"assets/minecraft/models/{model}.json"))
            for tex, ref in d.get("textures", {}).items():
                if isinstance(ref, str):
                    if ref.startswith("#"):
                        continue  # resolved via parent chain in real bake; skip here
                    stem = ref.replace("minecraft:", "").replace("block/", "")
                    needed.setdefault(stem, set()).add(name)
                    if stem not in tiles:
                        missing.add(stem)
            parent = d.get("parent", "")
            if not parent or parent.startswith("builtin/"):
                break
            model = parent.replace("minecraft:", "")
            seen += 1

print(f"needed stems: {len(needed)}")
print(f"MISSING from TILE_NAMES: {sorted(missing)}")
