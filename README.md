# mc2rs — Minecraft, rewritten in Rust, generated for **every version**

> `26.2` (latest official release, **unobfuscated era**) is the primary target,
> but the generator is version-agnostic: it reads Mojang's official
> `version_manifest_v2.json` and can scaffold the client for **any version id**
> from the manifest (release, snapshot, pre-release, old beta/alpha).

[![CI](https://github.com/PLANETA9091/rust-minecraft/actions/workflows/ci.yml/badge.svg)](./.github/workflows/ci.yml)
[![Pixel-perfect](https://github.com/PLANETA9091/rust-minecraft/actions/workflows/pixel.yml/badge.svg)](./.github/workflows/pixel.yml)

## What this is

**mc2rs** is a *generator*, not a hand-written port. You feed it a Minecraft
version id, and it produces / refreshes a Rust workspace that re-implements the
client for that exact version:

| Crate | Purpose |
|---|---|
| `mc2rs-gen` | The generator CLI. Fetches manifests, downloads official jars, analyzes them, emits generated code. |
| `mc-registry` | **Generated** from official data: blocks/items registry, the full bitmap font (real glyph metrics from the real font textures), the complete `en_us` language table, all 454 official splashes, the GUI sprite inventory (real sizes + nine-slice borders from `.mcmeta`), the sound-event table (official `sounds.json` via the asset index), biome/worldgen tables, and the block texture atlas (blockstates → models → PNG pixels, first frame, biome tints). |
| `mc-protocol` | Hand-rolled protocol layer: VarInt/VarLong, packet framing (incl. zlib compression), handshake, **server list ping**. |
| `mc-assets` | Asset index parsing + parallel downloader (runtime, from official CDN, sha1-verified). |
| `mc-client` | The client itself: CPU reference renderer (deterministic, pixel-perfect tests), the GUI runtime (boot chain → title → menus) rendering official fonts/sprites/strings, and a windowed fly-camera. |
| `mc2rs-pixel` | Golden-image pixel diff harness (RMS / heatmap) used by CI. |

### The client experience (v0.3)

Everything on screen is driven by **generated official data** — nothing
hand-drawn:

- **Mojang Studios loading screen** — red studio screen with the two-bar boot
  progress (`--screen loading`).
- **Title screen** — wordmark, the real splash text picked from the official
  `texts/splashes.txt` (pulsing, seeded), vanilla-layout button grid using the
  official `widget/button*.png` nine-slice sprites, version + copyright
  overlay (`--screen title`).
- **Server list** — `Play Multiplayer` screen with server entries, `§`-color
  MOTDs, official `server_list/ping_*` icons, Join/Refresh/Cancel
  (`--screen servers`).
- **Options** — `Options` screen with volume/FOV controls (`--screen options`).
- **Music scheduler** — the vanilla menu-music behavior (play `music.menu`
  track → random delay → next) driven by the official `sounds.json` tables.
- **Windowed app** — `--gui` runs the interactive boot chain (mouse + clicks).
- Text rendering uses the **real bitmap font**: glyph metrics measured from
  the official font textures with the vanilla trim rule, `§`-color codes and
  1px shadow, exactly like the original renderer.

## Why a generator

Minecraft changes every release: block states, item components, protocol
numbers, asset indexes. Instead of hand-porting every version, everything
version-specific is **derived from official data**:

```
piston-meta manifest ──► version json ──► client jar ──► analysis ──► codegen
                                                     (obfuscated?     mc-registry/src/generated/*
                                                      unobfuscated?)   mc-client/src/build_info.rs
```

The same pipeline works for `26.2`, `1.21.1`, `26.3-snapshot-10`, whatever —
as long as Mojang publishes the version, the generator can consume it.

## Clean-room policy (important)

* **No Mojang code and no Mojang assets are committed to this repository.**
* The generator downloads official jars from Mojang's own CDN at *generation
  time* and only extracts **functional metadata** (identifiers, state
  property names, asset hashes).
* Art/sound assets are downloaded at *runtime* by the end user from official
  servers and stored in a local cache. You must own the game.
* Protocol layer is written against public community documentation
  (wiki.vg), not against decompiled sources.

## Usage

```bash
# list all versions
cargo run -p mc2rs-gen -- manifest

# details for one version (obfuscation status, downloads, asset index)
cargo run -p mc2rs-gen -- info --version 26.2

# THE main command: generate/refresh the workspace for a version
cargo run -p mc2rs-gen -- generate --version 26.2

# download game assets (textures/sounds) into local cache (requires owning MC)
cargo run -p mc2rs-gen -- assets --version 26.2

# run the client
cargo run -p mc-client --release            # windowed fly-camera
cargo run -p mc-client --release -- --headless --width 640 --height 360 --out shot.png

# pixel diff
cargo run -p mc2rs-pixel -- --a render.png --b goldens/cpu_320x180.png --out diff.png
```

## Performance architecture (the 240 FPS plan)

* Palette-compressed chunk storage (u16 ids, 16×48×16 sections).
* Culled + greedy meshing, parallel across chunks (`rayon`).
* Vanilla-style smooth lighting: per-vertex ambient occlusion (levels 0-3,
  brightness curve `[0.55, 0.70, 0.85, 1.00]`) with the classic quad-diagonal
  flip against crease artifacts; water/leaves never occlude.
* Face-shaded, distance-fogged rendering; per-face color fetched from a
  texture-average LUT (built from the real textures when assets are present).
* CPU reference renderer = *deterministic* (bit-for-bit reproducible frames,
  used by the pixel-perfect CI) — the GPU path (`wgpu`, feature `gpu`) is the
  fast renderer: instanced quads, frustum culling, chunk streaming, no
  per-frame allocations.
* The golden rule: **optimization must never change a single pixel** — which is
  exactly what the CPU-reference + pixel-CI setup enforces.

## CI

* `ci.yml` — build + tests for the whole workspace; **generate for a version
  matrix** (`26.2`, `1.21.1`) and fail on *codegen drift*; headless render;
  CPU client build.
* `pixel.yml` — renders deterministic frames and compares them against
  `goldens/` with `mc2rs-pixel` (RMS threshold + heatmap artifact). Any visual
  change that is not an intentional, reviewed golden update = red build.

## Status / honest scope

This is the **foundation**: real protocol framing + ping, real generated
registries, real asset pipeline, real deterministic renderer, real pixel CI.
Reaching full game parity (mobs, lighting engine, redstone…) is a long-haul
effort by design — but every new system lands on rails: data-driven,
version-agnostic, pixel-tested.
