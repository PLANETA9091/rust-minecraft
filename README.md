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
| `mc-protocol` | Hand-rolled protocol layer: VarInt/VarLong, packet framing (incl. zlib compression), handshake, **server list ping**, **offline-mode login + the configuration phase** (client information, brand, keep-alive/ping, finish-configuration ack — reaches the play state). |
| `mc-assets` | Asset index parsing + parallel downloader (runtime, from official CDN, sha1-verified). |
| `mc-client` | The client itself: CPU reference renderer (deterministic, pixel-perfect tests), the GUI runtime (boot chain → title → menus) rendering official fonts/sprites/strings, and a windowed fly-camera. |
| `mc2rs-pixel` | Golden-image pixel diff harness (RMS / heatmap) used by CI. |

### The client experience (v0.3)

Everything on screen is driven by **generated official data** — nothing
hand-drawn:

- **Mojang Studios loading screen** — the real `title/mojangstudios.png`
  wordmark (512×512 at quarter scale) on the studio red screen with the
  two-bar boot progress (`--screen loading`).
- **Title screen** — the **real title panorama** (the six official
  1024×1024 cube faces streamed from the asset index — the jar only ships
  1×1 placeholders — sliding like the vanilla rotating cube), the **real
  `title/minecraft.png` logo** at vanilla LogoRenderer proportions, the real
  splash text picked from the official `texts/splashes.txt` (pulsing,
  seeded), vanilla-layout button grid using the official
  `widget/button*.png` nine-slice sprites, version + copyright
  overlay (`--screen title`).
- **Server list** — `Play Multiplayer` screen with server entries, `§`-color
  MOTDs, official `server_list/ping_*` icons, Join/Refresh/Cancel
  (`--screen servers`). **Refresh runs the real protocol**: a vanilla status
  ping (`mc-protocol`) against every hosted row, off the frame thread —
  MOTD, version and latency bars update from the server's actual response
  (unreachable rows render the vanilla *Failed to connect* offline state).
  **Join runs the real protocol too**: the vanilla ConnectScreen flow — the
  offline-mode login (handshake, Login Start with the official offline
  profile UUID, Set Compression, Login Success/Disconnect) fires on a worker
  thread against the selected row, and the outcome lands on the vanilla
  *Connection Lost* screen with the server's own reason (or the factual
  connected line for offline-mode servers that accept the join).
- **Singleplayer world** — clicking `Singleplayer` enters a live 3D world
  view rendered by the same deterministic CPU raster path as the pixel
  tests: real block textures from the generated atlas, the **official
  day-night sky** and the **vanilla HUD** (official `hud/crosshair`,
  `hud/hotbar` + selection sprite, ten `hud/heart/full`, ten
  `hud/food_full`) at vanilla geometry (`--screen world`).
- **Day-night cycle, generated** — the 26.x cycle is *data*: the generator
  extracts `timeline/day.json` + `timeline/moon.json` +
  `dimension_type/overworld.json`, and the client evaluates the official
  keyframe tracks (cubic-bezier-eased sun/moon/star arcs, sky/fog color
  fades, star brightness) exactly like the game's `KeyframeTrackSampler`.
  The **real `environment/celestial` sun + 8 moon-phase textures** render
  additively (official `BlendFunction.OVERLAY`) in the rotY(-90°)·rotX
  celestial frame, and the **star field replicates the official
  `SkyRenderer.buildStars`** (`RandomSource(10842L)`, 1500 candidates,
  radius 100 — LCG cross-checked against real `java.util.Random` on
  OpenJDK 21). `--screen worldnight` pins the night state (full moon over
  the eastern horizon, stars at the official 0.5 brightness).
- **Sunrise/sunset glow** — the official `visual/sunrise_sunset_color`
  track (`#AARRGGBB`, alpha 0 at midday -> the game's early-out) tints the
  18-vertex horizon fan replicated from `SkyRenderer.buildSunriseFan`
  (center (0,100,0) alpha 1, 17-vertex ring `(sin·120, cos·120, -cos·40)`
  alpha 0, `rotX(90°)·rotZ(90°[+180°])`, `scale(1,1,alpha)`), blended with
  the official TRANSLUCENT function — golden-orange glow at the rising
  horizon, crimson at the setting one.
- **Clouds, generated** — the official `CloudRenderer` replicated from
  bytecode: the real `environment/clouds.png` cell map (256×256 texels, one
  12×12-block cell each, solid iff alpha ≥ 10) tiles the world at
  `visual/cloud_height` 192.33 as 12×4×12 FANCY boxes, slides westward at
  the official 0.03 blocks/tick (`(gameTime % (width·400)) · 0.030000001`,
  wrapping exactly at the texture period; constant Z offset 3.96), colors
  via `ARGB.multiply(#CCFFFFFF, visual/cloud_color track)` — white by day,
  `#191926` by night — with the shader's face shading (bottom 0.7, N/S 0.8,
  W/E 0.9), the CLOUDS pipeline's TRANSLUCENT blend + depth test against
  terrain, and the per-pixel alpha fade to `cloud_fog_end` 2048.
- **Pause menu** — `Game Paused` over the dimmed frozen world with
  Back to Game / Options... / Save and Quit to Title, all official strings
  and widgets; Escape toggles it in `--gui` (`--screen pause`).
- **Options** — `Options` screen with volume/FOV controls (`--screen options`).
- **Music, generated — and now audible** — the vanilla menu-music behavior
  (weighted-random file from `music.menu` → play → random delay → next),
  driven by the official `sounds.json` tables. `mc2rs-gen` fetches the real
  26.2 menu pool (9 sha1-verified .ogg files — the five `music/game` tracks
  the menu event mixes in plus the four classics) through the asset index;
  symphonia decodes those exact bytes, the scheduler gets the REAL per-file
  lengths, and playback goes through rodio behind `--features audio`
  (deterministic silent backend everywhere else — CI has no speakers).
- **UI button click sounds** — every activated button plays the official
  `ui.button.click` (fetched, sha1-verified, decoded like the music), on an
  independent one-shot channel that never interrupts the music — the same
  rule as vanilla `AbstractButton.playDownSound`.
- **In-game music** — entering a world interrupts the menu track and hands
  the scheduler the spawn biome's official background-music situation
  (`attributes["minecraft:audio/background_music"]` of the real
  `data/minecraft/worldgen/biome/*.json`): biome-specific pools
  (`music.overworld.*`, `music.nether.*`, …) with their REAL 12000–24000
  delay bounds, the vanilla `music.game` default for biomes without an
  entry (32 sha1-verified files, weighted pick, real decoded lengths);
  Save and Quit hands back to the menu situation.
- **Windowed app** — `--gui` runs the interactive boot chain (mouse + clicks,
  with menu music when built with the audio feature).
- Text rendering uses the **real bitmap font**: glyph metrics measured from
  the official font textures with the vanilla trim rule, `§`-color codes and
  1px shadow, exactly like the original renderer.

- **Login sequence** — the protocol layer speaks the offline-mode login
  (handshake state 2 -> Login Start -> Set Compression -> Success /
  Disconnect) with the REAL protocol number from the jar's embedded
  `version.json` (26.2 = 776; launchermeta no longer carries it), vanilla
  offline-UUID derivation (`NameUUIDFromBytes`), and an in-process mock
  server exercising every path including the compressed envelope.

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
