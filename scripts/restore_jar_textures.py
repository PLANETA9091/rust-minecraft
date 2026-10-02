#!/usr/bin/env python3
"""Restore assets-cache/jar textures from the official client jar.

The 26.x client jar is unobfuscated and carries its textures verbatim;
the generate pipeline's extract lanes wrote them into the runtime root
under TWO conventions the loaders read:
  assets-cache/jar/assets/minecraft/textures/...   (colormap + ui extract)
  assets-cache/jar/<sub>                            (skin/uigen bare layout:
                                                     entity/..., gui/...)
The asset index (objects store) covers the sounds/lang/texts lanes —
restore_runtime_cache.py handles those; this one walks the jar.

Usage: python3 scripts/restore_jar_textures.py <client.jar>
"""
import hashlib
import os
import sys
import zipfile

def sha1(b: bytes) -> str:
    return hashlib.sha1(b).hexdigest()

def main():
    jar_path = sys.argv[1]
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    jar_root = os.path.join(root, "assets-cache", "jar")
    extracted = 0
    with zipfile.ZipFile(jar_path) as z:
        for name in z.namelist():
            if not name.startswith("assets/minecraft/") or name.endswith("/"):
                continue
            data = z.read(name)
            # layout 1: the verbatim jar path (the index misses the
            # jar-native files — en_us.json is the vanilla one). THE
            # IF-ABSENT LAW (the standing extract's own): the index
            # restore's copies STAY — the index carries the real
            # panorama faces where the jar ships 1x1 placeholders.
            dest = os.path.join(jar_root, name)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            fresh = not os.path.exists(dest)
            if fresh:
                with open(dest, "wb") as f:
                    f.write(data)
            extracted += 1
            # layout 2: the bare mirror (drop the assets/minecraft/textures
            # prefix) — the skins' entity/... and the uigen's gui/... reads
            if not name.startswith("assets/minecraft/textures/"):
                continue
            sub = name[len("assets/minecraft/textures/"):]
            dest2 = os.path.join(jar_root, sub)
            os.makedirs(os.path.dirname(dest2), exist_ok=True)
            if not os.path.exists(dest2):
                os.link(dest, dest2)
    print(f"extracted {extracted} textures from the jar -> assets-cache/jar")

if __name__ == "__main__":
    main()
