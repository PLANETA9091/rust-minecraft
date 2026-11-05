#!/usr/bin/env python3
"""traded2_471 — the traded stat's own display word the jar's lang
speaks: the en_us.json the `stat.minecraft.traded_with_villager` key
answers, the traded words the whole lang wears, the display name the
General tab's row wears. Usage: python3
scripts/jar_work_parse_traded2_471.py > scripts/jar_work/traded2_471.txt"""
import json
import sys
import zipfile

JAR = ".mc2rs/classpath/client.jar"
LANG = "assets/minecraft/lang/en_us.json"


def main():
    with zipfile.ZipFile(JAR) as z:
        raw = z.read(LANG).decode("utf-8")
    lang = json.loads(raw)
    hits = {k: v for k, v in lang.items() if "traded" in k.lower()}
    if not hits:
        print("== no traded keys in", LANG)
        return 1
    print(f"== {LANG}: {len(hits)} traded keys")
    for k in sorted(hits):
        print(f"   {k} = {hits[k]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
