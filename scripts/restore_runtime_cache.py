#!/usr/bin/env python3
"""Restore assets-cache/jar runtime layout from the content-addressed
object store (assets-cache/objects) + the official asset index.

The sandbox wipes demand an environment-recovery lane: the object store
is the downloader's sha1-verified truth, the jar layout the client's
loaders read is TWO conventions (the generate pipeline's own writes):
  assets-cache/jar/assets/minecraft/...   (sounds, textures, lang, texts)
  assets-cache/jar/<rest>                 (bare entity/ gui/ skin layout)
This script hardlinks every index object into both shapes it needs,
plus extracts jar-native files (sounds.json) the index does not carry.

Usage: python3 scripts/restore_runtime_cache.py <asset-index-url> <jar>
"""
import hashlib
import json
import os
import sys
import urllib.request
import zipfile

def sha1(b: bytes) -> str:
    return hashlib.sha1(b).hexdigest()

def main():
    index_url, jar_path = sys.argv[1], sys.argv[2]
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    cache = os.path.join(root, "assets-cache")
    objects = os.path.join(cache, "objects")
    jar_root = os.path.join(cache, "jar")

    idx = json.load(urllib.request.urlopen(index_url))
    objs = idx["objects"]
    print(f"index: {len(objs)} objects")

    def object_path(h):
        return os.path.join(objects, h[:2], h)

    def load_object(key, meta):
        h = meta["hash"]
        src = object_path(h)
        # verify the store's copy (the parallel downloader's own sha1 law
        # — a stale/partial entry re-fetches on the spot)
        if os.path.isfile(src):
            with open(src, "rb") as f:
                data = f.read()
            if len(data) == meta.get("size", -1) and sha1(data) == h:
                return src
        url = f"https://resources.download.minecraft.net/{h[:2]}/{h}"
        data = urllib.request.urlopen(url).read()
        if sha1(data) != h:
            raise SystemExit(f"sha1 mismatch {key}")
        os.makedirs(os.path.dirname(src), exist_ok=True)
        with open(src, "wb") as f:
            f.write(data)
        return src

    n_assets = n_bare = 0
    for key, meta in objs.items():
        h = meta["hash"]
        src = load_object(key, meta)
        if not key.startswith("minecraft/"):
            continue  # the launcher's own icons — no client lane reads them
        rest = key[len("minecraft/"):]
        # layout 1: assets/minecraft/<rest>
        dest = os.path.join(jar_root, "assets", "minecraft", rest)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        if not os.path.exists(dest):
            os.link(src, dest)
        n_assets += 1
        # layout 2: the bare textures mirror (entity/, gui/, ...)
        if rest.startswith("textures/"):
            dest2 = os.path.join(jar_root, rest)
            os.makedirs(os.path.dirname(dest2), exist_ok=True)
            if not os.path.exists(dest2):
                os.link(src, dest2)
            # layout 3: the loader's own bare lane — the index key minus
            # the leading `textures/` (the generated SpriteDef files ride
            # `gui/sprites/...`, the FlashSprite rides `particle/flash.png`,
            # the legacy inventory background rides
            # `gui/container/inventory.png` — all root-relative walks the
            # `textures/` prefix never carried)
            dest3 = os.path.join(jar_root, rest[len("textures/"):])
            os.makedirs(os.path.dirname(dest3), exist_ok=True)
            if not os.path.exists(dest3):
                os.link(src, dest3)
            n_bare += 1

    # the jar-native files the loaders read verbatim (the index carries
    # sounds.json too in 26.x, but the jar copy is the generator's own
    # source — keep both honest)
    # PLUS the jar-native TEXTURES the 26.x asset index no longer ships
    # (particle/flash.png, gui/container/inventory.png, the armor
    # layers, ...): the loaders' bare lanes read them root-relative, so
    # extract every `assets/minecraft/textures/<rest>` jar entry into
    # `jar/<rest>` (the bare law, only-if-absent — the generated table's
    # own sha1 pins keep the copies honest).
    with zipfile.ZipFile(jar_path) as z:
        for name in (
            "assets/minecraft/sounds.json",
            # the 26.x asset index no longer ships the en_us table — the
            # jar copy the lang lane's parse walk reads (the wiped-sandbox
            # restore the death/give/chat lanes' translated rows demand)
            "assets/minecraft/lang/en_us.json",
            # the 26.x texts/ lane — the splash/boot texts the title
            # screen's own golden the splash picker reads
            "assets/minecraft/texts/splashes.txt",
            "assets/minecraft/texts/end.txt",
            "assets/minecraft/texts/postcredits.txt",
            "assets/minecraft/texts/credits.json",
        ):
            try:
                data = z.read(name)
            except KeyError:
                continue
            dest = os.path.join(jar_root, name)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            if not os.path.exists(dest):
                with open(dest, "wb") as f:
                    f.write(data)
        n_jar_tex = 0
        prefix = "assets/minecraft/textures/"
        for name in z.namelist():
            if not name.startswith(prefix) or not name.endswith(".png"):
                continue
            rest = name[len(prefix):]
            dest = os.path.join(jar_root, rest)
            if os.path.exists(dest):
                continue
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            with z.open(name) as src_f, open(dest, "wb") as out_f:
                out_f.write(src_f.read())
            n_jar_tex += 1
        print(f"extracted: {n_jar_tex} jar-native textures -> bare layout")

    print(f"linked: {n_assets} assets paths, {n_bare} bare-mirror paths -> assets-cache/jar")

if __name__ == "__main__":
    main()
