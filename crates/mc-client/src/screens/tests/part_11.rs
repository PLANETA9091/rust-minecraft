use super::*;


    /// THE SNOWBALL'S OWN BREAK (javap `Snowball.handleEntityEvent`
    /// case 3 the 26.2 jar — the `bipush 8` loop the plain case
    /// speaks, the ZERO-velocity shards the jitter walk never wears,
    /// the NO-chick break the onHit override never rolls): the wall
    /// the clip ray banks, the eight SnowballItem shards the burst
    /// wears, the mobs the count the gate never adds.
    #[test]
    fn the_snowball_the_break_the_eight_shards_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let (px, pz) = (16i32, 16i32);
        let gy = 30i32;
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4..=4 {
                for dz in -4..=4 {
                    world.set(px + dx, gy - 1, pz + dz, crate::world::STONE);
                }
            }
            // THE WALL THE SHOT'S OWN TARGET (the south column the
            // flight's ray crosses)
            world.set(px, gy + 1, pz + 3, crate::world::STONE);
        }
        let mobs_before = app.mobs.len();
        app.eggs.push(crate::mob::EggProjectile {
            id: 777,
            x: f64::from(px) + 0.5,
            y: f64::from(gy) + 1.5,
            z: f64::from(pz) + 0.5,
            vx: 0.0,
            vy: 0.0,
            vz: 0.5,
            yaw: 0.0,
            pitch: 0.0,
            dead: false,
            kind: crate::mob::ThrownProjectileKind::Snowball,
        });
        for _ in 0..12 {
            if app.eggs.is_empty() {
                break;
            }
            app.update();
        }
        assert!(app.eggs.is_empty(), "the break the discard serves");
        let snow_shards = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SnowballItem))
            .count();
        let egg_shards = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::EggItem))
            .count();
        assert_eq!(snow_shards, 8, "the bipush-8 loop the case speaks ({snow_shards})");
        assert_eq!(egg_shards, 0, "the egg's own shards the kind never wears");
        assert_eq!(
            app.mobs.len(),
            mobs_before,
            "the NO-chick break the onHit override keeps"
        );
        // THE SHARD'S OWN REST (the ZERO wire velocity the case-3
        // call wears — the internal drift the only speed the shard
        // carries)
        let shard = app
            .particles
            .iter()
            .find(|p| matches!(p.kind, crate::particles::ParticleKind::SnowballItem))
            .expect("the burst the shards keep");
        assert!(
            shard.vx.abs() < 0.05 && shard.vy.abs() < 0.15 && shard.vz.abs() < 0.05,
            "the zero-velocity call the internal drift keeps ({},{},{})",
            shard.vx,
            shard.vy,
            shard.vz
        );
    }
