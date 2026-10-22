#!/usr/bin/env python3
"""recover_runtime_cache.py — the one-command environment recovery.

The sandbox disks reset between rounds (the 2026-10-22 wipe ate cargo,
.mc2rs/, assets-cache/ and the unpushed lane with them). The repo itself
survives on GitHub; the RUNTIME CACHE does not — yet every GUI/sound/skin
test and every golden render reads it. This script rebuilds it from the
official 26.2 jar + Mojang's public asset store, mirroring exactly what
the `generate` pipeline's extraction steps laid down:

  1. the client jar at .mc2rs/classpath/client.jar (sha1-verified,
      fetched from the version manifest when absent);
  2. the jar's assets/ tree extracted under assets-cache/jar/;
  3. the FLAT texture layout the loaders read from the cache root
      (gui/, entity/, environment/, font/, block/, item/, misc/,
      particle/ — font/ MERGES with the jar's font/*.json definitions);
  4. the REAL title-screen panorama pixels (the jar's own are 1x1
      placeholders — the store's objects carry the renderable ones);
  5. sounds.json + every .ogg the generator's wanted-filter names
      (music.menu/block./* + the EXTRA/MOB/PLAYER/INTERACTION/AMBIENT
      constants + the generated biome_music events), sha1-verified;
  6. a printout of the final `mc2rs-gen cache` line (font.bin +
      textures.bin — no network, no java).

Re-running is idempotent: present files are skipped (the standing
extract's only-if-absent law).

Usage: python3 scripts/recover_runtime_cache.py [--version 26.2]
"""
import argparse
import hashlib
import json
import os
import re
import shutil
import struct
import sys
import urllib.request
import zipfile

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CACHE = os.path.join(REPO, "assets-cache")
JAR_ROOT = os.path.join(CACHE, "jar")
CLASSPATH = os.path.join(REPO, ".mc2rs", "classpath")

# The flat prefixes the loaders read from the cache root (the census the
# recovery round ran: gui/entity/environment joins + the skin RUNTIME_PATH
# table's block/entity/item/misc/particle + the font.bin texture list).
FLAT_PREFIXES = ["gui", "entity", "environment", "font", "block", "item", "misc", "particle"]

# The generator's event constants (main.rs) — kept literally; soundgen.rs's
# MOB_VOICE_EVENTS and the worldgen.rs biome_music rows are parsed live.
CONST_EVENTS = set("""
ui.button.click music.game
entity.generic.explode entity.tnt.primed
entity.player.swim entity.player.splash entity.player.splash.high_speed
entity.generic.burn entity.generic.extinguish_fire entity.player.hurt_on_fire
entity.player.hurt_drown ui.hud.bubble_pop entity.experience_orb.pickup
entity.player.attack.crit entity.player.attack.strong entity.player.attack.sweep
entity.player.attack.weak entity.player.hurt entity.player.death
entity.player.big_fall entity.player.small_fall
entity.generic.eat entity.generic.drink item.honey_bottle.drink
entity.player.burp entity.player.levelup entity.item.pickup
entity.cow.milk entity.pig.saddle entity.boat.paddle_land entity.boat.paddle_water
item.shovel.flatten item.hoe.till item.crop.plant
item.golden_dandelion.use item.golden_dandelion.unuse
ambient.cave
""".split())


def fetch(url: str) -> bytes:
    last = None
    for attempt in range(3):
        try:
            with urllib.request.urlopen(url, timeout=90) as r:
                return r.read()
        except Exception as e:  # noqa: BLE001
            last = e
            print(f"  retry {attempt + 1} {url.split('/')[2]}: {e}", file=sys.stderr)
    raise RuntimeError(f"fetch failed: {url}: {last}")


def fetch_verified(index: dict, key: str, dest: str) -> bool:
    obj = index.get(key)
    if obj is None:
        return False
    dest = os.path.join(JAR_ROOT, dest)
    if os.path.isfile(dest) and os.path.getsize(dest) > 0:
        return False  # the only-if-absent law
    data = fetch(f"https://resources.download.minecraft.net/{obj['hash'][:2]}/{obj['hash']}")
    if hashlib.sha1(data).hexdigest() != obj["hash"]:
        raise RuntimeError(f"sha1 mismatch {key}")
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    with open(dest, "wb") as f:
        f.write(data)
    return True


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="26.2")
    args = ap.parse_args()
    version = args.version

    # 1. the client jar (sha1 the manifest pins).
    jar = os.path.join(CLASSPATH, "client.jar")
    if os.path.isfile(jar) and os.path.getsize(jar) > 1_000_000:
        print(f"[1/6] jar present: {jar} ({os.path.getsize(jar)} B)")
    else:
        man = json.loads(fetch("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"))
        entry = next(v for v in man["versions"] if v["id"] == version)
        vjson = json.loads(fetch(entry["url"]))
        d = vjson["downloads"]["client"]
        os.makedirs(CLASSPATH, exist_ok=True)
        data = fetch(d["url"])
        assert hashlib.sha1(data).hexdigest() == d["sha1"], "client.jar sha1"
        with open(jar, "wb") as f:
            f.write(data)
        print(f"[1/6] jar fetched: {d['sha1']} ({len(data)} B)")

    # 2. the jar's assets tree (the standing extract).
    extracted = 0
    with zipfile.ZipFile(jar) as z:
        for info in z.infolist():
            name = info.filename
            if not name.startswith("assets/") or name.endswith("/"):
                continue
            dest = os.path.join(JAR_ROOT, name)
            if os.path.isfile(dest) and os.path.getsize(dest) == info.file_size:
                continue
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            with z.open(info) as src, open(dest, "wb") as out:
                shutil.copyfileobj(src, out)
            extracted += 1
    print(f"[2/6] jar assets extracted/refreshed: {extracted} files -> {JAR_ROOT}")

    # 3. the flat texture layout (font/ merges: jar defs + texture pngs).
    textures = os.path.join(JAR_ROOT, "assets/minecraft/textures")
    copied = 0
    for prefix in FLAT_PREFIXES:
        src = os.path.join(textures, prefix)
        dst = os.path.join(JAR_ROOT, prefix)
        if not os.path.isdir(src):
            continue
        for root, _dirs, files in os.walk(src):
            rel = os.path.relpath(root, src)
            target = os.path.join(dst, rel) if rel != "." else dst
            os.makedirs(target, exist_ok=True)
            for fn in files:
                f_src, f_dst = os.path.join(root, fn), os.path.join(target, fn)
                if not os.path.isfile(f_dst):
                    shutil.copyfile(f_src, f_dst)
                    copied += 1
    print(f"[3/6] flat layout copied: {copied} files under {{{', '.join(FLAT_PREFIXES)}}}")

    # 4+5. the store objects: index once, panorama + sounds.json + oggs.
    vjson = None
    man = json.loads(fetch("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"))
    entry = next(v for v in man["versions"] if v["id"] == version)
    vjson = json.loads(fetch(entry["url"]))
    aidx = json.loads(fetch(vjson["assetIndex"]["url"]))["objects"]
    pano = 0
    for key, _obj in aidx.items():
        if key.startswith("minecraft/textures/gui/title/background/panorama"):
            rel = key[len("minecraft/textures/"):]
            if fetch_verified(aidx, key, rel):
                pano += 1
    print(f"[4/6] real panorama pixels fetched: {pano}")

    sj = os.path.join(JAR_ROOT, "assets/minecraft/sounds.json")
    if not os.path.isfile(sj):
        if not fetch_verified(aidx, "minecraft/sounds.json", "assets/minecraft/sounds.json"):
            raise RuntimeError("sounds.json missing from the asset index")
    sounds = json.load(open(sj))
    print(f"[5/6] sounds.json: {len(sounds)} events")

    # the wanted filter (soundgen's mob rows + worldgen's biome_music rows).
    sg = open(os.path.join(REPO, "crates/mc2rs-gen/src/soundgen.rs")).read()
    m = re.search(r"MOB_VOICE_EVENTS[^=]*=\s*\[(.*?)\];", sg, re.S)
    mob = set(re.findall(r'"([a-z_.0-9]+)"', m.group(1)))
    wg = open(os.path.join(REPO, "crates/mc-registry/src/generated/worldgen.rs")).read()
    biome = set(re.findall(r'event:\s*"([a-z_.0-9]+)"', wg))

    def wanted(ev: str) -> bool:
        return (
            ev.startswith("music.menu")
            or ev.startswith("block.")
            or ev in CONST_EVENTS
            or ev in mob
            or ev in biome
        )

    files = {}
    for ev_id, ev in sounds.items():
        if not wanted(ev_id):
            continue
        rows = ev.get("sounds", []) if isinstance(ev, dict) else ev
        for row in rows:
            name = row if isinstance(row, str) else row.get("name", "")
            if name:
                files.setdefault(name if name.endswith(".ogg") else f"{name}.ogg", ev_id)
    ok = miss = 0
    for name in sorted(files):
        if fetch_verified(aidx, f"minecraft/sounds/{name}", f"assets/minecraft/sounds/{name}"):
            ok += 1
        elif not os.path.isfile(os.path.join(JAR_ROOT, "assets/minecraft/sounds", name)):
            miss += 1
    print(f"      wanted oggs: {len(files)} distinct, fetched {ok}, not-in-index {miss}")

    # 6. the derived gen cache (font.bin + textures.bin).
    font_bin = os.path.join(CACHE, "gen/font.bin")
    tex_bin = os.path.join(CACHE, "gen/textures.bin")
    if os.path.isfile(font_bin) and os.path.isfile(tex_bin):
        n = struct.unpack("<I", open(font_bin, "rb").read()[10:14])[0]
        print(f"[6/6] gen cache present: font.bin ({n} textures) + textures.bin")
    else:
        print("[6/6] NOW run:  cargo run --release -p mc2rs-gen -- cache "
              f"--version {version} --jar .mc2rs/classpath/client.jar")
    print("recover_runtime_cache: done")


if __name__ == "__main__":
    main()
