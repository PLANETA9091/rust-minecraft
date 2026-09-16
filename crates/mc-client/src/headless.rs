//! Headless deterministic rendering (CI / pixel tests / no-display hosts).

use std::path::Path;
use std::time::Instant;

use anyhow::Result;

use crate::camera::Camera;
use crate::math::{Mat4, Vec3};
use crate::mesher::mesh_world;
use crate::png_io::write_rgb_png;
use crate::raster::{Framebuf, RenderCtx};
use crate::textures::TextureLut;
use crate::world::World;

pub const SKY_TOP: [f32; 3] = [0.45, 0.66, 0.94];
pub const SKY_BOTTOM: [f32; 3] = [0.74, 0.85, 0.95];
pub const FOG_COLOR: [f32; 3] = [0.74, 0.85, 0.95];

/// Deterministic orbit camera for frame `frame_index` of `frames`.
pub fn orbit_camera(world: &World, frame_index: u32, frames: u32) -> Camera {
    let n = world.size_blocks() as f32;
    let center = Vec3::new(n / 2.0, 12.0, n / 2.0);
    let radius = n * 0.62;
    let t = if frames <= 1 { 0.62 } else { frame_index as f32 / frames as f32 };
    let angle = 0.62 + t * 0.8;
    let eye = Vec3::new(
        center.x + angle.sin() * radius,
        center.y + 6.5,
        center.z + angle.cos() * radius,
    );
    Camera { pos: eye, yaw: 0.0, pitch: -0.33 }.with_look_at(center.add(Vec3::new(0.0, 2.0, 0.0)))
}

impl Camera {
    fn with_look_at(mut self, target: Vec3) -> Camera {
        let dir = target.sub(self.pos).normalized();
        self.pitch = dir.y.asin();
        self.yaw = dir.z.atan2(dir.x) + std::f32::consts::FRAC_PI_2;
        self
    }
}

/// Render `frames` deterministic frames; save the last one as PNG.
/// Returns per-frame render milliseconds.
pub fn render_frames(
    world: &World,
    lut: Option<&TextureLut>,
    width: usize,
    height: usize,
    frames: u32,
    out_path: &Path,
) -> Result<Vec<f64>> {
    let quads = mesh_world(world, lut);
    let mut fb = Framebuf::new(width, height);
    let n = world.size_blocks() as f32;
    let mut timings = Vec::with_capacity(frames as usize);

    for f in 0..frames {
        let cam = orbit_camera(world, f, frames);
        let t0 = Instant::now();
        fb.clear(SKY_TOP, SKY_BOTTOM);
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(70.0f32.to_radians(), width as f32 / height as f32, 0.1, 512.0),
            fog_color: FOG_COLOR,
            fog_near: n * 0.55,
            fog_far: n * 1.7,
        };
        fb.draw_quads(&ctx, &quads);
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        timings.push(ms);
        println!(
            "frame {}/{}: {:6.1} ms ({:6.1} fps) — {}x{}",
            f + 1,
            frames,
            ms,
            if ms > 0.0 { 1000.0 / ms } else { 0.0 },
            width,
            height
        );
    }

    write_rgb_png(out_path, width, height, &fb.color)?;
    println!("saved {}", out_path.display());
    Ok(timings)
}

/// THE SHOTS LANE's seeds — the deterministic stand-ins for the state
/// the live client carries (the F5 camera's cycle, the body the
/// third-person avatar renders, the mob lane's zombie). Every field
/// defaults absent, so the pixel-goldens' scenes stay untouched bit
/// for bit: a default `SceneOpts` seeds nothing, renders nothing new,
/// changes nothing the goldens pin.
#[derive(Default)]
pub struct SceneOpts {
    /// The F5 camera (the vanilla `CameraType` CODEC names, parsed by
    /// the CLI's `--perspective` before it lands here). `None` keeps
    /// the first-person default.
    pub perspective: Option<crate::camera::CameraType>,
    /// Spawn the deterministic player body (the third-person avatar's
    /// camera entity) at the spawn column's surface, facing east — the
    /// vanilla yaw -90 (the same east the first-person camera faces).
    pub body: bool,
    /// The body's facing override (the vanilla yaw degrees — the seed's
    /// own east -90 rides when absent; the shots stage the encounter
    /// the facing names).
    pub body_yaw: Option<f32>,
    /// The body's look pitch override (the vanilla degrees, positive
    /// up — the detached camera rides it; the shots stage the ground
    /// the look opens).
    pub body_pitch: Option<f32>,
    /// The body's mid-stride walk seed "speed,position" — the
    /// WalkAnimationState's own pair the avatar's limb cosines speak
    /// (e.g. "0.9,5.0"). The rest pose (no seed) rides speed 0.
    pub body_walk: Option<(f32, f32)>,
    /// The resolved crouch pose (the POSES table's own 1.5-body fold
    /// the avatar's setupAnim crouch branch speaks).
    pub body_crouch: bool,
    /// Spawn one deterministic zombie 4 blocks east of the body (or of
    /// the spawn column when no body) at THAT column's surface, facing
    /// west — toward the body the way the mob lane's AI approaches.
    pub zombie: bool,
    /// The zombie's remainingFireTicks (the sun-burn brand's own
    /// clock; the igniteForSeconds(8) law speaks 160).
    pub zombie_fire: Option<i32>,
    /// The zombie's deathTime (0..20 — the death-flip lane the render
    /// state's deathTime+partial read speaks).
    pub zombie_death: Option<i32>,
    /// The aggressive raised-arms pose (`ZombieRenderState.isAggressive`).
    pub zombie_aggro: bool,
    /// The hurt-flash clock (hurtTime — the OverlayTexture's 0.7 mix).
    pub zombie_hurt: Option<i32>,
    /// THE TICK LANE: game ticks with the FORWARD impulse held — the
    /// body's own movement input through the App's update (the vanilla
    /// tick the windowed loop speaks: the travel, the walk store's
    /// evolution, the footsteps). The seeded pose is the state the
    /// ticks START from. 0 = the static pose the seeds speak.
    pub walk_ticks: u32,
    /// Input-less game ticks (fed zero impulses) — the mob AI thinks,
    /// the burn roll speaks, the idle bob lives: the vanilla laws
    /// evolve the seeded scene the way the windowed loop would.
    pub settle_ticks: u32,
    /// Showcase zombies placed AHEAD of the body along its facing —
    /// 2.5 + 1.6*i blocks out, each on the spawn law's own ground
    /// scan (solid ground, free fluid-free head room), the initial
    /// yaw facing the player. Stacks with `zombie`.
    pub mobs: usize,
    /// Ignite every placed zombie for this many seconds (the
    /// igniteForSeconds' own remainingFireTicks law — 8.0 speaks 160).
    pub ignite_secs: f32,
    /// The body's remainingFireTicks (the render clock the flame
    /// feature reads — the deterministic stand-in for the live fire
    /// the lava's brand would speak; the burn lane's hurt laws ride
    /// the ticks too).
    pub body_fire: Option<i32>,
    /// Carve a shallow lava pool ahead of the body (the seed's own
    /// 3x3 shelf at four cells out along the facing, the top ground
    /// cells become lava): the walk ticks carry the body INTO it and
    /// the REAL laws speak — the ignite, the 4.0, the burn — no
    /// scripted fire.
    pub lava_pool: bool,
    /// Spawn the XP orbs ahead of the body (the award lane's own
    /// physical truth — the [3, 1, 1] walk the zombie's flat 5 speaks,
    /// floating at chest height two blocks out along the facing so the
    /// shot stages the flight the follow law owns).
    pub orbs: bool,
    /// Stage the item drops ahead of the body (the dig's own dirt and
    /// the kill's own rotten flesh — the popResource law's own
    /// physics): the mid-bounce and the settle hop the gravity law
    /// owns, the fold the render's lerp walks at the partial frames.
    pub drops: bool,
    /// THE WIRE'S OWN HOUR: the network world's Set Time opinion (the
    /// day tick the wire speaks) — the world arm swaps the staged world
    /// to the network lane and the day clock re-bases through the REAL
    /// live fold (live_view_updated's align_day_clock_to_wire), the
    /// sky renders the server's hour.
    pub net_day: Option<i64>,
    /// THE WIRE'S OWN MOBS: the network world's staged zombie count —
    /// the live_mobs_updated entry the drain speaks, the lerpTo law
    /// walking their pose.
    pub net_mobs: usize,
    /// THE DIVE SEED: carve a 3x3 pool AT the body's own column — three
    /// cells of water down (the old surface cell becomes the water
    /// surface), a forced-solid basin floor, the headroom cleared — and
    /// SINK the body onto the floor so the EYE sits under the surface
    /// (the vanilla isEyeInFluid's own geometry, feet + 1.62). The water
    /// screen overlay reads the honest gate off the seeded scene — no
    /// scripted blue.
    pub water_pool: bool,
    /// The water vision clock's seed (javap `LocalPlayer.waterVisionTime`)
    /// — the deterministic stand-in for the soak time the live drift
    /// can't hold in a staged pool; 0 = the fresh dive, 600 = the fully
    /// soaked eye (the getWaterVision ramp's own ends 24.0 / 96.0).
    pub water_vision: i32,
    /// THE AIR CLOCK'S SEED (the --water-vision discipline): the body's
    /// own airSupply — the deterministic stand-in for the mid-drain
    /// frames the live dive can't hold (the pool walks the body out);
    /// the HUD's bubble row reads the seeded store (the extract
    /// gate's own law). Clamped [-20, 300] — the dose floor the
    /// -20 fold speaks to the 300 the getMaxAirSupply sipush names.
    pub air_supply: Option<i32>,
    /// THE LAVA DIVE: carve a 3x3 pool AT the body's own column — three
    /// cells of LAVA down, a forced-solid basin floor, the headroom
    /// cleared — and SINK the body onto the floor so the EYE sits in
    /// the lava (the FogType.LAVA gate the camera cell speaks). The
    /// lava fog's murk reads the honest gate off the seeded scene; the
    /// REAL contact laws ride the ticks (the ignite, the 4.0) — the
    /// burning overlay the vanilla frame layers over the murk.
    pub lava_dive: bool,
    /// THE BURY: fill the eye's own cell with DIRT — the
    /// getViewBlockingState walk's eight samples all read the solid,
    /// the head-in-block screen arm speaks its honest frame (the
    /// particle sprite the 0xFF191919 tint carries).
    pub bury: bool,
    /// THE XP SEED "level,progress" — the deterministic stand-in for
    /// the stat the mining grants tick by tick (the level the green
    /// digits speak, the progress fraction the fill clips). The
    /// contextual bar reads the seeded store (the extract law's own
    /// gate).
    pub xp: Option<(i32, f32)>,
    /// THE PARTICLE STAGE (the lane's three scenes the shots speak):
    /// "burst" — the broken cell's 4x4x4 debris (the read-then-break
    /// discipline: the cell's own kind names the tile), "poof" — the
    /// death cloud at the body's facing point, "crit" — the emitter's
    /// 16-sample stars around a staged zombie. Anything else refuses
    /// loudly (the CLI's own paranoia).
    pub particles: Option<String>,
    /// THE FIXED CLOCK's render remainder override (the --partial
    /// lane): the staged frame's interpolation folds ride it — None
    /// keeps the default 1.0 (the current tick position bit for bit,
    /// the goldens' byte-identical contract).
    pub partial: Option<f32>,
}

impl SceneOpts {
    /// Whether ANY seed rides the opts (the CLI's loud-refuse gate:
    /// a seeded menu screen would silently render a guessed gallery,
    /// so the seeded entry demands a world screen).
    pub fn is_seeded(&self) -> bool {
        self.perspective.is_some()
            || self.body
            || self.body_yaw.is_some()
            || self.body_pitch.is_some()
            || self.body_walk.is_some()
            || self.body_crouch
            || self.zombie
            || self.zombie_fire.is_some()
            || self.zombie_death.is_some()
            || self.zombie_aggro
            || self.zombie_hurt.is_some()
            || self.mobs > 0
            || self.ignite_secs > 0.0
            || self.body_fire.is_some()
            || self.lava_pool
            || self.water_pool
            || self.water_vision != 0
            || self.air_supply.is_some()
            || self.lava_dive
            || self.bury
            || self.xp.is_some()
            || self.particles.is_some()
            || self.walk_ticks > 0
            || self.settle_ticks > 0
    }
}

/// Parse the CLI's walk seed "speed,position" — anything else refuses
/// loudly (a guessed pose would render a guessed body).
pub fn parse_walk_seed(s: &str) -> Option<(f32, f32)> {
    let (speed, position) = s.split_once(',')?;
    let speed = speed.trim().parse::<f32>().ok()?;
    let position = position.trim().parse::<f32>().ok()?;
    Some((speed, position))
}

/// Parse the CLI's xp seed "level,progress" — anything else refuses
/// loudly (a guessed stat would render a guessed bar). The progress
/// clamps [0.0, 1.0] (the fraction the fill clips); the level's [0,
/// 24791] bound is the CLI's own paranoia (the vanilla law itself is
/// the saturated add plus the under-zero bankruptcy — no ceiling).
pub fn parse_xp_seed(s: &str) -> Option<(i32, f32)> {
    let (level, progress) = s.split_once(',')?;
    let level = level.trim().parse::<i32>().ok()?;
    let progress = progress.trim().parse::<f32>().ok()?.clamp(0.0, 1.0);
    if !(0..=24791).contains(&level) {
        return None;
    }
    Some((level, progress))
}

/// The CLI name -> the CameraType (the F5 cycle's own three states —
/// an unknown name refuses loudly; a guessed state would render a
/// guessed camera).
pub fn parse_perspective(name: &str) -> anyhow::Result<crate::camera::CameraType> {
    match name {
        "first" => Ok(crate::camera::CameraType::FirstPerson),
        "third-back" => Ok(crate::camera::CameraType::ThirdPersonBack),
        "third-front" => Ok(crate::camera::CameraType::ThirdPersonFront),
        other => anyhow::bail!(
            "perspective must be \"first\", \"third-back\" or \"third-front\" (the CameraType cycle's own states), got {other:?}"
        ),
    }
}

/// Seed the deterministic scene states onto a world-screen App — the
/// shots lane's own injection point (the world exists = enter_world
/// already ran; the menu scenes carry no level and stay untouched).
/// The mobs/ignite branches refuse loudly when the stage is missing
/// (a silently shorter cast would render a guessed gallery).
pub fn seed_scene(app: &mut crate::screens::App, opts: &SceneOpts) -> Result<()> {
    if app.world_quads.is_none() {
        return Ok(()); // the menus have no level — nothing to seed
    }
    if let Some(p) = opts.perspective {
        app.perspective = p;
    }
    let n = app.world_size;
    if opts.body {
        // the spawn column's surface: the same scan enter_world spoke
        // for spawn_y, the feet AT the surface (the AABB's bottom)
        let y = app.spawn_y as f64;
        let mut body = crate::body::PlayerBody::new(
            n as f64 / 2.0 + 0.5,
            y,
            n as f64 / 2.0 + 0.5,
            opts.body_yaw.unwrap_or(-90.0), // east — the sun-rise facing
            opts.body_pitch.unwrap_or(0.0),
        );
        body.on_ground = true;
        if opts.body_crouch {
            body.crouching = true;
            body.want_crouch = true;
        }
        if let Some((speed, position)) = opts.body_walk {
            body.walk.speed_old = speed;
            body.walk.speed = speed;
            body.walk.position = position;
        }
        app.body = Some(body);
    }
    if opts.zombie {
        // THE APPROACH CELL: the AI's own lane walks on GROUND — the
        // deterministic stand-in scans the columns around the body for
        // the first WALKABLE one (same surface level, the pathable
        // approach the follow-goal settles into), radii 3..=8, the
        // fixed east->west->north->south ring order. Every candidate
        // refuses unless its surface equals the body's own — a zombie
        // on a hilltop above the camera renders a guessed scene.
        let bx = n / 2;
        let bz = n / 2;
        let spawn_y = app.spawn_y;
        let mut cell = None;
        'ring: for d in 3..=8i32 {
            let candidates = [(bx + d, bz), (bx - d, bz), (bx, bz + d), (bx, bz - d)];
            for (cx, cz) in candidates {
                let y = app
                    .offline_world
                    .as_ref()
                    .map(|w| crate::screens::top_solid_y(w, cx, cz))
                    .unwrap_or(spawn_y);
                if y == spawn_y {
                    cell = Some((cx, cz, y));
                    break 'ring;
                }
            }
        }
        let (cx, cz, y) = cell.unwrap_or((bx + 4, bz, spawn_y));
        let mut z = crate::mob::Zombie::new(
            1,
            cx as f64 + 0.5,
            y as f64,
            cz as f64 + 0.5,
            262, // the 26.2 jar's own number — as good a seed as any
        );
        // facing the body it walks to: the cardinal the scan landed on
        // names the yaw (west if east of the body, east if west,
        // south if north, north if south — the vanilla yaw degrees)
        z.yaw = if cx > bx {
            90.0
        } else if cx < bx {
            -90.0
        } else if cz > bz {
            180.0
        } else {
            0.0
        };
        z.on_ground = true;
        z.age_ticks = 40; // two seconds in — the bob's stable mid value
        z.aggressive = opts.zombie_aggro;
        z.fire_ticks = opts.zombie_fire.unwrap_or(0);
        z.death_time = opts.zombie_death.unwrap_or(0);
        z.hurt_time = opts.zombie_hurt.unwrap_or(0);
        // the approach's mid-stride (the walk store the limb cosines
        // speak — the same pair the live AI's distance feed grows)
        z.walk.speed_old = 0.7;
        z.walk.speed = 0.7;
        z.walk.position = 2.5;
        app.mobs.push(z);
    }
    if opts.orbs {
        // THE ORB SEED (the --zombie-fire discipline: the deterministic
        // stand-in for the orbs the kill's award spawns): the flat 5's
        // own [3, 1, 1] walk, placed 2 blocks ahead of the body's
        // facing at chest height (the flight the follow law owns — the
        // shot stages the orbs mid-air, the pull's own truth). The
        // ages stagger so the pulse's phase spread shows (the
        // ageInTicks/2 sin bands the render law speaks).
        let yaw = opts.body_yaw.unwrap_or(-90.0);
        let (fx, fz) = (
            (-yaw.to_radians().sin()) as f64,
            yaw.to_radians().cos() as f64,
        );
        let (bx, by, bz) = match app.body.as_ref() {
            Some(b) => (b.x, b.y, b.z),
            None => (n as f64 / 2.0 + 0.5, app.spawn_y as f64, n as f64 / 2.0 + 0.5),
        };
        for (i, v) in [3, 1, 1].into_iter().enumerate() {
            let spread = (i as f64 - 1.0) * 0.35;
            let rx = -fz;
            let rz = fx;
            let mut o = crate::xp::ExperienceOrb::spawn(
                app.next_orb_id,
                bx + fx * 2.0 + rx * spread,
                by + 1.2 + (i as f64 - 1.0) * 0.12,
                bz + fz * 2.0 + rz * spread,
                v,
            );
            o.age = i as i32 * 7;
            app.next_orb_id += 1;
            app.orbs.push(o);
        }
    }
    if opts.drops {
        // THE DROP SEED (the --orbs discipline: the deterministic
        // stand-in for the loot the dig's popResource and the kill's
        // spawnAtLocation spawn): the dig's own dirt and the kill's
        // own rotten flesh, placed ahead of the body's facing on the
        // ground lane (the physics the gravity + sweep laws own — the
        // shot stages the mid-bounce, the fold the render's lerp
        // walks at the partial frames). The unknown model renders
        // nothing — the honest skip the render law speaks.
        let yaw = opts.body_yaw.unwrap_or(-90.0);
        let (fx, fz) = (
            (-yaw.to_radians().sin()) as f64,
            yaw.to_radians().cos() as f64,
        );
        let (bx, by, bz) = match app.body.as_ref() {
            Some(b) => (b.x, b.y, b.z),
            None => (n as f64 / 2.0 + 0.5, app.spawn_y as f64, n as f64 / 2.0 + 0.5),
        };
        let dirt = mc_registry::item_id_by_name("dirt").unwrap_or(0);
        let flesh = mc_registry::item_id_by_name("rotten_flesh").unwrap_or(dirt);
        let mut rng = crate::drops::Lcg::new(0xD10F);
        // the dig's own reward: one dirt at the near lane, the settle
        // phase's own hop (the ticks stagger the bob's phase spread)
        let mut d0 = crate::drops::ItemDrop::spawn_entity_loot(
            app.next_drop_id,
            dirt,
            1,
            bx + fx * 1.5 - fz * 0.3,
            by + 0.6,
            bz + fz * 1.5 + fx * 0.3,
            &mut rng,
        );
        d0.age = 12;
        app.next_drop_id += 1;
        app.drops.push(d0);
        // the kill's own loot: one rotten flesh further out, the
        // fresh-spawn mid-bounce (the popResource lift the law
        // speaks)
        let mut d1 = crate::drops::ItemDrop::spawn_entity_loot(
            app.next_drop_id,
            flesh,
            1,
            bx + fx * 2.2 + fz * 0.4,
            by + 0.8,
            bz + fz * 2.2 - fx * 0.4,
            &mut rng,
        );
        d1.vy = 0.12;
        app.next_drop_id += 1;
        app.drops.push(d1);
    }
    if let Some(fire) = opts.body_fire {
        if let Some(b) = app.body.as_mut() {
            // THE BRAND'S SEED (the render clock the flame feature
            // reads — the deterministic stand-in for the live fire the
            // lava's brand speaks; the burn laws ride the ticks too)
            b.remaining_fire_ticks = fire.max(0);
        }
    }
    if opts.lava_pool {
        // THE POOL'S CARVE (the deterministic shelf AT the body): 3x3
        // centered on the body's own column (the spawn column's own
        // when no body) — the top ground cells become LAVA. The
        // vanilla shallow-pool physics speak: the body sinks to the
        // pool floor (the cell below stays solid), the box overlaps
        // the lava cell, the REAL laws answer (the ignite, the 4.0,
        // the burn) — no scripted fire. The carve rebuilds the mesh
        // (the pool's pixels ride the same FACE_TILES tile the
        // terrain speaks). AT the body (not ahead of it): the spawn
        // valleys' walls sit 2-3 cells out — a farther shelf lands on
        // the hillside the walls carry, the sunk scene the gallery
        // wants never forms.
        let n = app.world_size as i32;
        let (bx, bz) = (n / 2, n / 2);
        let (px, pz) = (bx, bz);
        if let Some(w) = app.offline_world.as_mut() {
            for ox in -1..=1i32 {
                for oz in -1..=1i32 {
                    let x = px + ox;
                    let z = pz + oz;
                    let sy = crate::screens::top_solid_y(w, x, z);
                    w.set(x, sy, z, crate::world::LAVA);
                    // THE DRY BASIN (the pond-spawn lesson: a pool
                    // carved under the pond's water speaks the douse,
                    // the brand dies the same tick — the headroom
                    // clears FOUR cells above, AND the water table
                    // UNDER the surface cell (the saturated beach
                    // layer the worldgen fills) becomes lava too —
                    // the pool's own body, no surviving water cell
                    // the sunk body's box could touch)
                    for hy in 1..=4 {
                        w.set(x, sy + hy, z, crate::world::AIR);
                    }
                    let mut dy = sy - 1;
                    while dy >= 0 && w.get(x, dy, z) == crate::world::WATER {
                        w.set(x, dy, z, crate::world::LAVA);
                        dy -= 1;
                    }
                }
            }
            app.remesh_offline();
        }
    }
    if opts.water_pool {
        // THE DIVE'S CARVE (the deterministic pool AT the body): 3x3
        // centered on the body's own column (the spawn column's own
        // when no body) — the pool level anchors to the CENTER column's
        // own surface (a taller neighbor — a tree, a slope — never
        // lifts the basin), the top THREE cells become WATER, the
        // basin floor under them is forced solid (a pool with a
        // missing bottom renders a guessed scene), the headroom above
        // is cleared four cells like the lava shelf speaks. The body
        // SINKS onto the floor (feet at the floor's top): the eye rides
        // feet + 1.62 one cell under the surface — the vanilla
        // isEyeInFluid's own geometry, the water-screen gate reads it
        // honestly.
        dive_carve(app, crate::world::WATER);
    }
    if opts.lava_dive {
        // THE LAVA DIVE'S CARVE (the water dive's own sibling — the
        // SAME 3x3/3-deep shape, the LAVA the palette's second fluid
        // speaks): the eye IN the lava cell — the FogType.LAVA gate the
        // fog lane reads. THE BRAND rides the stage (the
        // igniteForSeconds(15) fold's 300): the contact's same-tick
        // ignite the live law speaks — the --screen lane owns no ticks
        // (the frame counter only aligns the sky), the staged stand-in
        // keeps the frame honest (a frame IN lava is always a burning
        // frame).
        dive_carve(app, crate::world::LAVA);
        if let Some(b) = app.body.as_mut() {
            b.remaining_fire_ticks = b.remaining_fire_ticks.max(crate::body::LAVA_FIRE_TICKS);
        }
    }
    if opts.bury {
        // THE BURY (getViewBlockingState's own gate the eye's cell
        // speaks): the DIRT block lands in the cell the EYE occupies
        // (feet + 1.62, the vanilla eye geometry) — the eight samples
        // all read the solid, the head-in-block arm submits the block
        // sprite (the suffocation view the vanilla screen speaks). No
        // ticks ride the stage (the --screen lane owns none): the
        // frame is the buried view's own truth.
        if let Some(w) = app.offline_world.as_mut() {
            if let Some(b) = app.body.as_ref() {
                let (px, pz) = (n / 2, n / 2);
                let eye_cell_y = (b.y + 1.62).floor() as i32;
                w.set(px, eye_cell_y, pz, crate::world::DIRT);
                app.remesh_offline();
            }
        }
    }
    if let Some((level, progress)) = opts.xp {
        // THE XP SEED (the --zombie-fire discipline: the deterministic
        // stand-in for the stat the mining grants tick by tick). The
        // seeded store reads through the SAME gates the live grant
        // feeds — the contextual bar's extract law, the level digits'
        // gate. No ticks ride the stage (the --screen lane owns none):
        // the frame is the seeded stat's own truth.
        app.xp.level = level;
        app.xp.progress = progress;
    }
    if opts.water_vision != 0 {
        // THE SOAK CLOCK'S SEED (the --zombie-fire discipline): the
        // deterministic stand-in for the soak time the live drift
        // can't hold — the eye's +1/tick climb needs a pool the water's
        // own push never walks out of. The getWaterVision ramp reads
        // the seeded clock (the water fog's end distance law).
        app.water_vision_time = opts.water_vision.clamp(0, 600);
    }
    if let Some(air) = opts.air_supply {
        // THE AIR CLOCK'S SEED (the same discipline): the body's own
        // airSupply — the mid-drain frames the live dive can't hold,
        // the bubble row's extract reads the seeded store. The
        // [-20, 300] clamp: the dose floor to the sipush's own 300.
        if let Some(b) = app.body.as_mut() {
            b.air_supply = air.clamp(crate::body::DROWN_HURT_FLOOR, crate::body::TOTAL_AIR_SUPPLY);
        }
    }
    if opts.mobs > 0 {
        spawn_showcase_mobs(app, opts.mobs)?;
    }
    if opts.ignite_secs > 0.0 {
        // THE BRAND (igniteForSeconds' own law: remainingFireTicks =
        // max(remaining, secs * 20) — the seeded clock only grows)
        let ticks = (opts.ignite_secs * 20.0) as i32;
        for mob in app.mobs.iter_mut() {
            mob.fire_ticks = mob.fire_ticks.max(ticks);
        }
    }
    if let Some(dt) = opts.zombie_death.filter(|d| *d > 0) {
        // THE DEATH CLOCK's own seed (the same only-grows law the
        // brand speaks) — the death-flip lane renders mid-flip at the
        // clock's value, the 20-tick removal never runs headless)
        for mob in app.mobs.iter_mut() {
            mob.death_time = mob.death_time.max(dt);
        }
    }
    if let Some(kind) = opts.particles.as_deref() {
        // THE PARTICLE STAGE (the --orbs discipline: the facing point
        // two ahead, the body's own yaw naming it, the flight ticks
        // walked so the frame shows the mid-air truth — the --screen
        // lane owns no ticks, the stage walks the spawned ones its
        // own law speaks).
        let yaw = opts.body_yaw.unwrap_or(-90.0);
        let (fx, fz) = (
            (-yaw.to_radians().sin()) as f64,
            yaw.to_radians().cos() as f64,
        );
        let (bx, by, bz) = match app.body.as_ref() {
            Some(b) => (b.x, b.y, b.z),
            None => (
                n as f64 / 2.0 + 0.5,
                app.spawn_y as f64,
                n as f64 / 2.0 + 0.5,
            ),
        };
        match kind {
            "burst" => {
                // THE READ-THEN-BREAK: the SURFACE cell of the column
                // two ahead (top_solid_y — the classic mining look, the
                // burst rides the ground's own level), the cell's own
                // kind names the particle tile, the mesh re-speaks the
                // break, the 64 debris fly the fraction law
                let cx = (bx + fx * 1.5).floor() as i32;
                let cz = (bz + fz * 1.5).floor() as i32;
                // the LOCAL ground: the first solid at or below the
                // feet level (the mining-the-block-in-front-of-you
                // framing — the column's global top rides the ridge the
                // valley walls carry, out of the frame's own view)
                let cell_kind = app
                    .offline_world
                    .as_ref()
                    .map(|w| {
                        let mut y = by as i32;
                        while y > 0 && w.get(cx, y, cz) == crate::world::AIR {
                            y -= 1;
                        }
                        (w.get(cx, y, cz), y)
                    })
                    .unwrap_or((crate::world::AIR, by as i32));
                let (kind, cy) = cell_kind;
                let tile = crate::screens::particle_tile(kind);
                if let Some(w) = app.offline_world.as_mut() {
                    w.set(cx, cy, cz, crate::world::AIR);
                    app.remesh_offline();
                }
                if tile != mc_registry::generated::atlas::TILE_NONE {
                    let burst = crate::particles::terrain_burst(
                        tile, cx, cy, cz, &mut app.mob_rng,
                    );
                    app.particles.extend(burst);
                }
            }
            "poof" => {
                // THE CLOUD AT THE FACING POINT (the zombie's own box —
                // the ground the spawn's own surface speaks)
                let px = bx + fx * 1.5;
                let pz = bz + fz * 1.5;
                let bbox = (
                    px - 0.3,
                    by,
                    pz - 0.3,
                    px + 0.3,
                    by + crate::mob::ZOMBIE_HEIGHT,
                    pz + 0.3,
                );
                let cloud = crate::particles::poof_cloud(bbox, &mut app.mob_rng);
                app.particles.extend(cloud);
            }
            "crit" => {
                // THE EMITTER'S STARS (the staged zombie ahead —
                // the emitter arms at its box, two ticks of the
                // 16-sample walk spawn the stars mid-flight)
                let px = bx + fx * 2.5;
                let pz = bz + fz * 2.5;
                // the LOCAL ground (the first solid at or below the
                // feet level — the column's global top rides the tree
                // crown the facing column carries)
                let y = app
                    .offline_world
                    .as_ref()
                    .map(|w| {
                        let mut gy = by as i32;
                        while gy > 0
                            && w.get(px.floor() as i32, gy, pz.floor() as i32)
                                == crate::world::AIR
                        {
                            gy -= 1;
                        }
                        gy + 1
                    })
                    .unwrap_or(by as i32);
                let z = crate::mob::Zombie::new(1, px, y as f64, pz, 262);
                let bbox = z.box_at();
                let id = z.id;
                app.mobs.push(z);
                let mut em = crate::particles::CritEmitter::new(id, bbox);
                for _ in 0..3 {
                    let (spawned, alive) =
                        em.tick(Some((px, y as f64, pz)), &mut app.mob_rng);
                    app.particles.extend(spawned);
                    if !alive {
                        break;
                    }
                }
                if em.life < 3 {
                    app.crit_emitters.push(em);
                }
            }
            "damage" => {
                // THE HIT'S OWN STAGE (the damageStatsAndHearts law the
                // live attack site answers at the gate): the staged
                // zombie at the facing point, the dealt-3.0 hit's
                // count-1... the stage replays the FULL emitter chain
                // with the dealt the staging names (6.0 -> 3 stars —
                // the sword's own band), the six gaussians the packet
                // walk speaks, the +1.0 up-burst the provider rides
                let px = bx + fx * 1.8;
                let pz = bz + fz * 1.8;
                let y = app
                    .offline_world
                    .as_ref()
                    .map(|w| {
                        let mut gy = by as i32;
                        while gy > 0
                            && w.get(px.floor() as i32, gy, pz.floor() as i32)
                                == crate::world::AIR
                        {
                            gy -= 1;
                        }
                        gy + 1
                    })
                    .unwrap_or(by as i32);
                let z = crate::mob::Zombie::new(1, px, y as f64, pz, 262);
                app.mobs.push(z);
                // THE COUNT THE SWORD SPEAKS (damageStatsAndHearts'
                // own gate: the dealt 6.0 crosses > 2.0 and counts
                // (int)(6.0 * 0.5) = 3 — the three stars the walk
                // spreads)
                let stars = crate::particles::damage_indicators(
                    px,
                    y as f64 + crate::mob::ZOMBIE_HEIGHT * 0.5,
                    pz,
                    3,
                    &mut app.mob_rng,
                );
                app.particles.extend(stars);
            }
            "bubble" => {
                // THE DROWN MOMENT'S OWN STAGE (the broadcast-67 arm
                // the live lane answers at the dose): the pool the
                // dive carve speaks, the air clock SPENT (the dose's
                // own reset the body fold speaks — the HUD's bubble
                // row reads the zero), the 8 bubbles at the body's
                // feet riding the body's own delta movement — the
                // triangle spread the makeDrownParticles law speaks
                dive_carve(app, crate::world::WATER);
                if let Some(b) = app.body.as_mut() {
                    b.air_supply = 0;
                }
                if let Some(b) = app.body.as_ref() {
                    let cloud = crate::particles::drown_bubbles(
                        b.x, b.y, b.z, b.vx, b.vy, b.vz, &mut app.mob_rng,
                    );
                    app.particles.extend(cloud);
                }
            }
            "splash" => {
                // THE ENTRY'S OWN STAGE (the doWaterSplashEffect loops
                // the live lane answers at the gate): the pool the
                // dive carve speaks, the two rings at the body's own
                // pose — the surface Y the Mth.floor law folds
                // (floor(feet) + 1.0 — the cell the sunk body's own
                // depth speaks), the 13-per-ring walk the width * 20
                // bound speaks, the BUBBLE ring first (the carried
                // velocity's z thinned - r * 0.2), the SPLASH ring
                // second (the ctor's own random pop)
                dive_carve(app, crate::world::WATER);
                if let Some(b) = app.body.as_ref() {
                    let rings = crate::particles::water_entry_rings(
                        b.x, b.y, b.z, b.vx, b.vy, b.vz,
                        crate::body::WIDTH,
                        &mut app.mob_rng,
                    );
                    app.particles.extend(rings);
                }
            }
            "lava" => {
                // THE POOL'S OWN STAGE (the animateTick lane the live
                // tick answers every world tick): the lava pool the
                // --lava-pool carve speaks (the stage DEMANDS it — the
                // CLI's own paranoia), the body AT the pool (the
                // spawn valleys' walls sit 2-3 cells out — a shifted
                // body lands on the hillside the walls carry), the
                // 667x2 walk run for the staged tick count (the two
                // seeded streams the live lane speaks) so the drips +
                // the ambient voices fire on the pool's 9 surface
                // cells
                if !opts.lava_pool {
                    anyhow::bail!(
                        "--particles lava demands --lava-pool (the pool the walk reads)"
                    );
                }
                for _ in 0..240 {
                    let spawns = if let Some(b) = app.body.as_ref() {
                        app.lava_ambient_walk((b.x, b.y, b.z))
                    } else {
                        Vec::new()
                    };
                    app.particles.extend(spawns);
                }
            }
            "sweep" => {
                // THE ARC'S OWN STAGE (the doSweepAttack tail the live
                // attack site answers at the gate): the staged zombie
                // at the facing point (the sword's own hit the arc
                // rides), the ONE particle the count-0 walk leaves at
                // the body's own mid height one forward block — the
                // position `getX() - sin(yaw), getY(0.5), getZ() +
                // cos(yaw)` the tail speaks, the (d8, 0.0, d10)
                // velocity pair the ctor's quadSize fold reads, the
                // grey tint the one draw the ctor keeps
                let px = bx + fx * 1.8;
                let pz = bz + fz * 1.8;
                let y = app
                    .offline_world
                    .as_ref()
                    .map(|w| {
                        let mut gy = by as i32;
                        while gy > 0
                            && w.get(px.floor() as i32, gy, pz.floor() as i32)
                                == crate::world::AIR
                        {
                            gy -= 1;
                        }
                        gy + 1
                    })
                    .unwrap_or(by as i32);
                let z = crate::mob::Zombie::new(1, px, y as f64, pz, 262);
                app.mobs.push(z);
                if let Some(b) = app.body.as_ref() {
                    let vyaw = b.yaw as f64 * std::f64::consts::PI / 180.0;
                    let sy = vyaw.sin();
                    let cy = vyaw.cos();
                    let mid = b.y
                        + if b.crouching {
                            crate::body::CROUCH_HEIGHT
                        } else {
                            crate::body::STANDING_HEIGHT
                        }
                            * 0.5;
                    let arc = crate::particles::sweep(
                        b.x - sy,
                        mid,
                        b.z + cy,
                        -sy,
                        cy,
                        app.mob_rng.next_f32(),
                    );
                    app.particles.push(arc);
                }
            }
            other => {
                anyhow::bail!(
                    "--particles: unknown scene '{other}' (the lane speaks burst | poof | crit | damage | bubble | splash | lava | sweep)"
                );
            }
        }
        // the walk ticks per scene (the base Particle law the spawned
        // ones ride — the frame shows the flight's own truth: the
        // burst's two spread ticks, the poof's ten (the generic
        // frames' own wisp-to-cloud growth the fade-in speaks), the
        // crit's two bloom ticks, the bubble's ten rise ticks) — the
        // bubble's own walk answers the water gate the pool's cells
        // speak (the dry cell removes, the live lane's own read)
        let walk = match kind {
            "poof" => 20,
            "crit" => 6,
            // the stars' own flight (the +1.0 burst lifts them a
            // block a tick — the shutter catches the head-height
            // moment the first tick speaks)
            "damage" => 1,
            // the arc's own walk (the static pose the override keeps —
            // the shutter catches age 1, the frame-1 moment the get
            // law speaks)
            "sweep" => 1,
            "bubble" => 10,
            "splash" => 4,
            // the drips' own flight (the pop's arc rides ~3 ticks of
            // the 16..80 lifetime — mid-air at the shutter)
            "lava" => 3,
            _ => 2,
        };
        let wet_world = app.offline_world.as_ref();
        let mut smoke_out: Vec<(f64, f64, f64, f64, f64, f64)> = Vec::new();
        for _ in 0..walk {
            app.particles.retain_mut(|p| match p.kind {
                crate::particles::ParticleKind::Bubble => {
                    let wet = wet_world
                        .map(|w| {
                            w.get(
                                p.x.floor() as i32,
                                p.y.floor() as i32,
                                p.z.floor() as i32,
                            ) == crate::world::WATER
                        })
                        .unwrap_or(false);
                    p.tick_bubble(wet)
                }
                crate::particles::ParticleKind::Splash { .. } => p.tick_splash(),
                // the staged arc ticks the same age-only walk (the
                // static pose the override keeps)
                crate::particles::ParticleKind::Sweep => p.tick_sweep(),
                // the staged drips tick the same law (the smoke gate's
                // rolls ride the level stream, the children land after
                // — the pool's own truth the live lane speaks)
                crate::particles::ParticleKind::Lava => {
                    let mut rng = std::mem::replace(
                        &mut app.level_ambient_rng,
                        crate::drops::Lcg::new(0),
                    );
                    let kept = p.tick_lava(&mut rng, &mut smoke_out);
                    app.level_ambient_rng = rng;
                    kept
                }
                _ => p.tick(),
            });
        }
        // the staged children land after the loop (the live lane's
        // own order — the ctor draws ride the level stream)
        for (px, py, pz, pvx, pvy, pvz) in smoke_out {
            let child = crate::particles::smoke_puff(
                px, py, pz, pvx, pvy, pvz,
                app.level_ambient_rng.next_f32(),
                app.level_ambient_rng.next_f32(),
                app.level_ambient_rng.next_f32(),
            );
            app.particles.push(child);
        }
    }
    Ok(())
}

/// The dive carve's own shape (the water and the lava dive share it):
/// a 3x3 pool centered on the spawn column — the top THREE cells become
/// the caller's fluid, the basin floor under them is forced solid (a
/// pool with a missing bottom renders a guessed scene), the headroom
/// above is cleared four cells — and the body SINKS onto the floor
/// (feet at the floor's top): the eye rides feet + 1.62 one cell under
/// the surface, the vanilla isEyeInFluid's own geometry the fluid gates
/// read honestly.
fn dive_carve(app: &mut crate::screens::App, fluid: u16) {
    let n = app.world_size as i32;
    let (px, pz) = (n / 2, n / 2);
    if let Some(w) = app.offline_world.as_mut() {
        let level = crate::screens::top_solid_y(w, px, pz);
        for ox in -1..=1i32 {
            for oz in -1..=1i32 {
                let x = px + ox;
                let z = pz + oz;
                w.set(x, level, z, fluid);
                w.set(x, level - 1, z, fluid);
                w.set(x, level - 2, z, fluid);
                // THE BASIN FLOOR: the saturated beach layer under the
                // carve speaks the fluid in the pond columns — a fluid
                // floor lets the pool drain into a guessed pit, the
                // solid dirt keeps the scene honest
                if !crate::body::solid(w, x, level - 3, z) {
                    w.set(x, level - 3, z, crate::world::DIRT);
                }
                for hy in 1..=4 {
                    w.set(x, level + hy, z, crate::world::AIR);
                }
            }
        }
        app.remesh_offline();
        // THE SINK (the pool's own column): the feet land on the basin
        // floor's top (level - 2), the eye one cell under the surface
        if let Some(b) = app.body.as_mut() {
            b.y = (level - 2) as f64;
            b.on_ground = true;
        }
    }
}

/// The showcase zombies' placement (the spawn law's own ground scan):
/// n zombies AHEAD of the body along its facing — the i-th one seeks
/// outward from 2.5 + 1.6*i blocks, stepping the ray in 1.0-block
/// increments up to 12 more, and lands at the CENTER of the first
/// cell whose ground is solid and whose head room is air and
/// fluid-free (the spawn law's own water gate — a pool column never
/// hosts one, the ray walks past it), the initial yaw already facing
/// the player. THE LATERAL SPREAD: every mob rides a perpendicular
/// offset — +1.1 right for even i, -1.1-1.1*k left for odd — so the
/// cast flanks the body instead of stacking into its silhouette (an
/// on-axis zombie hides behind the very avatar it approaches). No RNG
/// in the placement (a pinned shot needs a pinned stage); the
/// per-entity seed rides the index so the AI stream stays
/// deterministic.
const SHOWCASE_SEEK_BASE: f64 = 2.5;
const SHOWCASE_SEEK_STRIDE: f64 = 1.6;
const SHOWCASE_SEEK_EXTRA: f64 = 12.0;

/// One column's ground verdict (the spawn law's own scan): the
/// topmost solid with a free, fluid-free head cell — the water
/// surface never hosts one.
fn showcase_ground_at(
    w: &crate::world::World,
    sx: i32,
    sz: i32,
    top: i32,
) -> Option<i32> {
    for y in (0..top).rev() {
        if crate::body::solid(w, sx, y, sz)
            && !crate::body::solid(w, sx, y + 1, sz)
            && w.get(sx, y + 1, sz) != crate::world::WATER
        {
            return Some(y + 1);
        }
    }
    None
}

fn spawn_showcase_mobs(app: &mut crate::screens::App, n: usize) -> Result<()> {
    let (bx, bz, body_yaw) = {
        let body = app.body.as_ref().ok_or_else(|| {
            anyhow::anyhow!("--mobs requires --body (the placement rides the player's position)")
        })?;
        (body.x, body.z, body.yaw)
    };
    // THE FACING (wire_angles' own inverse: cam.yaw = body yaw + PI;
    // the camera forward's xz pair = (sin, -cos))
    let cam_yaw = body_yaw.to_radians() + std::f32::consts::PI;
    let (fx, fz) = (cam_yaw.sin() as f64, -(cam_yaw.cos() as f64));
    // THE PERPENDICULAR (the left-hand pair: rotate the facing 90°)
    let (px, pz) = (-fz, fx);
    // the placements resolve first (the world reads before the push)
    let mut placed: Vec<(f64, f64, f64)> = Vec::with_capacity(n);
    {
        let w = app.offline_world.as_ref().ok_or_else(|| {
            anyhow::anyhow!("--mobs requires the offline world (the world screen owns one)")
        })?;
        let size = w.size_blocks() as i32;
        for i in 0..n {
            let base = SHOWCASE_SEEK_BASE + SHOWCASE_SEEK_STRIDE * i as f64;
            // THE LATERAL (the flank offset the silhouette law speaks:
            // even i right, odd i left, each wider than the last)
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            let lat = side * (1.1 + 1.1 * (i as f64 / 2.0).floor());
            let (ox, oz) = (px * lat, pz * lat);
            // THE SEEK (the ray walk past the water): the first valid
            // column wins, the bound refuses loudly
            let mut picked: Option<(i32, i32, i32)> = None;
            let mut probe = base;
            while probe <= base + SHOWCASE_SEEK_EXTRA {
                let (mx, mz) = (bx + fx * probe + ox, bz + fz * probe + oz);
                let (sx, sz) = (mx.floor() as i32, mz.floor() as i32);
                if sx < 1 || sx >= size - 1 || sz < 1 || sz >= size - 1 {
                    break; // the world's edge ends the seek
                }
                if let Some(fy) = showcase_ground_at(w, sx, sz, size) {
                    picked = Some((sx, fy, sz));
                    break;
                }
                probe += 1.0;
            }
            let (sx, fy, sz) = picked.ok_or_else(|| {
                anyhow::anyhow!(
                    "no solid ground along the facing ray for mob {i} (body at {bx},{bz})"
                )
            })?;
            placed.push((sx as f64 + 0.5, fy as f64, sz as f64 + 0.5));
        }
    }
    let first_id = app.next_mob_id;
    app.next_mob_id += n as i32;
    for (i, (mx, my, mz)) in placed.into_iter().enumerate() {
        let mut z = crate::mob::Zombie::new(first_id + i as i32, mx, my, mz, 9000 + i as u64);
        // the initial yaw faces the player (the AI's own atan2(-dx, dz)
        // law over the mob->player direction; the chase refines it the
        // first tick it thinks)
        let (dx, dz) = (bx - mx, bz - mz);
        z.yaw = (-dx).atan2(dz).to_degrees() as f32;
        app.mobs.push(z);
    }
    Ok(())
}

/// THE TICK LANE (the seeded scene's evolution): the walk ticks feed
/// the body's own movement input through the App's update (the vanilla
/// tick the windowed loop speaks — the travel, the walk store's
/// evolution, the footsteps, the mob lane's clocks), the settle ticks
/// feed nothing (the AI chase, the burn roll, the idle bob live
/// through them). The frozen swing (a seeded render slice) re-freezes
/// AFTER the ticks — a real tick resumes the clock, the freeze re-pins
/// the render's slice.
pub fn tick_scene(
    app: &mut crate::screens::App,
    opts: &SceneOpts,
    swing_progress: Option<f32>,
) -> Result<()> {
    if opts.walk_ticks == 0 && opts.settle_ticks == 0 {
        return Ok(()); // the static lane — nothing to evolve
    }
    anyhow::ensure!(
        app.screen == crate::screens::ScreenKind::World,
        "the tick lane stages the world screens only (world | worldnight | pause)"
    );
    for _ in 0..opts.walk_ticks {
        // the forward impulse (the crouch rides the body's own seed —
        // the crouch-walk the POSES table's 1.5-body fold speaks)
        let crouch = app.body.as_ref().map(|b| b.want_crouch).unwrap_or(false);
        app.on_movement_input(1.0, 0.0, false, crouch, false);
        app.update();
    }
    for _ in 0..opts.settle_ticks {
        app.on_movement_input(0.0, 0.0, false, false, false);
        app.update();
    }
    if let Some(p) = swing_progress {
        app.swing = crate::hand::SwingState::frozen_at(p);
    }
    Ok(())
}

/// Render one deterministic GUI screen (boot chain / menus) to PNG.
/// `frame` drives boot progress, splash pulse and music scheduling.
/// `held_item` (a model id, e.g. "item/iron_sword") holds that item in
/// the first-person hand on the world screens — the deterministic
/// stand-in for inventory state; `None` = the empty-hand arm (goldens).
/// `held_charge` seeds the held stack's CHARGED_PROJECTILES component
/// ("arrow" | "rocket") — the loaded crossbow's own resting lane, the
/// charge_type select's case render; requires the crossbow in hand.
/// `swing_progress` freezes the attack-swing clock at that progress
/// (`getAttackAnim`, 0 = rest) — the deterministic stand-in for the
/// swing clock; the world screens render the mid-swing pose of the
/// held item (or of the empty-hand arm), the other screens ignore it.
/// `slim_arm` selects the slim (Alex) skin variant for the empty-hand
/// arm — the deterministic stand-in for the profile's texture metadata
/// (the goldens never set it: the default stays the wide classic).
pub fn render_gui_screen(
    screen: &str,
    frame: u64,
    seed: u64,
    width: usize,
    height: usize,
    held_item: Option<&str>,
    held_charge: Option<&str>,
    swing_progress: Option<f32>,
    slim_arm: bool,
    out_path: &Path,
) -> Result<()> {
    // the goldens' own entry — the default opts seed NOTHING, so this
    // call renders the exact pixels the pre-shots-lane scenes spoke
    render_gui_screen_opts(
        screen,
        frame,
        seed,
        width,
        height,
        held_item,
        held_charge,
        swing_progress,
        slim_arm,
        &SceneOpts::default(),
        out_path,
    )
}

/// The seeded entry — the shots lane's flags land here (the
/// `SceneOpts` above, plus the tick lane the walk/settle fields
/// speak). Every goldens call rides the plain `render_gui_screen`
/// delegate, so the default opts never touch those pixels; a SEEDED
/// call demands a world screen (the menus refuse loudly — a silently
/// seeded menu would render a guessed gallery).
pub fn render_gui_screen_opts(
    screen: &str,
    frame: u64,
    seed: u64,
    width: usize,
    height: usize,
    held_item: Option<&str>,
    held_charge: Option<&str>,
    swing_progress: Option<f32>,
    slim_arm: bool,
    opts: &SceneOpts,
    out_path: &Path,
) -> Result<()> {
    use crate::gui::GuiAssets;
    use crate::screens::{App, ScreenKind};

    if let Some(p) = swing_progress {
        anyhow::ensure!(
            (0.0..=1.0).contains(&p),
            "swing-progress must be in [0, 1] (getAttackAnim's domain), got {p}"
        );
    }
    let mut app = App::new(seed);
    app.slim_arm = slim_arm;
    if let Some(id) = held_item {
        app.held_item = crate::hand::load_item(id);
    }
    if let Some(kind) = held_charge {
        // THE CHARGE SEED: the held stack's own component state (the
        // loaded crossbow's resting lane). The kind names are the
        // CrossbowItem$ChargeType enum's own lowercase codec names —
        // anything else refuses loudly (a guessed state would render a
        // guessed payload)
        let kind = match kind {
            "arrow" => mc_registry::ChargeKind::Arrow,
            "rocket" => mc_registry::ChargeKind::Rocket,
            other => anyhow::bail!(
                "held-charge must be \"arrow\" or \"rocket\" (the CrossbowItem$ChargeType codec names), got {other:?}"
            ),
        };
        let wire = held_item
            // the CLI names a MODEL id ("item/crossbow") — the registry
            // names the ITEM ("crossbow"); the namespace strip routes it
            .and_then(|m| m.strip_prefix("item/").or(Some(m)))
            .and_then(mc_registry::item_id_by_name)
            .ok_or_else(|| anyhow::anyhow!("held-charge requires --held-item item/crossbow"))?;
        anyhow::ensure!(
            mc_registry::item_charge_def(wire).is_some(),
            "held-charge requires a charge-lane item in hand (the crossbow)"
        );
        app.held_charge = Some(kind);
        app.held_item = crate::hand::load_charge_item(wire, kind);
    }
    if let Some(p) = swing_progress {
        app.swing = crate::hand::SwingState::frozen_at(p);
    }
    match screen {
        "loading" => {
            app.screen = ScreenKind::Loading;
            for _ in 0..frame.min(19) {
                app.update();
            }
        }
        "title" => {
            app.screen = ScreenKind::Title;
            app.frame = frame;
        }
        "servers" => app.open(ScreenKind::Servers),
        "options" => app.open(ScreenKind::Options),
        "world" => {
            app.enter_world();
            app.frame = frame;
            if opts.net_day.is_some() || opts.net_mobs > 0 {
                // THE WIRE'S OWN LANE: the network world swaps in — the
                // server's Set Time opinion re-bases the day clock, the
                // wire mobs land through the REAL live_mobs_updated
                // entry (the lerpTo law the session's refresh rides),
                // the sky and the mobs render the SERVER'S word over
                // the SAME terrain (the re-mesh the swap speaks)
                let mut net = app
                    .offline_world
                    .take()
                    .unwrap_or_else(|| crate::world::World::new(16, 42));
                net.day_tick = opts.net_day;
                app.world_is_network = true;
                app.live_view_updated(net, None, None, None);
                if opts.net_mobs > 0 {
                    // THE MOB RING (the --zombie discipline's own
                    // facing point: the surface scan ahead of the
                    // body's camera — the mobs land IN VIEW, never a
                    // guessed cell behind the terrain): the wire's
                    // zombies face the camera, the head mid-look, the
                    // lerpTo walk armed to a half-cell drift (the ease
                    // the 250ms refresh speaks, frozen at its step)
                    let yaw_cam = opts.body_yaw.unwrap_or(-90.0);
                    let (fx, fz) = (
                        (-yaw_cam.to_radians().sin()) as f64,
                        yaw_cam.to_radians().cos() as f64,
                    );
                    let (b_x, b_z) = app
                        .body
                        .as_ref()
                        .map(|b| (b.x, b.z))
                        .unwrap_or((
                            app.world_size as f64 / 2.0,
                            app.world_size as f64 / 2.0,
                        ));
                    let mut wire_mobs = Vec::new();
                    for i in 0..opts.net_mobs {
                        // the staggered line the drops lane speaks:
                        // 2.0/3.5 cells ahead, the side offsets the
                        // Lcg's own draw (the deterministic spread)
                        let t = 1.8 + i as f64 * 1.5;
                        let side = if i % 2 == 0 { 0.4 } else { -0.4 };
                        let px = b_x + fx * t - fz * side;
                        let pz = b_z + fz * t + fx * side;
                        let w = app.net_world.as_ref().unwrap();
                        // the surface scan the damage lane speaks: DOWN
                        // from the body's own level (never the world
                        // top — the wall's roof would hide the mob)
                        let b_y = app.body.as_ref().map(|b| b.y as i32).unwrap_or(64);
                        let mut gy = b_y;
                        while gy > 0
                            && w.get(px.floor() as i32, gy, pz.floor() as i32)
                                == crate::world::AIR
                        {
                            gy -= 1;
                        }
                        let y = (gy + 1) as f64;
                        // facing the body (the vanilla yaw the
                        // atan2 fold speaks), the head mid-look
                        let face = (-((b_x - px)).atan2(b_z - pz)).to_degrees() as f32;
                        wire_mobs.push(crate::world_view::WireMob {
                            id: 100 + i as i32,
                            x: px,
                            y,
                            z: pz,
                            yaw: face,
                            pitch: 0.0,
                            head_yaw: face + 35.0, // the look mid-swing
                            type_name: "minecraft:zombie".to_string(),
                        });
                    }
                    app.live_mobs_updated(wire_mobs);
                    // the lerpTo walk's own first step: one tick of the
                    // half-cell drift the refresh cadence paces
                    app.tick_net_mobs();
                }
            }
        }
        "worldnight" => {
            // Night world view (tick 14040 = ~20:00 in-game): full moon ~25
            // degrees above the eastern horizon (in view of the east-facing
            // camera), stars at the official full brightness 0.5, night
            // sky/fog colors from the timeline.
            app.enter_world();
            app.day_time_base = 14010;
            app.frame = frame;
        }
        "pause" => {
            app.enter_world();
            app.paused = true;
            app.frame = frame;
        }
        "chest" => {
            app.open_chest();
            app.frame = frame;
        }
        other => anyhow::bail!(
            "unknown screen {other} (loading|title|servers|options|world|worldnight|pause|chest)"
        ),
    }

    // THE SHOTS LANE's gate (the seeded calls stage the world screens
    // only) and its injection points: the seeds land on the set screen
    // state, then the tick lane evolves it through the vanilla laws.
    if opts.is_seeded() {
        anyhow::ensure!(
            app.screen == ScreenKind::World,
            "the scene seeds stage the world screens only (world | worldnight | pause), got {screen:?}"
        );
    }
    seed_scene(&mut app, opts)?;
    tick_scene(&mut app, opts, swing_progress)?;
    // THE PARTIAL OVERRIDE (the --partial lane): the fixed clock's
    // remainder the staged frame speaks — clamped to the fold's [0, 1]
    // domain (the lerp's own walk, the vanilla partialTick shape)
    if let Some(p) = opts.partial {
        app.partial_tick = p.clamp(0.0, 1.0);
    }

    let mut fb = Framebuf::new(width, height);
    let mut assets = GuiAssets::open("assets-cache/jar");
    app.render(&mut fb, &mut assets);
    write_rgb_png(out_path, width, height, &fb.color)?;
    println!(
        "gui screen '{screen}' (frame {frame}, seed {seed}) -> {}",
        out_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default opts seed NOTHING — the goldens' scenes stay the
    /// exact states they rendered before the shots lane existed (the
    /// no-body, no-mob, first-person determinism).
    #[test]
    fn the_default_opts_seed_nothing() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(&mut app, &SceneOpts::default()).unwrap();
        tick_scene(&mut app, &SceneOpts::default(), None).unwrap();
        assert!(app.body.is_none());
        assert!(app.mobs.is_empty());
        assert!(app.perspective.is_first_person());
    }

    /// The body seed stands on the spawn column's surface facing east,
    /// the walk pair rides the store verbatim, the crouch flag folds
    /// the resolved pose (the POSES table's own body).
    #[test]
    fn the_body_seed_stands_on_the_spawn_column() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                body_walk: Some((0.9, 5.0)),
                body_crouch: true,
                perspective: Some(crate::camera::CameraType::ThirdPersonBack),
                ..SceneOpts::default()
            },
        )
        .unwrap();
        let body = app.body.expect("the body seed must land");
        assert!(body.on_ground);
        assert_eq!(body.yaw, -90.0); // east
        assert_eq!(body.y as f32, app.spawn_y as f32); // feet AT the surface
        assert_eq!(body.walk.speed, 0.9);
        assert_eq!(body.walk.position, 5.0);
        assert!(body.crouching);
        assert!(body.want_crouch);
        assert_eq!(app.perspective, crate::camera::CameraType::ThirdPersonBack);
    }

    /// The zombie seed walks toward the body (facing west, the
    /// approach's mid-stride), the pose/brand/clock flags ride the
    /// render state's own fields verbatim.
    #[test]
    fn the_zombie_seed_walks_toward_the_body() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                zombie: true,
                zombie_aggro: true,
                zombie_fire: Some(160),
                zombie_death: Some(8),
                zombie_hurt: Some(5),
                ..SceneOpts::default()
            },
        )
        .unwrap();
        assert_eq!(app.mobs.len(), 1);
        let z = &app.mobs[0];
        // the approach cell's own yaw (the scan names the facing)
        let facing = [90.0, -90.0, 180.0, 0.0];
        assert!(facing.contains(&z.yaw), "the zombie faces the body, got {}", z.yaw);
        assert!(z.on_ground);
        assert_eq!(z.walk.speed, 0.7);
        assert_eq!(z.walk.position, 2.5);
        assert!(z.aggressive);
        assert_eq!(z.fire_ticks, 160);
        assert_eq!(z.death_time, 8);
        assert_eq!(z.hurt_time, 5);
        // the zombie's feet stand on ITS column's surface at the
        // BODY's level (the walkable-approach scan's own law) — the
        // hilltop columns refuse
        assert_eq!(z.y as i32, app.spawn_y);
    }

    /// The walk seed's pair parser — the trims ride, the garbage
    /// refuses (a guessed pose would render a guessed body).
    #[test]
    fn the_walk_seed_parses_the_pair() {
        assert_eq!(parse_walk_seed("0.9,5.0"), Some((0.9, 5.0)));
        assert_eq!(parse_walk_seed(" 0.9 , 5.0 "), Some((0.9, 5.0)));
        assert_eq!(parse_walk_seed("0.9"), None);
        assert_eq!(parse_walk_seed("a,b"), None);
        assert_eq!(parse_walk_seed(""), None);
        assert_eq!(parse_walk_seed("0.9,5.0,2"), None);
    }

    /// The perspective name parser — the F5 cycle's own three states,
    /// the garbage refuses (a guessed state would render a guessed
    /// camera).
    #[test]
    fn the_perspective_names_parse_to_the_f5_states() {
        assert_eq!(
            parse_perspective("first").unwrap(),
            crate::camera::CameraType::FirstPerson
        );
        assert_eq!(
            parse_perspective("third-back").unwrap(),
            crate::camera::CameraType::ThirdPersonBack
        );
        assert_eq!(
            parse_perspective("third-front").unwrap(),
            crate::camera::CameraType::ThirdPersonFront
        );
        assert!(parse_perspective("back").is_err());
        assert!(parse_perspective("THIRD-BACK").is_err());
        assert!(parse_perspective("").is_err());
    }

    /// THE TICK LANE's walk: the seeded body travels its facing and
    /// the walk store evolves through the body's own update (the
    /// vanilla tick the windowed loop speaks), the frame clock riding
    /// one per tick.
    #[test]
    fn the_walk_ticks_travel_and_fill_the_walk_state() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                ..SceneOpts::default()
            },
        )
        .unwrap();
        let x0 = app.body.as_ref().unwrap().x;
        tick_scene(
            &mut app,
            &SceneOpts {
                walk_ticks: 12,
                ..SceneOpts::default()
            },
            None,
        )
        .unwrap();
        let body = app.body.as_ref().expect("the body survives the walk");
        assert!(
            body.x - x0 > 0.5,
            "the body traveled east (its facing): {} -> {}",
            x0,
            body.x
        );
        assert!(body.walk.speed > 0.05, "the walk store filled: {}", body.walk.speed);
        assert_eq!(app.frame, 12, "one frame per tick");
    }

    /// The tick lane refuses the menu screens (a silently ticked menu
    /// would render a guessed gallery).
    #[test]
    fn the_ticks_refuse_on_the_menu_screens() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        app.open(crate::screens::ScreenKind::Title);
        assert!(tick_scene(
            &mut app,
            &SceneOpts {
                walk_ticks: 1,
                ..SceneOpts::default()
            },
            None,
        )
        .is_err());
        // the static lane stays silent-zero — no ticks requested, no
        // refusal (the plain GUI screens ride it)
        assert!(tick_scene(&mut app, &SceneOpts::default(), None).is_ok());
    }

    /// The mobs lane's placement law: n zombies ahead of the body on
    /// SOLID, fluid-free ground (the spawn law's own scan), each
    /// within the approach band, the ids the counter's own block; the
    /// ignite fold speaks the max() law over every placed zombie.
    #[test]
    fn the_mobs_lane_places_ground_honest_zombies() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                mobs: 3,
                ignite_secs: 8.0,
                ..SceneOpts::default()
            },
        )
        .unwrap();
        assert_eq!(app.mobs.len(), 3, "every placement landed");
        let body = app.body.as_ref().unwrap();
        let (bx, bz) = (body.x, body.z);
        let world = app.offline_world.as_ref().unwrap();
        for z in &app.mobs {
            let (sx, sy, sz) = (z.x.floor() as i32, z.y as i32, z.z.floor() as i32);
            assert!(
                crate::body::solid(world, sx, sy - 1, sz),
                "the ground under the feet is solid"
            );
            assert_eq!(
                world.get(sx, sy, sz),
                crate::world::AIR,
                "the feet cell is air (the fluid gate held)"
            );
            let dist = ((z.x - bx) * (z.x - bx) + (z.z - bz) * (z.z - bz)).sqrt();
            assert!(
                (2.0..=(SHOWCASE_SEEK_BASE + SHOWCASE_SEEK_STRIDE * 3.0 + SHOWCASE_SEEK_EXTRA))
                    .contains(&dist),
                "the seek bound holds: {dist}"
            );
            assert_eq!(z.fire_ticks, 160, "the 8-second brand");
        }
        assert_eq!(app.next_mob_id, 4, "the id block consumed (the counter's own 1 + the 3 ids the block speaks)");
        // the mobs lane without the body refuses loudly
        let mut app2 = crate::screens::App::new(262);
        app2.enter_world();
        assert!(seed_scene(
            &mut app2,
            &SceneOpts {
                mobs: 1,
                ..SceneOpts::default()
            },
        )
        .is_err());
    }

    /// The frozen swing re-freezes AFTER the ticks (a real tick
    /// resumes the timeline — the freeze pins the render's slice).
    #[test]
    fn the_frozen_swing_survives_the_ticks() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                walk_ticks: 4,
                ..SceneOpts::default()
            },
        )
        .unwrap();
        tick_scene(
            &mut app,
            &SceneOpts {
                walk_ticks: 4,
                ..SceneOpts::default()
            },
            Some(0.5),
        )
        .unwrap();
        assert_eq!(app.frame, 4, "the ticks ran");
        assert_eq!(
            app.swing.get_attack_anim(0.5),
            0.5,
            "the freeze re-pinned after the ticks"
        );
    }

    /// THE DIVE SEED: the pool's 3x3 water column stands at the body,
    /// the feet on the forced-solid basin floor, the EYE one cell under
    /// the surface (the vanilla isEyeInFluid's own geometry: feet +
    /// 1.62) — the water-screen gate's honest scene.
    #[test]
    fn the_water_pool_sinks_the_eye_under_the_surface() {
        let mut app = crate::screens::App::new(262);
        app.enter_world();
        seed_scene(
            &mut app,
            &SceneOpts {
                body: true,
                water_pool: true,
                perspective: Some(crate::camera::CameraType::FirstPerson),
                ..SceneOpts::default()
            },
        )
        .unwrap();
        let body = app.body.as_ref().expect("the body seed must land");
        let world = app.offline_world.as_ref().unwrap();
        let (bx, by, bz) = (body.x.floor() as i32, body.y as i32, body.z.floor() as i32);
        assert_eq!(
            world.get(bx, by, bz),
            crate::world::WATER,
            "the feet's cell is water"
        );
        assert_eq!(
            world.get(bx, by + 1, bz),
            crate::world::WATER,
            "the eye's cell is water"
        );
        assert!(
            crate::body::solid(world, bx, by - 1, bz),
            "the basin floor is solid under the feet"
        );
        // the eye geometry: floor(by + 1.62) == by + 1, one cell under
        // the surface (the surface rides the cell above the eye's)
        let eye = body.y + body.eye_height();
        assert_eq!(eye.floor() as i32, by + 1, "the eye sits in the eye cell");
        // the first-person camera rides the eye INTO the water cell —
        // the gate reads it (the body_state's own query)
        let cam = app.world_camera();
        assert_eq!(
            world.get(
                cam.pos.x.floor() as i32,
                cam.pos.y.floor() as i32,
                cam.pos.z.floor() as i32
            ),
            crate::world::WATER,
            "the camera's own cell is water"
        );
        // the surface: the cell above the eye's carries the pool's top
        // water, the headroom above it cleared — no canopy roofs the pool
        let surface = by + 2;
        assert_eq!(
            world.get(bx, surface, bz),
            crate::world::WATER,
            "the surface cell is the pool's top water"
        );
        assert_eq!(
            world.get(bx, surface + 1, bz),
            crate::world::AIR,
            "the headroom above the surface is cleared"
        );
    }
}
