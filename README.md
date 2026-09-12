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
| `mc-protocol` | Hand-rolled protocol layer: VarInt/VarLong, packet framing (incl. zlib compression), handshake, **server list ping**, **offline-mode login + the configuration phase** (client information, brand, keep-alive/ping, **select-known-packs + cookie replies**, finish-configuration ack), **play state** — the first real world from the network (world login, keep-alive/ping/teleport/chunk-batch answered, chunk-data decoded into resolved block/biome ids with vanilla palette packing). |
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
  full join arc (handshake, Login Start with the official offline profile
  UUID, Set Compression, Login Success, Login Acknowledged, Client
  Information + brand, keep-alive/ping probes, Finish Configuration, then
  the **play state**: world info, chunk streaming answered, real chunk
  data decoded) fires on a worker thread against the selected row and is
  pumped to the world-load milestone; a disconnect lands on the vanilla
  *Connection Lost* screen with the server's own reason, while a success
  does what the original does — **puts you in the world**: the join
  worker materializes the snapshot's player-centered window into the
  renderer grid and the world screen (official sky, celestials, fog,
  clouds, HUD, pause) renders the network terrain with the exact same
  raster path as the pixel tests. Decoded columns land in a compact
  client-side **world model** (chunk-keyed storage, vanilla block-index
  queries, replace/unload semantics) with a **materializer** that projects
  any window of it into the renderer's voxel grid. The id mapping is
  **bytecode-true and generated**: `mc2rs-gen dump-block-states` boots the
  official jar's REAL vanilla registry (client jar + official libraries,
  all sha1-verified, through the checked-in runtime dumper) and emits the
  global state id table the join mapper resolves through — the palette
  itself is **generated and grown**: the survival classics (stone, dirt,
  grass, logs, leaves, water, sand) plus the generated members (granite,
  diorite, andesite, deepslate, cobbled deepslate, gravel, bedrock, the
  coal/copper/iron/gold/redstone/lapis/diamond/emerald ores, netherrack —
  24 kinds, all official textures in the generated atlas) render as
  themselves, blocks the palette has no member for stay truthful air; the
  substrate
  of the in-world view of a network server. The configuration
  phase's **Registry Data is decoded**: the project's first retained NBT
  reader/writer pulls the `minecraft:dimension_type` registry (every
  dimension's min_y/height vertical layout) out of the streamed packets
  and carries it through the join into the world model; the other
  registries (worldgen, banners...) are walked and skipped — EXCEPT the
  **`minecraft:worldgen/biome` registry, now RETAINED** as the id -> name
  table in arrival order (the index IS the synced network id the chunk
  biome palettes reference), the entry tags walked and skipped. The
  **current dimension is pinned at login**: the Login and Respawn
  payloads were re-aligned to the javap-proven vanilla 26.2 shapes
  (CommonPlayerSpawnInfo — dimension type registry id, dimension,
  portal cooldown, sea level), so the decoded registry immediately
  turns the login's dimension into the model's live `min_y` —
  `block()` answers real world-Y coordinates, and Respawn (the
  proven packet id 82) re-pins the layout on dimension change.
  The **world window is anchored on the player**: the materialized
  join view centers the player's chunk in x/z and anchors its
  vertical window on the teleport's feet (`min_y` + `player.y`,
  clamped at the world floor) — the first player-exact network world
  view: a real overworld join shows the surface the player stands
  on, not the first 48 blocks of deepslate above bedrock. The
  **camera is the teleport too**: the materialized view ships the
  join teleport's exact pose (feet + the vanilla standing eye
  height, the server-directed yaw/pitch converted from the vanilla
  angle conventions) and the world screen renders FROM the player —
  the vanilla first person; without layout knowledge the
  deterministic flight camera stands (never a guess).
  The join is **LIVE**: the one-shot milestone grew a session that
  stays — `connect_and_join_live` returns the snapshot AND the play
  session, and the join worker keeps pumping it while you play.
  Chunks, teleports, unloads, dimension changes and probes stream into
  the SAME `LiveView` model the screen renders; every world-state change
  re-materializes the player-centered window (cadence-limited) and the
  world screen swaps it in place — the terrain refreshes under the
  player's eyes and the camera rides every server teleport, exactly
  like the original's chunk updates. A session death in-world is the
  vanilla kick (the DisconnectedScreen with the server's reason), and
  Save and Quit closes the socket like the original's — no ghost client
  lingers on the server.
  The **spawn biome is REAL now** — the section biomes the chunk decoder
  already resolved meet the synced registry: the model answers
  `biome()`/`biome_name()` at any world position (the vanilla 4×4×4 cell
  grid, world-Y through the dimension layout), the live view ships the
  biome AT THE PLAYER's feet with every refresh, and the join's music
  situation is the spawn biome's own official background-music event —
  no more empty-key fallback. Live updates re-fire the situation on
  CHANGE only (the vanilla biome-music behavior — a same-biome refresh
  never restarts the track); an unresolvable biome keeps what is
  playing; the situation unloads with the level on Save and Quit/kick.
  A **dimension change is the vanilla level recreation**: the Respawn
  (the proven packet id 82) re-pins the vertical layout through the
  registry (the nether's min_y 0 takes over) and DROPS every loaded
  column of the old dimension — its blocks re-read under the new min_y
  would be pure fiction — so the window truthfully shows air until the
  new dimension's own chunks stream in and re-anchor on the new pose;
  the whole crossing is pinned end to end by the live e2e's
  dimension-change leg (Respawn → nether chunk → teleport, all on the
  SAME retained wire). The crossing also **resolves the new
  dimension's own biome**: the synced biome table carries every
  dimension's biomes in one registry, the nether's chunks reference
  their own id (crimson_forest, not the overworld's plains), and the
  change-only rule re-fires the music with the nether situation —
  the crossing's full biome chain (named → honestly None mid-transition
  → the nether's own name at the new pose) is pinned at the view layer
  and on the wire.
  The **chunk light tail is REAL now** (the fifth javap audit —
  `ClientboundLightUpdatePacketData` from the official 26.2 jar): after
  the block entities, the packet carries four BitSets (sky/block lit
  masks + empty masks) and two `byteArray(2048)` lists (sky then
  block) — one 2048-byte nibble array per section in the lit mask,
  count == popcount, and the light sections span ONE EXTRA LAYER below
  and above the block sections (`getLightSectionCount() =
  getSectionsCount() + 2`). The parser decodes all of it strictly
  (a lying array count, a short array, an out-of-range mask bit, a
  section marked both lit and empty, and trailing bytes after the
  lists are all hard errors), the columns carry the result, and the
  world model answers `sky_light()`/`block_light()` at any world
  position through the engine-layer grid — a known-empty section
  honestly answers 0, an unmentioned layer the honest None; the even
  nibble index lives in the low half-byte (javap-pinned DataLayer
  convention). The renderer wiring (cave darkness under the sky layer)
  is the next step; no pixel changed here. The **standalone Update
  Light packet (generated id 48) rides the live wire too** (the sixth
  javap check — the real constructor reads VarInt chunk X/Z, NOT the
  chunk packet's Ints, then the exact same light data): the pump
  dispatches it, and the model REPLACES the stored column's light —
  refused honestly for unloaded columns or out-of-range mask bits,
  never half-applied (the vanilla masks are the engine's full current
  storage for the column, so unmentioned sections drop to the honest
  None); the live e2e grew the leg (chunk → teleport → light update →
  disconnect, eight pump steps).
- **The light layer reached the RENDERER (v0.3.37 — the first render
  change driven by real server data)**: `materialize` now carries the
  model's sky/block light into the view grid (sky and block SEPARATELY,
  every cell of a sent column, air included; the honest `LIGHT_UNKNOWN`
  where the engine never answered), and the mesher consumes it
  vanilla-style: each corner averages the 4 face-plane cells (the face
  cell plus the 3 AO diagonals — the same cells the AO walk samples),
  KNOWN cells only, through the vanilla brightness ramp
  `f/(4 - 3f)` (level 15 → exactly 1.0 — open-sky faces keep the legacy
  look; level 0 → 0.0 — a known-dark cave face is black). Corners with
  no known cell keep the legacy look (unsent neighbors never darken
  what they touch). A world without a light layer — the whole
  procedural scene, every golden — meshes BIT-IDENTICALLY to the legacy
  path: no golden changed, and the all-unknown layer is proven
  invisible by test.
- **The NETWORK view has its own golden (v0.3.38 — the wire pipeline is
  now pixel-pinned)**: `--net-world` renders a deterministic scene that
  never exists as Rust data: full Chunk Data packets (the shared
  `test_support` writer) are decoded by the REAL `parse_chunk_data`,
  applied to the `WorldModel`, materialized through the generated
  runtime mapper, then meshed WITH the light layer active. The scene is
  a pure function of grid coordinates — a stepped hill whose bedrock,
  ore sprinkles and grass skin ride section 0, one floating stone shelf
  whose footprint the sky arrays mark as hard shadow (sky 0 under it),
  and a torch cell painting the vanilla 14..0 block-light gradient over
  the shadowed ground. The fixed CPU orbit camera (the same framing as
  the procedural goldens) looks under the slab: bright sky-15 terrain
  proves the ramp's top end renders legacy-identical, the shadow proves
  the dark end, the gradient pins the corner-averaging. Block state ids
  resolve FROM the generated table at build time (a registry drift
  panics the fixture instead of silently misrendering), the palette
  entries are wire indices, and the engine layers (bit 0 below, bit 4
  above) ride the tail for real. `goldens/net_320x180.png` joins the
  CI pixel job and the local verifier at rms <= 8 — the same class as
  the CPU goldens (shared float raster math).
  The **night is wired end to end (v0.3.39 — the seventh javap
  audit)**: 26.2
  broke the old Set Time packet — the real `ClientboundSetTimePacket`
  is `LONG gameTime` + a `ByteBufCodecs.map` of `WorldClock` holders
  (`holderRegistry(WORLD_CLOCK)` = VarInt registry ids) to
  `ClockNetworkState` records (`VAR_LONG totalTicks, FLOAT
  partialTick, FLOAT rate`); `parse_set_time` decodes it with the
  usual lying-count and trailing-byte guards, the pump grew a ninth
  live step, and the model stores game time + the synced clocks
  (`day_tick` = the first clock's total ticks mod the day period —
  the multi-clock disambiguation waits for the WORLD_CLOCK registry
  names). The darkening itself is no longer a formula in 26.2:
  `Level.updateSkyBrightness` reads the dimension's ENVIRONMENT
  ATTRIBUTES, which are TIMELINE TRACKS (`data/minecraft/timeline/
  day.json`, bound to the overworld clock, period 24000), and the
  lightmap fragment shader (`assets/minecraft/shaders/core/
  lightmap.fsh`, read verbatim from the jar) applies
  `get_brightness(sky) * SkyFactor` — additive with the block term.
  The mesher therefore keeps sky and block SEPARATE through the
  per-corner smooth-lighting average (the vanilla packed-int halves —
  the golden's mixed torch/sky corners shifted by rms 0.475, the
  golden regenerated to pin the corrected math) and scales the sky
  term by the day timeline's `visual/sky_light_factor` (1.0 day ->
  0.24 night, jar-extracted keyframes in `timeline.rs`; the curve is
  cyclic-linear, wrap segment included). Worlds with no Set Time yet
  stay day — every procedural and GUI golden is bit-identical, the
  8 legacy checks untouched.
  The **clock ids got their names (v0.3.40 — the multi-clock
  disambiguation)**: the configuration phase now retains the synced
  `minecraft:world_clock` registry as the arrival-order id table (the
  element codec is UNIT — `WorldClock.DIRECT_CODEC =
  MapCodec.unitCodec` — so real entries ride tagless, names only) and
  the dimension-type elements keep their `default_clock` name
  (overworld/end carry one, the nether has none — no day cycle).
  `WorldModel::day_tick` resolves the CURRENT dimension's clock by
  NAME through that table: the end's clock state never masquerades as
  the overworld's, whatever order the server syncs them in; a named
  clock whose state never arrived is the honest no-opinion (day),
  and only a server with NO world_clock knowledge falls back to the
  first clock in the packet. The e2e mock streams the world_clock
  registry like vanilla does.
  The **lightmap combine is the shader's own (v0.3.41 — the true
  additive sum)**: where sky and block light meet at a corner the
  mesher now computes `get_brightness(sky) * SkyFactor +
  get_brightness(block)` clamped at 1.0 — the lightmap fragment
  shader's arithmetic verbatim — instead of the pre-night max() of
  the two terms. A torch under the night sky is fully bright again
  (0.24 + brightness(14) clamps to 1.0), a dusk-lit wall next to a
  lantern glows warmer than either term alone, and the single-layer
  and sky-15-day corners stay bit-identical (the clamp returns the
  old max's verdict there). The net golden re-pins the corrected
  math (old vs new rms 0.363 — the shadow-boundary corners where the
  smooth average carries both terms; the regenerated golden re-renders
  at rms 0.000), and the eight legacy goldens are untouched by
  construction (procedural scenes carry no light layer).
  The **day/night curve is data-driven (v0.3.42 — the TIMELINE
  registry parse)**: the eighth javap audit proved the wire facts —
  `Registries.TIMELINE` = the `minecraft:timeline` registry
  (synced); the Registry Data packet is ELEMENTS ONLY
  (`ClientboundRegistryDataPacket` is the record (registry, entries))
  and the tags ride a separate common-phase `ClientboundUpdateTagsPacket`
  (a map of registry name -> tag name -> VarInt id list); the
  timeline element (`Timeline.DIRECT_CODEC`) is `clock` (required
  string) + `period_ticks` (optional int) + `tracks` (a map whose
  values are `keyframes` of {ticks, value} with `ease` defaulting to
  LINEAR). The client decodes ONE track today —
  `visual/sky_light_factor` (the string/bool gameplay tracks are
  walked past) — and resolves the dimension's curve DATA-DRIVEN: the
  element's `timelines` reference (the tag form
  `#minecraft:in_overworld`) narrows through the Update Tags ids to
  the tag's timelines, the first track-carrier serves (real data:
  only `minecraft:day` carries it — moon/early_game do not), and the
  cyclic-linear evaluation is the golden-pinned math from the
  snapshot era. The compile-time day.json keyframes REMAIN as the
  documented fallback: a server with no timeline knowledge renders
  bit-identically, and an unknown tag never resurrects an off-tag
  carrier. The e2e mock streams the timeline registry + Update Tags
  like a real vanilla server; the live join carries the table.
  The **net golden grew a sibling (v0.3.43 — the second matrix size)**:
  `goldens/net_480x270.png` pins the wire pipeline at 480x270 — the
  cpu golden matrix has carried two sizes since its first run, the net
  path had one, and resolution-dependent raster drift (aspect, fog
  curve, frustum culling) is exactly the class a single-size gate
  cannot see. Rendered once with the exact CI flags, bit-exact on
  re-render (rms 0.000), gated at rms <= 8 like its 320x180 sibling;
  verify_goldens.sh and the CI pixel job grew the tenth check.
  The **wire ids of every phase are generated**:
  `mc2rs-gen dump-packet-ids` binds the official jar's REAL protocols —
  play, login, configuration, status AND handshake — and dumps each
  packet's live wire id from the vanilla dispatch codec's id map. The
  hand-kept wiki-era tables had drifted in two waves: the play ids
  (login 0x27 -> 49, chunk data 0x23 -> 45, respawn = 82, ...) and —
  found by the second, all-phase dump — **every clientbound
  configuration id one low** (registry data 0x06 -> 0x07, finish 0x02
  -> 0x03, keep alive 0x03 -> 0x04, ping 0x04 -> 0x05, disconnect 0x01
  -> 0x02: the vanilla flow grew cookie_request at its head), a drift
  the self-consistent e2e mocks could never expose — a real 26.2 join
  would have failed to recognize a single configuration packet.
  The **payload shapes got the same runtime proof** (javap on the
  official 26.2 classes — a third drift wave, this time in the wire
  DATA the ids point at): **Login Success** is the record
  (GameProfile, sessionId) — uuid + name + a property list (count +
  name/value/nullable-signature entries) + a 16-byte session UUID the
  wiki-era "uuid + username" shape never knew; **Registry Data** is
  (registry key, List<PackedRegistryEntry>) — a VarInt entry count of
  {entry id, optional network NBT} pairs, NOT "identifier + whole
  NBT" (the old parse would have read the entry count as an NBT root
  and failed on a real server; the dimension element now arrives per
  entry with min_y/height at its top level); and **Client
  Information** grew a ninth field (particle status) with the view
  distance as a raw BYTE, not a VarInt. The e2e mocks were rebuilt to
  the proven shapes — the self-consistency mask is broken at the
  payload level too. The **configuration probes are now answered**
  (javap-proven off the 26.2 codec bytecode): **Select Known Packs** —
  a list of three-STRING KnownPack records (the version is a string,
  not a VarInt; the serverbound reply capped at 64 by the vanilla's
  own `list(64)`) — gets the honest EMPTY reply, because the vanilla
  server BLOCKS on it before finishing configuration (the old
  wholesale skip stalled a real join for 30 seconds); **Cookie
  Requests** get the vanilla client's own no-cookie answer (key echoed
  + the absent nullable payload, `byteArray(5120)` under the flag).
  The **play-phase packets got the same audit** — and a fifth drift
  wave fell out: **Synchronize Player Position** is the composite
  (VAR_INT teleportId, PositionMoveRotation, Relative.SET) — the
  teleport id comes FIRST (the wiki-era tail), the position rides a
  record with a **deltaMovement Vec3 the wiki-era shape never knew**,
  and the relativity flags are a full INT bitmask (the 1.21.2+ nine
  Relative bits), not a byte; on a real server the old parser would
  have misread every teleport. Everything else play-phase (Login,
  CommonPlayerSpawnInfo, Respawn, Chunk Data, keep-alive/ping, center
  chunk, unload chunk, chunk batch, Accept Teleportation) is
  javap-proven UNCHANGED off the same classes.
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
  Save and Quit hands back to the menu situation. On a NETWORK join the
  biome is the server's own truth: the synced registry names + the
  decoded section biomes under the player drive the situation (see the
  multiplayer section) — the seeded pick remains only for the
  procedural singleplayer world.
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
  flip against crease artifacts; water/leaves never occlude. Network worlds
  add the REAL light layer on top: per-corner sky/block averages through the
  vanilla brightness ramp `f/(4 - 3f)` — cave darkness and torch-lit faces
  straight from the server's engine arrays.
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
