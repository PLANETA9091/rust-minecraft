# rust-minecraft — worklog

> THE FILE'S OWN RESET (2026-10-22): the sandbox disk was wiped between
> autopush-122 and this entry — this worklog, cargo, `.mc2rs/`,
> `assets-cache/` and the UNPUSHED comparator lane died with it. The
> history below restarts from autopush-123; the pre-wipe per-round
> history lives in the git log (the commit messages carry the full
> per-round record) and, from this entry on, in the durable mirror the
> repo keeps at `logs/worklog.md`.

---
Task ID: autopush-123
Agent: cron autopush (Job 384923, 2026-10-22 ~09:10 +08:00)
Task: the standing autopush round — but the disk was gone, so the ONE
reasonable step was the environment-recovery lane: clone, toolchain,
runtime cache, full-suite re-proof.

Work Log:
- Step 1 (auto-sync) found NO repo at /home/z/my-project/rust-minecraft —
  the fresh /home/z/my-project held only the platform scaffold (Initial
  commit, empty download/, skills/, upload/). Every local artifact from
  the pre-wipe rounds was gone: cargo, worklog.md, the pending
  autopush-123 worklog insurance file, .mc2rs/classpath/client.jar,
  assets-cache/, AND the unpushed comparator-simulation lane (the whole
  autopush-123 work: world/mesher/screens code + atlas pin, check-green,
  never committed — the 08:16 round's infra collapse left it disk-only).
- Cloned from the token URL: HEAD = 12db5f0 (autopush-122, v0.3.846) —
  everything PUSHED survived. The repo's own `.cargo/config.toml` OOM
  baseline (jobs=1, lld --threads=1, debug=0, no incremental, mc-client
  dev-assertions off) and the `[profile.test] debug = 0` came back with
  the clone — no rebuild gymnastics needed.
- rustup installed (minimal profile): rustc/cargo 1.99.1, same band the
  pre-wipe rounds ran.
- Re-fetched the official 26.2 client.jar from Mojang's piston-data
  (sha1 2dc72797acbc1b63fc16a11c4ac393605f453754 — the manifest's own
  pin) into .mc2rs/classpath/.
- cargo check --workspace --all-targets: GREEN in 1m43s.
- FIRST full-suite attempt: the mc-client test binary BUILT AND RAN on
  the fresh box — the OOM wall the old machine wore (the 1.9 GB rustc
  link peak vs the host agent's hold) is GONE with the agent's 1.8 GB
  resident. But 158/2294 failed — the runtime cache the tests read was
  the missing half of the environment.
- The cache rebuild, layer by layer (each layer's failures named the
  next): jar assets extracted under assets-cache/jar/ (10968 files) →
  the FLAT layout the loaders read from the cache root (gui/ +
  entity/ + environment/ + font/ + block/ + item/ + misc/ + particle/
  — the census over `.join("…")` calls and the skin RUNTIME_PATH table
  named every prefix; sandbox REFUSES symlinks, so plain copies) → the
  REAL title-screen panorama pixels (the jar's own are 1x1 placeholders;
  7 store objects sha1-verified into gui/title/background/) →
  sounds.json (a store object, NOT in the jar) + 2147 wanted .ogg files
  (the generator's wanted-filter mirrored: music.menu/block.* + the
  EXTRA/MOB/PLAYER/INTERACTION/AMBIENT constants + the worldgen
  biome_music rows; every file sha1-verified; 9 events' files absent
  from the index — the generator's own walk skips the same) →
  `mc2rs-gen cache` emitted assets-cache/gen/font.bin (43543 B,
  3 glyph textures) + textures.bin (3362797 B).
- Suite re-proof: 2608 passed / 0 failed / 22 ignored (2272 mc-client +
  157 mc-protocol + 99 mc-registry + 80 mc2rs-pixel; +1 vs autopush-121's
  2607 = the stab-lane test the 122 round added). THE FULL SUITE RUNS
  AGAIN.
- verify_goldens.sh: 18/18 PASS, rms 0.000 everywhere (gui_title's
  real-pixel panorama reproduces the golden exactly — the store fetch
  was byte-true).
- Repo additions this round: scripts/recover_runtime_cache.py (the
  one-command recovery the next wipe runs — jar, extract, flat layout,
  panorama, sounds, gen-cache hint; idempotent, re-run on the live
  cache reports 0 fetched) and logs/worklog.md (this file's durable
  mirror — the worklog lived outside the repo once and that killed it).
  Version bump 0.3.846 → 0.3.847.

Stage Summary:
- The environment is WHOLE again and re-proven: full suite 2608/0/22,
  goldens 18/18 rms 0.000, cargo 1.99.1, jar + runtime cache + gen cache
  restored byte-verified. No render code changed — no goldens touched.
- THE LOST LANE (the next round's ONE step candidate): redo the
  comparator simulation the wipe ate. The full spec, compressed from the
  lost round's javap forensics (26.2 jar, scripts/classfile_dump.py):
  (a) variant byte = low2 facing (compass order N=0/E=1/S=2/W=3), bit2
  subtract, bit3 powered, high4 output cache; (b) calculateOutputSignal:
  i==0→0, j>i→0, SUBTRACT→i−j, COMPARE→i with the tie through if_icmple
  (COMPARE keeps i, SUBTRACT refuses); shouldTurnOn: i>j, tie → COMPARE
  true / SUBTRACT false; (c) getControlInputSignal: sides answer DIODE
  family only (wire/lever never serve a side), back reads any source;
  (d) checkTickOnNeighbor: POWERED != shouldTurnOn && !willTick →
  scheduleTick(delay 2); (e) refreshOutputState: setBlock flag-2 +
  updateNeighborsInFront (front cell = pos.relative(FACING.getOpposite()));
  (f) useWithoutItem: cycle(MODE) + click sound 0.3 vol (SUBTRACT pitch
  0.55f = 0x3F0CCCCD, COMPARE 0.5f = 0x3F000000) + INSTANT refresh
  (not the 2-tick wait); (g) getStateForPlacement: FACING =
  getHorizontalDirection().getOpposite() (faces the placer); shape = the
  2px diode plate; (h) atlas pin: kind 768 (N_KINDS 768→769), tiles
  appended at the pool tail 909 (comparator) / 910 (comparator_on) so
  every existing index holds → goldens safe; FACE_TILES [909;6] +
  TINTS/DOOR_MODEL_BY_ROW/ENTITY_HEAD_FACES rows + elements.rs four
  models (comparator, _on, _subtract, _on_subtract) via
  derive_model_inherited; mesher: #top→TILE_COMPARATOR(_ON) swap,
  #slab→smooth_stone, #unlit/#lit→redstone-torch kinds, DOWN cullface
  rotation, per-corner lightmap; (i) signal walk: wire direct power,
  lever/button/plate 15, standing torch d!=UP gate, wall torch FACING
  gate, detector rail, same-family FACING gate → powered?output:0; flood
  seed = wire_source_strength_near (replaces the bool wire_sources_near;
  existing sources stay 15, comparator front feeds 0..15); schedule/
  drain Vec<(u64,i32,i32,i32)> dedup+sort; screens: tick_comparator_beats
  + use arm + placement gate + attach_wire_voice consult (collect
  mismatches → drop(read) → schedule).
- Environment facts for the rounds ahead: cargo at $HOME/.cargo/bin;
  the sandbox REFUSES symlinks under /home; background nohup builds die
  with the shell (foreground only); the 10-minute tool ceiling wants
  single-command test runs (one suite pass per call, log to a file).

---
Task ID: autopush-124
Agent: cron autopush (Job 384923, 2026-10-22 ~10:05 +08:00)
Task: the ONE step the worklog named — REDO the comparator simulation
lane the disk wipe ate (the lost round's spec, re-proven byte-true
against the 26.2 jar before a line was written).

Work Log:
- Environment held (the 123 recovery): clean tree at 86b45f3, cargo
  1.99.1, jar + runtime cache + gen cache all present. No re-recovery
  needed.
- THE FORENSICS REDO (scripts/classfile_dump.py, the 26.2 jar):
  ComparatorBlock + DiodeBlock + SignalGetter dumped whole. The jar
  sharpened the lost spec's compressed corners:
  (a) DiodeBlock.sideInputDiodesOnly() returns FALSE — the sides read
  SignalGetter.getControlInputSignal(pos, dir, false), NOT the
  diode-only walk: REDSTONE_BLOCK→15, REDSTONE_WIRE→the raw POWER
  property, isSignalSource→getDirectSignal (the STRONG walk — the ON
  lever answers only toward its support, plates/torches/rails the 0
  the no-override keeps); the lost round's "sides diode-only" gloss
  was wrong, the jar's own bytes settle it;
  (b) checkTickOnNeighbor schedules when (calculateOutputSignal !=
  the BE's stored output) OR (POWERED != shouldTurnOn) — the
  OUTPUT-CACHE arm the lost spec never named (the stale-output fix
  the 2-tick beat owns); willTickThisTick dedups, priority NORMAL;
  (c) DiodeBlock.getSignal answers dir == FACING only (ownSignal) —
  the front cell = FACING.getOpposite() reads it, the back read of a
  sibling comparator gates on ITS FACING == ours; getDirectSignal =
  getSignal (the diode's front is STRONG power);
  (d) DiodeBlock.getInputSignal: the back cell's weak getSignal +
  the REDSTONE_WIRE override (the raw POWER property, the
  connected-side verdict skipped);
  (e) shouldTurnOn: i==0→false, i>j→true, tie→COMPARE only;
  calculateOutputSignal: i==0→0, j>i→0, COMPARE→i, SUBTRACT→i−j;
  (f) useWithoutItem: mayBuild gate, cycle(MODE), click sound 0.3
  (pitch 0.55 subtract / 0.5 compare), flag-2 setBlock, instant
  refreshOutputState when still-comparator;
  (g) getStateForPlacement: FACING = getHorizontalDirection()
  .getOpposite() (the front faces where the placer LOOKS);
  (h) refreshOutputState: BE.setOutputSignal + the POWERED sync +
  updateNeighborsInFront.
- THE PIN (scripts/autopush124_comparator_pin.py, idempotent,
  section-guarded): atlas.rs — TILE_909/910 decoded from the jar's
  comparator.png/comparator_on.png (the pool tail, every existing
  index holds), TILES 911, N_KINDS 769, KIND_NAMES +
  REGISTRY_NAMES the "comparator" tails, TILE_NAMES the pair,
  FACE_TILES [909;6], TINTS/DOOR_MODEL_BY_ROW/ENTITY_HEAD_FACES the
  769-row law, TILE_COMPARATOR/TILE_COMPARATOR_ON named consts.
  elements.rs + models.rs ALREADY carried the four typed rows (the
  generator had them — only the atlas tiles were missing; the pin's
  elements section the honest no-op keeps).
- world.rs: COMPARATOR 768 + the variant codec (low2 facing compass
  N0/E1/S2/W3, 0x04 subtract, 0x08 powered, high4 output cache), the
  back/side/alternate walks (the jar's own lanes), output + should-
  turn-on verdicts, refresh (pure read), comparator_sources_near +
  comparator_front_output_near, comparator_can_survive, the placement
  variant (the look's opposite — the +2/+2 cancellation the look
  index answers directly), wire_source_strength_near (the bool walk's
  own upgrade: the flood seeds the POWER, the comparator front the
  0..15 the max fold carries), the flood site swapped, the
  shouldConnectTo diode seat, the collision Empty row, the schedule/
  drain pair (the pos-dedup the willTickThisTick law keeps), the
  comparator_ticks queue + the test initializers' 9th field.
- mesher.rs: comparator_model_quads (the four OnceLock bakes) + the
  element lane (the per-quad tile verdict: #top→TILE_COMPARATOR(_ON),
  #slab→smooth_stone, #unlit/#lit→the torch kinds' tiles; the
  blockstate y-table south 0 / west 1 / north 2 / east 3 turns; the
  DOWN cullface the ground law keeps; the FACE_SHADE term the quad
  direction reads; the lightmap the per-corner fold answers).
- screens.rs: tick_comparator_beats (the drain → the refresh → the
  attach broadcast the cell+six scan covers), the use arm (the mode
  cycle + the instant refresh + the click voice + the front seed),
  the placement survival gate + the facing stamp, the attach_wire_
  voice consult (the cell+six scope, the refresh-mismatch → schedule
  +2, the collect→drop(read)→schedule discipline).
- THE PIN BUMPS THE TESTS DEMAND: 34 kind-pins the palette law grew
  (N_KINDS 768→769 ×8 + the one-line form ×27) + the 21 NAME_OF.len()
  pins — the same bumps every kind-addition round has paid.
- TESTS +4: the truth table (the wire back's raw read, the SUBTRACT
  gap 15-4=11, the COMPARE tie keeps 4, the side dominance, the
  shouldTurnOn tie laws), the schedule/refresh/front-seed walk (the
  dedup, the drain order, the strength seed the front alone answers),
  the placement/survival walk, the mesher bakes (the four states, the
  wire-geometry count pair, the resolved texture names).
- THE GATES: cargo check --workspace --all-targets GREEN; cargo test
  --workspace 2612 passed / 0 failed / 22 ignored (+4); verify_goldens
  .sh 18/18 PASS rms 0.000 (the pool tail the goldens never touch —
  no regeneration); cargo fmt --check 27292 = 27292 EXACTLY (the
  rustfmt install the minimal profile lacked came back, the +54 the
  first pass wore the hand-fixed three spots — the lane's context
  splits, the pts one-line, the signature split, the bamboo
  alignment).

Stage Summary:
- THE COMPARATOR PLAYS LIKE VANILLA: place it (the front follows the
  look), feed the back (the wire's raw power, the sources' weak walk),
  the sides read the strong law (the wire's raw POWER, the lever's
  support seat, the sibling's front), the click flips the mode (the
  subtract gap the COMPARE tie), the 2-tick beat refreshes the output
  cache + the powered verdict, the front cell carries the 0..15 into
  the wire flood and the consumers' walk, the diode plate renders the
  jar's own four models with the per-quad tiles the blockstate rows
  name. The suite 2612/0/22, the goldens 18/18 rms 0.000, the fmt
  debt the baseline keeps.
- NEXT CANDIDATES: the container-analog lane (the comparator's
  hasAnalogOutputSignal back read — the chest-fill signal the BE
  tables would need), the REPEATER kind (the DiodeBlock sibling the
  wire_connectable_dir comment names — the client's vocabulary never
  banks it), the redstone BLOCK kind (the side walk's REDSTONE_BLOCK
  arm waits), the item-frame analog read (the getInputSignal's
  behind-conductor lane).

---
Task ID: autopush-125
Agent: cron autopush (Job 384923, 2026-10-22 ~11:35 +08:00)
Task: the ONE step the worklog's candidates named — the REDSTONE
BLOCK kind, the constant-15 source the comparator's side arm and the
wire flood have been waiting for (the smallest kind-addition the
vanilla redstone family carries).

Work Log:
- Environment held (clean tree at 34edf40, cargo 1.99.1, caches in
  place).
- THE PIN'S OWN SHRINKAGE (the name-keyed tables the recon credits):
  block_dig/block_resistances/block_sounds/items are REGISTRY-NAME
  keyed across the whole 1196-block vanilla set — the "redstone_block"
  rows they already carry — so the pin rides the ATLAS alone
  (scripts/autopush125_redstone_block_pin.py, idempotent): TILE_911
  decoded from the jar's redstone_block.png (the pool tail, every
  existing index holds), TILES 912, N_KINDS 770, the KIND_NAMES/
  REGISTRY_NAMES/TILE_NAMES tails, FACE_TILES [911;6] (the full cube
  the DEFAULT walk renders — no named const, no mesher lane),
  TINTS/DOOR_MODEL/ENTITY_HEAD the 770-row law, world.rs NAME_OF the
  770th entry.
- world.rs the source's own arms (javap RedStoneBlock the 26.2 jar —
  getSignal AND getDirectSignal the constant 15 the any-dir read
  answers): the wire flood's six seeds (wire_source_bools_near), the
  consumers' redstone_signal_at (rides the seed walk), the strong
  lane's neighbor arm (strong_signal_conductors_near — the source the
  conductor's own neighbor carries, the NO attachment decode the
  lever pair keeps — the redstone-block-into-block chain the lamps
  wear), the wire's shouldConnectTo seat (the wire points at the
  block), the comparator's back arm (the weak 15) AND the side arm
  (the strong 15 — the getControlInputSignal's REDSTONE_BLOCK arm the
  autopush-124 comment reserved now lives), is_redstone_block.
- Collision the DEFAULT full cube keeps (no Empty row — the block
  the body walks AROUND).
- THE PIN BUMPS THE TESTS DEMAND AGAIN: 56 kind-pins the palette law
  grew (N_KINDS 769→770 ×35 + NAME_OF.len() ×21).
- TESTS +2: the seed/strong/consumer walk (the six seats the 15
  answers, the far cell the silence keeps, the conductor-adjacent
  strong verdict, the full-cube collision, the placeable_block
  lookup the NAME_OF row answers), the comparator arms (the back the
  15 speaks, the side the dominance wears, the COMPARE tie keeps,
  the SUBTRACT tie the 0 wears, the bare side the 15−0 gap speaks).
- THE GATES: cargo check --workspace --all-targets GREEN; cargo test
  --workspace 2614 passed / 0 failed / 22 ignored (+2); verify_goldens
  .sh 18/18 PASS rms 0.000 (the pool tail no golden scene touches);
  cargo fmt --check 27292 = 27292 EXACTLY (the lane's edits the clean
  form kept — zero fmt debt this round).

Stage Summary:
- THE REDSTONE BLOCK PLAYS LIKE VANILLA: place it, the wire around it
  lights to 15 (the flood seeds), the lamps/doors/rails beside answer,
  the comparator's back AND sides read the constant 15 (the compare
  tie the subtract gap the dominance arms the same walk serves), the
  block-into-block chain the strong lane carries, the dig lane the
  name-keyed tables the destroy-speed row resolves, the placement
  the item pairing answers. The suite 2614/0/22, the goldens 18/18
  rms 0.000, the fmt debt the baseline keeps.
- NEXT CANDIDATES: the REPEATER kind (the DiodeBlock sibling — the
  delay dial the right-click cycle, the one-way gate, the lock state,
  the boost-to-15 arm — the biggest redstone lane left), the
  container-analog lane (the comparator's hasAnalogOutputSignal back
  read — the chest-fill signal the BE tables would need), the
  item-frame analog read (the getInputSignal's behind-conductor
  lane), the observer kind (the shouldConnectTo comment's other
  missing sibling).


