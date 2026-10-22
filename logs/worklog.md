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
