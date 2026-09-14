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
  The **per-dimension skybox gates the celestials (v0.3.90 — the nether
  has no sky)**: the sky pipeline's dimension gate lands. The audit ran
  first-hand on the 26.2 jar: `DimensionType$Skybox` is the vanilla
  string-representable enum (`none` / `overworld` / `end`), the codec is
  `optionalFieldOf("skybox", OVERWORLD)` — an ABSENT JSON key is the
  Overworld default (the real jar: the overworld pair states nothing,
  the end states `"end"`, the nether states `"none"`) — and the client's
  own `SkyRenderer.extractRenderState` stores `dimensionType.skybox()`
  and returns BEFORE any sky state is extracted when it is NONE, while
  END extracts only the end-flash triple and returns: the disc / sun /
  moon / stars path is the OVERWORLD skybox's alone. The generator
  parses the field into the generated table (`DimensionTypeDef.skybox`,
  the values verified against the raw JSONs: overworld/caves Overworld,
  end End, nether None; an unknown string the vanilla codec would
  reject outright, so the parse falls back to the codec default — never
  a guess). The client consumes it the vanilla way: the App pins the
  dimension's skybox as render state at LEVEL ENTRY (procedural →
  Overworld; a network join resolves the snapshot's dimension through
  the built-in table — the same skip-sync fallback slice the dimension
  layout rides; an unknown custom dimension rides the Overworld sky
  path until the synced-skybox wire decode lands, the ambient arc's
  shape) and `render_celestials` renders nothing unless the skybox is
  Overworld. Goldens untouched BY DATA: every golden world is an
  overworld world, and the headless orbit/net scenes never render the
  sky pipeline at all — 12/12 pass (the two net scenes at their
  designed shared-float tolerance, byte-identical before and after the
  round, proven by a d395db6 worktree re-render). Tests +3 (workspace
  398 → 401 passed / 0 failed / 3 ignored): the render gate witness
  (at midnight the stars tile the sphere — OVERWORLD provably paints,
  NONE and END leave the cleared framebuffer untouched), the built-in
  table pins, and the App pin discipline (a nether join pins None, a
  procedural re-entry re-pins Overworld).
  The **synced skybox rides the wire (v0.3.91 — the custom-dimension
  half closes)**: the skybox arc's wire half lands, the ambient
  attribute's v0.3.89 precedent verbatim. The audit answered the
  round's question from the codec itself: `DimensionType.NETWORK_CODEC`
  is built by the SAME `createDirectCodec` the JSON-side DIRECT_CODEC
  uses (`NETWORK_CODEC = createDirectCodec(EnvironmentAttributeMap.
  NETWORK_CODEC)`, javap) — the skybox field RIDES the registry sync,
  serialized as the StringRepresentable enum's lowercase name
  (`none` / `overworld` / `end`), still bound
  `optionalFieldOf("skybox", OVERWORLD)`: an ABSENT key is the
  Overworld default while a STATED `none` is its own verdict — the two
  are NOT the same, so unlike the grass modifier (whose absent/`none`
  collapse) the wire field stays an Option. mc-protocol's
  `DimensionTypeEntry` grows `skybox: Option<Skybox>` — the wire-local
  three-constant enum, the GrassColorModifier shape; unknown names
  never ride a working sync (vanilla's own codec refuses them) and
  read `None`, the codec default, never a guess. The model grows
  `WorldModel::skybox()` — the ambient discipline, skybox edition:
  the SYNCED entry's stated skybox wins (a custom overworld stating
  `none` renders no sky), its SILENCE reads the codec's own default
  Overworld (NOT the built-in nether's None), the BUILT-IN table
  serves ONLY the skip-sync case, and an unknown custom key rides the
  Overworld path — the vanilla default, never a guess. Both join
  sites (the live worker and the one-shot connect) resolve through
  the model the snapshot adopts into, and the retired
  `world_view::skybox_for_dimension` helper leaves no survivors (the
  timeline-tag walk precedent — the deletion keeps one truth). The
  mock registry's nether element now STATES `"skybox":"none"` (the
  real vanilla shape — the nether is the one dimension that does), so
  a wire nether join pins None through the DECODE, not the built-in
  table. Goldens untouched BY CONSTRUCTION: no render-path change
  (the only difference is WHICH table answers, and for real vanilla
  data the answers are identical); 12/12 pass. Tests +2 net (workspace
  401 → 403 passed / 0 failed / 3 ignored): the wire enum vectors (the
  three names + the refusals) and the synced-truth precedence (stated
  wins / silent reads the codec default / no key keeps the default,
  never a built-in fallback), plus the packet-level assertions (the
  overworld element states NO key, the nether STATES none); the
  retired helper's built-in pins moved into the model accessor tests
  one-for-one.
  The **END sky's flash lands (v0.3.92 — the end-flash consumer)**: the
  END skybox's remaining visual is now rendered, and it is DETERMINISTIC.
  The audit ran first-hand on the 26.2 jar: `EndFlashState` redraws its
  parameters only when the 600-tick window index changes, from an LCG
  SEEDED by that index (`RandomSource.createThreadLocalInstance(
  gameTime / 600)` — a SingleThreadedRandomSource, the classic Java
  random) — so the flash is a pure function of game time and a
  pixel-deterministic client reproduces it exactly, no dropped shimmer,
  no documented drift. The draw order is pinned and observable: one
  DISCARDED `nextFloat`, the offset `randomBetweenInclusive(0, 200)`,
  the duration `randomBetweenInclusive(100, min(380, 600 - offset))`,
  then the angles `randomBetween(-60, 10)` and `randomBetween(-180,
  180)` (Mth.randomBetween = `nextFloat * (max - min) + min` in f32);
  the intensity is the SIN-TABLE hump `Mth.sin((t - offset) * PI /
  duration)` — the 65536-entry table (`SIN[i] = (float)
  Math.sin(i / 10430.378350470453)`, the DIVISION the bytecode does),
  NOT libm sin — and `getIntensity` lerps the last tick's fade. The
  vanilla quirk the port preserves: the state's defaults ride window 0
  (`flashSeed == 0 == window(0)`), so the FIRST window never redraws
  and renders nothing — the first real flash is window 1 (offset 133,
  duration 311, cross-checked against a Python f32/f64 oracle). The
  generator reads `celestial/end_flash.png` (64x64 — its OWN
  END_FLASH_SIZE, not the sun/moon 32x32) into the generated sky table
  (the regenerated table's diff is exactly 6 lines; every other table
  byte-stable). The client: `mc-client/src/end_flash.rs` (the state +
  the Mth pieces; the shared sky::JavaRandom grows the javap-exact
  `nextInt` — power-of-two shift shortcut or the wrapping rejection
  loop), the sky gate becomes the extract's match (NONE returns before
  ANY state, END renders ONLY the flash — `rotY(180 - y) * rotX(-90 -
  x)`, base (±1,0,±1) scaled (60,1,60) translated (0,100,0) inside the
  rotated frame, tinted `(i,i,i,i)` through the CELESTIAL pipeline
  whose BlendFunction.OVERLAY is exactly the raster's Additive), and
  the App ticks the state on its game-time clock (the frame == tick
  model — the draw reads the tick-exact current intensity, the lerp
  machinery pinned at both partials). Goldens untouched BY DATA: every
  golden world is overworld/procedural and the render path never saw
  the END branch — 12/12 pass. Tests +5 (workspace 403 -> 408 passed /
  0 failed / 3 ignored): the LCG vectors (the scramble, the
  power-of-two shortcut, the rejection loop, the oracle's window-0
  draw), the first-window quirk, the window-1 flash against the oracle
  (offset/duration/angles/peak), the window-2 redraw + the lerp at
  both partials, and the render witness (an active flash paints the
  END sky, NONE still returns before any state, OVERWORLD is
  byte-identical with and without a flash riding along).
  The **nether's fogged walls render (v0.3.93 — the stated fog
  distances)**: the environment-attribute chain consumes its next pair.
  The audit ran first-hand on the 26.2 jar: `EnvironmentAttributes.<clinit>`
  pins `visual/fog_start_distance` default 0.0 and
  `visual/fog_end_distance` default 1024.0 (both FLOAT, both
  `.syncable()` — both RIDE the registry sync), and of the four
  dimension JSONs ONLY the nether states the pair — 10.0 / 96.0, the
  fogged walls (the overworld pair, the caves and the end state no fog
  keys at all). The generator parses both into the built-in table
  (`DimensionTypeDef.fog_start_distance`/`fog_end_distance`, None = the
  bytecode defaults' shape — the regenerated table's diff is exactly
  the 4 rows + the struct docs). mc-protocol's `DimensionTypeEntry`
  grows the same two fields decoded from the flat attributes compound
  under the PAIR-OR-NOTHING rule: a one-sided statement is not a real
  vanilla shape and reads as the honest absence, never a half guess;
  the mock registry's nether element now states the real 10/96 and the
  overworld states none (the packet-level assertions pin both). The
  model grows `WorldModel::fog_distances()` — the ambient discipline's
  three-branch precedence: the SYNCED entry's stated pair wins, the
  built-in table serves the skip-sync case, and `None` (no pair
  anywhere) is the defaults' verdict. The consumer: the App pins the
  pair at level entry like the skybox (both join sites resolve through
  the model; the procedural entry drops it) and the world view's fog
  becomes `world_fog.unwrap_or((n * 0.55, n * 1.7))` — the STATED
  distances render verbatim when a dimension states them, and the
  WINDOW TREATMENT covers the unstated shape: the bytecode defaults
  (0/1024) are far beyond this client's 5x5-chunk window, where a
  nearly-invisible vanilla fog would show raw window edges — the
  documented, deliberate rendering of the far defaults, and the reason
  every golden world (overworld, unstated) is byte-stable. Goldens:
  12/12 pass untouched. Tests +1 (workspace 408 -> 409 passed / 0
  failed / 3 ignored): the fog accessor's precedence (the built-in
  nether pair on skip-sync, the synced truth winning, the silent sync
  and the one-sided statement reading None) and the App pin discipline
  (the nether join pins 10/96, the procedural re-entry drops it).
  The **block-light tint is STATE — and the literal was wrong (v0.3.96
  — the lightmap's block_light_tint consumer + the blue-channel
  correction)**: the first consumer of the four decoded attributes.
  The audit re-derived the tint's whole chain from the 26.2 jar and
  found the shipped `BLOCK_LIGHT_TINT` constant contradicting its own
  citation: `EnvironmentAttributes.<clinit>` pins `defaultValue(sipush
  -10100)`, and `-10100 = 0xFFFFD88C` = #FFD88C — blue **140**, not
  the shipped 172 (#FFD8AC would be -10068; a one-hex-digit misread of
  0x8C as 0xAC had ridden the codebase since the tint landed). The
  correction is bytecode-pinned five links deep: the attribute codec
  is `ExtraCodecs.STRING_RGB_COLOR` = `hexColor(6)` xmap'd with
  `ARGB.opaque`/`ARGB.transparent` (the canonical int is the ARGB
  0xFFRRGGBB), and `LightmapRenderStateExtractor.extract` feeds the
  probe's value through `ARGB.vector3fFromRGB24` (`red = (i>>16)&255,
  green = (i>>8)&255, blue = i&255` — the alpha bits fall off the
  masks) straight into the `BlockLightTint` uniform the lightmap.fsh
  `mix(BlockLightTint, vec3(1.0), 0.9 * parabolicMixFactor)` consumes.
  The implementation threads the attribute the cloud_fog way: the
  generator parses `minecraft:visual/block_light_tint` into the
  built-in table (four None rows — the regenerated table's diff is
  exactly the struct doc + the 4 rows, every other table byte-stable),
  the model grows `WorldModel::block_light_tint()` (the three-branch
  precedence), `materialize` stamps it onto the world snapshot beside
  its sibling `ambient_light` (the lightmap family's own transport,
  not the App-pin path), and the mesher's `block_light_color` grows
  the tint parameter — `tint.unwrap_or(DEFAULT_BLOCK_LIGHT_TINT)`,
  the corrected constant, a stated tint replacing the default
  VERBATIM (the extractor's no-blend shape). GOLDENS REGENERATED —
  the deliberate, documented act: the correction moves torch-lit
  pixels (the two net scenes' rms drifted 0.386→0.393 / 0.398→0.406
  against the old references), so `net_320x180.png` and
  `net_480x270.png` were re-rendered at their exact recorded flags;
  the regen also RESET the historical shared-float residue — all
  twelve goldens now verify at rms 0.000, the matrix fully
  bit-identical for the first time since the tolerance was
  introduced. Tests +3 (workspace 413 -> 416 passed / 0 failed / 3
  ignored): the stated-tint override (the verbatim replace + the
  mesher witness), the accessor's three-branch precedence, and the
  materialize stamp (the absent shape end to end + a stated entry).
  The golden-policy lesson: an rms move inside the tolerance is not
  a pass to keep quiet about — the render changed, so the reference
  changed with it, and the diff explained the regen.
  The **the world changes while you play — Block Update and Section
  Blocks Update land (v0.3.100)**: the block-mutation packets, the
  wire's answer to a rendered world. The audit re-derived BOTH wire
  formats from the 26.2 bytecode, not the wiki: Block Update (generated
  id 8) is `BlockPos.STREAM_CODEC` — a packed position LONG whose
  layout javap pins as `((x & 0x3FFFFFF) << 38) | (y & 0xFFF) |
  ((z & 0x3FFFFFF) << 12)` with the getters' two-shift ARITHMETIC
  sign-extension (`lshr`, not the logical shift; `PACKED_HORIZONTAL_LENGTH`
  = 26 from `log2(smallestEncompassingPowerOfTwo(30000000)) + 1`) —
  then a VarInt state id (`ByteBufCodecs.idMapper(BLOCK_STATE_REGISTRY)`,
  the chunk decoder's own id space). Section Blocks Update (generated id
  84) is `SectionPos.STREAM_CODEC` (`((sx & 0x3FFFFF) << 42) |
  (sy & 0xFFFFF) | ((sz & 0x3FFFFF) << 20)`), a VarInt count, then
  VarLong entries — `position = packed & 4095`, `state = packed >>> 12`
  (LOGICAL `lushr`), the 12-bit local packing `(x << 8) | (z << 4) | y`
  (javap `sectionRelativeX/Y/Z`). The model consumes BOTH the Light
  event's way: the stored column mutates IN PLACE through the SAME
  arithmetic `block()` reads through — the next materialize renders the
  change, nothing else exists to invalidate — under the strict
  all-or-nothing rule (an absent column, an out-of-range slot, or ONE
  bad entry in a batch refuses the WHOLE update, never a half-applied
  lie; the section index is the exact floor walk
  `(sy * 16 - min_y) >> 4`). The parse side keeps the set-time
  discipline: counts validated against the remaining bytes BEFORE any
  allocation. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 at rms 0.000 (no
  golden flow carries block-update packets; the mesher untouched).
  Tests +4 (workspace 425 -> 429 passed / 0 failed / 3 ignored): the
  position round-trip (the negative half-plane, the 26/12-bit field
  edges), the packed entries (the (15,15,15) corner, a 2-byte VarLong
  state, the lying count refused), the single update's in-place
  mutation + the refused shapes, and the batch's all-or-nothing rule
  (the valid prefix must NOT land when a later entry fails).
  The **entities exist in the world — Add Entity, Remove Entities and
  Entity Position Sync land (v0.3.101)**: the population layer the
  mutable world was waiting for. The audit re-derived all THREE wire
  formats from the 26.2 bytecode, and the first one broke the wiki-era
  assumption outright: `Vec3.LP_STREAM_CODEC` delegates to
  `net.minecraft.network.LpVec3` — NOT the old three-shorts-times-8000
  layout. The real shape is 6 bytes packed as a 48-bit big-endian
  integer: byte 0 carries the scale's low 2 bits, a continuation flag
  (`(i & 4) == 4`) and the data's low 5 bits (a zero FIRST byte is the
  ZERO sentinel — nothing else is read); three 15-bit lanes sit at
  shifts 3 / 18 / 33, each `unpack(v) = min(v & 32767, 32766) * 2 /
  32766 - 1` (the clamp IS the bytecode: `Math.min`), and the whole
  vector multiplies by a scale that can grow past the 2-bit field
  through ONE continuation VarInt (`scale |= (varint & 0xFFFFFFFF) <<
  2`) — a magnitude-times-quantized-direction encoding. Add Entity
  (generated id 1) is VarInt id, UUID (two big-endian longs, most
  first), VarInt entity-type registry id, three full-precision doubles,
  the LpVec3 movement, three angle BYTES (`Mth.unpackDegrees(b) = b *
  360 / 256`, the sign-extended byte's [-180, 180)) and the VarInt
  entity-type data. Entity Position Sync (generated id 35) is the
  `PositionMoveRotation` record — TWO full-precision Vec3s (position,
  deltaMovement), FLOAT yaw, FLOAT pitch — plus the BOOL ground flag.
  Remove Entities (generated id 77) is `readIntIdList`: a VarInt count
  validated against the remaining bytes BEFORE any allocation, then
  that many VarInt ids. The model grows the entity registry (keyed by
  entity id): the spawn CREATES (re-spawning an id REPLACES — the
  vanilla lookup's own semantics), the sync MOVES in place (an unknown
  id is refused — the spawn packet stays the only creator, no phantom
  entities from a correction), the batch removal drops whatever exists
  and reports whether ANYTHING changed (unknown ids are honest no-ops,
  the same `.is_some()` honesty UnloadChunk carries), and the Respawn
  crossing clears the population WITH the columns — the vanilla client
  swaps the whole level, so the old dimension's mobs do not follow the
  player through the portal. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12
  at rms 0.000 (no golden flow carries entity packets; the mesher
  untouched). Tests +6 (workspace 429 -> 435 passed / 0 failed /
  3 ignored): the Add Entity layout (the bit-exact grid movement, the
  -128 -> -180.0 angle floor, the UUID's 128 bits), the LpVec3 decode
  (the zero sentinel's exact one-byte consumption, the scale-100
  continuation reassembled, the quantization step, the hand-rolled
  32767 clamp), the id-list batch (the lying count refused, the honest
  empty batch), the sync record (both ground-flag bytes), and the
  model's lifecycle (spawn/move/despawn in place, the no-phantom-sync
  rule, the dimension swap).
  The **entities carry metadata — Set Entity Data lands (v0.3.103)**:
  the accessor-indexed key-value set (generated id 99) the syncs don't
  carry — the on-fire/crouched/pose surfaces' wire transport. The audit
  re-derived the format from the 26.2 bytecode: a VarInt entity id,
  then entries (a BYTE index, a VarInt serializer id, the value)
  until the 0xFF index — the EOF marker, real accessors 0..=254.
  The round's discovery: the serializer ids are the
  `EntityDataSerializers.<clinit>` REGISTRATION order and it is NOT
  the field declaration order — VILLAGER_DATA is 18 and
  OPTIONAL_GLOBAL_POS is 33 (the wiki-era table swaps them), the
  animal variant/sound-variant Holders fill 21-32, and STRING is
  capped at 32767 (`stringUtf8(32767)`, not the wiki-era 65536). The
  client decodes the PRIMITIVE registry (BYTE, INT, LONG as a VarLong,
  FLOAT, STRING, BOOLEAN, ROTATIONS, BLOCK_POS, OPTIONAL_BLOCK_POS,
  DIRECTION, BLOCK_STATE, OPTIONAL_BLOCK_STATE, the
  OptionalInt's BIAS-BY-ONE OPTIONAL_UNSIGNED_INT where wire n means
  n-1, POSE) into a typed entry enum and refuses the object-graph
  shapes (components, item stacks, particles, villager data, entity
  references, profiles, the variant holders) — a value's byte length
  is only knowable through its codec, so there is no honest skip, and
  the vanilla read side fails the packet the same way. The consumer is
  vanilla's own handle: `level.getEntity(id)` with a NULL lookup
  DROPPING the packet (`ifnull -> return`) — metadata keeps current
  entities current and never creates one; the entries are the
  assignValues walk (each listed accessor replaced, the unmentioned
  ones untouched) into the snapshot's new BTreeMap store. GOLDENS
  UNTOUCHED BY CONSTRUCTION: 12/12 at rms 0.000. Tests +5 (workspace
  443 -> 448 passed / 0 failed / 3 ignored): the full round-trip of
  all fourteen serializers in one packet (the registration-order
  table pinned), the EOF marker (the empty set, the post-marker
  drift, the truncated entry), the object-graph + unknown refusal
  (VILLAGER_DATA at 18 doubles as the order proof), the generated-id
  pin, and the model's merge semantics (replace-one-leave-others, the
  null-lookup drop, the re-spawn reset).
  The **the variant-holder census — ALL 43 entity-data serializers
  decode; the object-graph refusal set retired to the unknown ids
  alone (v0.3.117)**: the LAST refusals in the registry walk retire —
  ids 21-40 and 42 (RESOLVABLE_PROFILE at 41 landed the round
  before). The javap audit, seventeen classes in one sweep: the
  clinit's registration loop confirms the port's id table AND pins
  the holders' wire order, which DIVERGES from the field-init order
  — the loop runs CAT, CAT_SOUND, COW, COW_SOUND, WOLF, WOLF_SOUND,
  FROG, PIG, PIG_SOUND, CHICKEN, CHICKEN_SOUND, ZOMBIE_NAUTILUS
  (ids 21-32) where the field-init block had the CHICKEN pair third
  and the COW pair fifth; the same swap puts SNIFFER_STATE (35)
  before ARMADILLO_STATE (36) — only the loop is the wire (the
  autopush-115 lesson's third family this round). The lanes: the
  twelve holders = `ByteBufCodecs.holderRegistry(REGISTRY)` (the
  anonymous $29 = `VarInt.read` -> `byIdOrThrow` — the villager
  lane's family; the port refuses the negative half, the
  arithmetic-only check, and retains the non-negative unknown
  verbatim — no variant-registry tables ride this client);
  OPTIONAL_GLOBAL_POS (33) = `GlobalPos.STREAM_CODEC` under the
  #149-bsm ONE-FLAG-BYTE optional — the composite is the dimension
  key via `ResourceKey.streamCodec` = `Identifier.STREAM_CODEC
  .map(..)` (the dimension rides a plain STRING like
  "minecraft:overworld", NOT a registry id — the name-shaped trap
  sitting beside the villager VarInts) + `BlockPos.STREAM_CODEC`
  (the packed i64 the chunk path decodes); PAINTING_VARIANT (34) =
  `ByteBufCodecs.holder(PAINTING_VARIANT, DIRECT_STREAM_CODEC)` —
  the anonymous $30, the holder family's ONLY bias-by-one member:
  wire 0 = the INLINE record (width, height, assetId string,
  title/author each the TRUSTED_OPTIONAL flag+tag lane), wire n >=
  1 = registry id n - 1, wire n < 0 throws; the enum states =
  `ByteBufCodecs.idMapper(BY_ID, ordinal)` over
  `ByIdMap.continuous(..)` — SNIFFER (7 states, ZERO), ARMADILLO
  (4, ZERO), COPPER_GOLEM (5, ZERO), WEATHERING_COPPER (4, the
  family's only CLAMP — vanilla's decode map clamps out-of-range
  keys, and the port retains the wire ordinal verbatim, the clamp
  the consumer's act), HUMANOID_ARM (LEFT = 0, RIGHT = 1 — the
  declaration order's own truth, no explicit id field exists so no
  convention trap); VECTOR3 (39) = `ByteBufCodecs.VECTOR3F` =
  `readVector3f` = three big-endian f32s; QUATERNION (40) =
  `QUATERNIONF` = `readQuaternion` = four big-endian f32s in the
  JOML x, y, z, w order. The implementation: the 21 consts (the
  census docs carrying each lane's audit), the `GlobalPos` struct
  and the `PaintingVariant` enum (Registry/Direct), 21
  `EntityDataEntry` variants, the shared `read_holder_id` reader,
  the decode arms, the enum census now claiming ALL FORTY-THREE
  ids (0-42), and the lib.rs exports. mc-client untouched BY
  CONSTRUCTION. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 PASS.
  Tests +1 (workspace 467 -> 468 passed / 0 failed / 3 ignored):
  one 25-entry packet riding every new lane (twelve holders each
  keeping its own wire id, the global pos present AND absent, the
  painting BOTH shapes — the direct record and the bias-by-one
  reference — one named state per enum lane, the float layouts,
  RIGHT, and the retention pins: a wire 99 sniffer state and a
  wire 999 weathering state stored verbatim), the negative holder
  ids refusing (byIdOrThrow's direction), the truncated quaternion
  and the truncated direct painting refusing (the honest
  short-buffer failure). The object-graph refusal test's survivor
  list is the unknown ids alone — id 21's 0x00 probe byte became a
  legal cat-variant id this round.
  The **the resolvable profile — serializer 41, the last low-id
  object-graph refusal, retired (v0.3.116)**: the RESOLVABLE_
  PROFILE lane decodes. The javap audit, six hops: `Resolvable
  Profile.STREAM_CODEC` = `composite(either(ByteBufCodecs.GAME_
  PROFILE, ResolvableProfile$Partial.STREAM_CODEC), PlayerSkin$
  Patch.STREAM_CODEC)` — the either is the ONE-BYTE flag (true =
  LEFT = the FULL GameProfile, false = RIGHT = the Partial);
  `GAME_PROFILE` = `composite(UUIDUtil.STREAM_CODEC [16 bytes],
  PLAYER_NAME [stringUtf8(16)], GAME_PROFILE_PROPERTIES)`; the
  properties map is the anonymous `ByteBufCodecs$32`:
  `readCount(buf, 16)` — ONLY the upper bound throws, and a
  NEGATIVE count passes and the builder loop iterates zero times:
  an EMPTY map (the never-refuse family, discovered by the
  bytecode and pinned by a probe) — then count × {name ≤64, value
  ≤32767, nullable signature ≤1024 (FriendlyByteBuf.readNullable's
  one-byte flag)}; the Partial = `composite(optional(PLAYER_NAME),
  optional(UUIDUtil.STREAM_CODEC), GAME_PROFILE_PROPERTIES)`;
  `PlayerSkin$Patch` = FOUR one-byte-flag optionals — body, cape,
  elytra (each a `ClientAsset$ResourceTexture` = `composite(
  Identifier.STREAM_CODEC, Identifier.STREAM_CODEC)`, two
  ≤32767-byte strings — STRING_UTF8's own cap; the Identifier wire
  is the plain string mapped) and model (`PlayerModelType.STREAM_
  CODEC` = `ByteBufCodecs.BOOL.map(..)` — the enum's own lambda
  pair: TRUE = SLIM, FALSE = WIDE, pinned by lambda$static$1/2's
  bytecode). The implementation: the `ProfileProperty`,
  `SkinAsset`, `PlayerSkinPatch` types, the `ResolvableProfile`
  enum (Full/Partial), the shared `read_uuid` extraction (the
  reference lane and the profile lane share the ONE codec), the
  capped-string reader family (the VarInt length IS the byte
  length, so the post-read cap check is exact), the decode arm,
  the EntityDataEntry variant, and the lib.rs exports. The decode
  census: TWENTY-TWO of the registry's 43 serializers (0-20, 41).
  mc-client untouched BY CONSTRUCTION. GOLDENS UNTOUCHED BY
  CONSTRUCTION: 12/12 PASS. Tests +1 (workspace 466 -> 467 passed
  / 0 failed / 3 ignored): the FULL side (uuid + name + one
  property + the body-asset patch with model SLIM), the PARTIAL
  side (the optional name rides, the optional id absent, the
  NEGATIVE property count decoding to an EMPTY map, the patch
  all-absent), the 17-property count refusing (readCount's upper
  bound), and the truncated identity refusing. The object-graph
  refusal set is now the variant holders only.
  The **the dispatched particle-type registry — the PARTICLE and
  PARTICLES lanes walk all 125 types (v0.3.115)**: the last two
  serializer refusals in the low-id range retire. The javap audit:
  `ParticleTypes.STREAM_CODEC` = `StreamCodec.dispatch(ByteBufCodecs
  .registry(PARTICLE_TYPE), typeToCodec, codecToType)` — the $29
  byIdOrThrow family (an unknown AND a negative id throw), then the
  type's OWN payload codec. The clinit's 125 `register` calls ARE
  the registry (angry_villager 0 .. sulfur_cube_goo 124): 103
  SimpleParticleType carriers ride ZERO payload bytes (the
  `StreamCodec.unit` lane), and the 22 payload carriers map to 13
  composite shapes, each pinned by its four-arg register's
  codecFactory bootstrap handle: BlockParticleOption (block,
  block_marker, falling_dust, dust_pillar, block_crumble) =
  `idMapper(BLOCK_STATE_REGISTRY)`; GeyserParticleOptions (geyser,
  geyser_plume) = `composite(INT)` (waterBlocks);
  GeyserBaseParticleOptions (geyser_base, geyser_poof) =
  `composite(INT, FLOAT)` (waterBlocks + burstImpulseBase);
  PowerParticleOption (dragon_breath) = `composite(FLOAT)`;
  DustParticleOptions (dust) = `composite(INT, FLOAT)`; DustColor
  TransitionOptions (dust_color_transition) = `composite(INT, INT,
  FLOAT)`; SpellParticleOption (effect, instant_effect) =
  `composite(INT, FLOAT)`; ColorParticleOption (entity_effect,
  tinted_leaves, flash) = `composite(INT)`; ItemParticleOption
  (item) = `map(ItemStackTemplate.STREAM_CODEC)` = `composite(
  Item.STREAM_CODEC, VAR_INT, DataComponentPatch.STREAM_CODEC)` —
  the item id, a RAW count lane (NO count-is-emptiness sentinel,
  the ItemStack$1 line does not apply), and the same patch shape
  the metadata lane decodes; VibrationParticleOption (vibration) =
  `composite(PositionSource.STREAM_CODEC, VAR_INT)` where the
  source is ANOTHER dispatched registry (POSITION_SOURCE_TYPE:
  block 0 = the packed-pos i64, entity 1 = `composite(VAR_INT,
  FLOAT)`); TrailParticleOption (trail) = `composite(Vec3.STREAM_
  CODEC, INT, VAR_INT)` — three big-endian f64 (Vec3$1's
  readDouble trio), the color, the duration;
  ShriekParticleOption (shriek) = `composite(VAR_INT)`;
  SculkChargeParticleOptions (sculk_charge) = `composite(FLOAT)`.
  The implementation: the `particle_types` module (COUNT = 125 +
  the 22 carrier ids), the `ParticleValue` enum (14 variants), the
  `PositionSource` enum, the dispatch walk + nested-source reader,
  the PARTICLE and PARTICLES entity-data arms (the list being the
  VarInt count + entries, a negative count the readCount family's
  panic-free refusal), and the lib.rs exports. The decode census:
  TWENTY-ONE of the registry's 43 serializers (0-20). mc-client
  untouched BY CONSTRUCTION. GOLDENS UNTOUCHED BY CONSTRUCTION:
  12/12 PASS. Tests +2 (workspace 464 -> 466 passed / 0 failed /
  3 ignored): the dispatched walk pins fourteen payload-shape
  representatives in ONE packet (simple, block, both geysers,
  power, dust, transition, spell, color, item template with its
  patch, the nested block-source vibration, the f64 trail, shriek,
  sculk), and the refusal map holds (unknown id 125, negative id,
  a negative block-state payload, an unknown position-source type,
  the negative list count — each probe landing ON its lane). The
  object-graph refusal set is now the profile (41) and the variant
  holders only.
  The **the living-entity reference lane — serializer 13 retired
  from the object-graph refusal set (v0.3.114)**: the OPTIONAL_
  LIVING_ENTITY_REFERENCE lane decodes. The javap audit, three hops:
  `EntityReference.STREAM_CODEC` = `UUIDUtil.STREAM_CODEC.map(
  wrap-left, unpack-uuid)` — the wire carries ONLY the referenced
  entity's UUID (vanilla's `Either<UUID, StoredEntityType>` rides
  the LEFT side on the wire; the decode wraps the UUID in Either
  .left and the encode unpacks via getUUID — a resolved entity is
  never serialized); `UUIDUtil.STREAM_CODEC` is the anonymous
  `UUIDUtil$1` = `FriendlyByteBuf.readUUID` = `new UUID(readLong(),
  readLong())` — 16 bytes, msb long then lsb long, both big-endian;
  and the serializer wraps the whole codec in the ONE-FLAG-BYTE
  optional operation (the same invokedynamic bsm BlockPos.optional
  and GlobalPos.optional ride — the port's OPTIONAL_BLOCK_POS shape
  mirrored). The implementation: the `EntityReference` struct
  (uuid: [u8; 16], retained verbatim — the resolution is the
  consumer's act, the retained-tag-is-a-store line), the flag-then-
  16-bytes decode arm, the new `OptionalLivingEntityReference`
  variant, and the lib.rs export. mc-client untouched BY
  CONSTRUCTION. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 PASS.
  Tests +1 (workspace 463 -> 464 passed / 0 failed / 3 ignored):
  the empty lane (the flag byte alone), the present lane (16
  distinctive bytes riding verbatim), and the truncated UUID (15
  bytes after the flag) refusing — the honest short-buffer failure.
  No existing probe inverted; the object-graph refusal set keeps
  PARTICLE, PARTICLES, profiles and the variant holders.
  The **the OptionalInt lane's signed truth — the arithmetic-codec
  deferral retired (v0.3.113)**: the autopush-114 round deferred the
  OptionalInt serializer with evidence, and this round retires it.
  The javap audit, fresh: `EntityDataSerializers$3` (the
  OPTIONAL_UNSIGNED_INT field's anonymous codec) decodes `i =
  VarInt.read`; `i == 0` is `OptionalInt.empty()`, ELSE `OptionalInt
  .of(i - 1)` — the arithmetic NEVER refuses, a negative wire n
  storing a NEGATIVE value (the `isub`'s two's-complement wrap:
  wire MIN_VALUE stores MAX_VALUE), and the encode mirror is
  `writeVarInt(optional.orElse(-1) + 1)` (empty rides 0, present
  MAX_VALUE rides MIN_VALUE). The delivered lane's `ensure!(n >= 0)`
  refused the negative half — a strictness the bytecode does not
  have — and the `Option<u32>` store FORGED the rest (the `as u32`
  cast wrapped a negative n - 1 into a huge positive where vanilla
  stores the negative). The fix: the store is the SIGNED truth
  (`Option<i32>`, the variant name keeping the wire's field name
  OPTIONAL_UNSIGNED_INT while the docs carry the name-vs-type
  divergence — the field NAME says unsigned, the codec's type is
  the signed `OptionalInt`), the read lane retains any i32 verbatim
  via `wrapping_sub` (the honest mirror of the Java `isub`; a plain
  subtraction would panic in debug on the MIN edge), and the family
  sweep confirms the lane is the delivered serializer set's ONLY
  bias-by-one arithmetic codec (BLOCK_POS's packed-long arithmetic
  is signed shifts, INT/LONG are raw VarInt/VarLong — no other
  member). mc-client untouched BY CONSTRUCTION. GOLDENS UNTOUCHED BY
  CONSTRUCTION: 12/12 PASS. Tests +1 (workspace 462 -> 463 passed /
  0 failed / 3 ignored): the signed truth pinned end-to-end — wire
  -5 is Some(-6) (the negative half the old lane refused), and wire
  i32::MIN is Some(i32::MAX) (the wrap edge, probe landing ON the
  lane). No existing probe needed inversion: the strictness shipped
  unpinned twelve versions ago.
  The **the idMapper ordinal lanes' never-refuse contract — the
  Rarity fidelity-fix round (v0.3.112)**: the delivered RARITY lane
  refused a NEGATIVE ordinal while retaining out-of-range positives —
  an asymmetry the bytecode does not have. The javap audit, fresh:
  `Rarity.STREAM_CODEC` = `ByteBufCodecs.idMapper(BY_ID,
  Rarity::ordinal)` where `BY_ID` = `ByIdMap.continuous(..,
  OutOfBoundsStrategy.ZERO)` — the map NEVER refuses, every key
  outside [0, 4) (negative AND out-of-range) filling to values[0] =
  COMMON; and the audit's sweep caught the SAME defect family in two
  more delivered lanes: `Pose.STREAM_CODEC`'s map is the same
  continuous ZERO shape (out-of-range fills to STANDING), and
  `Direction.STREAM_CODEC`'s `BY_ID` is the WRAP strategy
  (`ByIdMap.continuous(.., OutOfBoundsStrategy.WRAP)` — out-of-range
  keys floorMod into [0, 6), a negative -1 wrapping to 5 = up; also
  never refuses). The fix: all three lanes drop their negative
  refusals and retain every wire ordinal VERBATIM — negative and
  out-of-range — the fill/wrap being the consumer's resolution (the
  operation/slot/animation line from the wave-four round: retaining
  the DATA ordinal is the family norm, and the strictness was the
  invention). The audit also found the OptionalInt serializer ($3)
  accepts a negative n as `OptionalInt.of(n - 1)` where the port
  refuses — the same stricter-than-vanilla class but a DIFFERENT
  family (an arithmetic codec, not an idMapper, and the honest fix
  changes the store's Option<u32> to a signed shape) — DEFERRED with
  evidence, a candidate of its own. mc-client untouched BY
  CONSTRUCTION. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 PASS. Tests
  +1 (workspace 461 -> 462 passed / 0 failed / 3 ignored): the
  never-refuse contract pinned end-to-end — Direction -1 and 42,
  Pose -1 and 99 through the entity-data transport, Rarity -2 and 9
  through the patch walk (the ordinals the old lane refused, now
  retained verbatim).
  The **the dispatched predicate-type registry's payload census — the
  wave-three partial-matcher deferral retired (v0.3.111)**: the
  component-matchers' PARTIAL side — which the wave-three round
  refused at any NON-EMPTY count — now WALKS. The javap audit, four
  hops: `DataComponentMatchers.STREAM_CODEC`'s partial side =
  `DataComponentPredicate.STREAM_CODEC` = `SINGLE_STREAM_CODEC
  .list(64)` mapped to a Map (a 64-capped VarInt count, then count ×
  entries); each `Single` entry is `StreamCodec.dispatch(
  DataComponentPredicate$Type.STREAM_CODEC, type.singleStreamCodec)`;
  the TYPE lane = `ByteBufCodecs.either(registry(
  DATA_COMPONENT_PREDICATE_TYPE), registry(DATA_COMPONENT_TYPE))` —
  a ONE-BYTE `$26` readBoolean (true = LEFT) then a `$29` byIdOrThrow
  VarInt into the branch's registry (BOTH are real lookups, NOT the
  idMapper-continuous family: unknown AND negative ids throw, the
  port refuses at the same lane); and the PAYLOAD rides as ONE full
  NBT tag — `TypeBase.singleStreamCodec` = `ByteBufCodecs
  .fromCodecWithRegistries(record.CODEC)` = the `tagCodec` bridge
  (`NbtIo.readAnyTag`: type byte + payload, no name framing) parsed
  through the record's OWN NBT-side codec — the payload is
  self-delimiting, so the walk continues and the wave-three
  "unframed wire" objection retires by construction. The registry:
  15 types pinned by clinit order (damage 0, enchantments 1,
  stored_enchantments 2, potion_contents 3, custom_data 4, container
  5, bundle_contents 6, firework_explosion 7, fireworks 8,
  writable_book_content 9, written_book_content 10, attribute_modifiers
  11, trim 12, jukebox_playable 13, villager/variant 14 — the FIFTH
  registry pinned by clinit order; the names are the `ldc` strings
  verbatim, and `villager/variant` carries its SLASH). The RIGHT
  branch is the `AnyValueType` — a plain DATA_COMPONENT_TYPE id means
  "the component is present, any value" (the synced table's 111-row
  census is the byIdOrThrow bound); its payload needs no value codec.
  The port retains every payload tag VERBATIM (the
  ComponentSerialization line: the per-record parse is a consumer's
  act), refusing the 0x00 root byte where vanilla's bridge throws
  "Expected non-null compound tag". mc-client untouched BY
  CONSTRUCTION. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 at their
  tolerances. Tests +1 (workspace 460 -> 461 passed / 0 failed / 3
  ignored): the two-lane dispatched walk (LEFT `damage` with its
  payload compound retained byte-identical, RIGHT the any-value shape
  over DAMAGE with the empty compound), and the refusal map extended
  IN PLACE — an unknown and a negative predicate-type id, an unknown
  and a negative any-value component id (each probe landing ON its
  lane, not dying of byte exhaustion), and the End-tag payload byte.
  The **the dispatched consume-effect registry's payload census —
  the consumable deferral retired (v0.3.110)**: the effects list the
  previous round pinned EMPTY now WALKS. The javap audit, one hop per
  type: `ConsumeEffect.STREAM_CODEC` = `StreamCodec.dispatch(
  ByteBufCodecs.registry(CONSUME_EFFECT_TYPE), ..)` — a `$29` VarInt
  byIdOrThrow (an unknown AND negative id throw, the port refuses at
  the same lane) then that type's own payload; the registry's clinit
  order is apply_effects 0, remove_effects 1, clear_all_effects 2,
  teleport_randomly 3, play_sound 4 — and the registry NAMES carry
  no "status" infix (the wiki-era "apply_status_effects" is not the
  wire's — the fourth name-vs-wire divergence this family has shown).
  The payloads: apply_effects = `composite(MobEffectInstance.
  STREAM_CODEC.list(), FLOAT probability)`; remove_effects =
  `composite(ByteBufCodecs.holderSet(MOB_EFFECT))` (the wave-three
  tagged union, decoded in BOTH directions); clear_all_effects =
  `StreamCodec.unit(INSTANCE)` — ZERO bytes; teleport_randomly =
  `composite(FLOAT diameter)` (the DEFAULT_DIAMETER constant is the
  NBT-side default, not the wire's); play_sound = `composite(
  SoundEvent.STREAM_CODEC)` — the SAME tagged holder union as the
  consumable's sound lane (one codec, two call sites). The element
  record: MobEffectInstance = `composite(holderRegistry(MOB_EFFECT),
  Details.STREAM_CODEC)` where Details = `StreamCodec.recursive(
  composite(VAR_INT amplifier, VAR_INT duration, BOOL ambient, BOOL
  showParticles, BOOL showIcon, optional(recursive self)
  hiddenEffect))` — the record's component order confirmed by its
  bootstrap strings, the recursion mirrored walk-for-walk (the
  boxed `Option<Box<EffectDetails>>`). THE ROUND'S OWN CATCH: the
  negative holder-set tag probe PANICKED the decode (`with_capacity
  (negative as usize)` capacity overflow) — vanilla's `$31` reads
  `i = VarInt - 1` then `new ArrayList(Math.min(i, 65536))`, so a
  negative tag dies at the constructor's IllegalCapacity, never a
  panic; the same missing refusal sat in the delivered wave-three
  block-set lane (its "can never be negative" comment was wrong —
  the named lane only owns wire 0), and BOTH now refuse at the
  lane's own line. mc-client untouched BY CONSTRUCTION. GOLDENS
  12/12 at rms 0.000, untouched BY CONSTRUCTION. Tests +2 (workspace
  459 -> 460 passed / 0 failed / 3 ignored): the consumable's
  effects list walking all five dispatched types in one entry chain
  (the recursive details two levels deep, the named AND direct
  holder sets, the zero-byte unit lane, the registry sound), the
  refusal map (unknown dispatch id 5, negative dispatch id, a wire
  ending before the dispatch id, a negative effect id inside
  apply_effects, a negative holder-set tag — the probe that caught
  the overflow), and the empty-list zero walk.
  The **the held-item's stat surface decodes — the wave four
  (v0.3.109)**: attribute_modifiers (16), food (23) and consumable
  (24) leave the patch-value refusal list. The javap walk: 16 =
  `composite(Entry.STREAM_CODEC.list(), ..)` — a VarInt count then
  count × FOUR-lane entries: `holderRegistry(ATTRIBUTE)` (the $29
  byIdOrThrow VarInt — negative refuses, unknown retained verbatim,
  the ENCHANTMENTS line), `AttributeModifier.STREAM_CODEC`
  (composite of an Identifier UTF id, `ByteBufCodecs.DOUBLE` = a
  BIG-ENDIAN f64 amount — the DOUBLE/INT/LONG trap family again, the
  construction site is the shape — and the Operation idMapper
  ordinal), `EquipmentSlotGroup.STREAM_CODEC` (the same idMapper
  shape, ANY=0 .. SADDLE=10), and the 26.2-NEW
  `ItemAttributeModifiers$Display.STREAM_CODEC` — a DISPATCH: the
  Type ordinal (DEFAULT=0, HIDDEN=1, OVERRIDE=2) then the variant's
  payload (default/hidden are `StreamCodec.unit` = ZERO bytes, the
  override one bare component tag). The round's fidelity find: the
  idMapper tables are `ByIdMap.continuous(.., OutOfBoundsStrategy.ZERO)`
  whose `lambda$continuous$0` maps EVERY key outside `[0, len)` —
  negative AND out-of-range — to values[0]; they NEVER refuse, so
  the port retains the data ordinals verbatim (refusing would
  invent a data flow vanilla does not have) and the DISPLAY ordinal,
  being FRAMING (it selects the payload walk), fills to Default
  in-decode exactly as vanilla's map does — pinned by a test. (The
  banked corollary: the delivered Rarity lane's negative-ordinal
  refusal is STRICTER than vanilla's continuous map — a candidate
  fidelity-fix round, not silently extended.) food 23 =
  `composite(VAR_INT, FLOAT, BOOL)` — nutrition is a VAR_INT in 26.2
  (the record holds `int nutrition`; the wiki-era float instinct is
  not the wire's shape) with no clamp anywhere. consumable 24 = a
  FIVE-lane composite (f32 consume_seconds, the ItemUseAnimation
  ordinal NONE=0 .. BRUSH=9, `SoundEvent.STREAM_CODEC` =
  `ByteBufCodecs.holder(..)` — a TAGGED union where wire 0 is the
  DIRECT `{UTF id, f32 range}` lane and wire tag >= 1 the `(tag - 1)`
  registry id, a negative tag refusing where vanilla's lookup
  throws — one particle byte) with the effects list pinned EMPTY:
  `ConsumeEffect.STREAM_CODEC` = `ByteBufCodecs.registry(
  CONSUME_EFFECT_TYPE)` is a DISPATCHED registry (each entry = a
  type id + that type's own payload; the six known types —
  apply_status_effects, clear_all_status_effects, play_sound,
  remove_status_effects, teleport_randomly — are a census wave of
  their own) and the unframed wire cannot be skipped honestly (the
  wave-three partial-matcher line). The decode census: TWENTY-FOUR
  of the registry's 111 component types carry value codecs; the
  other 87 refuse. Tests +2 (workspace 457 -> 459 passed / 0 failed
  / 3 ignored): the FOUR-lane stat composite bit-exact (three
  entries covering the default/hidden/override displays, the
  negative operation -3 and slot 42 RETAINED verbatim, the negative
  food nutrition RETAINED — the record constructor clamps nothing),
  the out-of-range Display ordinal 9 filling to Default (the framing
  fill), and the consumable's five lanes with the sound union in
  BOTH directions plus its refusal map (a non-empty effects list, a
  negative sound holder tag); the patch refusal probe moved 16 -> 5
  (the dense-registry slot between unbreakable 4 and custom_name 6
  that no wave has claimed). GOLDENS UNTOUCHED BY CONSTRUCTION:
  12/12 at rms 0.000.
  The **the adventure-mode pair rides the patch — the wave three
  (v0.3.108)**: can_place_on (14) and can_break (15) leave the
  patch-value refusal list, and the javap walk (four hops) shows they
  are the SAME record — `net.minecraft.world.item.AdventureModePredicate`,
  registered under two component ids, so one decode serves both. The
  record = `composite(BlockPredicate.STREAM_CODEC.list(), ..)` — a
  VarInt count over the NO-ARG `ByteBufCodecs.list()` (ceiling
  `Integer.MAX_VALUE`; readCount's ceiling check is the only guard, a
  negative count still dies at the constructor), then count × a
  FOUR-lane BlockPredicate: `optional(holderSet(BLOCK))` (the $31
  VarInt TAG — wire 0 is a NAMED set carried as its UTF identifier,
  wire tag >= 1 is a DIRECT set of `tag - 1` numeric holder ids, so
  the EMPTY direct set is wire tag 1 and a negative count cannot
  arrive), `optional(StatePropertiesPredicate)` (a
  `PropertyMatcher.list()` whose value lane is
  `ByteBufCodecs.either(exact, ranged)` — a ONE-BYTE readBoolean
  discriminator ($26), the exact matcher a bare UTF string, the
  ranged matcher BOTH bounds riding as optional UTF STRINGS ("10"),
  not doubles — the numeric NBT schema is not the wire's shape),
  `optional(NbtPredicate)` (a bare COMPOUND_TAG whose non-compound
  shape refuses — the custom_data DecoderException line), and
  `DataComponentMatchers` — the EXACT side decodes fully through the
  patch's added-entry walk (`TypedDataComponent.STREAM_CODEC.list()`),
  while the PARTIAL side (a dispatched predicate-type registry,
  `list(64)`) enforces its 64-entry cap and refuses any NON-EMPTY
  list: each dispatched type carries its own payload codec (damage,
  enchantments, potion contents — a census wave of their own) and the
  unframed wire cannot be skipped honestly. The named-set identifier
  is retained verbatim — vanilla resolves it `orElseThrow` against
  the synced block tags; this client carries no tag tables, so the
  strictness is a consumer's act (the VILLAGER_DATA line). The decode
  census: TWENTY-ONE of the registry's 111 component types carry
  value codecs; the other 90 refuse. Tests +2 (workspace 455 -> 457
  passed / 0 failed / 3 ignored): the four-lane decode bit-exact
  (named + direct sets, exact + ranged properties, the compound nbt,
  the exact matcher entry), the ALL-ABSENT predicate and the EMPTY
  list shapes, and the shape-refusal map (a negative predicate
  count, a negative block id, a non-compound nbt, an unknown exact
  id, a non-empty and an over-cap partial list); the patch refusal
  probe moved 14 -> 16 (attribute_modifiers). GOLDENS UNTOUCHED BY
  CONSTRUCTION: 12/12 at rms 0.000.
  The **the patch values grow their record codecs — the wave two
  (v0.3.107)**: three more component types leave the patch-value
  refusal list, each audited one hop from its own builder lambda.
  enchantments (13) = `composite(ByteBufCodecs.map(factory,
  Enchantment.STREAM_CODEC, VAR_INT), ..)` — a VarInt count then
  count x {the enchantment's registry id, the level}: the key id is
  the $29 byIdOrThrow holderRegistry VarInt (retained verbatim, no
  enchantment tables client-side), and the LEVEL rides the wire as a
  PLAIN VarInt — the `intRange(1, 255)` LEVEL_CODEC is the NBT-schema
  side, not the wire's, and clamping at decode would invent a data
  flow vanilla does not have. custom_model_data (17) =
  `composite(FLOAT.list(), BOOL.list(), STRING_UTF8.list(),
  INT.list(), ..)` — FOUR VarInt-count lists in composite order,
  whose colors ride `ByteBufCodecs.INT` = `readInt` = a BIG-ENDIAN
  i32 — NOT a VarInt (the same trap family as the LONG/VAR_LONG
  discovery: the codec's construction site is the shape, the field's
  name is not). tooltip_display (18) = `composite(BOOL,
  DataComponentType.STREAM_CODEC.collection(factory), ..)` — one
  hide-tooltip byte, then a VarInt count of component type registry
  ids, SELF-DELIMITING (one VarInt each, no value attached), so any
  non-negative id is retained verbatim — the DIRECTION/RARITY line —
  where the patch's added entries refuse unknowns because of the
  sizing problem. The shared `ByteBufCodecs.readCount` shape: it
  checks only the maxSize CEILING; a NEGATIVE count passes the check
  and fails at the collection/map constructor's negative capacity —
  the read fails either way, so the port refuses at the same lane.
  The decode census: NINETEEN of the registry's 111 component types
  carry value codecs; the other 92 refuse. Tests +2 (workspace 453 ->
  455 passed / 0 failed / 3 ignored): the three composites decoded
  bit-exact (the two-entry enchantment map, the four-list model data
  with its BE-i32 color, the hidden set) plus their EMPTY shapes, and
  the shape-refusal map (a negative count on each lane, a negative
  map key, a negative hidden id); the refusal test's
  known-but-unimplemented probe moved from 13 (now decodes) to 14
  (can_place_on).
  The **the items carry their state — ITEM_STACK decodes (v0.3.106)**:
  serializer 7 leaves the object-graph refusal list. The audit walked
  three bytecode hops: `ItemStack.OPTIONAL_STREAM_CODEC` = the
  anonymous `ItemStack$1` over `DataComponentPatch.STREAM_CODEC` —
  a VarInt COUNT whose `ifgt` makes count <= 0 the EMPTY stack with
  NOTHING else on the wire, a VarInt item registry id (`Item.STREAM_CODEC`
  = `holderRegistry(ITEM)`, the $29 byIdOrThrow), then the patch:
  `DataComponentPatch$3.decode` = VarInt ADDED count, VarInt REMOVED
  count (both zero = the EMPTY patch early return), added entries as
  {component type id, the type's OWN value}, removed entries as {type
  id} — the absence IS the payload. The component type itself rides
  `DataComponentType.STREAM_CODEC` = `StreamCodec.recursive(..)` =
  `ByteBufCodecs.registry(DATA_COMPONENT_TYPE)` — the same numeric
  byIdOrThrow VarInt, and its ids are `DataComponents.<clinit>`'s
  REGISTRATION order (the third registry this project has pinned by
  clinit order against the wiki-era instinct). The patch wire is
  UNFRAMED — no length prefix — so the decode carries the SIXTEEN
  primitive value codecs (four VarInt durability lanes, the two
  ZERO-BYTE unit lanes where the type's presence IS the payload, the
  bare component tags for custom_name/item_name, the compound-only
  custom_data — `COMPOUND_TAG`'s decode map is the DecoderException on
  any other shape, the BE f32, the three UTF identifier strings, the
  256-capped lore tag list, the idMapper rarity ordinal, the bool) and
  REFUSES the other ninety-five: an unknown id exactly where vanilla's
  byIdOrThrow throws, a known-but-unimplemented one because a value's
  byte length is only knowable through a codec the port does not
  carry — there is no honest skip. mc-client untouched BY
  CONSTRUCTION (the metadata BTreeMap is variant-agnostic). GOLDENS
  UNTOUCHED BY CONSTRUCTION: 12/12 at rms 0.000 (no golden flow
  carries metadata packets; the mesher untouched). Tests +2 (workspace
  451 -> 453 passed / 0 failed / 3 ignored): the item stack's three
  layers (both empty-stack sentinels incl. the negative count, the
  drift-behind-empty refusal, the empty patch header, the removal-only
  patch, and the FULL sixteen-type added patch in one entry — every
  codec shape decoded), the patch's refusal map (the unknown 999, the
  negative id, the known-but-unimplemented enchantments at 13, the
  negative added count, the lore list over its 256 cap, and the
  non-compound custom_data), and the ITEM_STACK entry absorbed into
  the twenty-two-serializer round-trip packet.
  The **the villagers carry their data — VILLAGER_DATA decodes
  (v0.3.105)**: serializer 18 leaves the object-graph refusal list.
  The javap walk pinned the composite from the 26.2 bytecode:
  `VillagerData.STREAM_CODEC` = `composite(holderRegistry(
  VILLAGER_TYPE), holderRegistry(VILLAGER_PROFESSION), VAR_INT,
  VillagerData::new)`, and `ByteBufCodecs$29.decode` is
  `VarInt.read` -> `IdMap.byIdOrThrow` — THREE numeric registry ids
  on the wire (not names), an id the registry does not know throws
  and fails the packet read. The entry retains the ids verbatim (no
  villager registry tables client-side; a consumer resolves them), a
  NEGATIVE id is outside every IdMap and refuses (byIdOrThrow's own
  direction), and the LEVEL is the record constructor's own
  `Math.max(1, level)` — the javap shows `iconst_1` feeding the
  `Math.max`, so composite decode (which calls `VillagerData::new`
  directly) already clamps: wire 0 is a stored 1, no consumer-side
  arithmetic. Tests +1 (workspace 450 -> 451 passed / 0 failed / 3
  ignored): the three-VarInt round-trip (also absorbed into the
  sixteen-serializer packet as entry 20), the constructor clamp
  (wire 0 and wire -7 both store 1), the negative type/profession
  refusals, and the truncated record refused.
  The **the metadata grows the name-tag lane — COMPONENT decodes
  (v0.3.104)**: serializers 5 and 6 leave the object-graph refusal
  list. The audit pinned the real wire shape through three bytecode
  hops: `ComponentSerialization.TRUSTED_STREAM_CODEC` =
  `fromCodecWithRegistriesTrusted` = `tagCodec(supplier)` + the
  NBT-ops adapter — ONE BARE network NBT tag on the wire (a
  TAG_String for plain text, a TAG_Compound for the structured
  `{text:..}` form, a TAG_List for siblings), "trusted" meaning the
  accounter is unlimited (the server's own packet); OPTIONAL_COMPONENT
  is the shared `optional` op over it (a boolean flag, then the tag
  only if present). nbt.rs grows the bare-tag PAIR —
  `read_bare_nbt`/`write_bare_nbt` (id byte + payload, no root
  framing) — beside the existing anonymous-root reader the chunk
  packets frame; the two entry points are a pinned TRAP: 0x0A through
  the bare reader is a legal EMPTY compound, not an error, so the
  wrong reader fails silently — the test pins the trap so a future
  refactor cannot merge them. The store is the TAG (the rich-text
  schema stays a rendering-time concern). GOLDENS UNTOUCHED BY
  CONSTRUCTION: 12/12 at rms 0.000. Tests +2 (workspace 448 -> 450
  passed / 0 failed / 3 ignored, RE-RUN on the pushed final tree):
  the bare round-trip across the component's four shapes (string,
  compound, list, scalar) plus the 0x0A trap, the dedicated
  component-wire test (the plain-text name, the styled name read back
  positionally), and the sixteen-serializer round-trip absorbing the
  component lanes (the refusal census drops COMPONENT —
  ITEM_STACK/VILLAGER_DATA/PARTICLE/200 still refuse).
  The **entities walk the world — the MOVE-family deltas land
  (v0.3.102)**: Move Entity Pos (generated id 53), Move Entity Pos and
  Rotation (id 54), Move Entity Rot (id 56) and Rotate Head (id 83) —
  the per-tick traffic the sync's teleports don't cover. The audit
  re-derived all FOUR wire formats from the 26.2 bytecode: Pos = a
  VarInt id + three SHORT deltas + BOOL; PosRot adds yaw/pitch angle
  BYTES (yaw THEN pitch, `Mth.unpackDegrees(b) = b * 360 / 256` — the
  Add Entity quantization); Rot drops the position lane entirely; and
  Rotate Head (whose read IS its private constructor) is ONE head-yaw
  byte. TWO traps named by the artifacts themselves: the generated id
  table puts the MINECART at 55 (`move_minecart_along_track`) — the
  wiki-era guess would have shipped Rot on the wrong id — and the
  head yaw is its OWN channel, riding beside the body packets, never
  inside them. The consumer is the interesting half: the vanilla
  client's `handleMoveEntity` runs the deltas through
  `VecDeltaCodec.decode(JJJ)`, and the javap walk shows that is NOT a
  naive `pos += delta / 4096` — the ALL-ZERO payload is the codec's
  own early return (the base comes back UNCHANGED), a ZERO lane keeps
  its axis EXACT (no quantization), and a non-zero lane moves the
  QUANTIZED base: `(Math.round(base * 4096) + delta) / 4096` — every
  real step snaps the accumulated position onto the 1/4096 grid. And
  `Math.round` is JAVA's round — ties toward POSITIVE INFINITY, where
  Rust's `f64::round` rounds halves away from zero, a one-grid-step
  difference on negative half-grid bases (the negative-coordinate
  class again). The model grows the four `apply` arms (unknown id
  refused — the spawn packet stays the ONLY creator; `EntitySnapshot`
  gains the head-yaw channel the spawn seeds and Rotate Head keeps
  current) plus the private `vanilla_round`/`delta_axis` pair
  carrying the bytecode's arithmetic verbatim. GOLDENS UNTOUCHED BY
  CONSTRUCTION: 12/12 at rms 0.000 (no golden flow carries entity
  packets; the mesher untouched). Tests +8 (workspace 435 -> 443
  passed / 0 failed / 3 ignored): the Pos layout (both short
  boundaries, the all-zero payload legal, the rot id refused on the
  pos parser), the angle unpack (-128 floors to -180.0, 127 lands at
  178.59375), the Rot packet's missing lane, the one-byte head read,
  the codec walk (the moved axes snapped to the grid, the zero lane
  unquantized, the idle walk a no-op), the Java tie direction
  (Math.round(-0.5) = 0 pinned at 2^-13), the rotation family (turn
  in place, rotation asserted on the zero walk), and the head
  channel (spawn-seeded, Rotate-Head-kept, body-independent, unknown
  ids refused by every mover).
  The **night-vision color is STATE — the fourth decoded light attribute,
  under the effect-state factor (v0.3.99)**: the `night_vision_color`
  consumer closes the four-attribute arc. The audit pinned BOTH ends of
  the shader term: the COLOR rides the probe like its three siblings
  (`LightmapRenderStateExtractor.extract` reads
  `EnvironmentAttributes.NIGHT_VISION_COLOR` — the bytecode default
  `ldc -6710887` = 0xFF999999 = **#999999 gray**,
  `spatiallyInterpolated().syncable()`), while the FACTOR is the
  player's EFFECT STATE, not a timeline: `hasEffect(NIGHT_VISION)` ->
  `GameRenderer.nightVisionScale` (1.0, or `0.7 + sin((duration -
  partialTick) · π · 0.2) · 0.3` while the effect ends within 200 ticks
  — the fade-out flicker), conduit power + water vision ->
  `getWaterVision()`, else exactly **0.0**. lightmap.fsh consumes
  `max(AmbientColor, NightVisionColor * NightVisionFactor)` — with the
  factor at this client's floor the term is the ambient base BIT FOR
  BIT for any tint (`max(A, C·0.0) = A`, a color is non-negative), so
  the attribute rides the fold structured for the day the effect system
  lifts it. Implementation the cloud_fog way: the generator column (the
  regenerated table's diff exactly the struct doc + 4 rows — NO vanilla
  dimension states it, the honest all-None shape), the model accessor
  `night_vision_color()` (the three-branch precedence), the materialize
  stamp beside its lightmap siblings, and the mesher's
  `night_vision_base(ambient, tint, factor)` at both render paths — the
  factor rides as a parameter so the term's contract is pinnable at a
  lifted state. GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 at rms 0.000.
  Tests +3 (workspace 422 -> 425 passed / 0 failed / 3 ignored): the
  accessor precedence (the built-in all-None rows pinned), the
  materialize stamp (the absent shape + a stated custom entry verbatim),
  and the fold contract (the factor-0 identity exact-equal for None AND
  Some, the lifted factor-1.0 arithmetic — the gray default's channel
  lift, the stated white's saturation, the per-channel max that cannot
  darken — and the render-path witness: a stated color keeps every
  pixel at the floor).
  The **sky-light color is STATE — the last light attribute's fold
  (v0.3.98 — INTEGER arithmetic, and the moonlight pairs)**: the
  sky_light_color consumer, the third of the four decoded light
  attributes. The audit pinned the modifier's exact semantics:
  `ColorModifier.MULTIPLY_RGB` -> `ARGB.multiply(int, int)` — the -1
  white default short-circuits to the other operand, else per-channel
  `imul` / `idiv 255` — the probe's folded value IS an `Integer` the
  extractor unpacks through `ARGB.vector3fFromRGB24`, and the color
  track's keyframe lerp is `ARGB.srgbLerp` (the byte-space operand the
  fold consumes). The generated table's honest rows: the END states
  **#ac60cd** and the NETHER **#7a7aff** (the moonlight pairs), the
  overworlds `None` — v0.3.97's discovery, now a column. Implementation
  the cloud_fog way: the generator column + the `TL_DAY_SKY_LIGHT_COLOR`
  track array, the model accessor (the three-branch precedence), the
  materialize stamp beside its lightmap siblings, and the mesher's
  fold at both render paths — `None` = the track verbatim (the exact
  shipped expression), `Some(stated)` = per-channel integer multiply
  against `timeline::sky_light_color_bytes` (the byte-space sampler).
  GOLDENS UNTOUCHED BY CONSTRUCTION: 12/12 at rms 0.000 (every golden
  world overworld/unstated — the -1 default's white identity). Tests
  +4 (workspace 418 -> 422 passed / 0 failed / 3 ignored): the accessor
  precedence (the built-in end/nether moonlight pairs pinned), the
  materialize stamp, the fold contract (the integer floor 61 vs the
  float 61.24 — the arithmetic is the bytecode's, not a float
  approximation), and the byte sampler (the dusk midpoint 188.5 -> 189).
  The **sky-light factor folds under the timeline (v0.3.97 — the
  attribute-system fold, and the end/nether discovery)**: the
  sky_light_factor consumer. The audit started at the mesher's sky
  term and climbed the vanilla stack: `EnvironmentAttributeProbe` ->
  `EnvironmentAttributeSystem.getValue` -> the sampler's LINEAR FOLD —
  `value = base; for layer in layers { value = layer.apply(value) }`,
  layers in add order: the dimension's attributes as a CONSTANT layer
  (the OverrideModifier — a stated value REPLACES the default), the
  biome positional layers, the dimension's TIMELINES last (the
  TimeBased layer, the track's modifier — the day track's is
  `multiply`, the raw jar JSON). So the stated
  `visual/sky_light_factor` is the multiply-BASE and the day cycle
  scales it: `stated.unwrap_or(1.0) x track(tick)`. AND THE
  DISCOVERY: the raw dimension JSONs state the attribute — the END
  and the NETHER pin **0.0** (`the_end` also states
  `sky_light_color #ac60cd`, the nether `#7a7aff`), so vanilla's own
  verdict is that NO sky light reaches those lightmaps; the
  v0.3.95 round's "no vanilla dimension states the four" held only
  for the tint and night-vision, and the mock registry (which
  carried no keys) diverged from the real jar — the autopush-95
  lesson again. The implementation: the generator parses the
  attribute into the built-in table (the honest shape now carries
  `Some(0.0)` rows for the end/nether), the model grows
  `WorldModel::sky_light_factor()` (the three-branch precedence) and
  `sky_factor()` folds the stated base under the curve; the
  snapshot stamps it beside its lightmap siblings and the mesher's
  two paths multiply `stated.unwrap_or(1.0) * track` (the
  materialize transport, the lightmap family's own — not the App
  pin); the mock nether element grew the REAL stated pair
  (`#7a7aff` + 0.0) and the v0.3.95 real-element assertions now pin
  the nether's Some(0.0) and the overworld's absent shape. `World`
  drops `Eq` (the stated f32 arrives — derives follow the data).
  GOLDENS UNTOUCHED BY CONSTRUCTION: every golden world is
  overworld/unstated, and `x * 1.0` is an exact f32 identity —
  12/12 at rms 0.000, byte-identical. Tests +2 (workspace 416 ->
  418 passed / 0 failed / 3 ignored): the fold contract (the built-in
  nether's 0.0, the custom 0.5 halving the day curve, the silent
  sync's identity) and the materialize stamp; the skip-sync end test
  now pins the zero factor the jar itself states.
  The **four light attributes ride the wire (v0.3.95 — the syncable
  sweep's DATA round)**: the mc-protocol expansion. The audit re-ran
  on the 26.2 jar (EnvironmentAttributes.<clinit>): four syncable
  VISUAL attributes remained undecoded — `visual/block_light_tint`
  (RGB_COLOR, default -10100 = #FFD88C, the warm block-light cast),
  `visual/sky_light_color` (RGB_COLOR, default -1 = #FFFFFF),
  `visual/sky_light_factor` (FLOAT/UNIT_FLOAT, default 1.0) and
  `visual/night_vision_color` (RGB_COLOR, default -6710887 = #999999)
  — all four `.spatiallyInterpolated().syncable()`, so all four RIDE
  the registry sync, and NO vanilla dimension states any of them (the
  four dimension JSONs carry none of the keys). The round is DECODE
  ONLY, deliberately: `DimensionTypeEntry` grows the four fields (the
  three rgb_color entries through the ambient-light `parse_rgb_hex`
  shape, one FLOAT through `as_f32`; ABSENT reads None per field,
  never a guess) with a custom-element wire test stating all four and
  the real overworld/nether elements pinned to the absent shape — but
  no model accessor and no consumer yet, because an accessor's
  three-branch precedence exists to serve a consumer and an unread
  column is speculation; when a consumer arrives (the lightmap's block
  light is the obvious first), the round threads the built-in table
  and the accessor the cloud_fog way. `sky_fog_end_distance` is
  honestly deferred the same way: it has NO consumer in this client
  (the 512.0 hits are projection far-planes), so it waits for its
  consumer round rather than riding half-threaded. Goldens untouched
  BY CONSTRUCTION (no render code moved). Tests +1 (workspace 412 ->
  413 passed / 0 failed / 3 ignored).
  The **cloud fog rides the stated attribute (v0.3.94 — the
  cloud_fog_end_distance consumer)**: the remaining-syncable-attributes
  sweep's first consumer. The audit re-ran on the 26.2 jar
  (EnvironmentAttributes.<clinit> + the four dimension JSONs): the
  visual/fog pair has two single-attribute siblings — sky_fog_end
  (default 512.0) and cloud_fog_end_distance (default 2048.0), both
  FLOAT and both `.syncable()` — and NO vanilla dimension declares
  either (the fogged pair is the nether's alone), so the field rides
  only a custom server's statement. The round picked the attribute the
  client already consumed as a HIDDEN LITERAL: the cloud layer's
  FogCloudsEnd was `min(cloudRange * 16, 2048.0)` hardcoded at the
  render site — the sweep's whole point is that an official attribute
  must live as STATE, not as a buried number. The implementation is
  the fog-pair's shape minus the pairing rule: the generator parses
  `minecraft:visual/cloud_fog_end_distance` into the built-in table
  (four None rows — the bytecode default's shape; the regenerated
  table's diff is exactly the 4 rows + the struct docs), mc-protocol's
  `DimensionTypeEntry` grows the same field decoded from the flat
  attributes compound (ABSENT reads None — the consumer's default; a
  single FLOAT has nothing to pair with), the model grows
  `WorldModel::cloud_fog_end_distance()` (the three-branch precedence:
  the synced statement wins, the built-in table serves skip-sync, None
  is the default's verdict), and the consumer caps the cloud fog
  through `sky::clouds_fog_end(range, stated)` — `min(cloudRange * 16,
  stated.unwrap_or(DEFAULT_CLOUD_FOG_END))`: a stated smaller end wins
  verbatim, a stated larger one never lifts the render-distance cap.
  The App pins the value at level entry like the fog pair (both join
  sites resolve through the model; the procedural re-entry drops it).
  Goldens untouched BY CONSTRUCTION: every golden world is
  overworld/unstated — None rides the 2048.0 default, byte-identical
  with the retired literal; 12/12 pass. Tests +3 (workspace 409 -> 412
  passed / 0 failed / 3 ignored): the wire stated-FLOAT decode (a
  custom element's attributes compound; the real overworld/nether
  elements assert the absent shape), the model accessor's precedence
  (the built-in table's four None rows, the synced truth winning, the
  silent sync reading None), and the cap vectors (the default, the
  render-distance cap firing first, the stated smaller end winning,
  the stated larger end never lifting the cap).
  The **synced ambient rides the wire (v0.3.89 — the server's truth)**:
  the ambient arc's wire half closes. The audit ran first-hand on the
  26.2 jar: `DimensionType.NETWORK_CODEC` builds on
  `EnvironmentAttributeMap.NETWORK_CODEC` — the attributes field RIDES
  the registry sync (the network codec xmaps the same dispatchedMap the
  JSON uses, filtered to the SYNCABLE attributes, and the ambient
  attribute's builder chains `.syncable()`); the map codec is a FLAT
  compound keyed by the full attribute ids (the JSON shape, learned in
  v0.3.88, holds on the wire), and the rgb_color type serializes the
  `"#rrggbb"` STRING form (`ExtraCodecs.hexColor` — the `#` prefix
  REQUIRED, exactly six digits). mc-protocol's dimension_type decode
  grows `ambient_light_color: Option<[u8; 3]>` with the hand-rolled
  `parse_rgb_hex` (dependency-free, the base64/MD5 precedent; every
  non-`#rrggbb` shape is None — never a guess). The model accessor
  grows the dimension_min_y discipline: the SYNCED entry is the
  server's truth (its value wins over the built-in; its SILENCE reads
  the bytecode-default black — the vanilla EnvironmentAttribute
  fallback); the built-in table serves ONLY the skip-sync case (no
  dimension table at all); a live sync omitting the key keeps None.
  The mock registry fixture now carries the attribute, riding the REAL
  e2e packet path. Goldens untouched BY CONSTRUCTION: no render-path
  change (the mesher/lightmap never see this round), the fallback
  values are bit-identical — 12/12 pass at rms 0.000 on the rebuilt
  release binaries. Tests +2 (workspace 396 → 398 passed / 0 failed /
  3 ignored): the hex vectors (three vanilla colors + the refusals —
  no prefix / short / the argb form / non-hex / empty) and the
  synced-truth precedence (no sync → built-in; sync with the value →
  it wins; sync silent → black, its own truth; sync without the key →
  None, never the built-in).
  The **lightmap's AmbientColor base lands (v0.3.88 — the world has a
  floor)**: the lightmap.fsh line's LAST term is consumed — the shader's
  `color = max(AmbientColor, nightVisionColor)` starting color, the base
  the sky/block terms are added ON TOP of (not a final floor). The audit
  ran first-hand on the 26.2 jar: `EnvironmentAttributes.<clinit>` pins
  the attribute DEFAULT (`visual/ambient_light_color`,
  `defaultValue(-16777216)` = 0xFF000000 — black), the four built-in
  `dimension_type` JSONs state it per dimension (`attributes` is a FLAT
  compound — the keys are the full namespaced paths verbatim:
  overworld `#0a0a0a`, overworld_caves `#0a0a0a`, nether `#302821`,
  end `#3f473f`), and no timeline track overrides it (a sweep of the
  jar's timeline/dimension data: ambient appears ONLY in the four
  dimension JSONs). NightVisionFactor is 0 at this client's state, so
  the max reads the ambient color itself. The generator parses the
  attribute into `DimensionTypeDef.ambient_light_color` (absent =
  the bytecode-default black — the honest no-op), the model exposes it
  through the built-in table by `current_dimension` (the synced wire
  decode is the documented next consumer, the sky_light_factor track's
  precedent; an unknown custom key keeps `None` — the black default,
  never a guess), `materialize` stamps it into the world snapshot, and
  `lightmap_combine` starts its sum from the converted base. GOLDENS
  untouched BY DATA AND BY DEFAULT: the procedural worlds carry no
  light layer (the legacy early return wins before the base is ever
  read) AND no ambient opinion (None = black = the pre-ambient
  arithmetic bit for bit) — 12/12 pass at rms 0.000 on the rebuilt
  release binaries. Tests +4 (workspace 392 → 396 passed / 0 failed /
  3 ignored): the generator's parse (stated #0a0a0a vs the nether
  fixture's absent-attribute black), the model accessor's table pins
  (the three vanilla floors + the None verdicts), the materialize
  stamp, and the mesher witnesses (the overworld lift is exactly
  10/255 per channel over the derived black arithmetic, the nether
  floor is per-CHANNEL, the legacy early return wins with a stated
  floor riding nothing, day sky-15 still clamps to the legacy
  full-bright over the floor, and the end's #3f473f night cell adds
  the sky sliver per channel over its floor).
  The **lightmap's BlockLightColor lands (v0.3.87 — the torch is warm,
  not grey)**: the block term's white stand-in retires, completing the
  `lightmap.fsh` line the v0.3.86 round left open. The audit chain ran
  fresh and first-hand against the official 26.2 jar: the shader source
  itself (`assets/minecraft/shaders/core/lightmap.fsh`) carries the
  formula VERBATIM — `BlockLightColor = mix(BlockLightTint, vec3(1.0),
  0.9 * parabolicMixFactor(block_level))`, `block_brightness =
  get_brightness(block_level) * BlockFactor` — where the parabola
  `(2·level−1)²` rides the RAW normalized level (pure `#ffd8ac` warmth
  at mid levels, near-white at BOTH ends: the level-0 mix is
  irrelevant since its brightness is 0, the level-15 torch is
  white-hot). `LightmapRenderStateExtractor.extract` pins
  `blockFactor = blockLightFlicker + 1.4` — the flicker is a mean-zero
  random walk ((r1−r2)·r3·r4·0.1 decayed ×0.9/tick, the torch
  shimmer), so the deterministic client pins the flicker's EXPECTATION
  (0) and keeps the constant 1.4, the mean of the vanilla range (the
  shimmer itself is the documented non-goal, the end-flash precedent).
  `EnvironmentAttributes.<clinit>` pins the tint default first-hand:
  `defaultValue(sipush -10100)` = 0xFFD8AC, and no dimension JSON or
  timeline track overrides it — the default IS the whole value in
  every dimension the registry carries. The mesher's
  `lightmap_combine` grows `block_light_color` + `parabolic_mix_factor`
  and collapses its three clamped branches into ONE shape — the
  shader's single final `clamp(color, 0, 1)` — with the block term
  `BlockLightColor · get_brightness(block) · 1.4` per channel; the
  both-terms branch stays bit-identical (same ops, same order), the
  sky-only branch gains a no-op clamp (every existing pin under 1.0),
  the block-only branch reads the tinted factored term. The verdicts
  move honestly: a block-14-only torch now saturates red AND green
  (1.0889, 1.0352 → clamped) with blue riding under (0.9740) — the
  warm torch look; a full-sky torch-lit face at night saturates the
  clamp entirely (fully lit, the vanilla verdict); day sky-12 over
  block-10 sums to 29/30, still "the additive sum, not the max".
  The goldens are untouched — BY DATA: the procedural golden worlds
  carry NO block-light opinion (the `lm = 1.0` legacy path), so the
  tint lands on nothing they render; 12/12 pass, every one rms 0.000
  through verify_goldens.sh on the rebuilt release binaries. Tests +2
  (workspace 390 → 392 passed / 0 failed / 3 ignored): the parabola
  pins (0 at the mid, 1 at both ends, the 7/8 symmetry, red constant,
  the derived extreme mix) and the full-channel mesher witness (block
  7 all-under-clamp per channel, block 14 the red/green saturation
  verdict); the three block-term pins in the riding tests moved WITH
  the formula they ride (the autopush-88 honesty precedent).
  The **lightmap's SkyLightColor lands (v0.3.86 — the moonlight is
  blue, not grey)**: the carried candidate's javap audit ran fresh
  against the official 26.2 jar and the architecture finding is the
  round's backbone — `ClientLevel.getSkyColor` NO LONGER EXISTS: the
  whole old cos-darkening + sunrise-blend formula was retired into the
  environment-attribute system, and `SkyRenderer.extractRenderState`
  now reads the final sky color straight from
  `EnvironmentAttributes.SKY_COLOR` (one probe per attribute: SUN_ANGLE,
  MOON_ANGLE, STAR_ANGLE, STAR_BRIGHTNESS, SUNRISE_SUNSET_COLOR,
  MOON_PHASE, SKY_COLOR — degrees × 0.017453292f, RGBA ints), exactly
  the data-driven chain the client already implements. The dimension
  bases were re-verified BIT-FOR-BIT from the real dimension_type JSONs
  (`attributes.minecraft:visual/sky_color "#78a7ff"` = DIM_SKY_COLOR,
  `visual/fog_color "#c0d8ff"` = DIM_FOG_COLOR, `visual/cloud_color
  "#ccffffff"` = CLOUD_BASE_COLOR, `visual/cloud_height 192.33` — all
  four equal), and the javap static-init pins the attribute DEFAULTS:
  `visual/sky_light_color` = -1 (0xFFFFFFFF, WHITE) — the overworld
  carries NO base, so the day timeline's `multiply` track IS the whole
  value; `visual/block_light_tint` = -10100 (0xFFD8AC, the warm torch
  tint — the documented NEXT consumer); `visual/sky_light_factor` = 1.0
  (already consumed). THE IMPLEMENTATION: the day timeline's
  `visual/sky_light_color` track (white by day → `#7a7aff` through the
  night plateau — the moonlight blue) is extracted into
  `timeline::sky_light_color` (the same cyclic-linear `evaluate` walk,
  raw 0..=255 channel space, one final /255 — the sky.rs color-track
  convention), and the mesher's scalar lightmap combine generalizes to
  the SHARED per-channel core `lightmap_combine` (both the cube and the
  leaf-litter paths call it — generalize the core, not the copy):
  `SkyLightColor * get_brightness(sky) * SkyFactor + BlockLightColor *
  get_brightness(block)`, clamped at 1.0 per channel; the block term
  keeps the white stand-in (the `#ffd8ac` parabolic mix is next). At
  the white day tint every branch is bit-identical to the scalar
  arithmetic it replaced; at night the sky-lit surfaces tint BLUE — the
  first time the night world reads moonlit instead of darkened-white.
  The goldens are untouched — BY DATA: the procedural golden worlds
  carry NO light layer (the `lm = 1.0` legacy path — the night look
  comes from the sky.rs clear colors, which this round never touches),
  so the tint lands on nothing they render; 12/12 pass, every one rms
  0.000 through verify_goldens.sh. Tests +2 (workspace 388 → 390
  passed / 0 failed / 3 ignored): the track pins (day white, the
  #7a7aff plateau exact at BOTH the golden night tick 14010 and the
  riding tests' 18000, the dusk midpoint per channel, the dawn wrap)
  and the full-channel mesher witness (the night sky term is
  blue-tinted per channel against the day-derived product; the riding
  `sky_factor_scales_only_the_sky_term` updated honestly to the tinted
  night arithmetic — red 0.8926 under the clamp, blue 1.0 clamped, the
  block-only and day branches byte-equal to before).
  The **profile-metadata consumer lands (v0.3.85 — the login's skin
  model picks the arm)**: the slim arc completes its natural lifecycle —
  the `--slim-arm` stand-in gets the REAL selection source. The login
  success's profile properties (parsed since the login arc but consumed
  by nothing) now yield the skin model: `mc_protocol::login::
  profile_skin_model` reads the `textures` property's base64 JSON and
  answers `Some("slim")` ONLY for `SKIN.metadata.model == "slim"` — no
  property (every offline server), no SKIN entry, no metadata, an
  unknown model value, or ANY malformed input (a hand-rolled RFC 4648
  base64 decoder — the protocol layer stays dependency-free, like the
  MD5 — refusing bad length/padding/alphabet; serde_json refusing bad
  JSON) is `None`, the classic wide default: untrusted profile data
  means "no opinion", never a panic and never a guess. The fact threads
  the vanilla join chain — `PlayOutcome::Joined` → `PlaySession` → the
  client join worker → `LoginResult::Loaded.slim_arm` → `App` — so a
  real login's metadata IS the first-person arm's kind (an explicit
  `--slim-arm` remains the no-profile stand-in, overridden by a
  profiled login). The goldens never connect, so the default path is
  untouched — BY DATA AND BY MATRIX: 12/12 pass, every one rms 0.000,
  through verify_goldens.sh on the rebuilt release binaries. Tests +4
  (workspace 384 → 388 passed / 0 failed / 3 ignored; mc-protocol 78 →
  82): the base64 RFC-vector pin (plus the malformed-shape refusals),
  the skin-model shapes (slim / no-metadata / unknown value / wrong
  property / malformed base64 / non-JSON / no-SKIN — each its own
  refusal), the properties-capable login-success encoder round-trip
  (write → parse → the profile lands on the model), and the
  END-TO-END mock-server ride (a real TCP join whose login success
  carries the textures property lands `session.skin_model == Some
  ("slim")`; the empty-property join lands None) plus the App-level
  consumer assertion (a slim login sets `app.slim_arm`).
  The **slim arm variant lands (v0.3.84 — the Alex model, data-driven
  end-to-end)**: the first-person arm gains the SLIM (3×12×4) geometry —
  javap-evidenced from the official 26.2 jar's
  `PlayerModel.createMesh(CubeDeformation, slim = true)` bytecode: the
  right arm is `texOffs(40, 16)` (UNCHANGED from the wide model),
  `addBox(-2, -2, -2, 3, 12, 4)`, pivot `(-5, 2, 0)` (UNCHANGED — the
  whole transform chain is kind-independent; only the box shrinks one
  model unit inward, so the OUTER plane x = +1 is shared and the INNER
  plane moves x = -3 → -2). The generator extracts BOTH official skins
  from the jar (wide `steve.png` + slim `alex.png`, each sha1-pinned
  verbatim), emits the slim constants + the parameterized unwrap
  (`arm_faces_for` — the same `ModelPart$Cube` layout at width 3: the
  strip right(outer)→front→left(inner)→back shifts with w, front 3 wide,
  left at u+d+w, back at u+d+w+d), and materializes alex.png into the
  runtime root next to steve.png. The client (`hand.rs`) gains
  `ArmKind::{Wide, Slim}` + `load_kind` (per-kind sha1 gate — each file
  verifies against ITS OWN generated digest) with the quad builder
  parameterized by (origin, box, shared texOffs): the wide path performs
  the SAME arithmetic it always did (the pre-variant `build_quads_at`
  delegates bit-identically — the bit-identity and sweep tests ride it
  unchanged). Vanilla picks the variant from the profile's texture
  metadata; this client has no profile yet, so the selection is explicit
  (`--slim-arm`, the same deterministic stand-in pattern as
  `--held-item`) — the DEFAULT stays the wide classic. GOLDENS:
  untouched — BY CONSTRUCTION AND BY MATRIX: the default never selects
  slim, and the full matrix re-verified through verify_goldens.sh on the
  rebuilt release binaries, 12/12 pass, every one rms 0.000. The e2e
  flag was probed on the binary (`--screen world` wide vs slim: different
  sha1s). Tests +6 (workspace 378 → 384 passed / 0 failed / 3 ignored;
  mc2rs-gen skingen 2 → 3, mc-client hand 17 → 22): the javap box pin +
  slim unwrap rects, the per-quad UV-rect reconstruction (min/max over
  the Polygon corner assignment, robust to the V-flipped wrist face),
  the inner-plane/outer-plane geometry witness (EAST quad corners
  kind-identical, WEST moves; EAST UV shifts), the real-alex hermetic
  load, and the PROBE-FIRST pixel pin — the slim rest pose moves EXACTLY
  2534 pixels vs wide in the in-test render (the moved plane + the
  shifted unwrap texels), probed before pinning (the probe discipline
  caught the hermetic-skip trap: the test asset root is anchored at the
  workspace root via CARGO_MANIFEST_DIR, not the crate-relative path
  that silently skips). The generator refuses a jar without alex.png
  (no fabricated defaults).
  The **timeline-tag single truth lands (v0.3.83 — the client's private
  tag walk retires into the registry's resolver)**: the skip-sync
  fallback (`WorldModel::built_in_timeline_curve`) now resolves the
  dimension's `#minecraft:in_overworld` reference through
  `mc_registry::flatten_timeline_tag` — the vanilla built-tag verdict
  the registry has owned since v0.3.80/84 (file-order walk, depth-first
  `#`-nested expansion in place, first-occurrence dedup, fail-closed on
  unknown tag / unknown member / cycle, the one generic core behind BOTH
  tag tables) — deleting the hand-rolled `expand_built_in_timeline_tag`
  that had duplicated the semantic client-side since before the registry
  existed. The consolidation is the tint chain's pattern (entry_tints /
  built_in_tints share resolve_tints) applied to the tag chain: ONE
  expansion truth, and the semantic drift it carries is documented — the
  old walk silently SKIPPED unknown members (its own no-blind-filter
  invention) while the registry verdict refuses WHOLE (the vanilla
  TagLoader shape); on the real 26.2 jar the two agree member-for-member
  (the registry sweep proves every timeline-tag member resolves), so the
  observable curve resolution is unchanged, and a drifted jar now
  refuses instead of partially resolving. GOLDENS: untouched — BY DATA:
  the resolution output is pinned identical (member order
  `[villager_schedule, day, moon, early_game]`, the day-curve first-
  carrier pick), and the full matrix re-verified through
  verify_goldens.sh on the rebuilt release binaries, 12/12 pass, every
  one rms 0.000. Tests: the direct-walk test is REPLACED one-for-one by
  `the_built_in_tag_path_reads_the_registry_flatten` (the member order
  through the resolved TimelineDefs, the single-ref end/nether forms,
  the unknown-tag refusal, and the end-to-end chain: overworld resolves
  the DAY curve's keyframes byte-equal to the generated carrier with
  period 24000, the end resolves NOTHING) — workspace stays 378 passed
  / 0 failed / 3 ignored.
  The **timeline-tag resolution lands (v0.3.82 — one resolver semantic,
  two validated tables)**: `mc-registry::flatten_timeline_tag` applies
  the v0.3.80 built-tag verdict (file-order walk, depth-first `#`-nested
  expansion in place, first-occurrence dedup, fail-closed on unknown
  tag/unknown ref/cycle) to the jar's `data/minecraft/tags/timeline/*.json`
  table — the same generic core, now validated against the TIMELINES
  registry (every concrete member must resolve). This is the resolution
  the dimension types imply: a dimension's `timelines` field names a TAG
  verbatim (`"#minecraft:in_overworld"` — the raw form the config
  registry carries), and the timelines that apply to it are the tag's
  flattened members. The real-data pins: in_overworld's FIRST member is
  the nested `#minecraft:universal`, so universal's expansion
  (`villager_schedule`) lands AT THE HEAD before the tag's own three
  concrete members (day, moon, early_game — file order); in_end and
  in_nether are the single-ref form and build to universal's flatten
  exactly; the unknown-tag and raw-`#` forms refuse. GOLDENS: untouched
  — BY DATA: grep proves zero render-path references; the full matrix
  re-verified through verify_goldens.sh on the rebuilt release binaries,
  12/12 pass, every one rms 0.000. Tests +2 (workspace 376 → 378 passed
  / 0 failed / 3 ignored; mc-registry 76 → 78): the universal-ref
  flatten pins and the all-four-tags sweep with member-by-member
  registry validation.
  The **swing-arc sweep lands (v0.3.81 — the swing chain's pixel anchors
  stretch from one progress to the whole curve)**: the carried-since-
  v0.3.76 candidate finally banked. Both first-person term curves are
  extracted into named functions (`hand::swing_terms` — the bare arm's
  f10..f14 of `renderPlayerArm`; `hand::item_swing_terms` — the held
  item's f5s/f6s/f7s/f5/f6 of swingArm + applyItemArmAttackTransform),
  an op-for-op refactor the existing bit-identity tests plus the full
  golden matrix prove byte-safe (12/12 pass, every one rms 0.000, on
  the REBUILT release binaries that embed the refactor). The sweep then
  pins BOTH curves BIT-EXACTLY at seven progresses (0, 0.1, 0.25, 0.5,
  0.75, 0.9, 1.0) against an independently computed oracle
  (`scripts/swing_terms_oracle.py`, a numpy f32 emulation of the
  f64-sin/sqrt-narrowed-to-f32 chain): 70 bit assertions, matched on
  the FIRST run. Three facts the tables lock: (1) rest carries EXACT
  zeros — including the two NEGATIVE zeros of the multiplied terms,
  which an IEEE == cannot distinguish but from_bits can; (2) progress 1
  is NOT rest at the term level — sin(pi_f32) is -8.742278e-8, the
  f64-narrowing fingerprint a f32-sin rewrite would flip; (3) the
  mid-arc shape (the rotY amount EXACT 1.0 at p=0.25, the Z pull EXACT
  -0.4/-0.2 at p=0.5). The pixel level closes the arc: a binary probe
  (--swing-progress over the full held scene) and the in-test renders
  agree that p=0.25 (6240 px) and p=0.75 (5583 px) really move pixels
  with the bulk right of center, and **p=1.0 renders BIT-IDENTICAL to
  rest** — every vanilla swing term is a sin multiple vanishing at both
  ends, and the ~1e-8 pi_f32 residue flips NO rasterized pixel (sha1-
  identical PNGs; the coverage flips are background-independent: the
  left-half counts 337/0 match the binary and in-test probes to the
  pixel). Tests +3 (workspace 373 → 376 passed / 0 failed / 3 ignored;
  mc-client hand 17 → 20): the term sweep tables, the p=1 epsilon pose
  (terms move the corner bits while the UV unwrap stays and the pixels
  do not), and the pixel-arc sweep with the exact in-test counts pinned
  probe-first. No golden regenerated, no render semantics touched.
  The **biome-tag resolution semantic lands (v0.3.80 — the generated
  tag table's first consumer surface)**: `mc-registry::flatten_biome_tag`
  turns a tag id into the flattened member set the vanilla datapack
  loader builds from `data/minecraft/tags/worldgen/biome/*.json` (the
  raw 68-tag / 390-member table the generator emits verbatim): entries
  walked in FILE ORDER, a `#`-nested reference expanded DEPTH-FIRST at
  its position (the referenced tag's own file order), the built set
  deduplicating on FIRST occurrence (the tag is a set — a biome
  reachable through two paths resolves once), and three load failures
  refused to `None` exactly as vanilla refuses the datapack: unknown
  tag, unknown nested reference, and a REFERENCE CYCLE (the expansion
  stack fails closed instead of looping). The flattened members are
  additionally validated against the biome table (a registry-bound tag
  loads only when every reference resolves). The real table cannot
  cycle (the census join proves every ref resolves), so the dedup and
  cycle rules pin on a SYNTHETIC table through a generic core the
  public wrapper delegates to. GOLDENS: untouched — BY DATA, NOT BY
  LUCK: grep proves mc-client/mc2rs-pixel carry ZERO references to the
  new functions (pure registry-side data work, no render path), and the
  full matrix was re-verified through verify_goldens.sh ON THE REBUILT
  release binaries (the script's own rebuild embeds the new code), 12/12
  pass, every one rms 0.000. Tests +4 (workspace 369 → 373 passed / 0
  failed / 3 ignored; mc-registry 72 → 76): flat tags build to their
  own file order (is_forest verbatim; is_overworld the largest flat
  set, disjoint from is_nether/is_end), the nested reference expands
  depth-first in place (is_ocean's four deep oceans at the HEAD, the
  two-level recursion ruined_portal_ocean == is_ocean's flatten), the
  synthetics pin dedup/cycle/self-ref/unknown-ref, and the end-to-end
  sweep builds all 68 tags with every flattened member registered.
  The **leaf-litter variant→model palette mapping lands (v0.3.79 — the
  banked v0.3.78 item, and the palette's chain of custody is now
  complete for the ground cover)**: the mesher's leaf-litter flat
  fallback (the `TILE_NONE` branch of the 13th-audit path) now reads
  `colors::palette_leaf_litter_color(segment, face)` FIRST — the same
  palette → LUT → defaults precedence the cube path has had since
  v0.3.78 — instead of jumping straight to the LUT. The mapping exists
  because the blockstate is a MULTIPART whose `segment_amount` selects
  the template model(s) (1→m1, 2→m2, 3→m2+m3 via the `2|3` OR-list
  plus `3` — the multipart collect rule, 4→m4) and each template
  samples a DIFFERENT uv rect of the same 16×16 grayscale cutout, so
  the flat color must follow the variant (the cube table pins
  leaf_litter at segment_amount=1 and could not have been reused: its
  segment-1 entry (165,165,165) differs from segment 4's (171,171,171)
  — the full cover the no-op variant 0 renders). `facing` is pinned to
  north BY PROOF: the y-rotation turns the element and its glued uv
  together and the quad rect is the rotation-invariant min/max, so the
  average cannot change with the facing. This round also RESTORES
  `scripts/palette_oracle.py` — the v0.3.77 oracle was referenced by
  this README but silently missing from the tree (the v0.3.74 lesson
  re-applied: grep the references) — a dependency-free hand-written
  PNG reader (filters, palette/gray/truecolor depths, tRNS) that
  re-pins stone (126,126,126) / grass_block_top (147,147,147)
  bit-exactly and provides the new numbers: segment 1 = the NW-quarter
  rect (165,165,165) [27/64 opaque], segment 2 = the west strip
  (171,171,171) [55/128], segment 3 = the half-up compose of the strip
  (171) + the SE quarter (170) → (171,171,171), segment 4 = the full
  cover (171,171,171) [117/256 opaque] — three of four segments
  coincide on THIS texture, the mapping is still required (segment 1
  differs; a future texture is free to split them).
  GOLDENS: NONE regenerated — BY DATA, NOT BY LUCK: the atlas covers
  leaf_litter on every golden scene, so the rewired fallback executes
  on NO golden pixel; the full matrix re-verified through
  verify_goldens.sh, 12/12 pass, every one rms 0.000. Tests +2
  (workspace 367 → 369 passed / 0 failed / 3 ignored): the variant
  table IS the oracle's numbers (up/down share one color — the two
  template faces sample the same rect; the four side slots None; the
  segment 0/5 edges fail closed) and the variant table genuinely
  diverges from the canonical cube-table entry (segment 1 matches it
  bit-for-bit, segment 4 does not).
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
