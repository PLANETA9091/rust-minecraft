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
  The **biomes are decoded (v0.3.44 — the tint foundation)**: the synced
  `minecraft:worldgen/biome` registry grew from a name table to real
  elements — the NINTH javap audit pinned `Biome.NETWORK_CODEC` (the
  codec `RegistryDataLoader`'s networked list binds — NOT the disk
  DIRECT_CODEC): a record of exactly THREE, the climate settings
  (`has_precipitation`/`temperature`/optional `temperature_modifier`/
  `downfall`), the OPTIONAL `attributes` compound (the 26.x
  environment-attribute migration's home for sky/fog/music —
  `minecraft:visual/sky_color`, `minecraft:visual/fog_color`,
  `minecraft:audio/background_music`; walked past: nothing consumes
  them yet), and the required `effects` compound — and
  `BiomeSpecialEffects` in 26.2 carries ONLY water color + the three
  optional overrides (`grass_color`/`foliage_color`/the new
  `dry_foliage_color`) + `grass_color_modifier` (exactly THREE
  constants serialized `none`/`swamp`/`dark_forest`; absent ≡ "none",
  vanilla's own optionalFieldOf default). The Update Tags payload's
  biome tags are retained beside the timeline tags (tag -> ids ->
  arrival-order names, the same chain the model resolves). Tagless
  biome entries, missing climate/effects fields and unknown modifier
  names are the drifts they are — errors, like vanilla's own loader.
  Zero render-path pixels: the tint CONSUMERS come next; the ten
  goldens are bit-identical by construction (the wire fixture carries
  no registry knowledge).
  The **pack-version claim replies with the real record (v0.3.54 — the
  arc's final step)**: Select Known Packs no longer gets the honest
  empty reply — the client now claims exactly what the vanilla client
  claims, `("minecraft", "core", WorldVersion.id())`, javap-proven from
  the real jar (`BuiltInPackSource.<clinit>` binds `CORE_PACK_INFO =
  KnownPack.vanilla("core")`, and `KnownPack.vanilla(id)` passes
  `SharedConstants.getCurrentVersion().id()` — so the version string is
  the jar's version.json `id` field, NOT the pack-format number; 26.2's
  version.json even grew the four-field
  `pack_version` object `{resource_major, resource_minor, data_major,
  data_minor}`, which is a separate surface and NOT the claim string).
  The data half is generated: the new offline subcommand
  `mc2rs-gen pack-claim --version --jar --out` reads the jar's root
  `version.json` (no network, no java — the worldgen/block-ids shape)
  and emits `mc-protocol/src/generated/pack_claim.rs` — `VERSION_ID`
  (the claim string), `VERSION_NAME`, `WORLD_VERSION`, `PROTOCOL_VERSION`
  and the four pack-format counters — refusing a jar without the
  manifest, an empty id, or a pack_version that does not state the full
  four-field schema (guessing a shape the jar does not state would put a
  lie on the wire); the checked-in file re-generates byte-identically.
  The hand half carries the proof: `vanilla_known_packs()` builds the
  one-record reply from the generated constants, and `run_configuration`
  sends it. The safety valve is the server's own comparison — a server
  whose pack version differs does not recognize the claim and streams
  its registries anyway (the pre-claim behavior, unchanged); a server
  that DOES skip the sync agrees with this client by construction (the
  v0.3.53 order tables ARE the vanilla wire ids, the v0.3.52 built-in
  dimension layouts answer the world-height lookups). The e2e mock —
  which has offered exactly `("minecraft", "core", "26.2")` since the
  blocking-probe arc — now asserts the reply echoes that record instead
  of the empty list. Zero render-path pixels: the config phase touches
  no golden scene (the net fixture is programmatic, no join runs); the
  ten goldens are bit-identical, verified TWICE on the release builds.
  Tests +3 (workspace 250 → 253 passed / 0 failed / 3 ignored): the
  generator's manifest parse/emission fixture (the real 26.2 surface,
  the missing/empty/legacy-schema refusals) and the protocol claim-shape
  pin (one record, minecraft/core, non-empty version = the generated
  id, the exact offer-echo bytes).
  The **biome tints fall back to the built-in tables (v0.3.55 — the first
  server-visible consequence of the claim)**: a vanilla server that accepts
  the known-packs claim skips Registry Data, and the client now renders its
  biomes anyway — the biome-tint slice of the skip-sync fallback, joining
  the v0.3.53 wire-order evidence with the v0.3.51 jar tables. The data
  side grew ONE appended table: the worldgen emitter now carries the
  per-biome tint extras the classic color surface does not —
  `effects.grass_color_modifier` (the vanilla enum names verbatim; "none"
  states the absence and is dropped; an unknown name is a REFUSAL — the jar
  does not state it, guessing would put a lie in the table) and
  `effects.dry_foliage_color` (the `"#rrggbb"` string form the real jar
  uses) — sparse, exactly the four 26.2 carriers (dark_forest,
  mangrove_swamp, swamp, pale_garden), appended LAST so the previous
  29318-byte generated file verifies as a byte-exact prefix. The hand side:
  `tint::resolve_tints` is now the ONE resolution chain both paths share
  (the synced `BiomeEntry` and the built-in `BiomeDef`+extras alike — the
  two paths cannot disagree about a vanilla biome's verdict),
  `tint::built_in_tints(name)` resolves a built-in biome through it, and
  `WorldModel::biome_tints()` serves the quads in WIRE order when (and only
  when) no biome sync arrived — a sync that DID arrive stays the
  unchallenged truth (the quad list keeps the synced table's length, a
  lying entry renders as it lies), mirroring the v0.3.52 dimension
  fallback's scoping; `biome_name` grew the same fallback (a bare id walks
  `BIOME_ORDER`). THE FIXTURE HONESTY (the fallout the contract demands):
  the net fixture's single-valued biome id moved 1 → 200 — a mock that
  sent NO registry must not carry a bare wire id the vanilla order names
  (id 1 would resolve bamboo_jungle and fold its jungle tint into the
  golden grass); the same honesty the "mock:test" dimension name rides,
  and one world_view assertion updated for the same reason (a cleared sync
  names the bare vanilla id 0 badlands now — that IS the skip-sync state).
  Zero golden pixels: the golden scenes carry a synced registry or ids the
  vanilla order never names and never consult the fallback — verified
  TWICE on the release builds with the EXACT CI flags (cpu/net rms 0.000;
  the gui screens at their per-screen CI frames — title/servers/loading/
  world/worldnight/pause = 30/60/15/30/30/30 — rms 0.000). Tests +6
  (workspace 253 → 259 passed / 0 failed / 3 ignored): the generator's
  extras parse/emission fixture + the unknown-modifier refusal,
  mc-registry's jar-value + join-consistency pin, tint's verdict-for-
  verdict equivalence (plains + the dark_forest blend) and the verbatim
  overrides, and the model's wire-order fallback (names, quads,
  sync-precedence, no-rescue).
  The **registry-order evidence is dumped from the real runtime (v0.3.53 —
  the pack-version claim's LAST prerequisite)**: the generator grew its
  third runtime dumper — the checked-in clean-room `DumpRegistries.java` —
  and its online/offline subcommand pair (`mc2rs-gen dump-registries` /
  `mc2rs-gen registry-order --dump <saved>`), the block-ids arc's shape.
  The dumper does NOT stop at the static `BuiltInRegistries` root (whose
  member list carries no dimension_type/biome/timeline/world_clock — those
  are DATA-driven registries): it boots the vanilla data side exactly as
  the WorldLoader does — the jar's own vanilla data pack through
  `VanillaPackResources` + `MultiPackResourceManager(SERVER_DATA)`, block
  TAGS bound through `TagLoader.loadTagsForExistingRegistries` (the
  `PendingTags.lookup()` wrappers — without them the overworld dimension
  type refuses to parse: its infiniburn reference is a block tag), then
  `RegistryDataLoader.load(..., SYNCHRONIZED_REGISTRIES, ...)` — the very
  list a vanilla server streams during configuration. The result: the
  bytecode-true WIRE order of every synchronized registry, `registry|id|
  entry` per line, saved as `.mc2rs/classpath/registries.txt`. The
  generated table (`registry_order.rs`) carries the four the client's
  runtime consumes — `DIMENSION_TYPE_ORDER` (overworld, overworld_caves,
  the_end, the_nether), `BIOME_ORDER` (66), `TIMELINE_ORDER` (day,
  early_game, moon, villager_schedule), `WORLD_CLOCK_ORDER` (overworld,
  the_end) — the array index IS the wire id, so a skip-sync server and
  this client agree on ids by construction. The emitter REFUSES a dump
  missing any ordered registry, refuses non-dense ids and duplicate
  names, and a consistency test pins the evidence layer against the data
  layer: each order table must name exactly the same member set as its
  built-in table twin (a drift would map wire ids onto wrong entries).
  For 26.2 the dumped orders happen to be name-sorted — now EVIDENCED,
  not assumed. The claim itself (a non-empty Select Known Packs reply)
  is the small step this unlocks next. Zero render-path pixels: nothing
  consumes the order tables yet; the ten goldens are bit-identical by
  construction, verified TWICE on the release builds. Tests +2 (workspace
  248 → 250 passed / 0 failed / 3 ignored): the registry-dump
  parse/emission fixture (dense-id and duplicate rejection, the
  missing-registry refusal, the offline emission) and the
  evidence-vs-data consistency pin.
  The **built-in dimension table is the skip-sync fallback (v0.3.52 —
  the first CONSUMER of the v0.3.51 tables)**: a server that never
  sends Registry Data (a bare mock, a trimmed proxy — or, once the
  pack-version claim lands, a vanilla server that skipped the sync
  BECAUSE the client claimed its known packs) no longer leaves the
  world layout unknown. `WorldModel::dimension_min_y` now falls back
  to the generated built-in `minecraft:dimension_type` table — the
  SAME jar data a matching server would sync — when the synced
  dimension table is EMPTY, and the honest scoping is pinned by test:
  a bare server naming `minecraft:overworld` gets the real floor
  (-64: the login pin and the nether Respawn re-pin resolve through
  the fallback), an ARRIVED sync is the only truth (its entry wins
  even when it disagrees with the jar, and a key it omits gets NO
  built-in rescue — the server runs its own dimension set), and the
  vanilla names the jar does not build stay unknown. The bare-mock
  test fixtures were renamed accordingly (`mock:test`): a mock layout
  that is NOT vanilla-shaped must not claim the vanilla name without
  a sync, or the fallback would (correctly) shift its window under
  the terrain. Zero render-path pixels: the golden scenes carry a
  synced registry (sync-first) or none at all and never consult the
  fallback — the ten goldens are bit-identical by construction,
  verified on the release build. Tests +1 (workspace 247 → 248
  passed / 0 failed / 3 ignored): the skip-sync fallback pin (all
  four built-in layouts through a bare model, the login/Respawn flow,
  sync-precedence, and the no-rescue-on-partial-sync rule).
  The **built-in registry tables ship from the jar (v0.3.51 — the
  pack-version claim's prerequisite)**: the generator now emits the
  data/minecraft/{dimension_type,timeline,tags/timeline} side of the
  jar beside the biome tables — the registry entries a vanilla server
  syncs and a matching client could claim as known packs.
  `worldgen::generate` widened its listing from the worldgen/ tree to
  all of data/minecraft/ (every entry still classified by strict
  prefix), parsing 4 dimension types (the nether: min_y 0 / height 256
  / logical_height 128, has_fixed_time, ambient_light 0.1,
  coordinate_scale 8.0, no default_clock — the overworld: -64/384/384
  with the default clock), 4 timelines (clock, period_ticks, track
  count, and the `minecraft:visual/sky_light_factor` numeric keyframes
  — the render track the lightmap's SkyFactor multiplies; the other
  real timelines carry none and emit empty, not invented), and 4
  timeline tags verbatim (nested "#"-references intact —
  #minecraft:in_overworld resolves through #minecraft:universal to
  villager_schedule). The emitted tables are appended AFTER the biome
  data so the old generated file verified as a byte-exact prefix of
  the new one (the determinism discipline). A new offline subcommand
  `mc2rs-gen worldgen --jar <path>` re-emits the file from a LOCAL
  client jar — no network, no java, the `block-ids` precedent. The
  day timeline's emitted keyframes are EXACTLY the constants
  `mc-client/src/timeline.rs` pins as the compile-time snapshot
  ((730,1.0),(11270,1.0),(13140,0.24),(22860,0.24), period 24000) —
  now test-pinned from BOTH sides: mc-registry asserts the generated
  table's shape, and mc-client asserts its constants still byte-match
  the generated table, so the hand-kept render constants can only
  drift by failing a test. The pack-version claim (a non-empty Select
  Known Packs reply) stays unclaimed here — it needs this FULL table
  work (done) plus the skip-sync fallback (future). Zero render-path
  pixels: the generated tables feed no golden scene; the ten goldens
  are bit-identical by construction, verified TWICE on the release
  builds. Tests +4 (workspace 243 → 247 passed / 0 failed / 3
  ignored): the generator parse/emission fixture (unsorted keyframes
  arrive sorted, hex-string visual tracks are not the numeric render
  track, absent has_fixed_time defaults false), the mc-registry
  dimension-layout and day-keyframe pins, and the mc-client
  constants-vs-table cross-pin.
  The **leaf litter renders per-state — state-property extraction (v0.3.50 —
  the fourteenth audit retires the documented follow-up)**: v0.3.49's
  closing note ("per-segment coverage and per-facing rotation need
  state-property extraction") is done. The FOURTEENTH audit re-read the
  real jar data directly (the saved dumper output `block_states.txt`,
  plus the blockstate JSON and all four template models re-extracted
  from the official client.jar) and pinned the exact shape: the
  multipart's model sets are 1->{m1}, 2->{m2}, 3->{m2,m3}, 4->{m4}
  (each `when` matches `segment_amount` literally — "2|3" applies
  model_2, "3" adds model_3, "4" swaps in model_4, whose element is the
  FULL cover), the y-rotation table is north=0 / east=90 / south=180 /
  west=270, and the template down-face uvs (`[x0, z1, x1, z0]`) mirror
  v WITHIN the rect — `v = (z0 + z1 - z)/16`, which collapses exactly
  to the old `face_uv` `1 - z` at the full rect. The generator keeps
  the props now: `parse_dump` carries the dumper's third field verbatim
  and `emit_block_ids` APPENDS the per-state property-assignment table
  (`STATE_PROPS`, 32062 runs tiling [0, 32366) — adjacent identical
  props collapse, no-property runs merge across blocks) plus the
  `props_of` binary-search accessor, with the generated name table
  above it byte-identical (the v0.3.49 file verified as a byte-exact
  prefix), and a new `mc2rs-gen block-ids` subcommand re-emits the
  table OFFLINE from a saved dump (no java, no network). The client
  grew the per-cell STATE-VARIANT byte: `World.variants` (`None` for
  the procedural scenes — bit for bit the previous mesh),
  `materialize` decodes leaf_litter's facing x segment_amount through
  `props_of` during the SAME walk (packing
  `1 + facing + (segment - 1) * 4`, the y-rotation order), and the
  mesher's flat path generalizes: per-rect up+down quads with the
  template uv arithmetic and the blockstate y-rotation about the cell
  center, uv glued to the corners (the element turns, the texture
  turns with it — vanilla). Segment 4 IS the full rect at y-rot 0: the
  no-opinion look and the shaped look emit identical bytes
  (test-proven). Tests +6 (workspace 237 → 243 passed / 0 failed /
  3 ignored): the real-id decode table, the materialize variant fill
  (through a REAL 15-bit direct-palette section — the wire encoding
  for big palettes), the NW quarter, the segment 2/3 rect stacking,
  the east rotation with glued uv, and the segment-4 bit-identity.
  Ten goldens bit-identical by construction (no golden scene places
  leaf litter and no procedural scene carries variants), verified
  TWICE on the release builds.
  The **leaf litter renders — the first non-cube mesh path (v0.3.49 —
  the thirteenth audit completes the dry fold)**: the DRY_SLOT carried
  since v0.3.48 finally has its consumer. The THIRTEENTH audit (official
  26.2 jar, asset side) pinned leaf_litter's real shape: the blockstate
  is a MULTIPART (`leaf_litter_1..4` × facing y-rotations, segment 3
  cumulatively applying model_2 AND model_3), and every
  `template_leaf_litter_N` is a ZERO-THICKNESS horizontal element at
  y=0.25 (a quarter pixel above the floor) with EXACTLY `up` + `down`
  faces, BOTH carrying `tintindex: 0` (→ the registered
  `BlockTintSources.dryFoliage()` source), `ambientocclusion: false`
  (no corner AO) and no cullface — while the texture is a GRAYSCALE
  master with BINARY alpha (probe: all 117 opaque pixels r==g==b,
  alphas only 0/255 — genuinely tintable cutout, the exact opposite of
  pre-colored pale oak). The generator grew its FIRST non-cube member
  (RUNTIME_PALETTE 25 → 26; the full-cube resolution chain cannot
  apply — multipart blockstate + a flat element — so the build loop
  special-cases the flat stem like water) plus `TINT_DRY = 0xA37546`,
  the dry analog of the plains constants: the REAL dry_foliage grid
  sampled at the plains climate (probed in the twelfth run); the atlas
  regenerated with the 28th tile appended and all 27 old tiles +
  colormap.rs byte-identical. The client's mesher grew its FIRST
  non-cube path: leaf_litter meshes as up + down quads on the
  y=0.25/16 plane (full footprint), shade-only multipliers (vanilla
  direction shade 1.0/0.5, NO AO term — the model verdict), the same
  vanilla-verbatim lightmap combine and per-face corner light as the
  cube path, cutout via the binary texel alpha; `is_transparent`
  admits the kind (non-occluding, casts no AO), `slot_for` maps its
  two model faces to DRY_SLOT, and the runtime mapper folds ALL 16
  states (the half-open 30339..30355 range) onto one kind with the
  synced dry override riding the same vertex carrier (the no-op rule
  keeps the baked plains sample). Tests +4 (workspace 233 → 237
  passed / 0 failed / 3 ignored). Ten goldens bit-identical by
  construction (no golden scene places leaf litter; the cpu/net pairs
  re-measured rms 0.000 explicitly), verified on the fresh v0.3.49
  release build. Documented follow-up: per-segment coverage
  (quarter/half-strip quads) and per-facing texture rotation need
  state-property extraction — every state renders the full footprint
  at the north orientation for now.
  The **dry foliage is consumed + pale oak leaves render (v0.3.48 — the
  twelfth audit and a verdict)**: the next documented deviation lands —
  and the audit CORRECTED the assumption that came with it. The TWELFTH
  javap audit pinned the dry chain byte-for-byte:
  `Biome.getDryFoliageColor()` is
  `effects.dryFoliageColorOverride().orElseGet(climate lookup)`, the
  climate lookup (`getDryFoliageColorFromTexture`) is bytecode-identical
  to its foliage sibling (RAW climate fields, caller-side
  `Mth.clamp(0,1)`, f2d widening), and `DryFoliageColor.get` delegates
  the SAME `ColorMapColorUtil.get` with the not-loaded default
  -10732494 (ConstantValue of `FOLIAGE_DRY_DEFAULT`; the 0xFF5C3C32
  argb word → RGB 0x5C3C32). The client consumed the resolution
  surface end to end: the generator bakes the jar's
  `textures/colormap/dry_foliage.png` as a THIRD grid
  (`mc-registry/generated/colormap.rs`, REAL_GRIDS-gated, probe-verified
  through the same `ColorMapColorUtil.get` arithmetic — plains climate
  0xA37546, pale_garden climate 0xA36D46, desert corner 0xA38046,
  jungle 0xA36346, swamp climate 0xA36946), and
  `tint::entry_tints` grew to a FOUR-slot quad
  (grass / foliage / water / dry) whose dry leg mirrors the audited
  `orElseGet` shape exactly (the real pale_garden override #A0A69C
  pinned as a test). THE VERDICT the audit surfaced: vanilla's ONLY
  block-color registration for `BlockTintSources.dryFoliage()` is
  leaf_litter — a non-cube ground cover whose mesh path is future work
  (the DRY_SLOT is carried, ready, selected by no kind yet) — while
  `pale_oak_leaves` carries NO tint source at all: its model's
  tintindex falls back to `BlockColors.BLANK_LAYER` (constant white)
  and the texture is PRE-COLORED (probe-verified: zero pure-gray
  pixels), so it renders as authored. pale_oak_leaves therefore joined
  the runtime palette (kind 24, the leaves family — non-occluding, no
  AO, cutout holes) with NO tint slot, exactly vanilla; the atlas
  regenerated with its official tile appended (28 tiles) and the old
  26 tiles byte-identical. Tests +3 (workspace 233 passed / 0 failed /
  3 ignored). Ten goldens bit-identical by construction (no golden
  scene places pale oak leaves or flows biome entries; the tint quads
  resolve only where a synced registry exists), verified on the fresh
  v0.3.48 release build.
  The **grass modifier is consumed (v0.3.47 — the dark_forest blend)**:
  the next documented deviation lands. The ELEVENTH javap audit pinned
  `BiomeSpecialEffects.GrassColorModifier` (declaration order NONE,
  DARK_FOREST, SWAMP): DARK_FOREST is pure whole-int arithmetic —
  `ARGB.opaque(((base & 0xFEFEFE) + 0x28340A) >> 1)`, where the
  channel-carry into bit 24 folds back through the shift exactly as
  vanilla's int math does and `ARGB.opaque = i | 0xFF000000` stays
  outside our 0xRRGGBB tints; the blend runs over the RESOLVED base
  (override or the colormap sample — vanilla wraps `modifyColor` around
  `getBaseGrassColor()`), so the end-to-end verdict for a real dark
  forest is the climate sample 0x79C05A blended into the canonical
  0x507A32, probe-verified against the jar grids before any client
  code. NONE is the bytecode identity; SWAMP stays the documented
  deviation — its verdict is `Biome.BIOME_INFO_NOISE` (a static
  PerlinSimplexNoise) at (x·0.0225, z·0.0225) with two FIXED colors
  (packed 0xFF4C763C below -0.1, else 0xFF6A7039), genuinely
  position-dependent and unexpressable in the per-entry tint triple,
  so the base passes through. Tests +3 (workspace 230 passed / 0
  failed / 3 ignored): the blend over the plains sample (0x5C7831),
  the verbatim whole-int carry case (0xFE0000 → 0x931A05), the
  canonical dark-forest end-to-end verdict, the override-as-base path,
  and the swamp pass-through. Ten goldens bit-identical by
  construction (no golden scene carries a modifier-bearing biome),
  verified on the v0.3.47 release build.
  The **colormap is consumed (v0.3.46 — the climate-driven tints)**:
  the top documented candidate lands — an override-less biome now
  samples the REAL jar colormap grids instead of keeping the baked
  plains tint. The TENTH javap audit pinned the full vanilla chain
  (`ColorMapColorUtil.get` + its callers): the util itself does NOT
  clamp — `Biome.getGrassColorFromTexture` clamps the RAW climate
  floats to [0,1] (`temperature_modifier` is NOT in the path) and
  widens; the util scales the downfall by the temperature FIRST
  (`rain *= temp`), picks column `i = (int)((1-temp)*255)` and row
  `j = (int)((1-rain)*255)` of the row-major 256×256 grid, and falls
  to the passed default out of range (unreachable clamped; vanilla
  CRASHES on unclamped inputs — the port returns the default, a
  strictly-safer documented deviation). The generator now also emits
  `mc-registry/src/generated/colormap.rs`: the jar's
  `textures/colormap/{grass,foliage}.png` decoded row-major to opaque
  0xRRGGBB (256×256 verified, `REAL_GRIDS` pins real-vs-fallback — a
  no-jar build zeroes the grids and the client keeps the pre-colormap
  baked-plains verdicts). THE PLAINS EQUIVALENCE is the chain's
  end-to-end proof, verified against the real grids BEFORE a line of
  client code existed and pinned as tests: the sample at the plains
  climate (0.8, 0.4) IS the baked `TINT_GRASS` 0x91BD59 /
  `TINT_FOLIAGE` 0x77AB2F — so plains folds as a no-op while desert
  (0xBFB755), jungle (0x59C93C) and snowy (0x80B497) climates now
  carry their vanilla-canonical colors. The resolution chain in
  `tint.rs` is vanilla-verbatim: override wins, else the clamped
  climate through the grids; the modifier (applied AFTER the base in
  vanilla) and `dry_foliage_color` stay decoded-but-unconsumed. Tests
  +8 (workspace 227 passed / 0 failed / 3 ignored): the plains
  equivalence, the known-biome climate spots, the `rain *= temp`
  ordering (a (0.5, 1.0) climate samples row 127, not the downfall-only
  row 0), the hand-indexed cell, the `Mth.clamp` mirror (NaN passes —
  and `(int)NaN` lands at 0 exactly like the bytecode), the
  out-of-range default, the real-vs-fallback gate, and the desert/
  jungle verdicts through `entry_tints`. Ten goldens bit-identical by
  construction (no golden scene flows biome entries), verified on the
  v0.3.46 release build.
  The **biome tints are consumed (v0.3.45 — the tint foundation pays)**:
  the first RENDER-FACING use of the decoded table. The client gained
  `tint.rs` (the resolution: `entry_tints` — grass/foliage overrides
  plus the REQUIRED water color per synced element; `slot_for` — the
  atlas' tintindex flags: grass BLOCK TOP, leaves and water on all
  faces) and `World::face_tint` (the voxel's registry id through the
  per-voxel biome layer `materialize` now ships — ids expanded from the
  wire's 4×4×4 cell grids through the model's proven queries, the
  honest `BIOME_UNKNOWN` where the server never answered). The mesher
  folds the resolved color into the VERTEX-COLOR carrier — the vanilla
  `vertexColor * texelColor` multiply, one code path with shade×AO×light.
  THE NO-OP RULE keeps every existing scene bit-identical: the generated
  atlas BAKES the vanilla plains constants into the tile pixels
  (`TINT_GRASS`/`TINT_FOLIAGE`/`TINT_WATER`), so a resolved tint EQUAL
  to the baked default stays unfolded — the fold fires ONLY when the
  synced table actually changes the color (a real server's swamp water,
  the badlands' grass/foliage overrides). Documented deviations (the
  next consumers): the COLORMAP lookup (temperature/downfall →
  grass.png/foliage.png) is not consumed — a biome without an override
  keeps the baked plains tint; `grass_color_modifier` (swamp needs the
  biome noise, dark_forest a blend) is not consumed; `dry_foliage_color`
  is carried with no texture to tint yet. Tests +8 (workspace 219 passed
  / 0 failed / 3 ignored): the resolution surface, the no-op rule, the
  fold into the carrier, the materialize ride-along, and the e2e
  milestone's decoded water color. Ten goldens bit-identical, verified
  on the release build.
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
