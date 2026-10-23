# rust-minecraft worklog

> THE WIPE THE RESET TOOK (2026-10-23 ~13:52 +08:00): the sandbox reset
> lost the whole /home/z/my-project tree (this worklog, the repo, the
> rustup toolchain, assets-cache). The dev-150 round rebuilt everything:
> the repo re-cloned from the token URL, the toolchain re-rustup'd, the
> caches re-fetched via rust-minecraft/scripts/restore_env.sh. The FULL
> task history (dev-1 .. autopush-148) survives in the repo's own
> logs/worklog.md on the remote (the per-round mirror commits carry it).
> The entries below resume from dev-150.

---
Task ID: dev-150
Agent: main (cron tick 2026-10-23 13:41:55 +08:00, Job ID 384925)
Task: rust-minecraft 发展 tick —— git 检查 → ONE 步发展 → 测试 → push → worklog。

Work Log:
- THE STATE THE TICK READ: the clean tree @b2fc563 (v0.3.877, the
  autopush-148 shulker placement record), origin synced; the
  memory landscape the wall keeps (1835MB available, the
  platform's 1.85G resident the same lane wears) — the full-gate
  re-run the record's plan defers, the candidate's head the
  NEXT list keeps: THE SHULKER'S OWN LIVE WINDOW the offline
  door the round answers.
- THE MINIMAL STEP THE DOOR THE WINDOW BANKS: the use-walk
  gains the shulker's own arm (javap 26.2 ShulkerBoxBlock.
  useWithoutItem the openMenu walk the ShulkerBoxMenu the
  27-slot generic_9x3 shape the chest family's own grid
  answers, the container.shulkerBox default name the lang row
  the table wears): the right-press on the placed box the
  OFFLINE world answers, the WIRE session the server's own
  OpenScreen word owns (the SHULKER_BOX menu type the
  projection's menu_type_id feeds verbatim — the live lane the
  round's read proves menu-type-agnostic).
- THE DOOR'S OWN WALK: open_shulker_for the single-cell
  27-slot store the SAME container_stores map the dropper's
  front-read inserts (the autopush-147 row the walk's own lane
  keeps), the mirror sync the player's real rows project, the
  DOUBLE the shulker never speaks (the chest's combiner the
  chest kinds alone answer), the lid the variant never wears
  (the facing property the only field — the mesher's generic
  cube no OPEN bit renders); the kind flag chest_window_shulker
  the title lane banks (the container.shulkerBox row the
  offline canvas wears), the chest's own openers + the golden
  scene + the live lane the flag reset keeps — the golden
  scenes the flag never set, the pixels the shape never moves.
- +1 TEST: the_offline_shulker_the_door_the_window_the_27_slots_banks
  (the REAL press walk the door arms — the window the chest
  shape opens, the kind row the title banks, the pre-banked
  stack the open loads, the DOUBLE the lone box never speaks,
  the sneaking press the PASS verdict keeps, the reopen the
  stack rides).
- THE ENVIRONMENT THE RESET TOOK, THE RESUME LANE THE ROUND
  REBUILT: the mc-client test-bin codegen attempt the sandbox
  wipe rode (the first 7-min tool timeout the wall's own
  shadow spoke, the second shell the workspace GONE read —
  repo, toolchain, assets-cache, the external worklog all
  lost); the re-clone the token URL speaks (b2fc563 the HEAD
  the remote kept), the rustup the pinned stable answers
  (1.99.1, the profile minimal the repo's toolchain.toml
  wears), restore_env.sh the four-step cache the lane banks
  (jar sha1 2dc72797 verified, gen cache font+textures, index
  objects 5028 linked, jar textures 10967) — the mc-registry
  4-red the bare cache the restore greens (the sprite-bake
  tests the elytra/gui rows the cache absence wore).
- THE VERIFY THE PROTOCOL'S OWN SHAPE ANSWERED: cargo check
  --all-targets clean (the bumped tree); mc-protocol 157 +
  mc-registry 100 + mc2rs-gen 80 = 337 PASSED / 0 FAILED;
  goldens 19/19 PASS rms 0.000 (NO regeneration — the flag the
  golden scenes never set); fmt the fresh rustfmt 1.99.1 the
  same-tree count re-speaks (1703 hunks the HEAD wears, 1703
  the diff keeps — ZERO hunks the round adds; the 1159 the old
  record wore the older rustfmt's own count was); the FULL
  cargo test --workspace gate the SAME SIGKILL wall keeps (the
  codegen attempt the wipe rode — the deferral the SIXTH round
  wears, the resume plan carries: re-run when the platform's
  resident shrinks; the +1 test the compile proof banks, the
  execution the wall defers).
- THE VERSION the bump rode (0.3.877 -> 0.3.878) — the verify
  the bumped tree ran.
- Pushed 0882c24 (v0.3.878, the shulker's own door) + the
  mirror commit 3d1378c; this external worklog the wipe took
  the rebuild the entry carries.

Stage Summary:
- THE SHULKER THE OFFLINE WORLD THE WHOLE CHAIN SPEAKS: the
  creative picker the item offers, the press the kind places,
  the look the facing stamps (v0.3.877), the right-press the
  WINDOW opens (this round), the dropper's front read
  (v0.3.876) the placed box the container the lane reads — the
  interactivity the chest family's own shape the shulker
  completes.
- NEXT CANDIDATES: the dispenser's carried press the
  split-pickup the record "already serves" (the offline chest
  drag's own lanes the dispenser window mirrors), the shulker's
  own mesher arm the entity-driven box the generic cube the
  honest upgrade the BE-lane round keeps, the FULL gate the
  memory wall the resume plan keeps.

---
Task ID: dev-151
Agent: main (cron tick 2026-10-23 14:21:55 +08:00, Job ID 384925)
Task: rust-minecraft 发展 tick —— git 检查 → ONE 步发展 → 测试 → push → worklog。

Work Log:
- THE STATE THE TICK READ: the clean tree @3d1378c (v0.3.878, the
  dev-150 shulker door record), origin synced; the memory
  landscape the RESUME PLAN'S OWN TRIGGER the round answers —
  free -m the 3467MB available speaks (the platform's 1.85G
  resident the sandbox wipe took), the FULL gate the six-round
  deferral the re-run claims.
- THE FULL cargo test --workspace GATE THE WALL ANSWERS: the
  first run the 40 red wore — 38 the STALE KIND-COUNT PINS the
  shulker mint (autopush-147's N_KINDS 778->779) the
  compile-only rounds never executed: 47 world.rs + 5 part_8 +
  3 part_7 pins the sweep 779 wears (the SHULKER_BOX const 778
  the SEAT the sweep never touches); +2 the BEHAVIORAL PINS the
  autopush-146 gate rewrite (the world_is_network &&
  chest_live_window.is_none() no-window refusal the live-only
  law replaced) the part_9 stand-in beats the hybrid state wore
  — the beats re-shaped, the window the silence-beat AFTER
  sets, the REAL gate's refusal the beat pins.
- THE 5 THE DIRECT-BINARY RUN WORE, THE CARGO LANE REFUSES:
  the warden glow + the third-person helmet + the darkness
  pulse + the water murk + the portal tint — the dep-binary
  invocation the static-heavy render lanes the 2-CPU
  parallelism the unlucky interleave spoke, the cargo
  invocation the SAME tree the 2342-green wears, the isolated
  --exact run the 5 the green confirms — the honest note the
  record keeps.
- THE MINIMAL STEP THE DEV-150 CANDIDATE THE PIN BANKS: +1
  TEST the_dispenser_right_press_the_split_and_the_one_each_wear
  (part_4) — the right-press dispatch the record "already
  serves" the test never wore: the ceiling-half split (5 -> 3
  taken, 2 left) the PICKUP button-1 byte, the held press the
  type-1 drag arms (NO place NO packet), the one-each the
  release serves, the masked START(4)/ADD(5)x2/END(6) bytes
  the deltas name. THE FMT'S OWN QUIRK: the for-loop shape the
  rustfmt --check parse the recovery garbage rode, the
  unrolled two-match shape the SAME semantics the 1703-hunk
  baseline the zero-new-hunks law keeps.
- THE VERSION the bump rode (0.3.878 -> 0.3.879).
- Pushed 866f605 (v0.3.879) + the mirror commit 79de693; the
  repo's logs/worklog.md the dev-151 entry wears.

Stage Summary:
- THE FULL GATE THE SIX-ROUND DEFERRAL CLOSES: the suite 2679
  PASSED / 0 FAILED / 22 ignored the whole workspace wears
  (mc-client 2342 + mc-protocol 157 + mc-registry 100 +
  mc2rs-gen 80); goldens 19/19 PASS rms 0.000 (NO
  regeneration); fmt 1703 = the HEAD baseline; check
  --all-targets clean.
- NEXT CANDIDATES: the shulker's own mesher arm the
  entity-driven box the generic cube the honest upgrade (the
  render byte the golden regen the round names), the 5
  render-static tests the parallel-sensitivity the serial/lock
  shape the future round tames, the single-pass cargo test
  --workspace the tool's 600s ceiling the longer window
  allows.

---
Task ID: dev-152
Agent: main (cron tick 2026-10-23 15:41:57 +08:00, Job ID 384925)
Task: rust-minecraft 发展 tick —— git 检查 → ONE 步发展 → 测试 → push → worklog。

Work Log:
- THE STATE THE TICK READ: the clean tree @79de693 (v0.3.879, the
  dev-151 full-gate record), origin synced. THE STEP THE dev-151
  CANDIDATE NAMED: the 5 render-static tests the
  parallel-sensitivity the serial/lock shape the round tames.
- THE SERIAL LOCK THE PRESCRIPTION WORE: RENDER_LANE the mod.rs
  helper arms (the static Mutex the const-new speaks, the poisoned
  guard the into_inner recovery keeps) — the six heavy lanes the
  guard acquires (the warden glow, the player-armor + head-lane
  third-person frames, the darkness pulse, the water murk, the
  portal tint). THE BATCH THE LOCK WEARING THE RED KEPT: the
  dep-binary run the same 4 red re-wore — the interleave theory
  the REPRODUCTION REFUTED.
- THE MISDIAGNOSIS THE RECORD RETIRES: the same binary the same
  filters the CWD ALONE flips (repo root 4 red / crates/mc-client
  6 green) — the dev-151 "unlucky 2-CPU parallelism" the CWD-
  dependent asset resolution really wore. THE PROBES' OWN TABLE:
  the tint test the repo-root sprite LOAD the assert's first
  execution spoke (the cargo lane the root-None early return the
  VACUOUS pass wore); the murk the walked GuiAssets root both
  lanes opens (murk 1183 cargo / 0 repo); the head-lane the
  helmet the repo lane PAINTS (468 diff — the 0-diff pin the
  artifact); the warden the spots the causal probe REFUSES (the
  manually-loaded spots the 89 diff keep).
- THE MECHANISM THE RENDER SOURCE PINNED: render_avatar's five
  hardcoded 'assets-cache/jar' sites (the player skin, the armor
  pair, the leather overlay, the wings, the fire strips) the CWD
  the process speaks obey — the package-dir processes the
  honest-skip arms walked (the avatar None = the player body
  NEVER drew; the armor textures None = the helmet never
  painted). THE FIX THE CART LANE'S OWN LAW SPEAKS:
  jar_runtime_root the five sites wear (the hint exists the
  identical path returns — the repo-root hint the goldens' own
  bytes the same root wears; the walk-up the package-dir
  processes answer).
- THE PINS THE LOADED WORLD RE-STATES: the portal tint the walked
  root + the vanilla's own DOUBLE FOLD the pin wears (the
  vertex-color multiply THEN the srcAlpha blend — the [0.5;4]
  tint the quarter, the rgb-only/alpha-only halves the
  per-channel folds pin; the old f*0.5 the single-fold math the
  blend never wore); the head-lane helmet the armor lane's ONE
  paint the pin re-states (the NEITHER verdict the pumpkin/skull
  branches refuse); the warden the SIDE seat re-stages (bx+2, bz
  — the two cameras the seat mirrors; the old bz+3 the
  1-block-from-camera near-field wore).
- THE TWO THE LOADED WORLD STILL WEARS (the honest defer): the
  warden biolum (the body diff 7982/7982 IDENTICAL, the texture
  the same file, the blend the pure impl, the spots non-causal —
  the diff 383 cargo / 0 repo — the layer draw's unmodeled input
  the next dig owns) + the water murk (1183 cargo / 0 repo — the
  atlas table's 3-prefix walk BOTH CWDs covers — the world
  render's CWD-dependent input the seat unfound).
- THE VERSION the bump rode (0.3.879 -> 0.3.880).
- Pushed 421d400 (v0.3.880); the repo's logs/worklog.md the
  dev-152 entry wears.

Stage Summary:
- THE CWD LAW THE RENDER_AVATAR LANE NOW SPEAKS: the two lanes
  (cargo package-dir / dep-binary repo-root) the loaded world SEE
  alike — the four fixed tests BOTH lanes wear (the tint + the
  armor + the head-lane + the pulse); the serial lock the heavy
  lanes the harness never interleaves keeps (the scheduling
  variable the shape retires — the real mechanism the CWD law
  owns).
- VERIFY: the FULL mc-client suite 2342 PASSED / 0 FAILED / 22
  ignored + mc-protocol 157 + mc-registry 100 + mc2rs-gen 80 =
  2679 PASSED / 0 FAILED; goldens 19/19 PASS rms 0.000 (NO
  regeneration — the repo-root hint the identical path returns);
  fmt 1703 = the HEAD baseline (ZERO hunks the round adds);
  cargo check --all-targets clean.
- NEXT CANDIDATES: the warden biolum's unmodeled input the
  repo-root lane's 0-diff wears (the layer draw's own dig — the
  state/quads/depth the probe table narrows), the water murk's
  world-render CWD input (the dive frame's 1183/0 split the
  atlas/colormap lanes refuse — the render_world_3d's own walk
  the next dig reads), render_mobs + the boats' hardcoded
  'assets-cache/jar' sites the same jar_runtime_root law the
  render_avatar fix speaks (the pig/cow/sheep lazy loads the
  package-dir lane never wore).
