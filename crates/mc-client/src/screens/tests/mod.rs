// THE SCREENS' OWN TEST BANK (the split the codegen peak asked for):
// the one 81k-line `mod tests` the dev-146 round's rustc codegen peak
// crossed the sandbox's ceiling with is now round-sized part modules —
// the same bytes the same order wear, the helper bank this mod.rs keeps,
// the glob chain (`use super::*;`) the parts' own one-liner speaks. The
// move is byte-verbatim: no test body, no attribute, no doc line the
// reshape touched — the test count and the harness output the same.

mod part_1;
mod part_10;
mod part_11;
mod part_2;
mod part_3;
mod part_4;
mod part_5;
mod part_6;
mod part_7;
mod part_8;
mod part_9;

use super::*;

/// THE SOUND TABLE'S OWN LOADED GATE (the generated/ -> runtime-load
/// migration's third lane — the table the runtime cache keeps, the
/// loader the only parser): the voice asserts speak where the table
/// exists, the honest silence the cache-absent boot keeps (the CI's
/// pre-generate test job the same arm walks — the lang lane's
/// expect-closure form the boolean here carries).
fn sounds_loaded() -> bool {
    !crate::sound_table::events().is_empty()
}

/// THE TESTS' OWN ASSETS ROOT (the CWD-independent pin the darkness
/// walk's ancestor pattern wears — the bare `assets-cache/jar` the
/// package-root cargo seat never resolves, the stub lane the absent
/// sprites wear): the MC2RS_ASSETS env the first word keeps (the
/// value the GuiAssets root itself — the jar dir the open seats
/// wear), the ancestor walk the repo's own root finds from every
/// seat inside (the `assets-cache/jar` exists the each-candidate's
/// verdict answers), the pop's exhaustion the None floor keeps.
fn assets_root_opt() -> Option<std::path::PathBuf> {
    if let Some(p) = std::env::var_os("MC2RS_ASSETS") {
        return Some(std::path::PathBuf::from(p));
    }
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let cand = dir.join("assets-cache").join("jar");
        if cand.exists() {
            return Some(cand);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// THE ROOT'S OWN DEGRADE SHAPE (the bare relative fallback the
/// packaged run keeps — the helper's Option the honest-skip seats
/// wear, the unwrap the stub lane's own shape answers).
fn assets_root() -> std::path::PathBuf {
    assets_root_opt().unwrap_or_else(|| std::path::PathBuf::from("assets-cache/jar"))
}

/// A hand-built Brigadier graph mirroring the real servers' own:
/// root(0) → time(1) → set(3)/query(4); tp(2) redirect→teleport(5)
/// → the targets argument(7); tick(6). The structure the local
/// walk's verdicts ride.
fn test_tree() -> mc_protocol::play::CommandTree {
    use mc_protocol::play::{CommandNode, CommandNodeKind};
    let node = |kind: CommandNodeKind, name: &str, children: Vec<u32>, redirect: Option<u32>| {
        CommandNode {
            kind,
            executable: false,
            restricted: false,
            children,
            redirect,
            name: name.to_string(),
            parser: None,
            suggestion_id: None,
        }
    };
    let mut nodes = vec![
        node(CommandNodeKind::Root, "", vec![1, 2, 6], None),
        node(CommandNodeKind::Literal, "time", vec![3, 4], None),
        node(CommandNodeKind::Literal, "tp", vec![], Some(5)),
        node(CommandNodeKind::Literal, "set", vec![], None),
        node(CommandNodeKind::Literal, "query", vec![], None),
        node(CommandNodeKind::Literal, "teleport", vec![7], None),
        node(CommandNodeKind::Literal, "tick", vec![], None),
    ];
    nodes.push(CommandNode {
        kind: CommandNodeKind::Argument,
        executable: false,
        restricted: false,
        children: vec![],
        redirect: None,
        name: "targets".to_string(),
        parser: Some((6, "minecraft:entity".to_string())),
        suggestion_id: Some("minecraft:ask_server".to_string()),
    });
    mc_protocol::play::CommandTree {
        root_index: 0,
        nodes,
    }
}

/// THE PROVIDER TREE'S OWN FIXTURE (the argument slots' split the
/// parser names speak: the game-profile slot the no-id tab belief
/// answers, the entity slot the same, the score-holder slot no
/// local provider names, the entity slot the 0x10 id the custom
/// provider's own ask lane keeps).
fn test_provider_tree() -> mc_protocol::play::CommandTree {
    use mc_protocol::play::{CommandNode, CommandNodeKind};
    let node = |kind: CommandNodeKind, name: &str, children: Vec<u32>, redirect: Option<u32>| {
        CommandNode {
            kind,
            executable: false,
            restricted: false,
            children,
            redirect,
            name: name.to_string(),
            parser: None,
            suggestion_id: None,
        }
    };
    let mut nodes = vec![
        // 0: the root — kick, kill, score, give ride (the pushed
        // arguments below land at 4, 5, 6)
        node(CommandNodeKind::Root, "", vec![1, 2, 3, 7], None),
        // 1: /kick <targets: game_profile> (no custom provider)
        node(CommandNodeKind::Literal, "kick", vec![4], None),
        // 2: /kill <targets: entity> (no custom provider)
        node(CommandNodeKind::Literal, "kill", vec![5], None),
        // 3: /score <holders: score_holder> (no local provider)
        node(CommandNodeKind::Literal, "score", vec![6], None),
    ];
    // the argument slots the parsers name — the game-profile slot
    // the tab belief answers, the entity slot the same, the
    // score-holder slot no local provider speaks
    nodes.push(CommandNode {
        kind: CommandNodeKind::Argument,
        executable: false,
        restricted: false,
        children: vec![],
        redirect: None,
        name: "targets".to_string(),
        parser: Some((0, "minecraft:game_profile".to_string())),
        suggestion_id: None,
    });
    nodes.push(CommandNode {
        kind: CommandNodeKind::Argument,
        executable: false,
        restricted: false,
        children: vec![],
        redirect: None,
        name: "targets".to_string(),
        parser: Some((6, "minecraft:entity".to_string())),
        suggestion_id: None,
    });
    nodes.push(CommandNode {
        kind: CommandNodeKind::Argument,
        executable: false,
        restricted: false,
        children: vec![],
        redirect: None,
        name: "holders".to_string(),
        parser: Some((22, "minecraft:score_holder".to_string())),
        suggestion_id: None,
    });
    // 7, 8: /give <player: entity + the ask_server id> (the custom
    // provider the ask lane keeps)
    nodes.push(node(CommandNodeKind::Literal, "give", vec![8], None));
    nodes.push(CommandNode {
        kind: CommandNodeKind::Argument,
        executable: false,
        restricted: false,
        children: vec![],
        redirect: None,
        name: "player".to_string(),
        parser: Some((6, "minecraft:entity".to_string())),
        suggestion_id: Some("minecraft:ask_server".to_string()),
    });
    mc_protocol::play::CommandTree {
        root_index: 0,
        nodes,
    }
}

/// An all-air network window of `side` chunks per side (the state
/// machine must not care about content) with a distinguishable seed.
fn air_net_world(side: usize, seed: u64) -> crate::world::World {
    crate::world::World {
        darkness_scale: 0.0,
        chunks_per_side: side,
        seed,
        difficulty: crate::difficulty::Difficulty::Normal,
        difficulty_locked: false,
        inhabited: None,
        // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
        // the empty store the offline walk alone fills
        fire_ticks: Vec::new(),
        fire_age: std::collections::HashMap::new(),
        portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
        container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
        frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
        has_skylight: true,                            // the portal's own axis store: empty
        campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
        torch_ticks: Vec::new(),                       // the torch's own queue: empty
        detector_ticks: Vec::new(),                    // the detector's own queue: empty
        comparator_ticks: Vec::new(),
        dispenser_ticks: Vec::new(),
        repeater_ticks: Vec::new(),
        observer_ticks: Vec::new(),
        piston_ticks: Vec::new(),
        moving_states: Vec::new(),
        moving_ticks: Vec::new(),
        torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
        bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
        chunks: vec![
            vec![
                crate::world::AIR;
                crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z
            ];
            side * side
        ],
        light: None,
        day_tick: None,           // test scenes stay day
        biomes: None,             // test scenes carry no biome layer
        variants: None,           // test scenes carry no state variants
        ambient_light: None,      // no dimension opinion — the black bytecode default
        block_light_tint: None,   // no dimension opinion — the bytecode default tint
        sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
        sky_light_color: None,    // no dimension opinion — the -1 default's white identity
        night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
        night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        rain_level: 0.0,          // the dry test world — no weather word rides
        thunder_level: 0.0,       // the dry test world — no storm speaks
        // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
        // the gate the gen-time default keeps off, the journal the
        // session's rows never names
        journaling: false,
        edit_journal: Vec::new(),
        variant_journal: Vec::new(),
        saved_edits_mark: 0,
        saved_variants_mark: 0,
        chest_lids: std::collections::HashMap::new(),
        lid_gen: 0,
        lid_partial: 1.0,
    }
}

/// The chest rects the render lays (the grid's 0..27, the mirror's
/// 27..63 — the +1 inset the store's own lane speaks): the test
/// stage's own copy of the render walk's shape.
fn chest_rects_for(panel: (i32, i32)) -> Vec<(i32, i32, i16)> {
    (0..27)
        .map(|i| {
            let (sx, sy) = chest_container_slot(panel, i);
            (sx + 1, sy + 1, i as i16)
        })
        .chain((0..36).map(|i| {
            let (sx, sy) = chest_mirror_slot(panel, i);
            (sx + 1, sy + 1, (27 + i) as i16)
        }))
        .collect()
}

/// The 3x3 canvas's render-shaped hit boxes (the render's own
/// layout the test pre-lays — the render-then-hit pattern the
/// chest helper's shape keeps): the nine the j + i*3 order rides,
/// the standard rows the 9..45 ids speak.
fn dispenser_rects_for(panel: (i32, i32)) -> Vec<(i32, i32, i16)> {
    (0..9)
        .map(|i| {
            let (sx, sy) = dispenser_grid_slot(panel, i);
            (sx + 1, sy + 1, i as i16)
        })
        .chain((0..36).map(|i| {
            let (sx, sy) = chest_mirror_slot(panel, i);
            (sx + 1, sy + 1, (9 + i) as i16)
        }))
        .collect()
}

fn wire_zombie(id: i32, x: f64, y: f64, z: f64) -> crate::world_view::WireMob {
    crate::world_view::WireMob {
        id,
        x,
        y,
        z,
        yaw: 90.0,
        pitch: 5.0,
        head_yaw: 60.0,
        health: None,
        hurt_time: 0,
        on_fire: false,
        crouching: false,
        climbing: false,
        wool_byte: 0,
        baby: false,
        chicken_variant: None,
        pig_variant: None,
        cow_variant: None,
        chicken_sound_variant: None,
        pig_sound_variant: None,
        cow_sound_variant: None,
        held_weapon: None,
        held_offhand: None,
        armor: [None; 4],
        custom_name: None,
        name_visible: false,
        score_text: None,
        type_name: "minecraft:zombie".to_string(),
    }
}

fn wire_mob_named(id: i32, x: f64, y: f64, z: f64, type_name: &str) -> crate::world_view::WireMob {
    crate::world_view::WireMob {
        id,
        x,
        y,
        z,
        yaw: 90.0,
        pitch: 0.0,
        head_yaw: 90.0,
        health: None,
        hurt_time: 0,
        on_fire: false,
        crouching: false,
        climbing: false,
        wool_byte: 0,
        baby: false,
        chicken_variant: None,
        pig_variant: None,
        cow_variant: None,
        chicken_sound_variant: None,
        pig_sound_variant: None,
        cow_sound_variant: None,
        held_weapon: None,
        held_offhand: None,
        armor: [None; 4],
        custom_name: None,
        name_visible: false,
        score_text: None,
        type_name: type_name.to_string(),
    }
}

/// THE COLUMN'S OWN POOL (the test fixture the whirlpool's walk
/// rides — the water cells the mouth + the body keep, the variant
/// byte the drag bit carries, the variants layer the grid stamps).
fn stamp_offline_column(app: &mut App, c: i32, pool_y: i32, variant: u8) {
    let w = app.offline_world.as_mut().unwrap();
    w.set(c, pool_y, c, crate::world::WATER);
    w.set(c, pool_y - 1, c, crate::world::WATER);
    w.variants = Some(vec![vec![0u8; 16 * 48 * 16]; WORLD_CHUNKS * WORLD_CHUNKS]);
    let vi = ((c % 16) + (c % 16) * 16) as usize + (pool_y as usize) * (16 * 16);
    w.variants.as_mut().unwrap()[((c / 16) + (c / 16) * WORLD_CHUNKS as i32) as usize][vi] =
        variant;
}

/// THE WIRE HULL'S OWN COLUMN FIXTURE (the net world's stamped
/// column the client arm's scan rides — the WATER cells the
/// variants layer carries, the mouth the top row's AIR above
/// speaks, the hull the snapshot's own pose the scan reads).
fn wire_column_app(hull_y: f64, variant: Option<u8>) -> App {
    let mut app = App::new(3);
    let side = 2usize;
    let mut net = air_net_world(side, 0);
    net.set(8, 10, 8, crate::world::WATER);
    net.set(8, 9, 8, crate::world::WATER);
    if let Some(v) = variant {
        net.variants = Some(vec![vec![0u8; 16 * 48 * 16]; side * side]);
        let ci = 8 / 16 + (8 / 16) * side;
        net.variants.as_mut().unwrap()[ci][8 + 8 * 16 + 10 * (16 * 16)] = v;
        net.variants.as_mut().unwrap()[ci][8 + 8 * 16 + 9 * (16 * 16)] = v;
    }
    app.net_world = Some(net);
    app.net_boats.push(crate::world_view::WireBoat {
        id: 700,
        x: 8.5,
        y: hull_y,
        z: 8.5,
        yaw: 0.0,
        raft: false,
        chest: false,
        wood: "oak".to_string(),
        paddle_left: false,
        paddle_right: false,
        bubble_time: 0,
        hurt_time: 0,
        hurt_dir: 1,
        damage: 0.0,
    });
    app
}

fn wire_splash_count(app: &App) -> usize {
    app.sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.generic.splash @"))
        .count()
}

/// THE FALLING TESTS' OWN FLOOR (the beat tests' platform the same
/// voxels wear — the stone 0..8, the grass 8, the air above)
fn falling_platform(app: &mut App) -> i32 {
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    {
        let w = app.offline_world.as_mut().unwrap();
        for x in 0..size {
            for z in 0..size {
                for y in 0..8 {
                    w.set(x, y, z, crate::world::STONE);
                }
                w.set(x, 8, z, crate::world::GRASS);
                for y in 9..size {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
        }
    }
    app.body = Some(crate::body::PlayerBody::new(40.0, 9.0, 40.0, 0.0, 0.0));
    size
}

impl App {
    fn render_title_buttons_for_test(&mut self) {
        let mut fb = crate::raster::Framebuf::new(320, 180);
        let mut assets = GuiAssets::open(assets_root());
        self.render(&mut fb, &mut assets);
    }
}

/// THE CONTROLLED SCENE'S OWN CORRIDOR (the pick's block-clip
/// compare the vanilla law keeps — the plants the outline's
/// full-cube fold never passes): the body->target band the aim
/// corridor rides stays air (the terrain the seed's own surface
/// keeps, the DECOR the controlled walk clears — the tall grass
/// pair the decorate pass plants the controlled scenes never aim
/// through). The east-facing ring the seed_scene's own approach
/// answers — the corridor the facing's own row wears.
fn clear_the_aim_corridor(app: &mut App) {
    let Some(world) = app.offline_world.as_mut() else {
        return;
    };
    let Some(b) = app.body.as_ref() else {
        return;
    };
    let bx = b.x.floor() as i32;
    let bz = b.z.floor() as i32;
    let spawn = app.spawn_y;
    for dx in 0..=10 {
        for dy in 0..=3 {
            world.set(bx + dx, spawn + dy, bz, crate::world::AIR);
        }
    }
}

/// A bubble-cell helper: WATER at the cell + the variant byte the
/// materialize walk would stamp (1 = whirlpool, 2 = upward).
fn bubble_cell(world: &mut crate::world::World, x: i32, y: i32, z: i32, variant: u8) {
    world.set(x, y, z, crate::world::WATER);
    world.variants.as_mut().expect("the caller builds it")[0]
        [(x.rem_euclid(16) + z.rem_euclid(16) * 16 + y.rem_euclid(48) * 256) as usize] = variant;
}

fn third_person_frame(app: &mut App) -> Vec<u8> {
    let mut assets = GuiAssets::open(
        [
            "assets-cache/jar",
            "../assets-cache/jar",
            "../../assets-cache/jar",
        ]
        .iter()
        .map(std::path::Path::new)
        .find(|p| p.join("gui/container/inventory.png").exists())
        .expect("the runtime cache the prefix walk keeps"),
    );
    let mut fb = crate::raster::Framebuf::new(320, 200);
    app.render(&mut fb, &mut assets);
    fb.color.clone()
}

fn diff_pixels(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(3)
        .zip(b.chunks_exact(3))
        .filter(|(x, y)| x != y)
        .count()
}

/// THE RENDER LANE'S OWN SERIAL LOCK (the dev-151 record's own
/// prescription — the five render-lane tests the direct-binary
/// run's 2-CPU interleave wore): the heavy lanes (the warden
/// glow, the third-person armor/head frames, the darkness pulse,
/// the water murk, the portal tint) the guard serializes — the
/// world-build + frame-render bodies the parallel harness never
/// interleaves, the honest-skip arms the transient load failure
/// the pressure window closes. The lock the execution ORDER
/// alone owns: no rendered byte the shape moves (the goldens the
/// same suite wears), the poisoned guard the panic's own
/// recovery keeps (`into_inner` — one test's panic the rest of
/// the lane never spreads).
static RENDER_LANE: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn render_lane() -> std::sync::MutexGuard<'static, ()> {
    RENDER_LANE.lock().unwrap_or_else(|e| e.into_inner())
}

/// The armored SKELETON's own stage (the flat strip the world's
/// spawn neighborhood keeps — the strip the terrain's own hills
/// never occlude): the body at the spawn, the skeleton 3 blocks
/// ahead of the body's straight-ahead look (the yaw 0 the
/// camera's own forward law speaks), the world's strip cleared
/// and re-meshed the fresh stage the frame draws.
fn armored_skeleton_stage(app: &mut App) {
    app.enter_world();
    app.spawn_offline_body();
    app.perspective = crate::camera::CameraType::ThirdPersonBack;
    // THE ZOMBIE SKIN'S OWN GATE (render_mobs's early return the
    // failed sha1 keeps — the pre-fill the walked root speaks, the
    // hardcoded CWD the test process never consults): the mob lane
    // the pre-pass and the draw both live in
    if app.zombie_avatar.is_none() {
        if let Some(root) = [
            "assets-cache/jar",
            "../assets-cache/jar",
            "../../assets-cache/jar",
        ]
        .iter()
        .map(std::path::Path::new)
        .find(|p| p.join("gui/container/inventory.png").exists())
        {
            app.zombie_avatar = crate::avatar::AvatarModel::load_zombie(root);
        }
    }
    {
        let body = app.body.as_mut().unwrap();
        body.yaw = 0.0;
        body.pitch = 0.0;
    }
    let body = app.body.as_ref().unwrap();
    let (bx, by, bz) = (body.x as i32, body.y as i32, body.z as i32);
    {
        let w = app.offline_world.as_mut().unwrap();
        for x in (bx - 5)..=(bx + 5) {
            for z in (bz - 7)..=(bz + 3) {
                for y in by..=(by + 5) {
                    w.set(x, y, z, crate::world::AIR);
                }
                w.set(x, by - 1, z, crate::world::STONE);
            }
        }
    }
    if let Some(w) = app.offline_world.as_ref() {
        app.world_quads = Some(crate::mesher::mesh_world(w, None));
    }
    // THE ARCHER (the offline mob the Skeleton kind arms — the
    // ctor's prev trio armed at the pose, the static stage the
    // no-tick frames keep): 3 blocks ahead of the body's look,
    // one block right — the camera's own frame the skeleton fills
    let mut sk = crate::mob::Zombie::new(901, (bx as f64) + 1.0, by as f64, (bz as f64) - 3.0, 262);
    sk.kind = crate::mob::MobKind::Skeleton;
    sk.on_ground = true;
    app.mobs.push(sk);
}
