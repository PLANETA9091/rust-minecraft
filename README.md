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
  The **world_clock has its data-side twin (v0.3.56 — the day-tick
  path's last hand-carried surface)**: the worldgen emitter now parses the
  jar's `data/minecraft/world_clock/*.json` — and the honest finding is
  that the 26.2 entries are EMPTY objects (`overworld.json`,
  `the_end.json`, two bytes each): the clock behavior lives code-side and
  the NAMES are the data. The emitted `WORLD_CLOCKS` table (appended
  last — the previous generated file verified as a byte-exact prefix)
  completes the consistency matrix: every registry the runtime dumper
  ordered (dimension_type, biome, timeline, world_clock) now has a
  built-in data-table twin pinned to the same member set. A stated field
  in a future world_clock json is a REFUSAL, not a silent walk-past —
  dropped data would be a lie. THE SKIP-SYNC DAY-TICK (the first
  consumer): a claim-accepted server streams no Set Time registry
  context, so `WorldModel::day_tick` falls back to the built-in evidence
  under ONE gate — no dimension sync arrived at all: the clock NAME
  through the built-in dimension table (overworld/overworld_caves name
  `minecraft:overworld`, the_end names `minecraft:the_end`, the nether
  names nothing), the clock ID through the wire-order table the twin
  pins (the vanilla registration order IS the wire id). A sync that DID
  arrive keeps every existing contract — its element names the clock (or
  doesn't: the documented first-clock shape), its world_clock table maps
  the id, and a key it omits gets no built-in rescue. Zero render-path
  pixels by construction: the golden scenes carry a synced registry or
  no Set Time knowledge and never consult the fallback — verified TWICE
  on the release builds with the EXACT CI flags, all ten rms 0.000. Tests
  +2 (workspace 259 → 261 passed / 0 failed / 3 ignored): the generator's
  world_clock parse/emission + the payload refusal, mc-registry's twin
  pin (the order table joins its data twin), and the model's skip-sync
  day-tick (overworld through wire-order id 0, the end through id 1
  never the overworld's ticks, the nether and a mock name on the
  documented first-clock shape, a partial sync unrescued).
  The **wire protocol number speaks the generated manifest (v0.3.57 —
  one fact, one source)**: the login handshake's `776` used to flow from
  `mc-client/src/build_info.rs` — a generated stamp from the v0.2.0 era
  whose `unwrap_or(-1)` means it may LEGALLY carry "unknown" — while the
  v0.3.54 `pack_claim.rs` manifest (the emitter REFUSES a jar that does
  not state `protocol_version`) stated the same number with zero
  consumers. The live join (`window.rs`) and its screens-side mirror now
  pass `mc_protocol::generated::pack_claim::PROTOCOL_VERSION` — the only
  surface that cannot lie by omission — and the two generated surfaces
  are CROSS-PINNED (the timeline-snapshot pattern): if either generator
  flow is ever re-run against a different jar or the emissions drift, a
  test fails loudly instead of the wire quietly speaking two truths. The
  stale `mc_registry::build::PROTOCOL_VERSION` reference in the login
  doc comment (a file that has not existed for generations) now points
  at the real source. Zero render-path pixels by construction (protocol
  layer only); the goldens verified with the EXACT CI flags. Tests +1
  (workspace 261 → 262 passed / 0 failed / 3 ignored): the cross-pin
  (both surfaces agree; the manifest's number is a real stated one,
  never a fallback).
  The **timelines join the skip-sync fallback (v0.3.58 — the last
  registry surface without one)**: a claim-accepted vanilla server
  streams no Registry Data, and `WorldModel::sky_factor`'s curve
  resolution now walks the BUILT-IN jar tables when (and only when) no
  sync arrived — the same `dimension_types.is_empty()` gate the
  v0.3.52/55/56 fallbacks run under, sync precedence strict, a partial
  sync rescued by nothing. The chain is the one the synced path runs,
  ported to the generated twins: the dimension's `timelines` reference
  (the overworld names `#minecraft:in_overworld`) resolves through the
  built-in TIMELINE_TAGS — whose members are the RAW jar form, nested
  `#` references included (`in_overworld` → `#universal` →
  `villager_schedule`; a vanilla server streams tags pre-resolved as
  flat wire-id lists, so only the built-in walk needs the recursive
  expansion, with a visited set stopping cycles) — and the first
  timeline carrying `visual/sky_light_factor` serves. For the real
  26.2 jar every path CONVERGES on the day curve (`minecraft:day` is
  the only carrier; the end's `in_end` resolves to a non-carrier and
  the snapshot serves, unchanged), and that convergence is the
  contract: the snapshot has been byte-pinned to the generated
  TL_DAY_SKY_LIGHT_FACTOR since the timeline arc, so a future jar whose
  tables drift fails the pin instead of the pixels. Zero golden-path
  pixels by construction (golden scenes carry a synced registry or no
  Set Time knowledge — the new path is unreachable in both); the
  goldens verified with the EXACT CI flags. Tests +2 (workspace 262 →
  264 passed / 0 failed / 3 ignored): the skip-sync curve (overworld
  midnight/noon through the tag chain, overworld_caves same tag, the
  end's chain-miss → snapshot at the end's own ticks, the partial-sync
  no-rescue) and the expansion itself (nested order verbatim:
  villager_schedule first via #universal, then day/moon/early_game;
  in_end/in_nether single-member; unknown tag empty; the visited-set
  guard).
  The **biome tags have their data-side twin (v0.3.59 — the tag
  matrix completes)**: the worldgen emitter now parses the jar's
  `data/minecraft/tags/worldgen/biome/*.json` and emits `BIOME_TAGS`
  beside the timeline tags — the SAME raw form (concrete
  `minecraft:` members and nested `#`-references side by side;
  `has_structure/*` subdirectory stems carry their slash). The real
  26.2 surface: 68 tags, 390 members, and NOT ONE `replace` key
  anywhere — verified before a line of emitter code was written. The
  emission is appended LAST and the previous 31304-byte generated
  file verified as a BYTE-EXACT PREFIX of the new 44910-byte one
  (the prefix discipline survived a 68-entry table addition). The
  mc-registry join-pin: every concrete member names a biome the
  table carries, every nested ref names a tag the table carries —
  the join NEVER dangles (proven, not assumed: 390/390 resolved).
  This is the data half, the autopush-53 precedent: the runtime
  consumer (a skip-sync fallback for the retained-but-unread Update
  Tags biome tags) follows when a tag-driven lookup exists to serve;
  no golden scene can change — no render path touches a tag table.
  Tests +2 (workspace 264 → 266 passed / 0 failed / 3 ignored): the
  generator's parse/emission fixture (concrete + nested members, the
  strict prefix classification — a biome json is not a tag) and the
  registry's jar-values + never-dangling join.
  The **player skin joins the generated registry (v0.3.60 — the
  first-person hand's data half)**: a new generator arc reads the jar's
  `assets/minecraft/textures/entity/player/wide/steve.png` (the classic
  64×64 layout, 848 bytes, sha1-pinned) and emits
  `mc-registry/src/generated/skin.rs` — the integrity sha1s at BOTH
  layers (the verbatim source PNG and the generator's RGBA8 decode of
  it, independently cross-checked against a second decoder over all
  16 384 bytes) plus the vanilla `PlayerModel` arm-box geometry:
  texOffs (40, 16), box 4×12×4, the six faces unfolded exactly like
  vanilla `ModelPart`'s cube unwrap (top, bottom, then the side strip
  right → front → left → back), every rect pinned in-bounds. The
  PIXELS follow the font-texture pattern: the full regen pipeline now
  extracts the skin into the runtime asset root
  (`entity/player/wide/steve.png`) for the client to stream and
  sha1-verify at render time. The offline arc is a standalone
  subcommand (`mc2rs-gen skin --version 26.2 --jar <client.jar>`), the
  worldgen/pack-claim shape. This is the data half, the autopush-53
  precedent: the view-space arm pass (the first-person hand itself)
  consumes it next and is the round that regenerates goldens with its
  explanation; until then no render path touches the skin table and no
  golden scene can change. Tests +3 (workspace 266 → 269 passed / 0
  failed / 3 ignored): the generator's parse/emission fixture (painted
  arm faces, sha1 pins, in-bounds rects, the non-square refusal) and
  the registry's vanilla-unwrap pin (both sha1 layers, the face math,
  the bounds).
  The **first-person hand renders the real arm (v0.3.61 — the skin
  table's consumer)**: `mc-client/src/hand.rs` turns the generated skin
  table into the vanilla empty right-hand arm, and EVERY constant is
  26.2 bytecode evidence (Temurin JDK 21 + javap against the
  unobfuscated client jar, the sky/clouds recon's tooling):
  `ItemInHandRenderer.renderPlayerArm` gives the rest pose chain —
  translate(side·0.64000005, −0.6, −0.71999997) → rotY(side·45°) →
  translate(−side, 3.6, 3.5) → rotZ(side·120°) → rotX(200°) →
  rotY(side·−135°) → translate(side·5.6, 0, 0), the swing/equip terms
  collapsing to zero at rest — `GameRenderer.renderItemInHand` proves
  hand space IS view space (the camera rotation's inverse), 
  `AvatarRenderer.renderHand` gives the part's `zRot = 0.1` rad and the
  two-sided `entity_translucent` submit, `HumanoidModel.createMesh`
  gives the arm box `addBox(−3,−2,−2,4,12,4)` with pivot (−5,2,0) at
  texOffs (40,16), and `ModelPart$Cube`+`$Polygon` give the six faces'
  corner orders and UV-rect corners (including the UP face's
  v-flipped rect — caught by the literal UV pin before commit). The
  pixels stream from the runtime asset root and are sha1-verified
  against the generated table before a single texel is trusted; an
  absent or mismatched skin renders NOTHING (a render may never lie
  about its data). The pass runs after the level with a cleared depth
  buffer (vanilla's hand never clips into walls), no fog (the hand
  sits at the player's ~0 fog distance), vertex color = white (the
  player-position light is full skylight in the reference scenes).
  GOLDENS: 3 regenerated (gui_world, gui_worldnight, gui_pause — the
  FIRST-PERSON views, which now show the arm at the bottom-right; the
  vanilla-correct rule is that the hand renders ONLY from the player's
  own eyes) and 7 untouched: the cpu/net goldens are the orbit
  camera's detached world view — NOT first person — so no hand appears
  there, and title/servers/loading have no world at all. All 10
  verified at the exact CI flags, every one rms 0.000. Tests +3
  (workspace 269 → 272 passed / 0 failed / 3 ignored): the literal
  six-face UV unwrap pin (would have caught the v-flip), the
  refuse-unverified-skin gate, and the load-when-present probe.
  The **arm swings (v0.3.62 — the renderPlayerArm swing terms,
  decoded)**: the hand pass is no longer frozen at rest — the swing
  and equip parameters of `ItemInHandRenderer.renderPlayerArm` are
  now LIVE, from the same javap decode (slot 5 = swing, slot 4 =
  equip): the X wobble `side·(0.64000005 − 0.3·sin(√s·π))`, the Y bob
  `0.4·sin(√s·2π) − 0.6 + equip·−0.6`, the Z pull
  `−0.4·sin(s·π) − 0.71999997`, and the two swing rotations
  `rotY(side·sin(√s·π)·70°)`, `rotZ(side·sin(s²·π)·−20°)` slotted
  exactly where the bytecode puts them — after the constant 45° yaw,
  before the `translate(−side, 3.6, 3.5)` arm-flip. The TRIG function
  fidelity matters and is pinned: `Mth.sin`/`Mth.sqrt` compute in f64
  and narrow to f32 (their own bodies), so the client mirrors that
  instead of sinning in f32. The CLOCK is LivingEntity's, verbatim:
  `swing()` resets swingTime to **−1** with the no-retrigger guard
  (a restart is ignored while swingTime sits in [0, dur/2)),
  `updateSwingTime` advances one tick and resets to rest at the
  duration, `attackAnim = swingTime / duration`, and `getAttackAnim`
  is the o/attack lerp with the forward-wrap guard for the 1→0 jump —
  the duration itself is `SwingAnimation.DEFAULT = (WHACK, 6)`, the
  bipush 6 in its static init (the fist swing is 6 ticks). The
  renderer samples mid-tick (`partial 0.5`, the game camera's
  canonical partial); the left mouse press's rising edge in the world
  screen triggers `swing()` (the vanilla attack key; no attack while
  paused). The deterministic headless scenes never swing, so the
  rest pose is what the goldens always see — and a test proves the
  zero-swing pose is BIT-IDENTICAL to rest (every added rotation is
  the exact identity), so no golden moved. All 10 goldens verified at
  the exact CI flags, every one rms 0.000, none regenerated. Tests +5
  (workspace 272 → 277 passed / 0 failed / 3 ignored): the swing
  timeline (−1 → 0..5/6 → rest at exactly 6 ticks), the re-trigger
  guard, the wrap-guard lerp, the mid-swing geometry delta, and the
  bit-identical rest pose.
  The **held-item client consumer lands (v0.3.74 — the display arc's
  final wiring)**: the client now DRAWS a held item. The registry gains
  `display::posed_item_quads` — the generated-item bake quads through
  the resolved display pose (the bake's block-unit vertices are already
  the vanilla BakedQuad space: FaceBakery.bakeVertex divides the
  FaceInfo-selected corner by 16.0f immediately after VertexInfo.select,
  javap — and the first posed sword vertex is bit-pinned against the
  joml oracle's stdin mode: `3d4a3d70 3eba99c6 bec32df1`). The client
  gains `hand::HeldItem`: the quads ride the ITEM-IN-HAND ANCHOR chain,
  op-for-op from ItemInHandRenderer's bytecode — `applyItemArmTransform`
  `translate(side·0.56, −0.52, equip·−0.6 + −0.72)`, `swingArm`'s WHACK
  translate terms (exact zeros at rest), and
  `applyItemArmAttackTransform` `rotY(side·(45 + f5·−20)°) ·
  rotZ(side·f6·−20°) · rotX(f6·−80°) · rotY(side·−45°)` (the ±45° pair
  is composed every frame, never collapsed). Rendering replaces the arm
  exactly like vanilla's `submitArmWithItem`: a non-empty main hand
  draws the item INSTEAD of the empty-hand arm, after the clouds pass,
  with a cleared depth buffer. The texels stream from the generated
  texture namespace through `sprite_uv::sample_uv` (embedded, sha-pinned
  — no runtime file), rasterized by the new `raster::draw_sampled_quads`
  (same clip / perspective-correct uv / depth conventions as the arm
  path, which stays bit-frozen; the alpha<128 cutout rule applies to
  the sampler output). The item is chosen via `--held-item <model_id>`
  (e.g. `item/iron_sword`) — the deterministic stand-in for inventory
  state; the default scenes hold NOTHING, so the goldens never see it.
  All 10 goldens verified at the exact CI flags, every one rms 0.000,
  none regenerated. Tests +6 (workspace 353 → 359 passed / 0 failed /
  3 ignored): the posed-item composition (the oracle pin, the uv/texture
  ride, the domain refusals), the anchored rest region (every corner in
  front of the camera, right of center, the bulk below — the blade tip
  quads cross the midline like the real held sword), the swing geometry
  delta, the client pixel draw (right half only), and the sampler
  cutout rule. Also repairs a pre-existing doctest break (an unannotated
  pseudo-code fence in the item_bake docs compiled as Rust and failed
  `cargo test --workspace` on the clean tree).
  The **palette consumer lands (v0.3.78 — the world's flat colors come
  from the real chain)**: the mesher's flat-color path now reads
  `colors::palette_color` FIRST — the blockstate → model → texture
  chain of v0.3.77 — and demotes the runtime-texture LUT to the first
  fallback (the hand-written `default_color` guesses fall to last). The
  chain walk is far too heavy for the mesh hot path, so the consumer
  computes the table ONCE (a `OnceLock` over the world's NAME_OF kinds
  × 6 faces, the procedural world's canonical states: grass_block
  snowy=false, the logs axis=y, leaf_litter facing=north +
  segment_amount=1) and then indexes it per quad; the mesher's face
  order [east, west, top, bottom, south, north] maps onto the palette's
  Cull slots [East, West, Up, Down, South, North]. **The goldens did
  NOT move — by data, not by luck**: the generated atlas covers 25 of
  the 26 world kinds on every face (only air is TILE_NONE, and air is
  never rendered), so every golden pixel rides the TEXTURED path and
  the flat-color path this round rewires executes on NO golden scene —
  the full matrix re-verified 12/12 at rms 0.000, NONE regenerated.
  The rewire's real effect is the chain of custody: any face that
  loses its atlas tile (a future kind, a non-atlas view) now gets its
  color from the same evidence chain every other consumer uses, not
  from a hand-typed guess. The leaf-litter special path keeps its
  LUT/defaults (its four variant models need a variant→model mapping —
  banked with the biome-tint work). Tests +2 (workspace 365 → 367
  passed / 0 failed / 3 ignored): the flat colors ARE the chain's
  numbers (stone (126,126,126) on all six faces, grass top
  (147,147,147), the grass sides the (141,131,111) two-quad compose,
  grass bottom dirt), and the precedence (the palette beats the old
  hand-written grass side; water/air stay None for the old chain).
  The **blockstate-driven palette lands (v0.3.77 — the world palette's
  data half)**: `crates/mc-registry/src/palette.rs` turns a block state
  into the per-face average color of its REAL model's REAL textures —
  the data-driven replacement for hand-written color guessing, assembled
  from the existing census-validated chain: BLOCKSTATES (v0.3.62) →
  `blockstate_lookup`'s vanilla selection semantics (a new by-value
  `select_applies` reusing the same key/when matchers — first-match
  variants, when-matching multipart collect) → the child's merged
  `texture_slots` (v0.3.69) + the first parent-chain ancestor carrying
  ELEMENTS (the vanilla element inheritance — `block/stone` has no
  elements of its own, `block/cube_all` does) → `element_quads` (v0.3.68)
  → the generated jar texels (v0.3.70). Semantics pinned: a quad's color
  is the average of the OPAQUE pixels (alpha != 0, vanilla's rule) of
  its uv RECT (rotation-invariant min/max, scaled to the texture's real
  size); a face is the equal-weight average of its quads, keyed by the
  model's OWN slot (the blockstate x/y rotations remap at the consumer);
  tinted faces average their RAW texels (the biome tint multiplies at
  the consumer); unresolvable refs and missingno materials contribute
  nothing; unknown blocks fail closed to None. Averages round HALF-UP,
  the same convention as the independent oracle. The oracle is a
  hand-written python PNG reader over the real jar
  (`scripts/palette_oracle.py`): it pins stone `(126,126,126)`,
  dirt `(134,96,67)`, grass_block_top `(147,147,147)` — and it caught
  TWO fidelity facts the hard way: the grass side is TWO quads (#side +
  the #overlay fringe, whose 135-byte grayscale png carries a tRNS
  chunk — transparent-gray gaps the oracle must honor or the average
  reads (27,27,27) instead of (155,155,155) over the 45 opaque pixels;
  composed side = (141,131,111)), and 26.2's snowy grass keeps
  `grass_block_top` on top (the snow is a runtime tint, not texels) with
  the snowy-side TEXTURE on the sides (170,151,133). WATER is honestly
  absent — its blockstate points at a particle-only template (no
  elements, no parent; the fluids are vanilla's special render path).
  GOLDENS: untouched — zero render-path contact (the mesher consumes
  this next round). Tests +4 (workspace 361 → 365 passed / 0 failed /
  3 ignored): the stone oracle gray on all six faces, the grass
  three-texture split, the snowy state selector + the fail-closed
  edges, and the world-blocks census (24 of the 26 NAME_OF ids resolve;
  air and water are the honest None cases).
  The **swing chain joins the golden matrix (v0.3.76 — the swing clock
  gets its deterministic stand-in)**: the WHACK swing arc was fully
  implemented since v0.3.73 (the `SwingState` timeline, the
  `renderPlayerArm` / `swingArm` / `applyItemArmAttackTransform` terms
  in the anchor chains) but the deterministic scenes could never EXERCISE
  it — `--held-item` had a flag, the swing clock did not. The client
  gains `--swing-progress <f>` (getAttackAnim's domain [0, 1]): the
  deterministic stand-in for the swing clock (the same role
  `--held-item` plays for inventory state — the headless client has no
  gameplay input). `SwingState::frozen_at(progress)` carries the
  progress in BOTH `o_attack_anim` and `attack_anim`, so
  `get_attack_anim(partial)` returns it exactly at every partial tick —
  the vanilla lerp is the identity on a no-change frame, a legal
  timeline slice; the world screens render the mid-swing pose of the
  held item (or the empty-hand arm), and the non-ticking headless loop
  holds the state stable. `goldens/gui_world_held_swing_320x180.png` is
  the TWELFTH golden — the world scene with the iron sword mid-swing at
  progress 0.5 (`--screen world --ui-frame 30 --seed 20260214
  --held-item item/iron_sword --swing-progress 0.5`, 320x180), gating at
  max-rms 0: the WHACK translate terms and the rotY(45 + f5·−20)°
  composite are now pinned AT a nonzero progress, not just at rest.
  Proven the project way before pinning: two independent renders
  SHA-IDENTICAL (`71140a74…`), then diffed against the rest-pose held
  golden — rms 30.786, 8.11% of pixels changed (the swung sword's screen
  region). GOLDENS: one ADDED (`gui_world_held_swing_320x180`), none
  regenerated — the full matrix re-verified through `verify_goldens.sh`,
  12/12 pass, every one rms 0.000. Tests +2 (workspace 359 → 361 passed
  / 0 failed / 3 ignored): the frozen stand-in samples its exact
  progress at every partial tick (and frozen 0 IS the rest sample), and
  the swing scene's pixel-level delta against the rest scene (the bulk
  right of center; a few pixels flip left — the blade tip already
  crosses the midline at rest).
  The **held item joins the golden matrix (v0.3.75 — persistent pixel
  coverage for the display arc's render path)**:
  `goldens/gui_world_held_320x180.png` is the ELEVENTH golden — the world
  scene with the iron sword in hand (`--screen world --ui-frame 30 --seed
  20260214 --held-item item/iron_sword`, 320x180), gating at max-rms 0
  like its six GUI siblings: the whole first-person stack (the posed
  item-model bake → display pose → hand anchor → `draw_sampled_quads`
  with the alpha<128 cutout) is now pinned pixel-for-pixel, so the arc's
  render path can no longer drift silently. Proven deterministic the
  project way before pinning: two independent renders SHA-IDENTICAL
  (`5274d725…`), then diffed against the plain world golden — rms 30.338,
  9.45% of pixels changed (the sword's screen region), i.e. the scene
  really exercises the new path and is not a copy of `gui_world`. The
  second half of the round closes a documentation gap the README itself
  exposed: `verify_goldens.sh` has been referenced since v0.3.43 but the
  file never landed — it now exists at the repo root and IS the golden
  gate: all eleven scenes re-rendered at their exact recorded flags
  (the same flags every worklog round verified with), each diffed
  through mc2rs-pixel, CPU/NET gated at max-rms 8 and the seven GUI
  screens at max-rms 0, one command, strict exit code, no automatic
  regeneration (a golden update stays a deliberate, documented act).
  GOLDENS: one ADDED (`gui_world_held_320x180`), none regenerated — the
  full matrix re-verified through the new script, 11/11 pass, every one
  rms 0.000. Tests +0 (the render path itself is unchanged — this round
  pins it; workspace 359 passed / 0 failed / 3 ignored).
  The **item-model bake lands (v0.3.73 — the held-item geometry link)**:
  `crates/mc-registry/src/item_bake.rs` derives the generated item
  models' quads — the sprite extrusion of 26.2's
  net.minecraft.client.resources.model.cuboid.ItemModelGenerator — on
  demand over the generated textures table (no new table; the same
  lazy-derive pattern as element_quads). Every rule is javap evidence:
  the bake domain is the models whose parent chain contains
  item/generated (1231 of 3928; item/generated's own parent is the
  builtin/generated GENERATED_ITEM_MODEL_ID); LAYERS = layer0..layer4
  resolved through the texture_slots chain merge with the bake() BREAK
  at the first null material (layers must be contiguous from layer0 —
  9 domain models bake EMPTY: item/spear_in_hand, item/handheld,
  item/generated, the template_bundle_open_* sides, item/handheld_mace,
  item/handheld_rod, item/template_music_disc, item/amethyst_bud);
  MaterialInfo.of pins tintIndex = the LAYER INDEX, shade = true,
  lightEmission = 0; SpriteContents.isTransparent = ARGB.alpha == 0
  (any nonzero alpha is opaque) with a MISSING texture baking from the
  missingno checkerboard (16x16, fully opaque — 64 border edges); the
  front/back are ONE unculled quad each over the whole sprite, from
  (0,0,7.5) to (16,16,8.5) (MIN_Z/MAX_Z), SOUTH with UVs(0,0,16,16) and
  NORTH with the mirrored UVs(16,0,0,16); the sides record
  (SideDirection, x, y) per opaque pixel whose NEIGHBOR
  (x - stepX, y - stepY; out of bounds counts transparent) is
  transparent — the 26.2 enum carries UP/DOWN/LEFT->EAST/RIGHT->WEST —
  and each recorded edge emits ONE quad (NO pixel-run merging:
  SideFace is a Record(facing, x, y)) with UV_SHRINK = 0.1f insets
  (f32 literals: 13.0f+0.1f and 7.0f+0.1f round to DIFFERENT bits —
  0x4151999a vs 0x40e33333, both pinned), the V REVERSED for the
  UP/DOWN sides (minV = y+1-0.1 > maxV = y+0.1), the from/to slabs
  scaled by 16/w and 16/h (the whole sprites like the 32x32 spears),
  the 16-y model flip, and per-side z from 7.5 to 8.5. The step vectors
  were the round's bug-catch: Direction.UP.stepY = +1 makes the UP
  neighbor (x, y-1) — an inverted reading mislabels every side quad
  onto the opposite edge (same counts, wrong geometry) — the oracle
  and the port were both corrected before pinning. The bit-exact
  oracle: scripts/item_bake_census.py bakes the jar's real PNG pixels
  (PIL, f32-strict via struct). The census: 1231-model domain, layer
  presence [1222, 332, 44, 0, 0] (contiguous), 64 distinct missing
  layer textures (all trims/items/), exactly ONE animated layer texture
  (block/sculk_vein via item/sculk_vein — the frame-0 approximation is
  documented; vanilla unions the exposed edges over all frames but the
  textures table stores the first-frame crop), 102106 total baked quads
  over the domain, fixtures iron_sword 82 / bow 98 / copper_torch 26
  (the minimum). The vanilla facing drift is pinned too: 40 of
  iron_sword's 82 quads report the flipped direction (the inverted
  from/to slabs wind backwards — the same calculateFacing phenomenon
  the element census counted 273 of). The tests reuse element_quads'
  census-validated bakeQuad machinery (faceinfo_corners, vertex_uvs,
  calculate_facing, recalculate_winding, now pub(crate)). Zero
  render-path contact (mc-client carries zero references); all 10
  goldens verified at the exact CI flags, every one rms 0.000, none
  regenerated. Tests +5 (workspace 348 -> 353 passed / 0 failed /
  3 ignored; mc-registry 62 -> 67): the iron_sword extrusion (the
  whole-sprite faces' uv walk, the UP/LEFT first edges, the reversed V,
  the drift), copper_torch + bow, the layerless empty bakes, the
  missingno path (132 quads, the missingno tint layer), and the
  registry-wide census.
  The **display-transform link lands (v0.3.72 — the held-item view-space
  pose, the display arc's last pure-data link)**:
  `crates/mc-registry/src/display.rs` resolves the models table's
  `display` stanzas per context through the parent chain into the posed
  vertices over the quads. Every rule is javap evidence from the 26.2
  jar + the bundled joml-1.10.8: the ItemTransform deserializer divides
  translation by 16 (mul 0.0625f) THEN clamps to ±5 (MAX_TRANSLATION),
  clamps scale to ±4 (MAX_SCALE), refuses rotation/translation/scale
  arrays that are not exactly 3 floats, and defaults (0,0,0)/(0,0,0)/
  (1,1,1) — an explicit empty context {} is a REAL transform that blocks
  the parent; ItemTransforms$Deserializer reads exactly the nine
  ItemDisplayContext serial names (on_shelf included — the container's
  ninth field is Mojang-named fixedFromBottom but receives ON_SHELF; NO
  legacy thirdperson/firstperson keys) and replaces an absent left-hand
  context with the right-hand transform AT PARSE TIME; 26.2's
  ResolvedModel.findTopTransform resolves PER CONTEXT — walk the parent
  chain, the nearest stanza declaring the context wins (1.20's wholesale
  inheritance is GONE: block/stairs owns gui [30,135,0] but still
  inherits block/block's firstperson [0,45,0]); ItemTransform.apply on a
  fresh identity pose is translate(±tx, ty, tz) → ONE
  Quaternionf.rotationXYZ(rx·DEG, ±ry·DEG, ±rz·DEG) → scale(sx, sy, sz)
  with NO left-hand flip → translate(−0.5×3) — the 26.2 mirroring
  negates ONLY translation.x, rotation.y and rotation.z (rotation.x and
  scale are raw), the NO_TRANSFORM branch is the bare −0.5 centering,
  and DEG = 0.017453292f is multiplied in f32 before rotationXYZ. The
  JOML paths actually taken are mirrored op-for-op: translate-on-identity
  builds T; rotateTranslation ASSIGNS the rotation matrix wholesale (the
  f19..f27 forms verbatim); scaleGeneric scales the FULL columns 0..2
  (0.0·(−4.0) = −0.0 propagates — the oracle pins those bits); the tail
  translate and Vector3f.mulPosition are the NESTED PLAIN fma
  (org.joml.Math.fma falls back to a·b+c because the vanilla client
  never sets joml.useMathFMA — Runtime.HAS_Math_fma=false); rotationXYZ
  is f64-trig-then-f32-store (org.joml.Math.sin = (float)
  Math.sin((double) x) with the cosFromSin sqrt+wrap sign fix) — which
  is why the 180° on_shelf rotation is EXACT ±1/0. The bit-exact oracle:
  scripts/display_ref/DisplayRef.java runs the REAL joml-1.10.8
  op-for-op and prints Float.floatToIntBits hex; the tests pin five
  fixtures (iron_sword firstperson_righthand, block/block 45°-zone gui +
  firstperson + on_shelf, a synthetic clamp exercise) × both hands, 16
  matrix entries each, plus 42 vertex probes and the e2e (block/stairs'
  quads through its gui pose vs the harness fed the same vertex) — every
  bit equal. The 26.2 census (scripts/display_census.py): 3928 models,
  66 with their own display stanza (zero legacy keys, zero clamp hits —
  max |translation|/16 = 1.40625, max |scale| = 2.0), 483 explicit
  45-multiple rotation entries + 14 no-rotation contexts (the [0,0,0]
  default) = 525 effective; per-context chain resolution over all 3928
  models: [2622, 2622, 2618, 2618, 1386, 1431, 2621, 2621, 1357]; the
  left-hand wins split declared-vs-parse-fallback 141+2481
  (thirdperson) and 1344+1274 (firstperson). posed_quads(model,
  context) composes the whole arc: element_quads → the resolved pose →
  block-unit vertices. Zero render-path contact (mc-client carries zero
  references to the module); all 10 goldens verified at the exact CI
  flags, every one rms 0.000, none regenerated. Tests +14 (workspace
  334 → 348 passed / 0 failed / 3 ignored; mc-registry 48 → 62): the
  deserializer semantics (handheld stanza verbatim, the clamp order,
  the empty-context block, the fail-closed refusals), the oracle
  matrices + vertices, NO_TRANSFORM, the per-context chain walk
  (stairs/fence/sword discriminators), the left-hand slot map, and the
  registry-wide census + chain-resolution + attribution pins.
  The **sprite-uv consumer lands (v0.3.71 — the held-item pixel link)**:
  `crates/mc-registry/src/sprite_uv.rs` maps the quads' 0..16 uv space
  to texels over the texture namespace, composing the display arc's
  three data layers in one call (element_quads' quads → texture_slots'
  materials → generated::textures' pixels). Every rule is javap
  evidence from the 26.2 jar: CuboidFace.getU/getV divides the
  quadrant-rotated 0..16 uv by 16.0f into the per-sprite [0,1] domain
  (mirrored rects ride as the raw floats); FaceBakery.bakeVertex feeds
  that normalized uv DIRECTLY to TextureAtlasSprite.getU — the
  non-identity branch rotates the uv around (0.5, 0.5) by
  ModelState.inverseFaceTransformation, the blockstate carrier that is
  identity for standalone element models; TextureAtlasSprite.getU(u) =
  u0 + (u1−u0)·u over the atlas-normalized span, and a standalone
  per-id texture is the whole image so the mapping is the EXACT f32
  identity (the /16 and ·16 are exponent shifts — no rounding either
  way); UVPair.pack is Float.floatToIntBits — bit-exact, no
  quantization; the atlas sampler is
  SamplerCache.getClampToEdge(FilterMode.NEAREST) — NEAREST +
  CLAMP_TO_EDGE, texel = clamp(floor(u·size), 0, size−1): u = 1.0 lands
  on the LAST texel (no REPEAT wrap) and mirrored/negative uv clamps to
  the edge; the missing material is the literal `missingno` sprite —
  generateMissingImage() 16×16, magenta −524040 = 0xFFF800F8 → RGBA
  (248, 0, 248, 255), black −16777216 → (0, 0, 0, 255), the pattern
  (y < h/2) XOR (x < w/2) → magenta (the top-right and bottom-left
  quadrants). The namespace join strips exactly one leading
  `minecraft:` (the slots' 10287 namespaced vs 807 bare values land on
  one lookup). `sample_material(model, ref, u, v)` returns None for the
  dangling template refs — vanilla's warn+drop. The tint is NOT applied
  (the tintindex multiply is the draw-time step; grass_block_top
  returns the raw grayscale master, pinned ≠ the atlas bake). The e2e
  pin derives block/heavy_core's up/down quads, resolves the bare "all"
  ref, and samples PIL-pinned texels including the CLAMP boundary:
  uv (16, 0) → texel (15, 0) = (49, 52, 54, 255). Zero render-path
  contact (mc-client carries zero references to the module); all 10
  goldens verified at the exact CI flags, every one rms 0.000, none
  regenerated. Tests +8 (workspace 326 → 334 passed / 0 failed /
  3 ignored; mc-registry 40 → 48): the bit-exact normalize/map pins,
  the clamp table (u = 1.0 → size−1, NaN fail-safe, 1×1), the
  checkerboard XOR, the heavy_core e2e, the missingno + trim sampling,
  the namespace forms, the raw-master tint boundary, and the
  sample_material composition (bare + namespaced + dangling + unknown).
  The **texture namespace lands (v0.3.70 — the atlas-mapping round's
  data half: texture id → pixels)**: `mc2rs-gen textures` walks every
  model file the 26.2 jar ships (3928, strict block/|item/ prefixes),
  collects every concrete texture id the textures maps reference
  (including the 26.2 inline sprite dicts' "sprite" fields), and decodes
  each one to RAW RGBA8 into `mc-registry/src/generated/textures.rs`
  (7.1 MB): 2113 ids — 1252 block/ + 796 item/ + 64 trims/ + the literal
  `missingno` (item/air.json and block/air.json reference it) — of which
  2048 exist (2,241,536 texels: 2001 square 16×16 + 47 whole 32×32
  sprites: hanging signs, signs, shelves, dried_ghast tentacles, the
  seven spears in hand) and 65 are MISSING (the trims/items armor-trim
  family + missingno — width=height=0, an empty texel span; the consumer
  renders the vanilla checkerboard). The animation gate is MCMETA-FAITHFUL:
  a texture is cropped to its first w×w frame ONLY when a sibling
  .png.mcmeta carries "animation" (the census pins all 52 h>w domain
  textures as mcmeta animations — so the crop is ALSO unconditional-crop
  faithful on real data; zero w>h exist; a tall texture without mcmeta
  stays whole, the vanilla rule). Pixels are RAW — NO tint is baked
  (tintindex rides the quads; the consumer multiplies at draw time; the
  WORLD atlas bakes because its render path has no per-face tint pass
  yet — the tint-boundary pair is pinned: stone is byte-identical across
  the two tables, grass_block_top differs by exactly the 0x91BD59 bake).
  The census ran first (scripts/atlas_domain_census.py +
  atlas_domain_census2.py) and the Rust walk reproduces every number
  (2113/2048/65/52 crops/texels); reference pixels are pinned against an
  independent PIL decode (stone's corners/center, iron_sword's
  transparency, water_still's alpha 180). The domain is the models'
  domain: the 57 trims/color_palettes PNGs exist but are unreferenced and
  stay OUT. Test builds carry no debuginfo now ([profile.test] debug = 0
  in the root Cargo.toml — rustc's debuginfo over 2.24M array literals
  OOMs 4 GB runners mid-link; release never carried it). Zero
  render-path contact (mc-client carries zero references to the new
  table; the 5 crate-internal `textures::` hits are the pre-existing
  TextureLut module); all 10 goldens verified at the exact CI flags,
  every one rms 0.000, none regenerated. Tests +10 (workspace 316 → 326
  passed / 0 failed / 3 ignored; mc-registry 34 → 40, mc2rs-gen +4): the
  sorted binary-searchable namespace (a hit is NOT existence — missingno
  hits and is missing), the per-tile span invariants (2048 = 2001×16² +
  47×32², offsets tile 2,241,536 texels exactly), the cross-table stone
  pin, the tint-boundary pair, the mcmeta crop gate, the sprite-dict
  refusal, and the missing census.
  The **texture-slots resolution lands (v0.3.69 — the quad's texture ref
  becomes a concrete id)**: `crates/mc-registry/src/texture_slots.rs`
  resolves every model's texture slots through the parent chain over the
  MODELS table — zero dependencies, zero render-path contact (mc-client
  carries no references; the goldens are safe by construction AND by
  measurement). Every rule is javap evidence from the 26.2 jar
  (TextureSlots$Resolver/parseEntry/getMaterial): the textures-map values
  split at parse time into Values (full ids as written — 10287 carry
  "minecraft:", 807 are bare paths like heavy_core's "block/heavy_core")
  and References ("#var", '#' stripped); the 26.2 inline sprite dict
  {"force_translucent": true, "sprite": ...} (163 carriers, the flag
  ALWAYS true) is a Value over the sprite field; Resolver.resolve walks
  Lists.reverse(entries) — root FIRST, child LAST — so the child
  OVERRIDES the ancestors PER KEY (a Value removes a prior reference and
  vice versa); the reference FIXPOINT loops one hop per pass in the
  vanilla do-while shape (the futile scan of a dangling-only model
  counts — census ledger 2063/1857/8, max 2 passes through the furnace
  family's bottom->particle->front chains); leftover references are
  warn'ed and DROPPED (exactly 258 (model, slot, target) triples, all in
  the TEMPLATE models whose children fill the slots when baked —
  block/button's {"particle": "#texture"} is the textbook case);
  getMaterial strips a leading '#' and looks the key up — BARE refs
  behave identically (heavy_core's six face refs). The census
  (scripts/texture_slots_census.py) walked all 3928 models: chain depths
  {0:302, 1:2256, 2:874, 3:480, 4:16} (all 3460 parents
  "minecraft:"-prefixed, builtins anchor), 3883 models with textures,
  11094 resolved slots, overrides {0:3740, 1:110, 3:16, 4:51, 5:9, 8:2},
  and the face-ref join over the 344 element models: 2623 of the 4238
  refs resolve at the model's own level, 1615 dangle by design.
  Tests +7 (workspace 309 -> 316 passed / 0 failed / 3 ignored): the
  button template vs its filled child, the acacia_shelf override, the
  furnace fixpoint, bare-vs-hash ref identity, the sprite dict, the
  lookup misses, and the registry-wide census pin. All 10 goldens
  verified at the exact CI flags, every one rms 0.000, none regenerated.
  The **vanilla FaceBakery quad derivation lands (v0.3.68 — the
  element-geometry consumer, the bridge to block/item display)**:
  `crates/mc-registry/src/element_quads.rs` turns the typed elements
  table (v0.3.65) into quads as plain data — zero dependencies, zero
  render-path contact (mc-client carries no references; the goldens
  are safe by construction AND by measurement). Every formula is
  javap evidence from the 26.2 client jar
  (net.minecraft.client.resources.model.cuboid.*): the FaceInfo
  corner table (Extent MIN->from / MAX->to, the 6-arg bbox select),
  the absent-uv bounds projection (defaultFaceUV per direction), the
  vertex-uv quadrant walk ((i + rotation/90) % 4 over
  (minU,minV)(minU,maxV)(maxU,maxV)(maxU,minV) — mirrored rects
  legal), the rotation forms (single-axis Rodrigues with angle==0 ->
  identity; euler = JOML rotationZYX(z,y,x) = Rz*Ry*Rx, X applied
  first; rescale = diag(1/max|M*unit|) per axis, POST-multiplied —
  the vector scales FIRST, then rotates), the origin semantics (the
  deserializer stores origin *0.0625 in BLOCK units while the
  generated table keeps pixels — the consumer divides by 16), the
  computed facing ((v1-v0) x (v2-v0), argmax positive dot in the
  vanilla enum order, requireNonNullElse(facing, UP) for the
  degenerate zero-area quads), and the winding recalculation (ONLY
  when rotation is absent and the facing resolved; canonical corners
  from the quad's own bbox; exact float equality; vertices AND uvs
  swap together). The bytecode homework is banked: heavy_core's bare
  texture refs resolve IDENTICALLY to "#var" refs
  (TextureSlots.getMaterial strips the leading '#' and looks the
  string up — the slots merge is the texture round), and the
  from/to domain is REFUSED outside [-16, 32], never clamped. The
  census discipline caught a real trap: the first python draft ran
  f64 math (drift 273) and an unfaithful f64-trig draft (356) — the
  JOML-faithful f32 census (scripts/quads_census.py, per-op f32
  rounding, f64-trig-then-f32-store) agrees with this implementation
  PER MODEL, all 344 lines: 4238 quads (every face exactly one),
  362 facing drifts, 6 rewound quads (the inner-faces cubes'
  reversed from/to [15.998..]->[0.002..] mirror trick), 160 UP
  fallbacks (the mangrove propagule's collinear strands), rotation
  forms 882 none / 301 axis / 2 euler, 97 rescale carriers (all
  axis-form), face-rotation histogram 3993/151/39/55. Tests +9
  (workspace 300 -> 309 passed / 0 failed / 3 ignored): the
  canonical cube quads bit-exact, the six default-uv projections,
  the uv quadrants, the axis+euler forms against the census (the
  euler carrier's north quad lands facing EAST — the
  45-degree-diagonal tie), the rescale fixed point, the inner-faces
  rewind, the degenerate UP fallback, and the registry-wide census
  pin. All 10 goldens verified at the exact CI flags, every one
  rms 0.000, none regenerated.
  The **blockstate consumer's selection layer lands (v0.3.67 — the
  when-condition homework, the state-id → model+rotation link)**:
  `crates/mc-registry/src/blockstate_lookup.rs` is the hand-written
  consumer half of the blockstates table — a strict zero-dependency
  JSON-subset parser for both canonical columns plus the vanilla
  selection semantics, the module the mesher's palette will call. A
  fresh independent census of the real jar
  (scripts/blockstate_when_census.py) ran BEFORE the evaluator code
  and pinned the consumption shapes: variant keys are always the
  block's FULL property set in BLOCKS order (**7296 keys** over 1096
  files, **549 empty**, zero pipes, zero charset violations); when
  values are strings only (1874 plain + **32 pipe-carriers** — exactly
  4 distinct "a|b" sets, max 3: side|up, 2|3|4, 3|4, 2|3 across
  leaf_litter/pink_petals/redstone_wire/wildflowers); the OR/AND
  wrappers NEVER nest (13 OR / 768 AND map elements); weight rides
  only on chorus_plant's six noside applies (always 2); no when map
  is empty. The parser accepts the semantically-defined supersets the
  census merely didn't refute (an empty when map = always-true,
  recursive wrappers) and refuses everything MEANINGLESS: unknown
  apply/part keys, non-string when values, string escapes, floats,
  duplicate keys, empty apply lists, rotations off the 0/90/180/270
  domain, a wrapper sharing its object with plain keys. Selection
  semantics: variants pick the FIRST key in file order whose pairs
  all match — on full-set keys this equals exact-key lookup, and BOTH
  are pinned (the registry-wide test walks every one of the 7296 keys
  and asserts it selects ITSELF); multipart collects EVERY part whose
  when matches, in part order; a pipe value matches if the state's
  value is any alternative; a property absent from the state fails
  fail-closed; unmentioned state properties are wildcards. The spot
  selections prove the fun facts from the table itself:
  redstone_wire's east=up+north=side stacks dot+side0+side_alt1+up
  (the OR wrapper's five maps working), leaf_litter segment 3 stacks
  BOTH the "2|3" and the exact-"3" models (vanilla multipart
  stacking), acacia_shelf's AND chains resolve the unpowered/left
  variants. Tests +5 (workspace 295 → 300 passed / 0 failed / 3
  ignored): the column split + lookups, the variants first-match
  (rotations, arrays, wildcards, the empty-key any-state match), the
  multipart OR/AND/pipe battery, the registry-wide census walk
  (7296/549/1574/13/768/32/6 pinned in Rust against BLOCKS' property
  sets), and the 25-shape refusal battery. Zero render path touches
  the module (mc-client carries no reference — by construction) and
  the goldens confirm by measurement: cpu 320, net 320 and gui world
  all rms 0.000 at the exact CI flags, none regenerated. The mesher
  wiring (state-id → BLOCKSTATES → model+rotation for the world
  palette) consumes this layer next.
  The **blockstates land (v0.3.66 — the world-render chain's data link,
  the models round's sibling table)**: the generator grew its fifteenth
  table — `assets/minecraft/blockstates/` of the official jar parsed
  into `generated/blockstates.rs`, **1198 files** split EXACTLY
  **1096 `variants` / 102 `multipart`** (never both, never neither),
  each retained as **value-faithful canonical compact JSON** (the
  models round's texture/display-stanza precedent). The census ran
  first — twice — and the real-jar run REFUSED twice, the
  extend-parser-first discipline catching the census's own blind
  spots: **the empty state key is a real vanilla shape**
  (`acacia_leaves` and all 549 one-variant property-less blocks ship
  `{"": {…}}`), and **the when conditions have an AND wrapper too** —
  `{"AND": [maps]}` ×288 (acacia_shelf…) beside `{"OR": […]}`
  ×5 (fire ×4, redstone_wire ×1), never mixed with plain keys, never
  nested. The closed sets: the apply 5-key set
  {model, x, y, uvlock, weight} over **8661 applies**; rotations only
  0/90/180/270 (0 never written — absent means 0); uvlock only ever
  true; weight on 2 applies (value 2); parts carry {apply} (46
  unconditional) or {apply, when} (1086); apply LISTS (19 multipart
  parts + the 56 weighted/equal-random variant arrays — grass_block's
  four y-rotations, turtle_egg's twelve) ride in jar order; max 64
  variants (repeater) / 52 parts (chiseled_bookshelf). The model-ref
  join is PERFECT: all **2380 distinct refs** resolve to model files,
  0 dangling — and the census's own 2363 was WRONG (a python
  loop-variable leak skipped every apply-list member but the last);
  the independent re-derivation caught its own census, the third such
  self-catch in the project's history. 4 vanilla refs ship WITHOUT a
  namespace (item_frame ×2, glow_item_frame ×2 — the map/state
  variants) and ride as written beside the 8657 `minecraft:` ones.
  Offline arc: `mc2rs-gen blockstates --version 26.2 --jar
  <client.jar>` + `write_blockstates()` in the regen pipeline after
  write_models. Tests +4 (workspace 291 → 295 passed / 0 failed / 3
  ignored): the generator's both-shapes parse fixture (weighted
  arrays, uvlock, x/y, the bare ref, the AND/OR wrappers, the
  when-absent part, the apply list), the unknown-key/doubled-shape
  refusals, the bad-domain refusals (foreign namespace, y=45,
  negative rotation, zero weight, OR mixed with plain keys,
  non-string when values) and the registry census pin (the
  1198/1096/102/8661/2380 counts, the bare refs, the
  grass_block/leaf_litter/redstone_wire/acacia_shelf spot fragments,
  the repeater 64, and the join both ways — every ref resolves into
  MODELS, every id is a block in BLOCKS). Zero render path touches
  the table yet; goldens checked at the CI flags (rms 0.000), none
  regenerated. The consumer round (state-id → model + rotation, the
  mesher's palette going data-driven) comes later — the 14th audit's
  hand-coded leaf_litter coverage is the shape of what this table
  will generalize.
  The **elements geometry lands (v0.3.65 — the model-geometry round, the
  held-item arc's data half, round 3)**: the honest skip list is paid
  off — the 344 element-carrying models' `elements` arrays are parsed
  into `generated/elements.rs` as TYPED rows (1185 elements / 4238
  faces), and the census ran first — twice: the second pass locked the
  shapes the first surfaced, and it caught THREE format facts the
  common case hides. **26.2 ships a euler rotation extension**: two
  elements of block/template_hanging_sign_rot_3 carry `{x,y,z,origin}`
  instead of the classic single-axis `{angle,axis,origin,rescale}` —
  both forms parse into an enum now, mixing refuses. **The angle domain
  is relaxed**: the old {−45,−22.5,0,22.5,45} five-value set is gone —
  −90 (×2), −67.5 (×7) and 0.0001 (×1) ship, so the angle rides as an
  f32 as written. **Bare texture refs exist**: heavy_core's six faces
  reference `"all"` without the leading `#` — the face texture column
  stores the ref AS WRITTEN (the runtime resolution is the consumer
  round's bytecode homework). The closed key sets (element: 8 incl.
  `light_emission` and the editor metadata `name`/`__comment`, both
  retained — 94 names / 63 comments; face: 5 incl. the uv `rotation`,
  246 carriers at {0×1, 90×151, 180×39, 270×55}) refuse anything else,
  extend-parser-first. Each element row carries from/to f32 bounds
  (verified inside the vanilla clamp domain [−16, 32]; the jar uses
  [−8, 24]), shade (default true; 333 explicit false), light_emission
  (exactly the four emissive-cross elements at 15), the rotation enum,
  and the six face slots in vanilla Direction order — uv (None = the
  vanilla bounds projection, 531 defaults), texture ref, cullface
  (1041), tintindex ({0×90, 1×32}) and the uv rotation. The emit is a
  join-checked triple: `ELEMENTS` rows ↔ `ELEMENT_MODELS` ids ↔ the
  MODELS element-count column, id-for-id both directions. Offline arc:
  `write_models()` now writes both files from one parse
  (`mc2rs-gen models --version 26.2 --jar <client.jar>`). Tests +4
  (workspace 287 → 291 passed / 0 failed / 3 ignored): three generator
  fixture tests (every outlier shape in one parse — euler, bare ref,
  light_emission, editor metadata, the axis-rotation + rescale +
  tintindex + uv-rotation combo; unknown element/face/rotation-key
  refusals incl. the mixed rotation forms; bad cullface/direction/
  bounds/light-emission/rotation-quantum refusals) and the registry
  census pin (the 344/1185/4238 counts, block/cube verbatim, the euler
  rotations verbatim, every closed-domain scan, the angle histogram,
  and the joins). The quad derivation — rotation math, shade normals,
  default-UV expansion, texture-ref resolution — stays the consumer
  round's work; zero render path touched, goldens checked at the CI
  flags (rms 0.000), none regenerated.
  The **model files land (v0.3.64 — the held-item arc's data half, round
  2)**: the generator grew its twelfth table — `assets/minecraft/models/`
  of the official jar parsed into `generated/models.rs`. The census ran
  BEFORE any emitter code and the generator must agree with it
  byte-for-byte (two independent implementations, the itemgen
  dual-census shape): **3928 model files** split **2657 `block/` + 1271
  `item/`**, every one a JSON object, zero parse failures. Each row
  carries the **parent chain** (26.2 mixes bare and `minecraft:`-namespaced
  parents — 167 vs 3460 — both normalized to the bare path; `builtin/*`
  terminates the chain without a file), the **texture + display stanzas**
  as canonical compact JSON, **gui_light** (12 carriers: block/block
  `side`, the eleven `front`s) and **ambientocclusion** (157) typed, and
  the **element count**. The generator VALIDATES the graph: every parent
  resolves to a file or a builtin (a dangling parent refuses, a cycle
  refuses), max depth **4** — the crafter family walks verbatim
  crafter_crafting_triggered → crafter_crafting → crafter_triggered →
  crafter → block/block. The display stanzas are the held-item
  consumer's constants: `item/generated`'s **firstperson_righthand
  rotation [0,−90,25] translation [1.13, 3.2, 1.13] scale 0.68**
  (handheld's thirdperson [0,−90,55] scale 0.85 rides beside it), the
  `block/block` root's gui [30,225,0] scale 0.625 — and 26.2's new
  **`on_shelf`** context [0,180,0]. The census caught TWO shapes the
  common case hides: **163 object-valued texture entries**
  (`{"sprite":…,"force_translucent":true}` — redstone lines, glass
  panes), retained value-faithfully by the canonical-JSON columns, and
  **5 outlier files** the first real-jar run REFUSED (the
  extend-parser-first discipline doing its job): the four
  custom_fence_side models carry Blockbench `groups` editor metadata
  (retained as JSON, never dropped) and heavy_core states
  `texture_size [16,16]` (typed as the element-UV override). The
  `elements` GEOMETRY is honestly skipped: **344 models** retained in
  `ELEMENT_MODELS` (bamboo leaves → spyglass_in_hand), their counts in
  the table — and **24 plain item defs point directly at element-carrying
  models** (beacon, cactus, crafter…), the consumer round's bridge set.
  Offline arc = the worldgen/skin/item-models shape: `mc2rs-gen models
  --version 26.2 --jar <client.jar>` + `write_models()` in the regen
  pipeline. Tests +6 (workspace 281 → 287 passed / 0 failed / 3
  ignored): five generator fixture tests (parse + graph validation +
  the three outlier shapes, unknown-key refusal, dangling/cycle/
  foreign-namespace refusals, unknown display-context/transform/
  texture-shape refusals, the strict prefix discipline) and the
  registry pin (the 3928 census, verbatim spot rows incl. the
  firstperson constants, the parent graph re-walked FROM the emitted
  table, and the joins — all 1390 item defs resolve to model files).
  Zero render path touches the table yet; goldens untouched by
  construction.
  The **item model definitions land (v0.3.63 — the held-item arc's data
  half, the biome-tags data-first precedent)**: the generator grew its
  eleventh table — `assets/minecraft/items/` of the official jar parsed
  into `generated/item_models.rs` (the 1.21.4+ definition format). The
  census ran BEFORE any emitter code and the generator must agree with
  it byte-for-byte (two independent implementations, the skingen
  dual-decoder shape): **1537** definitions split into **1390 plain
  `minecraft:model` roots** resolved to (model path, tints,
  swap_animation_scale) — **731 pointing at `block/`, 659 at `item/`,
  586 at `block/<own id>`** — plus **147 selector/special trees**
  retained in `COMPLEX_ITEM_MODELS`, never silently dropped (select 71,
  special 51, composite 16, condition 7, range_dispatch 2). Tints ride
  as canonical compact JSON, value-faithful: grass_block's
  `{"downfall":1.0,"temperature":0.5,"type":"minecraft:grass"}`,
  filled_map's TWO (constant −1 + map_color 4603950). The key-set census
  caught a field the format ships beyond `{model}`: `swap_animation_scale`
  — the 26.2 **spears** all carry **1.95**, every one beside a `select`
  tree (gui/ground vs in_hand contexts), so ZERO plain def carries it
  and the column is all-None by evidence, not by omission (the parser
  ACCEPTS the key — refusing would break the real jar; dropping it
  silently would lose stated data). The extend-parser-first refusals
  are pinned: an unknown root key or model-node key refuses, an
  unnamespaced model path refuses, and a `models/item/` json is NOT an
  item definition (the strict prefix discipline). Offline arc = the
  worldgen/skin shape: `mc2rs-gen item-models --version 26.2 --jar
  <client.jar>` + `write_item_models()` in the regen pipeline. Tests +4
  (workspace 277 → 281 passed / 0 failed / 3 ignored): three generator
  fixture tests (resolve + tints + census, unknown-key refusal,
  unnamespaced refusal) and the registry pin (counts, the verbatim spot
  rows, the spear facts, the 731/659 split, and the joins — every def
  and every skipped item names an item `ITEMS` carries). Zero render
  path touches the table yet, so all 10 goldens verified at the exact
  CI flags, every one rms 0.000, none regenerated.
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

# pixel diff (one scene)
cargo run -p mc2rs-pixel -- --a render.png --b goldens/cpu_320x180.png --out diff.png

# the FULL golden matrix — all scenes re-rendered at their exact recorded
# flags and diffed against goldens/ (CPU/NET max-rms 8, GUI max-rms 0)
./verify_goldens.sh
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
