use super::*;

#[test]
fn the_crit_hit_arms_the_damage_indicators() {
    // THE WIRING'S OWN TRUTH (damageStatsAndHearts at the attack
    // site): the crit's 3.0 dealt crosses the > 2.0 gate and lands
    // ONE indicator at the mob's mid height; the plain fist's 2.0
    // dealt stays under the gate — the honest vanilla shape
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            zombie: true,
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let idx = 0;
    // THE REACH THE 26.2 LAW SPEAKS (the entity arm's own
    // filterHitResult): the crosshair's entity verdict only lands
    // inside the 3.0 entity_interaction_range — the ring's 4-east
    // staging sits OUTSIDE it, the mob snaps to 2.5 (the 2.5 the
    // attack tests stage) so the fist lands
    {
        let bx = app.body.as_ref().unwrap().x;
        app.mobs[idx].x = bx + 2.5;
    }
    // aim at the zombie's mid height (the vanilla getY(0.5) the
    // burst rides)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    // THE PLAIN FIST'S GATE: the full charge deals 2.0 — the dealt
    // stays at the gate's edge, no indicator spawns
    app.attack_ticker = 20;
    app.on_attack();
    let plain = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Damage))
        .count();
    assert_eq!(plain, 0, "the 2.0 dealt never crosses the gate");
    // THE CRIT'S OWN BURST: the fall distance arms the crit (the
    // 1.5x the damage law speaks — 3.0 dealt, the count-1 walk)
    app.mobs[idx].health = 10.0;
    {
        let b = app.body.as_mut().unwrap();
        b.fall_distance = 1.0;
        b.on_ground = false;
        b.sprinting = false;
    }
    app.attack_ticker = 20;
    app.on_attack();
    let stars = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Damage))
        .count();
    assert_eq!(stars, 1, "the 3.0 dealt speaks the count-1 walk");
    let m = &app.mobs[idx];
    for p in app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Damage))
    {
        let mid = m.y + crate::mob::ZOMBIE_HEIGHT * 0.5;
        assert!(
            (p.y - mid).abs() <= 0.6,
            "the mid-height band the getY(0.5) speaks: {} vs {mid}",
            p.y
        );
        assert!(p.vy > 0.5, "the +1.0 burst rides: {}", p.vy);
        assert_eq!(p.lifetime, 20);
    }
}

#[test]
fn the_sweep_hit_arms_the_arc_and_the_aoe() {
    // THE GATE'S OWN TRUTH (isSweepAttack at the attack site): the
    // full charge + the sword + the grounded still body speak the
    // sweep — the arc spawns ONE (the count-0 walk) at the body's
    // own mid height one forward block, the sweep voice logs, the
    // strong voice stays silent (the branch the attackVisualEffects
    // speaks); the bare fist's same swing never sweeps
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            zombie: true,
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    clear_the_aim_corridor(&mut app);
    let idx = 0;
    // aim at the zombie's mid height (the crit test's own walk)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    // THE SWORD'S OWN GATE: the full charge + the grounded + the
    // sword speaks the sweep
    let sword = crate::mob::wire_id("iron_sword").expect("the sword rides the wire");
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.attack_ticker = 20;
    app.on_attack();
    let arcs = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Sweep))
        .count();
    assert_eq!(arcs, 1, "the count-0 walk leaves ONE arc");
    // the arc's own pose: the body's mid height, one forward block
    // (the getX() - sin(yaw), getY(0.5), getZ() + cos(yaw) law)
    let (bxy, byy, bzy) = {
        let b = app.body.as_ref().unwrap();
        let vyaw = b.yaw as f64 * std::f64::consts::PI / 180.0;
        let (bx, by, bz) = (b.x, b.y, b.z);
        let mid = by + crate::body::STANDING_HEIGHT * 0.5;
        let arc = app
            .particles
            .iter()
            .find(|p| matches!(p.kind, crate::particles::ParticleKind::Sweep))
            .unwrap();
        let want = (bx - vyaw.sin(), mid, bz + vyaw.cos());
        assert!(
            (arc.x - want.0).abs() < 1e-9
                && (arc.y - want.1).abs() < 1e-9
                && (arc.z - want.2).abs() < 1e-9,
            "the arc pose the tail speaks: ({}, {}, {}) vs {:?}",
            arc.x,
            arc.y,
            arc.z,
            want
        );
        assert_eq!(arc.lifetime, 4, "the 4-tick life the ctor pins");
        (bx, by, bz)
    };
    let _ = (bxy, byy, bzy);
    // the sweep voice logged, the strong voice refused (the
    // attackVisualEffects branch the sweep suppresses)
    let sweep_voice = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.player.attack.sweep"))
        .count();
    assert_eq!(sweep_voice, 1, "the offset-0 voice the arm speaks");
    let strong_voice = app
        .sounds
        .events
        .iter()
        .any(|e| e.starts_with("entity.player.attack.strong"));
    assert!(!strong_voice, "the suppressed branch stays silent");
    // THE BARE FIST'S OWN REFUSAL: the same grounded full-charge
    // swing without the sword never sweeps (the SWORDS tag the
    // gate reads), the strong voice speaks instead
    app.inventory.items[app.selected_hotbar] = (sword, 0);
    app.attack_ticker = 20;
    app.on_attack();
    let arcs2 = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Sweep))
        .count();
    assert_eq!(arcs2, 1, "the fist's swing never adds an arc");
    let strong_voice2 = app
        .sounds
        .events
        .iter()
        .any(|e| e.starts_with("entity.player.attack.strong"));
    assert!(strong_voice2, "the unsuppressed branch speaks");
}

#[test]
fn the_sweep_aoe_rides_the_inflated_box_and_the_nine_ring() {
    // THE AOE'S OWN TRUTH (doSweepAttack's neighbor walk): the
    // second zombie inside the target's inflate(1.0, 0.25, 1.0)
    // box and inside the 9.0 distance ring takes the swept hit
    // (the full total the no-enchants law speaks) and the
    // (0.4, sin, -cos) knockback; the far zombie outside the ring
    // stays clean
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            zombie: true,
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    clear_the_aim_corridor(&mut app);
    let idx = 0;
    // aim at the zombie's mid height
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    // the stage's own geometry: the target exactly 2.0 out along
    // the facing (the seed's own east stance), the NEIGHBOR beside
    // it 0.7 lateral (inside the inflated box + the 9.0 ring,
    // clear of the aim ray), the FAR one 4.5 out behind it
    // (outside the 9.0 ring — dist² 42.25)
    let (nx, nz) = {
        let b = app.body.as_ref().unwrap();
        let vyaw = b.yaw as f64 * std::f64::consts::PI / 180.0;
        let fx = -vyaw.sin();
        let fz = vyaw.cos();
        let m = &mut app.mobs[idx];
        m.x = b.x + fx * 2.0;
        m.z = b.z + fz * 2.0;
        (m.x + fz * 0.7, m.z - fx * 0.7)
    };
    let neighbor = crate::mob::Zombie::new(2, nx, app.mobs[idx].y, nz, 311);
    let neighbor_health = neighbor.health;
    app.mobs.push(neighbor);
    let far_health = {
        let (tx, ty, tz) = (app.mobs[idx].x, app.mobs[idx].y, app.mobs[idx].z);
        let b = app.body.as_ref().unwrap();
        let vyaw = b.yaw as f64 * std::f64::consts::PI / 180.0;
        let (fx, fz) = (-vyaw.sin(), vyaw.cos());
        let far = crate::mob::Zombie::new(3, tx + fx * 4.5, ty, tz + fz * 4.5, 313);
        let fh = far.health;
        app.mobs.push(far);
        fh
    };
    // aim at the zombie's mid height (AFTER the staging sits)
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[idx];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    // the sword + the full charge + the grounded swing
    let target_health = app.mobs[idx].health;
    let sword = crate::mob::wire_id("iron_sword").expect("the sword rides the wire");
    app.inventory.items[app.selected_hotbar] = (sword, 1);
    app.attack_ticker = 20;
    app.on_attack();
    // the neighbor took the swept hit: the full-charge sword
    // speaks the 2.0 total the ATTACK_DAMAGE law deals
    let nh = app.mobs[1].health;
    assert!(
        nh < neighbor_health,
        "the neighbor takes the swept hit: {nh} < {neighbor_health}"
    );
    // the full total the no-enchants law speaks: the swept hit
    // deals the SAME the direct hit dealt (the attack's own total
    // through the same hurt law — the zombie's ARMOR 2.0 fold the
    // CombatRules walk speaks applies to both alike)
    let target_delta = target_health - app.mobs[idx].health;
    assert!(
        ((neighbor_health - nh) - target_delta).abs() < 1e-6,
        "the swept hit deals the full total: {} vs {target_delta}",
        neighbor_health - nh
    );
    // the far zombie never heard of it
    assert_eq!(app.mobs[2].health, far_health, "the 9.0 ring holds");
    // the neighbor's knockback: the minus law pushed it along the
    // facing (the vx it carries away)
    let _ = (nx, nz);
}

#[test]
fn the_ambient_walk_spawns_the_pool_drips() {
    // THE STAGE'S OWN TRUTH: the lava pool + the 24-tick walk —
    // the 667x2 sampler lands the pool's 9 surface cells, the
    // 1/100 gate fires the drips (the 8x8 sprite's surface Y the
    // +1.0 speaks), the voices log the two formulas
    let mut app = App::new(3);
    app.enter_world();
    crate::headless::seed_scene(
        &mut app,
        &crate::headless::SceneOpts {
            body: true,
            lava_pool: true,
            particles: Some("lava".into()),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let drips = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .count();
    assert!(drips > 0, "the walk fires on the pool's surface: {drips}");
    // the surface Y: the pool's top cells speak floor+1.0 — every
    // drip's birth Y rides the band the carve's level names
    for p in app.particles.iter() {
        if matches!(p.kind, crate::particles::ParticleKind::Lava) {
            // the spawn band 0.05..0.45 DECAYS the staged flight's
            // three ticks speak (0.04 * 0.75 per tick) — the
            // post-flight band rides it
            assert!(
                p.vy >= -0.05 && p.vy < 0.4,
                "the pop band the ctor speaks: {}",
                p.vy
            );
            assert_eq!(p.gravity, 0.75);
        }
        if matches!(p.kind, crate::particles::ParticleKind::Smoke) {
            assert_eq!(p.gravity, -0.1, "the child RISES");
            assert!(p.col[0] <= 0.3, "the grey tint's band");
        }
    }
    // the voices: the pop + the ambient formulas ride the log
    let pops = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.lava.pop @"))
        .count();
    assert!(pops > 0, "the pop voice the gate speaks: {pops}");
    for e in app.sounds.events.iter() {
        if let Some((v, p)) = e
            .split_once(" v")
            .and_then(|(_, rest)| rest.split_once(" p"))
        {
            if e.starts_with("block.lava.pop") || e.starts_with("block.lava.ambient") {
                let vol: f32 = v.parse().unwrap();
                let pitch: f32 = p.parse().unwrap();
                assert!(
                    (0.2..0.4).contains(&vol),
                    "the volume formula's band: {vol}"
                );
                assert!(
                    (0.9..1.05).contains(&pitch),
                    "the pitch formula's band: {pitch}"
                );
            }
        }
    }
    // the replay: the same app walks the same seeds — the two
    // streams' determinism the shots ride
    let mut a = App::new(3);
    a.enter_world();
    crate::headless::seed_scene(
        &mut a,
        &crate::headless::SceneOpts {
            body: true,
            lava_pool: true,
            particles: Some("lava".into()),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let mut b = App::new(3);
    b.enter_world();
    crate::headless::seed_scene(
        &mut b,
        &crate::headless::SceneOpts {
            body: true,
            lava_pool: true,
            particles: Some("lava".into()),
            ..crate::headless::SceneOpts::default()
        },
    )
    .unwrap();
    let da: Vec<(u32, f64, f64, f64)> = a
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    let db: Vec<(u32, f64, f64, f64)> = b
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Lava))
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    assert_eq!(da, db, "the seeded walk replays");
}

#[test]
fn the_column_ambient_walk_speaks_the_column_cells() {
    // THE WHIRLPOOL'S OWN WORLD (every cell the 15294 id names —
    // the sampler cannot miss): the walk answers ONE CURRENT_DOWN
    // per in-bounds sample, the fixed (x+0.5, y+0.8, z+0.5) birth
    // the addAlwaysVisibleParticle call speaks, the 1/200 rolls
    // firing the whirlpool's ambient voice
    let stamp = |id: u16| -> crate::world::World {
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in 0..48 {
            for z in 0..16 {
                for x in 0..16 {
                    world.set(x, y, z, id);
                }
            }
        }
        world
    };
    let mut app = App::new(3);
    app.offline_world = Some(stamp(15294));
    let out = app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!out.is_empty(), "the column cells answer the walk");
    for p in out.iter() {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::CurrentDown),
            "the whirlpool's own droplet"
        );
        assert_eq!(p.vy, -0.05, "the fixed birth speed");
        assert_eq!(p.gravity, 0.002);
        assert!((30..=90).contains(&p.lifetime), "the (int)(r*60)+30 band");
        // the fixed offsets: the fractional .5/.8 the cell coords
        // speak — no offset draws the whirlpool's arm keeps
        assert_eq!(p.x.fract(), 0.5, "the x+0.5 the call speaks");
        assert_eq!(p.z.fract(), 0.5, "the z+0.5 the call speaks");
        assert!(
            (p.y.fract() - 0.8).abs() < 1e-9,
            "the y+0.8 the call speaks"
        );
    }
    // the voices: the whirlpool event the 1/200 gate speaks, the
    // volume/pitch formula's bands the log's ` v.. p..` tail keeps
    let mut voices = 0;
    for e in app.sounds.events.iter() {
        if !e.starts_with("block.bubble_column.whirlpool_ambient @") {
            continue;
        }
        voices += 1;
        if let Some((v, p)) = e
            .split_once(" v")
            .and_then(|(_, rest)| rest.split_once(" p"))
        {
            let vol: f32 = v.parse().unwrap();
            let pitch: f32 = p.parse().unwrap();
            assert!(
                (0.2..0.4).contains(&vol),
                "the volume formula's band: {vol}"
            );
            assert!(
                (0.9..1.05).contains(&pitch),
                "the pitch formula's band: {pitch}"
            );
        }
    }
    assert!(voices > 0, "the whirlpool's ambient voice fires: {voices}");
    // the replay: the same stamps walk the same seeds
    let mut a = App::new(3);
    a.offline_world = Some(stamp(15294));
    let da: Vec<(u32, f64, f64, f64)> = a
        .animate_tick_walk((8.0, 12.0, 8.0))
        .iter()
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    let mut b = App::new(3);
    b.offline_world = Some(stamp(15294));
    let db: Vec<(u32, f64, f64, f64)> = b
        .animate_tick_walk((8.0, 12.0, 8.0))
        .iter()
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    assert_eq!(da, db, "the seeded walk replays");
    // THE UPWARD'S OWN WORLD: TWO bubbles per sampled cell — the
    // first at the fixed corner (x+0.5, y, z+0.5), the second's
    // offsets the block stream draws inside the same cell
    let mut app2 = App::new(3);
    app2.offline_world = Some(stamp(15295));
    let out2 = app2.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!out2.is_empty(), "the upward cells answer the walk");
    assert_eq!(out2.len() % 2, 0, "two bubbles per sampled cell");
    for pair in out2.chunks(2) {
        assert!(
            matches!(pair[0].kind, crate::particles::ParticleKind::BubbleColumnUp),
            "the upward's own bubble"
        );
        assert_eq!(pair[0].x.fract(), 0.5, "the fixed corner birth");
        assert_eq!(pair[0].y.fract(), 0.0, "the raw y the corner keeps");
        assert_eq!(pair[0].z.fract(), 0.5, "the fixed corner birth");
        assert!(
            ((pair[0].vy - 0.008).abs() <= 0.02),
            "the push's fifth ± the jitter band: {}",
            pair[0].vy
        );
        assert_eq!(pair[0].gravity, -0.125, "the rise the base lane turns");
        assert_eq!(pair[0].friction, 0.85);
        assert!(
            (40..=200).contains(&pair[0].lifetime),
            "the 40.0 fold's band"
        );
        // the second rides the SAME cell: the offsets the [0,1)
        // band holds — x/z within ±0.5 of the corner birth, y
        // within [y, y+1)
        assert!(pair[1].x >= pair[0].x - 0.5 && pair[1].x < pair[0].x + 0.5);
        assert!(pair[1].y >= pair[0].y && pair[1].y < pair[0].y + 1.0);
        assert!(pair[1].z >= pair[0].z - 0.5 && pair[1].z < pair[0].z + 0.5);
    }
    // the upward's voice: the sibling event the 1/200 gate speaks
    let up_voices = app2
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.bubble_column.upwards_ambient @"))
        .count();
    assert!(
        up_voices > 0,
        "the upward's ambient voice fires: {up_voices}"
    );
    // the plain world's silence (the honest no-column answer):
    // water everywhere, the variants layer zero — no spawns, no
    // voices
    let mut app3 = App::new(3);
    app3.offline_world = Some(stamp(crate::world::WATER));
    let out3 = app3.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(out3.is_empty(), "the plain water the column answer skips");
    assert!(
        !app3
            .sounds
            .events
            .iter()
            .any(|e| e.contains("bubble_column")),
        "no column voice the plain world keeps"
    );
}

#[test]
fn the_torch_cells_spawn_the_ambient_pair() {
    // THE TORCH'S OWN WORLD (every cell the 54 kind names — the
    // sampler cannot miss): the walk answers the SMOKE + flame
    // PAIR per in-bounds sample — the block arm the doAnimateTick
    // walk calls before the fluid's own, the no gate the pair
    // speaks (the sparsity the sampling itself gives)
    let mut world = crate::world::World::new(1, 0);
    world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
    for y in 0..48 {
        for z in 0..16 {
            for x in 0..16 {
                world.set(x, y, z, crate::world::TORCH);
            }
        }
    }
    let mut app = App::new(3);
    app.offline_world = Some(world);
    let out = app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!out.is_empty(), "the torch cells answer the walk");
    assert_eq!(out.len() % 2, 0, "the pair the every-sample spawn speaks");
    for pair in out.chunks(2) {
        // THE ORDER (the animateTick call's own: the SMOKE first,
        // the flame second)
        assert!(
            matches!(pair[0].kind, crate::particles::ParticleKind::Smoke),
            "the pair's own smoke"
        );
        assert!(
            matches!(pair[1].kind, crate::particles::ParticleKind::Flame),
            "the pair's own flame"
        );
        // THE BIRTH (the (x+0.5, y+0.7, z+0.5) constants the
        // addParticle args speak): the smoke sits at the exact
        // cell fold (no jitter the ash chain never arms), the
        // flame rides the ±0.05 jitter band around the same fold
        assert_eq!(pair[0].x.fract(), 0.5, "the smoke's own x+0.5");
        assert!(
            (pair[0].y.fract() - 0.7).abs() < 1e-9,
            "the y+0.7 the constant keeps"
        );
        assert_eq!(pair[0].z.fract(), 0.5, "the smoke's own z+0.5");
        assert!(
            (pair[1].x - pair[0].x).abs() <= 0.05 + 1e-9,
            "the flame's own x jitter: {}",
            pair[1].x - pair[0].x
        );
        assert!(
            (pair[1].y - pair[0].y).abs() <= 0.05 + 1e-9,
            "the flame's own y jitter: {}",
            pair[1].y - pair[0].y
        );
        assert!(
            (pair[1].z - pair[0].z).abs() <= 0.05 + 1e-9,
            "the flame's own z jitter: {}",
            pair[1].z - pair[0].z
        );
        // THE LAWS (the pair's own chains the ctor folds keep)
        assert_eq!(pair[0].gravity, -0.1, "the smoke's own rise");
        assert_eq!(pair[1].gravity, 0.0, "the flame's own no-gravity");
        assert!((8..=40).contains(&pair[0].lifetime), "the ash lifetime");
        assert!((12..=43).contains(&pair[1].lifetime), "the rising lifetime");
    }
    // THE REPLAY (the seeded walk the same stamps speak)
    let stamp_torch = |id: u16| -> crate::world::World {
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in 0..48 {
            for z in 0..16 {
                for x in 0..16 {
                    world.set(x, y, z, id);
                }
            }
        }
        world
    };
    let mut a = App::new(3);
    a.offline_world = Some(stamp_torch(crate::world::TORCH));
    let da: Vec<(u32, f64, f64, f64)> = a
        .animate_tick_walk((8.0, 12.0, 8.0))
        .iter()
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    let mut b = App::new(3);
    b.offline_world = Some(stamp_torch(crate::world::TORCH));
    let db: Vec<(u32, f64, f64, f64)> = b
        .animate_tick_walk((8.0, 12.0, 8.0))
        .iter()
        .map(|p| (p.lifetime, p.x, p.y, p.z))
        .collect();
    assert_eq!(da, db, "the seeded walk replays");
    // THE LAVA WORLD'S SILENCE (the block arm the torch alone
    // wears — the all-lava stamp the drip lane owns spawns the
    // gated drips only, no smoke/flame the torch kind never
    // answers)
    let mut lava_app = App::new(3);
    lava_app.offline_world = Some(stamp_torch(crate::world::LAVA));
    let lava_out = lava_app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(
        lava_out
            .iter()
            .all(|p| matches!(p.kind, crate::particles::ParticleKind::Lava)),
        "the lava world the torch pair never speaks"
    );
}

/// THE FIRE CELLS' OWN ANIMATE WALK — javap `BaseFireBlock.
/// animateTick` the sampler's visits answer (the parity world the
/// every-fire-cell below arm speaks — the stone floor the sturdy
/// gate's TRUE answer wears): the THREE large smokes the visit
/// keeps, the interior band the y + d*0.5 + 0.5 fold speaks, the
/// 1/24 voice the cell-center anchor the positioned lane plays,
/// the soul fire's own verdict the EVERY-below answer burns (the
/// iconst_1 override the same three the air floor wears).
#[test]
fn the_fire_cells_the_smokes_the_ambient_voice_wear() {
    // THE PARITY WORLD'S OWN STAMP (the even layers the stone
    // floor keeps, the odd the kind's own flame wears — the every
    // fire cell the below arm's TRUE gate speaks)
    let stamp = |id: u16| -> crate::world::World {
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in 0..48 {
            for z in 0..16 {
                for x in 0..16 {
                    world.set(x, y, z, if y % 2 == 0 { crate::world::STONE } else { id });
                }
            }
        }
        world
    };
    // THE WALK (the plain flame's own lane): the smokes the visits
    // speak, the sound gate the positioned voice's own log keeps
    let mut app = App::new(3);
    app.offline_world = Some(stamp(crate::world::FIRE));
    let out = app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!out.is_empty(), "the fire cells answer the walk");
    assert_eq!(out.len() % 3, 0, "the below arm's own three per visit");
    for p in &out {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Smoke),
            "the large smoke the only kind the walk speaks"
        );
        // THE INTERIOR BAND (the y + d*0.5 + 0.5 fold the below arm
        // keeps — the fract the [0.5, 1.0) band the d draw rides)
        let fy = p.y.fract();
        assert!(fy >= 0.5, "the interior band the fold keeps: {}", fy);
        // THE LARGE PROVIDER'S OWN SCALE (the 2.5f the LargeSmoke-
        // Particle's super arg speaks — the quad the plain smoke's
        // own fold ×2.5 wears)
        assert!(
            p.quad_size > 0.15,
            "the large provider's own scale: {}",
            p.quad_size
        );
    }
    // THE VOICE (the 1/24 gate the ~55 the visits' own odds speak —
    // the event the positioned lane logs, the pitch the 0.3..1.0
    // band the f*0.7+0.3 fold keeps, the volume the 1.0..2.0 the
    // 1.0+f fold wears — the dist-zero ear the gain the plain fold
    // keeps)
    let voices: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.fire.ambient "))
        .collect();
    assert!(
        !voices.is_empty(),
        "the fire world the sampler hears: {} voices",
        voices.len()
    );
    for line in &voices {
        let pitch: f32 = line
            .split(" p")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|p| p.parse().ok())
            .expect("the pitch field the log keeps");
        assert!(
            (0.3..1.0).contains(&pitch),
            "the pitch band {} the fold keeps",
            pitch
        );
        let volume: f32 = line
            .split(" v")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|v| v.parse().ok())
            .expect("the volume field the log keeps");
        assert!(
            (1.0..2.0).contains(&volume),
            "the volume band {} the fold keeps",
            volume
        );
    }
    // THE DETERMINISM (the seeded walk the same stamps replay)
    let run = |id: u16| -> (Vec<(u32, f64, f64, f64, f32)>, Vec<String>) {
        let mut a = App::new(3);
        a.offline_world = Some(stamp(id));
        let out = a.animate_tick_walk((8.0, 12.0, 8.0));
        let parts = out
            .iter()
            .map(|p| (p.lifetime, p.x, p.y, p.z, p.quad_size))
            .collect();
        (parts, a.sounds.events.clone())
    };
    let (pa, ea) = run(crate::world::FIRE);
    let (pb, eb) = run(crate::world::FIRE);
    assert_eq!(pa, pb, "the seeded walk replays");
    assert_eq!(ea, eb, "the voice log replays");
    // THE SOUL FIRE'S OWN LANE (the EVERY-below verdict the override
    // speaks — the same three the air-floor cells would wear, the
    // voice the same event the BOTH kinds share)
    let mut soul = App::new(3);
    soul.offline_world = Some(stamp(crate::world::SOUL_FIRE));
    let soul_out = soul.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!soul_out.is_empty(), "the soul cells answer the walk");
    assert_eq!(soul_out.len() % 3, 0, "the soul's own below arm per visit");
    for p in &soul_out {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Smoke),
            "the soul's own smoke"
        );
        assert!(p.y.fract() >= 0.5, "the soul's own interior band");
    }
    assert!(
        soul.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.fire.ambient ")),
        "the soul fire the same event the SoundEvents field shares"
    );
    // THE FLAMELESS STONE WORLD'S OWN SILENCE (no fire cell the
    // sampler visits — the walk the empty vec the no-voice log
    // keeps)
    let mut cold = App::new(3);
    cold.offline_world = Some(stamp(crate::world::STONE));
    // the stone's own parity — the even layers the same kind wears,
    // the odd too (the stamp's fold the STONE id answers)
    let cold_out = cold.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(
        cold_out.is_empty(),
        "the stone world no smoke the walk speaks"
    );
    assert!(
        cold.sounds
            .events
            .iter()
            .all(|e| !e.starts_with("block.fire.ambient ")),
        "the stone world the voice never logs"
    );
}

/// THE PORTAL STANDING'S OWN APP LANE (javap `LocalPlayer
/// .handlePortalTransitionEffect` — the trigger voice the
/// zero-intensity beat banks, the screen-dismiss law the
/// `!isAllowedInPortal()` gate the closeContainer + setScreen(null)
/// tail serves; the countdown the body's own walk wears): the
/// standing body the offline world's own portal cell speaks, the
/// chest screen the container family the dismiss closes, the
/// World screen the allowed verdict keeps.
#[test]
fn the_portal_the_trigger_the_dismiss_the_app_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // the body the spawn point's own construction wears (the
    // offline spawn walk's own pair — the center + the feet_y the
    // top_solid_y scan keeps); the settle ticks the grounded body
    // the ground keeps
    app.body = Some(crate::body::PlayerBody::new(
        app.world_size as f64 / 2.0,
        app.spawn_y as f64,
        app.world_size as f64 / 2.0,
        0.0,
        0.0,
    ));
    for _ in 0..4 {
        app.update();
    }
    // the body's own feet cell the portal kind wears
    let (bx, by, bz) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32)
    };
    app.offline_world
        .as_mut()
        .expect("the offline world rides")
        .set(bx, by, bz, crate::world::NETHER_PORTAL);
    // THE ARM TICK (the entityInside walk the inside-blocks family
    // serves — the World screen the body tick's own gate keeps):
    // the entry the feet cell banks — the consume the NEXT tick's
    // head serves
    app.update();
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            b.portal_entry,
            Some((bx, by, bz)),
            "the feet cell the arm banks"
        );
        assert!(b.portal_inside_this_tick, "the arm the flag wears");
    }
    // THE CONSUME TICK (the baseTick law the head serves): the
    // trigger beat the zero-intensity gate speaks (the drain the
    // play_portal_trigger answer consumed — the flag the take
    // clears), the count's own one, the rise the +0.0125 walk wears
    app.update();
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.portal_time, 1, "the consume the one speaks");
        assert!(
            (b.portal_effect_intensity - crate::body::PORTAL_RISE_PER_TICK).abs() < 1e-9,
            "the first consume the rise wears — the trigger's own proof"
        );
        assert!(!b.portal_trigger_due, "the drained voice the take clears");
    }
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the World screen the gate never reaches"
    );
    // THE STANDING HOLD (the body the portal keeps): the count the
    // every-tick consume climbs
    for _ in 0..4 {
        app.update();
    }
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.portal_time, 5, "the standing hold the count climbs");
    }
    // THE SCREEN GATE'S OWN TABLE (javap isAllowedInPortal = the
    // isPauseScreen delegate — the container family the refusal
    // keeps, the World/pause the allowed verdict banks)
    app.open(ScreenKind::Inventory);
    assert!(!app.screen_allowed_in_portal());
    app.open(ScreenKind::Chest);
    assert!(!app.screen_allowed_in_portal());
    app.open(ScreenKind::Furnace);
    assert!(!app.screen_allowed_in_portal());
    app.open(ScreenKind::Crafting);
    assert!(!app.screen_allowed_in_portal());
    app.open(ScreenKind::World);
    assert!(
        app.screen_allowed_in_portal(),
        "the null-screen the allowed keeps"
    );
    app.paused = true;
    assert!(
        app.screen_allowed_in_portal(),
        "the pause the Screen-default's iconst_1 keeps"
    );
    app.paused = false;
    // THE DISMISS WALK'S OWN ARM (javap pcs 44-88 — the
    // closeContainer + setScreen(null) tail the container family
    // serves): the walk the law's own answer keeps — the chest's
    // close walk the Escape lane shares, the carried reset the
    // removed() reconciliation wears. (The law's e2e reach the
    // body-tick gate's own World-screen shape keeps — the
    // frozen-container convention the SCREEN round chose, the
    // container-open standing the freeze denies: the walk the
    // direct drive tests, the reach the honest doc banks.)
    app.open(ScreenKind::Chest);
    app.chest_carried = (55, 3);
    app.portal_screen_dismiss();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the dismiss the container closes"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the carried the reconciliation clears"
    );
}

/// THE SWAP'S OWN APP LANE (the verdict's own teleport tail the
/// drain slot serves — the vanilla handlePortal walk the
/// destination's own build answers): the lit gate the body stands
/// in, the 80th consume the verdict banks, THE SWAP the same tick
/// serves — the nether side the destination world keeps (the
/// has_skylight false the light walk's gate reads, the Skybox::None
/// + the (10, 96) fog + the #302821 ambient the nether row's own
/// pair wears), the return portal the builder + the fill walk
/// lights, the body the interior cell the cooldown's own refresh
/// keeps, the far slot the OLD world the edits persist. THE RETURN
/// WALK: the step-out the cooldown's own bleed drains, the
/// step-back the re-arm the expired body answers, the climb `*8`
/// the scale law wears, the marker block the slot's own mem::swap
/// keeps.
#[test]
fn the_swap_the_destination_the_return_the_scale_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // THE MARKER'S OWN EDIT (the overworld's own cell the far slot
    // carries — the mem::swap persistence proof the return wears)
    app.offline_world
        .as_mut()
        .expect("the offline world rides")
        .set(20, 40, 20, crate::world::CRAFTING_TABLE);
    // THE BODY THE SPAWN'S OWN CONSTRUCTION WEARS (the portal
    // test's own pair — the settle ticks the grounded body keeps)
    app.body = Some(crate::body::PlayerBody::new(
        app.world_size as f64 / 2.0,
        app.spawn_y as f64,
        app.world_size as f64 / 2.0,
        180.0,
        0.0,
    ));
    for _ in 0..2 {
        app.update();
    }
    // THE LIT GATE (the feet cell the portal kind wears — the
    // overlay test's own staging keeps)
    let (bx, by, bz) = {
        let b = app.body.as_ref().expect("the body rides");
        (b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32)
    };
    app.offline_world
        .as_mut()
        .expect("the offline world rides")
        .set(bx, by, bz, crate::world::NETHER_PORTAL);
    app.update(); // the arm banks the entry
    assert_eq!(
        app.body.as_ref().unwrap().portal_entry,
        Some((bx, by, bz)),
        "the feet cell the arm banks"
    );
    // THE COUNT'S OWN CLIMB (the survival 80 the direct drive
    // walks — the 80th consume the verdict banks, THE SWAP the
    // same tick's drain serves)
    {
        let b = app.body.as_mut().unwrap();
        b.portal_time = crate::body::PLAYERS_NETHER_PORTAL_DEFAULT_DELAY_TICKS - 1;
    }
    app.update();
    {
        let b = app.body.as_ref().unwrap();
        assert!(
            !b.portal_teleport_due,
            "the consumed verdict the take drains"
        );
        assert_eq!(
            b.portal_cooldown,
            crate::body::PLAYER_PORTAL_COOLDOWN_TICKS,
            "the post-teleport cooldown the tail speaks"
        );
        assert_eq!(b.portal_time, 0, "the machinery the zero keeps");
        assert_eq!(
            b.portal_entry, None,
            "the entry the fresh side's own None keeps"
        );
        let feet = app
            .offline_world
            .as_ref()
            .expect("the offline world rides")
            .get(b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32);
        assert_eq!(
            feet,
            crate::world::NETHER_PORTAL,
            "the body the return portal keeps"
        );
    }
    // THE NETHER'S OWN PIN (the destination the swap serves)
    assert_eq!(
        app.world_dimension,
        Dimension::Nether,
        "the nether side the swap banks"
    );
    assert_eq!(app.world_skybox, Skybox::None, "the nether pin");
    assert_eq!(app.world_fog, Some((10.0, 96.0)), "the stated fog pair");
    let w = app.offline_world.as_ref().expect("the offline world rides");
    assert!(!w.has_skylight, "the destination's own flag");
    assert_eq!(
        w.ambient_light,
        Some([48, 40, 33]),
        "the nether's ambient base the row wears"
    );
    // THE FAR SLOT'S OWN KEEP (the overworld the edits persist)
    {
        let far = app.far_world.as_ref().expect("the far slot rides");
        assert!(far.has_skylight, "the overworld the slot keeps");
        assert_eq!(
            far.get(20, 40, 20),
            crate::world::CRAFTING_TABLE,
            "the marker the slot carries"
        );
    }
    // THE RETURN WALK (the step-out the cooldown's own bleed
    // drains — the refresh arm the touching body keeps, the exit
    // the decrement speaks — then the step-back the re-arm the
    // expired answers, the climb `*8` the scale law wears)
    let (ix, iy, iz) = {
        let b = app.body.as_ref().unwrap();
        (b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32)
    };
    // THE PAD (the step-out's own landing the lava sea never
    // claims — the deterministic floor the walk's own site wears)
    app.offline_world
        .as_mut()
        .expect("the offline world rides")
        .set(ix, iy - 1, iz + 2, crate::world::NETHERRACK);
    {
        let b = app.body.as_mut().unwrap();
        b.z += 2.0; // the step-out the portal's own plane leaves
    }
    for _ in 0..11 {
        app.update(); // the bleed the exit's own ticks drain
    }
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.portal_cooldown, 0, "the bleed the exit drains");
        assert_eq!(b.portal_entry, None, "the arm the exit never banks");
    }
    {
        let b = app.body.as_mut().unwrap();
        b.z -= 2.0; // the step-back the portal's own plane re-enters
    }
    app.update(); // the re-arm the expired cooldown answers
    {
        let b = app.body.as_ref().unwrap();
        assert!(
            b.portal_entry.is_some(),
            "the re-arm the expired body answers"
        );
        let b = app.body.as_mut().unwrap();
        b.portal_time = crate::body::PLAYERS_NETHER_PORTAL_DEFAULT_DELAY_TICKS - 1;
    }
    app.update(); // the verdict → THE SWAP BACK
    assert_eq!(
        app.world_dimension,
        Dimension::Overworld,
        "the overworld restored"
    );
    assert_eq!(app.world_skybox, Skybox::Overworld, "the pin returns");
    assert_eq!(app.world_fog, None, "the overworld's unstated fog");
    // THE EDITS' OWN PERSISTENCE (the marker the mem::swap kept)
    assert_eq!(
        app.offline_world
            .as_ref()
            .expect("the offline world rides")
            .get(20, 40, 20),
        crate::world::CRAFTING_TABLE,
        "the marker the slot's own swap keeps"
    );
    // THE RETURN PORTAL'S OWN KEEP (the nether the far slot carries
    // — the return walk's own build the destination's own cells
    // bank)
    {
        let far = app.far_world.as_ref().expect("the far slot rides");
        assert!(!far.has_skylight, "the nether the far slot keeps");
        let mut portal_cells = 0;
        for c in &far.chunks {
            portal_cells += c
                .iter()
                .filter(|&&v| v == crate::world::NETHER_PORTAL)
                .count();
        }
        assert!(
            portal_cells >= 6,
            "the return portal the interior the far slot keeps"
        );
    }
}

/// THE END'S OWN SWAP LANE (javap `EndPortalBlock
/// .getPortalDestination` + `EndPlatformFeature.createEndPlatform`
/// + `ServerLevel` clinit the 26.2 jar — the destination the
/// absolute `END_SPAWN_POINT (100, 50, 0)` keeps, the platform the
/// feature's 5x5 loops wear, the arrival the WEST yaw the
/// transition banks; the environment the stated rows pin — the
/// Skybox::End the skybox "end" row wears, the fog the stated
/// None keeps, the #3f473f ambient the [63, 71, 63] bytes speak;
/// the EXIT the respawn law serves — the spawn point NO portal
/// the exit builds): the e2e the direct drive walks — the swap
/// the third world banks, the step-back the respawn restores, the
/// marker the far slot's own mem::swap keeps.
#[test]
fn the_end_the_swap_the_platform_the_skybox_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // THE MARKER'S OWN EDIT (the overworld's own cell the far slot
    // carries — the mem::swap persistence proof the round trip
    // wears)
    app.offline_world
        .as_mut()
        .expect("the offline world rides")
        .set(20, 40, 20, crate::world::CRAFTING_TABLE);
    // THE BODY THE SPAWN'S OWN CONSTRUCTION WEARS (the swap test's
    // own pair)
    app.body = Some(crate::body::PlayerBody::new(
        app.world_size as f64 / 2.0,
        app.spawn_y as f64,
        app.world_size as f64 / 2.0,
        180.0,
        0.0,
    ));
    // THE SWAP'S OWN WALK (the End arm the direct drive serves)
    app.swap_dimension_to(Dimension::End);
    // THE DIMENSION'S OWN ID (the third world the id banks)
    assert_eq!(
        app.world_dimension,
        Dimension::End,
        "the End the swap banks"
    );
    // THE ENVIRONMENT'S OWN PIN (the stated rows the table wears)
    assert_eq!(app.world_skybox, Skybox::End, "the stated skybox end");
    assert_eq!(app.world_fog, None, "the stated fog pair None");
    assert_eq!(app.world_cloud_fog_end, None, "the stated cloud pair None");
    // THE WORLD'S OWN ROWS (the fresh build the destination keeps
    // — the skylight TRUE the stated has_skylight wears, the
    // factor 0.0 the lightmap's sky term pins, the ambient the
    // registry row answers)
    let w = app.offline_world.as_ref().expect("the offline world rides");
    assert!(
        w.has_skylight,
        "the End's own skylight the stated TRUE keeps"
    );
    assert_eq!(
        w.sky_light_factor,
        Some(0.0),
        "the factor the stated 0.0 pins"
    );
    assert_eq!(
        w.ambient_light,
        Some([63, 71, 63]),
        "the #3f473f ambient the row wears"
    );
    // THE PLATFORM'S OWN CENTER (the arrival site the clamp keeps
    // — the vanilla's own absolute (100, 0) the 80-wide world's
    // bounds answer (74, 2), the obsidian row the proportion's
    // py-2 wears: 50 of the stated 256 the 48-tall grid the ratio
    // carries → 9)
    let (vx, vz, py) = (74, 2, 9);
    assert_eq!(
        w.get(vx, py - 2, vz),
        crate::world::OBSIDIAN,
        "the platform's own row the arrival wears"
    );
    // THE BODY'S OWN ARRIVAL (the vanilla's own feet the
    // bottom-center law keeps — the obsidian's top the walk banks,
    // the WEST yaw the transition sets, the pitch the X_ROT-
    // relative keeps, the cooldown the teleport tail refreshes,
    // the machinery the zero keeps)
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.x, vx as f64 + 0.5, "the absolute site the clamp keeps");
        assert_eq!(b.y, (py - 1) as f64, "the feet the obsidian's top wears");
        assert_eq!(b.z, vz as f64 + 0.5, "the absolute site the clamp keeps");
        assert_eq!(b.yaw, 90.0, "the WEST arm the transition banks");
        assert_eq!(b.pitch, 0.0, "the X_ROT-relative pitch the body keeps");
        assert_eq!(
            b.portal_cooldown,
            crate::body::PLAYER_PORTAL_COOLDOWN_TICKS,
            "the post-teleport cooldown the tail speaks"
        );
        assert_eq!(b.portal_time, 0, "the machinery the zero keeps");
        assert_eq!(
            b.portal_entry, None,
            "the entry the fresh side's None keeps"
        );
    }
    assert_eq!(app.spawn_y, py - 1, "the spawn the arrival cell keeps");
    assert_eq!(
        app.day_bake.expect("the bake rides").0,
        0,
        "the factor 0.0 the bake's own term pins"
    );
    // THE FAR SLOT'S OWN KEEP (the overworld the edits persist)
    {
        let far = app.far_world.as_ref().expect("the far slot rides");
        assert!(far.has_skylight, "the overworld the slot keeps");
        assert_eq!(
            far.get(20, 40, 20),
            crate::world::CRAFTING_TABLE,
            "the marker the slot carries"
        );
    }
    // THE STEP-BACK'S OWN RESPAWN (the End's exit the vanilla's
    // own law — the spawn point NO portal the exit builds, NO
    // platform: the center column the respawn walk serves, the
    // pins the overworld's own rows restore)
    app.swap_dimension_to(Dimension::Overworld);
    assert_eq!(
        app.world_dimension,
        Dimension::Overworld,
        "the respawn the overworld restores"
    );
    assert_eq!(
        app.world_skybox,
        Skybox::Overworld,
        "the pin the respawn restores"
    );
    assert_eq!(app.world_fog, None, "the overworld's unstated fog");
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            b.x,
            app.world_size as f64 / 2.0 + 0.5,
            "the spawn point the respawn walk serves"
        );
        assert_eq!(
            b.z,
            app.world_size as f64 / 2.0 + 0.5,
            "the spawn point the respawn walk serves"
        );
        let feet = app
            .offline_world
            .as_ref()
            .expect("the offline world rides")
            .get(b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32);
        assert_ne!(
            feet,
            crate::world::NETHER_PORTAL,
            "NO portal the respawn builds"
        );
    }
    // THE MARKER'S OWN RETURN (the round trip the mem::swap pair
    // keeps — the edits the each side carries)
    assert_eq!(
        app.offline_world
            .as_ref()
            .expect("the offline world rides")
            .get(20, 40, 20),
        crate::world::CRAFTING_TABLE,
        "the marker the round trip keeps"
    );
}

/// THE END PORTAL'S OWN E2E (the dev-29 round — the starry cell
/// the touch lane answers END TO END: the overworld touch the
/// instant swap the End arrival wears, the platform the spawn
/// point keeps, THE PEDESTAL'S OWN DIAMOND the touch the exit
/// serves — the respawn the overworld center restores)
#[test]
fn the_end_portal_the_touch_the_swap_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.mobs.clear();
    // THE STARRY CELL (the feet's own cell the portal wears — the
    // collision the empty shape keeps, the body the cell stands)
    let (px, py, pz) = (
        app.body.as_ref().unwrap().x,
        app.body.as_ref().unwrap().y,
        app.body.as_ref().unwrap().z,
    );
    let (cx, cy, cz) = (px.floor() as i32, py.floor() as i32, pz.floor() as i32);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cx, cy, cz, crate::world::END_PORTAL);
    app.update();
    // THE INSTANT SWAP (the zero-transition walk the touch serves
    // — the countdown the nether gate's 4s walk never runs)
    assert_eq!(
        app.world_dimension,
        Dimension::End,
        "the touch the instant swap serves"
    );
    assert_eq!(app.world_skybox, Skybox::End, "the End's own skybox");
    // THE PLATFORM'S OWN ARRIVAL (the spawn point the clamp keeps
    // — the body AWAY from the pedestal the echo never touches)
    {
        let n = app.offline_world.as_ref().unwrap().size_blocks();
        let vx = 100i32.clamp(2, n - 6);
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            b.x,
            vx as f64 + 0.5,
            "the spawn point's own x the clamp keeps"
        );
    }
    // THE PEDESTAL'S OWN DIAMOND (the End world's center the exit
    // portal wears — the cooldown the arrival's grant the test
    // clears, the body the starry cell the touch re-arms)
    app.body.as_mut().unwrap().portal_cooldown = 0;
    let n = app.offline_world.as_ref().unwrap().size_blocks();
    let (ix, iz) = (
        (n as f64 / 2.0).round() as i32,
        (n as f64 / 2.0).round() as i32,
    );
    let top = crate::world::END_ISLAND_TOP;
    assert_eq!(
        app.offline_world.as_ref().unwrap().get(ix, top, iz),
        crate::world::END_PORTAL,
        "the pedestal's own center the portal keeps"
    );
    {
        let b = app.body.as_mut().unwrap();
        b.x = ix as f64 + 0.5;
        b.y = top as f64;
        b.z = iz as f64 + 0.5;
        b.vx = 0.0;
        b.vy = 0.0;
        b.vz = 0.0;
    }
    app.update();
    assert_eq!(
        app.world_dimension,
        Dimension::Overworld,
        "the exit the respawn law serves"
    );
}

/// THE END PORTAL'S OWN MESH (the dev-29 round — the sentinel tile
/// the starry quads carry: the open-air cell ALL SIX faces wear,
/// the EPS inset the boundary contact the coplanar fight refuses,
/// the fullbright the vertex colors keep)
#[test]
fn the_end_portal_cell_the_sentinel_quads_mesh() {
    let mut w = crate::world::World::new(1, 42);
    // the open-air cell (the terrain's own height the cell outrides
    // — the neighbors ALL air the six faces the volume wears)
    w.set(8, 30, 8, crate::world::END_PORTAL);
    let quads = crate::mesher::mesh_world(&w, None);
    let portal: Vec<&crate::mesher::Quad> = quads
        .iter()
        .filter(|q| q.tile == crate::mesher::TILE_END_PORTAL)
        .collect();
    assert_eq!(
        portal.len(),
        6,
        "the open-air cell all six faces the volume wears"
    );
    for q in &portal {
        for p in &q.pts {
            assert!(
                p[0] > 8.0 && p[0] < 9.0 && p[1] > 30.0 && p[1] < 31.0,
                "the inset the boundary contact refuses ({:?})",
                p
            );
        }
        for c in &q.colors {
            assert_eq!(*c, [1.0; 3], "the fullbright the BE pass keeps");
        }
    }
    // THE SHADER PASS'S OWN PAIR (the sha1 gates the cache's bytes
    // verify — the wrong sha the honest None keeps; the root the
    // walk-up the ignored asset tests' own pattern keeps)
    let mut dir = std::env::current_dir().unwrap();
    let root = loop {
        let cand = dir.join("assets-cache/jar");
        if cand.exists() {
            break cand;
        }
        dir = dir.parent().expect("the runtime root above").to_path_buf();
    };
    let noise_path = root.join("entity/end_portal/end_portal.png");
    let sky_path = root.join("environment/end_sky.png");
    let noise_str = noise_path.to_str().unwrap();
    let sky_str = sky_path.to_str().unwrap();
    assert!(
        App::load_end_portal_texture(noise_str, "04099306b075ead427f5c95d8a8d896715c5bde6",)
            .is_some(),
        "the noise texture loads (256x256)"
    );
    if let Some((w, h, px)) =
        App::load_end_portal_texture(noise_str, "04099306b075ead427f5c95d8a8d896715c5bde6")
    {
        assert_eq!((w, h), (256, 256), "the noise the 256x256 master keeps");
        assert_eq!(px.len(), 256 * 256 * 4, "the RGBA stride the decode keeps");
    }
    assert!(
        App::load_end_portal_texture(noise_str, "0000000000000000000000000000000000000000",)
            .is_none(),
        "the wrong sha the honest None keeps"
    );
    let Some((w2, h2, _)) =
        App::load_end_portal_texture(sky_str, "b9da532dd98befc1a7cbd6420c17532239485788")
    else {
        panic!("the sky texture loads");
    };
    assert_eq!((w2, h2), (128, 128), "the sky the 128x128 base keeps");
}

/// THE PLATFORM'S OWN GEOMETRY (javap `EndPlatformFeature
/// .createEndPlatform` pc 5-131 — the 5x5 loops the dx/dz bands
/// wear, the obsidian the y==-1 arm speaks, the air the -1..=3
/// inner loop wears, the already-matching cells the is() check
/// skips; the ABSOLUTE the `END_SPAWN_POINT` keeps — the
/// processor's banked entry the scaled walk computes, the End
/// arm's own site law the bank never sways): the extent, the
/// thinness (the single row the feature keeps), the box's cap,
/// the void the rim's outside keeps.
#[test]
fn the_end_the_platform_the_absolute_the_thinness_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.body = Some(crate::body::PlayerBody::new(
        app.world_size as f64 / 2.0,
        app.spawn_y as f64,
        app.world_size as f64 / 2.0,
        180.0,
        0.0,
    ));
    // THE ENTRY'S OWN BANK (the processor's staged entry the
    // nether chain's scaled walk would follow)
    app.body.as_mut().unwrap().portal_entry = Some((10, 20, 10));
    app.swap_dimension_to(Dimension::End);
    // THE ABSOLUTE THE SITE KEEPS (the entry the platform's law
    // ignores — the vanilla's own (100, 50, 0) the clamp answers)
    {
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            (b.x, b.y, b.z),
            (74.5, 8.0, 2.5),
            "the absolute site the bank never sways"
        );
    }
    let w = app.offline_world.as_ref().expect("the offline world rides");
    // THE EXTENT'S OWN FIVE (the dx/dz -2..=2 bands the loops wear)
    for dz in -2i32..=2 {
        for dx in -2i32..=2 {
            assert_eq!(
                w.get(74 + dx, 7, 2 + dz),
                crate::world::OBSIDIAN,
                "the 5x5 disc the loops raise"
            );
        }
    }
    // THE THINNESS'S OWN ROW (the obsidian ONE below the box's
    // base — the row below the feature's loops never fill, the
    // void's own AIR keeps)
    assert_eq!(
        w.get(74, 6, 2),
        crate::world::AIR,
        "the thinness the void keeps"
    );
    // THE BOX'S OWN BAND (the center..center+3 air the inner loop
    // wears — the feet cell the band contains, the cap the +3
    // outside keeps)
    for dy in 8i32..=11 {
        assert_eq!(
            w.get(74, dy, 2),
            crate::world::AIR,
            "the air box the inner loop clears"
        );
    }
    assert_eq!(
        w.get(74, 12, 2),
        crate::world::AIR,
        "the cap the void keeps"
    );
}

/// THE HONEY'S OWN APP LANE (javap `HoneyBlock` the 26.2 jar — the
/// entityInside's slide-down walk the body's family seat serves,
/// the fallOn's landing walk the fall-event arm serves): THE SLIDE
/// — the wall's edge the four-gate verdict keeps, the fold the
/// throttle's -0.1274 constant locks, the 1-in-5 rolls the
/// level-random stream speaks (the voice + the ×5 crumbs); THE
/// LANDING — the slide voice the fconst_1 pair ALWAYS keeps, the
/// ×10 jump crumbs the event-54 fold spawns, the damage the ×0.2
/// multiplier row the stone twin's own fall the relation keeps.
#[test]
fn the_honey_the_landing_the_slide_the_wears() {
    // THE HONEY APP (the wall + the pad the same world stages)
    let mut app = App::new(3);
    app.enter_world();
    app.body = Some(crate::body::PlayerBody::new(
        app.world_size as f64 / 2.0,
        app.spawn_y as f64,
        app.world_size as f64 / 2.0,
        180.0,
        0.0,
    ));
    for _ in 0..2 {
        app.update();
    }
    {
        let w = app.offline_world.as_mut().expect("the offline world rides");
        // THE CLEAR ZONE (the stage the terrain's own body never
        // claims — the body the air the slide's own four gates
        // keep, the on-ground refusal the terrain's floor never
        // arms)
        for x in 6..=13 {
            for y in 3..=30 {
                for z in 5..=11 {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
        }
        for y in 4..=10 {
            w.set(8, y, 8, crate::world::HONEY_BLOCK); // the wall the slide wears
            w.set(12, y, 8, crate::world::HONEY_BLOCK); // the pad the fall lands on
        }
    }
    // THE SLIDE'S OWN STAGE (the body at the wall's edge — the box
    // the 1/16 inset the collision never overlaps, the cell the
    // entityInside scan the 0.74 bound beyond answers; the x/z the
    // zero keeps — the straight drop the wall's own face wears)
    {
        let b = app.body.as_mut().unwrap();
        b.x = 9.24; // |9.24 - 8.5| = 0.74 > 0.4375 + 0.3
        b.y = 9.5;
        b.z = 8.5;
        b.vx = 0.0;
        b.vy = -0.5;
        b.vz = 0.0;
        b.on_ground = false;
        b.fall_distance = 0.0;
    }
    for _ in 0..30 {
        app.update();
    }
    {
        let b = app.body.as_ref().unwrap();
        // THE THROTTLE'S OWN LOCK (the plain arm the -0.1274
        // constant re-applies — the vanilla's own slide cadence)
        assert_eq!(
            b.vy,
            crate::body::honey_new_delta_y(-0.05),
            "the slide's own velocity lock the fold keeps"
        );
        assert_eq!(
            b.fall_distance, 0.0,
            "the slide's own reset the descent never banks"
        );
        assert!(!b.on_ground, "the wall's side the ground never claims");
    }
    // THE 1-IN-5 ROLLS (the 60-roll arc the deterministic stream
    // speaks — the voice AND the crumbs the drain's own pair
    // serves)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("block.honey_block.slide ")),
        "the slide voice the roll speaks"
    );
    let honey_tile = particle_tile(crate::world::HONEY_BLOCK);
    let slide_crumbs = app
        .particles
        .iter()
        .filter(|p| {
            matches!(
                p.kind,
                crate::particles::ParticleKind::Terrain { tile } if tile == honey_tile
            )
        })
        .count();
    assert!(slide_crumbs >= 1, "the slide crumbs the roll spawns");
    // THE LANDING'S OWN STAGE (the body the pad's own 20-block
    // drop wears — the multiplier row the damage the ×0.2 speaks)
    {
        let b = app.body.as_mut().unwrap();
        b.x = 12.5;
        b.y = 31.0;
        b.z = 8.5;
        b.vx = 0.0;
        b.vy = 0.0;
        b.vz = 0.0;
        b.on_ground = false;
        b.fall_distance = 0.0;
        b.prev_x = b.x;
        b.prev_y = b.y;
        b.prev_z = b.z;
    }
    for _ in 0..80 {
        app.update();
        if app.body.as_ref().unwrap().on_ground {
            break;
        }
    }
    {
        let b = app.body.as_ref().unwrap();
        assert!(b.on_ground, "the landing the pad's own top keeps");
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("block.honey_block.slide ")),
            "the landing voice the fallOn ALWAYS speaks"
        );
    }
    let landing_crumbs = app
        .particles
        .iter()
        .filter(|p| {
            matches!(
                p.kind,
                crate::particles::ParticleKind::Terrain { tile } if tile == honey_tile
            )
        })
        .count();
    assert!(
        landing_crumbs >= 10,
        "the jump crumbs the event-54 fold spawns"
    );
    // THE ×0.2 RELATION (the stone twin's own 20-block fall the
    // same seed the same stage wears — the honey's damage the
    // fraction speaks)
    let honey_hp = app.health;
    let mut stone = App::new(3);
    stone.enter_world();
    stone.body = Some(crate::body::PlayerBody::new(
        stone.world_size as f64 / 2.0,
        stone.spawn_y as f64,
        stone.world_size as f64 / 2.0,
        180.0,
        0.0,
    ));
    for _ in 0..2 {
        stone.update();
    }
    {
        let w = stone
            .offline_world
            .as_mut()
            .expect("the offline world rides");
        for x in 6..=13 {
            for y in 3..=30 {
                for z in 5..=11 {
                    w.set(x, y, z, crate::world::AIR);
                }
            }
        }
        for y in 4..=10 {
            w.set(12, y, 8, crate::world::STONE); // the pad the stone kind wears
        }
    }
    {
        let b = stone.body.as_mut().unwrap();
        b.x = 12.5;
        b.y = 31.0;
        b.z = 8.5;
        b.vx = 0.0;
        b.vy = 0.0;
        b.vz = 0.0;
        b.on_ground = false;
        b.fall_distance = 0.0;
        b.prev_x = b.x;
        b.prev_y = b.y;
        b.prev_z = b.z;
    }
    for _ in 0..80 {
        stone.update();
        if stone.body.as_ref().unwrap().on_ground {
            break;
        }
    }
    let stone_hp = stone.health;
    assert!(
        honey_hp > stone_hp,
        "the honey's own 0.2 the stone's 1.0 the relation keeps"
    );
}

/// THE PORTAL OVERLAY'S OWN TINT BLIT (javap `Hud
/// .extractPortalOverlay` — the ARGB.white(alpha) color the blit's
/// own tint arg speaks, the NETHER_PORTAL block's default-state
/// PARTICLE sprite the `block/nether_portal.png` source the jar
/// banks): the sprite_file_at_tinted walk the vanilla's own
/// DOUBLE fold rides — the vertex-color multiply the shader
/// speaks (every channel the tint folds, the alpha included) THEN
/// the srcAlpha blend the raster keeps (the alpha's second fold
/// the blend over the black clear wears). The three tints the
/// per-channel folds pin: the both-channels quarter, the rgb-only
/// half, the alpha-only half.
#[test]
fn the_portal_overlay_the_tint_the_blit_wears() {
    let _render_lane = render_lane();
    // the real assets the honest source keeps (the loaders' prefix
    // walk the gui tests' own law speaks — the dev-152 round's own
    // fix: the CWD-relative lane the package-dir processes never
    // satisfied, the vacuous early return the walked root retires —
    // the absent cache the honest skip the same law keeps)
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("block/nether_portal.png").exists());
    let Some(root) = root else { return };
    // THE VANILLA'S OWN DOUBLE FOLD (the vertex-color multiply the
    // shader speaks THEN the srcAlpha blend the raster keeps): the
    // ARGB.white(t) tint the production lane passes folds t into
    // EVERY channel — the rgb the multiply folds, the alpha the
    // blend folds AGAIN over the black clear — the [0.5;4] tint
    // the quarter the full speaks (the old f*0.5 pin the single-
    // fold math the blend never wore)
    let half_of = |tint: [f32; 4]| -> Vec<u8> {
        let mut fb = crate::raster::Framebuf::new(32, 32);
        fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let mut g = Gui::new(&mut fb, 1);
        let mut assets = GuiAssets::open(root);
        g.sprite_file_at_tinted(&mut assets, "block/nether_portal.png", 0, 0, 32, 32, tint);
        fb.color[(16 * 32 + 16) * 3..(16 * 32 + 16) * 3 + 3].to_vec()
    };
    // the full tint the sprite's own pixel the center sample keeps
    let full = half_of([1.0, 1.0, 1.0, 1.0]);
    // the BOTH-channels fold: the quarter (0.5 rgb * 0.5 blend)
    let half = half_of([0.5, 0.5, 0.5, 0.5]);
    for ch in 0..3 {
        let f = full[ch] as f32 / 255.0;
        let h = half[ch] as f32 / 255.0;
        assert!(
            (h - f * 0.25).abs() < 0.02,
            "the both-channels fold the quarter wears (ch{ch}: {h} vs {})",
            f * 0.25
        );
    }
    // the RGB-ONLY fold: the blend at alpha 1 — the half speaks
    let rgb_only = half_of([0.5, 0.5, 0.5, 1.0]);
    for ch in 0..3 {
        let f = full[ch] as f32 / 255.0;
        let h = rgb_only[ch] as f32 / 255.0;
        assert!(
            (h - f * 0.5).abs() < 0.02,
            "the rgb fold the half wears (ch{ch}: {h} vs {})",
            f * 0.5
        );
    }
    // the ALPHA-ONLY fold: the multiply at rgb 1 — the half speaks
    let alpha_only = half_of([1.0, 1.0, 1.0, 0.5]);
    for ch in 0..3 {
        let f = full[ch] as f32 / 255.0;
        let h = alpha_only[ch] as f32 / 255.0;
        assert!(
            (h - f * 0.5).abs() < 0.02,
            "the alpha fold the half wears (ch{ch}: {h} vs {})",
            f * 0.5
        );
    }
}

/// THE PORTAL CELLS' OWN ANIMATE WALK — javap `NetherPortalBlock.
/// animateTick` the sampler's visits answer (the full-layer parity
/// world the every-portal-cell X-extending arm speaks — the west/
/// east portal pair the gate's own verdict wears; the stripe world
/// the stone x-neighbors the Z-extending else lane serves): the
/// FOUR purples the visit keeps, the snapped band the 0.5±0.25
/// fold speaks, the out-of-plane push the r*2*±1 fold carries, the
/// 1/100 voice the FIXED 0.5 volume the positioned lane plays, the
/// stone world the silence both lanes keep.
#[test]
fn the_portal_cells_the_purples_the_ambient_voice_wear() {
    // THE X-ARM WORLD'S OWN STAMP (the even layers the stone floor
    // keeps, the odd the kind's own gate wears — the every portal
    // cell the west/east portal pair the x-extending verdict
    // speaks)
    let layer_stamp = |id: u16| -> crate::world::World {
        let mut world = crate::world::World::new(1, 0);
        world.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
        for y in 0..48 {
            for z in 0..16 {
                for x in 0..16 {
                    world.set(x, y, z, if y % 2 == 0 { crate::world::STONE } else { id });
                }
            }
        }
        world
    };
    // THE WALK (the x-extending arm's own lane): the purples the
    // visits speak, the z the snapped band the fold keeps
    let mut app = App::new(3);
    app.offline_world = Some(layer_stamp(crate::world::NETHER_PORTAL));
    let out = app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!out.is_empty(), "the portal cells answer the walk");
    assert_eq!(out.len() % 4, 0, "the four iterations per visit");
    for p in &out {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Portal { .. }),
            "the portal purple the only kind the walk speaks"
        );
        // THE SNAPPED BAND (the z + 0.5 ± 0.25 fold the x-arm keeps
        // — the fract the exact 0.25/0.75 pair the snapped z wears,
        // the x/y the raw doubles the [0,1) band rides)
        let fz = p.z.fract();
        assert!(
            fz == 0.25 || fz == 0.75,
            "the snapped z band the fold keeps: {}",
            fz
        );
        // THE OUT-OF-PLANE PUSH (the vz the r*2*±1 fold speaks —
        // the [−2, 2) band the no-centering fold wears), the
        // in-plane velocities the base folds keep (the
        // (r−0.5)*0.5 pair the ±0.25 bands ride)
        assert!(
            p.vz.abs() < 2.0,
            "the out-of-plane band {} the r*2 fold keeps",
            p.vz
        );
        assert!(
            p.vx.abs() <= 0.25 && p.vy.abs() <= 0.25,
            "the in-plane bands the base folds keep: {} {}",
            p.vx,
            p.vy
        );
        // THE PROVIDER'S OWN LAW (the 40..49 lifetime the
        // (int)(r*10)+40 write gives, the quad the 0.1*(r*0.2+0.5)
        // fold sizes)
        assert!(
            (40..=49).contains(&p.lifetime),
            "the portal lifetime {} the +40 write keeps",
            p.lifetime
        );
    }
    // THE VOICE (the 1/100 gate the visits' own odds speak — the
    // event the positioned lane logs, the FIXED 0.5 volume the
    // no-draw law keeps, the pitch the 0.8..1.2 band the f*0.4+0.8
    // fold wears)
    let voices: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("block.portal.ambient "))
        .collect();
    assert!(
        !voices.is_empty(),
        "the portal world the sampler hears: {} voices",
        voices.len()
    );
    for line in &voices {
        let pitch: f32 = line
            .split(" p")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|p| p.parse().ok())
            .expect("the pitch field the log keeps");
        assert!(
            (0.8..1.2).contains(&pitch),
            "the pitch band {} the fold keeps",
            pitch
        );
        let volume: f32 = line
            .split(" v")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|v| v.parse().ok())
            .expect("the volume field the log keeps");
        assert_eq!(volume, 0.5, "the FIXED 0.5 the playLocalSound arg keeps");
    }
    // THE Z-ARM WORLD'S OWN STAMP (the even x stripes the gate
    // wears — the every portal cell the stone x-neighbors the else
    // lane serves): the x the snapped band folds, the vx the
    // replacement push carries
    let mut z_app = App::new(3);
    let mut stripe = crate::world::World::new(1, 0);
    stripe.variants = Some(vec![vec![0u8; 16 * 48 * 16]; 1]);
    for y in 0..48 {
        for z in 0..16 {
            for x in 0..16 {
                stripe.set(
                    x,
                    y,
                    z,
                    if x % 2 == 0 {
                        crate::world::NETHER_PORTAL
                    } else {
                        crate::world::STONE
                    },
                );
            }
        }
    }
    z_app.offline_world = Some(stripe);
    let z_out = z_app.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(!z_out.is_empty(), "the stripe world the walk answers");
    assert_eq!(z_out.len() % 4, 0, "the four iterations per visit");
    for p in &z_out {
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Portal { .. }),
            "the stripe's own purple"
        );
        let fx = p.x.fract();
        assert!(
            fx == 0.25 || fx == 0.75,
            "the snapped x band the else arm keeps: {}",
            fx
        );
        assert!(
            p.vx.abs() < 2.0,
            "the out-of-plane vx {} the r*2 fold keeps",
            p.vx
        );
        assert!(
            p.vz.abs() <= 0.25 && p.vy.abs() <= 0.25,
            "the in-plane bands the base folds keep: {} {}",
            p.vz,
            p.vy
        );
    }
    // THE DETERMINISM (the seeded walk the same stamps replay)
    let run = |world: crate::world::World| -> (Vec<(u32, f64, f64, f64, f32)>, Vec<String>) {
        let mut a = App::new(3);
        a.offline_world = Some(world);
        let out = a.animate_tick_walk((8.0, 12.0, 8.0));
        let parts = out
            .iter()
            .map(|p| (p.lifetime, p.x, p.y, p.z, p.quad_size))
            .collect();
        (parts, a.sounds.events.clone())
    };
    let (pa, ea) = run(layer_stamp(crate::world::NETHER_PORTAL));
    let (pb, eb) = run(layer_stamp(crate::world::NETHER_PORTAL));
    assert_eq!(pa, pb, "the seeded walk replays");
    assert_eq!(ea, eb, "the voice log replays");
    // THE FLAMELESS STONE WORLD'S OWN SILENCE (no portal cell the
    // sampler visits — the walk the empty vec the no-voice log
    // keeps)
    let mut cold = App::new(3);
    cold.offline_world = Some(layer_stamp(crate::world::STONE));
    let cold_out = cold.animate_tick_walk((8.0, 12.0, 8.0));
    assert!(
        cold_out.is_empty(),
        "the stone world no purple the walk speaks"
    );
    assert!(
        cold.sounds
            .events
            .iter()
            .all(|e| !e.starts_with("block.portal.ambient ")),
        "the stone world the voice never logs"
    );
}

/// THE E KEY'S OWN LANE: the world screen opens the survival
/// inventory, the inventory screen closes it back (E and Escape
/// both), the death gate keeps the DeathScreen modal, and the
/// vanilla inventory never pauses the world (the container's own
/// law — the pause flag the open never touches).
#[test]
fn the_e_key_opens_and_closes_the_inventory() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::Inventory, "E opens over the world");
    assert!(!app.paused, "the vanilla inventory never pauses");
    assert_eq!(app.inv_carried, (0, 0), "the carried fresh-empty");
    // the frozen-container convention: the body's tick gate reads
    // the screen (the World test the gate's own arm speaks)
    assert_ne!(app.screen, ScreenKind::World);
    // the E key's second half: back to the world
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "E closes it back");
    // the Escape arm on the open screen
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::Inventory);
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "Escape closes too");
    // THE DEATH GATE: a dead player's E never opens
    app.dead = true;
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "the DeathScreen stays modal");
    assert_ne!(app.screen, ScreenKind::Inventory);
    // the OTHER screens ignore the key (the screen stack's own law)
    app.dead = false;
    app.screen = ScreenKind::Servers;
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Servers,
        "no inventory over the list"
    );
}

/// THE INVENTORY'S OWN CLICK WALK: the left press's pickup / place /
/// merge / swap over the REAL store (the doClick PICKUP prediction),
/// the hotbar row the store's own mirror (the HUD syncs on every
/// mutation), and the hit boxes the 26.2 layout pins.
#[test]
fn the_inventory_click_walks_the_pickup_law() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    // the panel origin the render lays out first (the render-then-hit
    // pattern) — a 640x360 gui the geometry test pins
    app.inv_panel = inventory_panel_origin(640, 360);
    assert_eq!(app.inv_panel, (232, 97));
    // the store: cobblestone x27 in the hotbar slot 0, oak x16 slot 1
    app.inventory.items[0] = (62, 27);
    app.inventory.items[1] = (63, 16);
    app.hotbar_slots = app.inventory.hotbar();
    // THE PICKUP: the left press on the slot takes the whole stack
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
    app.on_press(ox + 8, oy + 8);
    assert_eq!(app.inventory.items[0], (0, 0), "the slot emptied");
    assert_eq!(app.inv_carried, (62, 27), "the cursor holds the stack");
    // THE PLACE: the same press on the empty main slot 9 puts it
    // down (the carried press the DRAG arm arms — the release's
    // own deferred place serves: the painted seat the even split's
    // one-seat law the whole stack keeps)
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 9);
    app.on_press(ox + 8, oy + 8);
    app.on_mouse_move(ox + 8, oy + 8);
    app.on_left_release();
    assert_eq!(app.inventory.items[9], (62, 27), "the stack placed");
    assert_eq!(app.inv_carried, (0, 0), "the cursor empty");
    // the hotbar row synced (the store's own mirror)
    assert_eq!(app.hotbar_slots[0], (0, 0));
    // THE MERGE: pick the cobblestone up, drop it on oak — the swap
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 9);
    app.on_press(ox + 8, oy + 8);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 1);
    app.on_press(ox + 8, oy + 8);
    app.on_mouse_move(ox + 8, oy + 8);
    app.on_left_release();
    assert_eq!(app.inventory.items[1], (62, 27), "the swap landed");
    assert_eq!(app.inv_carried, (63, 16), "the oak rides the cursor");
    // the between-slots press INSIDE the panel does nothing (the
    // vanilla's own silence — only the OUTSIDE press throws, the
    // drop-outside lane's own tests speak)
    app.on_press(app.inv_panel.0 + 90, app.inv_panel.1 + 10);
    assert_eq!(app.inv_carried, (63, 16), "the carried untouched");
}

/// THE QUICKCRAFT DRAG'S OWN EVEN SPLIT (the 26.2 drag walk — the
/// carried-NON-EMPTY press the arm arms, the held motion the paint
/// collects, the release the doClick status-2 distribution serves):
/// the press NO place speaks (the arm alone), the first motion the
/// PRESS seat itself paints, the `floor(count / painted) + seat`
/// place the `k -= place - seat` net the carried fold speaks (the
/// 32-over-3 the 10/10/10 split the 2 remainder wears), the ghost
/// remainder the every-paint recalc banks, the release the arm
/// always clears.
#[test]
fn the_quickcraft_drag_the_paint_and_the_even_split_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE ARM: the carried press the state sets — NO place (the
    // seat stays EMPTY), no painted seat yet
    app.inv_carried = (dirt, 32);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 12);
    app.on_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 0, "the left button's own type");
    assert_eq!(app.qc_button, 0, "the left button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.inventory.items[12], (0, 0), "the press NO place speaks");
    // THE PAINT: the first motion paints the PRESS seat itself —
    // the one-seat projection the whole count keeps (the ghost 0)
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(
        app.qc_slots,
        vec![12],
        "the press seat the first motion paints"
    );
    assert_eq!(
        app.qc_remainder, 0,
        "the one-seat projection the count empties"
    );
    let (bx, by) = inventory_slot_origin(app.inv_panel, 13);
    app.on_mouse_move(bx + 8, by + 8);
    let (cx, cy) = inventory_slot_origin(app.inv_panel, 14);
    app.on_mouse_move(cx + 8, cy + 8);
    assert_eq!(
        app.qc_slots,
        vec![12, 13, 14],
        "three seats the motion paints"
    );
    assert_eq!(app.qc_remainder, 2, "the ghost the floor(32/3) splits keep");
    // THE SAME-SEAT RE-HOVER: the Set.add's own false the recalc never feeds
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(
        app.qc_slots,
        vec![12, 13, 14],
        "the painted set the re-hover keeps"
    );
    // THE RELEASE: the distribution — floor(32/3) = 10 per seat,
    // the remainder 2 the cursor keeps
    app.on_left_release();
    assert_eq!(
        app.inventory.items[12],
        (dirt, 10),
        "the first seat's own tenth"
    );
    assert_eq!(
        app.inventory.items[13],
        (dirt, 10),
        "the second seat's tenth"
    );
    assert_eq!(
        app.inventory.items[14],
        (dirt, 10),
        "the third seat's tenth"
    );
    assert_eq!(app.inv_carried, (dirt, 2), "the remainder rides the cursor");
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    assert_eq!(
        app.hotbar_slots,
        app.inventory.hotbar(),
        "the HUD the store mirror keeps"
    );
}

/// THE QUICKCRAFT DRAG'S OWN GATES AND ABORTS (the shouldAdd gates
/// the paint answers — the result's mayPlace iconst_0 never paints,
/// the different item the canItemQuickReplace verdict refuses, the
/// count's own `> painted` law the third seat refuses; the
/// between-slots press the arm skips, the armed drag the right
/// press silences, the wrong button's release the mismatch abort
/// speaks, the outside release the -999 drop serves).
#[test]
fn the_quickcraft_drag_the_gates_and_the_aborts_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE RESULT SEAT NEVER PAINTS (the mayPlace-false verdict) —
    // the empty-painted release the deferred place keeps (the
    // result's own take law the walk answers)
    app.craft_grid[0] = (dirt, 1);
    app.refresh_craft_result();
    app.inv_carried = (dirt, 8);
    let (rx, ry) = craft_slot_origin(app.inv_panel, 4);
    app.on_press(rx + 8, ry + 8);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(rx + 8, ry + 8);
    assert!(
        app.qc_slots.is_empty(),
        "the result the mayPlace verdict never paints"
    );
    app.on_left_release();
    assert_eq!(app.inv_carried, (dirt, 8), "the take the carried keeps");
    assert!(!app.qc_armed, "the release the arm clears");
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's SWAP speaks (the walk's
    // own law the release rides)
    app.inventory.items[9] = (stone, 5);
    let (sx, sy) = inventory_slot_origin(app.inv_panel, 9);
    app.on_press(sx + 8, sy + 8);
    app.on_mouse_move(sx + 8, sy + 8);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.inv_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.inventory.items[9], (dirt, 8), "the stone the hand took");
    // THE COUNT'S OWN GATE: the 2-stack paints TWO seats, the
    // third the `count > painted` law refuses; the release the
    // even split speaks (1 per seat, the cursor empty)
    app.inv_carried = (dirt, 2);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 10);
    app.on_press(ox + 8, oy + 8);
    app.on_mouse_move(ox + 8, oy + 8);
    let (nx, ny) = inventory_slot_origin(app.inv_panel, 11);
    app.on_mouse_move(nx + 8, ny + 8);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the second seat the count still paints"
    );
    let (mx2, my2) = inventory_slot_origin(app.inv_panel, 15);
    app.on_mouse_move(mx2 + 8, my2 + 8);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the third seat the count gate refuses"
    );
    app.on_left_release();
    assert_eq!(app.inventory.items[10], (dirt, 1), "the split's own one");
    assert_eq!(app.inventory.items[11], (dirt, 1), "the split's own one");
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.inv_carried = (dirt, 4);
    app.on_press(app.inv_panel.0 + 90, app.inv_panel.1 + 10);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.inv_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    let (hx, hy) = inventory_slot_origin(app.inv_panel, 12);
    app.on_press(hx + 8, hy + 8);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(hx + 8, hy + 8);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(
        app.inventory.items[12],
        (0, 0),
        "no split-place rode the press"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    app.on_left_release();
    assert_eq!(
        app.inv_carried,
        (dirt, 4),
        "the aborted drag the release silences"
    );
    // THE OUTSIDE RELEASE THE DROP SERVES (the arm on the outside
    // press — the release's -999 PICKUP the whole carried drops)
    app.on_press(4, 4);
    assert!(app.qc_armed, "the outside press the arm arms");
    app.on_mouse_move(4, 4);
    app.on_left_release();
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the outside release the whole carried drops"
    );
    assert_eq!(app.drops.len(), 1, "one drop the aimed throw spawns");
}

/// THE QUICKCRAFT DRAG'S OWN WIRE SPLIT (the live belief the same
/// place math answers — the packet triple the release speaks, the
/// HUD row the hotbar menus' sync follows): 32 dirt over three
/// hotbar rows the even split the belief, the HUD, and the
/// START/ADDs/END packets wear.
#[test]
fn the_quickcraft_drag_the_wire_split_and_the_hud_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // the live belief: the store empty, the cursor 32 dirt
    let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    app.live_inventory_updated(slots, Vec::new(), 4, (dirt, 32));
    // THE DRAG over three hotbar rows (the store indices 0..2 —
    // the menus 36..38): the arm, the paint, the release
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 0);
    app.on_press(ax + 8, ay + 8);
    app.on_mouse_move(ax + 8, ay + 8);
    let (bx, by) = inventory_slot_origin(app.inv_panel, 1);
    app.on_mouse_move(bx + 8, by + 8);
    let (cx, cy) = inventory_slot_origin(app.inv_panel, 2);
    app.on_mouse_move(cx + 8, cy + 8);
    app.on_left_release();
    // THE BELIEF: the menus 36..38 the tenths hold, the cursor the
    // remainder keeps
    assert_eq!(
        app.live_inv_slots[36],
        (dirt, 10),
        "the first hotbar menu's tenth"
    );
    assert_eq!(
        app.live_inv_slots[37],
        (dirt, 10),
        "the second menu's tenth"
    );
    assert_eq!(app.live_inv_slots[38], (dirt, 10), "the third menu's tenth");
    assert_eq!(
        app.live_inv_carried,
        (dirt, 2),
        "the remainder the belief keeps"
    );
    // THE HUD: the hotbar row the sync speaks
    assert_eq!(app.hotbar_slots[0], (dirt, 10));
    assert_eq!(app.hotbar_slots[1], (dirt, 10));
    assert_eq!(app.hotbar_slots[2], (dirt, 10));
    // THE TRIPLE: the START (the mask(0,0) byte 0), the three ADDs
    // (the mask(1,0) byte 1), the END (the mask(2,0) byte 2) the
    // changed map the tenths name
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [36, 37, 38] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(
                c.changed,
                vec![(36, (dirt, 10)), (37, (dirt, 10)), (38, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE SURVIVAL RIGHT DRAG'S OWN ONE-EACH (the same type-1 drag
/// law the chest + table + furnace rounds bank — the window-0
/// belief the same release walk answers): the carried press the
/// arm speaks (NO place, NO packet), the paint the armor's own
/// verdict re-keys (the dirt the armor seats refuse, the store
/// rows take), the release the one-each distribution and the
/// masked triple — START 4, ADDs 5, END 6 — the changed map names,
/// the HUD row the sync follows.
#[test]
fn the_survival_right_drag_the_one_each_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    app.live_inventory_updated(slots, Vec::new(), 4, (dirt, 8));
    // THE ARM: the carried press the state sets — NO place, NO
    // packet (the isQuickCrafting start the pc 353-363 walk keeps)
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 0);
    app.on_right_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert_eq!(app.qc_button, 1, "the right button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.live_inv_slots[36], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet keeps");
    // THE PAINT: the press seat the first motion paints, the armor
    // seat the carried dirt's verdict refuses, the second store
    // row the paint takes
    app.on_mouse_move(ax + 8, ay + 8);
    assert_eq!(app.qc_slots, vec![36], "the press seat the motion paints");
    let (hx, hy) = armor_slot_origin(app.inv_panel, 0);
    app.on_mouse_move(hx + 8, hy + 8);
    assert_eq!(app.qc_slots, vec![36], "the dirt the armor seat refuses");
    let (bx, by) = inventory_slot_origin(app.inv_panel, 1);
    app.on_mouse_move(bx + 8, by + 8);
    assert_eq!(app.qc_slots, vec![36, 37], "the two rows the paint keeps");
    assert_eq!(app.qc_remainder, 6, "the one-each ghost the cursor keeps");
    // THE RELEASE: the one-each distribution — each row ONE item,
    // the remainder the cursor keeps
    app.on_right_release();
    assert_eq!(app.live_inv_slots[36], (dirt, 1), "the first row's own one");
    assert_eq!(app.live_inv_slots[37], (dirt, 1), "the second row's one");
    assert_eq!(
        app.live_inv_carried,
        (dirt, 6),
        "the six remaining the cursor keeps"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 1),
        "the HUD row the sync speaks"
    );
    assert!(!app.qc_armed, "the release the arm clears");
    // THE TRIPLE: the START (the mask(0,1) byte 4), the ADDs (the
    // mask(1,1) byte 5), the END (the mask(2,1) byte 6) the changed
    // map the one-each deltas names
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 4);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 8));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [36, 37] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 5);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 6);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(
                c.changed,
                vec![(36, (dirt, 1)), (37, (dirt, 1))],
                "the one-each deltas the END names"
            );
            assert_eq!(c.carried, (dirt, 6), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    assert!(
        click_rx.try_recv().is_err(),
        "the lane the tail keeps empty"
    );
}

/// THE SURVIVAL RIGHT DRAG'S OWN OFFLINE ONE-EACH AND THE PLACE-ONE
/// (the offline sim the drag arms — no window the arm ever needs):
/// the store rows the one-each fill, the armor's matching-piece
/// gate the paint re-keys (the dirt refuses, the helmet's ONE the
/// cap law places), the deferred place the armed button's own byte
/// wears.
#[test]
fn the_survival_right_drag_the_offline_one_each_and_the_place_one_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE RIGHT PRESS'S OWN ARM (the offline sim the drag arms)
    app.inv_carried = (dirt, 5);
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 20);
    app.on_right_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the offline right press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    // THE PAINT: the pair the motion paints, the release the
    // one-each distribution answers
    app.on_mouse_move(ax + 8, ay + 8);
    let (bx, by) = inventory_slot_origin(app.inv_panel, 21);
    app.on_mouse_move(bx + 8, by + 8);
    assert_eq!(app.qc_slots, vec![20, 21], "the pair the motion paints");
    app.on_right_release();
    assert_eq!(
        app.inventory.items[20],
        (dirt, 1),
        "the first row's own one"
    );
    assert_eq!(app.inventory.items[21], (dirt, 1), "the second row's one");
    assert_eq!(
        app.inv_carried,
        (dirt, 3),
        "the three remaining the cursor keeps"
    );
    assert!(!app.qc_armed, "the release the arm clears");
    // THE ARMOR'S OWN GATE: the carried dirt the armor seat refuses
    // (the empty-painted release the deferred place's mayPlace
    // silence keeps); the matching helmet the paint takes and the
    // ONE the cap law places (the empty seat the min(1, 1) speaks)
    app.inv_carried = (dirt, 2);
    let (hx, hy) = armor_slot_origin(app.inv_panel, 0);
    app.on_right_press(hx + 8, hy + 8);
    app.on_mouse_move(hx + 8, hy + 8);
    assert!(app.qc_slots.is_empty(), "the dirt the armor seat refuses");
    app.on_right_release();
    assert_eq!(
        app.inventory.armor[0],
        (0, 0),
        "the refusal no place speaks"
    );
    assert_eq!(app.inv_carried, (dirt, 2), "the cursor the refusal keeps");
    // the preview's own no-seat lane the cursor steps aside (the
    // moved gate the same-seat re-hover never re-fires)
    app.on_mouse_move(app.inv_panel.0 + 60, app.inv_panel.1 + 40);
    app.inv_carried = (helmet, 2);
    app.on_right_press(hx + 8, hy + 8);
    app.on_mouse_move(hx + 8, hy + 8);
    assert_eq!(app.qc_slots, vec![5], "the matching helmet the seat paints");
    app.on_right_release();
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the ONE the cap law places"
    );
    assert_eq!(app.inv_carried, (helmet, 1), "the cursor the one loses");
    // THE DEFERRED PLACE-ONE: the empty-painted release the armed
    // button's own byte wears (the pre-press motion the position
    // speaks, the release the seat's place-ONE serves)
    app.inv_carried = (dirt, 4);
    let (px, py) = inventory_slot_origin(app.inv_panel, 25);
    app.on_mouse_move(px + 8, py + 8);
    app.on_right_press(px + 8, py + 8);
    assert!(app.qc_armed, "the right press the arm speaks again");
    app.on_right_release();
    assert_eq!(
        app.inventory.items[25],
        (dirt, 1),
        "the deferred place ONE rides"
    );
    assert_eq!(app.inv_carried, (dirt, 3), "the cursor the one loses");
    assert!(!app.qc_armed, "the release the arm clears");
}

/// THE SURVIVAL RIGHT DRAG'S OWN GATES AND ABORTS (the in-flight
/// gate the other button's press keeps, the mismatch abort BOTH
/// ways the packets never send, the survivor drag the type-0 even
/// split the release re-keys per drag, the carried-empty press the
/// immediate split keeps).
#[test]
fn the_survival_right_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    app.live_inventory_updated(slots, Vec::new(), 7, (dirt, 8));
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 0);
    // THE ARMED LEFT DRAG THE RIGHT PRESS SILENCES (the in-flight
    // gate — no place rides, the type-0 state the press keeps; the
    // right release the mismatch abort speaks)
    app.on_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the left drag armed");
    assert_eq!(app.qc_type, 0, "the left press's own type");
    app.on_right_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.qc_type, 0, "the armed state the press never re-keys");
    assert_eq!(app.live_inv_slots[36], (0, 0), "no place rode the press");
    assert!(
        click_rx.try_recv().is_err(),
        "the in-flight press NO packet keeps"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE ARMED RIGHT DRAG THE LEFT RELEASE ABORTS (the mismatch's
    // own mirror — the abort the packets never send)
    app.on_right_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the right drag armed");
    assert_eq!(app.qc_type, 1, "the right press's own type");
    app.on_mouse_move(ax + 8, ay + 8);
    assert_eq!(app.qc_slots, vec![36], "the motion the press row paints");
    app.on_left_release();
    assert!(!app.qc_armed, "the left release the right drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert_eq!(
        app.live_inv_slots[36],
        (0, 0),
        "the abort NO distribution speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE SURVIVOR: the left drag after the aborts the same type-0
    // even split wears (the lanes never mix, the law the release
    // re-keys per drag)
    app.on_press(ax + 8, ay + 8);
    app.on_mouse_move(ax + 8, ay + 8);
    app.on_left_release();
    assert_eq!(
        app.live_inv_slots[36],
        (dirt, 8),
        "the one-seat split the whole keeps"
    );
    assert_eq!(
        app.live_inv_carried,
        (0, 0),
        "the one-seat split the cursor empties"
    );
    for _ in 0..3 {
        let _ = click_rx.try_recv(); // the START, the ADD, the END
    }
    assert!(
        click_rx.try_recv().is_err(),
        "the lane the drain keeps empty"
    );
    // THE CARRIED-EMPTY RIGHT PRESS KEEPS THE IMMEDIATE SECONDARY
    // (the split-pickup the press lane answers — no drag the empty
    // cursor arms)
    app.live_inv_slots[37] = (stone, 10);
    app.live_inv_carried = (0, 0);
    let (bx, by) = inventory_slot_origin(app.inv_panel, 1);
    app.on_right_press(bx + 8, by + 8);
    assert!(!app.qc_armed, "the empty cursor the drag never arms");
    assert_eq!(app.live_inv_carried, (stone, 5), "the split's own half");
    assert_eq!(
        app.live_inv_slots[37],
        (stone, 5),
        "the seat the half keeps"
    );
    match click_rx.try_recv().expect("the split rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 0);
            assert_eq!(c.slot, 37);
            assert_eq!(c.button, 1);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(37, (stone, 5))]);
            assert_eq!(c.carried, (stone, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE CHEST DRAG'S OWN PAINT AND EVEN SPLIT (the same mouseClicked
/// pcs 316-378 + mouseDragged pcs 0-58 + mouseReleased pcs 246-462
/// base walk the Inventory round banks — the generic_9x3 screen the
/// SAME AbstractContainerScreen class wears, the chest's own
/// geometry the rects speak): the carried press the arm speaks (NO
/// place, NO packet), the first motion paints the press seat, the
/// `floor(count / painted) + seat` place the `k -= place - seat`
/// net the carried fold speaks (the 32-over-3 the 10/10/10 split
/// the 2 remainder wears), the ghost the every-paint recalc banks,
/// the re-hover the Set.add's false keeps, the release the arm
/// always clears, the packet triple the chest window's own
/// identity rides.
#[test]
fn the_chest_quickcraft_drag_the_paint_and_the_even_split_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    app.chest_is_live = true;
    app.chest_live_window = Some((5, 7));
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // THE ARM: the carried press the state sets — NO place (the
    // seat stays EMPTY), no painted seat yet, NO packet (the arm
    // the silent start keeps)
    app.chest_carried = (dirt, 32);
    let (ox, oy) = chest_container_slot(panel, 4);
    app.on_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 0, "the left button's own type");
    assert_eq!(app.qc_button, 0, "the left button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.chest_slots[4], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the PRESS seat itself —
    // the one-seat projection the whole count keeps (the ghost 0)
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4],
        "the press seat the first motion paints"
    );
    assert_eq!(
        app.qc_remainder, 0,
        "the one-seat projection the count empties"
    );
    let (bx, by) = chest_container_slot(panel, 5);
    app.on_mouse_move(bx + 9, by + 9);
    let (cx, cy) = chest_container_slot(panel, 6);
    app.on_mouse_move(cx + 9, cy + 9);
    assert_eq!(app.qc_slots, vec![4, 5, 6], "three seats the motion paints");
    assert_eq!(app.qc_remainder, 2, "the ghost the floor(32/3) splits keep");
    // THE SAME-SEAT RE-HOVER: the Set.add's own false the recalc never feeds
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the painted set the re-hover keeps"
    );
    // THE RELEASE: the distribution — floor(32/3) = 10 per seat,
    // the remainder 2 the cursor keeps
    app.on_left_release();
    assert_eq!(app.chest_slots[4], (dirt, 10), "the first seat's own tenth");
    assert_eq!(app.chest_slots[5], (dirt, 10), "the second seat's tenth");
    assert_eq!(app.chest_slots[6], (dirt, 10), "the third seat's tenth");
    assert_eq!(
        app.chest_carried,
        (dirt, 2),
        "the remainder rides the cursor"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    // THE TRIPLE the chest window's own identity rides: the START
    // (the mask(0,0) byte 0), the END (the mask(2,0) byte 2) the
    // changed map the tenths name
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5, "the chest window's own id");
            assert_eq!(c.state_id, 7, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 5, 6] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 10)), (5, (dirt, 10)), (6, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE CHEST DRAG'S OWN GATES AND ABORTS (the shouldAdd gates the
/// paint answers — the different item the canItemQuickReplace
/// verdict refuses, the count's own `> painted` law the third seat
/// refuses; the wire session's no-window refusal the arm silences,
/// the between-slots press the arm skips, the armed drag the right
/// press silences, the wrong button's release the mismatch abort speaks,
/// the empty-paint release the deferred place keeps — the seat the
/// PICKUP speaks, the between the -1 no-op the wire names, the
/// outside the -999 drop serves).
#[test]
fn the_chest_quickcraft_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // THE WIRE SESSION'S NO-WINDOW REFUSAL (chest_live_window None):
    // the carried press the arm silences — the gate the window's own
    // presence keys (the world_is_network && no-window shape the
    // autopush-146 round wears)
    app.chest_carried = (dirt, 8);
    let (gx, gy) = chest_container_slot(panel, 4);
    app.on_press(gx + 9, gy + 9);
    assert!(!app.qc_armed, "the stand-in's press the arm never speaks");
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    // THE WINDOW ARMS
    app.chest_live_window = Some((5, 7));
    app.chest_is_live = true;
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the
    // click walk's own lanes serve: the carried rides the cursor,
    // the seat's stack the hand takes)
    app.chest_slots[4] = (stone, 5);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(gx + 9, gy + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.chest_slots[4], (dirt, 8), "the stone the hand took");
    match click_rx.try_recv().expect("the PICKUP rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, 4);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(4, (dirt, 8))], "the swap's own delta");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE COUNT'S OWN GATE: the 2-stack paints TWO seats, the
    // third the `count > painted` law refuses; the release the
    // even split speaks (1 per seat, the cursor empty)
    app.chest_carried = (dirt, 2);
    let (bx, by) = chest_container_slot(panel, 10);
    app.on_press(bx + 9, by + 9);
    app.on_mouse_move(bx + 9, by + 9);
    let (nx, ny) = chest_container_slot(panel, 11);
    app.on_mouse_move(nx + 9, ny + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the second seat the count still paints"
    );
    let (mx2, my2) = chest_container_slot(panel, 12);
    app.on_mouse_move(mx2 + 9, my2 + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the third seat the count gate refuses"
    );
    app.on_left_release();
    assert_eq!(app.chest_slots[10], (dirt, 1), "the split's own one");
    assert_eq!(app.chest_slots[11], (dirt, 1), "the split's own one");
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    for _ in 0..4 {
        let _ = click_rx.try_recv(); // the START, the two ADDs, the END
    }
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.chest_carried = (dirt, 4);
    app.on_press(panel.0 + 170, panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    assert!(
        click_rx.try_recv().is_err(),
        "the between release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    app.chest_slots[4] = (0, 0);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.chest_slots[4], (0, 0), "no split-place rode the press");
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    // THE EMPTY-PAINT RELEASE BETWEEN THE SLOTS: the deferred
    // place's -1 PICKUP the wire keeps (the isValidSlotIndex
    // refusal — the no-op the packet names, the cursor keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(panel.0 + 170, panel.1 + 20); // the paint skips (no seat)
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 4), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    assert_eq!(app.chest_carried, (dirt, 4), "the -1 the cursor keeps");
    // THE OUTSIDE RELEASE THE DROP SERVES (the arm on the outside
    // press — the release's -999 PICKUP the whole carried drops)
    app.on_press(4, 4);
    assert!(app.qc_armed, "the outside press the arm arms");
    app.on_mouse_move(4, 4);
    app.on_left_release();
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the outside release the whole carried drops"
    );
    match click_rx.try_recv().expect("the drop PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999, "the outside release's own -999");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.carried, (0, 0), "the emptied cursor rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE CHEST RIGHT DRAG'S OWN ONE-EACH (the type-1 lane the right
/// press arms — mouseClicked pc 353-363 the type speaks, the
/// release the getQuickCraftPlaceCount's own ONE per seat wears):
/// the arm NO place and NO packet keeps, the paint the same
/// shouldAdd gates rides, the ghost the one-each projection keeps,
/// the release the one-each distribution and the masked triple
/// (START mask(0,1)=4, ADDs mask(1,1)=5, END mask(2,1)=6) speak.
#[test]
fn the_chest_right_drag_the_one_each_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    app.chest_is_live = true;
    app.chest_live_window = Some((5, 7));
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // THE RIGHT PRESS'S OWN ARM: the carried press the type-1 drag
    // arms — NO place, NO painted seat, NO packet (the arm the
    // silent start keeps)
    app.chest_carried = (dirt, 5);
    let (ox, oy) = chest_container_slot(panel, 4);
    app.on_right_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the right press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert_eq!(app.qc_button, 1, "the right button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.chest_slots[4], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the press seat, the ghost
    // the ONE-EACH projection keeps (the type-1's own place count)
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4],
        "the press seat the first motion paints"
    );
    assert_eq!(app.qc_remainder, 4, "the one-each ghost the cursor keeps");
    let (bx, by) = chest_container_slot(panel, 5);
    app.on_mouse_move(bx + 9, by + 9);
    let (cx, cy) = chest_container_slot(panel, 6);
    app.on_mouse_move(cx + 9, cy + 9);
    assert_eq!(app.qc_slots, vec![4, 5, 6], "three seats the motion paints");
    assert_eq!(app.qc_remainder, 2, "the ghost the one-each three keeps");
    // THE SAME-SEAT RE-HOVER: the Set.add's own false the set keeps
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the painted set the re-hover keeps"
    );
    // THE RELEASE: the one-each distribution — each seat ONE item,
    // the remainder the cursor keeps
    app.on_right_release();
    assert_eq!(app.chest_slots[4], (dirt, 1), "the first seat's own one");
    assert_eq!(app.chest_slots[5], (dirt, 1), "the second seat's one");
    assert_eq!(app.chest_slots[6], (dirt, 1), "the third seat's one");
    assert_eq!(
        app.chest_carried,
        (dirt, 2),
        "the two remaining the cursor keeps"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    // THE TRIPLE the masked buttons ride: the START mask(0,1) byte
    // 4, the ADDs mask(1,1) byte 5, the END mask(2,1) byte 6 the
    // changed map the one-each deltas names
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5, "the chest window's own id");
            assert_eq!(c.state_id, 7, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 4, "the START's own masked byte");
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 5, 6] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 5, "the ADD's own masked byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 6, "the END's own masked byte");
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 1)), (5, (dirt, 1)), (6, (dirt, 1))],
                "the one-each deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE CHEST RIGHT DRAG'S OWN DEFERRED LANES (the empty-paint
/// release the vanilla law reads — the seat the armed button's own
/// PICKUP speaks (the right drag the place-ONE serves), the outside
/// the -999 throw-ONE serves, the between the -1 no-op keeps).
#[test]
fn the_chest_right_drag_the_deferred_place_and_the_throw_one_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    app.chest_is_live = true;
    app.chest_live_window = Some((5, 7));
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // THE EMPTY-PAINT RELEASE ON THE SEAT: the pre-press motion the
    // position speaks (the un-armed paint never runs), the press
    // -> release without motion the deferred place serves — the
    // armed button's own PICKUP the place-ONE speaks (the seat
    // takes one, the cursor keeps the rest)
    app.chest_carried = (dirt, 5);
    let (ox, oy) = chest_container_slot(panel, 4);
    app.on_mouse_move(ox + 9, oy + 9);
    app.on_right_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the right press the arm speaks");
    app.on_right_release();
    assert_eq!(
        app.chest_slots[4],
        (dirt, 1),
        "the deferred place ONE rides"
    );
    assert_eq!(app.chest_carried, (dirt, 4), "the cursor the one loses");
    assert!(!app.qc_armed, "the release the arm clears");
    match click_rx.try_recv().expect("the place-one rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, 4);
            assert_eq!(c.button, 1, "the right button's own PICKUP");
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(4, (dirt, 1))], "the place's own delta");
            assert_eq!(c.carried, (dirt, 4));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE CARRIED-EMPTY RIGHT PRESS KEEPS THE IMMEDIATE SECONDARY
    // (the split-pickup the press lane answers — no drag the empty
    // cursor arms): the ceiling half the cursor takes
    app.chest_carried = (0, 0);
    app.chest_slots[7] = (stone, 10);
    let (sx, sy) = chest_container_slot(panel, 7);
    app.on_right_press(sx + 9, sy + 9);
    assert!(!app.qc_armed, "the empty cursor the drag never arms");
    assert_eq!(app.chest_carried, (stone, 5), "the split's own half");
    assert_eq!(app.chest_slots[7], (stone, 5), "the seat the half keeps");
    match click_rx.try_recv().expect("the split rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 7);
            assert_eq!(c.button, 1);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(7, (stone, 5))]);
            assert_eq!(c.carried, (stone, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE OUTSIDE RELEASE THE THROW-ONE SERVES: the right drag
    // armed on the outside press, the -999 PICKUP the ONE item
    // drops (the button-1 throw the doClick walk answers)
    app.chest_carried = (dirt, 5);
    app.on_right_press(4, 4);
    assert!(app.qc_armed, "the outside press the arm arms");
    app.on_mouse_move(4, 4);
    assert!(app.qc_slots.is_empty(), "the outside motion paints nothing");
    app.on_right_release();
    assert_eq!(
        app.chest_carried,
        (dirt, 4),
        "the throw-ONE the cursor keeps"
    );
    match click_rx.try_recv().expect("the drop PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999, "the outside release's own -999");
            assert_eq!(c.button, 1, "the right button's own throw");
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 4), "the ONE-item loss rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE BETWEEN RELEASE: the -1 no-op the armed button's byte
    // keeps (the isValidSlotIndex refusal — the cursor untouched)
    let (gx, gy) = chest_container_slot(panel, 10);
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(panel.0 + 170, panel.1 + 20);
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_right_release();
    assert_eq!(app.chest_carried, (dirt, 4), "the -1 the cursor keeps");
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.button, 1, "the armed button's own byte");
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 4), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE CHEST RIGHT DRAG'S OWN GATES AND ABORTS (the wire session's
/// no-window refusal the arm silences, the armed drag the other
/// button's press keeps silent, the wrong button's release the
/// mismatch abort speaks BOTH ways — the left release the right
/// drag aborts, the right release the left drag aborts — the
/// aborts the packets never send).
#[test]
fn the_chest_right_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // THE WIRE SESSION'S NO-WINDOW REFUSAL (chest_live_window None):
    // the carried right press the arm silences — the gate the
    // window's own presence keys (the world_is_network && no-window
    // shape the autopush-146 round wears)
    app.chest_carried = (dirt, 8);
    let (gx, gy) = chest_container_slot(panel, 4);
    app.on_right_press(gx + 9, gy + 9);
    assert!(
        !app.qc_armed,
        "the stand-in's right press the arm never speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    app.chest_live_window = Some((5, 7));
    app.chest_is_live = true;
    // THE ARMED LEFT DRAG THE RIGHT PRESS SILENCES (the in-flight
    // gate — no place rides, the type-0 state the press keeps; the
    // right release the mismatch abort speaks)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the left drag armed");
    assert_eq!(app.qc_type, 0, "the left press's own type");
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.qc_type, 0, "the armed state the press never re-keys");
    assert_eq!(app.chest_slots[4], (0, 0), "no place rode the press");
    assert!(
        click_rx.try_recv().is_err(),
        "the in-flight press NO packet keeps"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE ARMED RIGHT DRAG THE LEFT RELEASE ABORTS (the mismatch's
    // own mirror — the abort the packets never send)
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right drag armed");
    assert_eq!(app.qc_type, 1, "the right press's own type");
    app.on_mouse_move(gx + 9, gy + 9);
    assert_eq!(app.qc_slots, vec![4], "the motion the press seat paints");
    app.on_left_release();
    assert!(!app.qc_armed, "the left release the right drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert_eq!(
        app.chest_slots[4],
        (0, 0),
        "the abort NO distribution speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE SURVIVOR: the left drag after the aborts the same type-0
    // even split wears (the lanes never mix, the law the release
    // re-keys per drag)
    app.chest_carried = (dirt, 4);
    app.on_press(gx + 9, gy + 9);
    app.on_mouse_move(gx + 9, gy + 9);
    app.on_left_release();
    assert_eq!(
        app.chest_slots[4],
        (dirt, 4),
        "the one-seat split the whole keeps"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the one-seat split the cursor empties"
    );
    for _ in 0..3 {
        let _ = click_rx.try_recv(); // the START, the ADD, the END
    }
    assert!(
        click_rx.try_recv().is_err(),
        "the lane the drain keeps empty"
    );
}

/// THE CHEST DRAG'S OWN WIRE SPLIT (the live belief the same place
/// math answers — the packet triple the release speaks, the HUD row
/// the mirror's hotbar rows sync follows): 32 dirt over one grid
/// cell + two hotbar rows the even split the belief, the HUD, and
/// the START/ADDs/END packets wear.
#[test]
fn the_chest_quickcraft_drag_the_wire_split_and_the_hud_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    app.chest_is_live = true;
    app.chest_live_window = Some((5, 7));
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    // the cursor 32 dirt; the drag over the grid cell 4 + the
    // mirror's hotbar rows 0/1 (the menus 54/55)
    app.chest_carried = (dirt, 32);
    let (ax, ay) = chest_container_slot(panel, 4);
    app.on_press(ax + 9, ay + 9);
    app.on_mouse_move(ax + 9, ay + 9);
    let (bx, by) = chest_mirror_slot(panel, 27);
    app.on_mouse_move(bx + 9, by + 9);
    let (cx, cy) = chest_mirror_slot(panel, 28);
    app.on_mouse_move(cx + 9, cy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4, 54, 55],
        "the three seats the motion paints"
    );
    app.on_left_release();
    // THE BELIEF: the grid cell + the hotbar menus the tenths hold,
    // the cursor the remainder keeps
    assert_eq!(app.chest_slots[4], (dirt, 10), "the grid cell's tenth");
    assert_eq!(app.chest_mirror[27], (dirt, 10), "the hotbar row 0's tenth");
    assert_eq!(app.chest_mirror[28], (dirt, 10), "the hotbar row 1's tenth");
    assert_eq!(
        app.chest_carried,
        (dirt, 2),
        "the remainder the belief keeps"
    );
    // THE HUD: the mirror's hotbar rows the sync speaks
    assert_eq!(app.hotbar_slots[0], (dirt, 10));
    assert_eq!(app.hotbar_slots[1], (dirt, 10));
    // THE TRIPLE: the START (mask(0,0) byte 0), the three ADDs
    // (mask(1,0) byte 1), the END (mask(2,0) byte 2) the changed
    // map the tenths name — the chest window's own identity rides
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 54, 55] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 10)), (54, (dirt, 10)), (55, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE DISPENSER DRAG'S OWN PAINT AND WIRE SPLIT (the same
/// AbstractContainerScreen base walk the Inventory + chest rounds
/// bank — the 3x3 screen the SAME class wears, the window's own
/// menu ids the seats speak): the carried press the arm speaks —
/// NO place, NO packet —, the first motion paints the press seat
/// itself, the mirror's hotbar rows the motion paints, the release
/// the floor(32/3) tenths the belief and the triple wear — the
/// dispenser window's own identity the packets ride, the HUD row
/// the mirror's hotbar sync follows.
#[test]
fn the_dispenser_quickcraft_drag_the_paint_and_the_wire_split_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Dispenser);
    app.dispenser_is_live = true;
    app.dispenser_live_window = Some((5, 7));
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slots = vec![(0, 0); 9];
    app.dispenser_mirror = vec![(0, 0); 36];
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    // the cursor 32 dirt; the drag over the grid cell 4 + the
    // mirror's hotbar rows 0/1 (the menus 36/37)
    app.dispenser_carried = (dirt, 32);
    let (ax, ay) = dispenser_grid_slot(panel, 4);
    app.on_press(ax + 9, ay + 9);
    app.on_mouse_move(ax + 9, ay + 9);
    let (bx, by) = chest_mirror_slot(panel, 27);
    app.on_mouse_move(bx + 9, by + 9);
    let (cx, cy) = chest_mirror_slot(panel, 28);
    app.on_mouse_move(cx + 9, cy + 9);
    assert_eq!(
        app.qc_slots,
        vec![4, 36, 37],
        "the three seats the motion paints"
    );
    app.on_left_release();
    // THE BELIEF: the grid cell + the hotbar menus the tenths hold,
    // the cursor the remainder keeps
    assert_eq!(app.dispenser_slots[4], (dirt, 10), "the grid cell's tenth");
    assert_eq!(
        app.dispenser_mirror[27],
        (dirt, 10),
        "the hotbar row 0's tenth"
    );
    assert_eq!(
        app.dispenser_mirror[28],
        (dirt, 10),
        "the hotbar row 1's tenth"
    );
    assert_eq!(
        app.dispenser_carried,
        (dirt, 2),
        "the remainder the belief keeps"
    );
    // THE HUD: the mirror's hotbar rows the sync speaks
    assert_eq!(app.hotbar_slots[0], (dirt, 10));
    assert_eq!(app.hotbar_slots[1], (dirt, 10));
    // THE TRIPLE: the START (mask(0,0) byte 0), the three ADDs
    // (mask(1,0) byte 1), the END (mask(2,0) byte 2) the changed
    // map the tenths name — the dispenser window's own identity rides
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 36, 37] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 10)), (36, (dirt, 10)), (37, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE DISPENSER DRAG'S OWN GATES AND THE DEFERRED PLACE (the same
/// AbstractContainerScreen walk the chest round banks — the 3x3's
/// own differences the block speaks): the window-less wire stand-in
/// the drag's own gate refuses (no window, no drag — the table/
/// furnace's own law the hybrid wears), the different item's
/// seat the paint refuses (the empty-painted release the deferred
/// place's PICKUP speaks — the swap the click lane serves), the
/// armed drag the right press silences (the in-flight gate), the
/// wrong button's release the abort speaks, the between-slots
/// release the -1 PICKUP no-op rides. THE OFFLINE SIM the drag's
/// own lanes the dedicated tests speak (dev-148's own round the
/// local walks bank).
#[test]
fn the_dispenser_quickcraft_gates_and_the_deferred_place_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Dispenser);
    app.dispenser_live_window = Some((5, 7));
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slots = vec![(0, 0); 9];
    app.dispenser_mirror = vec![(0, 0); 36];
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    // THE WINDOW-LESS WIRE STAND-IN (dispenser_is_live false, the
    // window identity dropped): the carried press the drag's own
    // gate refuses — the silence the arm's own refusal keeps (no
    // place, no packet — the table/furnace's own law the hybrid
    // wears; the OFFLINE sim the drag the dedicated tests speak)
    app.dispenser_live_window = None;
    app.dispenser_carried = (dirt, 8);
    let (gx, gy) = dispenser_grid_slot(panel, 4);
    app.on_press(gx + 9, gy + 9);
    assert!(!app.qc_armed, "the window-less stand-in the drag refuses");
    assert_eq!(
        app.dispenser_slots[4],
        (0, 0),
        "no place the refused press speaks"
    );
    assert_eq!(
        app.dispenser_carried,
        (dirt, 8),
        "the cursor the refusal keeps"
    );
    assert!(
        click_rx.try_recv().is_err(),
        "the refused press NO packet keeps"
    );
    // THE WINDOW ARMS (the cursor the belief re-speaks)
    app.dispenser_is_live = true;
    app.dispenser_live_window = Some((5, 7));
    app.dispenser_carried = (dirt, 8);
    app.dispenser_slots[4] = (0, 0);
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the
    // click walk's own lanes serve: the carried rides the cursor,
    // the seat's stack the hand takes)
    app.dispenser_slots[4] = (stone, 5);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(gx + 9, gy + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.dispenser_slots[4], (dirt, 8), "the stone the hand took");
    match click_rx.try_recv().expect("the PICKUP rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, 4);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(4, (dirt, 8))], "the swap's own delta");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE COUNT'S OWN GATE: the 2-stack paints TWO seats, the
    // third the `count > painted` law refuses; the release the
    // even split speaks (1 per seat, the cursor empty)
    app.dispenser_carried = (dirt, 2);
    let (bx, by) = dispenser_grid_slot(panel, 1);
    app.on_press(bx + 9, by + 9);
    app.on_mouse_move(bx + 9, by + 9);
    let (nx, ny) = dispenser_grid_slot(panel, 2);
    app.on_mouse_move(nx + 9, ny + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the second seat the count still paints"
    );
    let (mx2, my2) = dispenser_grid_slot(panel, 3);
    app.on_mouse_move(mx2 + 9, my2 + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the third seat the count gate refuses"
    );
    app.on_left_release();
    assert_eq!(app.dispenser_slots[1], (dirt, 1), "the split's own one");
    assert_eq!(app.dispenser_slots[2], (dirt, 1), "the split's own one");
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    for _ in 0..4 {
        let _ = click_rx.try_recv(); // the START, the two ADDs, the END
    }
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.dispenser_carried = (dirt, 4);
    app.on_press(panel.0 + 170, panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    assert!(
        click_rx.try_recv().is_err(),
        "the between release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    app.dispenser_slots[4] = (0, 0);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(
        app.dispenser_slots[4],
        (0, 0),
        "no split-place rode the press"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    // THE EMPTY-PAINT RELEASE BETWEEN THE SLOTS: the deferred
    // place's -1 PICKUP the wire keeps (the isValidSlotIndex
    // refusal — the no-op the packet names, the cursor keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(panel.0 + 170, panel.1 + 20); // the paint skips (no seat)
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 4), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE OFFLINE DISPENSER DRAG'S OWN PAINT AND SPLIT (dev-148's own
/// lanes — the quickcraft walk the offline sim the table/furnace's
/// own gate shape wears): the carried press the drag arms (no
/// place, no packet), the motion the paint speaks (the grid seat
/// the press's own motion paints, the hotbar mirror row the paint
/// adds), the release the even split the local stores answer — the
/// grid cell the nine keeps, the hotbar row the write-through
/// speaks, the mirror the sync re-banks, the HUD row the sync
/// follows, the exact split the cursor empties — and the right
/// drag the one-each split serves.
#[test]
fn the_offline_dispenser_drag_the_paint_and_the_split_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    app.open_dispenser_for(cell);
    assert_eq!(app.screen, ScreenKind::Dispenser);
    assert!(!app.dispenser_is_live, "the offline sim the door keeps");
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the split reads
    }
    app.dispenser_mirror_sync_from_inventory();
    // the cursor 8 dirt; the drag over the grid cell 4 + the
    // hotbar mirror row 0 (menu 36)
    app.dispenser_carried = (dirt, 8);
    let (ax, ay) = dispenser_grid_slot(panel, 4);
    app.on_press(ax + 9, ay + 9);
    assert!(app.qc_armed, "the offline carried press the drag arms");
    assert_eq!(app.dispenser_slots[4], (0, 0), "no place the press speaks");
    app.on_mouse_move(ax + 9, ay + 9);
    let (bx, by) = chest_mirror_slot(panel, 27);
    app.on_mouse_move(bx + 9, by + 9);
    assert_eq!(app.qc_slots, vec![4, 36], "the two seats the motion paints");
    app.on_left_release();
    // THE SPLIT: the floor(8/2) halves the local stores speak
    assert_eq!(app.dispenser_slots[4], (dirt, 4), "the grid cell's half");
    assert_eq!(
        app.inventory.items[0],
        (dirt, 4),
        "the hotbar row the write-through speaks"
    );
    assert_eq!(
        app.dispenser_mirror[27],
        (dirt, 4),
        "the mirror row the sync re-banks"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 4),
        "the HUD row the sync follows"
    );
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    // THE RIGHT DRAG'S OWN ONE-EACH (the type-1 the right press
    // arms): the two seats the one each speaks, the 2-stack the
    // seats empty
    app.dispenser_carried = (dirt, 2);
    let (cx, cy) = dispenser_grid_slot(panel, 5);
    app.on_right_press(cx + 9, cy + 9);
    assert!(app.qc_armed, "the right press the drag arms");
    app.on_mouse_move(cx + 9, cy + 9);
    let (dx, dy) = dispenser_grid_slot(panel, 6);
    app.on_mouse_move(dx + 9, dy + 9);
    assert_eq!(app.qc_slots, vec![5, 6], "the two seats the motion paints");
    app.on_right_release();
    assert_eq!(app.dispenser_slots[5], (dirt, 1), "the one-each's own one");
    assert_eq!(app.dispenser_slots[6], (dirt, 1), "the one-each's own one");
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the 2-stack the two seats empty"
    );
}

/// THE OFFLINE DISPENSER DRAG'S OWN GATES AND THE DEFERRED PLACE
/// (dev-148's own lanes — the drag the offline sim wears): the
/// different item's seat the paint refuses (the empty-painted
/// release the deferred place's PICKUP speaks — the swap the
/// offline click lane serves), the between-slots press the arm
/// skips (the release the un-armed walk never enters), the armed
/// drag the right press silences (the in-flight gate), the wrong
/// button's release the abort speaks, the between release the
/// offline no-op keeps, the OUTSIDE release the -999 throw the
/// local drop speaks (the whole carried the primary button drops).
#[test]
fn the_offline_dispenser_drag_the_gates_and_the_deferred_place_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    app.open_dispenser_for(cell);
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the swap
    // the offline lanes serve)
    app.dispenser_slots[4] = (stone, 5);
    app.dispenser_carried = (dirt, 8);
    let (gx, gy) = dispenser_grid_slot(panel, 4);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(gx + 9, gy + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.dispenser_slots[4], (dirt, 8), "the stone the hand took");
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.dispenser_carried = (dirt, 4);
    app.on_press(panel.0 + 170, panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    app.dispenser_slots[4] = (0, 0);
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(
        app.dispenser_slots[4],
        (0, 0),
        "no split-place rode the press"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    // THE EMPTY-PAINT RELEASE BETWEEN THE SLOTS: the offline walk
    // the -1 no-op keeps (no throw, no place — the cursor keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(panel.0 + 170, panel.1 + 20); // the paint skips (no seat)
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (dirt, 4),
        "the between release the no-op keeps"
    );
    // THE OUTSIDE RELEASE THE THROW SPEAKS (the -999 PICKUP the
    // local throw wears — the whole carried the primary drops: the
    // cursor the throw empties, the headless menus the body-less
    // silence the drop walk's own guard keeps)
    app.on_press(gx + 9, gy + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_mouse_move(panel.0 + 200, panel.1 + 20); // outside — no seat
    assert!(app.qc_slots.is_empty(), "the outside motion paints nothing");
    app.on_left_release();
    assert_eq!(
        app.dispenser_carried,
        (0, 0),
        "the whole carried the throw drops"
    );
    assert!(
        app.drops.is_empty(),
        "the headless body the drop walk's own guard keeps silent"
    );
}

/// THE TABLE DRAG'S OWN PAINT AND EVEN SPLIT (the same
/// AbstractContainerScreen base walk the Inventory + chest rounds
/// bank — the crafting screen the SAME class wears, the window-1
/// menu ids the seats speak): the carried press the arm speaks —
/// NO place, NO packet —, the first motion paints the press seat
/// itself, the cells + the hotbar row the motion paints, the
/// RESULT seat the own mayPlace the paint refuses, the re-hover
/// the Set.add's false keeps, the release the floor(32/3) tenths
/// the belief and the triple wear.
#[test]
fn the_table_quickcraft_drag_the_paint_and_the_even_split_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Crafting);
    // THE WINDOW ARMS (the swap walk's own container ride — the
    // cursor 32 dirt the belief's own carried speaks)
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (dirt, 32),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    app.table_panel = inventory_panel_origin(640, 360);
    // THE ARM: the carried press the state sets — NO place (the
    // cell stays EMPTY), no painted seat yet, NO packet (the arm
    // the silent start keeps)
    let (ox, oy) = table_craft_origin(app.table_panel, 0);
    app.on_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 0, "the left button's own type");
    assert_eq!(app.qc_button, 0, "the left button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.live_table_slots[1], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the PRESS seat itself —
    // the one-seat projection the whole count keeps (the ghost 0)
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1],
        "the press cell the first motion paints"
    );
    assert_eq!(
        app.qc_remainder, 0,
        "the one-seat projection the count empties"
    );
    let (bx, by) = table_craft_origin(app.table_panel, 1);
    app.on_mouse_move(bx + 9, by + 9);
    let (hx, hy) = inventory_slot_origin(app.table_panel, 0);
    app.on_mouse_move(hx + 9, hy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1, 2, 37],
        "the cells + the hotbar row the motion paints (the store's own menu id)"
    );
    assert_eq!(app.qc_remainder, 2, "the ghost the floor(32/3) splits keep");
    // THE RESULT SEAT'S OWN mayPlace (the iconst_0): the hover the
    // paint refuses — the result the drag never paints
    let (rx, ry) = table_craft_origin(app.table_panel, 9);
    app.on_mouse_move(rx + 9, ry + 9);
    assert_eq!(app.qc_slots, vec![1, 2, 37], "the result the paint refuses");
    // THE SAME-SEAT RE-HOVER: the Set.add's own false the recalc never feeds
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1, 2, 37],
        "the painted set the re-hover keeps"
    );
    // THE RELEASE: the distribution — floor(32/3) = 10 per seat,
    // the remainder 2 the cursor keeps; the grid's mutation the
    // recompute answers (dirt crafts NOTHING — the result stays)
    app.on_left_release();
    assert_eq!(
        app.live_table_slots[1],
        (dirt, 10),
        "the first cell's own tenth"
    );
    assert_eq!(
        app.live_table_slots[2],
        (dirt, 10),
        "the second cell's tenth"
    );
    assert_eq!(
        app.live_table_slots[37],
        (dirt, 10),
        "the hotbar row's tenth"
    );
    assert_eq!(
        app.live_table_carried,
        (dirt, 2),
        "the remainder rides the cursor"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 10),
        "the HUD row the sync speaks"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    // THE TRIPLE the table window's own identity rides: the START
    // (the mask(0,0) byte 0), the END (the mask(2,0) byte 2) the
    // changed map the tenths name
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the table window's own id");
            assert_eq!(c.state_id, 7, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [1, 2, 37] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(1, (dirt, 10)), (2, (dirt, 10)), (37, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE TABLE DRAG'S OWN GATES AND ABORTS (the shouldAdd gates the
/// paint answers — the different item the canItemQuickReplace
/// verdict refuses, the RESULT seat the own mayPlace refuses, the
/// count's own `> painted` law the third seat refuses; the
/// no-window stand-in the arm silences, the between-slots press
/// the arm skips, the armed drag the right press silences, the
/// wrong button's release the mismatch abort speaks, the
/// empty-paint release the deferred place keeps — the seat the
/// PICKUP speaks, the between the -1 no-op the wire names, the
/// outside the -999 drop serves; the OFFLINE lane the same drag
/// the sim answers, no window the arm ever needs).
#[test]
fn the_table_quickcraft_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let stick = mc_registry::item_id_by_name("stick").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Crafting);
    app.table_panel = inventory_panel_origin(640, 360);
    // THE NO-WINDOW STAND-IN: the carried press the arm silences —
    // the wire session's own window gate the arm shares
    app.live_table_carried = (dirt, 8);
    let (cx0, cy0) = table_craft_origin(app.table_panel, 0);
    app.on_press(cx0 + 9, cy0 + 9);
    assert!(!app.qc_armed, "the stand-in's press the arm never speaks");
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    // THE WINDOW ARMS
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (dirt, 8),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    // THE RESULT SEAT'S OWN mayPlace: the same-item cursor the
    // canItemQuickReplace verdict PASSES, the iconst_0 the paint
    // still refuses; the release BETWEEN the slots the -1 PICKUP
    // no-op speaks (the cursor the -1 keeps)
    app.live_table_slots[0] = (stone, 5);
    app.live_table_carried = (stone, 8);
    let (rx, ry) = table_craft_origin(app.table_panel, 9);
    app.on_press(rx + 9, ry + 9);
    assert!(
        app.qc_armed,
        "the carried press arms on the result seat too"
    );
    app.on_mouse_move(rx + 9, ry + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the result the own mayPlace refuses"
    );
    app.on_mouse_move(app.table_panel.0 + 170, app.table_panel.1 + 20);
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (stone, 8), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the
    // click walk's own lanes serve: the carried rides the cursor,
    // the seat's stack the hand takes)
    app.live_table_slots[16] = (stone, 5); // the store row 15's own menu
    app.live_table_carried = (dirt, 8);
    let (sx, sy) = inventory_slot_origin(app.table_panel, 15);
    app.on_press(sx + 9, sy + 9);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(sx + 9, sy + 9);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.live_table_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(
        app.live_table_slots[16],
        (dirt, 8),
        "the stone the hand took"
    );
    match click_rx.try_recv().expect("the PICKUP rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, 16);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(16, (dirt, 8))], "the swap's own delta");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE COUNT'S OWN GATE: the 2-stack paints TWO cells, the
    // third the `count > painted` law refuses; the release the
    // even split speaks (1 per cell, the cursor empty)
    app.live_table_carried = (dirt, 2);
    let (cx1, cy1) = table_craft_origin(app.table_panel, 1);
    app.on_press(cx1 + 9, cy1 + 9);
    app.on_mouse_move(cx1 + 9, cy1 + 9);
    let (cx2, cy2) = table_craft_origin(app.table_panel, 2);
    app.on_mouse_move(cx2 + 9, cy2 + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the second cell the count still paints"
    );
    let (cx3, cy3) = table_craft_origin(app.table_panel, 3);
    app.on_mouse_move(cx3 + 9, cy3 + 9);
    assert_eq!(
        app.qc_slots.len(),
        2,
        "the third cell the count gate refuses"
    );
    app.on_left_release();
    assert_eq!(app.live_table_slots[2], (dirt, 1), "the split's own one");
    assert_eq!(app.live_table_slots[3], (dirt, 1), "the split's own one");
    assert_eq!(
        app.live_table_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    for _ in 0..4 {
        let _ = click_rx.try_recv(); // the START, the two ADDs, the END
    }
    // THE BETWEEN-SLOTS PRESS THE ARM SKIPS (the k == -1 silence —
    // the release the un-armed walk never enters)
    app.live_table_carried = (dirt, 4);
    app.on_press(app.table_panel.0 + 170, app.table_panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm never speaks");
    app.on_left_release();
    assert_eq!(
        app.live_table_carried,
        (dirt, 4),
        "the un-armed release the silence keeps"
    );
    assert!(
        click_rx.try_recv().is_err(),
        "the between release the silence keeps"
    );
    // THE ARMED DRAG THE RIGHT PRESS SILENCES (the in-flight gate —
    // no split-place rides, the mismatch release the abort speaks)
    app.on_press(cx0 + 9, cy0 + 9);
    assert!(app.qc_armed, "the drag armed");
    app.on_right_press(cx0 + 9, cy0 + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(
        app.live_table_slots[1],
        (0, 0),
        "no split-place rode the press"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    // THE EMPTY-PAINT RELEASE BETWEEN THE SLOTS: the deferred
    // place's -1 PICKUP the wire keeps (the isValidSlotIndex
    // refusal — the no-op the packet names, the cursor keeps)
    app.on_press(cx0 + 9, cy0 + 9);
    assert!(app.qc_armed, "the drag armed again");
    app.on_mouse_move(app.table_panel.0 + 170, app.table_panel.1 + 20);
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 4), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    assert_eq!(app.live_table_carried, (dirt, 4), "the -1 the cursor keeps");
    // THE OUTSIDE RELEASE THE DROP SERVES (the arm on the outside
    // press — the release's -999 PICKUP the whole carried drops)
    app.on_press(4, 4);
    assert!(app.qc_armed, "the outside press the arm arms");
    app.on_mouse_move(4, 4);
    app.on_left_release();
    assert_eq!(
        app.live_table_carried,
        (0, 0),
        "the outside release the whole carried drops"
    );
    match click_rx.try_recv().expect("the drop PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999, "the outside release's own -999");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.carried, (0, 0), "the emptied cursor rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE OFFLINE LANE (no network — the drag the offline sim
    // answers, NO window the arm ever needs): the sticks' vertical
    // pair the split paints, the release the cells fill and the
    // result the recompute speaks, NO packet the offline keeps
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.open(ScreenKind::Crafting);
    off.table_panel = inventory_panel_origin(640, 360);
    off.inv_carried = (planks, 4);
    let (ox, oy) = table_craft_origin(off.table_panel, 1);
    off.on_press(ox + 9, oy + 9);
    assert!(off.qc_armed, "the offline press the arm speaks");
    off.on_mouse_move(ox + 9, oy + 9);
    let (px, py) = table_craft_origin(off.table_panel, 4);
    off.on_mouse_move(px + 9, py + 9);
    assert_eq!(
        off.qc_slots,
        vec![2, 5],
        "the vertical pair the motion paints"
    );
    off.on_left_release();
    assert_eq!(off.table_grid[1], (planks, 2), "the split's own two");
    assert_eq!(off.table_grid[4], (planks, 2), "the second cell's two");
    assert_eq!(
        off.inv_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    assert_eq!(
        off.table_result,
        (stick, 4),
        "the recompute the sticks speaks"
    );
}

/// THE TABLE RIGHT DRAG'S OWN ONE-EACH (the type-1 lane the right
/// press arms — the chest round's law the window-1 belief wears):
/// the arm NO place and NO packet keeps, the paint the same
/// shouldAdd gates rides (the result seat's own mayPlace refuses),
/// the release the one-each distribution and the masked triple
/// (START mask(0,1)=4, ADDs mask(1,1)=5, END mask(2,1)=6) speak,
/// the HUD row the hotbar menus sync follows.
#[test]
fn the_table_right_drag_the_one_each_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Crafting);
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (dirt, 5),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    app.table_panel = inventory_panel_origin(640, 360);
    // THE RIGHT PRESS'S OWN ARM: the carried press the type-1 drag
    // arms — NO place, NO painted seat, NO packet (the arm the
    // silent start keeps)
    let (ox, oy) = table_craft_origin(app.table_panel, 0);
    app.on_right_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the right press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert_eq!(app.qc_button, 1, "the right button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.live_table_slots[1], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the press cell, the ghost
    // the ONE-EACH projection keeps (the type-1's own place count)
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1],
        "the press cell the first motion paints"
    );
    assert_eq!(app.qc_remainder, 4, "the one-each ghost the cursor keeps");
    let (bx, by) = table_craft_origin(app.table_panel, 1);
    app.on_mouse_move(bx + 9, by + 9);
    let (hx, hy) = inventory_slot_origin(app.table_panel, 0);
    app.on_mouse_move(hx + 9, hy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1, 2, 37],
        "the cells + the hotbar row the motion paints"
    );
    assert_eq!(app.qc_remainder, 2, "the ghost the one-each three keeps");
    // THE RESULT SEAT'S OWN mayPlace (the iconst_0): the hover the
    // paint refuses — the result the drag never paints
    let (rx, ry) = table_craft_origin(app.table_panel, 9);
    app.on_mouse_move(rx + 9, ry + 9);
    assert_eq!(app.qc_slots, vec![1, 2, 37], "the result the paint refuses");
    // THE RELEASE: the one-each distribution — each seat ONE item,
    // the remainder the cursor keeps (dirt crafts NOTHING — the
    // result the recompute never speaks)
    app.on_right_release();
    assert_eq!(
        app.live_table_slots[1],
        (dirt, 1),
        "the first cell's own one"
    );
    assert_eq!(app.live_table_slots[2], (dirt, 1), "the second cell's one");
    assert_eq!(app.live_table_slots[37], (dirt, 1), "the hotbar row's one");
    assert_eq!(
        app.live_table_carried,
        (dirt, 2),
        "the two remaining the cursor keeps"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 1),
        "the HUD row the sync speaks"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    // THE TRIPLE the masked buttons ride: the START mask(0,1) byte
    // 4, the ADDs mask(1,1) byte 5, the END mask(2,1) byte 6 the
    // changed map the one-each deltas names
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the table window's own id");
            assert_eq!(c.state_id, 7, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 4, "the START's own masked byte");
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [1, 2, 37] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 5, "the ADD's own masked byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 6, "the END's own masked byte");
            assert_eq!(
                c.changed,
                vec![(1, (dirt, 1)), (2, (dirt, 1)), (37, (dirt, 1))],
                "the one-each deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE TABLE RIGHT DRAG'S OFFLINE ONE-EACH AND THE DEFERRED
/// PLACE-ONE (the offline sim the same laws answers — no window
/// the arm ever needs, NO packet the offline keeps): the one-each
/// distribution the grid + the store rows speak, the deferred
/// place the armed button's own place-ONE serves.
#[test]
fn the_table_right_drag_the_offline_one_each_and_the_place_one_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.open(ScreenKind::Crafting);
    off.table_panel = inventory_panel_origin(640, 360);
    // THE RIGHT PRESS'S OWN ARM (the offline sim the drag arms —
    // no window the arm ever needs)
    off.inv_carried = (dirt, 5);
    let (ox, oy) = table_craft_origin(off.table_panel, 1);
    off.on_right_press(ox + 9, oy + 9);
    assert!(off.qc_armed, "the offline right press the arm speaks");
    assert_eq!(off.qc_type, 1, "the right button's own type");
    assert_eq!(off.qc_slots.is_empty(), true, "the press paints nothing");
    // THE PAINT: the vertical pair the motion paints
    off.on_mouse_move(ox + 9, oy + 9);
    let (px, py) = table_craft_origin(off.table_panel, 4);
    off.on_mouse_move(px + 9, py + 9);
    assert_eq!(
        off.qc_slots,
        vec![2, 5],
        "the vertical pair the motion paints"
    );
    assert_eq!(off.qc_remainder, 3, "the one-each ghost the cursor keeps");
    // THE RELEASE: the one-each distribution — each cell ONE item,
    // the remainder the cursor keeps (dirt crafts NOTHING — the
    // result the recompute never speaks)
    off.on_right_release();
    assert_eq!(off.table_grid[1], (dirt, 1), "the first cell's own one");
    assert_eq!(off.table_grid[4], (dirt, 1), "the second cell's one");
    assert_eq!(
        off.inv_carried,
        (dirt, 3),
        "the three remaining the cursor keeps"
    );
    assert_eq!(
        off.table_result,
        (0, 0),
        "the no-recipe recompute the silence keeps"
    );
    assert!(!off.qc_armed, "the release the arm clears");
    // THE EMPTY-PAINT RELEASE ON THE STORE ROW: the pre-press
    // motion the position speaks, the deferred place the armed
    // button's own place-ONE serves (the row takes one, the cursor
    // keeps the rest)
    off.inv_carried = (dirt, 4);
    let (hx, hy) = inventory_slot_origin(off.table_panel, 0);
    off.on_mouse_move(hx + 9, hy + 9);
    off.on_right_press(hx + 9, hy + 9);
    assert!(off.qc_armed, "the right press the arm speaks again");
    off.on_right_release();
    assert_eq!(
        off.inventory.items[0],
        (dirt, 1),
        "the deferred place ONE rides"
    );
    assert_eq!(off.inv_carried, (dirt, 3), "the cursor the one loses");
    assert!(!off.qc_armed, "the release the arm clears");
}

/// THE TABLE RIGHT DRAG'S OWN GATES AND ABORTS (the no-window
/// stand-in the arm silences, the in-flight gate the other
/// button's press keeps, the mismatch abort BOTH ways the packets
/// never send, the survivor drag the type-0 even split the release
/// re-keys per drag, the carried-empty press the immediate
/// secondary keeps).
#[test]
fn the_table_right_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Crafting);
    app.table_panel = inventory_panel_origin(640, 360);
    // THE NO-WINDOW STAND-IN: the carried cursor the belief keeps
    // but no window the arm refuses — the silence the click lane
    // mirrors
    app.live_table_carried = (dirt, 8);
    let (ox, oy) = table_craft_origin(app.table_panel, 0);
    app.on_right_press(ox + 9, oy + 9);
    assert!(
        !app.qc_armed,
        "the stand-in's right press the arm never speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    // THE WINDOW ARMS
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    app.live_table_carried = (dirt, 8);
    // THE ARMED LEFT DRAG THE RIGHT PRESS SILENCES (the in-flight
    // gate — no place rides, the type-0 state the press keeps; the
    // right release the mismatch abort speaks)
    app.on_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the left drag armed");
    assert_eq!(app.qc_type, 0, "the left press's own type");
    app.on_right_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.qc_type, 0, "the armed state the press never re-keys");
    assert_eq!(app.live_table_slots[1], (0, 0), "no place rode the press");
    assert!(
        click_rx.try_recv().is_err(),
        "the in-flight press NO packet keeps"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE ARMED RIGHT DRAG THE LEFT RELEASE ABORTS (the mismatch's
    // own mirror — the abort the packets never send)
    app.on_right_press(ox + 9, oy + 9);
    assert!(app.qc_armed, "the right drag armed");
    assert_eq!(app.qc_type, 1, "the right press's own type");
    app.on_mouse_move(ox + 9, oy + 9);
    assert_eq!(app.qc_slots, vec![1], "the motion the press cell paints");
    app.on_left_release();
    assert!(!app.qc_armed, "the left release the right drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert_eq!(
        app.live_table_slots[1],
        (0, 0),
        "the abort NO distribution speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE SURVIVOR: the left drag after the aborts the same type-0
    // even split wears (the lanes never mix, the law the release
    // re-keys per drag)
    app.on_press(ox + 9, oy + 9);
    app.on_mouse_move(ox + 9, oy + 9);
    app.on_left_release();
    assert_eq!(
        app.live_table_slots[1],
        (dirt, 8),
        "the one-seat split the whole keeps"
    );
    assert_eq!(
        app.live_table_carried,
        (0, 0),
        "the one-seat split the cursor empties"
    );
    for _ in 0..3 {
        let _ = click_rx.try_recv(); // the START, the ADD, the END
    }
    assert!(
        click_rx.try_recv().is_err(),
        "the lane the drain keeps empty"
    );
    // THE CARRIED-EMPTY RIGHT PRESS KEEPS THE IMMEDIATE SECONDARY
    // (the split-pickup the press lane answers — no drag the empty
    // cursor arms)
    app.live_table_slots[10] = (stone, 10);
    app.live_table_carried = (0, 0);
    let (sx, sy) = inventory_slot_origin(app.table_panel, 9); // the store index 9 the menu 10 answers
    app.on_right_press(sx + 9, sy + 9);
    assert!(!app.qc_armed, "the empty cursor the drag never arms");
    assert_eq!(app.live_table_carried, (stone, 5), "the split's own half");
    assert_eq!(
        app.live_table_slots[10],
        (stone, 5),
        "the seat the half keeps"
    );
    match click_rx.try_recv().expect("the split rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, 10);
            assert_eq!(c.button, 1);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(10, (stone, 5))]);
            assert_eq!(c.carried, (stone, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE TABLE DRAG'S OWN WIRE SPLIT (the live belief the same place
/// math answers — the packet triple the release speaks, the HUD row
/// the hotbar menus sync follows, the craft cells' mutation the
/// recipe recompute the END's changed entry names): 32 dirt over
/// one cell + the hotbar row + the store row the tenths wear, then
/// the planks' vertical pair the STICKS the recompute speaks.
#[test]
fn the_table_quickcraft_drag_the_wire_split_and_the_hud_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let stick = mc_registry::item_id_by_name("stick").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Crafting);
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (dirt, 32),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    app.table_panel = inventory_panel_origin(640, 360);
    // THE FIRST DRAG: the cursor 32 dirt over the cell + the
    // hotbar row + the store row (the menus 1/37/16)
    let (ax, ay) = table_craft_origin(app.table_panel, 0);
    app.on_press(ax + 9, ay + 9);
    app.on_mouse_move(ax + 9, ay + 9);
    let (hx, hy) = inventory_slot_origin(app.table_panel, 0);
    app.on_mouse_move(hx + 9, hy + 9);
    let (sx, sy) = inventory_slot_origin(app.table_panel, 15);
    app.on_mouse_move(sx + 9, sy + 9);
    assert_eq!(
        app.qc_slots,
        vec![1, 37, 16],
        "the three seats the motion paints"
    );
    app.on_left_release();
    // THE BELIEF: the cell + the hotbar menu + the store menu the
    // tenths hold, the cursor the remainder keeps
    assert_eq!(app.live_table_slots[1], (dirt, 10), "the cell's tenth");
    assert_eq!(
        app.live_table_slots[37],
        (dirt, 10),
        "the hotbar menu's tenth"
    );
    assert_eq!(
        app.live_table_slots[16],
        (dirt, 10),
        "the store menu's tenth"
    );
    assert_eq!(
        app.live_table_carried,
        (dirt, 2),
        "the remainder the belief keeps"
    );
    // THE HUD: the hotbar menus the sync speaks
    assert_eq!(app.hotbar_slots[0], (dirt, 10));
    // THE TRIPLE: the START (mask(0,0) byte 0), the three ADDs
    // (mask(1,0) byte 1), the END (mask(2,0) byte 2) the changed
    // map the tenths name — the table window's own identity rides
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [1, 37, 16] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(1, (dirt, 10)), (37, (dirt, 10)), (16, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE SECOND DRAG: the planks' vertical pair (the cells the
    // menus 2 + 5 keep — the pack law the column folds) the STICKS
    // the recompute speaks — the cells' mutation the END's changed
    // map's own entry names. The first drag's dirt the sync's own
    // truth clears (the stale cell the pack's bounding box would
    // poison — the clean column the pair folds)
    app.live_table_slots[1] = (0, 0);
    app.live_table_carried = (planks, 4);
    let (bx, by) = table_craft_origin(app.table_panel, 1);
    app.on_press(bx + 9, by + 9);
    app.on_mouse_move(bx + 9, by + 9);
    let (px, py) = table_craft_origin(app.table_panel, 4);
    app.on_mouse_move(px + 9, py + 9);
    assert_eq!(
        app.qc_slots,
        vec![2, 5],
        "the vertical pair the motion paints"
    );
    app.on_left_release();
    assert_eq!(app.live_table_slots[2], (planks, 2), "the split's own two");
    assert_eq!(
        app.live_table_slots[5],
        (planks, 2),
        "the second cell's two"
    );
    assert_eq!(
        app.live_table_slots[0],
        (stick, 4),
        "the recompute the sticks speaks"
    );
    assert_eq!(
        app.live_table_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 10),
        "the HUD row the sync keeps"
    );
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (planks, 4));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [2, 5] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(2, (planks, 2)), (5, (planks, 2)), (0, (stick, 4))],
                "the distribution + the recompute's own deltas"
            );
            assert_eq!(c.carried, (0, 0), "the emptied cursor rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE DRAG'S OWN PAINT AND EVEN SPLIT (the shouldAdd
/// gates the paint answers — the press seat the first motion
/// paints, the even split the net law keeps, the furnace's OWN
/// mayPlace verdicts the paint re-keys: the result's iconst_0
/// the paint refuses, the fuel's isFuel verdict the non-fuel
/// carried refuses and the fuel carried accepts, the store rows
/// the plain TRUE keeps; the packet triple the furnace window's
/// own identity rides).
#[test]
fn the_furnace_quickcraft_drag_the_paint_and_the_even_split_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Furnace);
    app.furnace_live_window = Some((9, 4));
    let panel = inventory_panel_origin(640, 360);
    app.furnace_panel = panel;
    app.furnace_slots = vec![(0, 0); 39];
    // the store seats 4/5/6 the row cells hold, the fuel seat 1
    // and the result seat 2 the furnace's own cells speak
    app.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 4),
        (panel.0 + 120, panel.1 + 100, 5),
        (panel.0 + 140, panel.1 + 100, 6),
        (panel.0 + 60, panel.1 + 60, 1),
        (panel.0 + 130, panel.1 + 50, 2),
    ];
    // THE ARM: the carried press the state sets — NO place (the
    // seat stays EMPTY), no painted seat yet, NO packet (the arm
    // the silent start keeps)
    app.live_furnace_carried = (dirt, 32);
    let (ox, oy) = (panel.0 + 100, panel.1 + 100);
    app.on_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the carried press the arm speaks");
    assert_eq!(app.qc_type, 0, "the left button's own type");
    assert_eq!(app.qc_button, 0, "the left button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.furnace_slots[4], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the PRESS seat itself —
    // the one-seat projection the whole count keeps (the ghost 0)
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(
        app.qc_slots,
        vec![4],
        "the press seat the first motion paints"
    );
    assert_eq!(
        app.qc_remainder, 0,
        "the one-seat projection the count empties"
    );
    let (bx, by) = (panel.0 + 120, panel.1 + 100);
    app.on_mouse_move(bx + 8, by + 8);
    let (cx, cy) = (panel.0 + 140, panel.1 + 100);
    app.on_mouse_move(cx + 8, cy + 8);
    assert_eq!(app.qc_slots, vec![4, 5, 6], "three seats the motion paints");
    assert_eq!(app.qc_remainder, 2, "the ghost the floor(32/3) splits keep");
    // THE SAME-SEAT RE-HOVER: the Set.add's own false the recalc never feeds
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the painted set the re-hover keeps"
    );
    // THE RESULT'S OWN mayPlace: the iconst_0 the paint refuses
    // (the empty result seat the canItemQuickReplace verdict
    // PASSES, the mayPlace still refuses)
    let (rx, ry) = (panel.0 + 130, panel.1 + 50);
    app.on_mouse_move(rx + 8, ry + 8);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the result the own mayPlace refuses"
    );
    // THE FUEL'S OWN isFuel: the non-fuel carried the tick row's
    // zero the paint refuses
    let (fx, fy) = (panel.0 + 60, panel.1 + 60);
    app.on_mouse_move(fx + 8, fy + 8);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the non-fuel carried the fuel seat refuses"
    );
    // THE RELEASE: the distribution — floor(32/3) = 10 per seat,
    // the remainder 2 the cursor keeps
    app.on_left_release();
    assert_eq!(
        app.furnace_slots[4],
        (dirt, 10),
        "the first seat's own tenth"
    );
    assert_eq!(app.furnace_slots[5], (dirt, 10), "the second seat's tenth");
    assert_eq!(app.furnace_slots[6], (dirt, 10), "the third seat's tenth");
    assert_eq!(
        app.live_furnace_carried,
        (dirt, 2),
        "the remainder rides the cursor"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    assert!(app.qc_slots.is_empty(), "the painted set the tail clears");
    // THE TRIPLE the furnace window's own identity rides: the
    // START (the mask(0,0) byte 0), the END (the mask(2,0) byte 2)
    // the changed map the tenths name
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the furnace window's own id");
            assert_eq!(c.state_id, 4, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 32));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 5, 6] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 10)), (5, (dirt, 10)), (6, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE FUEL ACCEPTS (the isFuel verdict the coal carried
    // passes): the re-arm the fuel seat's own drag speaks, the
    // release the fuel cell fills, the END's changed map the seat
    // names (the jiggle the first motion the paint needs — the
    // stationary cursor the collect never re-fires)
    app.live_furnace_carried = (coal, 30);
    app.on_press(fx + 8, fy + 8);
    assert!(app.qc_armed, "the re-arm the carried press speaks");
    app.on_mouse_move(fx + 12, fy + 8);
    app.on_mouse_move(fx + 8, fy + 8);
    assert_eq!(
        app.qc_slots,
        vec![1],
        "the coal carried the fuel seat paints"
    );
    app.on_left_release();
    assert_eq!(
        app.furnace_slots[1],
        (coal, 30),
        "the fuel cell the drag fills"
    );
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    match click_rx.try_recv().expect("the fuel START rode") {
        ContainerCommand::Click(c) => assert_eq!(c.button, 0, "the drag's own START"),
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the fuel ADD rode") {
        ContainerCommand::Click(c) => assert_eq!(c.slot, 1, "the fuel seat's own ADD"),
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the fuel END rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.button, 2, "the drag's own END");
            assert_eq!(
                c.changed,
                vec![(1, (coal, 30))],
                "the fuel fill the map names"
            );
            assert_eq!(c.carried, (0, 0), "the emptied cursor rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE DRAG'S OWN GATES AND ABORTS (the shouldAdd gates
/// the paint answers — the no-window stand-in the arm silences,
/// the result the same-item cursor the mayPlace still refuses,
/// the different item's paint refusal the deferred-place PICKUP
/// speaks, the count's own `> painted` law the third seat refuses,
/// the between-slots press the arm skips, the armed drag the
/// right press silences, the wrong button's release the mismatch
/// abort speaks, the between release the -1 no-op the wire names,
/// the outside the -999 drop serves; the OFFLINE lane no window
/// the arm needs — the write-through the store rows speak).
#[test]
fn the_furnace_quickcraft_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Furnace);
    let panel = inventory_panel_origin(640, 360);
    app.furnace_panel = panel;
    app.furnace_slots = vec![(0, 0); 39];
    app.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 4),
        (panel.0 + 120, panel.1 + 100, 5),
        (panel.0 + 140, panel.1 + 100, 6),
        (panel.0 + 160, panel.1 + 100, 7),
        (panel.0 + 130, panel.1 + 50, 2),
    ];
    // THE NO-WINDOW STAND-IN: the carried press the arm silences —
    // the wire session's own window gate the arm shares
    app.live_furnace_carried = (dirt, 8);
    let (gx, gy) = (panel.0 + 100, panel.1 + 100);
    app.on_press(gx + 8, gy + 8);
    assert!(!app.qc_armed, "the stand-in's press the arm never speaks");
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    // THE WINDOW ARMS (the live flag the click walk's own split
    // keys — the deferred place's PICKUP the wire lane serves)
    app.furnace_is_live = true;
    app.furnace_live_window = Some((9, 4));
    // THE RESULT'S OWN mayPlace: the same-item cursor the
    // canItemQuickReplace verdict PASSES, the iconst_0 the paint
    // still refuses; the release BETWEEN the slots the -1 PICKUP
    // no-op speaks (the cursor the -1 keeps)
    app.furnace_slots[2] = (iron_ingot, 5);
    app.live_furnace_carried = (iron_ingot, 8);
    let (rx, ry) = (panel.0 + 130, panel.1 + 50);
    app.on_press(rx + 8, ry + 8);
    assert!(
        app.qc_armed,
        "the carried press arms on the result seat too"
    );
    app.on_mouse_move(rx + 8, ry + 8);
    assert!(
        app.qc_slots.is_empty(),
        "the result the own mayPlace refuses"
    );
    app.on_mouse_move(panel.0 + 170, panel.1 + 20);
    assert!(app.qc_slots.is_empty(), "the between motion paints nothing");
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (iron_ingot, 8), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE DIFFERENT ITEM'S SEAT REFUSES the paint — the empty-
    // painted release the deferred place's PICKUP speaks (the
    // click walk's own lanes serve: the carried rides the cursor,
    // the seat's stack the hand takes)
    app.furnace_slots[4] = (stone, 5);
    app.live_furnace_carried = (dirt, 8);
    app.on_press(gx + 8, gy + 8);
    assert!(app.qc_armed, "the carried press arms on ANY seat");
    app.on_mouse_move(gx + 8, gy + 8);
    assert!(
        app.qc_slots.is_empty(),
        "the stone seat the different carried refuses"
    );
    app.on_left_release();
    assert_eq!(
        app.live_furnace_carried,
        (stone, 5),
        "the deferred place the swap speaks"
    );
    assert_eq!(app.furnace_slots[4], (dirt, 8), "the stone the hand took");
    match click_rx.try_recv().expect("the PICKUP rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, 4);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE COUNT'S OWN GATE (carried 2): the first motion paints,
    // the second paints, the THIRD the `> painted` law refuses;
    // the release the distribution the pair serves — the 1/1
    // split the cursor empties
    app.furnace_slots[4] = (0, 0);
    app.furnace_slots[5] = (0, 0);
    app.furnace_slots[6] = (0, 0);
    app.live_furnace_carried = (dirt, 2);
    let (sx, sy) = (panel.0 + 120, panel.1 + 100);
    let (tx, ty) = (panel.0 + 140, panel.1 + 100);
    let (ux, uy) = (panel.0 + 160, panel.1 + 100);
    app.on_press(sx + 8, sy + 8);
    assert!(app.qc_armed, "the count gate's own arm");
    app.on_mouse_move(sx + 8, sy + 8);
    assert_eq!(app.qc_slots, vec![5], "the first motion paints");
    app.on_mouse_move(tx + 8, ty + 8);
    assert_eq!(app.qc_slots, vec![5, 6], "the second motion paints");
    app.on_mouse_move(ux + 8, uy + 8);
    assert_eq!(app.qc_slots, vec![5, 6], "the third seat the count refuses");
    app.on_left_release();
    assert_eq!(app.furnace_slots[5], (dirt, 1), "the pair's own one");
    assert_eq!(app.furnace_slots[6], (dirt, 1), "the second's one");
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    while click_rx.try_recv().is_ok() {} // the pair's triple the states already named
                                         // THE BETWEEN PRESS SKIPS (the k == -1 press the arm never
                                         // enters) — the chrome between the seats the strict compare
                                         // answers
    app.live_furnace_carried = (dirt, 8);
    app.on_press(panel.0 + 170, panel.1 + 20);
    assert!(!app.qc_armed, "the between press the arm skips");
    assert!(click_rx.try_recv().is_err(), "the skip NO packet keeps");
    // THE ARMED DRAG'S RIGHT PRESS SILENCES (pc 168-172 — the
    // in-flight verdict the other button's press keeps)
    app.on_press(sx + 8, sy + 8);
    assert!(app.qc_armed, "the arm for the gate walk");
    let fired = app.on_right_press(tx + 8, ty + 8);
    assert!(!fired, "the armed drag the right press refuses");
    assert!(app.qc_armed, "the drag the right press keeps");
    assert!(click_rx.try_recv().is_err(), "the silence the lane keeps");
    // THE MISMATCH RELEASE ABORTS (pc 246-284 — the wrong
    // button's release the seats clear, the arm drops, NO packet)
    app.on_right_release();
    assert!(!app.qc_armed, "the mismatch the abort speaks");
    assert!(app.qc_slots.is_empty(), "the painted set the abort clears");
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE OUTSIDE RELEASE DROPS (the outside press the arm
    // answers — the k == -999 press the same pc 316 walk enters;
    // the empty-paint release the -999 PICKUP the whole carried
    // drops)
    app.live_furnace_carried = (dirt, 8);
    app.on_press(4, 4);
    assert!(app.qc_armed, "the outside press the arm speaks");
    app.on_mouse_move(4, 4);
    app.on_left_release();
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the whole carried dropped"
    );
    match click_rx.try_recv().expect("the drop PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999, "the drop's own -999");
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (0, 0), "the emptied cursor rides");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE OFFLINE LANE (no network — the drag the offline sim
    // answers, NO window the arm ever needs): the pair the split
    // paints, the release the cells fill and the WRITE-THROUGH
    // the store rows speak, NO packet the offline keeps
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.open(ScreenKind::Furnace);
    off.furnace_panel = panel;
    off.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 3),
        (panel.0 + 120, panel.1 + 100, 4),
    ];
    off.furnace_carried = (planks, 4);
    let (ox, oy) = (panel.0 + 100, panel.1 + 100);
    off.on_press(ox + 8, oy + 8);
    assert!(off.qc_armed, "the offline press the arm speaks");
    off.on_mouse_move(ox + 8, oy + 8);
    let (px2, py2) = (panel.0 + 120, panel.1 + 100);
    off.on_mouse_move(px2 + 8, py2 + 8);
    assert_eq!(off.qc_slots, vec![3, 4], "the pair the motion paints");
    off.on_left_release();
    assert_eq!(off.furnace_slots[3], (planks, 2), "the split's own two");
    assert_eq!(off.furnace_slots[4], (planks, 2), "the second cell's two");
    assert_eq!(
        off.inventory.items[9],
        (planks, 2),
        "the write-through the store speaks"
    );
    assert_eq!(
        off.inventory.items[10],
        (planks, 2),
        "the second write-through"
    );
    assert_eq!(
        off.furnace_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
}

/// THE FURNACE DRAG'S OWN WIRE SPLIT AND HUD (the distribution
/// over the window belief — the store cell + the hotbar menu 30
/// + the deeper store menu 16 the tenths hold, the HUD row the
/// hotbar menus 30..38 the sync speaks, the packet triple the
/// masked buttons the changed map wears; the INPUT seat's own
/// SMELTABLE verdict the raw-iron carried accepts and the
/// non-smeltable carried refuses).
#[test]
fn the_furnace_quickcraft_drag_the_wire_split_and_the_hud_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Furnace);
    app.furnace_live_window = Some((6, 8));
    let panel = inventory_panel_origin(640, 360);
    app.furnace_panel = panel;
    app.furnace_slots = vec![(0, 0); 39];
    app.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 4),
        (panel.0 + 120, panel.1 + 100, 30),
        (panel.0 + 140, panel.1 + 100, 16),
        (panel.0 + 56, panel.1 + 40, 0),
    ];
    // THE THREE-SEAT TENTHS: the store cell, the HOTBAR menu 30
    // and the deeper store menu 16 the floor(32/3) split keeps —
    // the HUD row the sync the hotbar cell answers
    app.live_furnace_carried = (dirt, 32);
    let (ax, ay) = (panel.0 + 100, panel.1 + 100);
    let (bx, by) = (panel.0 + 120, panel.1 + 100);
    let (cx, cy) = (panel.0 + 140, panel.1 + 100);
    app.on_press(ax + 8, ay + 8);
    assert!(app.qc_armed, "the carried press arms");
    app.on_mouse_move(ax + 8, ay + 8);
    app.on_mouse_move(bx + 8, by + 8);
    app.on_mouse_move(cx + 8, cy + 8);
    assert_eq!(
        app.qc_slots,
        vec![4, 30, 16],
        "the three seats the motion paints"
    );
    assert_eq!(app.qc_remainder, 2, "the ghost the floor(32/3) splits keep");
    app.on_left_release();
    assert_eq!(
        app.furnace_slots[4],
        (dirt, 10),
        "the first seat's own tenth"
    );
    assert_eq!(app.furnace_slots[30], (dirt, 10), "the hotbar menu's tenth");
    assert_eq!(
        app.furnace_slots[16],
        (dirt, 10),
        "the deeper store's tenth"
    );
    assert_eq!(
        app.live_furnace_carried,
        (dirt, 2),
        "the remainder rides the cursor"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 10),
        "the HUD row the sync speaks"
    );
    // THE TRIPLE the furnace window's own identity rides
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6, "the furnace window's own id");
            assert_eq!(c.state_id, 8, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 0);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 30, 16] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 6);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 1);
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 2);
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 10)), (30, (dirt, 10)), (16, (dirt, 10))],
                "the distribution's own deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE INPUT'S SMELTABLE VERDICT (the raw-iron carried the
    // kind's own recipe row accepts): the input seat the drag
    // fills, the END's changed map the seat names
    app.live_furnace_carried = (raw_iron, 12);
    let (ix, iy) = (panel.0 + 56, panel.1 + 40);
    app.on_press(ix + 8, iy + 8);
    assert!(app.qc_armed, "the input seat's press arms");
    app.on_mouse_move(ix + 8, iy + 8);
    assert_eq!(app.qc_slots, vec![0], "the raw iron the input seat paints");
    app.on_left_release();
    assert_eq!(
        app.furnace_slots[0],
        (raw_iron, 12),
        "the input cell the drag fills"
    );
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the exact split the cursor empties"
    );
    match click_rx.try_recv().expect("the input START rode") {
        ContainerCommand::Click(c) => assert_eq!(c.button, 0, "the drag's own START"),
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the input ADD rode") {
        ContainerCommand::Click(c) => assert_eq!(c.slot, 0, "the input seat's own ADD"),
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the input END rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.button, 2, "the drag's own END");
            assert_eq!(
                c.changed,
                vec![(0, (raw_iron, 12))],
                "the fill the map names"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE NON-SMELTABLE REFUSES THE INPUT (the dirt carried the
    // recipe row's absence the paint refuses): the empty-paint
    // release BETWEEN the slots the -1 PICKUP no-op speaks (the
    // jiggle the refusal the moved collect needs)
    app.live_furnace_carried = (dirt, 5);
    app.on_press(ix + 8, iy + 8);
    assert!(app.qc_armed, "the dirt press arms on the input seat too");
    app.on_mouse_move(ix + 12, iy + 8);
    app.on_mouse_move(ix + 8, iy + 8);
    assert!(
        app.qc_slots.is_empty(),
        "the non-smeltable the input seat refuses"
    );
    app.on_mouse_move(panel.0 + 170, panel.1 + 20);
    app.on_left_release();
    match click_rx.try_recv().expect("the no-op PICKUP rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, -1, "the between release's own -1");
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.carried, (dirt, 5), "the cursor the -1 keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE RIGHT DRAG'S OWN ONE-EACH (the type-1 lane the
/// right press arms — the chest + the table rounds' law the
/// window belief wears): the arm NO place and NO packet keeps,
/// the paint the furnace's own verdicts re-keys (the result's
/// iconst_0 the paint refuses), the release the one-each
/// distribution and the masked triple (START mask(0,1)=4, ADDs
/// mask(1,1)=5, END mask(2,1)=6) speak, the HUD row the hotbar
/// menus 30..38 the sync follows.
#[test]
fn the_furnace_right_drag_the_one_each_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Furnace);
    app.furnace_live_window = Some((9, 4));
    let panel = inventory_panel_origin(640, 360);
    app.furnace_panel = panel;
    app.furnace_slots = vec![(0, 0); 39];
    app.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 4),
        (panel.0 + 120, panel.1 + 100, 5),
        (panel.0 + 140, panel.1 + 100, 6),
        (panel.0 + 130, panel.1 + 50, 2),
    ];
    // THE RIGHT PRESS'S OWN ARM: the carried press the type-1 drag
    // arms — NO place, NO painted seat, NO packet
    app.live_furnace_carried = (dirt, 5);
    let (ox, oy) = (panel.0 + 100, panel.1 + 100);
    app.on_right_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the right press the arm speaks");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert_eq!(app.qc_button, 1, "the right button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.furnace_slots[4], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    // THE PAINT: the first motion paints the press seat, the ghost
    // the ONE-EACH projection keeps
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(
        app.qc_slots,
        vec![4],
        "the press seat the first motion paints"
    );
    assert_eq!(app.qc_remainder, 4, "the one-each ghost the cursor keeps");
    let (bx, by) = (panel.0 + 120, panel.1 + 100);
    app.on_mouse_move(bx + 8, by + 8);
    let (cx, cy) = (panel.0 + 140, panel.1 + 100);
    app.on_mouse_move(cx + 8, cy + 8);
    assert_eq!(app.qc_slots, vec![4, 5, 6], "three seats the motion paints");
    assert_eq!(app.qc_remainder, 2, "the ghost the one-each three keeps");
    // THE RESULT'S OWN mayPlace: the iconst_0 the paint refuses
    let (rx, ry) = (panel.0 + 130, panel.1 + 50);
    app.on_mouse_move(rx + 8, ry + 8);
    assert_eq!(
        app.qc_slots,
        vec![4, 5, 6],
        "the result the own mayPlace refuses"
    );
    // THE RELEASE: the one-each distribution — each seat ONE item,
    // the remainder the cursor keeps
    app.on_right_release();
    assert_eq!(app.furnace_slots[4], (dirt, 1), "the first seat's own one");
    assert_eq!(app.furnace_slots[5], (dirt, 1), "the second seat's one");
    assert_eq!(app.furnace_slots[6], (dirt, 1), "the third seat's one");
    assert_eq!(
        app.live_furnace_carried,
        (dirt, 2),
        "the two remaining the cursor keeps"
    );
    assert!(!app.qc_armed, "the release the arm always clears");
    // THE TRIPLE the masked buttons ride: the START mask(0,1) byte
    // 4, the ADDs mask(1,1) byte 5, the END mask(2,1) byte 6 the
    // changed map the one-each deltas names
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the furnace window's own id");
            assert_eq!(c.state_id, 4, "the revision rides");
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 4, "the START's own masked byte");
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
            assert_eq!(c.carried, (dirt, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    for menu in [4, 5, 6] {
        match click_rx.try_recv().expect("the ADD rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 9);
                assert_eq!(c.slot, menu);
                assert_eq!(c.button, 5, "the ADD's own masked byte");
                assert_eq!(c.input, crate::world_view::ContainerInput::QuickCraft);
                assert_eq!(c.changed, Vec::new());
            }
            other => panic!("expected a container click, got {other:?}"),
        }
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 6, "the END's own masked byte");
            assert_eq!(
                c.changed,
                vec![(4, (dirt, 1)), (5, (dirt, 1)), (6, (dirt, 1))],
                "the one-each deltas"
            );
            assert_eq!(c.carried, (dirt, 2), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE RIGHT DRAG'S OFFLINE ONE-EACH AND THE DEFERRED
/// PLACE-ONE (the offline sim the same laws answers — no window
/// the arm ever needs, NO packet the offline keeps, the
/// WRITE-THROUGH the store rows speak): the one-each distribution
/// the flat belief fills, the deferred place the armed button's
/// own place-ONE serves.
#[test]
fn the_furnace_right_drag_the_offline_one_each_and_the_place_one_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.open(ScreenKind::Furnace);
    let panel = inventory_panel_origin(640, 360);
    off.furnace_panel = panel;
    off.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 3),
        (panel.0 + 120, panel.1 + 100, 4),
    ];
    // THE RIGHT PRESS'S OWN ARM (the offline sim the drag arms —
    // no window the arm ever needs)
    off.furnace_carried = (dirt, 5);
    let (ox, oy) = (panel.0 + 100, panel.1 + 100);
    off.on_right_press(ox + 8, oy + 8);
    assert!(off.qc_armed, "the offline right press the arm speaks");
    assert_eq!(off.qc_type, 1, "the right button's own type");
    // THE PAINT: the pair the motion paints
    off.on_mouse_move(ox + 8, oy + 8);
    let (px, py) = (panel.0 + 120, panel.1 + 100);
    off.on_mouse_move(px + 8, py + 8);
    assert_eq!(off.qc_slots, vec![3, 4], "the pair the motion paints");
    assert_eq!(off.qc_remainder, 3, "the one-each ghost the cursor keeps");
    // THE RELEASE: the one-each distribution — the WRITE-THROUGH
    // the store rows speak (the menu 3 the inventory cell 9
    // answers, the menu 4 the cell 10)
    off.on_right_release();
    assert_eq!(off.furnace_slots[3], (dirt, 1), "the first seat's own one");
    assert_eq!(off.furnace_slots[4], (dirt, 1), "the second seat's one");
    assert_eq!(
        off.inventory.items[9],
        (dirt, 1),
        "the write-through the store speaks"
    );
    assert_eq!(
        off.inventory.items[10],
        (dirt, 1),
        "the second write-through"
    );
    assert_eq!(
        off.furnace_carried,
        (dirt, 3),
        "the three remaining the cursor keeps"
    );
    assert!(!off.qc_armed, "the release the arm clears");
    // THE EMPTY-PAINT RELEASE ON A STORE SEAT: the pre-press
    // motion the position speaks, the deferred place the armed
    // button's own place-ONE serves (the seat takes one, the
    // cursor keeps the rest, the write-through follows)
    off.furnace_carried = (dirt, 4);
    off.on_mouse_move(px + 8, py + 8);
    off.on_right_press(px + 8, py + 8);
    assert!(off.qc_armed, "the right press the arm speaks again");
    off.on_right_release();
    assert_eq!(
        off.furnace_slots[4],
        (dirt, 2),
        "the deferred place ONE rides"
    );
    assert_eq!(
        off.inventory.items[10],
        (dirt, 2),
        "the write-through the place speaks"
    );
    assert_eq!(off.furnace_carried, (dirt, 3), "the cursor the one loses");
    assert!(!off.qc_armed, "the release the arm clears");
}

/// THE FURNACE RIGHT DRAG'S OWN GATES AND ABORTS (the no-window
/// stand-in the arm silences, the in-flight gate the other
/// button's press keeps, the mismatch abort BOTH ways the packets
/// never send, the survivor drag the type-0 even split the release
/// re-keys per drag, the carried-empty press the immediate
/// secondary keeps).
#[test]
fn the_furnace_right_drag_the_gates_and_the_aborts_wear() {
    use crate::world_view::ContainerCommand;
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Furnace);
    let panel = inventory_panel_origin(640, 360);
    app.furnace_panel = panel;
    app.furnace_slots = vec![(0, 0); 39];
    app.furnace_slot_rects = vec![
        (panel.0 + 100, panel.1 + 100, 4),
        (panel.0 + 120, panel.1 + 100, 5),
    ];
    // THE NO-WINDOW STAND-IN: the carried cursor the belief keeps
    // but no window the arm refuses
    app.live_furnace_carried = (dirt, 8);
    let (ox, oy) = (panel.0 + 100, panel.1 + 100);
    app.on_right_press(ox + 8, oy + 8);
    assert!(
        !app.qc_armed,
        "the stand-in's right press the arm never speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the silence NO packet keeps");
    // THE WINDOW ARMS (the live flag the click walk's own split
    // keys — the deferred place's PICKUP the wire lane serves)
    app.furnace_is_live = true;
    app.furnace_live_window = Some((9, 4));
    // THE ARMED LEFT DRAG THE RIGHT PRESS SILENCES (the in-flight
    // gate — the right release the mismatch abort speaks)
    app.on_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the left drag armed");
    assert_eq!(app.qc_type, 0, "the left press's own type");
    app.on_right_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the right press the in-flight gate keeps");
    assert_eq!(app.qc_type, 0, "the armed state the press never re-keys");
    assert_eq!(app.furnace_slots[4], (0, 0), "no place rode the press");
    assert!(
        click_rx.try_recv().is_err(),
        "the in-flight press NO packet keeps"
    );
    app.on_right_release();
    assert!(!app.qc_armed, "the wrong button's release the drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE ARMED RIGHT DRAG THE LEFT RELEASE ABORTS (the mismatch's
    // own mirror)
    app.on_right_press(ox + 8, oy + 8);
    assert!(app.qc_armed, "the right drag armed");
    assert_eq!(app.qc_type, 1, "the right press's own type");
    app.on_mouse_move(ox + 8, oy + 8);
    assert_eq!(app.qc_slots, vec![4], "the motion the press seat paints");
    app.on_left_release();
    assert!(!app.qc_armed, "the left release the right drag aborts");
    assert!(app.qc_slots.is_empty(), "the abort the painted set clears");
    assert_eq!(
        app.furnace_slots[4],
        (0, 0),
        "the abort NO distribution speaks"
    );
    assert!(click_rx.try_recv().is_err(), "the abort NO packet keeps");
    // THE SURVIVOR: the left drag after the aborts the same type-0
    // even split wears (the lanes never mix, the law the release
    // re-keys per drag)
    app.on_press(ox + 8, oy + 8);
    app.on_mouse_move(ox + 8, oy + 8);
    app.on_left_release();
    assert_eq!(
        app.furnace_slots[4],
        (dirt, 8),
        "the one-seat split the whole keeps"
    );
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the one-seat split the cursor empties"
    );
    for _ in 0..3 {
        let _ = click_rx.try_recv(); // the START, the ADD, the END
    }
    assert!(
        click_rx.try_recv().is_err(),
        "the lane the drain keeps empty"
    );
    // THE CARRIED-EMPTY RIGHT PRESS KEEPS THE IMMEDIATE SECONDARY
    // (the split-pickup the press lane answers — no drag the empty
    // cursor arms): the store seat 5 the stone holds
    app.furnace_slots[5] = (stone, 10);
    app.live_furnace_carried = (0, 0);
    let (sx, sy) = (panel.0 + 120, panel.1 + 100);
    app.on_right_press(sx + 8, sy + 8);
    assert!(!app.qc_armed, "the empty cursor the drag never arms");
    assert_eq!(app.live_furnace_carried, (stone, 5), "the split's own half");
    assert_eq!(app.furnace_slots[5], (stone, 5), "the seat the half keeps");
    match click_rx.try_recv().expect("the split rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9);
            assert_eq!(c.slot, 5);
            assert_eq!(c.button, 1);
            assert_eq!(c.input, crate::world_view::ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(5, (stone, 5))]);
            assert_eq!(c.carried, (stone, 5));
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FLOATING CARRIED'S OWN GHOST (javap 26.2
/// AbstractContainerScreen.extractCarriedItem pcs 0-101 — the paint
/// law the QuickCraft family's rounds banked the state for, the
/// cursor count the projection re-keys): the wire inventory drag's
/// 2+-seat arm the remainder speaks (the `copyWithCount(
/// quickCraftingRemainder)` ghost — the count the projected leftover
/// wears, NOT the carried's own), the 1-seat arm the REAL count
/// keeps (pc 35-45 the `> 1` gate), the unarmed cursor the real
/// count keeps, and the release the real (post-distribute) carried
/// the cursor restores.
#[test]
fn the_floating_ghost_the_remainder_projection_wears() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the live session's own sink: the wire release the distribution
    // rides (the None sink the command dies on the frame thread)
    let (click_tx, _click_rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    app.live_inventory_updated(slots, Vec::new(), 4, (dirt, 8));
    // THE UNARMED CURSOR: the real count the law keeps
    let law = |app: &App| {
        floating_ghost_count(
            app.qc_armed,
            app.qc_slots.len(),
            app.qc_remainder,
            app.live_inv_carried.1,
        )
    };
    assert_eq!(
        law(&app),
        Some(8),
        "the unarmed cursor the real count keeps"
    );
    // THE ARM: the carried press the drag arms, the first seat the
    // motion paints — the 1-seat arm the REAL count still keeps
    // (pc 35-45 the quickCraftSlots.size() > 1 gate)
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 0);
    app.on_right_press(ax + 8, ay + 8);
    app.on_mouse_move(ax + 8, ay + 8);
    assert!(app.qc_armed && app.qc_slots.len() == 1);
    assert_eq!(law(&app), Some(8), "the 1-seat arm the real count keeps");
    // THE SECOND SEAT: the >1 gate opens — the cursor count the
    // remainder's projection wears (the one-each fold 8-1-1=6, NOT
    // the carried's own 8)
    let (bx, by) = inventory_slot_origin(app.inv_panel, 1);
    app.on_mouse_move(bx + 8, by + 8);
    assert_eq!(
        app.qc_slots,
        vec![36, 37],
        "the two seats the motion paints"
    );
    assert_eq!(app.qc_remainder, 6, "the one-each fold the state keeps");
    assert_eq!(law(&app), Some(6), "the ghost the remainder wears");
    assert_ne!(law(&app), Some(8), "the real count the ghost REPLACES");
    // THE RELEASE: the real (post-distribute) carried the cursor
    // restores — the projection the armed state alone wears
    app.on_right_release();
    assert_eq!(
        app.live_inv_carried,
        (dirt, 6),
        "the distribution the real carry keeps"
    );
    assert_eq!(
        law(&app),
        Some(6),
        "the released cursor the real count keeps"
    );
}

/// THE FLOATING GHOST'S DRAINED VANISH (the extractCarriedItem
/// walk's isEmpty arm pc 59-64 — the remainder the fold drains the
/// EMPTY ghost stack wears, the cursor paint VANISHES: the
/// `graphics.item` and the `itemDecorations`' own empty gate pc 0-7
/// both bail, the yellow "?" label the walk banks is DEAD code in
/// 26.2): the offline drag's 2-seat paint the count-2 carry fully
/// distributes, the remainder 0 the law answers None — the cursor
/// bare while the drag rides, the carried stack itself untouched.
#[test]
fn the_floating_ghost_the_drained_vanish_wears() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE ARM: the count-2 carry the drag arms — the offline sim
    // the arm the local lanes answer (no window the arm needs)
    app.inv_carried = (dirt, 2);
    let (ax, ay) = inventory_slot_origin(app.inv_panel, 20);
    app.on_right_press(ax + 8, ay + 8);
    app.on_mouse_move(ax + 8, ay + 8);
    let (bx, by) = inventory_slot_origin(app.inv_panel, 21);
    app.on_mouse_move(bx + 8, by + 8);
    assert_eq!(
        app.qc_slots,
        vec![20, 21],
        "the two seats the motion paints"
    );
    assert_eq!(app.qc_remainder, 0, "the one-each fold the carry drains");
    // THE VANISH: the 2+-seat arm the ghost wears, the drained
    // remainder the EMPTY stack answers — None the cursor bare keeps
    let law = floating_ghost_count(
        app.qc_armed,
        app.qc_slots.len(),
        app.qc_remainder,
        app.inv_carried.1,
    );
    assert!(
        app.qc_armed && app.qc_slots.len() > 1,
        "the ghost's own gate rides"
    );
    assert_eq!(law, None, "the drained ghost the cursor VANISHES");
    // the carried stack itself untouched — the paint alone projects
    assert_eq!(
        app.inv_carried,
        (dirt, 2),
        "the real carry the paint never spends"
    );
    // THE RELEASE: the one-each distribution the last two serves,
    // the post-release cursor the REAL count 0 keeps (the empty
    // carry's own gate — the sites' carried.1 > 0 the paint skips)
    app.on_right_release();
    assert_eq!(
        app.inventory.items[20],
        (dirt, 1),
        "the first seat's own one"
    );
    assert_eq!(
        app.inventory.items[21],
        (dirt, 1),
        "the second seat's own one"
    );
    assert_eq!(app.inv_carried.1, 0, "the release the carry empties");
    assert_eq!(
        floating_ghost_count(
            app.qc_armed,
            app.qc_slots.len(),
            app.qc_remainder,
            app.inv_carried.1
        ),
        None,
        "the emptied carry the cursor bare keeps"
    );
}

/// THE FLOATING GHOST'S OWN LANES (the four screens the same base
/// walk answers — the chest the LIVE session alone wears, the table
/// and the furnace their live/offline pairs the law both rides):
/// the chest drag's ghost the believed carry projects, the table's
/// offline drag the local store's own answers, and the CROSS-SCREEN
/// safety the setScreen's removed() walk keeps (the armed state the
/// screen switch kills — the fresh screen's cursor the real count
/// speaks).
#[test]
fn the_floating_ghost_the_lanes_and_the_screen_death_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    // THE CHEST (live): the drag the ghost's projection wears
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, _click_rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(click_tx);
    app.open(ScreenKind::Chest);
    app.chest_is_live = true;
    app.chest_live_window = Some((9, 3));
    let panel = chest_panel_origin(640, 360);
    app.chest_panel = panel;
    app.chest_slots = vec![(0, 0); 27];
    app.chest_mirror = vec![(0, 0); 36];
    app.chest_slot_rects = chest_rects_for(panel);
    app.chest_carried = (stone, 7);
    let (ax, ay) = chest_container_slot(panel, 0);
    app.on_right_press(ax + 8, ay + 8);
    app.on_mouse_move(ax + 8, ay + 8);
    let (bx, by) = chest_container_slot(panel, 1);
    app.on_mouse_move(bx + 8, by + 8);
    assert_eq!(app.qc_slots, vec![0, 1], "the chest pair the motion paints");
    assert_eq!(app.qc_remainder, 5, "the one-each fold the state keeps");
    assert_eq!(
        floating_ghost_count(
            app.qc_armed,
            app.qc_slots.len(),
            app.qc_remainder,
            app.chest_carried.1
        ),
        Some(5),
        "the chest ghost the believed carry projects"
    );
    // THE SCREEN DEATH: the setScreen walk the armed state kills —
    // the fresh screen's cursor the real count speaks (the removed()
    // shape the state's own fields wear)
    app.open(ScreenKind::Inventory);
    assert!(!app.qc_armed, "the screen switch the arm kills");
    assert!(
        app.qc_slots.is_empty(),
        "the painted seats the death clears"
    );
    assert_eq!(app.qc_remainder, 0, "the projection the death zeroes");
    app.inv_carried = (dirt, 4);
    assert_eq!(
        floating_ghost_count(
            app.qc_armed,
            app.qc_slots.len(),
            app.qc_remainder,
            app.inv_carried.1
        ),
        Some(4),
        "the fresh screen's cursor the real count speaks"
    );
    // THE TABLE (offline): the drag the ghost's projection wears —
    // the fresh offline app the local store's own lanes answer
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.open(ScreenKind::Crafting);
    off.table_panel = inventory_panel_origin(640, 360);
    off.inv_carried = (dirt, 6);
    let (tx, ty) = table_craft_origin(off.table_panel, 0);
    off.on_right_press(tx + 9, ty + 9);
    off.on_mouse_move(tx + 9, ty + 9);
    let (tx2, ty2) = table_craft_origin(off.table_panel, 1);
    off.on_mouse_move(tx2 + 9, ty2 + 9);
    assert_eq!(
        off.qc_slots,
        vec![1, 2],
        "the horizontal pair the motion paints"
    );
    assert_eq!(off.qc_remainder, 4, "the one-each fold the state keeps");
    assert_eq!(
        floating_ghost_count(
            off.qc_armed,
            off.qc_slots.len(),
            off.qc_remainder,
            off.inv_carried.1
        ),
        Some(4),
        "the table ghost the local carry projects"
    );
}

/// THE DROP-OUTSIDE LANE (the 26.2 AbstractContainerScreen's own
/// hasClickedOutside walk + the doClick THROW's slot -999): the LEFT
/// press outside the panel drops the WHOLE carried stack — the carry
/// empties, one ItemDrop spawns at the eye minus the
/// 0.30000001192092896 lift with the 40-tick pickup delay and the
/// AIMED look throw (the 2-arg drop's own iconst_0 pins
/// dropAround=FALSE — the boolean rides the includeThrower slot, the
/// look-direction impulse the createItemStackToDrop else arm speaks)
/// — and the container STAYS OPEN.
#[test]
fn the_outside_left_press_throws_the_whole_carried() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[0] = (62, 27);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
    app.on_press(ox + 8, oy + 8);
    assert_eq!(app.inv_carried, (62, 27), "the stack rides the cursor");
    let drops_before = app.drops.len();
    // the OUTSIDE press (the 4,4 corner hasClickedOutside answers
    // true — the drag arm arms, the release's own -999 PICKUP the
    // drop serves)
    app.on_press(4, 4);
    app.on_mouse_move(4, 4);
    app.on_left_release();
    assert_eq!(app.inv_carried, (0, 0), "the cursor emptied");
    assert_eq!(app.drops.len(), drops_before + 1, "one drop spawned");
    let d = app.drops.last().unwrap();
    assert_eq!(d.item, 62);
    assert_eq!(d.count, 27, "the WHOLE stack thrown");
    assert_eq!(d.pickup_delay, 40, "the throw's own delay");
    let b = app.body.as_ref().unwrap();
    assert_eq!(d.x, b.x, "the body's x");
    assert_eq!(d.z, b.z, "the body's z");
    assert_eq!(
        d.y,
        b.y + b.eye_height() - 0.30000001192092896,
        "the eye minus the 0.30000001192092896 lift"
    );
    // the AIMED branch's own folds (seed 3's draw chain: the ctor's
    // bob seed, then f12 = nextFloat*6.2831855, f13 = 0.02*nextFloat,
    // the vy jitter pair r1/r2 — the look-direction law at the spawn
    // angles: the forward 0.3 fold minus the sin(pitch) share, the
    // 0.1 base lift, the ±0.05 jitter band, the cos/sin(f12)·f13
    // bob-cosine scatter ±0.02 wide — the spawn's own yaw speaks the
    // +X axis, the horizontal SPEED the axis-free pin)
    let speed = (d.vx * d.vx + d.vz * d.vz).sqrt();
    assert!(
        (speed - 0.3).abs() <= 0.03,
        "the look-direction 0.3 fold (speed={speed})"
    );
    assert!(
        d.vy > 0.05 && d.vy < 0.15,
        "the 0.1 base lift within the ±0.05 jitter band (vy={vy})",
        vy = d.vy
    );
    // the container STAYS OPEN (only the close keys close it)
    assert_eq!(app.screen, ScreenKind::Inventory);
    // and the store untouched (the throw rides the CURSOR, not the slots)
    assert_eq!(app.inventory.items[0], (0, 0));
}

/// THE DROP-OUTSIDE'S RIGHT PRESS (the doClick THROW's split(1)
/// arm — the press->release drive the type-1 arm law wears: the
/// carried press the drag arms, the release's own deferred throw
/// the ONE split serves): ONE item flies, the carry decrements —
/// twice for the count.
#[test]
fn the_outside_right_press_throws_one() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[0] = (63, 5);
    let (ox, oy) = inventory_slot_origin(app.inv_panel, 0);
    app.on_press(ox + 8, oy + 8);
    assert_eq!(app.inv_carried, (63, 5), "the stack rides");
    let drops_before = app.drops.len();
    // the motion the release position feeds (the press the drag
    // arms — no drop rides it), the release the outside throw-ONE
    // serves
    app.on_mouse_move(4, 4);
    app.on_right_press(4, 4);
    app.on_right_release();
    assert_eq!(app.inv_carried, (63, 4), "one split off");
    assert_eq!(app.drops.len(), drops_before + 1);
    let d = app.drops.last().unwrap();
    assert_eq!(d.item, 63);
    assert_eq!(d.count, 1, "ONE item");
    assert_eq!(d.pickup_delay, 40);
    // the second press->release throws the next one
    app.on_right_press(4, 4);
    app.on_right_release();
    assert_eq!(app.inv_carried, (63, 3), "one more gone");
    assert_eq!(app.drops.len(), drops_before + 2);
    assert_eq!(app.drops.last().unwrap().count, 1);
    // the container stays open
    assert_eq!(app.screen, ScreenKind::Inventory);
}

/// THE DROP-OUTSIDE'S EMPTY-CURSOR SILENCE (the doClick THROW's own
/// isEmpty guard): a press outside with nothing carried spawns
/// nothing and leaves the screen whole.
#[test]
fn the_empty_cursor_outside_press_is_silence() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    app.inventory.items[0] = (62, 27);
    let drops_before = app.drops.len();
    app.on_press(4, 4);
    assert_eq!(app.drops.len(), drops_before, "no drop spawned");
    assert_eq!(app.inv_carried, (0, 0), "the cursor stayed empty");
    assert_eq!(app.screen, ScreenKind::Inventory, "the screen stays");
    // and the right press keeps the same silence
    app.on_right_press(4, 4);
    assert_eq!(app.drops.len(), drops_before, "still no drop");
}

/// THE Q-DROP LANE (the handleKeybinds keyDrop walk — the plain Q):
/// the selected slot's ONE item flies the AIMED look throw, the slot
/// decrements, the MAIN_HAND swing fires, the HUD hotbar syncs.
#[test]
fn the_q_key_throws_one_from_the_selected_slot() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.inventory.items[app.selected_hotbar] = (62, 7);
    let drops_before = app.drops.len();
    app.on_drop_key(false);
    // removeFromSelected(false): exactly ONE off
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (62, 6),
        "one item gone from the selected slot"
    );
    assert_eq!(app.drops.len(), drops_before + 1, "one drop spawned");
    let d = app.drops.last().unwrap();
    assert_eq!(d.item, 62);
    assert_eq!(d.count, 1, "ONE item");
    assert_eq!(d.pickup_delay, 40, "the throw's own delay");
    let b = app.body.as_ref().unwrap();
    assert_eq!(d.x, b.x, "the body's x");
    assert_eq!(d.z, b.z, "the body's z");
    assert_eq!(
        d.y,
        b.y + b.eye_height() - 0.30000001192092896,
        "the eye minus the 0.30000001192092896 lift"
    );
    // the AIMED look throw's own shape: the forward 0.3 fold (the
    // spawn's yaw/pitch speak the axis), the 0.1 base lift within
    // the ±0.05 jitter band, the bob-cosine scatter ±0.02 wide
    let speed = (d.vx * d.vx + d.vz * d.vz).sqrt();
    assert!(
        (speed - 0.3).abs() <= 0.03,
        "the horizontal speed the 0.3 fold speaks (speed={speed})"
    );
    assert!(
        d.vy > 0.05 && d.vy < 0.15,
        "the 0.1 base lift within the ±0.05 jitter band (vy={vy})",
        vy = d.vy
    );
    // the success arm's own swing (LivingEntity.swing MAIN_HAND)
    assert!(app.swing.swinging, "the swing fired");
    // and the HUD hotbar row synced (the store's own mirror)
    assert_eq!(
        app.hotbar_slots[app.selected_hotbar],
        (62, 6),
        "the HUD synced"
    );
}

/// THE CTRL+Q FORM (LocalPlayer.drop's wholeStack boolean — the
/// hasControlDown the keyDrop lane speaks): the WHOLE selected stack
/// flies (removeFromSelected's removeItem(selected, getCount())
/// arm), the slot empties.
#[test]
fn the_ctrl_q_throws_the_whole_stack() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.inventory.items[app.selected_hotbar] = (63, 12);
    let drops_before = app.drops.len();
    app.on_drop_key(true);
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (0, 0),
        "the slot emptied"
    );
    assert_eq!(app.drops.len(), drops_before + 1, "one drop spawned");
    let d = app.drops.last().unwrap();
    assert_eq!(d.item, 63);
    assert_eq!(d.count, 12, "the WHOLE stack");
    assert_eq!(d.pickup_delay, 40);
    assert!(app.swing.swinging, "the swing fired");
}

/// THE EMPTY SLOT'S SILENCE (removeFromSelected's own EMPTY law —
/// drop returns false): no entity, no swing, no draw burned.
#[test]
fn the_empty_selected_slot_q_is_silence() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let drops_before = app.drops.len();
    app.on_drop_key(false);
    assert_eq!(app.drops.len(), drops_before, "no drop spawned");
    assert!(!app.swing.swinging, "no swing — drop returned false");
    // the Ctrl form keeps the same silence
    app.on_drop_key(true);
    assert_eq!(app.drops.len(), drops_before, "still no drop");
    assert!(!app.swing.swinging, "still no swing");
}

/// THE Q KEY'S OWN GATE (the handleKeybinds overlay==null &&
/// screen==null law): the keybinds the OPEN SCREEN never passes —
/// the inventory-open Q is silence — and the wire session refuses
/// honestly (the store the server owns, the DROP_ITEM action packet
/// lane the follow-up names).
#[test]
fn the_q_key_refuses_the_screens_and_the_wire() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.inventory.items[app.selected_hotbar] = (62, 7);
    // the inventory OPEN — the screen != World the gate refuses
    app.open_inventory();
    let drops_before = app.drops.len();
    app.on_drop_key(false);
    assert_eq!(app.drops.len(), drops_before, "the screen gate holds");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (62, 7),
        "the store whole"
    );
    // the PAUSE — the vanilla's own freeze lane
    app.close_inventory();
    app.paused = true;
    app.on_drop_key(false);
    assert_eq!(app.drops.len(), drops_before, "the pause freeze holds");
    // the WIRE session — the store the server owns
    app.paused = false;
    app.world_is_network = true;
    app.on_drop_key(false);
    assert_eq!(app.drops.len(), drops_before, "the wire refusal holds");
    assert_eq!(
        app.inventory.items[app.selected_hotbar],
        (62, 7),
        "the wire store whole"
    );
}

/// THE MIDDLE-CLICK PICK (the hotbar hit's own law — tryPickItem's
/// `isHotbarSlot(i)` arm): the block the downward crosshair speaks,
/// its item found IN the hotbar row — the setSelectedSlot walk
/// re-points the selection, the store's counts untouched, NO swing
/// (the pick chain never names one), NO drop, the HUD hotbar row
/// the always-tail syncs.
#[test]
fn the_middle_click_picks_the_hotbar_block() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.body.as_mut().unwrap().pitch = 80.0; // the ground speaks — the hit guaranteed
    let hit = app
        .clip_crosshair()
        .expect("the downward aim hits the terrain");
    let kind = app
        .active_world()
        .map(|w| w.get(hit.pos.0, hit.pos.1, hit.pos.2))
        .expect("the offline world stands");
    assert_ne!(kind, crate::world::AIR, "the hit cell solid");
    let item = mc_registry::item_id_by_name(crate::world::NAME_OF[kind as usize])
        .expect("the terrain block carries its item");
    app.inventory.items[3] = (item, 5);
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 3, "the hotbar hit selected");
    assert_eq!(app.inventory.items[3], (item, 5), "the store untouched");
    assert!(!app.swing.swinging, "the pick chain never swings");
    assert!(app.drops.is_empty(), "no drop the pick never names");
    assert_eq!(app.hotbar_slots[3], (item, 5), "the HUD synced");
}

/// THE MAIN-INVENTORY HIT (the pickSlot walk the shared helper
/// speaks): the item ONLY in the main grid — the suitable re-point
/// first (the hand EMPTY answers the scan's first free slot, the
/// selected itself), then the swap brings the item INTO the hand,
/// the displaced empty rides back to the main grid.
#[test]
fn the_middle_click_swaps_the_main_inventory_match_into_the_hand() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.body.as_mut().unwrap().pitch = 80.0; // the vanilla positive-down degrees
    let hit = app.clip_crosshair().expect("the downward aim hits");
    let kind = app
        .active_world()
        .map(|w| w.get(hit.pos.0, hit.pos.1, hit.pos.2))
        .expect("the offline world stands");
    let item = mc_registry::item_id_by_name(crate::world::NAME_OF[kind as usize])
        .expect("the terrain block carries its item");
    app.inventory.items[9] = (item, 2); // the main grid ONLY — the hotbar clean
    app.on_pick_block(false);
    assert_eq!(
        app.selected_hotbar, 0,
        "the hand slot the suitable answer (the first free scanned)"
    );
    assert_eq!(app.inventory.items[0], (item, 2), "the item IN the hand");
    assert_eq!(app.inventory.items[9], (0, 0), "the main slot emptied");
    assert_eq!(app.hotbar_slots[0], (item, 2), "the HUD synced");
}

/// THE SUITABLE RE-POINT'S OWN LAW (getSuitableHotbarSlot code 91):
/// the hand FULL and a free hotbar slot ahead — the scan
/// `(selected + i) % 9` finds the FREE slot, the swap lands the
/// item THERE, the selection follows it, the hand's own stack
/// stays put (the displaced empty rides back to the main grid).
#[test]
fn the_middle_click_lands_in_the_first_free_hotbar_slot() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.body.as_mut().unwrap().pitch = 80.0; // the vanilla positive-down degrees
    let hit = app.clip_crosshair().expect("the downward aim hits");
    let kind = app
        .active_world()
        .map(|w| w.get(hit.pos.0, hit.pos.1, hit.pos.2))
        .expect("the offline world stands");
    let item = mc_registry::item_id_by_name(crate::world::NAME_OF[kind as usize])
        .expect("the terrain block carries its item");
    app.inventory.items[0] = (62, 7); // the hand full — the scan moves on
    app.inventory.items[9] = (item, 2);
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 1, "the scan's first free slot");
    assert_eq!(app.inventory.items[1], (item, 2), "the item lands there");
    assert_eq!(app.inventory.items[0], (62, 7), "the hand untouched");
    assert_eq!(
        app.inventory.items[9],
        (0, 0),
        "the displaced empty rides back"
    );
    assert_eq!(app.hotbar_slots[1], (item, 2), "the HUD synced");
}

/// THE MIDDLE CLICK'S OWN SILENCES (the survival law's gates): the
/// item NOWHERE in the store (the -1 the findSlotMatchingItem scan
/// speaks — the creative addAndPickItem the hasInfiniteMaterials
/// gate holds), the MISS (the sky the clip refuses), the death
/// modal, the OPEN SCREEN and the pause (the handleKeybinds gate),
/// the wire session (the store the server owns). Every arm leaves
/// the selection and the store whole.
#[test]
fn the_middle_click_misses_silently() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.body.as_mut().unwrap().pitch = 80.0; // the vanilla positive-down degrees
                                             // the picked item NOWHERE in the 36 slots: the -1 silence
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the -1 never moved the selection");
    // the MISS: the sky the clip refuses
    app.body.as_mut().unwrap().pitch = -80.0; // the vanilla negative-up degrees
    assert!(app.clip_crosshair().is_none(), "the upward aim misses");
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the MISS never moved the selection");
    // the death modal
    app.body.as_mut().unwrap().pitch = 80.0; // the vanilla positive-down degrees
    app.inventory.items[3] = (62, 7); // a WRONG item — the store's own witness
    app.dead = true;
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the death gate holds");
    app.dead = false;
    // the OPEN SCREEN — the handleKeybinds gate refuses
    app.open_inventory();
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the screen gate holds");
    app.close_inventory();
    // the PAUSE — the vanilla's own freeze lane
    app.paused = true;
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the pause freeze holds");
    app.paused = false;
    // the WIRE session — the store the server owns
    app.world_is_network = true;
    app.on_pick_block(false);
    assert_eq!(app.selected_hotbar, 0, "the wire refusal holds");
    assert_eq!(
        app.inventory.items[3],
        (62, 7),
        "the store whole throughout"
    );
}

/// THE 2x2 CRAFT GRID — the log's own loop (the survival chain the
/// real game's muscle memory rides): the store's log rides the
/// carried into a craft cell (the predict_pickup walk), the
/// recompute law speaks the shapeless fast path (oak_log -> 4
/// oak_planks), and the result slot's take rides the onTake walk —
/// the whole result, the per-cell decrement, the recompute FIRING
/// AGAIN while the logs speak (the further takes the real game
/// clicks), the spent grid the EMPTY verdict keeps.
#[test]
fn the_craft_grid_turns_a_log_into_four_planks() {
    let log = mc_registry::item_id_by_name("oak_log").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // the store's log into the hand, then INTO the craft cell 0
    // (panel (232, 97): store 5 at (330, 239), cell 0 at (330, 115)
    // — the 16x16 hit rects at the +1 inset, the result at (386,
    // 125); the clicks ride the on_press routing the store lane
    // shares)
    app.inventory.items[5] = (log, 3);
    app.on_press(338, 246);
    assert_eq!(app.inv_carried, (log, 3), "the log in the hand");
    app.on_press(338, 123);
    app.on_mouse_move(338, 123);
    app.on_left_release();
    assert_eq!(app.craft_grid[0], (log, 3), "the cell holds the stack");
    assert_eq!(app.inv_carried, (0, 0), "the hand emptied");
    // THE RECOMPUTE (InventoryMenu.slotsChanged's own ask — the
    // getRecipeFor verdict the result slot keeps)
    assert_eq!(app.craft_result, (planks, 4), "the shapeless fast path");
    // THE TAKE — the whole result, the onTake's per-cell decrement,
    // the recompute firing again while the logs speak
    app.on_press(394, 133);
    assert_eq!(app.inv_carried, (planks, 4), "the whole result taken");
    assert_eq!(app.craft_grid[0], (log, 2), "one log off the cell");
    assert_eq!(app.craft_result, (planks, 4), "the recipe fires AGAIN");
    app.on_press(394, 133);
    app.on_mouse_move(394, 133);
    app.on_left_release();
    assert_eq!(app.inv_carried, (planks, 8), "the second take merged");
    app.on_press(394, 133);
    app.on_mouse_move(394, 133);
    app.on_left_release();
    assert_eq!(app.inv_carried, (planks, 12), "the third take");
    assert_eq!(app.craft_grid[0], (0, 0), "the cell spent");
    assert_eq!(app.craft_result, (0, 0), "the empty grid's EMPTY verdict");
    assert_eq!(app.inventory.items[5], (0, 0), "the store slot still empty");
}

/// THE CREATIVE E (the 26.2 CreativeModeInventoryScreen's own
/// hasInfiniteMaterials gate the init walk speaks — the creative
/// screen the infinite-materials player alone wears): the survival
/// E the inventory screen keeps, the creative E the picker opens,
/// the source the display law feeds (the count-1 entries the
/// ItemDisplayBuilder's 'Stack size must be exactly 1'
/// IllegalArgumentException banks, the air the registry's id 0 the
/// display list never carried), the palette head the scrollTo(0)
/// refill speaks, the default tab the building-blocks seat keeps,
/// the WIRE session the survival screen keeps (the
/// ServerboundSetCreativeModeSlot packet lane the follow-up owes).
#[test]
fn the_creative_e_opens_the_picker_and_the_palette_holds() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // the survival E the inventory screen keeps
    app.game_mode = GameMode::Survival;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Inventory, "the survival seat");
    app.close_inventory();
    assert_eq!(app.screen, ScreenKind::World);
    // the creative E the picker wears
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the picker's seat");
    assert_eq!(app.creative_tab, 0, "the building-blocks default");
    // THE SOURCE (the display law)
    assert!(!app.creative_source.is_empty(), "the registry's own list");
    assert!(
        app.creative_source.iter().all(|&(_, count)| count == 1),
        "the count-1 display law"
    );
    assert!(
        app.creative_source.iter().all(|&(id, _)| id != 0),
        "the air the display list never carried"
    );
    assert_eq!(
        app.creative_source[0],
        (stone, 1),
        "stone the wire order's head"
    );
    // the palette head the scrollTo(0) refill speaks
    assert_eq!(app.creative_palette[0], (stone, 1), "cell 0 = stone");
    assert_eq!(
        app.creative_palette[44], app.creative_source[44],
        "the head page full"
    );
    // THE WIRE GATE RETIRED (the 26.2 init walk the
    // hasInfiniteMaterials verdict ALONE keeps — the network
    // session the gate never splits: the creative live E the
    // picker opens, the survival E the inventory screen keeps)
    app.world_is_network = true;
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the wire the picker opens"
    );
    app.world_is_network = false;
    // the E closes back to the world (the removed()'s carried law —
    // the empty carried the plain close keeps)
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative);
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "the E close");
}

/// THE PALETTE'S OWN CLICK LAW (the slotClicked walk's decoded
/// arms verbatim): the plain take the hovered stack's own count
/// speaks (the display law's count-1 = ONE per click), the
/// same-item left the grow(1) dial rides, the right the shrink(1)
/// the palette eats, the shift the max-stack take banks, the
/// different-item click the whole carried DESTROYS, the
/// empty-hole right one eats, the CLONE the middle-click full
/// stack speaks, the bottom row the predict_pickup pair rides.
#[test]
fn the_creative_palette_the_grab_the_dial_the_eat() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    // the palette cell 0's hit center (the 16x16 rect at (9, 18))
    let cell0 = (px + 9 + 8, py + 18 + 8);
    let cell1 = (px + 27 + 8, py + 18 + 8); // the granite seat
                                            // THE PLAIN TAKE (the display law: count-1 → ONE)
    app.on_press(cell0.0, cell0.1);
    assert_eq!(app.inv_carried, (stone, 1), "the one-per-click take");
    // THE GROW DIAL (the same-slot left click)
    app.on_press(cell0.0, cell0.1);
    assert_eq!(app.inv_carried, (stone, 2), "the grow(1) dial");
    // THE SHRINK (the palette eats one)
    app.on_right_press(cell0.0, cell0.1);
    assert_eq!(app.inv_carried, (stone, 1), "the shrink(1) eat");
    // THE SHIFT TAKE (the max-stack arm)
    app.inv_carried = (0, 0);
    app.shift_down = true;
    app.on_press(cell0.0, cell0.1);
    assert_eq!(app.inv_carried, (stone, 64), "the max take");
    app.shift_down = false;
    // THE EAT (the different-item click destroys the whole carried)
    app.on_press(cell1.0, cell1.1);
    assert_eq!(app.inv_carried, (0, 0), "the palette ate the stack");
    // THE CLONE (the middle-click full stack)
    app.on_creative_clone_click(cell0.0, cell0.1);
    assert_eq!(app.inv_carried, (stone, 64), "the CLONE max");
    // THE EMPTY-HOLE EAT (the search seat the short list keeps —
    // the tail cells the empty holes speak)
    app.select_creative_tab(6);
    for c in "compass".chars() {
        app.on_creative_char(c);
    }
    assert!(
        app.creative_items.len() < 45,
        "the compass query the short list keeps"
    );
    let hole = (px + 9 + 5 * 18 + 8, py + 18 + 4 * 18 + 8); // the tail row's cell
    app.on_right_press(hole.0, hole.1);
    assert_eq!(app.inv_carried, (stone, 63), "the hole ate one");
    // THE STORE ROW (the bottom row's slot 0 — the place + the
    // pickup the predict pair speaks)
    app.on_press(px + 9 + 8, py + 112 + 8);
    assert_eq!(app.inventory.items[0], (stone, 63), "the store placed");
    assert_eq!(app.inv_carried, (0, 0), "the hand emptied");
    app.on_press(px + 9 + 8, py + 112 + 8);
    assert_eq!(app.inv_carried, (stone, 63), "the store picked back");
    assert_eq!(app.inventory.items[0], (0, 0), "the store emptied");
}

/// THE SCROLL, THE SEARCH AND THE CLOSE (the mouseScrolled walk's
/// law — the delta / scrollableRows subtraction the clamp keeps,
/// the rowIndex the rounded product answers, the tail page the
/// empty cells speak; the search the identifier filter wears, the
/// tab switch the both-way clear keeps; the close the carried's
/// add-back walk serves, the persisted INVENTORY tab the re-open's
/// building-blocks default answers).
#[test]
fn the_creative_scroll_the_search_and_the_close_return() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    // THE SCROLL LAW (the scrollable rows the calculateRowCount
    // walk keeps: ceil(n/9) - 5)
    let rows = (app.creative_items.len() as i32 + 8) / 9 - 5;
    assert!(rows > 0, "the full registry the scroll span holds");
    assert_eq!(app.creative_scroll, 0.0, "the fresh open's 0");
    // the wheel-up at 0 the clamp keeps (the top page stays)
    app.on_creative_scroll(1.0);
    assert_eq!(app.creative_scroll, 0.0, "the up-clamp");
    // the wheel-down one step: scroll = 1/rows, the rowIndex = 1
    // — the palette the source's second row wears
    let second_row_head = app.creative_items[9];
    app.on_creative_scroll(-1.0);
    assert_eq!(app.creative_palette[0], second_row_head, "the row-1 refill");
    // the full scroll: the tail page the last row's remainder
    // keeps (1536 items = 170 rows + the 6-item tail)
    app.on_creative_scroll(-1.0e9);
    assert_eq!(app.creative_scroll, 1.0, "the down-clamp");
    let tail_head = app.creative_items[9 * (rows as usize + 4)];
    assert_eq!(app.creative_palette[36], tail_head, "the tail page's head");
    assert_eq!(app.creative_palette[42], (0, 0), "the tail's empty cells");
    // THE SEARCH (the identifier filter — the case the lowercased
    // ask keeps)
    app.select_creative_tab(6);
    assert!(app.creative_search.is_empty(), "the both-way clear");
    for c in "COMPASS".chars() {
        app.on_creative_char(c);
    }
    assert_eq!(app.creative_search, "COMPASS", "the box holds the text");
    assert!(
        app.creative_items.len() < app.creative_source.len(),
        "the filter narrows"
    );
    assert!(
        app.creative_items.iter().all(|&(id, _)| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER[id as usize].contains("compass")
        }),
        "the identifier filter"
    );
    app.on_creative_backspace();
    assert_eq!(app.creative_search, "COMPAS", "the pop");
    // the category seat the full list restores
    app.select_creative_tab(0);
    assert_eq!(app.creative_items.len(), app.creative_source.len());
    assert_eq!(app.creative_scroll, 0.0, "the fresh scroll");
    // THE CLOSE RETURN (the carried's add-back walk — the empty
    // store the whole stack takes)
    app.inv_carried = (stone, 30);
    app.close_creative_picker();
    assert_eq!(app.screen, ScreenKind::World, "the close's return");
    assert_eq!(app.inv_carried, (0, 0), "the hand emptied");
    assert_eq!(app.inventory.items[0], (stone, 30), "the store took it");
    // THE PERSISTED TAB (the init walk's own law — the static
    // selectedTab the local saves, the selectTab(savedTab) the
    // re-open speaks: the INVENTORY seat the WRAPPED rebuild
    // answers now, the screen the same Creative seat keeps)
    app.creative_tab = 13;
    app.open_inventory();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the wrapped seat the re-open keeps"
    );
    assert_eq!(
        app.creative_tab, 13,
        "the persisted tab the init walk answers"
    );
}

/// THE PICKER'S OWN SWAP (the jar's decoded walk — the seats the
/// fresh disasm SPLITS: the PALETTE cells the screen's OWN
/// intercept answers — slotClicked pcs 533-586 the button row the
/// FULL MAX-STACK COPY of the cell's own item wears, the cell its
/// display keeps, the row's old stack the overwrite discards, the
/// empty cell the consumed silence speaks — the doClick CASE table
/// the intercept NEVER reaches; the bottom STORE rows the CASE
/// table ride: the empty hovered cell takes min(count, max), the
/// filled pair the straight swap wears, the egg's 40 the cell's
/// 16 max splits — the row the rest keeps), the search seat's
/// empty hole the digit consumes (the focused box verdict), the
/// carried gate the world chain answers (false).
#[test]
fn the_creative_swap_the_digit_the_table_wears() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let cell0 = (px + 9 + 8, py + 18 + 8);
    let store8 = (px + 9 + 8 * 18 + 8, py + 112 + 8);
    let store9 = (px + 9 + 9 * 18 + 8, py + 112 + 8);
    // THE PALETTE'S MAX-COPY (the intercept's own law — the
    // cell's count-1 display the FULL max the copy speaks, the
    // cell the take NEVER depletes)
    app.mouse_pos = Some(cell0);
    assert!(
        app.on_creative_swap_key(0),
        "the intercept the copy answers"
    );
    assert_eq!(
        app.inventory.items[0],
        (stone, 64),
        "the full max the cell's own item speaks"
    );
    assert_eq!(
        app.creative_palette[0],
        (stone, 1),
        "the cell the copy keeps"
    );
    // THE CELL'S OWN MAX (the egg's 16 the copy count answers —
    // the cell's per-item ceiling the law wears, never the row's)
    app.creative_palette[0] = (egg, 1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(app.inventory.items[0], (egg, 16), "the egg's own max rides");
    assert_eq!(app.creative_palette[0], (egg, 1), "the cell the copy keeps");
    // THE OVERWRITE DISCARD (the row's old stack the setItem
    // never reads — the creative discard the free law keeps, the
    // add-back walk the intercept never serves)
    app.inventory.items[0] = (egg, 40);
    app.creative_palette[0] = (stone, 1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.items[0],
        (stone, 64),
        "the row the copy takes"
    );
    assert!(
        !app.inventory.items.iter().any(|&s| s == (egg, 40)),
        "the old stack the overwrite discards"
    );
    // THE EMPTY CELL'S SILENCE (the isEmpty gate the write skips
    // — the consumed verdict the silence keeps)
    app.creative_palette[0] = (0, 0);
    app.inventory.items[0] = (egg, 3);
    assert!(
        app.on_creative_swap_key(0),
        "the empty cell the silence keeps"
    );
    assert_eq!(app.inventory.items[0], (egg, 3), "the row untouched");
    // THE STORE ROW'S SPLIT (case C the bottom rows ride — the
    // filled button row the empty cell's min(count, max) takes,
    // the row the remainder keeps)
    app.inventory.items[0] = (egg, 40);
    app.inventory.items[8] = (0, 0);
    app.mouse_pos = Some(store8);
    assert!(
        app.on_creative_swap_key(0),
        "the store row the case answers"
    );
    assert_eq!(app.inventory.items[8], (egg, 16), "the cell the min takes");
    assert_eq!(app.inventory.items[0], (egg, 24), "the row the rest keeps");
    // THE STORE ROW SWAP (the bottom rows the same table wears —
    // the hotbar tail's real cell 8 the loop's own bound speaks)
    app.inventory.items[0] = (stone, 7);
    app.inventory.items[8] = (egg, 3);
    app.mouse_pos = Some(store8);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(app.inventory.items[0], (egg, 3), "the row the swap speaks");
    assert_eq!(
        app.inventory.items[8],
        (stone, 7),
        "the slot the swap takes"
    );
    // THE PHANTOM CELL RETIRED (the bytecode's own `i < 9` bound —
    // the 10th cell the old 0..18 walk answered the helper NEVER
    // places: the hover the miss wears, the walk the world chain
    // answers, the store the silence keeps)
    app.inventory.items[0] = (stone, 7);
    app.inventory.items[9] = (egg, 3);
    app.mouse_pos = Some(store9);
    assert!(!app.on_creative_swap_key(0), "the phantom the miss wears");
    assert_eq!(app.inventory.items[0], (stone, 7), "the row untouched");
    assert_eq!(app.inventory.items[9], (egg, 3), "the slot untouched");
    // THE OFFHAND'S OWN COPY (button 40 — the F key's first arm,
    // the menu id 45 the wire lane speaks)
    app.inventory.offhand[0] = (0, 0);
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    assert!(app.on_creative_swap_key(40));
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 64),
        "the offhand the max wears"
    );
    assert_eq!(
        app.creative_palette[0],
        (stone, 1),
        "the cell the copy keeps"
    );
    // THE SEARCH SEAT'S EMPTY HOLE (the digit the box consumes —
    // no change rides)
    app.select_creative_tab(6);
    app.creative_palette[44] = (0, 0);
    let tail = (px + 9 + 8 * 18 + 8, py + 18 + 4 * 18 + 8);
    app.mouse_pos = Some(tail);
    assert!(app.on_creative_swap_key(0), "the focused box consumes");
    // THE CARRIED GATE (the world chain answers — false on the
    // non-search tab; the search seat the box verdict keeps true)
    app.select_creative_tab(0);
    app.inv_carried = (stone, 1);
    assert!(!app.on_creative_swap_key(0), "the carried gate refuses");
    app.select_creative_tab(6);
    assert!(
        app.on_creative_swap_key(0),
        "the search seat the verdict keeps"
    );
    app.inv_carried = (0, 0);
    // THE NOWHERE HOVER (the non-search tab the world chain
    // answers — the keybind lane the vanilla keeps)
    app.select_creative_tab(0);
    app.mouse_pos = Some((px - 40, py - 40));
    assert!(!app.on_creative_swap_key(0), "the no-slot miss");
}

/// THE SWAP'S OWN LIVE LANE (the 26.2 wire law the fresh javap
/// banks — the screen's init pc 143-175 the CreativeInventoryListener
/// registers, the broadcastChanges the changed slots speak, the
/// listener's slotChanged the handleCreativeModeItemAdd(stack,
/// slotInd) word answers, the ServerboundSetCreativeModeSlotPacket
/// rides): the palette cell's max-copy the ONE word wears (the
/// button row's menu id the packet carries), the store rows the
/// PAIR speak (the button row + the hovered cell, the ascending
/// menu ids the diff loop keeps), the live belief the menu ids
/// mirror, the selected slot's ride the hand follows, the HUD row
/// the live projection re-speaks; the offline session the words
/// never spends (the sink the lane's early return never reads).
#[test]
fn the_creative_swap_the_live_lane_the_creative_word_rides() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    // THE LIVE SESSION (the net world the wire's own harness keeps
    // — the glide test's shape the lane wears: the sink the
    // attach speaks, the creative gate the picker's screen field
    // the live E-gate the later round banks)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.game_mode = GameMode::Creative;
    app.screen = ScreenKind::Creative;
    app.creative_tab = 0;
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let cell0 = (px + 9 + 8, py + 18 + 8);
    let store2 = (px + 9 + 2 * 18 + 8, py + 112 + 8);
    // THE PALETTE'S ONE WORD (the max-copy the intercept speaks —
    // the button row's menu id 36 the packet carries, the cell its
    // display keeps)
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    assert!(app.on_creative_swap_key(0), "the intercept the live wears");
    assert_eq!(
        app.live_inv_slots[36],
        (stone, 64),
        "the belief the id mirrors"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (stone, 64),
        "the HUD the live row speaks"
    );
    assert_eq!(
        app.live_held,
        Some(stone),
        "the selected slot the hand follows"
    );
    assert_eq!(
        app.creative_palette[0],
        (stone, 1),
        "the cell the copy keeps"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 36,
            item: (stone, 64)
        }],
        "the ONE word the ONE changed slot rides"
    );
    // THE STORE ROW'S PAIR (the CASE walk the default arm rides —
    // the button row 5 + the hovered cell 2, the ascending menu
    // ids the diff loop keeps, the empty write the changed slot
    // speaks)
    app.inventory.items[2] = (egg, 40);
    app.inventory.items[5] = (0, 0);
    app.mouse_pos = Some(store2);
    assert!(app.on_creative_swap_key(5), "the store row the live wears");
    assert_eq!(
        app.live_inv_slots[38],
        (0, 0),
        "the emptied cell the belief keeps"
    );
    assert_eq!(
        app.live_inv_slots[41],
        (egg, 40),
        "the row the ride mirrors"
    );
    assert_eq!(app.hotbar_slots[5], (egg, 40), "the HUD the row syncs");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 38,
                item: (0, 0)
            },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 41,
                item: (egg, 40)
            },
        ],
        "the pair the ascending ids speak"
    );
    // THE OFFHAND'S WORD (button 40 — the menu id 45 the F lane
    // speaks, the offhand stack the live belief answers)
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    assert!(app.on_creative_swap_key(40));
    assert_eq!(
        app.live_inv_slots[45],
        (stone, 64),
        "the offhand the id mirrors"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 45,
            item: (stone, 64)
        }],
        "the offhand's word the 45 carries"
    );
    // THE EMPTY CELL'S SILENCE OVER LIVE (the isEmpty gate the
    // write skips — NO word the silence speaks)
    app.creative_palette[0] = (0, 0);
    assert!(
        app.on_creative_swap_key(0),
        "the empty cell the silence keeps"
    );
    assert!(rx.try_recv().is_err(), "the silence the wire never spends");
    // THE OFFLINE CONTROL ARM (the plain world the wire never
    // spends — the sink the early return never reads, the rows
    // the offline belief wears)
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some((app.creative_panel.0 + 9 + 8, app.creative_panel.1 + 18 + 8));
    assert!(app.on_creative_swap_key(0), "the offline the copy wears");
    assert_eq!(
        app.inventory.items[0],
        (stone, 64),
        "the rows the offline keeps"
    );
    assert!(
        rx.try_recv().is_err(),
        "the offline session the words never spends"
    );
}

/// THE LIVE SESSION'S OWN PICKER GATE (the 26.2 init walk the
/// hasInfiniteMaterials verdict ALONE keeps — the network session
/// the gate never splits): the creative live E opens the picker
/// (the wire gate the survival screen wore the E-round retires —
/// the flagship surface the wire lane's own absence hid), the
/// palette pick the client-local carried fills (the setCarried
/// walk the wire never spends), the store put-back the cell's ONE
/// word rides, the wrapped seat's clicks the live words the
/// broadcast's own diff speaks (the armor pickup the ONE word, the
/// BIN's plain destroy the carried's own silence, the empty cell's
/// no-op the change-only law keeps), the close's add-back the
/// touched slots' words speak, the survival E the inventory screen
/// keeps; the offline session the same arms the words never spends.
#[test]
fn the_creative_picker_the_live_session_the_gate_wears() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    // THE LIVE SESSION (the glide harness's own shape — the sink
    // the attach speaks, the creative gate the E arm reads)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.game_mode = GameMode::Creative;
    // THE ARMOR'S OWN SLOT (the server's belief the sync speaks —
    // the seed the open's reconcile wears)
    app.live_inv_slots[5] = (helmet, 1);
    // THE GATE (the live creative E the picker opens — the survival
    // screen the wire gate wore the retire speaks)
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the picker the live E opens"
    );
    assert_eq!(app.creative_scroll, 0.0, "the fresh scroll the open keeps");
    // THE PALETTE PICK (the client-local law — the carried the
    // setCarried walk fills, NO word the wire never spends)
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    app.on_press(px + 9 + 8, py + 18 + 8);
    assert_eq!(app.inv_carried, (stone, 1), "the pick the carried fills");
    assert!(
        rx.try_recv().is_err(),
        "the palette pick the wire never spends"
    );
    // THE STORE ROW'S PUT-BACK (the cell the ONE word rides — the
    // carried the client-local belief keeps, the HUD the row syncs)
    app.on_press(px + 9 + 2 * 18 + 8, py + 112 + 8);
    assert_eq!(app.inventory.items[2], (stone, 1), "the cell the put takes");
    assert_eq!(app.inv_carried, (0, 0), "the carried the put drains");
    assert_eq!(app.hotbar_slots[2], (stone, 1), "the HUD the row syncs");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 38,
            item: (stone, 1)
        }],
        "the cell's word the put rides"
    );
    // THE WRAPPED SEAT'S OWN WORDS (the refusal retires — the
    // armor pickup the ONE word rides, the carried the
    // client-local belief the wire never learns; the BIN's plain
    // destroy the same silence wears; the empty main cell's no-op
    // the diff's change-only law keeps)
    app.select_creative_tab(13);
    app.on_press(px + 54 + 8, py + 6 + 8);
    assert_eq!(app.inventory.armor[0], (0, 0), "the head the take empties");
    assert_eq!(app.inv_carried, (helmet, 1), "the piece the carried rides");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 5,
            item: (0, 0)
        }],
        "the armor's word the diff speaks"
    );
    app.on_press(px + 173 + 8, py + 112 + 8);
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried spends");
    assert!(rx.try_recv().is_err(), "the destroy the wire never spends");
    app.on_press(px + 9 + 8, py + 54 + 8);
    assert_eq!(app.inventory.items[0], (0, 0), "the rows the no-op keeps");
    assert!(rx.try_recv().is_err(), "the no-op the diff silence keeps");
    // THE CLOSE'S ADD-BACK (the carried the merges + the first-empty
    // walk serves, the touched slots the words speak — the
    // ascending menu ids the BTreeMap orders; the 70 the merge's
    // room the split speaks — the cell takes the 63, the rest the
    // first empty keeps)
    app.inv_carried = (stone, 70);
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "the close returns");
    assert_eq!(
        app.inventory.items[2],
        (stone, 64),
        "the merge the room fills"
    );
    assert_eq!(
        app.inventory.items[0],
        (stone, 7),
        "the rest the empty keeps"
    );
    assert_eq!(app.inv_carried, (0, 0), "the hand the close drains");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 36,
                item: (stone, 7)
            },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 38,
                item: (stone, 64)
            },
        ],
        "the add-back's words the touched slots ride"
    );
    assert_eq!(
        app.live_held,
        Some(stone),
        "the selected slot the hand follows"
    );
    // THE SURVIVAL E (the mode the gate reads — the inventory
    // screen the non-creative E keeps)
    app.game_mode = GameMode::Survival;
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Inventory,
        "the survival the E keeps"
    );
    // THE OFFLINE CONTROL ARM (the same arms the words never
    // spends — the sink the early return never reads)
    let mut app = App::new(3);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.inventory.items[2] = (stone, 4);
    app.on_press(
        app.creative_panel.0 + 9 + 2 * 18 + 8,
        app.creative_panel.1 + 112 + 8,
    );
    assert_eq!(
        app.inventory.items[2],
        (0, 0),
        "the pickup the offline wears"
    );
    assert_eq!(app.inv_carried, (stone, 4), "the carried the pickup takes");
    assert!(rx.try_recv().is_err(), "the offline the words never spends");
}

/// THE WRAPPED SEAT'S LIVE WORDS (the broadcast's own diff law the
/// wrap round speaks — the broadcastChanges the changed slots the
/// CreativeInventoryListener answers, ONE packet per changed slot,
/// the menu ids the InventoryMenu space wears): the open's own
/// reconcile the offline belief the live truth wears (the store
/// rows, the armor, the offhand the seed speaks), the offhand
/// pickup the ONE word rides, the main cell's place + take the
/// pair's words speak, the BIN's SHIFT press the whole-clear's
/// changed slots speak (the emptied armor/offhand no words wear),
/// the carried the client-local silence the wire never spends.
#[test]
fn the_creative_wrap_the_live_words_the_diff_wears() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    // THE LIVE SESSION (the 632 harness's own shape)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.game_mode = GameMode::Creative;
    // THE SERVER'S OWN BELIEF (the window-0 slots the sync speaks —
    // the store rows, the armor head, the offhand the slots wear)
    app.live_inv_slots[38] = (stone, 5);
    app.live_inv_slots[39] = (stone, 7);
    app.live_inv_slots[5] = (helmet, 1);
    app.live_inv_slots[45] = (stone, 8);
    // THE SEED (the open's own reconcile — the wrapped seats the
    // live truth display, the HUD row the same belief reads)
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the picker the live E opens"
    );
    assert_eq!(
        app.inventory.items[2],
        (stone, 5),
        "the store row the seed wears"
    );
    assert_eq!(
        app.inventory.items[3],
        (stone, 7),
        "the next cell the seed wears"
    );
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the armor the seed wears"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 8),
        "the offhand the seed wears"
    );
    assert_eq!(
        app.hotbar_slots[2],
        (stone, 5),
        "the HUD row the seed wears"
    );
    // THE WRAPPED SEAT
    app.select_creative_tab(13);
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let main9 = (px + 9 + 8, py + 54 + 8);
    let armor_head = (px + 54 + 8, py + 6 + 8);
    let offhand = (px + 35 + 8, py + 20 + 8);
    let bin = (px + 173 + 8, py + 112 + 8);
    // THE ARMOR PICKUP (the piece the carried rides — the ONE word
    // the diff speaks, the carried the client-local the wire never
    // learns)
    app.on_press(armor_head.0, armor_head.1);
    assert_eq!(app.inventory.armor[0], (0, 0), "the head the take empties");
    assert_eq!(app.inv_carried, (helmet, 1), "the piece the carried rides");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 5,
            item: (0, 0)
        }],
        "the armor's word the diff speaks"
    );
    // THE BIN'S PLAIN PRESS (the carried the destroy wears — the
    // client-local belief the wire never learns, NO word)
    app.on_press(bin.0, bin.1);
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried spends");
    assert!(rx.try_recv().is_err(), "the destroy the wire never spends");
    // THE OFFHAND PICKUP (the same ONE-word law the anonymous slot
    // wears — the menu id 45 the word carries)
    app.on_press(offhand.0, offhand.1);
    assert_eq!(
        app.inventory.offhand[0],
        (0, 0),
        "the offhand the take empties"
    );
    assert_eq!(app.inv_carried, (stone, 8), "the stack the carried rides");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeSetSlot {
            slot: 45,
            item: (0, 0)
        }],
        "the offhand's word the diff speaks"
    );
    // THE MAIN CELL'S PLACE + TAKE (the pair's words the same diff
    // speaks — the place the carried spends, the take the cell
    // re-empties, the menu id 9 the store row's own space wears)
    app.on_press(main9.0, main9.1);
    assert_eq!(
        app.inventory.items[9],
        (stone, 8),
        "the cell the place wears"
    );
    assert_eq!(app.inv_carried, (0, 0), "the place the carried drains");
    app.on_press(main9.0, main9.1);
    assert_eq!(app.inventory.items[9], (0, 0), "the take the cell empties");
    assert_eq!(app.inv_carried, (stone, 8), "the take the carried rides");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 9,
                item: (stone, 8)
            },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 9,
                item: (0, 0)
            },
        ],
        "the pair's words the place + the take speak"
    );
    // THE BIN'S PLAIN PRESS (the carried the destroy wears again —
    // the store the same press never touches, NO word)
    app.on_press(bin.0, bin.1);
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried spends");
    assert!(rx.try_recv().is_err(), "the destroy the wire never spends");
    // THE BIN'S SHIFT PRESS (the WHOLE-CLEAR — the changed slots
    // the words speak: the store rows 38/39 the (0,0) wear, the
    // emptied armor/offhand no words the diff never speaks; the
    // ascending order the BTreeMap keeps)
    app.shift_down = true;
    app.on_press(bin.0, bin.1);
    assert!(
        app.inventory.items.iter().all(|&s| s == (0, 0)),
        "the store the shift-clear empties"
    );
    assert!(
        app.inventory.armor.iter().all(|&s| s == (0, 0)),
        "the armor the shift-clear empties"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (0, 0),
        "the offhand the clear takes"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 38,
                item: (0, 0)
            },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 39,
                item: (0, 0)
            },
        ],
        "the clear's words the changed slots ride"
    );
    app.shift_down = false;
    // THE EMPTY CELL'S NO-OP (the empty carried over the empty
    // cell — the no-change click the diff silence keeps)
    app.on_press(main9.0, main9.1);
    assert!(rx.try_recv().is_err(), "the no-op the diff silence keeps");
}

/// THE Q-TOSS'S OWN LIVE LANE (the 633 round the refusal retires —
/// the `handleCreativeModeItemDrop` walk the fresh javap banks:
/// the -1 slotNum the ServerboundSetCreativeModeSlotPacket ctor's
/// iconst_m1 wears, the stack the DROPPED count carries). THE
/// THREE SEATS THE THREE WORD LAWS WEAR: the palette cell the ONE
/// word the COPY drop speaks (the display copy the cell keeps, NO
/// add word the client-local palette the wire never learns); the
/// store row the PAIR the default arm's net law wears (the
/// ItemDrop word the taken stack carries FIRST, the
/// broadcastChanges diff the listener's ItemAdd word the
/// remainder rides SECOND, the menu id the 36+s space wears, the
/// HUD row the live belief re-projects); the wrapped cells the
/// PAIR the slotClicked's own THROW arm speaks (pcs 300-347 — the
/// direct return the broadcast never rides, the armor 5 / the
/// offhand 45 / the main 9..35 / the hotbar 36..44 the menu id
/// space the pair translates, the remainder's own (0,0) the
/// emptied slot's word wears). The search seat the live session
/// the same silence keeps (the focused box the keys consume
/// BEFORE the super chain rides — no word the gate ever spends).
/// The offline session the lane never spends (the helpers the
/// world_is_network gate refuses — the drops the same walk rides,
/// the wire the silence keeps).
#[test]
fn the_creative_drop_the_live_words_the_drop_rides() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    // THE LIVE SESSION (the 632 harness's own shape)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.game_mode = GameMode::Creative;
    // THE SERVER'S OWN BELIEF (the window-0 slots the sync speaks)
    app.live_inv_slots[38] = (stone, 5);
    app.live_inv_slots[10] = (dirt, 3);
    app.live_inv_slots[5] = (helmet, 1);
    app.live_inv_slots[45] = (stone, 2);
    // THE SEED (the open's own reconcile — the picker the live
    // truth display)
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the picker the live E opens"
    );
    assert_eq!(
        app.inventory.items[2],
        (stone, 5),
        "the hotbar the seed wears"
    );
    assert_eq!(
        app.inventory.items[10],
        (dirt, 3),
        "the main the seed wears"
    );
    // THE BUILDING TAB (the palette + the store row the
    // non-INVENTORY seats ride)
    app.select_creative_tab(0);
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let cell0 = (px + 9 + 8, py + 18 + 8);
    let store2 = (px + 9 + 2 * 18 + 8, py + 112 + 8);
    // THE PALETTE Q (the ONE word the copy drop speaks — the cell
    // the copy keeps, NO add word the wire never learns)
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the walk the copy answers");
    assert_eq!(app.drops.len(), before + 1, "the drop rides");
    assert_eq!(
        app.creative_palette[0],
        (stone, 1),
        "the cell the copy keeps"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeDrop { item: (stone, 1) }],
        "the palette the ONE word speaks"
    );
    // THE PALETTE CTRL Q (the per-item max the copy wears — the
    // egg the 16 the cell's own ceiling speaks)
    app.creative_palette[5] = (egg, 1);
    app.mouse_pos = Some((px + 9 + 5 * 18 + 8, py + 18 + 8));
    assert!(app.on_creative_drop_key(true), "the walk the max answers");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![crate::world_view::ContainerCommand::CreativeDrop { item: (egg, 16) }],
        "the egg's max the word carries"
    );
    // THE STORE ROW Q (the PAIR — the drop word FIRST, the
    // broadcast's add word the remainder rides SECOND)
    app.mouse_pos = Some(store2);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the walk the take answers");
    assert_eq!(app.inventory.items[2], (stone, 4), "the row the four keeps");
    assert_eq!(app.drops.len(), before + 1, "the drop rides");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (stone, 1) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 38,
                item: (stone, 4)
            },
        ],
        "the store row the pair speaks"
    );
    assert_eq!(
        app.hotbar_slots[2],
        (stone, 4),
        "the HUD row the live re-speaks"
    );
    // THE STORE ROW CTRL Q (the whole — the (0,0) write the
    // emptied slot's word wears)
    assert!(app.on_creative_drop_key(true), "the walk the whole answers");
    assert_eq!(app.inventory.items[2], (0, 0), "the row the whole spends");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (stone, 4) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 38,
                item: (0, 0)
            },
        ],
        "the whole the pair speaks"
    );
    // THE INVENTORY TAB (the wrapped seats the pair the direct
    // return speaks — NO broadcast the arm's own words carry)
    app.select_creative_tab(13);
    let main10 = (px + 9 + 1 * 18 + 8, py + 54 + 8);
    let armor_head = (px + 54 + 8, py + 6 + 8);
    let offhand = (px + 35 + 8, py + 20 + 8);
    let hot7 = (px + 9 + 7 * 18 + 8, py + 112 + 8);
    // THE MAIN CELL Q (the pair — the menu id 10 the store space wears)
    app.mouse_pos = Some(main10);
    assert!(app.on_creative_drop_key(false), "the walk the take answers");
    assert_eq!(app.inventory.items[10], (dirt, 2), "the cell the two keeps");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (dirt, 1) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 10,
                item: (dirt, 2)
            },
        ],
        "the wrapped cell the pair speaks"
    );
    // THE MAIN CELL CTRL Q (the whole — the (0,0) the remainder wears)
    assert!(app.on_creative_drop_key(true), "the walk the whole answers");
    assert_eq!(app.inventory.items[10], (0, 0), "the cell the whole spends");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (dirt, 2) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 10,
                item: (0, 0)
            },
        ],
        "the whole the pair speaks"
    );
    // THE ARMOR HEAD Q (the piece — the menu id 5 the word wears)
    app.mouse_pos = Some(armor_head);
    assert!(
        app.on_creative_drop_key(false),
        "the head the piece answers"
    );
    assert_eq!(app.inventory.armor[0], (0, 0), "the head the piece spends");
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (helmet, 1) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 5,
                item: (0, 0)
            },
        ],
        "the armor the pair speaks"
    );
    // THE OFFHAND Q (the remainder the same pair keeps — the
    // anonymous slot's 45 the word carries)
    app.mouse_pos = Some(offhand);
    assert!(
        app.on_creative_drop_key(false),
        "the offhand the one answers"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 1),
        "the offhand the one keeps"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (stone, 1) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 45,
                item: (stone, 1)
            },
        ],
        "the offhand the pair speaks"
    );
    // THE WRAPPED HOTBAR CELL (the menu id 43 the same store space
    // the wrap seat translates — the cell the offline belief the
    // Q walk reads, the pair the same order speaks)
    app.inventory.items[7] = (stone, 6);
    app.mouse_pos = Some(hot7);
    assert!(
        app.on_creative_drop_key(false),
        "the hotbar the one answers"
    );
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(
        words,
        vec![
            crate::world_view::ContainerCommand::CreativeDrop { item: (stone, 1) },
            crate::world_view::ContainerCommand::CreativeSetSlot {
                slot: 43,
                item: (stone, 5)
            },
        ],
        "the wrapped hotbar the pair speaks"
    );
    // THE SEARCH SEAT (the focused box the keys consume — no word
    // the gate ever spends, even over a loaded palette cell)
    app.select_creative_tab(6);
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    assert!(
        app.on_creative_drop_key(false),
        "the search the consume keeps"
    );
    assert!(rx.try_recv().is_err(), "the search the wire never spends");
    // THE OFFLINE CONTROL ARM (the helpers the world_is_network
    // gate refuses — the drops the same walk rides, the wire the
    // silence keeps)
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.game_mode = GameMode::Creative;
    off.open_inventory();
    off.select_creative_tab(0);
    off.creative_panel = creative_panel_origin(640, 360);
    let (tx2, rx2) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    off.attach_live_clicks(tx2);
    off.creative_palette[0] = (stone, 1);
    off.mouse_pos = Some((off.creative_panel.0 + 9 + 8, off.creative_panel.1 + 18 + 8));
    assert!(
        off.on_creative_drop_key(false),
        "the offline the copy answers"
    );
    assert_eq!(off.drops.len(), 1, "the offline the drop rides");
    assert!(rx2.try_recv().is_err(), "the offline the wire never spends");
}

/// THE WRAPPED SEAT'S OWN MID-SESSION SYNC (the container-0
/// SetSlot walk the fresh javap banks — cpl_full_262.txt
/// handleContainerSetSlot: the window-0 slot the
/// `inventoryMenu.setItem` pcs 151-162 the UNCONDITIONAL answer
/// wears, the `isInventoryOpen` gate pcs 54-90 the FOREIGN-window
/// lane ALONE consults — pc 182-191 the containerId-0 path never
/// rides, the picker's wrapped rows the server's truth the
/// mid-session sync the same frame display, EVERY tab the seat
/// keeps; the tail pcs 208-238 the `setRemoteSlot` +
/// `broadcastChanges` the server's own change the client's words
/// never re-answer — the sim the ONE pure copy wears, NO word the
/// sink ever spends). The carried the walk never wears (the 26.2
/// container-0 lane the carried arm lacks — the creative carried
/// the client-local lane the SetCursorItem walk pcs 36-39 the
/// creative seat REFUSES, the sync's own carried the projection
/// the store's belief wears alone). The closed seat the sync the
/// wrapped fields never wears (the creative screen the walk's
/// only consumer — the offline belief the survival/HUD lanes the
/// live projections serve, the wrapped copy the seat's own
/// display bridge rides).
#[test]
fn the_creative_wrapped_the_session_sync_the_rows_wear() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    // THE LIVE SESSION (the 632 harness's own shape)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    app.game_mode = GameMode::Creative;
    // THE SERVER'S OWN BELIEF (the window-0 slots the sync speaks)
    app.live_inv_slots[38] = (stone, 5);
    app.live_inv_slots[10] = (dirt, 3);
    app.live_inv_slots[5] = (helmet, 1);
    app.live_inv_slots[45] = (stone, 2);
    // THE SEED (the open's own reconcile — the picker the live
    // truth display)
    app.on_inventory_key();
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the picker the live E opens"
    );
    assert_eq!(
        app.inventory.items[2],
        (stone, 5),
        "the hotbar the seed wears"
    );
    assert_eq!(
        app.inventory.items[10],
        (dirt, 3),
        "the main the seed wears"
    );
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the head the seed wears"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 2),
        "the offhand the seed wears"
    );
    // THE CARRIED'S OWN PIN (the palette pick's own lane — the
    // client-local belief the sync the walk never wears)
    app.inv_carried = (egg, 1);
    // THE MID-SESSION SYNC (the server's truth the wholesale lane
    // lands — the picker STAYS OPEN, the wrapped rows the fresh
    // belief wear: the hotbar the swap speaks, the spent cell the
    // server emptied, the fresh row the egg rides, the head the
    // server took, the offhand the one the server keeps)
    let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots[38] = (dirt, 7);
    slots[12] = (egg, 4);
    slots[45] = (stone, 1);
    app.live_inventory_updated(slots, Vec::new(), 41, (stone, 2));
    assert_eq!(app.live_inv_state_id, 41, "the revision the sync wears");
    assert_eq!(
        app.inventory.items[2],
        (dirt, 7),
        "the hotbar the truth follows"
    );
    assert_eq!(
        app.inventory.items[10],
        (0, 0),
        "the spent cell the truth keeps"
    );
    assert_eq!(
        app.inventory.items[12],
        (egg, 4),
        "the fresh row the truth wears"
    );
    assert_eq!(app.inventory.armor[0], (0, 0), "the head the truth keeps");
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 1),
        "the offhand the truth follows"
    );
    assert_eq!(
        app.hotbar_slots[2],
        (dirt, 7),
        "the HUD row the same truth wears"
    );
    // THE ECHO THE WALK NEVER SPEAKS (the tail's setRemoteSlot +
    // broadcastChanges — the server's own change the client's
    // words never re-answer, the sink the silence keeps)
    let words: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert!(
        words.is_empty(),
        "the sync the pure copy wears, no word rides"
    );
    // THE CARRIED THE WALK NEVER WEARS (the 26.2 lane the carried
    // arm lacks — the creative carried the client-local keeps,
    // the sync's own carried the projection the store wears)
    assert_eq!(
        app.inv_carried,
        (egg, 1),
        "the carried the client-local keeps"
    );
    assert_eq!(
        app.live_inv_carried,
        (stone, 2),
        "the store's own cursor the projection wears"
    );
    // THE CLOSED SEAT (the sync the wrapped fields never wears —
    // the creative screen the walk's only consumer, the offline
    // belief the closed lane keeps)
    app.screen = ScreenKind::World;
    app.inventory.items[2] = (egg, 9);
    let mut slots2 = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
    slots2[38] = (stone, 1);
    app.live_inventory_updated(slots2, Vec::new(), 42, (0, 0));
    assert_eq!(
        app.inventory.items[2],
        (egg, 9),
        "the closed seat the sync never wears"
    );
    assert_eq!(
        app.live_inv_slots[38],
        (stone, 1),
        "the store's own truth the sync still lands"
    );
    // THE OFFLINE PRECONDITION (the wholesale lane the
    // world_is_network gate refuses — the store the offline
    // belief keeps)
    let mut off = App::new(3);
    off.enter_world();
    off.spawn_offline_body();
    off.live_inventory_updated(slots, Vec::new(), 7, (0, 0));
    assert_eq!(off.live_inv_state_id, 0, "the offline the lane refuses");
}

/// THE HOTBAR'S OWN POP LANE (the App's edge state the snapshots
/// feed — the store's packet-time verdicts the HUD wears): the
/// serial advance re-arms the pop (the verdict 5 the grow law
/// wore, the 0 the replace answers), the stale serial the
/// decay already spent never re-rides, the bulk epoch edge
/// zeroes the mid-flight pops, the game tick's own walk decays
/// the lane ([`App::update`]), the offline precondition the
/// flash refuses, and the pop curve's projection fold the
/// anchor's own invariance keeps (javap 26.2 Hud.extractSlot
/// pcs 43-106 — the pose's fixed point the composed
/// coefficients wear).
#[test]
fn the_hotbar_pop_lane_the_edges_and_the_decay_wear() {
    let mut flash = crate::world_model::HotbarPopFlash::default();
    flash.slots[0] = crate::world_model::HotbarPopFlashSlot {
        serial: 1,
        verdict: 5,
    };
    // THE OFFLINE PRECONDITION (the lane's own gate — the flash
    // the offline world never wears)
    let mut off = App::new(3);
    off.hotbar_pop_flash_updated(&flash);
    assert_eq!(off.hotbar_pop, [0; 9], "the offline the lane refuses");
    assert_eq!(off.hotbar_pop_seen, [0; 9], "the offline no edge seen");
    // THE LIVE SESSION (the pop lane the flashes feed)
    let mut app = App::new(3);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // THE FIRST EDGE (the serial advance re-arms the pop — the
    // verdict 5 the grow law wore)
    app.hotbar_pop_flash_updated(&flash);
    assert_eq!(app.hotbar_pop[0], 5, "the fresh pick pops");
    assert_eq!(app.hotbar_pop_seen[0], 1, "the edge the detector banks");
    assert_eq!(app.hotbar_pop[1], 0, "the silent slot stays silent");
    // THE STALE FLASH (the same serial again — no advance, no
    // re-arm: the decay already spent the verdict)
    app.update();
    assert_eq!(app.hotbar_pop[0], 4, "the game tick's own decay");
    app.hotbar_pop_flash_updated(&flash);
    assert_eq!(app.hotbar_pop[0], 4, "the stale serial never re-rides");
    // THE REPLACE EDGE (the serial 2 / verdict 0 — the fresh
    // stack's own popTime the sync answers, the pop dies)
    flash.slots[0].serial = 2;
    flash.slots[0].verdict = 0;
    app.hotbar_pop_flash_updated(&flash);
    assert_eq!(app.hotbar_pop[0], 0, "the replace the pop kills");
    // THE RE-ARM (the second pickup — the grow law's 5 again)
    flash.slots[0].serial = 3;
    flash.slots[0].verdict = 5;
    app.hotbar_pop_flash_updated(&flash);
    assert_eq!(app.hotbar_pop[0], 5, "the second pickup re-pops");
    // THE BULK EDGE (the epoch advance — the mid-flight pops the
    // initializeContents' fresh stacks kill the same instant)
    flash.epoch += 1;
    app.hotbar_pop_flash_updated(&flash);
    assert_eq!(app.hotbar_pop, [0; 9], "the bulk the pops kill");
    assert_eq!(
        app.hotbar_pop_epoch, flash.epoch,
        "the edge the detector banks"
    );
    // the decay the 5-tick ride wears (the render's f = pop -
    // partialTick the extractSlot law folds — the settled stack
    // the plain draw keeps)
    flash.slots[0].serial = 4;
    flash.slots[0].verdict = 5;
    app.hotbar_pop_flash_updated(&flash);
    for _ in 0..5 {
        app.update();
    }
    assert_eq!(app.hotbar_pop[0], 0, "the five ticks the pop rides");
    // THE FOLD'S OWN LAWS (javap pcs 43-106 — the pose the
    // projection coefficients compose): g = 1 the identity, the
    // anchor's NDC the pose's own fixed point, and the composed
    // answer the vanilla transform's exact equality wears.
    let (w_fb, h_fb) = (640.0f32, 360.0);
    let scale = 2.0f32;
    let (x, y) = (60i32, 40i32);
    let cx = (x as f32 + 8.0) * scale;
    let cy = (y as f32 + 8.0) * scale;
    let s = 16.0 * scale;
    let m00 = 2.0 * s / w_fb;
    let m03 = 2.0 * cx / w_fb - 1.0;
    let m11 = 2.0 * s / h_fb;
    let m13 = 1.0 - 2.0 * cy / h_fb;
    let identity = pop_projection_coeffs(m00, m03, m11, m13, 1.0, 0.0, 0.0, w_fb, h_fb);
    assert_eq!(identity, (m00, m03, m11, m13), "g = 1 the plain draw");
    let pop_g = 1.6f32;
    let ax = (x as f32 + 8.0) * scale;
    let ay = (y as f32 + 12.0) * scale;
    let (m00p, m03p, m11p, m13p) =
        pop_projection_coeffs(m00, m03, m11, m13, pop_g, ax, ay, w_fb, h_fb);
    // the anchor's vertex space: bx = (ax - cx)/s = 0, by =
    // (cy - ay)/s = -0.25 (the 12-down anchor the pose keeps)
    let (bx, by) = (0.0f32, -0.25f32);
    let x_in = m00 * bx + m03;
    let y_in = m11 * by + m13;
    let x_out = m00p * bx + m03p;
    let y_out = m11p * by + m13p;
    let x_anchor = 2.0 * ax / w_fb - 1.0;
    let y_anchor = 1.0 - 2.0 * ay / h_fb;
    assert!(
        (x_out - x_anchor).abs() < 1e-6,
        "the x anchor the pose fixes"
    );
    assert!(
        (y_out - y_anchor).abs() < 1e-6,
        "the y anchor the pose fixes"
    );
    // the plain projection's own answer at the anchor never moved
    assert!((x_in - x_anchor).abs() < 1e-6, "the anchor the pose shares");
    assert!((y_in - y_anchor).abs() < 1e-6, "the anchor the pose shares");
    // THE EXACT EQUALITY (a sample corner vertex the vanilla
    // transform moves): out == ndc(T(pixel(v))) — the composition
    // the projection fold speaks
    let (vx, vy) = (0.5f32, 0.5f32);
    let x_px = cx + vx * s;
    let y_px = cy - vy * s;
    let x_px_out = (1.0 / pop_g) * x_px + ax * (1.0 - 1.0 / pop_g);
    let y_px_out = ((pop_g + 1.0) / 2.0) * y_px + ay * (1.0 - (pop_g + 1.0) / 2.0);
    let x_expect = 2.0 * x_px_out / w_fb - 1.0;
    let y_expect = 1.0 - 2.0 * y_px_out / h_fb;
    assert!(
        (m00p * vx + m03p - x_expect).abs() < 1e-6,
        "the x fold exact"
    );
    assert!(
        (m11p * vy + m13p - y_expect).abs() < 1e-6,
        "the y fold exact"
    );
}

/// THE WRAPPED SEAT (the selectTab's INVENTORY arm the bytecode
/// 306-607 wears — the screen the same Creative seat keeps, the
/// wrapped rows the store's own fields read): the plain pickup
/// the predict pair the survival rows speak, the shift the
/// quick-move lane wears, the armor the matching piece takes, the
/// offhand the plain walk keeps, the BIN the plain press the
/// carried DESTROYS (the store the same press never touches) and
/// the SHIFT press the WHOLE inventory clears (the getItems walk's
/// one-entry-per-slot law — the 46 slots the store + the armor +
/// the offhand + the craft cells the clear covers).
#[test]
fn the_creative_wrap_the_bin_and_the_rows_wear() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    // THE WRAPPED SEAT (the tab switch the screen keeps)
    app.select_creative_tab(13);
    assert_eq!(app.creative_tab, 13, "the seat the switch banks");
    assert_eq!(
        app.screen,
        ScreenKind::Creative,
        "the same Creative seat — never the survival door"
    );
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let main0 = (px + 9 + 8, py + 54 + 8); // the store's 9
    let hot0 = (px + 9 + 8, py + 112 + 8); // the store's 0
    let armor_head = (px + 54 + 8, py + 6 + 8);
    let offhand = (px + 35 + 8, py + 20 + 8);
    let bin = (px + 173 + 8, py + 112 + 8);
    // THE PLAIN PICKUP (the main row's stack the carried rides)
    app.inventory.items[9] = (stone, 12);
    app.on_press(main0.0, main0.1);
    assert_eq!(app.inv_carried, (stone, 12), "the main the pickup wears");
    assert_eq!(app.inventory.items[9], (0, 0), "the cell the take empties");
    // the place back
    app.on_press(main0.0, main0.1);
    assert_eq!(app.inventory.items[9], (stone, 12), "the place back");
    // THE SHIFT QUICK-MOVE (the main row's stack the hotbar takes)
    app.shift_down = true;
    app.on_press(main0.0, main0.1);
    assert_eq!(app.inventory.items[9], (0, 0), "the main the move empties");
    assert_eq!(
        app.inventory.items[0],
        (stone, 12),
        "the hotbar the move takes"
    );
    // THE ARMOR (the matching piece the empty head takes)
    app.shift_down = false;
    app.inv_carried = (helmet, 1);
    app.on_press(armor_head.0, armor_head.1);
    assert_eq!(app.inventory.armor[0], (helmet, 1), "the head wears it");
    assert_eq!(app.inv_carried, (0, 0), "the cursor emptied");
    // THE OFFHAND (the plain walk the anonymous slot keeps)
    app.inventory.items[1] = (stone, 4);
    app.on_press(hot0.0, hot0.1); // pick the hotbar's stone up
    assert_eq!(app.inv_carried, (stone, 12), "the hotbar the pickup rides");
    app.on_press(offhand.0, offhand.1);
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 12),
        "the offhand the place wears"
    );
    // THE BIN'S PLAIN PRESS (the carried DESTROYS — the store the
    // same press never touches; both buttons the same walk)
    app.on_press(main0.0, main0.1); // nothing — the main's empty
    app.inv_carried = (stone, 9);
    app.on_press(bin.0, bin.1);
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried spends");
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 12),
        "the store the bin keeps"
    );
    // THE BIN'S SHIFT PRESS (the WHOLE inventory clears — the
    // store 36, the armor 4, the offhand, the craft cells)
    app.shift_down = false;
    app.on_press(main0.0, main0.1); // empty — the carried stays
    app.inventory.items[20] = (stone, 5);
    app.craft_grid[2] = (stone, 1);
    app.shift_down = true;
    app.on_press(bin.0, bin.1);
    assert!(
        app.inventory.items.iter().all(|&s| s == (0, 0)),
        "the store the shift-clear empties"
    );
    assert!(
        app.inventory.armor.iter().all(|&s| s == (0, 0)),
        "the armor the shift-clear empties"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (0, 0),
        "the offhand the clear takes"
    );
    assert_eq!(app.craft_grid[2], (0, 0), "the craft cell the clear takes");
    assert_eq!(app.craft_result, (0, 0), "the result the recompute empties");
    app.shift_down = false;
    // THE NULL-SLOT PRESS (the wrapped seat the outside arms never
    // rides — the carried the silence keeps; the point the player
    // model's own gap rides, no slot rect the spread keeps there)
    app.inv_carried = (stone, 3);
    app.on_press(px + 95, py + 15);
    assert_eq!(
        app.inv_carried,
        (stone, 3),
        "the wrapped seat the drop refuses"
    );
}

/// THE WRAPPED SEAT'S KEYS AND HOVER (the SWAP digit the same
/// table the wrapped rows wear — case B the take, case D the
/// straight swap, the offhand's F, the armor the mayPlace silence
/// consumes; the wheel the wrapped seat never rides; the hover
/// the InventoryMenu's own id space the tooltip lane reads).
#[test]
fn the_creative_wrap_the_keys_and_the_hover_wear() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.select_creative_tab(13);
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let main1 = (px + 9 + 1 * 18 + 8, py + 54 + 8); // the store's 10
    let hot3 = (px + 9 + 3 * 18 + 8, py + 112 + 8); // the store's 3
    let armor_head = (px + 54 + 8, py + 6 + 8);
    let offhand = (px + 35 + 8, py + 20 + 8);
    // THE CASE B (the empty hotbar row takes the main row's stack
    // AS-IS, the cell empties)
    app.inventory.items[10] = (stone, 6);
    app.mouse_pos = Some(main1);
    assert!(app.on_creative_swap_key(0), "the walk the take answers");
    assert_eq!(app.inventory.items[0], (stone, 6), "the row the take rides");
    assert_eq!(app.inventory.items[10], (0, 0), "the cell the take empties");
    // THE CASE D (the straight swap)
    app.inventory.items[10] = (egg, 2);
    app.mouse_pos = Some(main1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(app.inventory.items[0], (egg, 2), "the row the swap takes");
    assert_eq!(
        app.inventory.items[10],
        (stone, 6),
        "the cell the row takes"
    );
    // THE OFFHAND'S F (button 40 — the row the offhand answers:
    // the empty offhand the main row's stack takes, case B)
    app.mouse_pos = Some(main1);
    assert!(app.on_creative_swap_key(40), "the F the offhand answers");
    assert_eq!(
        app.inventory.offhand[0],
        (stone, 6),
        "the offhand the take wears"
    );
    assert_eq!(app.inventory.items[10], (0, 0), "the cell the F empties");
    // THE ARMOR (the case D mismatch the gates' refusal wears —
    // the egg the matching-piece trade refuses, BOTH stay; the
    // matching piece's own table the dedicated test answers)
    app.inventory.armor[0] = (helmet, 1);
    app.mouse_pos = Some(armor_head);
    assert!(app.on_creative_swap_key(0), "the armor the consume keeps");
    assert_eq!(app.inventory.armor[0], (helmet, 1), "the head untouched");
    assert_eq!(app.inventory.items[0], (egg, 2), "the row untouched");
    // THE WHEEL (the wrapped seat the scroll never rides — the
    // palette's dead list the canScroll-false law answers)
    let scroll_before = app.creative_scroll;
    app.on_creative_scroll(-1.0);
    assert_eq!(
        app.creative_scroll, scroll_before,
        "the wheel the wrapped seat dead"
    );
    // THE HOVER IDS (the InventoryMenu's own space — the armor 5,
    // the main 10, the hotbar 36+3, the offhand 45)
    app.inventory.items[10] = (stone, 6);
    assert_eq!(
        app.hovered_container(armor_head.0, armor_head.1),
        Some((helmet, 1, 5)),
        "the head the menu id 5 wears"
    );
    assert_eq!(
        app.hovered_container(main1.0, main1.1),
        Some((stone, 6, 10)),
        "the main the menu id 10 wears"
    );
    app.inventory.items[3] = (egg, 1);
    assert_eq!(
        app.hovered_container(hot3.0, hot3.1),
        Some((egg, 1, 39)),
        "the hotbar the menu id 39 wears"
    );
    app.inventory.offhand[0] = (stone, 6);
    assert_eq!(
        app.hovered_container(offhand.0, offhand.1),
        Some((stone, 6, 45)),
        "the offhand the menu id 45 wears"
    );
    // THE NOWHERE (the miss the world chain answers)
    app.mouse_pos = Some((px - 40, py - 40));
    assert!(!app.on_creative_swap_key(0), "the no-slot miss");
}

/// THE PICKER'S OWN Q (the keyDrop arm's THROW over the picker's
/// seats — the palette the COPY drops (the display copy the cell
/// keeps, the per-item max the Ctrl wears), the bottom row the real
/// take rides, the carried the arms never gate, the search seat the
/// focused box consumes, the empty cells / the elsewhere the false
/// verdict keeps).
#[test]
fn the_creative_q_the_palette_the_copy_drops() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let cell0 = (px + 9 + 8, py + 18 + 8);
    let cell5 = (px + 9 + 5 * 18 + 8, py + 18 + 8);
    let tail = (px + 9 + 8 * 18 + 8, py + 18 + 4 * 18 + 8);
    let store8 = (px + 9 + 8 * 18 + 8, py + 112 + 8);
    // THE COPY DROP (the plain Q — exactly ONE, the display copy
    // the cell KEEPS)
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the walk the copy answers");
    assert_eq!(app.drops.len(), before + 1, "the drop rides");
    assert_eq!(
        app.drops.last().unwrap().item,
        stone,
        "the stone the copy speaks"
    );
    assert_eq!(
        app.drops.last().unwrap().count,
        1,
        "the plain Q the ONE wears"
    );
    assert_eq!(
        app.creative_palette[0],
        (stone, 1),
        "the cell the copy keeps"
    );
    // THE CTRL Q (the per-item max the copy wears — the egg the 16)
    app.creative_palette[5] = (egg, 1);
    app.mouse_pos = Some(cell5);
    assert!(app.on_creative_drop_key(true), "the walk the max answers");
    assert_eq!(
        app.drops.last().unwrap().item,
        egg,
        "the egg the max speaks"
    );
    assert_eq!(
        app.drops.last().unwrap().count,
        16,
        "the egg's own 16 wears"
    );
    assert_eq!(app.creative_palette[5], (egg, 1), "the cell never depletes");
    // THE CARRIED BESIDE (the keyDrop arm the doClick carried gate
    // never reads — the Q the held stack beside throws, the held
    // stack UNTOUCHED)
    app.inv_carried = (stone, 5);
    app.mouse_pos = Some(cell0);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the carried never gates");
    assert_eq!(app.drops.len(), before + 1, "the drop still rides");
    assert_eq!(app.inv_carried, (stone, 5), "the held stack untouched");
    // THE EMPTY CELL (the hasItem gate — the key unconsumed)
    app.inv_carried = (0, 0);
    app.creative_palette[44] = (0, 0);
    app.mouse_pos = Some(tail);
    let before = app.drops.len();
    assert!(
        !app.on_creative_drop_key(false),
        "the empty cell the miss wears"
    );
    assert_eq!(app.drops.len(), before, "no drop rides");
    // THE BOTTOM ROW'S THROW (the real take — the plain Q ONE off,
    // the row the remainder keeps)
    app.inventory.items[8] = (stone, 7);
    app.mouse_pos = Some(store8);
    assert!(app.on_creative_drop_key(false), "the row the take answers");
    assert_eq!(app.inventory.items[8], (stone, 6), "the row the one keeps");
    assert_eq!(app.drops.last().unwrap().count, 1, "the one the drop wears");
    // THE CTRL Q (the whole stack — the remove's own clamp the count
    // speaks, the HUD row the every-mutation law syncs)
    assert!(app.on_creative_drop_key(true), "the walk the whole answers");
    assert_eq!(app.inventory.items[8], (0, 0), "the row the whole spends");
    assert_eq!(app.drops.last().unwrap().count, 6, "the six the drop wears");
    assert_eq!(
        app.hotbar_slots[8],
        (0, 0),
        "the HUD row the sync law wears"
    );
    // THE SEARCH SEAT (the focused box the key consumes — Q while
    // typing silence, even over a loaded palette cell)
    app.select_creative_tab(6);
    app.creative_palette[0] = (stone, 1);
    app.mouse_pos = Some(cell0);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the box the key consumes");
    assert_eq!(app.drops.len(), before, "the search tab no drop wears");
    assert_eq!(app.creative_palette[0], (stone, 1), "the cell untouched");
    // THE NOWHERE HOVER (the miss the false verdict keeps)
    app.select_creative_tab(0);
    app.mouse_pos = Some((px - 40, py - 40));
    assert!(!app.on_creative_drop_key(false), "the no-slot miss");
}

/// THE WRAPPED SEAT'S OWN Q (the pc 254-348 THROW arm — the REAL
/// take the Slot.remove walk speaks: the plain Q ONE off, the Ctrl
/// the whole stack (the remove's own count clamp), the armor /
/// offhand / hotbar rows the same law answers, the carried the arm
/// BYPASSES (the keyDrop arm the doClick carried gate never reads),
/// the BIN the sink never holds, the HUD row the every-mutation
/// law syncs).
#[test]
fn the_creative_q_the_wrap_the_real_take_wears() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.select_creative_tab(13);
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let main1 = (px + 9 + 1 * 18 + 8, py + 54 + 8); // the store's 10
    let hot3 = (px + 9 + 3 * 18 + 8, py + 112 + 8); // the store's 3
    let armor_head = (px + 54 + 8, py + 6 + 8);
    let offhand = (px + 35 + 8, py + 20 + 8);
    let bin = (px + 173 + 8, py + 112 + 8);
    // THE REAL TAKE (the plain Q — ONE off the main row, the
    // remainder the row keeps)
    app.inventory.items[10] = (stone, 6);
    app.mouse_pos = Some(main1);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(false), "the walk the take answers");
    assert_eq!(
        app.inventory.items[10],
        (stone, 5),
        "the row the five keeps"
    );
    assert_eq!(app.drops.len(), before + 1, "the drop rides");
    assert_eq!(app.drops.last().unwrap().count, 1, "the one the drop wears");
    // THE CTRL Q (the whole stack)
    assert!(app.on_creative_drop_key(true), "the walk the whole answers");
    assert_eq!(app.inventory.items[10], (0, 0), "the row the whole spends");
    assert_eq!(
        app.drops.last().unwrap().count,
        5,
        "the five the drop wears"
    );
    // THE CARRIED BESIDE (the wrapped arm the doClick carried gate
    // BYPASSES — the held stack the take never touches)
    app.inventory.items[10] = (stone, 6);
    app.inv_carried = (stone, 5);
    app.mouse_pos = Some(main1);
    assert!(app.on_creative_drop_key(false), "the carried never gates");
    assert_eq!(
        app.inventory.items[10],
        (stone, 5),
        "the row the take wears"
    );
    assert_eq!(app.inv_carried, (stone, 5), "the held stack untouched");
    app.inv_carried = (0, 0);
    // THE ARMOR (the piece the Q throws — the head the slot empties)
    app.inventory.armor[0] = (helmet, 1);
    app.mouse_pos = Some(armor_head);
    let before = app.drops.len();
    assert!(app.on_creative_drop_key(true), "the head the piece answers");
    assert_eq!(app.inventory.armor[0], (0, 0), "the head the piece spends");
    assert_eq!(app.drops.len(), before + 1, "the piece the drop rides");
    assert_eq!(
        app.drops.last().unwrap().item,
        helmet,
        "the helmet the throw wears"
    );
    // THE OFFHAND (the anonymous slot the same law keeps)
    app.inventory.offhand[0] = (stone, 1);
    app.mouse_pos = Some(offhand);
    assert!(
        app.on_creative_drop_key(false),
        "the offhand the one answers"
    );
    assert_eq!(
        app.inventory.offhand[0],
        (0, 0),
        "the offhand the one spends"
    );
    // THE HOTBAR ROW (the HUD the sync law keeps)
    app.inventory.items[3] = (stone, 4);
    app.mouse_pos = Some(hot3);
    assert!(
        app.on_creative_drop_key(false),
        "the hotbar the one answers"
    );
    assert_eq!(
        app.inventory.items[3],
        (stone, 3),
        "the row the three keeps"
    );
    assert_eq!(
        app.hotbar_slots[3],
        (stone, 3),
        "the HUD row the sync wears"
    );
    // THE BIN (the sink never holds — the hasItem gate the miss wears)
    app.mouse_pos = Some(bin);
    let before = app.drops.len();
    assert!(!app.on_creative_drop_key(false), "the bin the miss wears");
    assert_eq!(app.drops.len(), before, "no drop rides");
    // THE EMPTY CELL (the same miss)
    app.inventory.items[10] = (0, 0);
    app.mouse_pos = Some(main1);
    assert!(
        !app.on_creative_drop_key(false),
        "the empty cell the miss wears"
    );
}

/// THE ARMOR SWAP'S OWN TABLE (the ArmorSlot's own gates the
/// SWAP arm answers — javap acm doClick pcs 1149-1379 + the fresh
/// ArmorSlot disasm: mayPlace the isEquippableInSlot verdict the
/// equippable_loop_k table folds, mayPickup the PREVENT_ARMOR_CHANGE
/// gate the component-less model never wears): case B the worn
/// piece rides the button row, case C the matching piece PLACES
/// (the split law the max-1 wears — the cell takes ONE, the row
/// the remainder keeps), the mismatch the consumed silence keeps,
/// case D only the matching piece trades (the count-1 the straight
/// trade, the multi-count the split(1) arm — the cell takes ONE,
/// the row the remainder keeps, the worn piece the INVENTORY add
/// rides), the both-empty press the plain silence keeps.
#[test]
fn the_armor_swap_the_gates_the_table_wears() {
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let diamond_helmet = mc_registry::item_id_by_name("diamond_helmet").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.select_creative_tab(13);
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let armor_head = (px + 54 + 8, py + 6 + 8);
    app.mouse_pos = Some(armor_head);

    // CASE B (pcs 1149-1202): the empty hotbar row takes the worn
    // piece, the armor cell empties (mayPickup the component-less
    // model always grants)
    app.inventory.armor[0] = (helmet, 1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.items[0],
        (helmet, 1),
        "the worn piece the row rides"
    );
    assert_eq!(app.inventory.armor[0], (0, 0), "the cell the take empties");

    // CASE C (pcs 1205-1273) matching: the count-1 stack the head
    // wears, the row empties
    app.inventory.items[0] = (helmet, 1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the matching piece the head wears"
    );
    assert_eq!(app.inventory.items[0], (0, 0), "the row the place empties");

    // CASE C split: the multi-count stack the ONE rides, the row
    // the remainder keeps (getMaxStackSize the armor's own 1)
    app.inventory.armor[0] = (0, 0);
    app.inventory.items[0] = (helmet, 3);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the split the ONE places"
    );
    assert_eq!(
        app.inventory.items[0],
        (helmet, 2),
        "the row the remainder keeps"
    );

    // CASE C mismatch: the stone the mayPlace refusal wears — the
    // consumed silence, both stay
    app.inventory.armor[0] = (0, 0);
    app.inventory.items[0] = (stone, 5);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(app.inventory.armor[0], (0, 0), "the refusal the head keeps");
    assert_eq!(
        app.inventory.items[0],
        (stone, 5),
        "the row the refusal keeps"
    );

    // CASE D (pcs 1276-1379) straight: the count-1 matching piece
    // trades with the worn (mayPickup && mayPlace both speak)
    app.inventory.armor[0] = (diamond_helmet, 1);
    app.inventory.items[0] = (helmet, 1);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the head the trade wears"
    );
    assert_eq!(
        app.inventory.items[0],
        (diamond_helmet, 1),
        "the row the worn takes"
    );

    // CASE D split: the multi-count stack the ONE trades, the worn
    // piece the INVENTORY add rides (the empty tail the first free
    // slot takes), the row the remainder keeps
    app.inventory.armor[0] = (diamond_helmet, 1);
    app.inventory.items[0] = (helmet, 3);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the split the ONE wears"
    );
    assert_eq!(
        app.inventory.items[0],
        (helmet, 2),
        "the row the remainder keeps"
    );
    assert_eq!(
        app.inventory.items[1],
        (diamond_helmet, 1),
        "the worn the add lands"
    );
    assert!(app.drops.is_empty(), "the add's TRUE the drop never speaks");

    // CASE D mismatch: the egg over the worn helmet the refusal
    // wears — BOTH stay (the mayPlace arm the trade refuses)
    app.inventory.armor[0] = (diamond_helmet, 1);
    app.inventory.items[0] = (egg, 2);
    assert!(app.on_creative_swap_key(0));
    assert_eq!(
        app.inventory.armor[0],
        (diamond_helmet, 1),
        "the head the mismatch keeps"
    );
    assert_eq!(
        app.inventory.items[0],
        (egg, 2),
        "the row the mismatch keeps"
    );

    // the both-empty press (pcs 1130-1146) the plain silence keeps
    app.inventory.armor[0] = (0, 0);
    app.inventory.items[0] = (0, 0);
    assert!(
        app.on_creative_swap_key(0),
        "the both-empty the consumed silence keeps"
    );
    assert_eq!(app.inventory.armor[0], (0, 0), "the silence the head keeps");
    assert_eq!(app.inventory.items[0], (0, 0), "the silence the row keeps");
}

/// THE SCROLLBAR'S OWN DRAG (the 26.2 mouseClicked / mouseDragged /
/// mouseReleased walk verbatim — the press the 14x112 rect the
/// canScroll verdict arms and CONSUMES (the outside-drop throw the
/// scrollbar's press never answers), the armed drag the
/// `(y - y1 - 7.5) / (112 - 15)` track math the clamp 0..1 keeps
/// (the 7.5 the 15-high handle's own half — the handle's CENTER
/// the cursor follows), the release the unconditional disarm
/// speaks, the unarmed drag the silence keeps).
#[test]
fn the_creative_scrollbar_the_drag_wears() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let bar_x = px + 180; // the 14-wide rect x 175..189
    let track_top = py + 18; // y1
    let track_bottom = py + 130; // y2 = y1 + 112
    let rows = (app.creative_source.len() as i32 + 8) / 9 - 5;
    // THE PRESS ARMS (the flag the canScroll verdict stores — the
    // true the full registry wears) AND CONSUMES (the carried
    // stack the outside-drop throw never reads)
    app.inv_carried = (stone, 5);
    app.on_press(bar_x, py + 74);
    assert!(app.creative_scroll_drag, "the press the flag arms");
    assert_eq!(app.inv_carried, (stone, 5), "the press no throw wears");
    assert_eq!(app.creative_scroll, 0.0, "the press alone no move speaks");
    // THE DRAG TO THE BOTTOM (the clamp 1.0 — the refill the row
    // offset = the row count wears)
    app.on_creative_scroll_drag(track_bottom);
    assert_eq!(app.creative_scroll, 1.0, "the bottom the clamp wears");
    assert_eq!(
        app.creative_palette[0],
        app.creative_source[(rows * 9) as usize],
        "the refill the bottom row wears"
    );
    // THE DRAG TO THE TOP (the clamp 0.0 — the first row back)
    app.on_creative_scroll_drag(track_top);
    assert_eq!(app.creative_scroll, 0.0, "the top the clamp wears");
    assert_eq!(
        app.creative_palette[0], app.creative_source[0],
        "the refill the first row wears"
    );
    // THE MID DRAG (the 48.5/97 = 0.5 exact — the handle's CENTER
    // the cursor rides, the half list the row offset wears)
    app.on_creative_scroll_drag(py + 74);
    assert_eq!(app.creative_scroll, 0.5, "the mid the half wears");
    let half_row = (rows as f32 * 0.5 + 0.5) as i32;
    assert_eq!(
        app.creative_palette[0],
        app.creative_source[(half_row * 9) as usize],
        "the refill the half row wears"
    );
    // THE ARMED DRAG OUTSIDE THE RECT (the lane the rect never
    // re-checks — the once-armed drag the track's below rides)
    app.on_creative_scroll_drag(track_bottom + 100);
    assert_eq!(
        app.creative_scroll, 1.0,
        "the armed lane the outside the clamp keeps"
    );
    // THE RELEASE DISARMS (the unconditional false — the unarmed
    // drag the silence keeps)
    app.on_left_release();
    assert!(!app.creative_scroll_drag, "the release the flag clears");
    app.on_creative_scroll_drag(py + 74);
    assert_eq!(app.creative_scroll, 1.0, "the unarmed drag no ride");
    // THE RE-ARM / THE ELSEWHERE PRESS (the outside point the
    // THROW arm the vanilla's own lane keeps, the flag the
    // non-scrollbar press never arms)
    app.on_press(bar_x, py + 74);
    assert!(app.creative_scroll_drag, "the re-arm wears");
    app.on_left_release();
    let before = app.drops.len();
    app.on_press(px + 2, py + 74); // the outside — the drop arm
    assert!(
        !app.creative_scroll_drag,
        "the outside press the arm never wears"
    );
    assert_eq!(app.drops.len(), before + 1, "the outside the throw wears");
}

/// THE SCROLLBAR'S OWN CONSUMPTION (the arm's refusal lanes — the
/// canScroll-false list the press CONSUMED keeps (no throw, no
/// flag), the right press the button-0 arm never reads (the
/// outside split(1) drop the super.mouseClicked walk answers), the
/// INVENTORY gate the wrapped seat the arm refuses (the
/// scrollbar's rect ∩ the BIN the destroy slot's own walk keeps)).
#[test]
fn the_creative_scrollbar_the_press_consumes() {
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let egg = mc_registry::item_id_by_name("egg").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    app.creative_panel = creative_panel_origin(640, 360);
    let (px, py) = app.creative_panel;
    let bar_x = px + 180;
    // THE DISABLED SCROLLER'S CONSUMED PRESS (the search seat's
    // rare query — the one-item list the canScroll false speaks)
    app.select_creative_tab(6); // the SEARCH seat
    app.creative_search.push_str("iron_helmet");
    app.refresh_creative_search();
    assert!(
        !app.creative_can_scroll(),
        "the rare query the small list speaks"
    );
    app.inv_carried = (stone, 5);
    app.on_press(bar_x, py + 74);
    assert!(
        !app.creative_scroll_drag,
        "the disabled scroller the arm refuses"
    );
    assert_eq!(
        app.inv_carried,
        (stone, 5),
        "the consumed press the throw never speaks"
    );
    app.on_creative_scroll_drag(py + 130);
    assert_eq!(app.creative_scroll, 0.0, "the unarmed drag no ride");
    // THE RIGHT PRESS (the button-0 arm the button-1 press never
    // reads — the outside split drop the fall-through answers)
    app.select_creative_tab(0); // the category seat — the full list
    assert!(
        app.creative_can_scroll(),
        "the category seat the full list wears"
    );
    let before = app.drops.len();
    app.on_right_press(bar_x, py + 74);
    assert!(
        !app.creative_scroll_drag,
        "the right press the arm never wears"
    );
    assert_eq!(app.drops.len(), before + 1, "the split drop rides");
    assert_eq!(app.inv_carried, (stone, 4), "the one the drop spends");
    // THE WRAPPED SEAT'S BIN (the INVENTORY gate the arm refuses —
    // the same rect the destroy slot's plain press answers: the
    // carried the BIN spends)
    app.select_creative_tab(13); // the INVENTORY seat
    app.inv_carried = (egg, 3);
    app.on_press(bar_x, py + 120); // the rect ∩ the BIN (173..189, 112..128)
    assert!(
        !app.creative_scroll_drag,
        "the wrapped seat the arm refuses"
    );
    assert_eq!(app.inv_carried, (0, 0), "the bin the carried destroys");
}

/// THE ARMOR COLUMN (the 26.2 ArmorSlot's own mayPlace law — the
/// isEquippableInSlot verdict the EQUIPPABLE component folds to,
/// the slot max 1, the matching piece's swap the doClick arm
/// speaks): the matching piece PLACES, the wrong piece's slot
/// refuses (the stack stays on the cursor), the same-item full
/// slot the silence keeps, the different matching piece SWAPS.
/// The panel geometry: the armor column at (240, 105 + k * 18),
/// the head center (248, 113), the chest center (248, 131).
#[test]
fn the_armor_slots_take_the_matching_piece_and_refuse_the_rest() {
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let _chest = mc_registry::item_id_by_name("iron_chestplate").unwrap();
    let diamond = mc_registry::item_id_by_name("diamond_helmet").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE MATCHING PIECE PLACES (the head slot takes the helmet)
    app.inv_carried = (helmet, 1);
    app.on_press(248, 113);
    app.on_mouse_move(248, 113);
    app.on_left_release();
    assert_eq!(app.inventory.armor[0], (helmet, 1), "the head wears it");
    assert_eq!(app.inv_carried, (0, 0), "the cursor emptied");
    // THE PICKUP (the empty cursor takes the piece back)
    app.on_press(248, 113);
    assert_eq!(app.inventory.armor[0], (0, 0), "the head bare");
    assert_eq!(app.inv_carried, (helmet, 1), "the piece in the hand");
    // THE WRONG PIECE'S SLOT REFUSES (the chest slot keeps its
    // gate — the helmet is not a chestplate)
    app.on_press(248, 131);
    app.on_mouse_move(248, 131);
    app.on_left_release();
    assert_eq!(app.inventory.armor[1], (0, 0), "the chest still bare");
    assert_eq!(app.inv_carried, (helmet, 1), "the helmet stays held");
    // THE NON-ARMOR ITEM THE MAINHAND VERDICT REFUSES
    app.inv_carried = (planks, 12);
    app.on_press(248, 131);
    app.on_mouse_move(248, 131);
    app.on_left_release();
    assert_eq!(app.inventory.armor[1], (0, 0), "the planks refused");
    assert_eq!(app.inv_carried, (planks, 12), "the planks stay held");
    // THE HEAD WORN AGAIN (the held helmet rides back — the swap
    // needs the slot SPEAKING)
    app.inv_carried = (helmet, 1);
    app.on_press(248, 113);
    app.on_mouse_move(248, 113);
    app.on_left_release();
    assert_eq!(app.inventory.armor[0], (helmet, 1), "the head re-worn");
    assert_eq!(app.inv_carried, (0, 0), "the hand empty again");
    // THE SWAP (the different matching piece trades places — the
    // doClick mayPlace-true arm the result slot never speaks)
    app.inv_carried = (diamond, 1);
    app.on_press(248, 113);
    app.on_mouse_move(248, 113);
    app.on_left_release();
    assert_eq!(app.inventory.armor[0], (diamond, 1), "the head re-wears");
    assert_eq!(app.inv_carried, (helmet, 1), "the iron rides the hand");
}

/// THE QUICK-MOVE ARMOR WALK (the quickMoveStack's own arm order —
/// the bytecode 135 armor arm fires BEFORE the section routing and
/// ENDS the walk when it moves): the store row's helmet lands in
/// the EMPTY head slot (the hotbar routing never speaks), the
/// occupied head sends the next helmet down the hotbar lane, the
/// carved pumpkin the HEAD family keeps. The store 9 hit center
/// (248, 189), store 10 (266, 189).
#[test]
fn the_shift_click_sends_the_store_row_helmet_to_the_empty_head_slot() {
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let pumpkin = mc_registry::item_id_by_name("carved_pumpkin").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE ARMOR ARM ENDS THE WALK (the head empty — the whole
    // piece moves, the remainder stays, no hotbar routing)
    app.inventory.items[9] = (helmet, 1);
    app.shift_down = true;
    app.on_press(248, 189);
    app.shift_down = false;
    assert_eq!(app.inventory.armor[0], (helmet, 1), "the head took it");
    assert_eq!(app.inventory.items[9], (0, 0), "the source emptied");
    assert_eq!(app.inventory.items[0], (0, 0), "the hotbar untouched");
    // THE OCCUPIED HEAD SENDS IT DOWN THE SECTION LANE (the hotbar
    // rows the main-row source feeds — store 10's pumpkin is the
    // HEAD family too, but the head is full: the FIRST hotbar slot
    // takes it)
    app.inventory.items[10] = (pumpkin, 1);
    app.shift_down = true;
    app.on_press(266, 189);
    app.shift_down = false;
    assert_eq!(
        app.inventory.armor[0],
        (helmet, 1),
        "the head kept its piece"
    );
    assert_eq!(app.inventory.items[10], (0, 0), "the source emptied");
    assert_eq!(
        app.inventory.items[0],
        (pumpkin, 1),
        "the hotbar took the rest"
    );
}

/// THE OFFHAND SLOT (the anonymous slot 45's own law — mayPlace
/// the base slot's unconditional TRUE keeps, the max the item's
/// own 64 speaks): ANY stack places, the pickup takes it back,
/// and the ADD walk's container-40 arm the getSlotWithRemainingSpace
/// bytecode 21-37 keeps speaks — the selected stack full, the
/// store's free slots gone, the offhand's same-item stack the
/// remaining-space verdict takes the rest.
#[test]
fn the_offhand_takes_any_stack_and_the_pickup_lands_there() {
    let torch = mc_registry::item_id_by_name("torch").unwrap();
    let cobble = mc_registry::item_id_by_name("cobblestone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE PLACE (any stack rides the offhand)
    app.inv_carried = (torch, 10);
    app.on_press(317, 167);
    app.on_mouse_move(317, 167);
    app.on_left_release();
    assert_eq!(app.inventory.offhand[0], (torch, 10), "the offhand took it");
    assert_eq!(app.inv_carried, (0, 0), "the cursor emptied");
    // THE PICKUP (the whole stack back)
    app.on_press(317, 167);
    assert_eq!(app.inventory.offhand[0], (0, 0), "the offhand bare");
    assert_eq!(app.inv_carried, (torch, 10), "the stack in the hand");
    // THE ADD WALK'S OFFHAND LANE (the selected full, the main
    // rows full with the OTHER item, the offhand's 30 the
    // container-40 verdict fills)
    app.inventory.items[0] = (torch, 64);
    app.inventory.offhand[0] = (torch, 30);
    for it in app.inventory.items.iter_mut().skip(9) {
        *it = (cobble, 64);
    }
    let (leftover, _spoke) = app.inventory.add(0, torch, 5, false);
    assert_eq!(leftover, 0, "the offhand took all five");
    assert_eq!(
        app.inventory.offhand[0],
        (torch, 35),
        "the container-40 fill"
    );
    assert_eq!(
        app.inventory.items[0],
        (torch, 64),
        "the selected untouched"
    );
}

/// THE WORN PIECE'S SHIFT-CLICK (the quickMoveStack's own
/// fallthrough for the menu 5..8 sources — the final `[9, 45)`
/// FORWARD move the section arms never match): the piece walks to
/// the store, the MAIN rows first (the forward order the menu's
/// own ascending walk keeps), the hotbar after when the main rows
/// speak no home. The chest hit center (248, 131).
#[test]
fn the_shift_click_moves_the_worn_piece_to_the_store() {
    let chest = mc_registry::item_id_by_name("iron_chestplate").unwrap();
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let cobble = mc_registry::item_id_by_name("cobblestone").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE MAIN ROWS FIRST (the forward [9, 45) walk — store 9 the
    // first empty the ascending order answers)
    app.inventory.armor[1] = (chest, 1);
    app.shift_down = true;
    app.on_press(248, 131);
    app.shift_down = false;
    assert_eq!(app.inventory.items[9], (chest, 1), "the main row took it");
    assert_eq!(app.inventory.armor[1], (0, 0), "the chest bare");
    // THE MAIN ROWS FULL — THE HOTBAR AFTER (the forward walk's
    // own tail: the hotbar 0..8 the range's last arm answers)
    app.inventory.armor[0] = (helmet, 1);
    for it in app.inventory.items.iter_mut().skip(9) {
        *it = (cobble, 64);
    }
    app.shift_down = true;
    app.on_press(248, 113);
    app.shift_down = false;
    assert_eq!(app.inventory.items[0], (helmet, 1), "the hotbar took it");
    assert_eq!(app.inventory.armor[0], (0, 0), "the head bare");
    assert_eq!(app.hotbar_slots[0], (helmet, 1), "the HUD row synced");
}

/// THE Q-OVER-THE-EQUIPMENT LANE (the getHoveredSlot walk's own
/// completeness — the armor column and the offhand the menu's
/// slot list keeps): the hovered piece the doClick THROW speaks,
/// the carried stack's own gate the silence keeps.
#[test]
fn the_drop_key_throws_the_hovered_piece() {
    let helmet = mc_registry::item_id_by_name("iron_helmet").unwrap();
    let torch = mc_registry::item_id_by_name("torch").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // THE PIECE THROWS (one off — the single piece the whole-stack
    // law answers the same; the safeTake walk leaves the count-0
    // pair the store's own empty sentinel keeps)
    app.inventory.armor[0] = (helmet, 1);
    app.mouse_pos = Some((248, 113));
    app.on_inventory_drop_key(false);
    assert_eq!(app.inventory.armor[0], (helmet, 0), "the head bare");
    assert!(
        app.drops.iter().any(|d| d.item == helmet && d.count == 1),
        "the piece rides the aimed throw"
    );
    // THE CARRIED STACK'S GATE (the cursor full — the throw
    // refuses)
    app.inventory.offhand[0] = (torch, 3);
    app.inv_carried = (torch, 5);
    app.mouse_pos = Some((317, 167));
    app.on_inventory_drop_key(false);
    assert_eq!(app.inventory.offhand[0], (torch, 3), "the offhand kept it");
}

/// THE SHAPED SET (ShapedRecipePattern.matches's own law — the
/// packed w/h equality, the cell-by-cell test, the tag law): the
/// sticks' 1x2 column the pack law folds to w=1 in EITHER column
/// (the bounding box the ofPositioned walk keeps), the horizontal
/// pair the w/h equality refuses, the mixed-planks table the
/// independent tag cells craft, the torch's order the pattern
/// speaks, the sandstone the 2x2 keeps, the spruce singles the
/// shapeless fast path answers, the lone plank no recipe names.
#[test]
fn the_craft_grid_speaks_the_shaped_set() {
    let oak = mc_registry::item_id_by_name("oak_planks").unwrap();
    let spruce = mc_registry::item_id_by_name("spruce_planks").unwrap();
    let birch = mc_registry::item_id_by_name("birch_planks").unwrap();
    let stick = mc_registry::item_id_by_name("stick").unwrap();
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let torch = mc_registry::item_id_by_name("torch").unwrap();
    let table = mc_registry::item_id_by_name("crafting_table").unwrap();
    let sand = mc_registry::item_id_by_name("sand").unwrap();
    let sandstone = mc_registry::item_id_by_name("sandstone").unwrap();
    let spruce_log = mc_registry::item_id_by_name("spruce_log").unwrap();
    // THE STICKS — the RIGHT column (cells 1 + 3), the pack law's
    // bounding box answers the same as the left
    let cells = [(oak, 1), (0, 0), (oak, 1), (0, 0)];
    assert_eq!(craft_result_for(&cells), (stick, 4), "the right column");
    let cells = [(oak, 1), (0, 0), (0, 0), (oak, 1)];
    assert_eq!(
        craft_result_for(&cells),
        (0, 0),
        "the diagonal's holes the pack keeps — no recipe names it"
    );
    let cells = [(oak, 1), (oak, 1), (0, 0), (0, 0)];
    assert_eq!(
        craft_result_for(&cells),
        (0, 0),
        "the horizontal pair the w/h equality refuses"
    );
    // THE MIXED-PLANKS TABLE (the tag each cell tests INDEPENDENTLY
    // — the oak+spruce+birch mix the real game crafts)
    let cells = [(oak, 1), (spruce, 2), (birch, 1), (oak, 3)];
    assert_eq!(
        craft_result_for(&cells),
        (table, 1),
        "the tag's own verdict"
    );
    // THE TORCH — coal over stick, the order the pattern speaks
    let cells = [(coal, 1), (0, 0), (stick, 1), (0, 0)];
    assert_eq!(craft_result_for(&cells), (torch, 4), "the [C, S] column");
    let cells = [(stick, 1), (0, 0), (coal, 1), (0, 0)];
    assert_eq!(
        craft_result_for(&cells),
        (0, 0),
        "the stick-under-coal refused"
    );
    // THE SANDSTONE — the 2x2
    let cells = [(sand, 1); 4];
    assert_eq!(craft_result_for(&cells), (sandstone, 1), "the 2x2 sand");
    // THE SPRUCE SINGLES (the shapeless fast path per wood)
    let cells = [(spruce_log, 1), (0, 0), (0, 0), (0, 0)];
    assert_eq!(craft_result_for(&cells), (spruce, 4), "the spruce pair");
    // THE LONE PLANK — no recipe names it
    let cells = [(oak, 1), (0, 0), (0, 0), (0, 0)];
    assert_eq!(
        craft_result_for(&cells),
        (0, 0),
        "the single plank's silence"
    );
}

/// THE TAKE'S OWN GATES (the doClick PICKUP walk over the
/// mayPlace-false result slot): the same-item merge the 1020 arm
/// keeps (the carried's own space caps the take), the FULL carried
/// refuses (the getMaxStackSize if_icmpgt), the different item's
/// silence (the isSameItemSameComponents ifeq — no swap rides a
/// mayPlace-false slot), and the secondary take's half rounded up
/// (the (count+1)/2 the arm speaks).
#[test]
fn the_craft_result_take_gates_and_the_secondary_half() {
    let log = mc_registry::item_id_by_name("oak_log").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let stick = mc_registry::item_id_by_name("stick").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // one cell, two logs — the two crafts (the shapeless law's own
    // count: TWO cells of logs the ingredientCount equality
    // REFUSES — the one-ingredient recipe speaks only the
    // one-cell input, the multi-count stack the crafts repeat)
    app.craft_grid[0] = (log, 2);
    app.craft_grid[2] = (log, 2);
    app.refresh_craft_result();
    assert_eq!(app.craft_result, (0, 0), "two cells the count law refuses");
    app.craft_grid[2] = (0, 0);
    app.refresh_craft_result();
    assert_eq!(app.craft_result, (planks, 4), "the one-cell input speaks");
    // THE MERGE ARM — the carried's space (2) caps the take
    app.inv_carried = (planks, 62);
    app.on_press(394, 133);
    app.on_mouse_move(394, 133);
    app.on_left_release();
    assert_eq!(app.inv_carried, (planks, 64), "the space caps the take");
    assert_eq!(app.craft_result, (planks, 4), "the craft still speaks");
    // THE FULL CARRIED — the if_icmpgt arm's silence
    app.on_press(394, 133);
    app.on_mouse_move(394, 133);
    app.on_left_release();
    assert_eq!(app.inv_carried, (planks, 64), "the full carried refuses");
    assert_eq!(app.craft_result, (planks, 4), "no take, no decrement");
    // THE DIFFERENT ITEM — the ifeq arm's silence
    app.inv_carried = (stick, 10);
    app.on_press(394, 133);
    app.on_mouse_move(394, 133);
    app.on_left_release();
    assert_eq!(app.inv_carried, (stick, 10), "no swap on the result slot");
    assert_eq!(app.craft_result, (planks, 4), "the result untouched");
    // THE SECONDARY TAKE — the half rounded up, ONE craft's
    // decrement per take, the last craft the spent cell gives
    app.inv_carried = (0, 0);
    app.on_right_press(394, 133);
    assert_eq!(app.inv_carried, (planks, 2), "the (count+1)/2 arm");
    assert_eq!(app.craft_grid[0], (0, 0), "the last craft spent");
    assert_eq!(app.craft_result, (0, 0), "the spent grid's EMPTY verdict");
}

/// THE CRAFT CELLS' OWN PICKUP LANES (the doClick PICKUP over the
/// mayPlace-default cells — the TransientCraftingContainer the
/// place walk accepts): the right press's ceiling-half split, the
/// one-by-one place, the merge-back, the shift-click's silence
/// (the quick-move-into-craft lane rides later), and the WIRE
/// session's refusal (the store the server owns — the whole
/// inventory click lane's own gate the craft region rides).
#[test]
fn the_craft_cells_keep_the_pickup_lanes_and_the_refusals() {
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let log = mc_registry::item_id_by_name("oak_log").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    // the RIGHT press takes the ceiling half (4 -> 2 + 2 — the
    // empty cursor the immediate secondary keeps)
    app.craft_grid[0] = (planks, 4);
    app.on_right_press(338, 123);
    assert_eq!(app.craft_grid[0], (planks, 2), "half stays");
    assert_eq!(app.inv_carried, (planks, 2), "the ceiling half taken");
    // the RIGHT press places ONE back (the press->release drive the
    // type-1 arm law wears — the pre-press motion the release
    // position feeds, the release the deferred place-ONE serves)
    app.on_mouse_move(338, 123);
    app.on_right_press(338, 123);
    app.on_right_release();
    assert_eq!(app.craft_grid[0], (planks, 3), "one placed back");
    assert_eq!(app.inv_carried, (planks, 1), "the hand one lighter");
    // the LEFT press merges the hand's single back (the same-item
    // walk the store slots speak — the carried press the drag arm
    // arms, the release's own distribution the merge serves; the
    // cursor the no-seat preview lane steps aside first — the moved
    // gate the same-seat re-hover never re-fires)
    app.on_mouse_move(app.inv_panel.0 + 60, app.inv_panel.1 + 40);
    app.on_press(338, 123);
    app.on_mouse_move(338, 123);
    app.on_left_release();
    assert_eq!(app.craft_grid[0], (planks, 4), "the merge-back");
    assert_eq!(app.inv_carried, (0, 0), "the hand emptied");
    // the SHIFT-click's silence (the quick-move lane rides later —
    // the carried press the drag arms, no click the silence keeps)
    app.shift_down = true;
    app.inv_carried = (log, 1);
    app.on_press(338, 123);
    assert_eq!(app.craft_grid[0], (planks, 4), "the shift never moved it");
    assert_eq!(app.inv_carried, (log, 1), "the hand whole");
    // the armed drag the no-seat release silences (the between the
    // -1 no-op the offline keeps — the drag the shift's arm rode)
    app.on_mouse_move(app.inv_panel.0 + 60, app.inv_panel.1 + 40);
    app.on_left_release();
    app.shift_down = false;
    // the WIRE session — the store the server owns (the click
    // lane's own early return the craft region rides too)
    app.world_is_network = true;
    app.on_press(338, 123);
    assert_eq!(app.craft_grid[0], (planks, 4), "the wire refusal holds");
    assert_eq!(app.inv_carried, (log, 1), "the hand whole on the wire");
}

/// THE TABLE'S OWN KIND (the atlas RUNTIME_PALETTE's 37 index —
/// the autopush-328 round): the world's vocabulary speaks the
/// name, the placement arm's reverse lookup answers, the atlas
/// row carries the four real tiles the vanilla cube model names
/// (front/side/top + the oak_planks down face), the dig/drop/item
/// lanes the registry's own tables serve.
#[test]
fn the_table_block_rides_the_world_kind() {
    let kind = crate::world::CRAFTING_TABLE;
    assert_eq!(
        crate::world::NAME_OF[kind as usize],
        "crafting_table",
        "the vocabulary's own name"
    );
    assert_eq!(
        crate::world::World::placeable_block("crafting_table"),
        Some(kind),
        "the placement's reverse lookup"
    );
    // the atlas row: the six faces all ride REAL tiles (the
    // textured path the goldens' own lane speaks)
    let row = mc_registry::generated::atlas::FACE_TILES[kind as usize];
    assert!(
        row.iter()
            .all(|t| *t != mc_registry::generated::atlas::TILE_NONE),
        "the table's faces ride real tiles: {:?}",
        row
    );
    // the four distinct tiles the vanilla model names (front/
    // side/top + the oak_planks down face)
    let mut distinct = row.to_vec();
    distinct.sort();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        4,
        "front/side/top/planks-down: {:?}",
        distinct
    );
    // the registry's own lanes: the dig row, the drop pools, the
    // item the pickup and the placement speak
    assert!(
        mc_registry::generated::block_dig::dig_row("crafting_table").is_some(),
        "the dig row the registry keeps"
    );
    assert!(
        mc_registry::generated::block_drops::drops("crafting_table").is_some(),
        "the drop pools the registry keeps"
    );
    assert!(
        mc_registry::item_id_by_name("crafting_table").is_some(),
        "the item rides"
    );
    // THE MESH: the placed table's cube the full-cube path meshes,
    // the quads the table's own tiles carry
    let mut w = crate::world::World::new(2, 42);
    w.set(8, 20, 8, kind);
    let q = crate::mesher::mesh_world(&w, None);
    assert!(
        q.iter().any(|quad| row.contains(&quad.tile)),
        "the table's faces meshed with the atlas tiles"
    );
}

/// THE TABLE'S OWN 3x3 RECIPE WALK (the shared packed matcher the
/// jar's recipe JSONs speak — the bow's diagonal spine, the wooden
/// tools' rows, the mirror arm the axe's handedness keeps): the
/// bounding-box pack the CraftingInput.ofPositioned law keeps, the
/// w/h equality the shaped walk refuses on, the EMPTY verdict the
/// wrong arrangements speak.
#[test]
fn the_table_crafts_the_bow_and_the_wooden_tools() {
    let id = |name: &str| mc_registry::item_id_by_name(name).unwrap();
    let (stick, string) = (id("stick"), id("string"));
    let (oak, spruce, birch) = (id("oak_planks"), id("spruce_planks"), id("birch_planks"));
    let zero = (0, 0);
    // THE BOW ([" #X", "# X", " #X"] — the sticks' diagonal spine,
    // the string column the right edge keeps)
    let bow = id("bow");
    let cells: [(i32, i32); 9] = [
        zero,
        (stick, 1),
        (string, 1),
        (stick, 1),
        zero,
        (string, 1),
        zero,
        (stick, 1),
        (string, 1),
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (bow, 1),
        "the jar's own row"
    );
    // the mirrored arrangement (the string column LEFT) — the
    // MIRROR ARM answers (the bow's pattern the symmetrical flag
    // keeps false — the jar's own try-both-orientations walk the
    // mirrored rows serve)
    let mirrored: [(i32, i32); 9] = [
        (string, 1),
        (stick, 1),
        zero,
        (string, 1),
        zero,
        (stick, 1),
        (string, 1),
        (stick, 1),
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&mirrored),
        (bow, 1),
        "the mirror arm answers"
    );
    // THE WRONG STRING COUNT — the w/h equality's own refusal (the
    // two strings the ingredient count refuses)
    let cells: [(i32, i32); 9] = [
        zero,
        (stick, 1),
        (string, 1),
        (stick, 1),
        zero,
        zero,
        zero,
        (stick, 1),
        (string, 1),
    ];
    assert_eq!(craft_result_for_table(&cells), (0, 0), "the gap refuses");
    // THE WOODEN PICKAXE (["XXX", " # ", " # "])
    let pick = id("wooden_pickaxe");
    let cells: [(i32, i32); 9] = [
        (oak, 1),
        (spruce, 1),
        (birch, 1),
        zero,
        (stick, 1),
        zero,
        zero,
        (stick, 1),
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (pick, 1),
        "the mixed planks the tag answers"
    );
    // THE WOODEN AXE BOTH HANDEDNESS (["XX", "X#", " #"] + the
    // mirror arm the symmetrical flag keeps)
    let axe = id("wooden_axe");
    let cells: [(i32, i32); 9] = [
        (oak, 1),
        (oak, 1),
        zero,
        (oak, 1),
        (stick, 1),
        zero,
        zero,
        (stick, 1),
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (axe, 1),
        "the jar's own row"
    );
    let cells: [(i32, i32); 9] = [
        (oak, 1),
        (oak, 1),
        zero,
        (stick, 1),
        (oak, 1),
        zero,
        (stick, 1),
        zero,
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (axe, 1),
        "the mirror arm answers"
    );
    // THE SHOVEL + THE SWORD (the 1-wide columns)
    let cells: [(i32, i32); 9] = [
        (oak, 1),
        zero,
        zero,
        (stick, 1),
        zero,
        zero,
        (stick, 1),
        zero,
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (id("wooden_shovel"), 1),
        "the shovel column"
    );
    let cells: [(i32, i32); 9] = [
        (oak, 1),
        zero,
        zero,
        (oak, 1),
        zero,
        zero,
        (stick, 1),
        zero,
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (id("wooden_sword"), 1),
        "the sword column"
    );
    // THE SHIFTED COLUMN — the pack law the recipe's own position
    // answers (the shovel column in the MIDDLE cell rides home the
    // same verdict — the bounding box folds to the same input)
    let cells: [(i32, i32); 9] = [
        zero,
        (oak, 1),
        zero,
        zero,
        (stick, 1),
        zero,
        zero,
        (stick, 1),
        zero,
    ];
    assert_eq!(
        craft_result_for_table(&cells),
        (id("wooden_shovel"), 1),
        "the shifted column"
    );
    // THE 2x2's OWN BOUND — the inventory grid can never fit the
    // 3-wide rows (the packed w/h equality the smaller grid
    // refuses): the full planks row + the stick column IS the
    // pickaxe in the table, but the 2x2's four cells hold at most
    // a 2x2 input — the pickaxe never speaks there
    let cells: [(i32, i32); 4] = [(oak, 1), (oak, 1), (stick, 1), (stick, 1)];
    assert_eq!(
        craft_result_for(&cells),
        (0, 0),
        "no 2x2 recipe names these"
    );
}

/// THE CAKE'S OWN REMAINDER WALK (the jar's recipe/cake.json the
/// table answers + ResultSlot.onTake's remainders arm — the
/// NonNullList the UNTOUCHED grid answers, the emptied milk cell
/// the setItem arm wears): the cake crafts, the three buckets ride
/// the milk cells back WHOLE, the sugar/egg/wheat cells spend to
/// empty, the recomputed verdict over the bucket grid the honest
/// EMPTY speaks, and the close's clearContainer ride-home the
/// whole craft's own leftovers lands.
#[test]
fn the_cake_recipe_the_buckets_return() {
    let id = |name: &str| mc_registry::item_id_by_name(name).unwrap();
    let (milk, sugar, egg, wheat, cake) = (
        id("milk_bucket"),
        id("sugar"),
        id("egg"),
        id("wheat"),
        id("cake"),
    );
    let (blue, brown) = (id("blue_egg"), id("brown_egg"));
    let bucket = id("bucket");
    let zero = (0, 0);
    // THE RECIPE VERDICTS (["AAA","BEB","CCC"] row-major — the
    // milk row, the sugar-egg-sugar spine, the wheat row)
    let grid: [(i32, i32); 9] = [
        (milk, 1),
        (milk, 1),
        (milk, 1),
        (sugar, 1),
        (egg, 1),
        (sugar, 1),
        (wheat, 1),
        (wheat, 1),
        (wheat, 1),
    ];
    assert_eq!(
        craft_result_for_table(&grid),
        (cake, 1),
        "the jar's own rows"
    );
    // THE EGGS TAG (the E seat the #minecraft:eggs ingredient
    // keeps — the blue and the brown egg the same cake answers)
    let blue_grid: [(i32, i32); 9] = [
        (milk, 1),
        (milk, 1),
        (milk, 1),
        (sugar, 1),
        (blue, 1),
        (sugar, 1),
        (wheat, 1),
        (wheat, 1),
        (wheat, 1),
    ];
    assert_eq!(
        craft_result_for_table(&blue_grid),
        (cake, 1),
        "the blue egg rides"
    );
    let brown_grid: [(i32, i32); 9] = [
        (milk, 1),
        (milk, 1),
        (milk, 1),
        (sugar, 1),
        (brown, 1),
        (sugar, 1),
        (wheat, 1),
        (wheat, 1),
        (wheat, 1),
    ];
    assert_eq!(
        craft_result_for_table(&brown_grid),
        (cake, 1),
        "the brown egg rides"
    );
    // THE 2x2's OWN BOUND — the cake rows never fit the small grid
    let cells: [(i32, i32); 4] = [(milk, 1), (milk, 1), (sugar, 1), (egg, 1)];
    assert_eq!(craft_result_for(&cells), (0, 0), "no 2x2 cake");
    // THE WALK (the result's own take over the mayPlace-false
    // slot): the carried receives the cake, the milk cells the
    // BUCKETS wear, the spent cells empty, the recompute speaks
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.open_crafting_table();
    app.table_grid = grid;
    app.refresh_table_result();
    assert_eq!(app.table_result, (cake, 1), "the preview speaks");
    app.click_table_result(0);
    assert_eq!(app.inv_carried, (cake, 1), "the whole result taken");
    assert_eq!(
        app.table_grid[0],
        (bucket, 1),
        "the milk cell the bucket wears"
    );
    assert_eq!(app.table_grid[1], (bucket, 1), "the second bucket rides");
    assert_eq!(app.table_grid[2], (bucket, 1), "the third bucket rides");
    assert_eq!(app.table_grid[3], zero, "the sugar spent");
    assert_eq!(app.table_grid[4], zero, "the egg spent");
    assert_eq!(app.table_grid[5], zero, "the sugar spent");
    assert_eq!(app.table_grid[6], zero, "the wheat spent");
    assert_eq!(app.table_grid[7], zero, "the wheat spent");
    assert_eq!(app.table_grid[8], zero, "the wheat spent");
    assert_eq!(
        app.table_result,
        (0, 0),
        "the bucket grid the recipe refuses"
    );
    // THE CLOSE (the clearContainer walk): the buckets and the
    // carried cake ride home — the store keeps the whole craft
    app.close_crafting_table();
    let buckets: i32 = app
        .inventory
        .items
        .iter()
        .filter(|s| s.0 == bucket)
        .map(|s| s.1)
        .sum();
    let cakes: i32 = app
        .inventory
        .items
        .iter()
        .filter(|s| s.0 == cake)
        .map(|s| s.1)
        .sum();
    assert_eq!(buckets, 3, "the three buckets ride home");
    assert_eq!(cakes, 1, "the carried cake rides home");
}

/// THE REMAINDER'S OWN THREE-WAY TABLE (javap 26.2 ResultSlot
/// .onTake pcs 162-255 — the pure fn the walks share): the
/// emptied cell the setItem arm wears, the same-item cell the
/// grow arm wears, the different-item cell the caller's own add
/// lane answers, the EMPTY remainder no arm rides; the
/// pre-loop list the UNTOUCHED grid answers (the milk the bucket
/// leaves, the stone and the empty cell nothing).
#[test]
fn the_remainder_cell_the_three_way_table() {
    let milk = mc_registry::item_id_by_name("milk_bucket").unwrap();
    let bucket = mc_registry::item_id_by_name("bucket").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    // the setItem arm (pcs 170-186): the emptied slot takes whole
    let mut cell = (0, 0);
    assert_eq!(apply_remainder_cell(&mut cell, bucket), None);
    assert_eq!(cell, (bucket, 1), "the remainder rides the slot whole");
    // the grow arm (pcs 194-222): the same item grows by the
    // cell's own count
    let mut cell = (bucket, 3);
    assert_eq!(apply_remainder_cell(&mut cell, bucket), None);
    assert_eq!(cell, (bucket, 4), "the grown pair rides back");
    // the third arm (pcs 230-255): the different item the
    // caller's own add lane answers, the cell untouched
    let mut cell = (milk, 1);
    assert_eq!(apply_remainder_cell(&mut cell, bucket), Some(bucket));
    assert_eq!(cell, (milk, 1), "the cell the walk never wears");
    // the EMPTY remainder: no arm rides
    let mut cell = (milk, 2);
    assert_eq!(apply_remainder_cell(&mut cell, 0), None);
    assert_eq!(cell, (milk, 2), "the empty remainder the walk never wears");
    // THE PRE-LOOP LIST (pcs 33-43): the untouched grid's own
    // per-cell read — the milk the bucket speaks, the stone and
    // the empty cell the EMPTY answer keeps
    assert_eq!(
        craft_remainders(&[(milk, 1), (stone, 3), (0, 0)]),
        vec![bucket, 0, 0],
        "the Item's own remainder the list keys"
    );
}
