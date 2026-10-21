#!/usr/bin/env bash
# restore_env.sh — the wiped-sandbox restore in ONE command.
#
# The environment-recovery lane the container resets demand (the disk the
# reset loses — repo, client jar, assets-cache all gone; the fresh clone
# the bare `cargo test --workspace` then reds on the cache-dependent
# lanes: tint's colormap verdicts, the lang/splash texts, the boot
# screens): this chains the exact sequence the round's fresh-clone
# greenening walked, in the order the loaders' lanes read:
#   1. the official client jar — fetched from piston-data at the
#      version manifest's own URL, sha1-verified against the manifest's
#      own pin (26.2 = 2dc72797...), cached at .mc2rs/classpath/client.jar
#   2. mc2rs-gen cache — the derived assets-cache/gen/font.bin +
#      textures.bin the loaders parse (the generator-DERIVED lane the
#      index never carries)
#   3. restore_runtime_cache.py — the asset index's object store +
#      the jar layout the index covers (sounds/lang/texts + the real
#      panorama faces where the jar ships 1x1 placeholders)
#   4. restore_jar_textures.py — the jar's own textures BOTH layouts:
#      assets/minecraft/... verbatim (the colormap lane's layout-1 walk)
#      + the bare entity/gui mirror — the lane the 26.x index misses.
#
# Idempotent: every step's own if-absent / sha1-verified law keeps the
# re-run honest; the standing cache the re-run never downgrades.
#
# Usage:  ./scripts/restore_env.sh [version]     (default 26.2)
# Needs:  python3, curl, cargo (rustup — the toolchain the repo pins).
set -euo pipefail

cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"

VERSION="${1:-26.2}"
JAR=".mc2rs/classpath/client.jar"

echo "== [1/4] the official client jar ($VERSION) =="
mkdir -p .mc2rs/classpath
python3 - "$VERSION" "$JAR" <<'PY'
import hashlib, json, sys, urllib.request

version_id, jar_path = sys.argv[1], sys.argv[2]

def sha1(b):
    return hashlib.sha1(b).hexdigest()

manifest = json.load(urllib.request.urlopen(
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"))
entry = next(v for v in manifest["versions"] if v["id"] == version_id)
vjson = json.load(urllib.request.urlopen(entry["url"]))
client = vjson["downloads"]["client"]
index_url = vjson["assetIndex"]["url"]

want = client["sha1"]
have = None
try:
    with open(jar_path, "rb") as f:
        have = sha1(f.read())
except FileNotFoundError:
    pass
if have == want:
    print(f"jar cached and sha1-honest: {jar_path} ({want[:12]}...)")
else:
    print(f"fetching {client['url']} ...")
    urllib.request.urlretrieve(client["url"], jar_path)
    with open(jar_path, "rb") as f:
        got = sha1(f.read())
    if got != want:
        raise SystemExit(f"sha1 mismatch: got {got}, manifest pins {want}")
    print(f"jar fetched, sha1 verified: {want[:12]}... ({client['size']} bytes)")

with open(".mc2rs/asset_index_url.txt", "w") as f:
    f.write(index_url)
PY

echo "== [2/4] mc2rs-gen cache (font.bin + textures.bin) =="
if [ ! -x target/debug/mc2rs-gen ]; then
    cargo build -p mc2rs-gen
fi
./target/debug/mc2rs-gen cache --version "$VERSION" --jar "$JAR"

echo "== [3/4] the asset index objects -> assets-cache/jar =="
python3 scripts/restore_runtime_cache.py "$(cat .mc2rs/asset_index_url.txt)" "$JAR"

echo "== [4/4] the jar's own textures -> BOTH layouts =="
python3 scripts/restore_jar_textures.py "$JAR"

echo "restore_env: done — cargo test --workspace is the greenening verdict."
