use super::*;


    /// THE OFFHAND CROSSBOW'S OWN OFFLINE SHOT — the OFF turn's own
    /// loaded shot the mirror walk answers (javap `CrossbowItem.use`
    /// the OFF hand's own turn the for-each-hand walk serves — the
    /// `performShooting` law the hand never reads): the off-charge
    /// the OFF pull loaded, the press the instant shot speaks — the
    /// arrow the same eye/velocity/crit law banks, the voice the
    /// same key rides, the LEFT render the standby restores, and the
    /// MAIN lane the OFF shot never leaks.
    #[test]
    fn the_offhand_crossbow_the_offline_shot_the_mirror_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE OFFHAND CROSSBOW the offline slot wears, the MAIN hand
        // the empty inventory speaks (the MAIN turn's PASS the OFF
        // turn answers)
        app.inventory.items[slot] = (0, 0);
        app.inventory.offhand[0] = (crossbow_id, 1);
        app.offhand_held_refreshed();
        // THE QUIVER'S OWN ARROW the MAIN hand holds (the held scan's
        // MAIN branch — the getHeldProjectile's OFF_HAND test the
        // crossbow itself fails, the MAIN hand the tail answers)
        let arrow_id = wire("arrow");
        app.inventory.items[slot] = (arrow_id, 1);
        // THE OFF PULL: the OFF turn's own pull the LEFT layers
        // re-bake, the full charge the crossing loads mid-hold (the
        // same onUseTick law the MAIN lane wears)
        assert!(app.on_right_press(0, 0), "the OFF pull starts");
        assert!(
            matches!(
                app.using_item.as_ref().map(|u| u.hand),
                Some(UseHand::Off)
            ),
            "the OFF hand the draw wears"
        );
        for _ in 0..25 {
            app.update();
        }
        assert_eq!(
            app.off_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the crossing LOADED the OFF lane mid-hold"
        );
        assert_eq!(
            app.inventory.items[slot].1, 0,
            "the survival draw the MAIN hand's stack costs"
        );
        let midhold = app.held_offhand.as_ref().expect("the LEFT charged mid-hold");
        assert!(
            midhold.binds_texture("item/crossbow_arrow"),
            "the charged bolt the LEFT mid-hold render wears"
        );
        app.on_right_release();
        assert_eq!(
            app.off_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the release the component keeps"
        );
        // THE OFF SHOT: the press the instant walk answers — the
        // arrow the same law banks (the hand the walk never reads)
        assert!(app.on_right_press(0, 0), "the charged OFF press answers");
        assert!(!app.is_using(), "the shot is instant — no pull");
        assert!(app.off_charge.is_none(), "the shot consumed the OFF charge");
        assert_eq!(app.arrows.len(), 1, "the OFF shot the arrow speaks");
        let a = &app.arrows[0];
        assert!(
            a.crit && a.player_shot,
            "the crit/owner law the player's shot keeps"
        );
        assert!(
            (a.y - (eye_y as f64 - 0.1)).abs() < 1e-6,
            "the spawn the eye walk keeps ({})",
            a.y
        );
        assert!(
            a.vz < -3.09 && a.vz > -3.21,
            "the -z the 3.15 flat power speaks ({})",
            a.vz
        );
        assert!(
            app.held_charge.is_none(),
            "the MAIN lane the OFF shot never leaks"
        );
        // THE VOICE: the same key the OFF turn rides
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.crossbow.shoot @")),
                "the CROSSBOW_SHOOT voice the OFF shot rode"
            );
        }
        // THE RESTORE: the LEFT render the plain crossbow wears
        let plain = app.held_offhand.as_ref().expect("the LEFT plain returns");
        assert!(
            plain.binds_texture("item/crossbow_standby"),
            "the standby returns through the LEFT ctx"
        );
    }


    /// THE CROSSBOW'S OWN FIREWORK SHOT — the Rocket kind the
    /// performShooting walk answers (javap `CrossbowItem.
    /// createProjectile` the FIREWORK_ROCKET branch the 26.2 bytecode
    /// speaks): the charged press the instant walk serves — the
    /// rocket the eye-minus-0.15 spawn rides (the FEET x/z the
    /// getX/getZ pair keeps), the 1.6 flat power the #84 ldc names
    /// (the arrow's 3.15 the rocket never wears), the NO crit lane
    /// the ArrowItem branch alone carries, the item.crossbow.shoot
    /// voice the same per-projectile walk rides, the launch the
    /// first tick's own word speaks, the lifetime the ctor's own
    /// 20..=31 band keeps, the lifetime crossing the despawn serves,
    /// and the arrow lane the Rocket kind never leaks.
    #[test]
    fn the_crossbow_the_firework_shot_the_rocket_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        // the spawn band's air (the terrain's own tops never reach),
        // the level -z look the camera's own forward keeps
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE ROCKET COMPONENT the stand-in arms (the Charged-
        // Projectiles the contains walk reads — the kind the shot's
        // own routing answers)
        app.held_charge = Some(mc_registry::ChargeKind::Rocket);
        // THE SHOT: the charged press the instant walk answers — the
        // rocket the createProjectile branch banks
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert!(!app.is_using(), "the shot is instant — no pull");
        assert!(app.held_charge.is_none(), "the shot consumed the charge");
        assert_eq!(
            app.fireworks.len(),
            1,
            "the offline shot the firework speaks"
        );
        assert!(
            app.arrows.is_empty(),
            "the Rocket kind the arrow lane never leaks"
        );
        let f = &app.fireworks[0];
        assert!(
            (f.x - (c as f64 + 0.5)).abs() < 1e-6 && (f.z - (c as f64 + 0.5)).abs() < 1e-6,
            "the spawn the FEET x/z the getX/getZ pair keeps ({},{})",
            f.x,
            f.z
        );
        assert!(
            (f.y - (eye_y as f64 - crate::mob::FIREWORK_EYE_BACKOFF)).abs() < 1e-6,
            "the spawn the eye-minus-0.15 walk keeps ({})",
            f.y
        );
        assert!(
            f.vz < -1.57 && f.vz > -1.63,
            "the -z the 1.6 flat power speaks ({})",
            f.vz
        );
        assert!(
            f.vx.abs() < 0.03 && f.vy.abs() < 0.03,
            "the triangle spread the inaccuracy 1.0 keeps"
        );
        assert!(
            (20..=31).contains(&f.lifetime),
            "the lifetime the ctor's own band keeps ({})",
            f.lifetime
        );
        // THE VOICE: the same per-projectile walk the arrow arm rides
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.crossbow.shoot @")),
                "the CROSSBOW_SHOOT voice the shot rode"
            );
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.launch @")),
                "the launch the first tick never speaks early"
            );
        }
        // THE FIRST TICK: the launch word the life==0 walk speaks,
        // the straight motion the shotAtAngle lane never bends
        app.update();
        assert_eq!(app.fireworks.len(), 1, "the flight the first tick keeps");
        let f = &app.fireworks[0];
        assert_eq!(f.life, 1, "the flight clock the tick advances");
        assert!(
            (f.z - (c as f64 + 0.5 + f.vz)).abs() < 1e-6,
            "the straight motion the shotAtAngle lane keeps ({})",
            f.z
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.launch @")),
                "the FIREWORK_ROCKET_LAUNCH the first tick rode"
            );
        }
        // THE LIFETIME CROSSING: the explode tail the despawn serves
        // (the explosion's own walk the later walk banks)
        let lifetime = app.fireworks[0].lifetime;
        app.fireworks[0].life = lifetime;
        app.update();
        assert!(
            app.fireworks.is_empty(),
            "the lifetime crossing the discard keeps"
        );
        // THE RE-CHARGE: the fresh pull the next rocket arms
        assert!(app.on_right_press(0, 0), "the fresh pull starts");
        assert!(app.is_using(), "the pull the fresh stack arms");
    }


    /// THE GROUND LAUNCH'S OWN CTOR LAW — javap `FireworkRocketEntity`
    /// the 5-arg base ctor the 26.2 bytecode speaks (the useOn's own
    /// spawn pair the [`crate::mob::Firework::ground_launch`] ctor
    /// wears): the motion the `setDeltaMovement(triangle(0.0, 0.002297),
    /// 0.05, triangle(0.0, 0.002297))` walk banks (the x draw FIRST,
    /// the z SECOND — the draws the flight KEEPS, the crossbow's
    /// overwritten pair the ground launch wears), the 0.05 the `ldc2_w
    /// #74` rise keeps, the lifetime the `10 * (1 + flightDuration) +
    /// nextInt(6) + nextInt(7)` walk banks (the plain rocket's
    /// flightDuration 1 the i = 2 keeps — the 20..=31 band the
    /// crossbow's own walk shares), the rotation the zero default
    /// keeps (no `updateRotation` the shoot-less path never rides), the
    /// SHOT_AT_ANGLE flag the accessor's false default keeps (the
    /// crossbow's SEVENTH arg `true` the ground launch never wears),
    /// and the spawn pose the caller's click answer wears verbatim.
    #[test]
    fn the_ground_launch_the_ctor_law_wears() {
        // THE STREAM MIRROR (the same id-mixed seed the same draws
        // spends — the x first, the z second, the lifetime's two
        // nextInts the tail keeps)
        let seed = crate::mob::arrow_entity_seed(7);
        let mut mirror = crate::drops::Lcg::new(seed);
        let mx = mirror.triangle(0.002297);
        let mz = mirror.triangle(0.002297);
        let mlifetime = 10 * (1 + 1) + mirror.next_int(6) + mirror.next_int(7);
        let f = crate::mob::Firework::ground_launch(7, 1.5, 2.5, 3.5);
        assert_eq!(f.vx, mx, "the x triangle the FIRST draw keeps");
        assert_eq!(f.vz, mz, "the z triangle the SECOND draw keeps");
        assert_eq!(f.vy, 0.05, "the 0.05 rise the ldc2_w #74 keeps");
        assert_eq!(f.lifetime, mlifetime, "the lifetime the ctor's walk banks");
        assert!(
            (20..=31).contains(&f.lifetime),
            "the lifetime the ctor's own band keeps ({})",
            f.lifetime
        );
        assert_eq!(f.life, 0, "the flight clock the zero keeps");
        assert!(!f.shot_at_angle, "the plain rocket's billboard the flag keeps");
        assert_eq!(f.y_rot, 0.0, "no updateRotation the shoot-less path keeps");
        assert_eq!(f.x_rot, 0.0, "no updateRotation the shoot-less path keeps");
        assert_eq!(
            (f.x, f.y, f.z),
            (1.5, 2.5, 3.5),
            "the spawn pose the caller's click answers verbatim"
        );
        assert!(!f.stuck && !f.dead, "the fresh launch the rest never wears");
    }


    /// THE LIFETIME BAND'S OWN RE-EXAMINATION — javap the 5-arg base
    /// ctor pcs 111-141 the 26.2 bytecode speaks: `bipush 6` the
    /// nextInt(6) draw the [0, 5] band keeps, `bipush 7` the
    /// nextInt(7) draw the [0, 6] band keeps, `10 * i` the i = 2 base
    /// the walk banks — the 20..=31 ceiling the SUM speaks (the
    /// 20..=32 claim the prior docs wore the nextInt(7)'s own
    /// exclusive bound the re-exam retires; the tick's crossing the
    /// `if_icmple` law the same walk confirms — life > lifetime, the
    /// increment the trail's own head banks, pcs 377-427, the code's
    /// `life > self.lifetime` the same strict verdict wears). THE
    /// EDGE IDS the probe's own corrected walk chose (each triangle
    /// spends TWO f64 draws — the pair 4 raw draws before the
    /// nextInts — the 2-draw slip the first pass wore the corrected
    /// walk retires): id 34 the (5, 6) max edge, id 48 the (0, 0)
    /// min edge.
    #[test]
    fn the_ground_launch_the_lifetime_band_the_bytecode_tightens() {
        // THE MAX EDGE (id 34's own walk: the two triangles the stream
        // spends, the nextInt(6) draw 5, the nextInt(7) draw 6 — the
        // ceiling the 31 lifetime banks)
        let f = crate::mob::Firework::ground_launch(34, 0.0, 0.0, 0.0);
        let mut r = crate::drops::Lcg::new(crate::mob::arrow_entity_seed(34));
        let _mx = r.triangle(0.002297);
        let _mz = r.triangle(0.002297);
        let a = r.next_int(6);
        let b = r.next_int(7);
        assert_eq!((a, b), (5, 6), "the max edge the probe's own id wears");
        assert_eq!(f.lifetime, 20 + a + b, "the ceiling the sum banks");
        assert_eq!(f.lifetime, 31, "the band's own top the walk keeps");
        // THE MIN EDGE (id 48's own walk: the zero draws the floor
        // banks — the 20 lifetime the i=2 base keeps)
        let f = crate::mob::Firework::ground_launch(48, 0.0, 0.0, 0.0);
        let mut r = crate::drops::Lcg::new(crate::mob::arrow_entity_seed(48));
        let _mx = r.triangle(0.002297);
        let _mz = r.triangle(0.002297);
        let a = r.next_int(6);
        let b = r.next_int(7);
        assert_eq!((a, b), (0, 0), "the min edge the probe's own id wears");
        assert_eq!(f.lifetime, 20, "the band's own floor the base keeps");
        // THE BAND'S OWN SWEEP (ids 1..=200 the walk replays — every
        // lifetime the 20..=31 window keeps, the 12 distinct values
        // the [0, 11] sum band answers)
        let mut seen = std::collections::HashSet::new();
        for eid in 1..=200 {
            let f = crate::mob::Firework::ground_launch(eid, 0.0, 0.0, 0.0);
            assert!(
                (20..=31).contains(&f.lifetime),
                "the band the bytecode's own bounds keep ({})",
                f.lifetime
            );
            seen.insert(f.lifetime);
        }
        assert_eq!(
            seen.len(),
            12,
            "the sum band the 12 values speaks: {:?}",
            seen
        );
    }


    /// THE ROCKET'S OWN USE-ON LAUNCH — javap `FireworkRocketItem.useOn`
    /// over the 26.2 jar (the ground launch the block click arms): the
    /// held rocket's RMB the spawn serves — the click + face-step ×
    /// 0.15 pose (the `ldc2_w #59` double the dmul/dadd pairs carry),
    /// the straight-up rise the ctor's own motion keeps (NO `shoot`
    /// the useOn path never calls), the SUCCESS the press consumes,
    /// the launch the life==0 tick's own voice speaks, and the flight
    /// the lifetime crossing the explode tail serves.
    #[test]
    fn the_rocket_use_on_the_ground_launch_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let rocket_id = wire("firework_rocket");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (rocket_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the solid ground keeps (the top face the DOWN
        // look answers — the hit the camera's own column serves)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.fireworks.is_empty(), "the fresh world no rocket keeps");
        // THE LAUNCH: the SUCCESS verdict the press consumes
        assert!(app.on_right_press(0, 0), "the useOn SUCCESS the press consumes");
        assert_eq!(app.fireworks.len(), 1, "the ground launch the rocket banks");
        let f = &app.fireworks[0];
        // THE SPAWN POSE (the click location + the UP face's step × 0.15
        // — the y the top face's own plane + 0.15 keeps, the x/z the
        // click's own ray answers)
        assert!(
            (f.x - (c as f64 + 0.5)).abs() < 1e-6 && (f.z - (c as f64 + 0.5)).abs() < 1e-6,
            "the click's own x/z the spawn keeps ({},{})",
            f.x,
            f.z
        );
        assert!(
            (f.y - ((surface + 1) as f64 + 0.15)).abs() < 1e-6,
            "the UP face step × 0.15 the spawn y keeps ({})",
            f.y
        );
        // THE CTOR'S OWN MOTION (the straight-up rise the jitter
        // perturbs — no shoot the useOn path never calls)
        assert_eq!(f.vy, 0.05, "the 0.05 rise the ctor's own law keeps");
        assert!(
            f.vx.abs() <= 0.002297 && f.vz.abs() <= 0.002297,
            "the triangle jitter the ctor's spread keeps"
        );
        assert!(!f.shot_at_angle, "the plain rocket's billboard the flag keeps");
        // THE LAUNCH VOICE (the life==0 tick's own word — the playSound
        // the useOn itself never speaks)
        if sounds_loaded() {
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.launch @")),
                "the launch the press never speaks early"
            );
        }
        let spawn_y = app.fireworks[0].y;
        app.update();
        assert_eq!(app.fireworks.len(), 1, "the flight the first tick keeps");
        assert_eq!(app.fireworks[0].life, 1, "the flight clock the tick advances");
        // THE RISE (the flight the +y keeps — the wall-free column the
        // straight-up motion never clips)
        assert!(
            app.fireworks[0].y > spawn_y,
            "the rise the flight's own tick keeps"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.launch @")),
                "the FIREWORK_ROCKET_LAUNCH the first tick rode"
            );
        }
        // THE LIFETIME CROSSING: the explode tail the despawn serves
        let lifetime = app.fireworks[0].lifetime;
        app.fireworks[0].life = lifetime;
        app.update();
        assert!(
            app.fireworks.is_empty(),
            "the lifetime crossing the discard keeps"
        );
    }


    /// THE WATER BUCKET'S OWN POUR — javap `BucketItem.use` +
    /// `emptyContents` the 26.2 walk (the filled bucket's Fluid-NONE
    /// POV clip the solid top answers, the target the relPos walk
    /// keeps, the ok gate the `isAir || (canBeReplaced && !shift)` law
    /// wears, the WATER the legacy block answer places, the
    /// `item.bucket.empty` voice the playEmptySound walk speaks, the
    /// EMPTY bucket the `heldItemTransformedTo` slot swap wears).
    #[test]
    fn the_water_bucket_the_pour_the_target_law_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (wire("water_bucket"), 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the solid ground keeps (the top face the DOWN
        // look answers — the target the above cell serves)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE POUR: the SUCCESS verdict the press consumes
        assert!(app.on_right_press(0, 0), "the pour the SUCCESS consumes");
        // THE TARGET (the relPos walk — the UP face's own above cell)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::WATER,
            "the water cell the legacy block answer keeps"
        );
        // THE VOICE (the playEmptySound walk — the water row the
        // item.bucket.empty pick keeps)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.empty @")),
                "the BUCKET_EMPTY voice the pour rode"
            );
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.empty_lava @")),
                "the lava pick the water row never wears"
            );
        }
        // THE ITEM'S OWN TRANSFORM (the EMPTY bucket the slot wears)
        assert_eq!(
            app.inventory.items[slot].0,
            wire("bucket"),
            "the empty bucket the transform speaks"
        );
        assert_eq!(app.inventory.items[slot].1, 1, "the count the swap keeps");
    }


    /// THE EMPTY BUCKET'S OWN PICKUP — javap `BucketItem.use` the
    /// SOURCE_ONLY arc (the fluid cell the POV clip answers, the
    /// `BucketPickup.pickupBlock` draw the AIR answer keeps, the
    /// `item.bucket.fill` voice the getPickupSound walk speaks, the
    /// WATER bucket the `heldItemTransformedTo` slot swap wears).
    #[test]
    fn the_bucket_the_pickup_the_source_cell_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (wire("bucket"), 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the fluid source keeps (the pool cell the DOWN
        // look answers — the stone floor the same column carries)
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::WATER);
            w.set(c, surface - 1, c, crate::world::STONE);
        }
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE PICKUP: the SUCCESS verdict the press consumes
        assert!(app.on_right_press(0, 0), "the pickup the SUCCESS consumes");
        // THE CELL (the AIR answer empties)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::AIR,
            "the source cell the pickup empties"
        );
        // THE VOICE (the getPickupSound walk — the BUCKET_FILL the ear
        // answers)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.fill @")),
                "the BUCKET_FILL voice the pickup rode"
            );
        }
        // THE ITEM'S OWN TRANSFORM (the WATER bucket the slot wears)
        assert_eq!(
            app.inventory.items[slot].0,
            wire("water_bucket"),
            "the water bucket the transform speaks"
        );
    }


    /// THE POUR'S OWN FAIL — javap `emptyContents` the ok gate the
    /// water cell refuses (the `canBeReplaced` walk the liquid state
    /// never passes, the isAir gate the same verdict keeps): the
    /// occupied target the pour refuses, the FAIL verdict the press
    /// never consumes, the voice the walk never speaks, the slot the
    /// transform never wears.
    #[test]
    fn the_water_bucket_the_occupied_target_the_fail_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (wire("water_bucket"), 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the solid ground keeps, the TARGET cell the
        // water pool wears (the Fluid-NONE clip the ray through the
        // pool keeps — the hit the stone's own top answers)
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::STONE);
            w.set(c, surface + 1, c, crate::world::WATER);
        }
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE FAIL: the press the verdict never consumes
        assert!(
            !app.on_right_press(0, 0),
            "the occupied target the FAIL keeps"
        );
        // THE CELLS (the pool the water keeps, the stone the same)
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!(w.get(c, surface, c), crate::world::STONE);
        assert_eq!(w.get(c, surface + 1, c), crate::world::WATER);
        // THE VOICE (the FAIL the playEmptySound never speaks)
        if sounds_loaded() {
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.empty @")),
                "the empty voice the FAIL never speaks"
            );
        }
        // THE SLOT (the transform the FAIL never wears)
        assert_eq!(app.inventory.items[slot].0, wire("water_bucket"));
    }


    /// THE LAVA BUCKET'S OWN POUR — javap `BucketItem.use` +
    /// `emptyContents` the 26.2 walk the lava content rides (the same
    /// Fluid-NONE POV clip the solid top answers, the target the
    /// relPos walk keeps, the ok gate the same law wears, the LAVA the
    /// `createLegacyBlock` answer places, the `item.bucket.empty_lava`
    /// voice the playEmptySound walk's LAVA-tag pick speaks — the
    /// water row the same press never wears, the EMPTY bucket the
    /// `heldItemTransformedTo` slot swap keeps).
    #[test]
    fn the_lava_bucket_the_pour_the_target_law_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (wire("lava_bucket"), 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the solid ground keeps (the top face the DOWN
        // look answers — the target the above cell serves)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE POUR: the SUCCESS verdict the press consumes
        assert!(app.on_right_press(0, 0), "the pour the SUCCESS consumes");
        // THE TARGET (the relPos walk — the LAVA the legacy block
        // answer keeps)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::LAVA,
            "the lava cell the legacy block answer keeps"
        );
        // THE VOICE (the playEmptySound walk — the LAVA row the
        // item.bucket.empty_lava pick keeps)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.empty_lava @")),
                "the BUCKET_EMPTY_LAVA voice the pour rode"
            );
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.empty @")),
                "the water pick the lava row never wears"
            );
        }
        // THE ITEM'S OWN TRANSFORM (the EMPTY bucket the slot wears)
        assert_eq!(
            app.inventory.items[slot].0,
            wire("bucket"),
            "the empty bucket the transform speaks"
        );
        assert_eq!(app.inventory.items[slot].1, 1, "the count the swap keeps");
    }


    /// THE EMPTY BUCKET'S OWN LAVA PICKUP — javap `BucketItem.use` the
    /// SOURCE_ONLY arc the lava source rides (the clip the LAVA kind
    /// the same predicate keeps, the `BucketPickup.pickupBlock` draw
    /// the AIR answer empties, the `item.bucket.fill` voice the
    /// getPickupSound walk speaks, the LAVA bucket the
    /// `heldItemTransformedTo` slot swap wears — the water pick the
    /// lava source never answers).
    #[test]
    fn the_bucket_the_lava_source_the_pickup_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (wire("bucket"), 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the lava source keeps (the pool cell the DOWN
        // look answers — the stone floor the same column carries)
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::LAVA);
            w.set(c, surface - 1, c, crate::world::STONE);
        }
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE PICKUP: the SUCCESS verdict the press consumes
        assert!(app.on_right_press(0, 0), "the pickup the SUCCESS consumes");
        // THE CELL (the AIR answer empties)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::AIR,
            "the source cell the pickup empties"
        );
        // THE VOICE (the getPickupSound walk — the BUCKET_FILL the ear
        // answers)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.bucket.fill @")),
                "the BUCKET_FILL voice the pickup rode"
            );
        }
        // THE ITEM'S OWN TRANSFORM (the LAVA bucket the slot wears)
        assert_eq!(
            app.inventory.items[slot].0,
            wire("lava_bucket"),
            "the lava bucket the transform speaks"
        );
    }


    /// THE HOTBAR SAVE CHORD'S OWN PAGE — javap `Minecraft
    /// .handleKeybinds` pc 258 + `CreativeModeInventoryScreen
    /// .handleHotbarLoadOrSave` pc 111-199 the 26.2 walk (the C-down
    /// chord the creative arm consumes): the `Hotbar.storeFrom` walk
    /// the hotbar row's OWN nine stacks bank, the overlay message the
    /// "inventory.hotbarSaved" law wears (the X + the digit's own name
    /// the positional pair keeps), the row the save never touches.
    #[test]
    fn the_hotbar_save_chord_the_page_banks() {
        let mut app = App::new(3);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // the hotbar row the distinct stacks wear (the storeFrom walk's
        // own getItem(0..SIZE) read)
        let row = [
            (wire("stone"), 7),
            (wire("dirt"), 13),
            (wire("oak_planks"), 29),
            (0, 0),
            (wire("torch"), 41),
            (wire("cobblestone"), 3),
            (0, 0),
            (wire("glass"), 17),
            (wire("ladder"), 23),
        ];
        app.inventory.items[0..9].copy_from_slice(&row);
        // THE SAVE CHORD (C + 4 — the saveDown && !loadDown verdict the
        // arm consumes)
        assert!(
            app.on_hotbar_save_load_key(3, true, false),
            "the creative save the chord consumes"
        );
        // THE PAGE (the row's own snapshot the HotbarManager banks)
        assert_eq!(
            app.hotbar_manager[3], row,
            "the page the storeFrom walk banks"
        );
        // THE ROW (the save the inventory never touches)
        assert_eq!(
            &app.inventory.items[0..9],
            &row,
            "the row the save never touches"
        );
        // THE OVERLAY (the inventory.hotbarSaved law — the X + 4 pair
        // the positional args keep)
        assert_eq!(
            app.overlay_message.as_deref(),
            Some("Item hotbar saved (restore with X + 4)"),
            "the overlay the save law speaks"
        );
    }


    /// THE HOTBAR LOAD CHORD'S OWN ROW — javap `handleHotbarLoadOrSave`
    /// pc 30-108 the 26.2 walk (the X-down chord the creative arm
    /// consumes, the BOTH-modifier press the load param's own first
    /// boolean wears — pc 30's `iload_2` the load branch consumes
    /// first, the save branch the `goto` return never reads): the
    /// `Inventory.setItem(j, stack)` row the j in 0..9 walk writes,
    /// the HUD row the same-frame sync speaks, the empty page the
    /// `(0, 0)` sentinels keep, the overlay the load never speaks.
    #[test]
    fn the_hotbar_load_chord_the_row_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let slot = app.selected_hotbar;
        // the page the "previously saved" stacks wear (the
        // HotbarManager.get(i) walk's own answer)
        let page = [
            (wire("sand"), 31),
            (0, 0),
            (wire("bricks"), 11),
            (wire("bookshelf"), 2),
            (0, 0),
            (wire("water_bucket"), 1),
            (wire("stick"), 19),
            (0, 0),
            (wire("red_wool"), 5),
        ];
        app.hotbar_manager[5] = page;
        // the row the garbage wears (the load's own overwrite answers)
        app.inventory.items[0] = (wire("diamond_sword"), 1);
        app.inventory.items[8] = (wire("apple"), 3);
        // THE LOAD CHORD (X + 6 — and the BOTH-modifier press the same
        // load verdict wears: the load param the first boolean keeps)
        assert!(
            app.on_hotbar_save_load_key(5, true, true),
            "the creative load the chord consumes"
        );
        // THE ROW (the setItem walk the page restores — the garbage the
        // snapshot's own truth replaces)
        assert_eq!(
            &app.inventory.items[0..9],
            &page,
            "the row the load walk restores"
        );
        // THE HUD ROW (the same-frame sync the broadcastChanges law
        // keeps)
        assert_eq!(app.hotbar_slots, page, "the HUD row the sync speaks");
        // THE HELD MODEL (the selected slot's own stack the reload
        // follows)
        assert_eq!(
            app.inventory.items[slot].0,
            page[slot].0,
            "the selected stack the row's own id keeps"
        );
        // THE OVERLAY (the load the message never speaks)
        assert_eq!(
            app.overlay_message, None,
            "the overlay the load never wears"
        );
    }


    /// THE SURVIVAL CHORD'S OWN REFUSAL — javap `Minecraft
    /// .handleKeybinds` pcs 226-233 the 26.2 walk (the
    /// `hasInfiniteMaterials` FALSE the save/load arm skips, the
    /// `setSelectedSlot` else arm pc 264-272 the plain select wears):
    /// the survival chord the arm never consumes, the page the
    /// snapshot never banks, the plain select the number key keeps.
    #[test]
    fn the_survival_chord_the_plain_select_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        // the survival default the constructor keeps (the creative
        // gate's own FALSE verdict)
        assert_eq!(
            app.game_mode,
            GameMode::Survival,
            "the survival default the fresh app keeps"
        );
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        app.inventory.items[0] = (wire("stone"), 9);
        // THE REFUSAL (C + 3 the survival verdict never consumes)
        assert!(
            !app.on_hotbar_save_load_key(2, true, false),
            "the survival chord the arm refuses"
        );
        // THE PAGE (the snapshot never banks — the empty page the
        // fresh install keeps)
        assert_eq!(
            app.hotbar_manager[2],
            [(0, 0); 9],
            "the page the refusal never banks"
        );
        // THE ROW (the save the inventory never wears)
        assert_eq!(
            app.inventory.items[0],
            (wire("stone"), 9),
            "the row the chord never touches"
        );
    }


    /// THE HOTBAR FILE'S OWN PAGES — javap `HotbarManager.save()` pcs
    /// 4-54 + `NbtIo.write` the 26.2 walk (the root compound the
    /// DataVersion row keeps, the `"0".."8"` list rows the get loop
    /// speaks, the `ItemStack.OPTIONAL_CODEC` item compound each entry
    /// wears): the round-trip the read_file_nbt mirror answers — the
    /// empty stack the `{}` compound keeps, the id the `minecraft:`
    /// prefix the registry key wears, the count the Int the
    /// always-written field keeps, the DataVersion the 4903 the jar's
    /// own world_version answers.
    #[test]
    fn the_hotbar_file_the_pages_wear() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut pages = [[(0, 0); 9]; 9];
        pages[2] = [
            (wire("stone"), 7),
            (0, 0),
            (wire("dirt"), 1),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (wire("torch"), 41),
        ];
        // THE ROOT (the save walk's own shape)
        let root = hotbar_pages_to_nbt(&pages);
        let mc_protocol::nbt::Nbt::Compound(entries) = &root else {
            panic!("the root the compound wears");
        };
        assert_eq!(
            entries[0],
            ("DataVersion".to_string(), mc_protocol::nbt::Nbt::Int(HOTBAR_DATA_VERSION)),
            "the DataVersion row the root keeps"
        );
        let mc_protocol::nbt::Nbt::List(items) = &entries[3].1 else {
            panic!("the page 2 the list wears");
        };
        // THE EMPTY STACK (the OPTIONAL empty map the `{}` wears)
        assert_eq!(
            items[1],
            mc_protocol::nbt::Nbt::Compound(vec![]),
            "the empty stack the empty compound keeps"
        );
        // THE NON-EMPTY (the id the minecraft: prefix wears, the count
        // the Int the always-written field keeps)
        assert_eq!(
            items[0],
            mc_protocol::nbt::Nbt::Compound(vec![
                ("id".to_string(), mc_protocol::nbt::Nbt::String("minecraft:stone".to_string())),
                ("count".to_string(), mc_protocol::nbt::Nbt::Int(7)),
            ]),
            "the stone stack the id+count pair wears"
        );
        // THE COUNT-1 (the field the same always-written walk keeps)
        assert_eq!(
            items[2],
            mc_protocol::nbt::Nbt::Compound(vec![
                ("id".to_string(), mc_protocol::nbt::Nbt::String("minecraft:dirt".to_string())),
                ("count".to_string(), mc_protocol::nbt::Nbt::Int(1)),
            ]),
            "the count-1 stack the Int the pair keeps"
        );
        // THE ROUND-TRIP (the NbtIo.write/read mirror the file format
        // keeps — the parse the same pages answer)
        let mut bytes = Vec::new();
        mc_protocol::nbt::write_file_nbt("", &root, &mut bytes).expect("the write serves");
        let (_, back) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the read serves");
        assert_eq!(
            hotbar_pages_from_nbt(&back),
            pages,
            "the pages the round-trip wears"
        );
    }


    /// THE FILE CHORD'S OWN RESTART — javap `HotbarManager` the 26.2
    /// walk (the save chord's `HotbarManager.save()` pc 199 the file
    /// writes, the fresh app's `get(i)` once-flag pc 1-13 the lazy
    /// `load()` speaks): the banked page the file wears, the NEW app
    /// the same options file the row restores — the restart walk the
    /// vanilla client's own pages keep.
    #[test]
    fn the_file_chord_the_restart_wears() {
        let dir = std::env::temp_dir().join(format!("mc2rs-hotbar-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("hotbar-save-round.nbt");
        let _ = std::fs::remove_file(&file);
        let mut app = App::new(3);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        app.hotbar_options_file = Some(file.clone());
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let row = [
            (wire("stone"), 7),
            (wire("dirt"), 13),
            (0, 0),
            (wire("oak_planks"), 29),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (wire("glass"), 17),
        ];
        app.inventory.items[0..9].copy_from_slice(&row);
        // THE SAVE CHORD (C + 4 — the page banks AND the file writes)
        assert!(app.on_hotbar_save_load_key(3, true, false), "the save consumes");
        assert!(file.exists(), "the file the save walk writes");
        // THE RESTART (the fresh app the same options file carries —
        // the first load chord the once-flag's lazy read speaks)
        let mut fresh = App::new(5);
        fresh.enter_world();
        fresh.game_mode = GameMode::Creative;
        fresh.hotbar_options_file = Some(file.clone());
        assert!(
            fresh.on_hotbar_save_load_key(3, false, true),
            "the load chord the fresh app consumes"
        );
        assert_eq!(
            fresh.hotbar_manager[3], row,
            "the banked page the restart wears"
        );
        // THE ONCE-FLAG (the load the file never re-reads — the second
        // chord the in-memory pages keep)
        fresh.hotbar_manager[3] = [(9, 9); 9];
        fresh.on_hotbar_save_load_key(3, false, true);
        assert_eq!(
            fresh.hotbar_manager[3], [(9, 9); 9],
            "the in-memory page the once-flag keeps"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE FILE'S OWN REFUSALS — javap `HotbarManager.load()` the
    /// resultOrPartial walk (the parse failure the fresh empties
    /// keeps, the log the error answers): the non-compound root the
    /// empties keep, the unknown item name the honest empty stack
    /// wears (never a guessed id), the missing count the fallback-1
    /// field keeps, the corrupt file the lazy load never wears.
    #[test]
    fn the_file_parse_the_refusals_keep() {
        // THE NON-COMPOUND ROOT (the list the pages never parse)
        let pages = hotbar_pages_from_nbt(&mc_protocol::nbt::Nbt::Int(1));
        assert_eq!(pages, [[(0, 0); 9]; 9], "the non-compound the empties keep");
        // THE UNKNOWN NAME + THE MISSING COUNT (the tolerant walk the
        // honest refusals keep)
        let root = mc_protocol::nbt::Nbt::Compound(vec![(
            "0".to_string(),
            mc_protocol::nbt::Nbt::List(vec![
                mc_protocol::nbt::Nbt::Compound(vec![(
                    "id".to_string(),
                    mc_protocol::nbt::Nbt::String("minecraft:not_an_item".to_string()),
                )]),
                mc_protocol::nbt::Nbt::Compound(vec![(
                    "id".to_string(),
                    mc_protocol::nbt::Nbt::String("minecraft:stone".to_string()),
                )]),
            ]),
        )]);
        let pages = hotbar_pages_from_nbt(&root);
        assert_eq!(pages[0][0], (0, 0), "the unknown name the empty stack keeps");
        let stone = mc_registry::item_id_by_name("stone").expect("the registry names");
        assert_eq!(
            pages[0][1],
            (stone, 1),
            "the missing count the fallback-1 field keeps"
        );
        // THE CORRUPT FILE (the lazy load the fresh empties keep — the
        // read failure the silent keep answers)
        let dir = std::env::temp_dir().join(format!("mc2rs-hotbar-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("hotbar-corrupt.nbt");
        std::fs::write(&file, [0x0A, 0x00, 0x01, 0xFF]).expect("the garbage writes");
        let mut app = App::new(3);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        app.hotbar_options_file = Some(file.clone());
        assert!(
            app.on_hotbar_save_load_key(6, false, true),
            "the load chord the corrupt file still consumes"
        );
        assert_eq!(
            app.hotbar_manager[6],
            [(0, 0); 9],
            "the corrupt file the fresh empties keep"
        );
        assert!(app.hotbar_file_loaded, "the once-flag the failed read banks");
        let _ = std::fs::remove_file(&file);
    }


    /// THE CREATIVE CHORDS' OWN LIVE STORE — javap
    /// `handleHotbarLoadOrSave` the live-session walk (the
    /// client-authoritative creative client the vanilla keeps — the
    /// `setItem` the local store wears, the
    /// `handleCreativeModeItemAdd` the ServerboundSetCreativeModeSlot
    /// packet per stack speaks): the LOAD the belief's in-place
    /// mutation keeps (the store's 36..44 rows the page wears, the
    /// HUD row the same-frame sync speaks) + the nine CreativeSetSlot
    /// words the wire rides in order; the SAVE the store's OWN row
    /// the page banks — the client-local walk the vanilla `save()`
    /// keeps (no wire the save spends).
    #[test]
    fn the_creative_chords_the_live_store_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        app.world_is_network = true;
        let (tx, rx) = std::sync::mpsc::channel();
        app.live_clicks = Some(tx);
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let page = [
            (wire("sand"), 4),
            (0, 0),
            (wire("bricks"), 8),
            (0, 0),
            (wire("glass"), 2),
            (0, 0),
            (wire("torch"), 6),
            (0, 0),
            (wire("red_wool"), 1),
        ];
        app.hotbar_manager[1] = page;
        // the belief row the garbage wears (the load's own overwrite
        // answers)
        app.live_inv_slots[36] = (wire("diamond_sword"), 1);
        app.live_inv_slots[44] = (wire("apple"), 3);
        // THE LOAD CHORD (X + 2 — the live walk the belief keeps)
        assert!(
            app.on_hotbar_save_load_key(1, false, true),
            "the creative load the live chord consumes"
        );
        // THE NINE WIRE WORDS (the CreativeSetSlot commands the wire
        // rides IN ORDER — the container ids 36..44 the walk keys)
        for (j, expected) in page.iter().enumerate() {
            match rx
                .try_recv()
                .expect("the load's own wire word")
            {
                crate::world_view::ContainerCommand::CreativeSetSlot { slot, item } => {
                    assert_eq!(slot, 36 + j as i32, "the container id the walk keys");
                    assert_eq!(item, *expected, "the stack the wire word rides");
                }
                other => panic!("the unexpected command the wire rode: {other:?}"),
            }
        }
        // THE BELIEF (the in-place mutation the store's own rows keep)
        assert_eq!(
            &app.live_inv_slots[36..45],
            &page,
            "the store rows the load walk wears"
        );
        // THE HUD ROW (the same-frame sync the belief speaks)
        assert_eq!(app.hotbar_slots, page, "the HUD row the sync speaks");
        // THE SAVE CHORD (C + 2 — the store's OWN row the page banks)
        assert!(
            app.on_hotbar_save_load_key(1, true, false),
            "the creative save the live chord consumes"
        );
        assert_eq!(
            app.hotbar_manager[1], page,
            "the page the store's own row banks"
        );
        // NO WIRE THE SAVE SPENDS (the client-local walk the vanilla
        // keeps — the channel the save never rides)
        assert!(
            rx.try_recv().is_err(),
            "the save the wire never spends"
        );
    }


    /// THE FIREWORK USE'S OWN GATE + SUCCESS — javap `FireworkRocketItem.
    /// use` the 26.2 walk (pcs 0-93): the isFallFlying gate the PASS
    /// verdict keeps (the NOT-gliding press the false the OFF hand's
    /// turn serves — no rocket the client banks, no count the stack
    /// loses, no echo the refused lane speaks), the gliding press the
    /// SUCCESS wears (the attached rocket the (Level, ItemStack,
    /// LivingEntity) ctor spawns — the attachment the flag arms, the
    /// billboard the SHOT_AT_ANGLE keeps, the lifetime the banked
    /// 20..=31 band keeps —, the `consume(1, player)` the survival
    /// stack wears, the use-item echo the click rides).
    #[test]
    fn the_firework_use_the_glide_asks() {
        let rocket = mc_registry::item_id_by_name("firework_rocket")
            .expect("the rocket rides the registry");
        // THE NOT-GLIDING PRESS (the grounded body the PASS keeps)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        // THE LEVEL'S OWN EYE (the pitch the clip's ray reads — the
        // spawn's slight down-pitch the ground lane would feed, the
        // level ray the useOn arm never arms)
        {
            let b = app.body.as_mut().expect("the offline body spawns");
            b.yaw = 0.0;
            b.pitch = 0.0;
        }
        // THE SKY'S OWN AIM (the clip the block lane never feeds — the
        // useOn ground-launch arm the BLOCK click answers, the AIR use
        // the item lane's own gate serves)
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, (app.spawn_y + 7) as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        app.inventory.items[app.selected_hotbar] = (rocket, 3);
        assert!(
            !app.on_right_press(0, 0),
            "the not-gliding press the PASS answers"
        );
        assert!(
            app.fireworks.is_empty(),
            "the PASS no rocket banks"
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar],
            (rocket, 3),
            "the stack the PASS never touches"
        );
        // THE GLIDING PRESS (the elytra chest + the airborne jump
        // chord the glide starts — the SUCCESS the rocket banks)
        let elytra = mc_registry::item_id_by_name("elytra")
            .expect("the elytra rides the registry");
        app.inventory.armor[1] = (elytra, 1);
        {
            let b = app.body.as_mut().expect("the offline body spawns");
            b.y = 40.0;
            b.on_ground = false;
            b.yaw = 0.0;
            b.pitch = 0.0;
        }
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the glide chord the start speaks"
        );
        let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
        app.attach_live_clicks(tx);
        assert!(
            app.on_right_press(0, 0),
            "the gliding press the SUCCESS answers"
        );
        assert_eq!(app.fireworks.len(), 1, "the attached rocket banks");
        let f = &app.fireworks[0];
        assert!(f.attached, "the attachment the use spawns");
        assert!(!f.shot_at_angle, "the billboard the attached ctor keeps");
        assert!(
            (20..=31).contains(&f.lifetime),
            "the lifetime the banked band keeps ({})",
            f.lifetime
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar],
            (rocket, 2),
            "the consume the survival stack wears"
        );
        // THE USE-ITEM ECHO (the click the networked lane rides — the
        // vanilla single-player client's own use-item packet)
        let mut saw_echo = false;
        while let Ok(cmd) = rx.try_recv() {
            if let crate::world_view::ContainerCommand::UseItem { .. } = cmd {
                saw_echo = true;
                break;
            }
        }
        assert!(saw_echo, "the use-item echo the press rode");
    }


    /// THE BOOST CHAIN THE GLIDE WEARS — the end-to-end walk the
    /// frame's own tick order speaks (the body's travel FIRST, the
    /// rocket's attached tick SECOND — the vanilla player-then-rocket
    /// pairing the update walk keeps): the fold the body's velocity
    /// wears (the PAIRED WALK the deterministic sim speaks — the
    /// boost-less twin the SAME body walk runs, the difference the
    /// exact `v += look*0.1 + (look*1.5 − v)*0.5` fold answers), the
    /// hand follow the setPos law pins (the view(0, +80°)*0.5 offset
    /// the feet origin keeps), the delta sync the rocket's own motion
    /// wears, the LANDING the boost retires (the ZERO arm — the
    /// follow the same law keeps), the lifetime crossing the boost's
    /// own end serves.
    #[test]
    fn the_boost_chain_the_glide_wears() {
        let rocket = mc_registry::item_id_by_name("firework_rocket")
            .expect("the rocket rides the registry");
        let elytra = mc_registry::item_id_by_name("elytra")
            .expect("the elytra rides the registry");
        // THE SCENARIO'S OWN BUILDER (the twin apps the paired walk
        // arms — the rocket stack the only variable)
        let glide_app = |rocket_count: i32| -> App {
            let mut app = App::new(3);
            app.enter_world();
            app.spawn_offline_body();
            // THE SKY'S OWN AIM (the clip the block lane never feeds —
            // the AIR use the item lane's own gate serves)
            let size = app.offline_world.as_ref().unwrap().size_blocks();
            let c = size / 2;
            app.offline_camera_updated(crate::camera::Camera {
                pos: crate::math::Vec3::new(
                    c as f32 + 0.5,
                    (app.spawn_y + 7) as f32,
                    c as f32 + 0.5,
                ),
                yaw: 0.0,
                pitch: 0.0,
            });
            if rocket_count > 0 {
                app.inventory.items[app.selected_hotbar] = (rocket, rocket_count);
            }
            app.inventory.armor[1] = (elytra, 1);
            {
                let b = app.body.as_mut().expect("the offline body spawns");
                b.y = 40.0;
                b.on_ground = false;
                b.yaw = 0.0;
                b.pitch = 0.0;
            }
            app.on_movement_input(0.0, 0.0, true, false, false);
            app.update();
            assert!(app.body.as_ref().expect("the body rides").fall_flying);
            app
        };
        // THE TWIN WALKS (the boost run the use speaks, the plain run
        // the empty hand keeps — the body's own glide walk the SAME
        // walk both frames run, the rocket's fold the only delta)
        let mut boosted = glide_app(1);
        let mut plain = glide_app(0);
        assert!(boosted.on_right_press(0, 0), "the use the SUCCESS speaks");
        // THE LOOK THE SNAPSHOT WORES (the body's pose the press
        // never moves — the same look the rocket's fold reads)
        let look = boosted
            .body
            .as_ref()
            .expect("the body rides")
            .camera()
            .forward();
        let (lx, ly, lz) = (look.x as f64, look.y as f64, look.z as f64);
        // THE TWIN'S OWN FRAME (the SAME frame count the plain run
        // walks — the body's own glide walk the identical walk both
        // bodies ran, the rocket's fold the only delta the boost
        // frame carries)
        plain.update();
        boosted.update();
        {
            let b = boosted.body.as_ref().expect("the body rides");
            let p = plain.body.as_ref().expect("the plain body rides");
            let f = &boosted.fireworks[0];
            // THE EXACT FOLD (the rider's v the twin's post-tick v
            // answers — the same deterministic walk both bodies ran,
            // the delta the fold's own walk wears)
            let rvz = p.vz;
            assert_eq!(
                b.vz,
                rvz + lz * 0.1 + (lz * 1.5 - rvz) * 0.5,
                "the exact fold the body's velocity wears ({})",
                b.vz
            );
            let rvx = p.vx;
            assert_eq!(
                b.vx,
                rvx + lx * 0.1 + (lx * 1.5 - rvx) * 0.5,
                "the exact x fold the body's velocity wears ({})",
                b.vx
            );
            let rvy = p.vy;
            assert_eq!(
                b.vy,
                rvy + ly * 0.1 + (ly * 1.5 - rvy) * 0.5,
                "the exact y fold the body's velocity wears ({})",
                b.vy
            );
            assert_eq!(f.vz, b.vz, "the delta sync the post-boost v wears");
            // THE HAND FOLLOW (the feet + the +80° offset the tick's
            // own setPos walk pins — the body's yaw the fold reads)
            let (hx, _, hz) = crate::mob::Firework::hand_offset(b.yaw);
            assert_eq!(f.y, b.y, "the y the pitch-0 view keeps");
            assert!((f.x - (b.x + hx)).abs() < 1e-9, "the x the hand pins");
            assert!((f.z - (b.z + hz)).abs() < 1e-9, "the z the hand pins");
        }
        // THE LANDING (the ground the glide retires — the ZERO arm
        // the boost never speaks, the follow the same law keeps)
        {
            let b = boosted.body.as_mut().expect("the body rides");
            b.on_ground = true;
            b.fall_flying = false;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
        }
        boosted.update();
        {
            let b = boosted.body.as_ref().expect("the body rides");
            let f = &boosted.fireworks[0];
            assert!(!b.fall_flying, "the landing the glide retires");
            assert_eq!(
                f.vz, b.vz,
                "the zero arm's sync the current v wears"
            );
            let (hx, _, _) = crate::mob::Firework::hand_offset(b.yaw);
            assert!(
                (f.x - (b.x + hx)).abs() < 1e-9,
                "the follow the landing still pins"
            );
        }
        // THE LIFETIME CROSSING (the boost's own end the discard
        // serves — the chain the expiry retires)
        let lifetime = boosted.fireworks[0].lifetime;
        boosted.fireworks[0].life = lifetime;
        boosted.update();
        assert!(
            boosted.fireworks.is_empty(),
            "the crossing the attached rocket discards"
        );
    }


    /// THE ELYTRA VOICE THE GLIDE WEARS — the end-to-end walk the
    /// frame's own order speaks (javap LocalPlayer's shared-flag sync
    /// arm the play ride, `ElytraOnPlayerSoundInstance.tick` the
    /// per-tick law): the rise the edge speaks (the play line the
    /// ctor's constants carry, the SAME frame's manager tick the
    /// vanilla's play-then-tick walk keeps), the hold the clock
    /// advances (the mute window the early voice wears), the landing
    /// the deferred stop serves (the 20-tick minimum the loop plays
    /// through, the stopped line banks), the re-glide the fresh
    /// instance arms (the second play line the new edge rides).
    #[test]
    fn the_elytra_voice_the_glide_wears() {
        let elytra = mc_registry::item_id_by_name("elytra")
            .expect("the elytra rides the registry");
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.inventory.armor[1] = (elytra, 1);
        {
            let b = app.body.as_mut().expect("the offline body spawns");
            b.y = 40.0;
            b.on_ground = false;
            b.yaw = 0.0;
            b.pitch = 0.0;
        }
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(app.body.as_ref().expect("the body rides").fall_flying);
        // THE RISE (the play line the edge spoke, the instance the
        // same frame's tick advanced — the time 1 the iadd keeps)
        let v = app
            .sounds
            .elytra_voice
            .expect("the voice the rise banks");
        assert_eq!(v.time, 1, "the same frame's tick the walk advances");
        let play_lines = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("item.elytra.flying @") && e.contains("#"))
            .count();
        assert_eq!(play_lines, 1, "the play line the rise spoke");
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.elytra.flying @")
                    && e.ends_with(" v0.100000 p1.000000")),
            "the play line the ctor's constants carry"
        );
        // THE HOLD (the glide the loop walks — the mute window the
        // early ticks keep, the clock the frames advance)
        for _ in 0..5 {
            app.update();
        }
        let v = app.sounds.elytra_voice.expect("the hold the voice keeps");
        assert_eq!(v.time, 6, "the hold the clock advances");
        assert_eq!(v.volume, 0.0, "the mute window the early voice keeps");
        // THE STOP (the landing the glide retires — the deferred
        // window the loop plays through, the crossing the discard
        // serves, the stopped line the record lane's shape wears)
        app.sounds.elytra_voice.as_mut().expect("the voice rides").time = 25;
        {
            let b = app.body.as_mut().expect("the body rides");
            b.on_ground = true;
        }
        app.update();
        assert!(
            !app.body
                .as_ref()
                .expect("the body rides")
                .fall_flying,
            "the landing the glide retires"
        );
        assert!(
            app.sounds.elytra_voice.is_none(),
            "the crossing the discard serves"
        );
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.elytra.flying @") && e.ends_with(" stopped")),
            "the stopped line the crossing banks"
        );
        // THE RE-GLIDE (the fresh edge the new instance arms — the
        // second play line the same law speaks)
        {
            let b = app.body.as_mut().expect("the body rides");
            b.y = 40.0;
            b.on_ground = false;
        }
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(app.body.as_ref().expect("the body rides").fall_flying);
        let v = app
            .sounds
            .elytra_voice
            .expect("the re-glide the fresh voice banks");
        assert_eq!(v.time, 1, "the fresh instance the same walk wears");
        let play_lines = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("item.elytra.flying @") && e.contains("#"))
            .count();
        assert_eq!(play_lines, 2, "the second edge the new play line spoke");
    }


    /// THE FIREWORK'S OWN CROSSING BURST — the lifetime crossing the
    /// `createFireworks` POOF walk answers (javap `handleEntityEvent`
    /// 17 → `ClientLevel.createFireworks(x, y, z, dm, EMPTY)` the 26.2
    /// walk over the jar): the RE-ROLLED `2 + nextInt(3)` bound the
    /// 2..=4 band keeps, the Poof kind the ExplodeParticle provider
    /// speaks, the burst the ONE anchor shares, and the live byte-17
    /// lane the same walk's wire landing keeps.
    #[test]
    fn the_firework_the_crossing_the_poof_burst_banks() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        app.held_charge = Some(mc_registry::ChargeKind::Rocket);
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert_eq!(app.fireworks.len(), 1, "the shot the rocket banks");
        // THE FIRST TICK: the launch the flight rides (the crossing
        // the early ticks never arm)
        app.update();
        assert_eq!(app.fireworks.len(), 1, "the flight the first tick keeps");
        // THE LIFETIME CROSSING: the explode word the burst answers —
        // the CROSSING TICK'S OWN TRAIL the spark the tick's client
        // walk banks FIRST (the pcs 430-507 walk the trail the tick
        // speaks BEFORE the crossing's own branch answers), the poofs
        // the ONE anchor share, the kind the provider speaks
        let before = app.particles.len();
        app.fireworks[0].life = app.fireworks[0].lifetime;
        app.update();
        assert!(app.fireworks.is_empty(), "the crossing the discard keeps");
        // THE TRAIL SPARK (the tick's own walk — the launch tick AND
        // the flight ticks the earlier updates banked, the crossing's
        // the last one speaks)
        let trail = &app.particles[before];
        assert!(matches!(
            trail.kind,
            crate::particles::ParticleKind::Spark { .. }
        ), "the crossing tick the trail spark banks before the burst");
        let burst: Vec<&crate::particles::Particle> =
            app.particles.iter().skip(before + 1).collect();
        assert!(
            (2..=4).contains(&burst.len()),
            "the re-rolled bound the 2..=4 band keeps ({})",
            burst.len()
        );
        for p in &burst {
            assert_eq!(
                p.kind,
                crate::particles::ParticleKind::Poof,
                "the ExplodeParticle provider the plain rocket wears"
            );
        }
        let (ax, ay, az) = (burst[0].x, burst[0].y, burst[0].z);
        for p in &burst {
            assert_eq!((p.x, p.y, p.z), (ax, ay, az), "the burst the ONE anchor shares");
        }
        // THE LIVE LANE: the byte-17 word the wire ships → the burst
        // the same empty-explosions law speaks at the arm's own anchor
        let before_live = app.particles.len();
        app.live_firework_explode_events_updated(vec![crate::world_view::WireFireworkExplodeEvent {
            entity_id: 40,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
            explosions: Vec::new(),
        }]);
        let live: Vec<&crate::particles::Particle> =
            app.particles.iter().skip(before_live).collect();
        assert!(
            (2..=4).contains(&live.len()),
            "the wire's 17 the burst banks ({})",
            live.len()
        );
        for p in &live {
            assert_eq!((p.x, p.y, p.z), (1.0, 2.0, 3.0), "the arm's anchor the burst keeps");
        }
    }


    /// THE FIREWORK'S OWN STARTER — the wire's NON-empty branch the
    /// Starter machine answers (javap `createFireworks` pcs 79-113 +
    /// `FireworkParticles$Starter` the 26.2 walk /tmp/fw_starter_538.txt
    /// pins): the non-empty event the Starter banks (the lifetime the
    /// `size × 2 − 1` ctor law keeps), the FIRST tick the life-0
    /// BLAST voice + the SMALL_BALL shell (the 98 sparks the
    /// 5×5×5-cube shell walk answers: 16 face rows × 5 + 9 inner rows
    /// × 2) + the FLASH the colors[0] arms, the SECOND tick the odd
    /// cadence never arms, the THIRD tick the lifetime crossing the
    /// discard serves (the twinkle the twinkleDelay bit never set
    /// keeps its silence) — the sparks the color decode + the fade
    /// laws the fields keep.
    #[test]
    fn the_firework_the_starter_the_cadence_wears() {
        let mut app = App::new(3);
        app.enter_world();
        // THE NON-EMPTY EVENT: the SMALL_BALL explosion the component
        // list carries, the anchor + the motion the ctor keeps
        let ball = mc_protocol::FireworkExplosion {
            shape: 0,
            colors: vec![0xFF0000],
            fade_colors: vec![],
            has_trail: false,
            has_twinkle: false,
        };
        app.live_firework_explode_events_updated(vec![crate::world_view::WireFireworkExplodeEvent {
            entity_id: 41,
            x: 10.0,
            y: 70.0,
            z: -5.0,
            mx: 0.2,
            my: 0.0,
            mz: -0.1,
            explosions: vec![ball.clone()],
        }]);
        assert_eq!(app.firework_starters.len(), 1, "the non-empty the Starter banks");
        let s = &app.firework_starters[0];
        assert_eq!(s.lifetime, 1, "the size × 2 − 1 the ctor law keeps");
        assert!(!s.twinkle_delay, "the twinkle-less list the bit never arms");
        assert_eq!((s.vx, s.vy, s.vz), (0.2, 0.0, -0.1), "the motion args the burst reads");
        // THE FIRST TICK: the life-0 BLAST + the shell walk + the flash
        let before = app.particles.len();
        app.update();
        assert_eq!(app.firework_starters[0].life, 1, "the cadence the first tick banks");
        // THE SHELL (the SMALL_BALL walk's own count — the 16 face
        // rows × 5 + the 9 inner rows × 2 the shell skip answers) +
        // the FLASH the explosion's own tail keeps
        let spawned = &app.particles[before..];
        assert_eq!(
            spawned.len(),
            98 + 1,
            "the ball shell the 98 sparks + the flash the walk keeps ({})",
            spawned.len()
        );
        // THE SPARKS' OWN COLOR (the single-color pick the decode
        // answers — the 0xFF0000 red the [1.0, 0.0, 0.0] f32 fold
        // gives), the fade the empty list never arms
        let sparks = &spawned[..98];
        for sp in sparks {
            match &sp.kind {
                crate::particles::ParticleKind::Spark { fade, .. } => {
                    assert_eq!(*fade, None, "the empty fade list the store never arms");
                }
                other => panic!("the shell the sparks alone wear, got {:?}", other),
            }
            assert_eq!(sp.col, [1.0, 0.0, 0.0], "the red the decode gives");
            assert_eq!(sp.gravity, 0.1, "the spark's own gravity the ctor keeps");
            assert_eq!(sp.friction, 0.91, "the spark's own friction the ctor keeps");
        }
        // THE FLASH (the last spawn the cadence's tail keeps — the
        // four-tick life + the zero motion + the colors[0] the
        // option's color decodes, the alpha the dye color's 0x00 byte
        // answers — the vanilla paint-nothing law the field banks)
        let flash = &spawned[98];
        assert!(
            matches!(flash.kind, crate::particles::ParticleKind::Flash { .. }),
            "the flash the tail banks"
        );
        assert_eq!(flash.lifetime, 4, "the iconst_4 the ctor law keeps");
        assert_eq!(flash.col, [1.0, 0.0, 0.0], "the colors[0] the flash decodes");
        assert_eq!(
            flash.kind,
            crate::particles::ParticleKind::Flash { alpha: 0.0 },
            "the dye color's 0x00 byte the alpha decode answers"
        );
        assert_eq!((flash.vx, flash.vy, flash.vz), (0.0, 0.0, 0.0), "the dconst_0 motion keeps");
        // THE BLAST VOICE (the life-0 word the AMBIENT lane speaks —
        // the large gate the single small ball never arms; the far
        // word the camera's own distance answers, the near/far pair
        // the two words cover)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.blast @")
                        || e.starts_with("entity.firework_rocket.blast_far @")),
                "the life-0 blast the first tick rode"
            );
        }
        // THE SECOND TICK: the odd cadence never arms (no spawns, no
        // voices)
        let before_odd = app.particles.len();
        app.update();
        assert_eq!(
            app.particles.len(),
            before_odd,
            "the odd tick the cadence never arms"
        );
        // THE THIRD TICK: the lifetime crossing the discard serves
        // (the twinkle the bit never set keeps its silence; the 2 >
        // 1 the crossing answers)
        app.update();
        assert!(
            app.firework_starters.is_empty(),
            "the crossing the discard keeps"
        );
        if sounds_loaded() {
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.twinkle")),
                "the twinkle-less starter the end never sings"
            );
        }
        // THE SECOND ARM: the two-explosion list the +15 ceiling rides
        // (the ctor's own break law keeps: one scan, the FIRST twinkle
        // the +15 arms — the lifetime the 2×2−1+15 the pair banks) —
        // the anchor the origin holds, the second explosion the BURST
        // shape with the EMPTY colors the BLACK fallback arms
        let twinkle_burst = mc_protocol::FireworkExplosion {
            shape: 4,
            colors: vec![],
            fade_colors: vec![],
            has_trail: false,
            has_twinkle: true,
        };
        app.live_firework_explode_events_updated(vec![crate::world_view::WireFireworkExplodeEvent {
            entity_id: 42,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
            explosions: vec![ball.clone(), twinkle_burst.clone()],
        }]);
        assert_eq!(app.firework_starters.len(), 1);
        let s2 = &app.firework_starters[0];
        assert_eq!(s2.lifetime, 3 + 15, "the first twinkle's +15 the break arms");
        assert!(s2.twinkle_delay, "the twinkle bit the ctor banks");
        // THE CADENCE'S OWN TWO TICKS: the life-0 the ball shell at
        // the origin banks, the odd tick never arms
        app.update();
        app.update();
        // THE THIRD TICK'S OWN BURST (life 2 — the index-1 explosion
        // the BURST walk answers, the 70-particle loop the bound
        // keeps, the EMPTY colors the BLACK decode names)
        let before_burst = app.particles.len();
        app.update();
        let spawned_burst = &app.particles[before_burst..];
        assert_eq!(
            spawned_burst.len(),
            70 + 1,
            "the burst the 70 sparks + the flash the walk keeps ({})",
            spawned_burst.len()
        );
        for sp in &spawned_burst[..70] {
            assert_eq!(
                sp.col,
                crate::particles::spark_color_decode(0x1E1B1B),
                "the empty colors the BLACK fallback arms"
            );
            assert_eq!(
                (sp.x, sp.y, sp.z),
                (0.0, 0.0, 0.0),
                "the burst the ONE anchor shares"
            );
        }
        let burst_flash = &spawned_burst[70];
        assert!(
            matches!(burst_flash.kind, crate::particles::ParticleKind::Flash { .. }),
            "the burst's own flash the tail banks"
        );
        assert_eq!(
            burst_flash.col,
            crate::particles::spark_color_decode(0x1E1B1B),
            "the flash the fallback's colors[0] decodes"
        );
    }


    /// THE FIREWORK'S OWN TRAIL — the tick's client walk the every-tick
    /// spark answers (javap `FireworkRocketEntity.tick` pcs 430-507 the
    /// 26.2 walk + `FireworkParticles$SparkProvider.createParticle` +
    /// `SparkParticle.<init>`): the spark the flight's EVERY tick
    /// banks (the `life % 2 >= 2` guard the never-taken arm keeps),
    /// the position the firework's own post-tick pose carries, the
    /// motion the y the -vy*0.5 fold speaks (the x/z the entity
    /// stream's gaussian pair the 0.05 fold scales), the ctor chain's
    /// own THREE engine-stream draws the lifetime band 48..=59 + the
    /// base-quad ×0.75 fold + the base-life overwrite bank, the
    /// friction 0.91 / gravity 0.1 / white (1,1,1) / alpha 0.99 laws
    /// the class's own fields keep, one spark per tick the ages walk
    /// rides.
    #[test]
    fn the_firework_the_trail_the_every_tick_spark_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        app.held_charge = Some(mc_registry::ChargeKind::Rocket);
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert_eq!(app.fireworks.len(), 1, "the shot the rocket banks");
        let shot = &app.fireworks[0];
        let vy = shot.vy;
        // THE LAUNCH TICK'S OWN TRAIL (the life-0 tick the spark the
        // launch voice's same walk banks)
        let before = app.particles.len();
        app.update();
        assert_eq!(app.fireworks.len(), 1, "the flight the first tick keeps");
        let sparks: Vec<&crate::particles::Particle> = app.particles
            [before..]
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Spark { .. }))
            .collect();
        assert_eq!(sparks.len(), 1, "the first tick ONE spark banks");
        let s = sparks[0];
        // THE POSITION (the firework's own post-tick pose the
        // addParticle call passes)
        let f = &app.fireworks[0];
        assert_eq!(
            (s.x, s.y, s.z),
            (f.x, f.y, f.z),
            "the spark the post-tick pose keeps"
        );
        // THE MOTION (the y the -vy*0.5 fold the current velocity
        // speaks bit for bit, the x/z the gaussian band the 0.05
        // fold scales)
        assert_eq!(s.vy, -vy * 0.5, "the y the -vy*0.5 fold keeps");
        assert!(s.vx.abs() < 0.3 && s.vz.abs() < 0.3, "the gaussians the small band keeps");
        // THE CTOR CHAIN'S OWN LAWS (the lifetime the 48 + nextInt(12)
        // re-roll banks, the friction/gravity/color the class fields
        // keep, the quad the base ×0.75 fold the band speaks)
        assert!(
            (48..=59).contains(&s.lifetime),
            "the 48 + nextInt(12) band the re-roll keeps ({})",
            s.lifetime
        );
        assert_eq!(s.gravity, 0.1, "the gravity the super's arg keeps");
        assert_eq!(s.friction, 0.91, "the friction the #13 keeps");
        assert_eq!(s.col, [1.0, 1.0, 1.0], "the white the base fields keep");
        assert!(
            (0.075..=0.15).contains(&s.quad_size),
            "the base ×0.75 quad the band keeps ({})",
            s.quad_size
        );
        assert_eq!(
            crate::particles::spark_alpha(s.age, s.lifetime),
            0.99,
            "the spawn the provider's 0.99 keeps"
        );
        // THE ONE SPARK PER TICK (the every-tick walk the ages ride —
        // the second and third updates the next sparks bank)
        for ticks in 1..=2u32 {
            let before = app.particles.len();
            app.update();
            let n = app.particles[before..]
                .iter()
                .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Spark { .. }))
                .count();
            assert_eq!(n, 1, "the tick {ticks} the spark banks");
            // the previous sparks the ages walk rides (the aging the
            // particle tick's own head answers BEFORE the firework
            // tick's spawn — the last spark the backward scan finds,
            // the PREVIOUS update's own spawn the one-tick walk aged)
            let aged = app.particles[..before]
                .iter()
                .rev()
                .find(|p| matches!(p.kind, crate::particles::ParticleKind::Spark { .. }))
                .expect("the previous tick's own spark");
            assert_eq!(aged.age, 1, "the age the tick's own walk rides");
            // THE MID-LIFE FADE (the age > lifetime/2 crossing the
            // alpha walk serves — the 1 - (age - half)/lifetime fold
            // the fsub/fdiv pair speaks)
            let half = aged.lifetime / 2;
            if aged.age > half {
                let want = 1.0 - (aged.age as f32 - half as f32) / aged.lifetime as f32;
                assert_eq!(
                    crate::particles::spark_alpha(aged.age, aged.lifetime),
                    want,
                    "the fade the crossing's own formula keeps"
                );
            }
        }
    }


    /// THE OFFHAND CROSSBOW'S OWN FIREWORK SHOT — the OFF turn's own
    /// Rocket kind the mirror walk answers (javap `CrossbowItem.use`
    /// the OFF hand's own turn the for-each-hand walk serves): the
    /// off-charge the Rocket kind carries, the OFF press the instant
    /// walk serves — the rocket the same eye/velocity/voice law
    /// banks, the MAIN lane the OFF shot never leaks, the LEFT
    /// render the standby restores, and the arrow lane the OFF shot
    /// never speaks.
    #[test]
    fn the_offhand_crossbow_the_firework_shot_the_mirror_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE OFFHAND CROSSBOW the offline slot wears, the MAIN hand
        // the empty inventory speaks (the MAIN turn's PASS the OFF
        // turn answers)
        app.inventory.items[slot] = (0, 0);
        app.inventory.offhand[0] = (crossbow_id, 1);
        app.offhand_held_refreshed();
        // THE ROCKET COMPONENT the OFF lane arms (the kind the OFF
        // pull's own walk never loads — the stand-in the shot's own
        // routing answers)
        app.off_charge = Some(mc_registry::ChargeKind::Rocket);
        // THE OFF SHOT: the press the instant walk answers — the
        // rocket the same createProjectile branch banks
        assert!(app.on_right_press(0, 0), "the charged OFF press answers");
        assert!(!app.is_using(), "the shot is instant — no pull");
        assert!(app.off_charge.is_none(), "the shot consumed the OFF charge");
        assert_eq!(
            app.fireworks.len(),
            1,
            "the OFF shot the firework speaks"
        );
        assert!(
            app.arrows.is_empty(),
            "the OFF shot the arrow lane never leaks"
        );
        assert!(
            app.held_charge.is_none(),
            "the MAIN lane the OFF shot never leaks"
        );
        let f = &app.fireworks[0];
        assert!(
            (f.y - (eye_y as f64 - crate::mob::FIREWORK_EYE_BACKOFF)).abs() < 1e-6,
            "the spawn the eye-minus-0.15 walk keeps ({})",
            f.y
        );
        assert!(
            f.vz < -1.57 && f.vz > -1.63,
            "the -z the 1.6 flat power speaks ({})",
            f.vz
        );
        // THE FIRST TICK: the launch word the same walk rides
        app.update();
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("entity.firework_rocket.launch @")),
                "the FIREWORK_ROCKET_LAUNCH the first tick rode"
            );
        }
        // THE RESTORE: the LEFT render the plain crossbow wears
        let plain = app.held_offhand.as_ref().expect("the LEFT plain returns");
        assert!(
            plain.binds_texture("item/crossbow_standby"),
            "the standby returns through the LEFT ctx"
        );
    }


    /// THE FIREWORK ROCKET'S OWN POSE — the render point's composite
    /// the 26.2 walk pins (javap `FireworkEntityRenderer.submit` +
    /// `Camera.setRotation` + the Axis constants the cp dump speaks):
    /// the FALSE arm the plain billboard keeps (the posed vertex the
    /// camera basis places verbatim), the TRUE arm the ZP 180 ->
    /// YP 180 -> XP 90 chain wears (the net (x, z, -y) the matrix
    /// product folds — the sprite's up the view-depth axis takes,
    /// the slab the screen-up axis carries), and the GROUND pose the
    /// updateForNonLiving walk feeds the chain (the centered bake the
    /// scale-0.5 + the [0, 2/16, 0] translation bounds).
    #[test]
    fn the_firework_rocket_the_pose_the_composite_wears() {
        // THE GROUND POSE (the posed_item_layer_quads chain — the
        // firework_rocket item the layer's own bounds carry)
        let rocket_id = mc_registry::item_id_by_name("firework_rocket")
            .expect("the firework_rocket item rides the registry");
        let layers = mc_registry::item_model_layers(rocket_id)
            .expect("the item model the registry speaks");
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        let mut quads = 0usize;
        for layer in &layers {
            let posed = mc_registry::display::posed_item_layer_quads(
                layer.model,
                layer.transform,
                mc_registry::display::GROUND,
            )
            .expect("the posed chain the known model keeps")
            .expect("the transform stanza the ground context keeps");
            for q in posed {
                quads += 1;
                for v in q.vertices {
                    for a in 0..3 {
                        min[a] = min[a].min(v[a]);
                        max[a] = max[a].max(v[a]);
                    }
                }
            }
        }
        assert!(quads >= 2, "the flat bake's two faces ({quads})");
        // the bounds the centered bake + the GROUND stanza speak: the
        // scale-0.5 sprite the x/y [-0.5, 0.5] centered bake carries,
        // the [0, 2/16, 0] translation the vanilla ground.json lifts,
        // the slab the z +-1/64 span keeps
        assert!(
            (min[0] - (-0.25)).abs() < 1e-6 && (max[0] - 0.25).abs() < 1e-6,
            "the x the centered scale-0.5 span keeps ({},{})",
            min[0],
            max[0]
        );
        assert!(
            (min[1] - (-0.125)).abs() < 1e-6 && (max[1] - 0.375).abs() < 1e-6,
            "the y the 2/16 lift + the span keeps ({},{})",
            min[1],
            max[1]
        );
        assert!(
            (min[2] - (-0.015625)).abs() < 1e-6 && (max[2] - 0.015625).abs() < 1e-6,
            "the z the slab the +-1/64 span keeps ({},{})",
            min[2],
            max[2]
        );
        // THE FALSE ARM (the plain billboard): the posed vertex the
        // camera basis places verbatim — at yaw 0 / pitch 0 the LEFT
        // = [-1,0,0], the UP = [0,1,0], the FORWARDS = [0,0,-1] the
        // constants speak (the min corner the chain's own bounds keep)
        let p = [-0.25, -0.125, -0.015625];
        let out = firework_rocket_point([10.0, 20.0, 30.0], 0.0, 0.0, false, p);
        let want = [10.0 + 0.25, 20.0 - 0.125, 30.0 + 0.015625];
        assert!(
            out.iter()
                .zip(want.iter())
                .all(|(o, w)| (o - w).abs() < 1e-5),
            "the plain billboard the basis places ({out:?} vs {want:?})"
        );
        // THE TRUE ARM (the shotAtAngle lane): the composite (x, z, -y)
        // the slab the screen-up takes and the lift the depth carries
        let out = firework_rocket_point([10.0, 20.0, 30.0], 0.0, 0.0, true, p);
        let want = [10.0 + 0.25, 20.0 - 0.015625, 30.0 - 0.125];
        assert!(
            out.iter()
                .zip(want.iter())
                .all(|(o, w)| (o - w).abs() < 1e-5),
            "the axis lane the composite places ({out:?} vs {want:?})"
        );
        // THE EAST YAW (yaw = +90 deg, the forward the +X speaks): the
        // LEFT the [-1,0,0] -> [0,0,-1] basis walk answers, the axis
        // lane the same composite carries (the lift the +x reads)
        let out = firework_rocket_point(
            [10.0, 20.0, 30.0],
            std::f32::consts::FRAC_PI_2,
            0.0,
            true,
            p,
        );
        let want = [10.0 + 0.125, 20.0 - 0.015625, 30.0 + 0.25];
        assert!(
            out.iter()
                .zip(want.iter())
                .all(|(o, w)| (o - w).abs() < 1e-5),
            "the east yaw the basis turns ({out:?} vs {want:?})"
        );
        // THE DOWN PITCH (pitch = -90 deg, the camera looks straight
        // down): the card the camera faces — the lift the -y axis
        // takes, the slab the -z reads (the sprite the camera sees)
        let out = firework_rocket_point(
            [10.0, 20.0, 30.0],
            0.0,
            -std::f32::consts::FRAC_PI_2,
            true,
            p,
        );
        let want = [10.0 + 0.25, 20.0 - 0.125, 30.0 + 0.015625];
        assert!(
            out.iter()
                .zip(want.iter())
                .all(|(o, w)| (o - w).abs() < 1e-5),
            "the down pitch the card faces ({out:?} vs {want:?})"
        );
    }


    /// THE FIREWORK ROCKET'S OWN RENDER — the flying rocket the frame
    /// keeps (javap `FireworkEntityRenderer` the 26.2 walk the lane
    /// answers): the shot the rocket banks, the camera the 1-block
    /// spot the sliver's own strips fill (the composite the sprite's
    /// up takes along the view axis — the near look the strips' 4px
    /// footprints cross the pixel centers), the render the sprite's
    /// own pixels lights (the with/without diff the frame speaks),
    /// and the empty lane the discard keeps (no rockets, no pixels —
    /// the honest skip).
    #[test]
    fn the_firework_rocket_the_render_the_pixels_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let eye_y = app.spawn_y + 7;
        // the rocket the look-axis spot 1 block ahead of the -45 down
        // look (the rig rides ABOVE the canopy the down-ray clips)
        app.held_charge = Some(mc_registry::ChargeKind::Rocket);
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert_eq!(app.fireworks.len(), 1, "the shot the rocket banks");
        assert!(
            app.fireworks[0].shot_at_angle,
            "the seventh ctor arg the shot keeps"
        );
        let s7 = 0.5f32.sqrt();
        let f = &mut app.fireworks[0];
        f.x = c as f64 + 0.5;
        f.y = eye_y as f64 + 6.0;
        f.z = c as f64 + 0.5;
        f.prev_x = f.x;
        f.prev_y = f.y;
        f.prev_z = f.z;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(
                c as f32 + 0.5,
                (eye_y + 6) as f32 + s7,
                c as f32 + 0.5 + s7,
            ),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_4,
        });
        // THE RENDER: the with/without diff the sprite's own pixels
        // answers (the same frame twice, the rocket the only delta)
        let mut fb_with = crate::raster::Framebuf::new(320, 180);
        let mut assets = GuiAssets::open(assets_root());
        app.render(&mut fb_with, &mut assets);
        let with = fb_with.color.clone();
        let rockets = std::mem::take(&mut app.fireworks);
        let mut fb_without = crate::raster::Framebuf::new(320, 180);
        app.render(&mut fb_without, &mut assets);
        let diff = with
            .iter()
            .zip(fb_without.color.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            diff > 64,
            "the rocket the frame lights ({diff} bytes differ)"
        );
        // THE EMPTY LANE: the discard the skip keeps — the re-render
        // the same frame answers (the rockets' own spot the without
        // frame already wears)
        app.fireworks = rockets;
        let mut fb_again = crate::raster::Framebuf::new(320, 180);
        app.render(&mut fb_again, &mut assets);
        assert_eq!(
            fb_again.color, with,
            "the same rockets the same frame wears"
        );
    }


    /// THE FLASH'S OWN RENDER — the alpha law the frame answers (javap
    /// `OverlayParticle.getQuadSize` + `ARGB.alpha` the 26.2 walk): the
    /// custom-ARGB color's alpha-1 quad the frame lights (the
    /// with/without diff the flash's own pixels answer — the bell's
    /// own span the age-0 fold's inverted corners cover), the dye
    /// color's alpha-0 quad the frame NEVER lights (the `setAlpha(0)`
    /// the vanilla render itself paints nothing for — the honest skip
    /// the zero arm keeps).
    #[test]
    fn the_flash_the_render_the_alpha_law_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let c = app.offline_world.as_ref().unwrap().size_blocks() / 2;
        let eye_y = app.spawn_y + 7;
        // the camera the -Z look keeps, the flash 2 blocks ahead (the
        // quad the bell's own span fills — the age-0 fold the negative
        // half answers inverted, the area the same square covers)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        let fx = c as f64 + 0.5;
        let fy = eye_y as f64;
        let fz = c as f64 + 0.5 - 2.0;
        let mut assets = GuiAssets::open(assets_root());
        // THE SPRITE'S OWN PRE-LOAD (the unit test's CWD the package
        // root keeps — the manifest-relative walk-up the audio tests
        // speak; the render's lazy load the Some arm never re-runs)
        app.flash_sprite = crate::avatar::FlashSprite::load(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../assets-cache/jar"),
        );
        assert!(
            app.flash_sprite.is_some(),
            "the flash sprite the cache load answers"
        );
        // THE DYE-COLOR FLASH (0xFF0000 — the alpha byte the 0x00
        // keeps): the quad the frame never lights
        app.particles.push(crate::particles::firework_flash(
            fx,
            fy,
            fz,
            0xFF0000,
            &mut app.level_ambient_rng,
        ));
        let mut fb_dye = crate::raster::Framebuf::new(320, 180);
        app.render(&mut fb_dye, &mut assets);
        let dye = fb_dye.color.clone();
        let _dye_particle = std::mem::take(&mut app.particles);
        let mut fb_base = crate::raster::Framebuf::new(320, 180);
        app.render(&mut fb_base, &mut assets);
        let diff_dye = dye
            .iter()
            .zip(fb_base.color.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert_eq!(
            diff_dye, 0,
            "the alpha-0 flash the frame never lights ({diff_dye})"
        );
        // THE CUSTOM-ARGB FLASH (0xFFFF0000 — the alpha byte the FF
        // keeps): the alpha-1 quad the frame lights
        app.particles.push(crate::particles::firework_flash(
            fx,
            fy,
            fz,
            0xFFFF0000u32 as i32,
            &mut app.level_ambient_rng,
        ));
        let mut fb_argb = crate::raster::Framebuf::new(320, 180);
        app.render(&mut fb_argb, &mut assets);
        let diff_argb = fb_argb
            .color
            .iter()
            .zip(fb_base.color.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            diff_argb > 64,
            "the alpha-1 flash the frame lights ({diff_argb})"
        );
    }


    /// THE CROSSBOW'S OWN ROCKET LOAD — the HELD firework the
    /// ARROW_OR_FIREWORK scan loads (javap `Player.getProjectile` +
    /// `CrossbowItem.onUseTick` the 26.2 walk speaks — the held scan
    /// the getHeldProjectile's OFF_HAND test keeps, the container
    /// order the ARROW_ONLY loop the firework never passes, the
    /// crossing the tryLoadProjectiles + useAmmo's split(1) walk
    /// serves): the offhand rocket the pull loads mid-hold, the
    /// charged FIREWORK bolt the mid-hold render wears, the release
    /// the component keeps, and the press the rocket's own shot
    /// speaks — the held-a-rocket arc the full loop answers.
    #[test]
    fn the_crossbow_the_rocket_load_the_held_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let rocket_id = wire("firework_rocket");
        let slot = app.selected_hotbar;
        // THE CROSSBOW the main hand holds, the ROCKET the offhand
        // carries (the held scan's OFF_HAND branch — the firework the
        // container scan never picks, the held hand alone loads it)
        app.inventory.items[slot] = (crossbow_id, 1);
        app.inventory.offhand[0] = (rocket_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let eye_y = app.spawn_y + 7;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
            yaw: 0.0,
            pitch: 0.0,
        });
        // THE PULL: the crossing the ROCKET loads mid-hold — the kind
        // the stack's own id names, the consume the offhand's own
        // stack costs
        assert!(app.on_right_press(0, 0), "the pull starts");
        for _ in 0..25 {
            app.update();
        }
        assert_eq!(
            app.held_charge,
            Some(mc_registry::ChargeKind::Rocket),
            "the held firework the crossing LOADED as the Rocket kind"
        );
        assert_eq!(
            app.inventory.offhand[0].1, 0,
            "the survival draw the offhand's stack costs"
        );
        let midhold = app.held_item.as_ref().expect("the charged payload mid-hold");
        assert!(
            midhold.binds_texture("item/crossbow_firework"),
            "the ROCKET bolt the mid-hold render wears"
        );
        app.on_right_release();
        assert_eq!(
            app.held_charge,
            Some(mc_registry::ChargeKind::Rocket),
            "the release the component keeps"
        );
        // THE SHOT: the charged press the firework's own walk answers
        // — the rocket the eye-minus-0.15 spawn rides, the arrow lane
        // the Rocket kind never leaks
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert!(app.held_charge.is_none(), "the shot consumed the charge");
        assert_eq!(
            app.fireworks.len(),
            1,
            "the loaded rocket the firework's own shot speaks"
        );
        assert!(
            app.arrows.is_empty(),
            "the Rocket kind the arrow lane never leaks"
        );
        let f = &app.fireworks[0];
        assert!(
            (f.y - (eye_y as f64 - crate::mob::FIREWORK_EYE_BACKOFF)).abs() < 1e-6,
            "the spawn the eye-minus-0.15 walk keeps ({})",
            f.y
        );
        assert!(
            f.vz < -1.57 && f.vz > -1.63,
            "the -z the 1.6 flat power speaks ({})",
            f.vz
        );
    }


    /// THE CROSSBOW'S OWN EMPTY SCAN — the no-load silence the
    /// tryLoadProjectiles' false keeps (javap `CrossbowItem.onUseTick`
    /// the 26.2 walk speaks — the crossing the isCharged gate re-arms
    /// every tick, the scan's own empty verdict the component never
    /// arms): the projectile-less pull the load never speaks, the
    /// release the plain crossbow restores, and the fresh press the
    /// pull re-arms (the vanilla's own uncharged behavior).
    #[test]
    fn the_crossbow_the_empty_scan_the_silence_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        // THE PAST-THE-CROSSING HOLD: the scan's empty verdict the
        // component never arms — every tick the isCharged gate keeps
        // asking, the empty scan the silence keeps
        assert!(app.on_right_press(0, 0), "the pull starts");
        for _ in 0..30 {
            app.update();
        }
        assert!(
            app.held_charge.is_none(),
            "the empty scan the load never speaks"
        );
        assert!(app.is_using(), "the hold the crossing never ends");
        app.on_right_release();
        assert!(
            app.held_charge.is_none(),
            "the release the empty scan's silence keeps"
        );
        let plain = app.held_item.as_ref().expect("the plain crossbow returns");
        assert!(
            plain.binds_texture("item/crossbow_standby"),
            "the under-charged law the plain restore keeps"
        );
        // THE RE-PULL: the fresh press the pull re-arms (the vanilla's
        // own uncharged behavior — the crossbow the load never wore)
        assert!(app.on_right_press(0, 0), "the fresh pull starts");
        assert!(app.is_using(), "the pull the fresh stack arms");
    }


    /// THE CROSSBOW'S OWN CHARGING-SOUND ARC — the three voices the
    /// ratio walk banks (javap `CrossbowItem.onUseTick` pcs 31-156 the
    /// 26.2 bytecode the round reads): the START the `f >= 0.2` arm
    /// speaks at elapsed 5 (5/25 == 0.2f the exact f32 the fcmpl
    /// equality answers), the MID the `f >= 0.5` arm at elapsed 13
    /// (13/25 == 0.52 the f32 band keeps), the END the tryLoad-
    /// Projectiles SUCCESS alone banks at the crossing (elapsed 25 —
    /// the empty scan's early return the silence keeps); the
    /// `f < 0.2` reset the once-only flags the fresh pull re-arms
    /// (the per-use state the vanilla statics' own reset law wears),
    /// the start/mid the FIXED 0.5/1.0 volume/pitch pair the
    /// lambda$0/lambda$1 fold keeps, the end the 1.0 volume + the
    /// `1.0F / (nextFloat() * 0.5F + 1.0F) + 0.2F` pitch the
    /// lambda$2's level-random draw speaks, and the ARC the
    /// once-per-pull law wears (the past-the-crossing ticks the
    /// played flags silence — the shot the no-arc voice keeps).
    #[test]
    fn the_crossbow_the_pull_arc_the_three_voices_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let arrow_id = wire("arrow");
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (crossbow_id, 1);
        app.inventory.items[(slot + 1) % 9] = (arrow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        let arc_count =
            |app: &App| {
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with("item.crossbow.loading_"))
                    .count()
            };
        // THE PRESS: the clock the fresh hold keeps, the base the arc
        // counter snapshots (the press the no-arc silence speaks)
        assert!(app.on_right_press(0, 0), "the pull starts");
        assert_eq!(app.using_item.as_ref().unwrap().ticks, 0);
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 0, "the press the arc silence keeps");
        }
        // THE FIRST FOUR TICKS: the `f < 0.2` reset the flags clear,
        // the start the threshold's own approach never arms early
        for _ in 0..4 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 0, "the elapsed 1..4 the silence keeps");
        }
        // THE FIFTH TICK: the exact 0.2f crossing the START arm speaks
        app.update();
        assert_eq!(app.using_item.as_ref().unwrap().ticks, 5);
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 1, "the elapsed 5 the start alone speaks");
            assert!(
                app.sounds.events.iter().any(|e| e
                    .starts_with("item.crossbow.loading_start @")
                    && e.contains(" v0.500000 p1.000000")),
                "the FIXED 0.5/1.0 pair the lambda$0 fold keeps: {:?}",
                app.sounds.events
            );
        }
        // THE SIXTH..TWELFTH: the once-only flag the re-arm silence keeps
        for _ in 0..7 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 1, "the past-start ticks the silence keeps");
        }
        // THE THIRTEENTH: the 0.52 crossing the MID arm speaks
        app.update();
        assert_eq!(app.using_item.as_ref().unwrap().ticks, 13);
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 2, "the elapsed 13 the mid speaks");
            assert!(
                app.sounds.events.iter().any(|e| e
                    .starts_with("item.crossbow.loading_middle @")
                    && e.contains(" v0.500000 p1.000000")),
                "the FIXED pair the lambda$1 fold keeps: {:?}",
                app.sounds.events
            );
        }
        // THE FOURTEENTH..TWENTY-FOURTH: the approach the silence keeps
        for _ in 0..11 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 2, "the past-mid ticks the silence keeps");
        }
        // THE CROSSING: the load the component banks, the END the
        // success alone speaks — the 1.0 volume the lambda$2's own
        // fold keeps, the pitch the level-random band [0.8667, 1.2]
        app.update();
        assert_eq!(app.using_item.as_ref().unwrap().ticks, 25);
        assert_eq!(
            app.held_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the crossing the component banks"
        );
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 3, "the crossing the end voice speaks");
            let end_line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("item.crossbow.loading_end @"))
                .expect("the end voice the log wears");
            assert!(
                end_line.contains(" v1.000000 p"),
                "the 1.0 volume the lambda$2 keeps: {end_line}"
            );
            let pitch: f32 = end_line
                .split(" p")
                .nth(1)
                .expect("the pitch the tail carries")
                .parse()
                .expect("the pitch the f32 parses");
            assert!(
                pitch >= 0.866_666_6 && pitch <= 1.2,
                "the `1.0F / (nextFloat() * 0.5F + 1.0F) + 0.2F` band keeps ({pitch})"
            );
        }
        // THE PAST-THE-CROSSING HOLD: the played flags the arc silence
        // keeps (the isCharged gate the load never re-banks)
        for _ in 0..5 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 3, "the once-per-pull law the hold keeps");
        }
        assert_eq!(app.inventory.items[(slot + 1) % 9].1, 0, "the draw the quiver costs");
        // THE RELEASE + THE CHARGED SHOT: the shot the no-arc voice
        // keeps (the use()'s own CONSUME branch the sounds never bank)
        app.on_right_release();
        assert!(app.on_right_press(0, 0), "the charged press answers");
        assert!(!app.is_using(), "the shot the instant walk serves");
        assert_eq!(app.arrows.len(), 1, "the shot the arrow speaks");
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 3, "the shot the arc silence keeps");
        }
        // THE EMPTY-QUIVER RE-PULL: the arc the scan's own verdict
        // never reads — the start the fresh pull re-arms (the reset
        // law the per-use state wears), the mid the same ratio speaks,
        // the END the failed scan the silence keeps
        app.on_right_release();
        assert!(app.on_right_press(0, 0), "the fresh pull starts");
        for _ in 0..5 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 4, "the fresh pull the start re-arms");
            assert!(
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with("item.crossbow.loading_start @"))
                    .count()
                    == 2,
                "the SECOND start the fresh pull speaks"
            );
        }
        for _ in 0..8 {
            app.update();
        }
        app.update();
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 5, "the fresh pull the mid re-arms");
        }
        for _ in 0..11 {
            app.update();
        }
        app.update();
        assert!(
            app.held_charge.is_none(),
            "the empty scan the component never arms"
        );
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 5, "the failed scan the END silence keeps");
        }
    }


    /// THE ARC THE TWO LANES ANSWER — the OFF pull the same three
    /// voices wear (the per-use flags the OFF turn's own state keeps,
    /// the crossing the OFF lane's END speaks), the LIVE session the
    /// silence keeps (the isClientSide head the server-side walk
    /// names — the server's own voices the wire's WorldSound lane
    /// banks, the client predicts none) while the LOCAL BELIEF the
    /// crossing still arms (the component the wire's own sync the
    /// recorded law keeps — the arrow stand-in the pull arms).
    #[test]
    fn the_crossbow_the_arc_the_offhand_wears_the_live_silence_keeps() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let crossbow_id = wire("crossbow");
        let arrow_id = wire("arrow");
        let arc_count =
            |app: &App| {
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with("item.crossbow.loading_"))
                    .count()
            };
        // THE OFF LANE: the empty main hand the OFF turn answers, the
        // arrow the MAIN slot the scan's held branch consumes
        let mut app = App::new(3);
        app.enter_world();
        let slot = app.selected_hotbar;
        app.inventory.items[slot] = (0, 0);
        app.inventory.offhand[0] = (crossbow_id, 1);
        app.offhand_held_refreshed();
        app.inventory.items[slot] = (arrow_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the OFF pull starts");
        assert!(matches!(
            app.using_item.as_ref().map(|u| u.hand),
            Some(UseHand::Off)
        ));
        for _ in 0..5 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 1, "the OFF pull the start speaks");
        }
        for _ in 0..8 {
            app.update();
        }
        app.update();
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 2, "the OFF pull the mid speaks");
        }
        for _ in 0..11 {
            app.update();
        }
        app.update();
        assert_eq!(
            app.off_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the crossing the OFF lane loads"
        );
        if sounds_loaded() {
            assert_eq!(arc_count(&app), 3, "the OFF lane the END voice wears");
        }
        // THE LIVE LANE: the silence the isClientSide head keeps — the
        // pull the wire echo arms, the ticks the same clock advances,
        // the crossing the LOCAL BELIEF the component still banks, no
        // voice the client's own log speaks
        let mut live = App::new(3);
        live.enter_world();
        let lslot = live.selected_hotbar;
        live.inventory.items[lslot] = (crossbow_id, 1);
        live.inventory.items[(lslot + 1) % 9] = (arrow_id, 1);
        live.hotbar_slots = live.inventory.hotbar();
        let (tx, _rx) = std::sync::mpsc::channel::<ContainerCommand>();
        live.attach_live_clicks(tx);
        assert!(live.on_right_press(0, 0), "the live pull the wire arms");
        for _ in 0..5 {
            live.update();
        }
        if sounds_loaded() {
            assert_eq!(
                arc_count(&live),
                0,
                "the live pull the client's own silence keeps"
            );
        }
        for _ in 0..20 {
            live.update();
        }
        assert_eq!(
            live.held_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the local belief the crossing arms"
        );
        if sounds_loaded() {
            assert_eq!(
                arc_count(&live),
                0,
                "the live session the server's own voices the wire banks"
            );
        }
    }


    /// THE OFFHAND'S OWN PULL ARC — the LAYER-pull lane the OFF turn
    /// re-bakes through the LEFT ctx (the vanilla startUseItem's
    /// for-each-hand walk: the MAIN hand's PASS the OFF hand's useItem
    /// answers): the off-hand crossbow the empty main hand backs, the
    /// press starts the OFF pull (the OFF_HAND ordinal the echo
    /// speaks), the full-charge crossing LOADS the OFF lane (javap
    /// `CrossbowItem.onUseTick`: tryLoadProjectiles sets the
    /// CHARGED_PROJECTILES component at the ratio >= 1.0 tick — the
    /// release the boolean the update walk reads), the LEFT resting
    /// render switches to the charged
    /// bolt (the charge_type select's arrow case through the LEFT
    /// context), the next right press fires INSTANTLY (the use()'s
    /// CONSUME branch — no pull starts), and the MAIN lane the OFF
    /// load never leaks (the dev-62 divergence the route closes).
    #[test]
    fn the_offhand_pull_charges_and_fires() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);

        // THE EMPTY MAIN HAND the OFF turn the press answers (the MAIN
        // lane's PASS the walk's own order keeps)
        app.live_held_updated(None);
        app.live_inv_slots[45] = (wire("crossbow"), 1);
        app.offhand_held_refreshed();
        assert!(app.off_charge.is_none(), "the fresh stack carries no component");
        assert!(
            app.on_right_press(0, 0),
            "the offhand crossbow starts the OFF pull"
        );
        assert!(matches!(
            app.using_item.as_ref().map(|u| u.hand),
            Some(UseHand::Off)
        ), "the OFF hand the use clock wears");
        match click_rx.try_recv().expect("the OFF pull's echo rode the lane") {
            ContainerCommand::UseItem { hand: 1, .. } => {}
            other => panic!("expected a hand-1 UseItem echo, got {other:?}"),
        }
        let pulling = app.held_offhand.as_ref().expect("the LEFT pulling loads");
        assert!(
            pulling.binds_texture("item/crossbow_pulling_0"),
            "the LEFT ctx the using-branch layers wear"
        );
        // 24 ticks then the release: under the charge duration — the
        // power < 1.0 loads nothing (the javap releaseUsing gate)
        for _ in 0..24 {
            app.update();
        }
        app.on_right_release();
        assert!(app.off_charge.is_none(), "24 < 25 ticks — no off load");
        assert!(
            matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)),
            "the release echo rode the lane"
        );
        // THE FULL CHARGE: 25 ticks then the release — the OFF lane
        // the load law routes (the MAIN lane the leak never speaks)
        assert!(app.on_right_press(0, 0), "the OFF pull starts again");
        let _ = click_rx.try_recv();
        for _ in 0..25 {
            app.update();
        }
        app.on_right_release();
        assert_eq!(
            app.off_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the OFF pull LOADED the off lane (the component set)"
        );
        assert!(
            app.held_charge.is_none(),
            "the OFF load the MAIN lane never leaks — the route the hand speaks"
        );
        let bolt = app.held_offhand.as_ref().expect("the LEFT charged bolt loads");
        assert!(
            bolt.binds_texture("item/crossbow_arrow"),
            "the charge_type select's arrow case through the LEFT ctx"
        );
        assert!(
            matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)),
            "the second release echo rode the lane"
        );
        // THE INSTANT SHOT: the right press on the charged OFF crossbow
        // consumes the OFF lane — no pull starts (the vanilla CONSUME
        // branch), the plain crossbow renders LEFT again, the shot's
        // hand-1 echo rides
        assert!(
            app.on_right_press(0, 0),
            "the charged OFF crossbow answers the press"
        );
        assert!(app.off_charge.is_none(), "the shot consumed the OFF charge");
        assert!(!app.is_using(), "the shot is instant — no pull started");
        let plain = app.held_offhand.as_ref().expect("the LEFT plain returns");
        assert!(
            plain.binds_texture("item/crossbow_standby"),
            "the standby texture returns through the LEFT ctx"
        );
        match click_rx.try_recv().expect("the OFF shot's echo rode the lane") {
            ContainerCommand::UseItem { hand: 1, .. } => {}
            other => panic!("expected a hand-1 UseItem echo, got {other:?}"),
        }
    }


    /// THE OFFHAND CHARGE'S OWN STACK LAW — the component belongs to
    /// the STACK, never the slot (the same law the main lane's
    /// `held_charge` wears): the off-hand item change drops the
    /// off_charge (the refresh walk's id-change key), the emptiness
    /// drops it too, and the SAME-id refresh the charge keeps (the
    /// lazy draw cadence never re-clears).
    #[test]
    fn the_offhand_charge_dies_with_the_stack() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        // THE LIVE BELIEF the slot-45 lane reads (the live_clicks the
        // offhand_stack's own source switch arms — the store's slot 45
        // the window-0 store keeps)
        let (click_tx, _click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.live_inv_slots[45] = (wire("crossbow"), 1);
        app.offhand_held_refreshed();
        // THE LOADED OFF STACK: the lane the OFF pull's own law set
        app.off_charge = Some(mc_registry::ChargeKind::Arrow);
        // THE SAME-ID REFRESH: the lazy draw cadence the charge keeps
        app.offhand_held_refreshed();
        assert_eq!(
            app.off_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the same-id refresh the charge never re-clears"
        );
        // THE ITEM CHANGE: the component the old stack alone kept —
        // the refresh the id-change key drops it
        app.live_inv_slots[45] = (wire("shield"), 1);
        app.offhand_held_refreshed();
        assert!(
            app.off_charge.is_none(),
            "the stack switch dropped the OFF charge"
        );
        // THE EMPTINESS: the cleared offhand slot the same law speaks
        app.off_charge = Some(mc_registry::ChargeKind::Rocket);
        app.live_inv_slots[45] = (0, 0);
        app.offhand_held_refreshed();
        assert!(
            app.off_charge.is_none(),
            "the emptied offhand dropped the OFF charge"
        );
    }


    /// THE OFFHAND BRUSH'S OWN TURN — the useOn lane the OFF turn
    /// speaks (the vanilla startUseItem's for-each-hand walk: the MAIN
    /// turn's PASS the OFF hand's useOn answers, javap `BrushItem.useOn`
    /// the BLOCK-hit gate the MAIN lane's own arm wears, the OFF ctx the
    /// window opens): the off-hand brush the empty main hand backs, the
    /// press the always-CONSUME the `getstatic #44` arms (the OFF window
    /// the `UseHand::Off` wears, the state the def-less item keeps),
    /// the echo the OFF ctx's own packet speaks (the hand-1 ordinal the
    /// two-turn walk's own field carries — the MAIN turn's block-half
    /// echo the head spoke first, neither doubles), the look-away the
    /// entity-level cancel closes (NO packet the cancel sends), the MISS
    /// the PASS keeps (the useOn the BLOCK case alone routes — the
    /// window never opens), and the re-press the window re-opens (the
    /// release the RMB edge serves).
    #[test]
    fn the_offhand_brush_the_window_the_echo_wear() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the STONE row keeps (the generic arm the tick
        // walk names — the same law the main lane's own arc wears)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        // THE OFFHAND BRUSH the LIVE slot-45 store keeps (the
        // offhand_stack's own source switch arms), the MAIN hand the
        // empty inventory speaks (the PASS the OFF turn answers)
        let (tx, rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(tx);
        let brush = wire("brush");
        app.live_inv_slots[45] = (brush, 1);
        app.offhand_held_refreshed();
        assert_eq!(
            app.offhand_stack(),
            (brush, 1),
            "the OFF stack the slot-45 store wears"
        );
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.on_right_press(0, 0),
            "the OFF turn the always-CONSUME speaks"
        );
        let active = app.using_item.as_ref().expect("the OFF window opened");
        assert_eq!(active.item, brush, "the OFF brush the use rides");
        assert!(
            matches!(active.hand, UseHand::Off),
            "the OFF hand the window wears"
        );
        assert_eq!(active.ticks, 0, "the clock the fresh hold keeps");
        assert_eq!(active.state, 0, "the state the def-less item keeps");
        // THE TWO ECHOES the two turns' own ctx wears: the MAIN turn's
        // block-half echo the head speaks (hand 0 — the packet every
        // block right-click rides), the OFF turn's own useOn the hand-1
        // ordinal carries (the OFF ctx's own packet)
        match rx.try_recv().expect("the MAIN turn's block echo rode") {
            ContainerCommand::UseItemOn { hand: 0, .. } => {}
            other => panic!("expected the MAIN turn's UseItemOn echo, got {other:?}"),
        }
        match rx.try_recv().expect("the OFF turn's own echo rode") {
            ContainerCommand::UseItemOn { hand: 1, pos, .. } => {
                assert_eq!(pos, (c, surface, c), "the hit cell the OFF ctx wears");
            }
            other => panic!("expected the OFF turn's hand-1 UseItemOn echo, got {other:?}"),
        }
        assert!(
            rx.try_recv().is_err(),
            "the window the echoes never double"
        );
        // THE LOOK-AWAY CANCEL (the tick walk's own clip re-walk — the
        // entity-level release the silence keeps, NO packet the cancel
        // sends)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        app.update();
        assert!(
            app.using_item.is_none(),
            "the look-away the OFF window closes"
        );
        assert!(
            rx.try_recv().is_err(),
            "the cancel no packet sends — the entity walk the vanilla silence keeps"
        );
        // THE MISS ARM (the OFF brush the air walk the PASS keeps — the
        // useOn the BLOCK case alone routes, the `use` body the jar
        // never banks)
        assert!(!app.on_right_press(0, 0), "the miss the PASS keeps");
        assert!(
            app.using_item.is_none(),
            "the miss the OFF window never opens"
        );
        assert!(rx.try_recv().is_err(), "the miss no echo speaks");
        // THE RE-PRESS (the window the OFF turn re-opens) + the RELEASE
        // (the RMB edge the hold ends, the RELEASE_USE echo rides)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the re-press the window re-opens");
        let _ = rx.try_recv();
        let _ = rx.try_recv();
        app.on_right_release();
        assert!(app.using_item.is_none(), "the release the OFF hold ends");
        assert!(
            matches!(rx.try_recv(), Ok(ContainerCommand::ReleaseUse)),
            "the release echo rode the lane"
        );
    }


    /// THE OFFHAND BRUSH'S OWN OFFLINE ARC — the offline sources the
    /// `offhand_stack` switch arms speak (the inventory's offhand slot
    /// the live-less lane reads) and THE FOR-EACH-HAND ORDER (the MAIN
    /// turn's own CONSUME the OFF turn never reaches): the both-hands
    /// brush the MAIN window the order answers, the cleared main hand
    /// the OFF window the OFF turn speaks, and the cadence voice the
    /// tick walk's own law rides (the hand the walk never reads — the
    /// tick the using_item's own name arms).
    #[test]
    fn the_offhand_brush_the_offline_lane_the_order_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        let brush = mc_registry::item_id_by_name("brush").unwrap();
        // THE BOTH-HANDS BRUSH: the MAIN turn's own arm the order
        // answers (the block lane's brush arm the CONSUME speaks — the
        // OFF turn the walk never reaches)
        app.inventory.items[app.selected_hotbar] = (brush, 1);
        app.hotbar_slots = app.inventory.hotbar();
        app.inventory.offhand[0] = (brush, 1);
        app.offhand_held_refreshed();
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the MAIN turn the order answers");
        assert!(
            matches!(
                app.using_item.as_ref().map(|u| u.hand),
                Some(UseHand::Main)
            ),
            "the MAIN window the for-each-hand order speaks"
        );
        app.on_right_release();
        assert!(app.using_item.is_none(), "the release the MAIN hold ends");
        // THE CLEARED MAIN HAND: the OFF turn the OFF window speaks (the
        // offline offhand slot the source switch answers)
        app.inventory.items[app.selected_hotbar] = (0, 0);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the OFF turn the press answers");
        assert!(
            matches!(
                app.using_item.as_ref().map(|u| u.hand),
                Some(UseHand::Off)
            ),
            "the OFF window the OFF turn speaks"
        );
        // THE CADENCE VOICE the tick walk's own law rides (the hand the
        // walk never reads — the t=5 voice the OFF window's tick arms)
        let _ = std::mem::take(&mut app.sounds.events);
        for _ in 0..5 {
            app.update();
        }
        if sounds_loaded() {
            assert_eq!(
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with("item.brush.brushing.generic @"))
                    .count(),
                1,
                "the t=5 voice the OFF window's tick arms"
            );
        }
        app.on_right_release();
        assert!(app.using_item.is_none(), "the release the OFF hold ends");
    }


    /// THE BLOCK-USE LANE's OFFLINE ARC — the App owns the procedural
    /// voxels, the `--held-item` stand-in holds dirt, the look seam aims
    /// the camera straight down at the spawn column, and the placement
    /// lands in the hit face's adjacent cell (the mesh re-speaks). The
    /// reach ends the lane honestly (the sky miss consumes nothing) and
    /// the vanilla refuses the placement into the player's own box.
    #[test]
    fn the_block_use_lane_places_the_held_block_offline() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/dirt"));
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        assert_eq!(app.offline_held, Some(wire("dirt")), "the reverse lookup route");
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        // top_solid_y speaks the STANDING cell (the surface's top solid
        // Y + 1) — the placement cell the straight-down ray fills
        let top = app.spawn_y;
        // THE AIM: the eye 3.0 above the standing cell's floor, straight
        // down — the hit voxel is the surface block (top-1), the face
        // UP, the adjacent cell = top itself
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        let quads_before = app.world_quads.as_ref().unwrap().len();
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DIRT,
            "the held dirt landed in the adjacent cell"
        );
        assert_ne!(
            app.world_quads.as_ref().unwrap().len(),
            quads_before,
            "the mesh re-spoke with the placed voxel"
        );
        // THE REACH MISS: 40 above the surface — the clip ends in the air,
        // the press consumes nothing (the vanilla's own no-hit use)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 40.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the miss does not consume");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top + 2, c),
            crate::world::AIR,
            "no voxel landed on the miss"
        );
        // THE PLAYER'S OWN BOX: the feet standing ON the placed dirt (the
        // eye 1.62 above the feet cell's floor) — the next placement cell
        // is the feet's own cell and the vanilla refuses it
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 2.0 + 0.62, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the vanilla refuses the self-intersecting placement");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top + 2, c),
            crate::world::AIR,
            "the refused cell stays air"
        );
    }


    // THE FLOWER-AT-YOUR-FEET LAW (the canPlace chain's own split — the
    // collision shape the obstruction walk reads): the SAME self-cell
    // staging the dirt refusal wears — the FULL-cube dirt the refusal
    // keeps, the DANDELION the noCollision() family's EMPTY shape
    // answers (the isUnobstructed shortcut the entity walk never
    // reaches) — the flower the player's own cell WEARS.
    #[test]
    fn the_placement_wears_the_flower_at_your_feet_law() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y; // the standing cell (the surface's top solid Y + 1)
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top - 1, c),
            crate::world::GRASS,
            "the below cell the survival law's grass wears"
        );
        // THE EYE AT THE STANDING CELL'S OWN 1.62: the feet at top (the
        // feet cell the placement cell the straight-down ray fills)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.62, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE FULL CUBE: the dirt the refusal keeps (the same law the
        // block-use test pins, the obstruction walk's own verdict)
        app.offline_held_updated(Some("item/dirt"));
        assert!(
            app.placement_obstructed(crate::world::DIRT, (c, top, c), 1),
            "the dirt's full box the player's body intersects"
        );
        assert!(
            !app.on_right_press(0, 0),
            "the vanilla refuses the dirt at the feet cell"
        );
        // THE EMPTY SHAPE: the dandelion the shortcut answers — the
        // placement at the player's own cell the walk NEVER refuses
        app.offline_held_updated(Some("item/dandelion"));
        assert!(
            !app.placement_obstructed(crate::world::DANDELION, (c, top, c), 1),
            "the dandelion's empty shape the walk never refuses"
        );
        assert!(app.on_right_press(0, 0), "the flower consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DANDELION,
            "the flower the player's own cell wears"
        );
    }


    // THE DOOR-IN-YOUR-OWN-CELL LAW (the vanilla's thin shape the
    // placement allows — the Full-band refusal the documented gap the
    // variant-aware lane retires, the autopush-657 round): the placed
    // state's facing the `door_facing_byte` walk the lane stamps — the
    // yaw 0 the north byte the SOUTH strip the far edge banks, the
    // body's own box the strip never intersects; the yaw pi the south
    // facing the NORTH strip the near edge wears, the body shifted
    // into the strip's own band the refusal keeps (the dirt control
    // the full-cube law the same staging wears).
    #[test]
    fn the_door_the_place_the_own_cell_the_band_admits() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE EYE AT THE STANDING CELL'S OWN 1.62 (the feet box the
        // cell's own ±0.3 wears — the far-edge strip the overlap never
        // speaks)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.62, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.placement_obstructed(crate::world::OAK_DOOR, (c, top, c), 1),
            "the far-edge strip the own-cell placement admits"
        );
        // THE DIRT CONTROL (the full cube the refusal keeps — the same
        // staging the flower test's own law wears)
        assert!(
            app.placement_obstructed(crate::world::DIRT, (c, top, c), 1),
            "the dirt's full box the body intersects"
        );
        // THE NEAR EDGE (the yaw pi the south facing the NORTH strip
        // wears — the eye shifted the strip's own band the body's box
        // overlaps, the refusal the vanilla's own walk speaks)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 1.62, c as f32 + 0.45),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            app.placement_obstructed(crate::world::OAK_DOOR, (c, top, c), 1),
            "the near-edge strip the body's box the refusal keeps"
        );
    }


    // THE OBSTRUCTION WALK'S OWN BODY LAW (the getEntities(null, bounds)
    // the canPlace chain speaks): the mobs' feet-anchored boxes the
    // placed shape's move collides with — the FULL cube the mob's cell
    // refuses, the EMPTY shape never, the COLUMN's 15/16 band the mob's
    // own height band intersects (the feet inside the cell), the body
    // ABOVE the band the walk lets pass.
    #[test]
    fn the_obstruction_walk_refuses_the_mobs_box() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE MOB: the zombie's feet anchored at the standing cell's
        // floor — its box (0.6 wide, 1.95 tall) fills the cell's own
        // band; the camera parked far above keeps the player's box out
        app.mobs.push(crate::mob::Zombie::new(
            9,
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5,
            3,
        ));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 40.0 + 1.62, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE FULL CUBE AT THE MOB'S CELL: the refusal
        assert!(
            app.placement_obstructed(crate::world::DIRT, (c, top, c), 1),
            "the dirt's box the zombie's box intersects"
        );
        // THE EMPTY SHAPE: the torch the shortcut never refuses
        assert!(
            !app.placement_obstructed(crate::world::TORCH, (c, top, c), 1),
            "the torch's empty shape never refuses"
        );
        // THE COLUMN'S BAND: the farmland's 15/16 the mob's feet band
        // intersects (the feet at the cell's floor, the box [0, 0.9375])
        assert!(
            app.placement_obstructed(crate::world::FARMLAND, (c, top, c), 1),
            "the column's band the zombie's feet intersects"
        );
        // THE BAND'S OWN EDGE: the mob's feet ON the column's own top
        // (the 15/16 stand — 0.9375) — the strict-edge intersect the
        // joinIsNotEmpty walk speaks: the body exactly ON the band the
        // walk lets pass
        app.mobs[0].y = top as f64 + 15.0 / 16.0;
        assert!(
            !app.placement_obstructed(crate::world::FARMLAND, (c, top, c), 1),
            "the body on the band's edge the walk lets pass"
        );
    }


    /// THE HOE'S OWN TILL ARM — the straight-down aim the vanilla useOn's
    /// SUCCESS speaks (javap HoeItem.useOn over the real 26.2 jar): the
    /// held hoe the surface grass looks up (the TILLABLE map's own
    /// GRASS_BLOCK row), the onlyIfAirAbove the standing cell's air
    /// answers, the farmland lands IN the clicked cell (the useOn's own
    /// setBlock(pos, FARMLAND, 11) — not the face's adjacent cell the
    /// placement fills), the mesh re-speaks, the HOE_TILL voice the log
    /// rides, the press consumed.
    #[test]
    fn the_hoe_tills_the_grass_the_farmland_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the GRASS row keeps (the beach's sand never
        // speaks — the test pins the tillable kind itself)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::GRASS);
        app.offline_held_updated(Some("item/wooden_hoe"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let quads_before = app.world_quads.as_ref().unwrap().clone();
        assert!(app.on_right_press(0, 0), "the till consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::FARMLAND,
            "the grass cell wears the farmland"
        );
        // THE MESH RE-SPOKE: the same cube footprint the quad COUNT keeps
        // (grass->farmland the geometry never changes), the TILES the uv
        // baked content moves — the top face's farmland master the
        // grass's tinted top never was
        assert_ne!(
            app.world_quads.as_ref().unwrap(),
            &quads_before,
            "the mesh re-spoke with the tilled voxel"
        );
        // THE TILL'S VOICE (the assert computes the expected from the
        // ACTUAL loaded state — the cache-absent boot the silent lane keeps)
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.hoe.till @")),
                "the HOE_TILL voice the useOn's own order rode"
            );
        }
    }


    /// THE TILL'S OWN AIR-ABOVE GUARD — the onlyIfAirAbove predicate's
    /// refusal (javap: getClickedPos().above().isAir() the strict air
    /// speaks): the held hoe the side ray rides at the dirt's own layer,
    /// the hit voxel the dirt cell keeps (the occupied above never
    /// blocks the horizontal walk), the predicate's false the PASS —
    /// no farmland, no sound, the press never consumed.
    #[test]
    fn the_till_the_air_above_refuses() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::DIRT); // the tillable hit
            w.set(c, surface + 1, c, crate::world::STONE); // the occupied above
            // the trench the side ray's own layer clears (the ray's
            // path the camera-to-face walk keeps solid-free)
            for dz in -3..0 {
                w.set(c, surface, c + dz, crate::world::AIR);
            }
        }
        app.offline_held_updated(Some("item/wooden_hoe"));
        // THE SIDE AIM: the eye the dirt's own layer rides, the yaw PI
        // the forward's +Z speaks — the hit voxel the dirt cell, the
        // NORTH face the entry speaks
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 0.5, (c - 3) as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: 0.0,
        });
        assert!(!app.on_right_press(0, 0), "the occupied above the PASS keeps");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::DIRT,
            "the dirt cell stays dirt"
        );
    }


    /// THE TILL'S OWN MAP-MISS PASS — the TILLABLE lookup's other rows
    /// (javap: the clinit's Map<Block, Pair> the GRASS_BLOCK/DIRT rows
    /// name, every other block the PASS keeps): the stone the hoe
    /// never tills, the fist the arm never even arms (the held id the
    /// `_hoe` suffix the arm's own gate reads), the DIRT row the
    /// second tillable the farmland wears.
    #[test]
    fn the_till_the_map_miss_passes() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_held_updated(Some("item/wooden_hoe"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the stone the map miss the PASS keeps");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::STONE,
            "the stone stays stone"
        );
        // THE FIST: the held id the `_hoe` gate never arms — the grass
        // stays grass, the press never consumed (the item lanes the
        // fist's PASS keep)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::GRASS);
        app.offline_held_updated(Some("item/stick"));
        assert!(!app.on_right_press(0, 0), "the non-hoe the arm never arms");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::GRASS,
            "the grass stays grass"
        );
        // THE DIRT ROW: the second tillable the farmland wears
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::DIRT);
        app.offline_held_updated(Some("item/iron_hoe"));
        assert!(app.on_right_press(0, 0), "the dirt row the SUCCESS speaks");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface, c),
            crate::world::FARMLAND,
            "the dirt cell wears the farmland"
        );
    }


    /// THE SEEDS' OWN PLANT ARM — the straight-down aim the vanilla
    /// BlockItem.place's SUCCESS speaks (javap BlockItem.place +
    /// CropBlock.canSurvive over the real 26.2 jar): the held
    /// wheat_seeds the surface farmland looks up (the SUPPORTS_CROPS
    /// tag's sole farmland row the below-check keeps), the WHEAT lands
    /// IN the above cell (the context's own relative walk), the mesh
    /// re-speaks with the four planes' eight quads (the block/crop
    /// model's own row the mesher's wheat path meshes), the
    /// `item.crop.plant` voice the place-after-set order rides, the
    /// press consumed.
    #[test]
    fn the_seeds_plant_the_farmland_the_wheat_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the FARMLAND row keeps (the till arm's own
        // product the seeds never re-till — the plant arm's floor)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::FARMLAND);
        app.offline_held_updated(Some("item/wheat_seeds"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let quads_before = app.world_quads.as_ref().unwrap().clone();
        assert!(app.on_right_press(0, 0), "the plant consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::WHEAT,
            "the above cell wears the wheat crop"
        );
        // THE MESH RE-SPOKE: the AIR cell the crop's own quads wear —
        // the four zero-thickness planes' eight quads (2 faces x 4
        // planes, the audited block/crop model's own count) the vec
        // grows; the farmland's own top face the transparent-wheat walk
        // keeps (is_transparent the non-occluding verdict speaks)
        let after = app.world_quads.as_ref().unwrap();
        assert_eq!(
            after.len() - quads_before.len(),
            8,
            "the four planes' eight quads the crop cell wears"
        );
        // THE PLANT'S VOICE (the assert computes the expected from the
        // ACTUAL loaded state — the cache-absent boot the silent lane
        // keeps): the SoundType.CROP's own place event
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.crop.plant @")),
                "the CROP_PLANTED voice the place-after-set order rode"
            );
        }
    }


    /// THE PLANT'S OWN FLOOR GUARD — the SUPPORTS_CROPS refusal (javap
    /// VegetationBlock.canSurvive -> CropBlock.mayPlaceOn:
    /// `floor.is(SUPPORTS_CROPS)` — the tag json's sole farmland member
    /// the probe speaks): the held seeds the dirt floor looks up, the
    /// below cell the DIRT the tag never carries, the canSurvive false
    /// the FAIL -> the PASS fall-through keeps — no crop, no voice, the
    /// press never consumed.
    #[test]
    fn the_plant_the_farmland_below_refuses() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::DIRT); // the tillable, never the croppable
        app.offline_held_updated(Some("item/wheat_seeds"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the dirt floor the PASS keeps");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::AIR,
            "the above cell stays air"
        );
        if sounds_loaded() {
            assert!(
                !app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("item.crop.plant")),
                "the refused plant the voice never speaks"
            );
        }
    }


    /// THE PLANT'S OWN GATE — the seeds' exact id the arm's held walk
    /// reads (javap Items: the harvest `wheat` item is no BlockItem —
    /// the right-press the use lane's PASS keeps; the placeable lookup
    /// the crop kind never wears): the held wheat the farmland aims
    /// DOWN at, the arm never arms, the generic placement lane the
    /// WHEAT refusal keeps (the crop kind the seeds' own product the
    /// plant arm alone wears), the STONE floor the second aim the
    /// below-refusal keeps (the SUPPORTS_CROPS tag the stone never
    /// carried) — the farmland stays farmland, the above cell stays
    /// air, no voice.
    #[test]
    fn the_plant_the_seeds_gate_the_harvest_never_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::FARMLAND);
        // THE HARVEST ITEM: the arm's gate never arms, the placeable
        // refusal the crop kind keeps
        app.offline_held_updated(Some("item/wheat"));
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.on_right_press(0, 0),
            "the harvest item the PASS keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::AIR,
            "no crop the harvest item wears"
        );
        // THE STONE FLOOR: the below cell the tag never carries — the
        // same refusal the dirt floor spoke
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::STONE);
        app.offline_held_updated(Some("item/wheat_seeds"));
        assert!(
            !app.on_right_press(0, 0),
            "the stone floor the PASS keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::AIR,
            "the above cell stays air over stone"
        );
    }


    /// THE BONE MEAL'S OWN LADDER JUMP — the pure law the
    /// [`crate::world::bonemeal_grow`] speaks (javap
    /// CropBlock.getBonemealAgeIncrease + growCrops over the real 26.2
    /// jar): every immature stage the ONE-draw jump wears (the increase
    /// the uniform 2..=5 band speaks, the min(7) ceiling the top
    /// stages clamp), the same seed the same jump the replay wears.
    #[test]
    fn the_bone_meal_the_ladder_jumps() {
        for age in 0u8..7 {
            let seed = 0x1_0000 + age as u64;
            let mut rng = crate::drops::Lcg::new(seed);
            let (kind, increase) = crate::world::bonemeal_grow(
                crate::world::WHEAT + age as u16,
                &mut rng,
            )
            .unwrap_or_else(|| panic!("the immature stage {age} the grow answers"));
            assert!(
                (2..=5).contains(&increase),
                "the increase the 2..=5 band wears — got {increase}"
            );
            let expected = 7u8.min(age + increase);
            assert_eq!(
                kind,
                crate::world::WHEAT + expected as u16,
                "the climb the min-ceiling law wears at stage {age}"
            );
            // THE SAME SEED THE SAME JUMP: the replay the determinism keeps
            let mut replay = crate::drops::Lcg::new(seed);
            assert_eq!(
                crate::world::bonemeal_grow(crate::world::WHEAT + age as u16, &mut replay),
                Some((kind, increase)),
                "the same seed the same jump the replay wears at stage {age}"
            );
        }
    }


    /// THE BONE MEAL'S OWN DRAWS THE MIRROR PAIRS — the zero-draw laws
    /// the mature and the non-crop PASSes keep (javap: the
    /// isValidBonemealTarget false the stream never touches, the
    /// instanceof miss the growCrop's own keeps) and the success arm's
    /// ONE draw the replay pairs (the nextInt(4) modulus the increase
    /// wears — the post-jump streams the paired draws never split).
    #[test]
    fn the_bone_meal_the_draw_counts_keep() {
        // THE MATURE PASS + THE NON-CROP MISS: the zero-draw law the
        // paired streams keep
        let mut a = crate::drops::Lcg::new(7);
        let mut b = crate::drops::Lcg::new(7);
        assert_eq!(
            crate::world::bonemeal_grow(crate::world::WHEAT_STAGE7, &mut a),
            None,
            "the mature the isValidBonemealTarget PASS keeps"
        );
        assert_eq!(
            crate::world::bonemeal_grow(crate::world::DIRT, &mut b),
            None,
            "the non-crop the growCrop miss keeps"
        );
        // the streams the PASSes never touched — the paired draws still speak
        assert_eq!(
            a.next_int(1000),
            b.next_int(1000),
            "the zero draws the paired streams keep"
        );
        // THE SUCCESS ARM: the ONE draw the mirror pairs (the same seed
        // the same nextInt(4) the increase wears, the post-jump pairing
        // the second draw the law never spends)
        let mut c = crate::drops::Lcg::new(31);
        let mut d = crate::drops::Lcg::new(31);
        let (_, increase) =
            crate::world::bonemeal_grow(crate::world::WHEAT_STAGE2, &mut c).unwrap();
        let mirror = (2 + d.next_int(4)) as u8;
        assert_eq!(
            increase, mirror,
            "the increase the ONE nextInt(4) draw wears"
        );
        assert_eq!(
            c.next_int(97),
            d.next_int(97),
            "the post-jump streams the ONE-draw law pairs"
        );
    }


    /// THE BONE MEAL'S OWN PRESS — the client arm the vanilla useOn's
    /// SUCCESS speaks (javap BoneMealItem.useOn + growCrop): the held
    /// bone_meal the immature crop looks up (the straight-down aim the
    /// crop cell the clip targets), the kind the ladder's legal band
    /// climbs (the stage-2 start + the 2..=5 jump the
    /// WHEAT_STAGE4..=WHEAT_STAGE7 band wears), the stack the shrink(1)
    /// rides, the mesh re-spoke; the mature crop the press never
    /// consumes (the isValidBonemealTarget the PASS keeps — the
    /// fall-through the placement's bone_meal refusal the press never
    /// consumes, the stack the zero draws never touch).
    #[test]
    fn the_bone_meal_the_press_the_crop_advances() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::FARMLAND);
            w.set(c, surface + 1, c, crate::world::WHEAT + 2); // the stage-2 crop
        }
        // THE STACK THE INVENTORY'S OWN SELECTION SPEAKS (the held id
        // the stack's lane answers before the `--held-item` stand-in)
        let bone_meal = mc_registry::item_id_by_name("bone_meal")
            .expect("the bone_meal item row the registry carries");
        app.inventory.items[app.selected_hotbar] = (bone_meal, 5);
        // THE AIM: the eye above the crop, the straight-down walk the
        // crop cell the clip targets (the stalk the ray hits first)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 3.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let quads_before = app.world_quads.as_ref().unwrap().clone();
        assert!(app.on_right_press(0, 0), "the grow consumed the press");
        let grown = app
            .offline_world
            .as_ref()
            .unwrap()
            .get(c, surface + 1, c);
        assert!(
            (crate::world::WHEAT_STAGE4..=crate::world::WHEAT_STAGE7).contains(&grown),
            "the stage-2 crop the legal band climbed — got {grown}"
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar].1,
            4,
            "the stack the shrink(1) rode"
        );
        assert_ne!(
            app.world_quads.as_ref().unwrap(),
            &quads_before,
            "the mesh re-spoke with the advanced crop"
        );
        // THE MATURE REFUSAL: the ceiling the PASS keeps — the press
        // never consumed, the mature cell never moves, the stack the
        // zero draws never touch
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface + 2, c, crate::world::WHEAT_STAGE7);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.on_right_press(0, 0),
            "the mature crop the isValidBonemealTarget PASS keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 2, c),
            crate::world::WHEAT_STAGE7,
            "the mature cell the PASS never moves"
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar].1,
            4,
            "the refused grow the stack never shrinks"
        );
    }


    /// THE BONE MEAL'S OWN GRASS WALK — the client arm the spread
    /// row's press speaks (javap GrassBlock the BonemealableBlock
    /// trio the growCrop flow's second arm wears): the held bone_meal
    /// the air-topped grass looks up, the 128-row walk the origin
    /// cell and the neighbours plant, the stack the shrink(1) rides,
    /// the mesh re-spokes; the tuft-topped grass the
    /// isValidBonemealTarget PASS keeps (the above-air arm the
    /// vanilla refuses — the press never consumed, the stack the zero
    /// draws never touch).
    #[test]
    fn the_bone_meal_the_grass_walk_the_plants_speak() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        // the hit cell the GRASS row keeps (the air-topped surface the
        // isValidBonemealTarget's own pair demands)
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::GRASS);
            w.set(c, surface + 1, c, crate::world::AIR);
            w.set(c, surface + 2, c, crate::world::AIR);
        }
        // THE STACK THE INVENTORY'S OWN SELECTION SPEAKS
        let bone_meal = mc_registry::item_id_by_name("bone_meal")
            .expect("the bone_meal item row the registry carries");
        app.inventory.items[app.selected_hotbar] = (bone_meal, 5);
        // THE AIM: the eye above the grass, the straight-down walk the
        // grass cell the clip targets (the two air cells the ray rides)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 3.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let quads_before = app.world_quads.as_ref().unwrap().clone();
        assert!(
            app.on_right_press(0, 0),
            "the grass walk consumed the press"
        );
        let origin = app
            .offline_world
            .as_ref()
            .unwrap()
            .get(c, surface + 1, c);
        assert!(
            origin == crate::world::SHORT_GRASS
                || origin == crate::world::DANDELION
                || origin == crate::world::POPPY,
            "the origin cell the walk's first rows plant — got {origin}"
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar].1,
            4,
            "the stack the shrink(1) rode"
        );
        assert_ne!(
            app.world_quads.as_ref().unwrap(),
            &quads_before,
            "the mesh re-spoke with the planted tufts"
        );
        // THE TUFT-TOPPED REFUSAL: the above-air arm the PASS keeps —
        // the press never consumed, the tuft the walk never moves, the
        // stack the zero draws never touch
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface + 1, c, crate::world::SHORT_GRASS);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 3.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(
            !app.on_right_press(0, 0),
            "the tuft-topped grass the isValidBonemealTarget PASS keeps"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface + 1, c),
            crate::world::SHORT_GRASS,
            "the tuft the PASS never moves"
        );
        assert_eq!(
            app.inventory.items[app.selected_hotbar].1,
            4,
            "the refused walk the stack never shrinks"
        );
    }


    #[test]
    fn the_bone_meal_the_press_the_growth_burst_the_in_block_law_speaks() {
        // THE GROWTH EFFECT'S OWN OFFLINE PREDICTION (javap the 26.2
        // chain: useOn's `levelEvent(1505, pos, 15)` → the client's
        // LevelEventHandler case → `BoneMealItem.addGrowthParticles`):
        // the crop press the GROWER lane answers — the in-block burst,
        // the count the data word verbatim (15), the positions the
        // stage-2 shape's own max-Y band keeps (`(age*2+2)/16 = 6/16`),
        // the useOn order the sound the shrink's slot BEFORE keeps.
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(c, surface, c, crate::world::FARMLAND);
            w.set(c, surface + 1, c, crate::world::WHEAT + 2); // the stage-2 crop
        }
        let bone_meal = mc_registry::item_id_by_name("bone_meal")
            .expect("the bone_meal item row the registry carries");
        app.inventory.items[app.selected_hotbar] = (bone_meal, 5);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 3.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.particles.is_empty(), "no burst before the press");
        assert!(app.on_right_press(0, 0), "the grow consumed the press");
        // THE IN-BLOCK BURST (the grower's own walk): 15 sparkles INSIDE
        // the crop block, the y band the shape's max-Y keeps — the
        // POST-GROWTH stage the client's own arrival order reads (the
        // setBlock's update the levelEvent FOLLOWS on the wire — the
        // stage-2 press the stage-3 shape answers, `(3*2+2)/16 = 0.5`)
        assert_eq!(
            app.particles.len(),
            15,
            "the grower's data-verbatim count"
        );
        let grown = app
            .offline_world
            .as_ref()
            .unwrap()
            .get(c, surface + 1, c);
        let grown_age = crate::world::wheat_age(grown)
            .expect("the advanced crop the setBlock's own word speaks");
        let max_y = (grown_age as f64 * 2.0 + 2.0) / 16.0;
        for p in &app.particles {
            assert_eq!(p.kind, crate::particles::ParticleKind::HappyVillager);
            assert!((c as f64..(c + 1) as f64).contains(&p.x), "x in-block: {}", p.x);
            assert!((c as f64..(c + 1) as f64).contains(&p.z), "z in-block: {}", p.z);
            assert!(
                ((surface + 1) as f64..(surface + 1) as f64 + max_y).contains(&p.y),
                "y under the post-growth shape: {}",
                p.y
            );
        }
        // THE SOUND (the playLocalSound the case speaks unconditionally
        // — the 26.2 handler's own BONE_MEAL_USE event, volume/pitch 1.0)
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.bone_meal.use @")),
            "the bone-meal voice the burst's own case plays"
        );
        // the stack the shrink rode (the event's slot the consume
        // precedes — the useOn offsets the doc keeps)
        assert_eq!(app.inventory.items[app.selected_hotbar].1, 4);
    }


    #[test]
    fn the_bone_meal_the_grass_press_the_wide_burst_the_gate_speaks() {
        // THE SPREADER'S OWN BURST (the grass press's wide lane —
        // `spawnParticles(pos.above(), 15 * 3, 3.0, 1.0, false)`): the
        // 45 attempts the imul keeps, the band the half-fold opens, the
        // carved floor the ground gate keeps alive (every attempt's
        // below block the GRASS answer speaks).
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        {
            let w = app.offline_world.as_mut().unwrap();
            // the arena the gate reads: the solid floor plane, the air
            // above (the burst's own 6-wide band the carve covers)
            for x in (c - 6)..(c + 6) {
                for z in (c - 6)..(c + 6) {
                    w.set(x, surface, z, crate::world::GRASS);
                    w.set(x, surface + 1, z, crate::world::AIR);
                    w.set(x, surface + 2, z, crate::world::AIR);
                }
            }
        }
        let bone_meal = mc_registry::item_id_by_name("bone_meal")
            .expect("the bone_meal item row the registry carries");
        app.inventory.items[app.selected_hotbar] = (bone_meal, 5);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, surface as f32 + 3.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the spread consumed the press");
        // THE WIDE BURST: 45 sparkles the data*3 law speaks, the band
        // [c-2.5, c+3.5) the 0.5-3.0 half-fold opens, the y band the
        // anchor's own +1 the pos.above() arm keeps
        assert_eq!(
            app.particles.len(),
            45,
            "the spreader's data*3 count the solid floor keeps whole"
        );
        for p in &app.particles {
            assert!(
                ((c as f64 - 2.5)..(c as f64 + 3.5)).contains(&p.x),
                "x inside the spread band: {}",
                p.x
            );
            assert!(
                ((c as f64 - 2.5)..(c as f64 + 3.5)).contains(&p.z),
                "z inside the spread band: {}",
                p.z
            );
            assert!(
                ((surface + 1) as f64..(surface + 2) as f64).contains(&p.y),
                "y above the grass: {}",
                p.y
            );
        }
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.bone_meal.use @")),
            "the voice the wide lane's case plays"
        );
    }


    #[test]
    fn the_growth_event_the_wire_lane_the_burst_the_double_refuses() {
        // THE WIRE LANE (the Level Event packet's own echo — the
        // networked world the server's broadcast answers): the arms
        // the live lane drains burst at their own anchors (the crop's
        // in-block count, the grass's wide one), the press lane the
        // networked world keeps SILENT (the echo the packet alone
        // speaks — the same word never bursts twice), and a foreign
        // anchor the default arm keeps (no burst, the sound still
        // plays — the playLocalSound the case speaks unconditionally).
        let mut app = App::new(2);
        {
            let mut w = crate::world::World::new(2, 11);
            w.set(10, 25, 10, crate::world::FARMLAND);
            w.set(10, 26, 10, crate::world::WHEAT + 2); // the stage-2 crop
            app.net_world = Some(w);
        }
        app.world_is_network = true;
        // THE WIRE ECHO (the crop word): the in-block burst the anchor's
        // own cell speaks — the net world the reads ride
        app.live_growth_events_updated(vec![crate::world_view::WireGrowthEvent {
            x: 10,
            y: 26,
            z: 10,
            data: 15,
        }]);
        assert_eq!(
            app.particles.len(),
            15,
            "the wire echo's in-block burst the crop anchor speaks"
        );
        // the grass word: the wide burst
        app.net_world = {
            let mut w = crate::world::World::new(2, 11);
            let surface = 26;
            for x in 4..17 {
                for z in 4..17 {
                    w.set(x, surface, z, crate::world::GRASS);
                    w.set(x, surface + 1, z, crate::world::AIR);
                    w.set(x, surface + 2, z, crate::world::AIR);
                }
            }
            Some(w)
        };
        app.live_growth_events_updated(vec![crate::world_view::WireGrowthEvent {
            x: 10,
            y: 26,
            z: 10,
            data: 15,
        }]);
        assert_eq!(
            app.particles.len(),
            60,
            "the wire echo's wide burst rides after the crop word's own 15"
        );
        // THE FOREIGN ANCHOR (the stone cell the switch's default keeps):
        // no burst — the sound still lands (the earlier bursts the
        // clear sweeps before the fresh word lands)
        app.particles.clear();
        let before = app.sounds.events.len();
        app.net_world = {
            let mut w = crate::world::World::new(2, 11);
            w.set(5, 5, 5, crate::world::STONE);
            Some(w)
        };
        app.live_growth_events_updated(vec![crate::world_view::WireGrowthEvent {
            x: 5,
            y: 5,
            z: 5,
            data: 15,
        }]);
        assert!(
            app.particles.is_empty(),
            "the foreign anchor no burst speaks"
        );
        assert!(
            app.sounds.events.len() > before,
            "the voice the unconditional case still plays"
        );
    }


    #[test]
    fn the_wire_in_block_word_the_shape_fold_the_silence_wears() {
        // THE IN-BLOCK GROWTH EFFECT'S OWN WIRE LANE (javap the cases
        // 2546/2561 — the bee's growth word and the turtle egg's
        // placement word the server's own pollinator and egg-lay
        // broadcast): the anchor cell's own shape the spawnParticleInBlock
        // walk reads (the wheat ladder the stage's own Y-max folds, the
        // air gate the 1.0 keeps), the count the data word carries (the
        // callers' own bipush 15), the whole burst the net world's own
        // reads place — and NO sound the cases speak (the silence the
        // growth lane's unconditional voice never shares).
        let mut app = App::new(2);
        {
            let mut w = crate::world::World::new(2, 11);
            w.set(10, 25, 10, crate::world::FARMLAND);
            w.set(10, 26, 10, crate::world::WHEAT + 3); // the stage-3 crop
            app.net_world = Some(w);
        }
        app.world_is_network = true;
        let before = app.sounds.events.len();
        // THE CROP WORD (the pollinator's own broadcast — the count the
        // bipush 15 the pollen's burst wears)
        app.live_in_block_growth_events_updated(vec![crate::world_view::WireInBlockGrowthEvent {
            x: 10,
            y: 26,
            z: 10,
            data: 15,
        }]);
        assert_eq!(
            app.particles.len(),
            15,
            "the in-block burst the stage-3 stalk's own shape speaks"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::HappyVillager),
                "the glint sprite the HAPPY_VILLAGER type answers"
            );
            // the stage-3 ladder the (3*2+2)/16 = 0.5 band keeps — the
            // burst lives INSIDE the stalk's own half-height
            assert!(
                p.y >= 26.0 && p.y < 26.5,
                "y inside the stalk's own band: {}",
                p.y
            );
            assert!(
                (10.0..11.0).contains(&p.x) && (10.0..11.0).contains(&p.z),
                "the in-block x/z band the 0.5 pair folds"
            );
        }
        // THE SILENCE (no sound the cases speak — the growth lane's
        // unconditional voice the in-block lane never shares)
        assert_eq!(
            app.sounds.events.len(),
            before,
            "the in-block lane's own silence the jar keeps"
        );
        // THE AIR WORD (the egg-lay's own cell the isAir gate keeps):
        // the dy 1.0 the full band speaks — the same count the walk
        // still bursts (the turtle egg's placement the sand cell's air
        // neighbours the same answer keeps)
        app.particles.clear();
        app.net_world = {
            let mut w = crate::world::World::new(2, 11);
            w.set(6, 5, 6, crate::world::SAND); // the cell below the burst
            Some(w)
        };
        app.live_in_block_growth_events_updated(vec![crate::world_view::WireInBlockGrowthEvent {
            x: 6,
            y: 6,
            z: 6,
            data: 15,
        }]);
        assert_eq!(app.particles.len(), 15, "the air cell the 1.0 band keeps");
        for p in &app.particles {
            assert!(
                (6.0..7.0).contains(&p.y),
                "y inside the air cell's own band: {}",
                p.y
            );
        }
        // THE ZERO COUNT (the data word the 0 the max(0) fold keeps):
        // the honest empty burst the negative words the same fold wears
        app.particles.clear();
        app.live_in_block_growth_events_updated(vec![crate::world_view::WireInBlockGrowthEvent {
            x: 6,
            y: 6,
            z: 6,
            data: 0,
        }]);
        assert!(app.particles.is_empty(), "the zero count no burst speaks");
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.live_in_block_growth_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    /// THE BLOCK-BREAK EFFECT'S OWN WIRE SERVE (javap
    /// `LevelEventHandler` case 2001 over the 26.2 jar — the sound the
    /// break lane's own `(v+1)/2, p*0.8` fold plays FIRST, the
    /// `addDestroyBlockEffect` burst the 4x4x4 grid speaks): the STONE
    /// word the voice + the 64-debris burst both wear, the AIR word the
    /// resolved-None arm BOTH lanes refuse (the isAir gate + the tile's
    /// own verdict), the UNBANKED anvil the burst's own absence keeps
    /// (the kind the palette never banks — the tile the atlas never
    /// finds — while the sound the honest ride keeps), the empty
    /// mailbox the no-op keeps.
    #[test]
    fn the_wire_break_event_the_voice_the_burst_wear() {
        let mut app = App::new(2);
        app.enter_world();
        // THE STONE WORD (the state walk's own answers the arm carries)
        app.live_break_events_updated(vec![crate::world_view::WireBreakEvent {
            x: 10,
            y: 20,
            z: 10,
            sound_name: Some("stone"),
            burst_kind: Some(crate::world::STONE),
        }]);
        assert_eq!(
            app.particles.len(),
            64,
            "the 4x4x4 grid the full cube speaks"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.stone.break @")),
                "the break voice the case's own head rides"
            );
        }
        // THE AIR WORD: the resolved-None arm BOTH lanes refuse — no
        // fresh voice, no fresh debris (the earlier burst the clear
        // sweeps before the silent word lands)
        app.particles.clear();
        let before = app.sounds.events.len();
        app.live_break_events_updated(vec![crate::world_view::WireBreakEvent {
            x: 3,
            y: 3,
            z: 3,
            sound_name: None,
            burst_kind: Some(crate::world::AIR),
        }]);
        assert!(
            app.particles.is_empty(),
            "the air kind the tile's own verdict refuses"
        );
        assert_eq!(app.sounds.events.len(), before, "the None name no voice");
        // THE UNBANKED KIND (the anvil the palette never banks): the
        // burst the honest absence keeps, the sound the ride speaks
        app.live_break_events_updated(vec![crate::world_view::WireBreakEvent {
            x: 5,
            y: 5,
            z: 5,
            sound_name: Some("anvil"),
            burst_kind: None,
        }]);
        assert!(
            app.particles.is_empty(),
            "the unbanked kind the burst never finds"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.anvil.break @")),
                "the unbanked kind's voice the sound table's own row keeps"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let before_all = app.sounds.events.len();
        app.live_break_events_updated(vec![]);
        assert!(app.particles.is_empty());
        assert_eq!(app.sounds.events.len(), before_all);
    }


    #[test]
    fn the_wire_smoke_word_the_ten_puff_walk_wears() {
        // THE SHOOT-SMOKE EFFECT'S OWN WIRE LANE (javap case 2000 →
        // `shootParticles` over the 26.2 jar): the ten puffs the
        // `bipush 10` loop fires, the SMOKE type the provider answers,
        // the facing fold the position anchors speak, the velocity the
        // `dir * d10 + gaussian * 0.01` fold gives, the ctor chain the
        // BaseAshSmokeParticle laws wear — and NO sound the case
        // speaks (the silence the smoke lane's own stream keeps).
        let mut app = App::new(2);
        app.enter_world();
        // THE EAST WORD (the dispenser's own facing — data 5 the
        // `get3DDataValue` broadcast carries)
        app.live_shoot_smoke_events_updated(vec![crate::world_view::WireShootSmokeEvent {
            x: 10,
            y: 20,
            z: 10,
            data: 5,
        }]);
        assert_eq!(app.particles.len(), 10, "the bipush 10 the loop keeps");
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Smoke),
                "the SMOKE type the provider answers"
            );
        }
        for p in &app.particles {
            // the EAST anchor fold (the bytecode's own operand order —
            // the jitter the zero Z step cancels: the px the anchor
            // EXACTLY keeps, the py the zero Y step centers, the pz
            // the X-step jitter spreads ±0.25)
            assert!(
                (p.x - (10.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
                "the east anchor the facing fold places"
            );
            assert!(
                (p.y - (20.0 + 0.0 * 0.6 + 0.5)).abs() < 1e-9,
                "the center the zero Y step keeps"
            );
            assert!(
                p.z > 10.0 + 0.5 - 0.25 - 1e-9 && p.z < 10.0 + 0.5 + 0.25,
                "the crosswise jitter the X step scales"
            );
            // the velocity folds: the eastward shoot the dir*d10 term
            // gives (d10 ∈ [0.01, 0.21] the +0.01 floor speaks), the
            // side axes the gaussian whisper alone carries, the smoke
            // rise the ash ctor's −0.1 gravity pulls
            assert!(p.vx > 0.0, "the eastward shoot the dir*d10 fold gives");
            assert!(
                p.vy.abs() < 0.5 && p.vz.abs() < 0.5,
                "the side axes the gaussian whisper carries"
            );
            assert!(
                (p.gravity + 0.1).abs() < 1e-9,
                "the −0.1 gravity the ash ctor passes"
            );
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction the ctor sets");
            // the grey tint the r*0.3 draw (the shared grey the
            // smoke_puff fold keeps — the [0, 0.3) band the draw gives)
            assert_eq!(p.col[0], p.col[1], "the shared grey the tint fold keeps");
            assert_eq!(p.col[1], p.col[2], "the shared grey the tint fold keeps");
            assert!(p.col[0] < 0.3, "the r*0.3 scale the draw speaks");
        }
        // THE DOWN WORD (data 0): the vy the dir*d10 term PULLS (the
        // downward shoot the facing's own step gives), the px/pz the
        // zero X/Z steps center EXACTLY (the jitter the zero steps
        // cancel), the py the dy*0.6 anchor below the cell keeps with
        // the Y-step's own jitter spread (±0.25 the band speaks) and
        // the dy*0.01 step the fold's own tail adds
        app.particles.clear();
        app.play_shoot_smoke(4, 5, 6, 0);
        assert_eq!(app.particles.len(), 10, "the down word the same loop fires");
        for p in &app.particles {
            assert!(
                (p.x - (4.0 + 0.0 * 0.6 + 0.5)).abs() < 1e-9,
                "the center the zero X step keeps"
            );
            assert!(
                (p.z - (6.0 + 0.0 * 0.6 + 0.5)).abs() < 1e-9,
                "the center the zero Z step keeps"
            );
            assert!(
                p.y > 5.0 - 0.6 + 0.5 - 0.01 - 0.25 && p.y < 5.0 - 0.6 + 0.5 - 0.01 + 0.25,
                "the below anchor the DOWN fold places, the Y jitter spread rides"
            );
            assert!(p.vy < 0.0, "the downward shoot the dir*d10 fold gives");
        }
        // THE NEGATIVE WORD (data -5): the `abs(irem(data, 6))` fold
        // the from3DDataValue walk answers — the same EAST facing the
        // fold speaks (irem -5 % 6 = -5, abs = 5)
        app.particles.clear();
        app.play_shoot_smoke(0, 0, 0, -5);
        assert_eq!(app.particles.len(), 10);
        assert!(
            (app.particles[0].x - (0.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
            "the same east anchor the negative word folds to"
        );
        // THE OUT-OF-RANGE WORD (data 11): the fold answers 11 % 6 = 5
        // — the same EAST (the vanilla's own modulo)
        app.particles.clear();
        app.play_shoot_smoke(3, 4, 5, 11);
        assert_eq!(app.particles.len(), 10);
        assert!(
            (app.particles[0].x - (3.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
            "the same east anchor the out-of-range word folds to"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_shoot_smoke_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_white_smoke_word_the_ten_puff_walk_wears() {
        // THE WHITE-SMOKE EFFECT'S OWN WIRE LANE (javap case 2010 →
        // offset 919 → the SAME `shootParticles` walk the 2000 word
        // wears): the ten puffs the `bipush 10` loop fires, the
        // WHITE_SMOKE type the provider answers — the ONE delta the
        // `WhiteSmokeParticle` ctor's tail speaks: the lavender-white
        // constants the r/g/b putfield trio overrides (the cp
        // #12/#17/#21 decode), the base's own tint draw the stream
        // still spends, the same physics the ash ctor's chain wears —
        // and NO sound the case speaks (the silence the smoke lane's
        // own stream keeps).
        let mut app = App::new(2);
        app.enter_world();
        // THE EAST WORD (the crafter's own facing — data 5 the
        // `get3DDataValue` broadcast carries)
        app.live_white_smoke_events_updated(vec![crate::world_view::WireWhiteSmokeEvent {
            x: 10,
            y: 20,
            z: 10,
            data: 5,
        }]);
        assert_eq!(app.particles.len(), 10, "the bipush 10 the loop keeps");
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Smoke),
                "the smoke render path the white kind shares"
            );
            // THE ONE DELTA (the ctor's tint override — the constants
            // the cp trio decodes, the grey the base's own draw the
            // r/g/b putfield trio kills)
            assert!((p.col[0] - 0.7294118).abs() < 1e-9, "the r the cp #12 keeps");
            assert!((p.col[1] - 0.69411766).abs() < 1e-9, "the g the cp #17 keeps");
            assert!((p.col[2] - 0.7607843).abs() < 1e-9, "the b the cp #21 keeps");
            // the same anchor fold the east word's smoke walk speaks
            assert!(
                (p.x - (10.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
                "the east anchor the facing fold places"
            );
            assert!(
                (p.y - (20.0 + 0.5)).abs() < 1e-9,
                "the center the zero Y step keeps"
            );
            assert!(
                p.z > 10.0 + 0.5 - 0.25 - 1e-9 && p.z < 10.0 + 0.5 + 0.25,
                "the crosswise jitter the X step scales"
            );
            assert!(p.vx > 0.0, "the eastward shoot the dir*d10 fold gives");
            assert!(
                (p.gravity + 0.1).abs() < 1e-9,
                "the −0.1 gravity the ash ctor passes"
            );
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction the ctor sets");
            // the lifetime band the base-8 fold gives (8..40 ticks the
            // 8/(r*0.8+0.2) law speaks — the SAME band the smoke kind
            // wears, the two ctors' identical bipush 8 the jar pins)
            assert!(p.lifetime >= 8 && p.lifetime <= 40, "the base-8 band the fold speaks");
        }
        // THE DRAW-CHAIN LAW (the white chain the smoke chain's own
        // count speaks — the two walks the IDENTICAL fresh streams
        // replay, the post-walk states the same draw count keeps: the
        // dead tint draw the white's override kills the exact position
        // the smoke's own grey draw spends, the six f32s + the
        // gaussians the shared chain per puff spends)
        let mut smoke_stream = crate::drops::Lcg::new(31);
        let mut white_stream = crate::drops::Lcg::new(31);
        app.particles.clear();
        let saved_rng = std::mem::replace(&mut app.level_ambient_rng, smoke_stream);
        app.play_shoot_smoke(0, 0, 0, 5);
        smoke_stream = app.level_ambient_rng;
        app.level_ambient_rng = white_stream;
        app.play_white_smoke(0, 0, 0, 5);
        white_stream = app.level_ambient_rng;
        app.level_ambient_rng = saved_rng;
        assert_eq!(
            smoke_stream.next(),
            white_stream.next(),
            "the same draw count the two chains keep"
        );
        // THE NEGATIVE WORD (data -5): the `abs(irem(data, 6))` fold
        // the from3DDataValue walk answers — the same EAST facing the
        // fold speaks (irem -5 % 6 = -5, abs = 5)
        app.particles.clear();
        app.play_white_smoke(0, 0, 0, -5);
        assert_eq!(app.particles.len(), 10);
        assert!(
            (app.particles[0].x - (0.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
            "the same east anchor the negative word folds to"
        );
        // THE OUT-OF-RANGE WORD (data 11): the fold answers 11 % 6 = 5
        // — the same EAST (the vanilla's own modulo)
        app.particles.clear();
        app.play_white_smoke(3, 4, 5, 11);
        assert_eq!(app.particles.len(), 10);
        assert!(
            (app.particles[0].x - (3.0 + 1.0 * 0.6 + 0.5 + 1.0 * 0.01)).abs() < 1e-9,
            "the same east anchor the out-of-range word folds to"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_white_smoke_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_dragon_word_the_pool_walk_the_gate_wears() {
        // THE DRAGON-FIREBALL SPLASH'S OWN WIRE LANE (javap case 2006
        // → offset 3893 over the 26.2 jar): the 200-puff
        // `PowerParticleOption DRAGON_BREATH` pool the `sipush 200`
        // loop fires at the RAW corner folds the case's own args
        // place, the setPower fold the provider's tail applies, the
        // explode voice the data==1 gate arms at the BLOCK CENTER —
        // the silent word the particles-only arm keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE NON-SILENT WORD (data 1 — the explode voice's gate
        // speaks): the pool rides the raw corner reads
        app.live_dragon_fireball_splash_events_updated(vec![
            crate::world_view::WireDragonFireballSplashEvent {
                x: 10,
                y: 20,
                z: 10,
                data: 1,
            },
        ]);
        assert_eq!(
            app.particles.len(),
            200,
            "the sipush 200 the pool loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::DragonBreath),
                "the DRAGON_BREATH type the provider answers"
            );
            // THE RAW CORNER FOLDS (the getX() + d8 * 0.1 the case's
            // own args speak — |d8| <= 4.0 the ±0.4 band keeps, the
            // +0.3 lift the getY() fold gives, NO bottom-center the
            // 2002 word adds)
            assert!(
                p.x >= 10.0 - 0.4 - 1e-9 && p.x <= 10.0 + 0.4 + 1e-9,
                "the x the d8*0.1 band keeps: {}",
                p.x
            );
            assert!(
                (p.y - 20.3).abs() < 1e-9,
                "the +0.3 the raw corner keeps: {}",
                p.y
            );
            assert!(
                p.z >= 10.0 - 0.4 - 1e-9 && p.z <= 10.0 + 0.4 + 1e-9,
                "the z the d12*0.1 band keeps: {}",
                p.z
            );
            // THE SET-POWER FOLD (the x/z the plain multiply, the y
            // the `(v - 0.1) * p + 0.1` the base's own 0.1 floor
            // keeps — the band the extremes fold: d10 ∈ [0.01, 0.51),
            // p ∈ [0, 4) → vy ∈ (-0.26, 1.74), the NEGATIVE arm the
            // small d10 × the big power answers)
            assert!(
                p.vy > -0.26 && p.vy < 1.74,
                "the setPower fold's own band: {}",
                p.vy
            );
            assert!(p.vx.abs() < 16.0 + 1e-9, "the power² band the x keeps: {}", p.vx);
            assert!(p.vz.abs() < 16.0 + 1e-9, "the power² band the z keeps: {}", p.vz);
            // THE CTOR LAWS (the no-gravity field the override's own
            // tick keeps, the 0.96 friction, the lifetime the
            // f64-domain fold's 20..100 band gives, the purple the
            // ranges carry — the green the (0,0) range keeps)
            assert!((p.gravity).abs() < 1e-9, "the no-gravity law");
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
            assert!(p.lifetime >= 20 && p.lifetime <= 100, "the band: {}", p.lifetime);
            assert_eq!(p.col[1], 0.0, "the green the zero-width range keeps");
        }
        // THE GATE'S VOICE (the tail's playLocalSound — the pitch the
        // 0.9..1.0 band the draw gives, the BLOCK CENTER the
        // playLocalSound(BlockPos, ...) walk resolves)
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.dragon_fireball.explode @"))
                .expect("the explode voice the gate speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                pitch >= 0.9 && pitch < 1.0,
                "the pitch band the nextFloat * 0.1 + 0.9 fold gives: {pitch}"
            );
        }
        // THE SILENT WORD (data -1 — the particles-only arm keeps:
        // the SAME pool the gate's either arm fires, the voice the
        // gate's own branch never arms)
        app.particles.clear();
        let voices_before = if sounds_loaded() {
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.dragon_fireball.explode @"))
                .count()
        } else {
            0
        };
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(77));
        app.live_dragon_fireball_splash_events_updated(vec![
            crate::world_view::WireDragonFireballSplashEvent {
                x: 4,
                y: 5,
                z: 6,
                data: -1,
            },
        ]);
        assert_eq!(
            app.particles.len(),
            200,
            "the silent word the same pool fires"
        );
        // the silence the gate's own branch keeps (no voice the arm
        // never reaches — the stream the 200 x 9-draw chains alone
        // spend)
        let silent_stream = std::mem::replace(&mut app.level_ambient_rng, saved);
        if sounds_loaded() {
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.dragon_fireball.explode @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the silent word the voice never arms"
            );
        }
        let _ = silent_stream;
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_dragon_fireball_splash_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_water_word_the_steam_walk_the_lift_wears() {
        // THE WATER-EVAPORATING'S OWN WIRE LANE (javap case 2009 →
        // offset 4070 over the 26.2 jar): the eight CLOUD puffs the
        // `bipush 8` loop fires at the raw corner folds the case's
        // own args place (the x/z the nextDouble bands carry, the
        // +1.2 lift the cp Double decode speaks, the zero velocity
        // trio the `dconst_0` answers), the near-white tint the
        // ctor's ONE draw carries — NO SOUND the case speaks (the
        // FIRE_EXTINGUISH voice the server's own playSound carries).
        let mut app = App::new(2);
        app.enter_world();
        app.live_water_evaporating_events_updated(vec![
            crate::world_view::WireWaterEvaporatingEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        assert_eq!(
            app.particles.len(),
            8,
            "the bipush 8 the steam loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Cloud),
                "the CLOUD type the provider answers"
            );
            // THE RAW CORNER FOLDS (the getX() + nextDouble() the
            // case's own args speak — the [10, 11) band the x/z keep,
            // the +1.2 lift the y's constant rides)
            assert!(
                p.x >= 10.0 && p.x < 11.0,
                "the x the nextDouble band keeps: {}",
                p.x
            );
            assert!(
                (p.y - 21.2).abs() < 1e-9,
                "the +1.2 lift the cp Double keeps: {}",
                p.y
            );
            assert!(
                p.z >= 10.0 && p.z < 11.0,
                "the z the nextDouble band keeps: {}",
                p.z
            );
            // THE ZERO VELOCITY TRIO (the dconst_0 the case passes)
            assert_eq!(p.vx, 0.0, "the zero vx");
            assert_eq!(p.vy, 0.0, "the zero vy");
            assert_eq!(p.vz, 0.0, "the zero vz");
            // THE CTOR LAWS (the 0.96 friction, the no-gravity field,
            // the 17..65 band the f64-domain fold gives, the shared
            // f16 the r/g/b trio keeps)
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
            assert!((p.gravity).abs() < 1e-9, "the no-gravity law");
            assert!(p.lifetime >= 17 && p.lifetime <= 65, "the band: {}", p.lifetime);
            assert_eq!(p.col[0], p.col[1], "the shared f16 the r/g keeps");
            assert_eq!(p.col[1], p.col[2], "the shared f16 the g/b keeps");
        }
        // THE STEAM'S OWN TICK (the lift the feet-level player never
        // arms): the body ABOVE the puffs the y > fold keeps silent —
        // the equality the strict > refuses, the pose the lift never
        // moves
        app.body = Some(crate::body::PlayerBody::new(10.0, 23.0, 10.0, 0.0, 0.0));
        app.update();
        for p in &app.particles {
            assert!(
                (p.y - 21.2).abs() < 1e-9,
                "the above-feet player the lift never arms: {}",
                p.y
            );
        }
        // THE PLAYER BELOW (the body the feet-level read lowers — the
        // 2.0 range the within-2 arms, the 0.2 blend the pose eases)
        app.body = Some(crate::body::PlayerBody::new(10.0, 20.0, 10.0, 0.0, 0.0));
        app.update();
        for p in &app.particles {
            assert!(
                p.y < 21.2,
                "the below-feet player the lift arms: {}",
                p.y
            );
        }
        // THE SILENCE (the FIRE_EXTINGUISH voice the server's own
        // playSound carries, NOT the case's — no sound event the
        // wire walk never speaks)
        if sounds_loaded() {
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.contains("fire.extinguish")),
                "the case's own silence the voice never shares"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_water_evaporating_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_smoke_words_the_voice_walk_the_puffs_wear() {
        // THE PITCHED-SMOKE BAND'S OWN WIRE LANE (javap cases
        // 1501/1502 → offsets 3522/3628 over the 26.2 jar): the
        // pitched voice each case body spends FIRST (the `0.5F`
        // volume the cp `#177` keeps, the `2.6F + (nf - nf) × 0.8F`
        // pitch the TWO nextFloat draws carry), then the puffs the
        // loops answer — the fizz's `bipush 8` LARGE_SMOKE at the
        // raw corner folds (+1.2 the y lift), the burnout's
        // `iconst_5` SMOKE at the shifted 0.2..0.8 corner bands.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the assignment banks —
        // the whole walk the seeded stream replays)
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1501);
        app.live_smoke_voice_events_updated(vec![crate::world_view::WireSmokeVoiceEvent {
            x: 10,
            y: 20,
            z: 10,
            voice: mc_protocol::LAVA_FIZZ_EFFECT,
        }]);
        assert_eq!(
            app.particles.len(),
            8,
            "the bipush 8 the fizz loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Smoke),
                "the LARGE_SMOKE type the smoke render bank carries"
            );
            // THE RAW CORNER FOLDS (the getX() + nextDouble() the
            // case's own args speak — the [10, 11) band the x/z keep,
            // the +1.2 lift the y's constant rides)
            assert!(
                p.x >= 10.0 && p.x < 11.0,
                "the x the nextDouble band keeps: {}",
                p.x
            );
            assert!(
                (p.y - 21.2).abs() < 1e-9,
                "the +1.2 lift the cp Double keeps: {}",
                p.y
            );
            assert!(
                p.z >= 10.0 && p.z < 11.0,
                "the z the nextDouble band keeps: {}",
                p.z
            );
            // THE VELOCITY (the zero SPEED trio the dconst_0 passes —
            // the chain's own spread the *0.1 scale carries: |vx|,
            // |vz| <= 0.018, the vy band the +0.1 bump scales to
            // [-0.008, 0.028) — the rise the fold and the gravity
            // both give)
            assert!(p.vx.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vx);
            assert!(p.vz.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vz);
            assert!(
                (-0.008..0.028).contains(&p.vy),
                "the smoke's own rise band: {}",
                p.vy
            );
            // THE CTOR LAWS (the 0.96 friction, the −0.1 gravity the
            // rise rides, the 8..40 band the smoke fold gives, the
            // grey tint the r*0.3 keeps, the ×1.875 size fold the 2.5
            // multiplier speaks — the plain smoke's ×0.75 the one
            // constant's own difference wears)
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
            assert!((p.gravity + 0.1).abs() < 1e-9, "the ash gravity");
            assert!(p.lifetime >= 8 && p.lifetime <= 40, "the band: {}", p.lifetime);
            assert_eq!(p.col[0], p.col[1], "the grey the r/g keeps");
            assert_eq!(p.col[1], p.col[2], "the grey the g/b keeps");
            assert!(
                (0.1875..0.375).contains(&p.quad_size),
                "the 0.75 × 2.5 fold's own band: {}",
                p.quad_size
            );
        }
        // THE PITCHED VOICE (the case body's own FIRST walk — the
        // fizz key the official table speaks, the 0.5 volume the cp
        // Float keeps, the pitch band the fold gives [1.8, 3.4])
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.lava.extinguish "))
                .expect("the fizz voice the case body speaks");
            let pitch: f32 = voice
                .rsplit(" p")
                .next()
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse::<f32>().ok())
                .expect("the logged pitch the walk carries");
            assert!(
                (1.8..=3.4).contains(&pitch),
                "the 2.6 ± 0.8×(nf−nf) band the fold keeps: {}",
                pitch
            );
            // THE GAIN (the 0.5 volume the attenuate folds — the
            // close ear the 0.5 verbatim keeps, the distance the
            // reach never amplifies)
            let gain: f32 = voice
                .rsplit(" v")
                .next()
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse().ok())
                .expect("the logged gain the walk carries");
            assert!(
                gain <= 0.5 + 1e-6 && gain > 0.0,
                "the 0.5 volume the attenuation folds: {}",
                gain
            );
        }
        // THE STREAM'S OWN ORDER (the voice's TWO pitch draws the
        // puffs' own follow — 2 + 8 × (2 corner + 9 ctor) = 90 raw
        // draws the whole walk spends, no more no less)
        let advanced = {
            let mut l = crate::drops::Lcg::new(0xC10D_1501);
            for _ in 0..90 {
                l.next();
            }
            l.next()
        };
        assert_eq!(
            app.level_ambient_rng.next(),
            advanced,
            "the voice-then-puffs order the stream's own count keeps"
        );
        // THE DETERMINISM (the same seed the same pitch answers)
        if sounds_loaded() {
            let pitch_first = {
                let mut app2 = App::new(2);
                app2.enter_world();
                app2.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1501);
                app2.live_smoke_voice_events_updated(vec![crate::world_view::WireSmokeVoiceEvent {
                    x: 10,
                    y: 20,
                    z: 10,
                    voice: mc_protocol::LAVA_FIZZ_EFFECT,
                }]);
                app2.sounds
                    .events
                    .iter()
                    .find(|e| e.starts_with("block.lava.extinguish "))
                    .expect("the fizz voice the replay speaks")
                    .to_string()
            };
            let pitch_again = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.lava.extinguish "))
                .expect("the fizz voice the original speaks")
                .to_string();
            assert_eq!(pitch_first, pitch_again, "the seeded chain replays");
        }
        // THE BURNOUT'S OWN WALK (the `iconst_5` loop the shifted
        // corner bands keep — the x/y/z draw order the three dstore
        // rows speak, the nextDouble() × 0.6 + 0.2 the 0.2..0.8 band
        // carries, the plain SMOKE the ×0.75 fold the 1.0 provider
        // keeps)
        app.particles.clear();
        app.sounds.events.clear();
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1502);
        app.live_smoke_voice_events_updated(vec![crate::world_view::WireSmokeVoiceEvent {
            x: 10,
            y: 20,
            z: 10,
            voice: mc_protocol::REDSTONE_TORCH_BURNOUT_EFFECT,
        }]);
        assert_eq!(
            app.particles.len(),
            5,
            "the iconst_5 the burnout loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Smoke),
                "the SMOKE type the provider answers"
            );
            assert!(
                p.x >= 10.2 && p.x < 10.8,
                "the x the shifted band keeps: {}",
                p.x
            );
            assert!(
                p.y >= 20.2 && p.y < 20.8,
                "the y the shifted band keeps: {}",
                p.y
            );
            assert!(
                p.z >= 10.2 && p.z < 10.8,
                "the z the shifted band keeps: {}",
                p.z
            );
            // THE VELOCITY (the zero speed trio the chain's own
            // spread carries — the same drift bands the fizz's puffs
            // wear)
            assert!(p.vx.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vx);
            assert!(p.vz.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vz);
            assert!(
                (-0.008..0.028).contains(&p.vy),
                "the smoke's own rise band: {}",
                p.vy
            );
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
            assert!((p.gravity + 0.1).abs() < 1e-9, "the ash gravity");
            assert!(p.lifetime >= 8 && p.lifetime <= 40, "the band: {}", p.lifetime);
            assert!(
                (0.075..0.15).contains(&p.quad_size),
                "the plain 0.75 fold's own band: {}",
                p.quad_size
            );
        }
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.redstone_torch.burnout ")),
                "the burnout voice the case body speaks"
            );
        }
        // THE BURNOUT'S OWN STREAM (2 + 5 × (3 corner + 9 ctor) = 62
        // raw draws the whole walk spends)
        let advanced = {
            let mut l = crate::drops::Lcg::new(0xC10D_1502);
            for _ in 0..62 {
                l.next();
            }
            l.next()
        };
        assert_eq!(
            app.level_ambient_rng.next(),
            advanced,
            "the voice-then-puffs order the burnout's own count keeps"
        );
        // THE CROSS-WORD SILENCE (each arm exactly one voice — the
        // fizz word never speaks the burnout key, the burnout never
        // the fizz's)
        assert!(
            !app
                .sounds
                .events
                .iter()
                .any(|e| e.contains("block.lava.extinguish")),
            "the burnout walk the fizz voice never shares"
        );
        // THE UNKNOWN WORD: the walk the model's own two words alone
        // bank — anything else the render never answers
        app.particles.clear();
        app.sounds.events.clear();
        app.live_smoke_voice_events_updated(vec![crate::world_view::WireSmokeVoiceEvent {
            x: 10,
            y: 20,
            z: 10,
            voice: 1503,
        }]);
        assert!(app.particles.is_empty(), "the unknown word never puffs");
        assert!(app.sounds.events.is_empty(), "the unknown word never voices");
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.live_smoke_voice_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_portal_frame_word_the_fill_walk_the_puffs_wear() {
        // THE END-PORTAL-FRAME FILL'S OWN WIRE LANE (javap case 1503
        // → offset 3773 over the 26.2 jar): the FIXED voice the case
        // body spends FIRST (the `fconst_1` pair the 1.0/1.0 keeps —
        // no randoms the voice never spends), then the `bipush 16`
        // loop the sixteen SMOKE puffs fire — the x draw FIRST, the z
        // draw SECOND, the +0.8125 lift the frame's top the constant
        // rides, the zero speed trio the dconst_0 rows keep.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the assignment banks —
        // the whole walk the seeded stream replays)
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1503);
        app.live_end_portal_frame_fill_events_updated(vec![
            crate::world_view::WireEndPortalFrameFillEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        assert_eq!(
            app.particles.len(),
            16,
            "the bipush 16 the fill loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Smoke),
                "the SMOKE type the provider answers"
            );
            // THE POSITION FOLDS (the `(5.0 + nextDouble() × 6.0) /
            // 16.0` the cp trio #246/#647/#649 keeps — the
            // 0.3125..0.6875 band the frame's eye slot hovers over,
            // the +0.8125 lift the y's constant rides)
            assert!(
                p.x >= 10.3125 && p.x < 10.6875,
                "the x the 5/16..11/16 band keeps: {}",
                p.x
            );
            assert!(
                (p.y - 20.8125).abs() < 1e-9,
                "the +0.8125 lift the cp Double keeps: {}",
                p.y
            );
            assert!(
                p.z >= 10.3125 && p.z < 10.6875,
                "the z the 5/16..11/16 band keeps: {}",
                p.z
            );
            // THE VELOCITY (the zero SPEED trio the dconst_0 passes —
            // the chain's own spread the *0.1 scale carries: |vx|,
            // |vz| <= 0.018, the vy band the +0.1 bump scales to
            // [-0.008, 0.028) — the rise the fold and the gravity
            // both give)
            assert!(p.vx.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vx);
            assert!(p.vz.abs() <= 0.018 + 1e-9, "the drift band: {}", p.vz);
            assert!(
                (-0.008..0.028).contains(&p.vy),
                "the smoke's own rise band: {}",
                p.vy
            );
            // THE CTOR LAWS (the 0.96 friction, the −0.1 gravity the
            // rise rides, the 8..40 band the smoke fold gives, the
            // grey tint the r*0.3 keeps, the plain ×0.75 fold the 1.0
            // provider keeps)
            assert!((p.friction - 0.96).abs() < 1e-9, "the 0.96 friction");
            assert!((p.gravity + 0.1).abs() < 1e-9, "the ash gravity");
            assert!(p.lifetime >= 8 && p.lifetime <= 40, "the band: {}", p.lifetime);
            assert_eq!(p.col[0], p.col[1], "the grey the r/g keeps");
            assert_eq!(p.col[1], p.col[2], "the grey the g/b keeps");
            assert!(
                (0.075..0.15).contains(&p.quad_size),
                "the plain 0.75 fold's own band: {}",
                p.quad_size
            );
        }
        // THE FIXED VOICE (the case body's own FIRST walk — the fill
        // key the official table speaks, the 1.0 pair the `fconst_1`
        // folds keep, the eyeplace files the sounds.json row banks)
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.end_portal_frame.fill "))
                .expect("the fill voice the case body speaks");
            let pitch: f32 = voice
                .rsplit(" p")
                .next()
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse::<f32>().ok())
                .expect("the logged pitch the walk carries");
            assert!(
                (pitch - 1.0).abs() < 1e-6,
                "the fconst_1 pitch the fixed fold keeps: {}",
                pitch
            );
            let gain: f32 = voice
                .rsplit(" v")
                .next()
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse().ok())
                .expect("the logged gain the walk carries");
            assert!(
                gain <= 1.0 + 1e-6 && gain > 0.0,
                "the 1.0 volume the attenuation folds: {}",
                gain
            );
        }
        // THE STREAM'S OWN ORDER (the fixed voice the walk's zero
        // draws keep — 16 × (2 corner + 9 ctor) = 176 raw draws the
        // whole walk spends, no more no less)
        let advanced = {
            let mut l = crate::drops::Lcg::new(0xC10D_1503);
            for _ in 0..176 {
                l.next();
            }
            l.next()
        };
        assert_eq!(
            app.level_ambient_rng.next(),
            advanced,
            "the voice-then-puffs order the fill's own count keeps"
        );
        // THE DETERMINISM (the same seed the same puffs answer — the
        // x/z bands the seeded chain replays)
        let again = {
            let mut app2 = App::new(2);
            app2.enter_world();
            app2.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1503);
            app2.live_end_portal_frame_fill_events_updated(vec![
                crate::world_view::WireEndPortalFrameFillEvent {
                    x: 10,
                    y: 20,
                    z: 10,
                },
            ]);
            app2
                .particles
                .iter()
                .map(|p| (p.x, p.y, p.z))
                .collect::<Vec<_>>()
        };
        let original = app
            .particles
            .iter()
            .map(|p| (p.x, p.y, p.z))
            .collect::<Vec<_>>();
        assert_eq!(again, original, "the seeded chain replays");
        // THE SECOND ARM (the other frame cell the same walk answers
        // — the anchor's own cell the bands shift to)
        app.particles.clear();
        app.sounds.events.clear();
        app.live_end_portal_frame_fill_events_updated(vec![
            crate::world_view::WireEndPortalFrameFillEvent {
                x: -4,
                y: 30,
                z: 7,
            },
        ]);
        assert_eq!(app.particles.len(), 16, "the fresh word re-walks");
        assert!(
            app.particles.iter().all(|p| (p.y - 30.8125).abs() < 1e-9),
            "the new anchor's own lift the y keeps"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.end_portal_frame.fill ")),
                "the fill voice the fresh arm speaks"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_end_portal_frame_fill_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_drip_word_the_hang_walk_the_chain_wears() {
        // THE DRIPSTONE-DRIP'S OWN WIRE LANE (javap case 1504 →
        // offset 3503 → the 4-arg spawnDripParticle over the 26.2
        // jar): the OFFSET the getOffset read folds (the seed law's
        // own 4-bit draws the ±0.125 clamp carries), the +0.25 lift
        // the tip's own point keeps, the HANG drop the one draw
        // spawns — the chain the walks answer: the hang's 40-tick
        // countdown the expiry rides, the fall the gravity walks, the
        // landing the world-read fold answers.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the assignment banks —
        // the whole walk the seeded stream replays)
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1504);
        app.live_dripstone_drip_events_updated(vec![
            crate::world_view::WireDripstoneDripEvent {
                x: 10,
                y: 20,
                z: 10,
                lava: false,
            },
        ]);
        assert_eq!(app.particles.len(), 1, "the one hang drop the walk spawns");
        let p = &app.particles[0];
        assert!(
            matches!(
                p.kind,
                crate::particles::ParticleKind::DripstoneHang { lava: false }
            ),
            "the water drop the lava bit keeps"
        );
        // THE OFFSET FOLD (the seed the `Mth.getSeed(x, 0, z)` law
        // hashes — the exact folds the offsetter keeps)
        let seed = crate::world::mth_get_seed(10, 0, 10);
        let want_x = 10.5 + crate::particles::dripstone_offset(seed & 15, 0.125);
        let want_z = 10.5 + crate::particles::dripstone_offset((seed >> 8) & 15, 0.125);
        assert!((p.x - want_x).abs() < 1e-12, "the x the offset fold keeps: {}", p.x);
        assert!((p.z - want_z).abs() < 1e-12, "the z the offset fold keeps: {}", p.z);
        // THE CLAMP BAND (the ±0.125 the speleothem's own max keeps —
        // whatever the seed's own slice)
        assert!(
            (10.375..=10.625).contains(&p.x) && (10.375..=10.625).contains(&p.z),
            "the ±0.125 clamp the lattice keeps: {} {}",
            p.x,
            p.z
        );
        // THE LIFT (the y the +0.25 fold the tip's own point answers —
        // 0.3125 − 0.0625 the STALACTITE_DRIP_START_PIXEL walk keeps)
        assert!((p.y - 20.25).abs() < 1e-12, "the +0.25 lift: {}", p.y);
        // THE CTOR LAWS (the 40-tick lifetime, the raw 0.0012 gravity
        // the ×0.02 fold gives, the ×0.02 damping the postMoveUpdate
        // keeps, the quadSize 0.01, the water tint the setColor
        // (0.2F, 0.3F, 1.0F) override speaks)
        assert_eq!(p.lifetime, 40, "the bipush 40 the DripHang keeps");
        assert!((p.gravity - 0.0012).abs() < 1e-12, "the 0.06 × 0.02 fold");
        assert!((p.friction - 0.02).abs() < 1e-12, "the damping trio");
        assert!((p.quad_size - 0.01).abs() < 1e-9, "the 0.01 bb the base ctor keeps");
        assert_eq!(p.col, [0.2, 0.3, 1.0], "the water tint");
        assert_eq!(p.vx, 0.0, "the zero speed trio the dconst_0 keeps");
        // THE STREAM'S OWN ORDER (the walk spends exactly ONE raw draw
        // — the sprite pick the nextInt(1) answers, nothing the fixed
        // walks spend)
        let advanced = {
            let mut l = crate::drops::Lcg::new(0xC10D_1504);
            l.next();
            l.next()
        };
        assert_eq!(
            app.level_ambient_rng.next(),
            advanced,
            "the one-draw spawn the sprite pick keeps"
        );
        // THE SPAWN POSE BANKED (the fresh spawn the determinism check
        // replays — the walk section below mutates the particle, the
        // banked pose the replay compares)
        let original = app
            .particles
            .iter()
            .map(|p| (p.x, p.y, p.z, p.col))
            .collect::<Vec<_>>();
        // THE HANG'S OWN CHAIN (the countdown walk 39 ticks keep —
        // the 40th the expiry speaks: the fall spawn the carried
        // motion rides)
        let mut falls: Vec<(f64, f64, f64, f64, f64, f64, bool)> = Vec::new();
        for _ in 0..39 {
            assert!(
                app.particles[0].tick_dripstone_hang(&mut falls),
                "the countdown the 39 ticks keep"
            );
            assert!(falls.is_empty(), "the living hang the expiry never speaks");
        }
        // THE LAVA FADE (the cooling walk the g/b folds speak — the
        // water drop the static tint keeps through the ticks)
        assert_eq!(app.particles[0].col, [0.2, 0.3, 1.0], "the static tint rides");
        assert!(
            !app.particles[0].tick_dripstone_hang(&mut falls),
            "the 40th tick the expiry speaks"
        );
        assert_eq!(falls.len(), 1, "the fall spawn the carried motion rides");
        let (fx, fy, fz, fvxn, fvyn, fvzn, flava) = falls[0];
        assert_eq!(fx, app.particles[0].x, "the spawn pose the expiry keeps");
        assert_eq!(fy, app.particles[0].y);
        assert_eq!(fz, app.particles[0].z);
        assert!(!flava, "the water drop the bit keeps");
        let _ = (fvxn, fvyn, fvzn);
        // THE FALL SPAWN (the two draws the provider keeps — the
        // sprite pick, then the lifetime float) + THE CTOR LAWS
        let r_sprite = app.level_ambient_rng.next_f32();
        let r_life = app.level_ambient_rng.next_f32();
        let fall = crate::particles::dripstone_drip_fall(fx, fy, fz, false, r_sprite, r_life);
        assert!(matches!(
            fall.kind,
            crate::particles::ParticleKind::DripstoneFall { lava: false }
        ));
        assert_eq!(fall.lifetime, (64.0f32 / (r_life * 0.8 + 0.2)) as u32, "the 64/(r×0.8+0.2) fold");
        assert!((fall.gravity - 0.06).abs() < 1e-12, "the raw 0.06 the base ctor keeps");
        assert!((fall.friction - 0.98).abs() < 1e-12, "the ×0.98 damping");
        assert_eq!(fall.col, [0.2, 0.3, 1.0], "the water tint the provider keeps");
        // THE LAVA VARIANTS (the fall's cooling-end tint the constants
        // keep, the land splat's own walk)
        let fall_lava = crate::particles::dripstone_drip_fall(0.0, 0.0, 0.0, true, 0.0, 0.5);
        assert_eq!(
            fall_lava.col,
            [1.0, 0.2857143, 0.083333336],
            "the cooling-end tint the lava fall keeps"
        );
        // THE FALL'S OWN LANDING (the world-read fold — the solid cell
        // the drop enters the landing answers, the boundary the splat
        // keeps)
        let mut w = crate::world::World::new(1, 7);
        w.set(0, 10, 0, crate::world::STONE);
        let mut land_out: Vec<(f64, f64, f64, bool)> = Vec::new();
        let mut dropper = crate::particles::dripstone_drip_fall(0.5, 12.9, 0.5, false, 0.0, 0.5);
        // the drop walks the air cells down to the floor at y 10
        for _ in 0..64 {
            if !dropper.tick_dripstone_fall(Some(&w), &mut land_out) {
                break;
            }
        }
        assert_eq!(land_out.len(), 1, "the landing the solid cell speaks");
        let (lx, lboundary, lz, llava) = land_out[0];
        assert!((lboundary - 11.0).abs() < 1e-12, "the boundary the cell top keeps: {}", lboundary);
        assert!((lx - 0.5).abs() < 1e-9 && (lz - 0.5).abs() < 1e-9, "the column the drop keeps");
        assert!(!llava, "the water drop the splash answers");
        // THE LAND WALKS (the water's SPLASH the three draws keep, the
        // lava's LANDING_LAVA the two)
        let splash = crate::particles::splash(
            lx, lboundary, lz, 0.0, 0.0, 0.0,
            app.level_ambient_rng.next_f32(),
            app.level_ambient_rng.next_f32(),
            app.level_ambient_rng.next_f32(),
        );
        assert!(matches!(splash.kind, crate::particles::ParticleKind::Splash { .. }));
        let lava_splat = crate::particles::dripstone_drip_land(
            lx, lboundary, lz,
            app.level_ambient_rng.next_f32(),
            app.level_ambient_rng.next_f32(),
        );
        assert!(matches!(
            lava_splat.kind,
            crate::particles::ParticleKind::DripstoneLand
        ));
        assert_eq!(
            lava_splat.col,
            [1.0, 0.2857143, 0.083333336],
            "the cooling-end tint the land keeps"
        );
        // THE LAND'S OWN REST (the pinned pose the collision fold
        // keeps — the splat the boundary holds)
        let y0 = lava_splat.y;
        let mut splat = lava_splat;
        for _ in 0..8 {
            assert!(splat.tick_dripstone_land(), "the splat the 16..=80 life keeps");
            assert_eq!(splat.y, y0, "the pose the resting fold pins");
        }
        // THE SEED LAW'S OWN PIN (the known hash the walk replays —
        // the x-multiply overflow the i32 lane keeps)
        assert_eq!(crate::world::mth_get_seed(0, 0, 0), 0, "the zero column the zero seed keeps");
        // THE DETERMINISM (the same seed the same folds answer — the
        // banked spawn pose the replay compares)
        let again = {
            let mut app2 = App::new(2);
            app2.enter_world();
            app2.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1504);
            app2.live_dripstone_drip_events_updated(vec![
                crate::world_view::WireDripstoneDripEvent {
                    x: 10,
                    y: 20,
                    z: 10,
                    lava: false,
                },
            ]);
            app2
                .particles
                .iter()
                .map(|p| (p.x, p.y, p.z, p.col))
                .collect::<Vec<_>>()
        };
        assert_eq!(again, original, "the seeded chain replays");
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_dripstone_drip_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_composter_word_the_fill_walk_the_puffs_wear() {
        // THE COMPOSTER-FILL'S OWN WIRE LANE (javap case 1500 →
        // offset 3483 over the 26.2 jar): the success ternary the
        // data bit picks (the handleFill voice the case's own head
        // speaks), the `bipush 10` loop the ten COMPOSTER puffs fire
        // — the THREE gaussian velocity draws the args carry first,
        // the interior position floats the f2d domain keeps, the
        // provider's own five draws the ctor spends (the autopush-450
        // correction — the shared fstore [15] tint the bytecodes
        // speak).
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the assignment banks —
        // the whole walk the seeded stream replays)
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1500);
        app.live_composter_fill_events_updated(vec![crate::world_view::WireComposterFillEvent {
            x: 12,
            y: 40,
            z: -21,
            data: 1,
        }]);
        assert_eq!(
            app.particles.len(),
            10,
            "the bipush 10 the fill loop keeps"
        );
        for p in &app.particles {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::HappyVillager),
                "the COMPOSTER type the glint bank carries"
            );
            // THE INTERIOR FOLDS (the x + 0.1875 + 0.625 * nf the
            // case's own args speak — the [12.1875, 12.8125) band the
            // x/z keep, the y band the d0 = 1.03125 shape fold keeps —
            // the puffs hover at the rim top [41.0, 41.03125])
            assert!(
                p.x >= 12.1875 && p.x < 12.8125,
                "the x the interior band keeps: {}",
                p.x
            );
            assert!(
                p.y >= 41.0 && p.y <= 41.03125,
                "the y the rim-top band keeps: {}",
                p.y
            );
            assert!(
                p.z >= -20.8125 && p.z < -20.1875,
                "the z the interior band keeps: {}",
                p.z
            );
            // THE VELOCITY (the gaussian trio the * 0.02 × 0.02 the
            // two folds keep — the drift the practical band gives)
            assert!(p.vx.abs() < 0.005, "the drift band: {}", p.vx);
            assert!(p.vy.abs() < 0.005, "the drift band: {}", p.vy);
            assert!(p.vz.abs() < 0.005, "the drift band: {}", p.vz);
            // THE CTOR LAWS (the white tint, the 3..7 lifetime, the
            // size band, the 0.99 friction, the no-gravity field)
            assert_eq!(p.col, [1.0, 1.0, 1.0], "the white the override keeps");
            assert!(p.lifetime >= 3 && p.lifetime <= 7, "the band: {}", p.lifetime);
            assert!(
                (0.01..0.022).contains(&p.quad_size),
                "the size band: {}",
                p.quad_size
            );
            assert!((p.friction - 0.99).abs() < 1e-9, "the 0.99 friction");
            assert!(p.gravity.abs() < 1e-9, "the no-gravity law");
        }
        // THE SUCCESS VOICE (the ternary's accept arm the data=1 bit
        // picks — the fill_success key the official table speaks, the
        // 1.0/1.0 pair the fconst_1 bytecode keeps)
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.composter.fill_success "))
                .expect("the success voice the accept bit picks");
            let pitch: f32 = voice
                .rsplit(" p")
                .next()
                .and_then(|s| s.split(' ').next())
                .and_then(|s| s.parse().ok())
                .expect("the logged pitch the walk carries");
            assert!((pitch - 1.0).abs() < 1e-6, "the fixed pitch: {}", pitch);
        }
        // THE DETERMINISM (the same seed the same walk answers)
        let replay = {
            let mut app2 = App::new(2);
            app2.enter_world();
            app2.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1500);
            app2.live_composter_fill_events_updated(vec![crate::world_view::WireComposterFillEvent {
                x: 12,
                y: 40,
                z: -21,
                data: 1,
            }]);
            app2.particles
                .iter()
                .map(|p| (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size))
                .collect::<Vec<_>>()
        };
        let original = app
            .particles
            .iter()
            .map(|p| (p.x, p.y, p.z, p.vx, p.vy, p.vz, p.lifetime, p.quad_size))
            .collect::<Vec<_>>();
        assert_eq!(original, replay, "the seeded chain replays");
        // THE FAIL WALK (the data=0 bit the ternary's chance-fail arm
        // picks — the plain fill key the voice wears, the ten puffs
        // the loop still fires)
        app.particles.clear();
        app.sounds.events.clear();
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_1500);
        app.live_composter_fill_events_updated(vec![crate::world_view::WireComposterFillEvent {
            x: 12,
            y: 40,
            z: -21,
            data: 0,
        }]);
        assert_eq!(
            app.particles.len(),
            10,
            "the bipush 10 the fail walk keeps"
        );
        if sounds_loaded() {
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("block.composter.fill ")),
                "the fail voice the bit picks"
            );
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.contains("fill_success")),
                "the fail walk the success voice never shares"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_composter_fill_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_eye_word_the_shatter_walk_the_ring_wears() {
        // THE EYE-OF-ENDER SHATTER'S OWN WIRE LANE (javap case 2003 →
        // offset 947 over the 26.2 jar): the `bipush 8` shard burst
        // the `ItemParticleOption(ParticleTypes.ITEM, Items.ENDER_
        // EYE)` loop fires first, the PORTAL ring the `(Math.PI * 2)`
        // half-circle walk answers — the TWO puffs per step the ×−5
        // and ×−7 velocity words spend, the 40 steps the 2π/40
        // constant draws — NO SOUND the case speaks.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the assignment banks — the
        // whole walk the seeded stream replays)
        app.level_ambient_rng = crate::drops::Lcg::new(0xC10D_2003);
        app.live_eye_of_ender_shatter_events_updated(vec![
            crate::world_view::WireEyeOfEnderShatterEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        // THE BURST COUNT (the 8 shards + the 40-step ring's TWO
        // puffs per step the 80 the half-circle keeps)
        assert_eq!(app.particles.len(), 8 + 80, "the 8 + 80 puffs the case body keeps");
        // THE SHARDS (the BreakingItemParticle walk the eye's own
        // item sprite wears — the anchor position the case's own
        // head places, the velocity trio the gauss×0.15 / double×0.2
        // bands give)
        let shards = &app.particles[..8];
        for s in shards {
            assert!(
                matches!(s.kind, crate::particles::ParticleKind::EyeItem),
                "the ITEM type the eye wears"
            );
            assert!((s.x - 10.5).abs() < 1e-9, "the x the +0.5 center keeps: {}", s.x);
            assert!((s.y - 20.0).abs() < 1e-9, "the raw y the plain dstore keeps: {}", s.y);
            assert!((s.z - 10.5).abs() < 1e-9, "the z the +0.5 center keeps: {}", s.z);
            assert!(
                s.vx.abs() < 0.5 && s.vz.abs() < 0.5,
                "the gauss band the ×0.15 fold gives: {} / {}",
                s.vx,
                s.vz
            );
            assert!(s.vy >= 0.0 && s.vy < 0.2, "the double band the ×0.2 fold gives: {}", s.vy);
            assert!((s.gravity - 1.0).abs() < 1e-9, "the gravity 1.0 the shard keeps");
        }
        // THE RING (the 40-step walk's own law — the cos/sin ×5
        // positions the `y − 0.4` lift wears, the ×−5 / ×−7 velocity
        // pairs the two puffs per step answer, the purple tint the
        // ctor's own draw chain keeps)
        let ring = &app.particles[8..];
        let steps = ring.len() / 2;
        assert_eq!(steps, 40, "the 40 steps the half-circle draws");
        for (i, pair) in ring.chunks(2).enumerate() {
            let d = (i as f64) * 0.15707963267948966f64;
            let (expect_px, expect_pz) = (10.5 + d.cos() * 5.0, 10.5 + d.sin() * 5.0);
            for (vel, puff) in [(-5.0f64, &pair[0]), (-7.0f64, &pair[1])] {
                assert!(
                    matches!(puff.kind, crate::particles::ParticleKind::Portal { .. }),
                    "the PORTAL type the ring wears"
                );
                assert!(
                    (puff.x - expect_px).abs() < 1e-9,
                    "the cos ×5 position the ring keeps: {} vs {}",
                    puff.x,
                    expect_px
                );
                assert!((puff.y - 19.6).abs() < 1e-9, "the y the −0.4 lift keeps: {}", puff.y);
                assert!(
                    (puff.z - expect_pz).abs() < 1e-9,
                    "the sin ×5 position the ring keeps: {} vs {}",
                    puff.z,
                    expect_pz
                );
                assert!(
                    (puff.vx - d.cos() * vel).abs() < 1e-9,
                    "the cos ×{} velocity the ring's own word keeps: {}",
                    vel,
                    puff.vx
                );
                assert!((puff.vy).abs() < 1e-9, "the zero vy the ring carries");
                assert!(
                    (puff.vz - d.sin() * vel).abs() < 1e-9,
                    "the sin ×{} velocity the ring's own word keeps: {}",
                    vel,
                    puff.vz
                );
                assert!(puff.lifetime >= 40 && puff.lifetime <= 49, "the lifetime band: {}", puff.lifetime);
                // THE PURPLE (the r·0.9 / g·0.3 / b trio — the b the
                // strongest the ring's own look keeps)
                assert!(puff.col[2] >= puff.col[0] && puff.col[0] > puff.col[1], "the tint order the purple keeps");
            }
        }
        // THE DETERMINISM (the same seed the same chain answers — the
        // 8 × 10 shard draws + the 80 × 4 ring draws the walk spends)
        let a = crate::drops::Lcg::new(0xC10D_2003);
        let mut app2 = App::new(2);
        app2.enter_world();
        app2.level_ambient_rng = a;
        app2.live_eye_of_ender_shatter_events_updated(vec![
            crate::world_view::WireEyeOfEnderShatterEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        for (pa, pb) in app.particles.iter().zip(app2.particles.iter()) {
            assert_eq!(
                (pa.x, pa.y, pa.z, pa.vx, pa.vy, pa.vz, pa.lifetime, pa.quad_size, pa.col),
                (pb.x, pb.y, pb.z, pb.vx, pb.vy, pb.vz, pb.lifetime, pb.quad_size, pb.col),
                "the seeded chain replays"
            );
        }
        // THE SILENCE (the ENDER_EYE_DEATH voice the server's own
        // playSound carries on the death branch, NOT the case's)
        if sounds_loaded() {
            assert!(
                !app.sounds.events.iter().any(|e| e.contains("ender_eye")),
                "the case's own silence the voice never shares"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_eye_of_ender_shatter_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_dragon_word_the_center_burst_wears() {
        // THE DRAGON BLOCK-BREAK'S OWN WIRE LANE (javap case 2008 →
        // offset 3440 over the 26.2 jar): the ONE EXPLOSION puff the
        // block's own center fires (the cp `#197` Double decode the
        // 0.5 centers speak, the zero velocity trio the `dconst_0`
        // answers, the stationary blast the whole 6..9-tick life
        // wears), the grey tint the ctor's ONE draw carries — NO
        // SOUND the case speaks (the block's own break voice the
        // server's separate broadcasts own).
        let mut app = App::new(2);
        app.enter_world();
        app.live_dragon_block_break_events_updated(vec![
            crate::world_view::WireDragonBlockBreakEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        assert_eq!(
            app.particles.len(),
            1,
            "the ONE puff the case body keeps"
        );
        let p = &app.particles[0];
        assert!(
            matches!(p.kind, crate::particles::ParticleKind::Explosion),
            "the EXPLOSION type the provider answers"
        );
        // THE CENTER FOLD (the cp Double the +0.5 the x/y/z keeps)
        assert!((p.x - 10.5).abs() < 1e-9, "the x the 0.5 center keeps: {}", p.x);
        assert!((p.y - 20.5).abs() < 1e-9, "the y the 0.5 center keeps: {}", p.y);
        assert!((p.z - 10.5).abs() < 1e-9, "the z the 0.5 center keeps: {}", p.z);
        // THE ZERO VELOCITY TRIO (the dconst_0 the case passes)
        assert_eq!(p.vx, 0.0, "the zero vx");
        assert_eq!(p.vy, 0.0, "the zero vy");
        assert_eq!(p.vz, 0.0, "the zero vz");
        // THE CTOR LAWS (the 6..9 band the nextInt(4) draw gives, the
        // 2.0 quad the zero-speed fold keeps, the shared f11 the
        // r/g/b trio keeps)
        assert!(p.lifetime >= 6 && p.lifetime <= 9, "the band: {}", p.lifetime);
        assert!((p.quad_size - 2.0).abs() < 1e-9, "the 2.0 quad");
        assert_eq!(p.col[0], p.col[1], "the shared f11 the r/g keeps");
        assert_eq!(p.col[1], p.col[2], "the shared f11 the g/b keeps");
        assert!(
            (0.4..=1.0).contains(&p.col[0]),
            "the tint band the grey keeps: {}",
            p.col[0]
        );
        // THE STATIONARY TICK (the app's own update the override's
        // body speaks — the pose the block's own center keeps, the
        // frame walk the ascending list rides)
        let pre = (p.x, p.y, p.z);
        app.update();
        let p = &app.particles[0];
        assert_eq!(
            (p.x, p.y, p.z),
            pre,
            "the stationary blast the pose keeps"
        );
        // THE SILENCE (the case's own body the voice never speaks —
        // no explode event the wire walk adds)
        if sounds_loaded() {
            assert!(
                !app
                    .sounds
                    .events
                    .iter()
                    .any(|e| e.contains("generic.explode")),
                "the case's own silence the voice never shares"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.particles.clear();
        app.live_dragon_block_break_events_updated(vec![]);
        assert!(app.particles.is_empty(), "the empty mailbox ships nothing");
    }


    #[test]
    fn the_wire_dispenser_word_the_center_voice_wears() {
        // THE DISPENSER-DISPENSE'S OWN WIRE LANE (javap case 1000 →
        // offset 776 over the 26.2 jar): the ONE `playLocalSound(pos,
        // DISPENSER_DISPENSE, BLOCKS, 1.0F, 1.0F, false)` walk the
        // case body spends — the click voice the fixed `fconst_1`
        // pair carries, the BLOCK CENTER the playLocalSound(BlockPos,
        // ...) resolve places, NO PARTICLES the case speaks (the
        // voice alone the broadcast renders), the level ambient
        // stream the fixed fold never spends.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the swap banks — no
        // draw the fixed 1.0/1.0 fold costs, the `fconst_1` pair the
        // randoms never ride)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(1234));
        let particles_before = app.particles.len();
        app.live_dispenser_dispense_events_updated(vec![
            crate::world_view::WireDispenserDispenseEvent {
                x: 10,
                y: 20,
                z: 10,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert_eq!(
            spent,
            crate::drops::Lcg::new(1234),
            "the fixed fold the ambient stream never spends"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case body speaks"
        );
        // THE VOICE (the case's own body the click speaks — the
        // volume/pitch BOTH the fixed 1.0 the `fconst_1` pair keeps,
        // the file pick the engine's own LCG banks)
        if sounds_loaded() {
            let voices = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.dispense @"))
                .count();
            assert_eq!(voices, 1, "the ONE playLocalSound the case body keeps");
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.dispenser.dispense @"))
                .expect("the click voice the case speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.0).abs() < 1e-6,
                "the fixed pitch the fconst_1 keeps: {pitch}"
            );
            let volume: f32 = voice
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 1.0).abs() < 1e-6,
                "the fixed volume the fconst_1 keeps: {volume}"
            );
        }
        // THE SECOND ARM (the next dispenser's eject the same walk
        // wears — each word ships exactly one voice)
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.dispense @"))
                .count();
            app.live_dispenser_dispense_events_updated(vec![
                crate::world_view::WireDispenserDispenseEvent {
                    x: 0,
                    y: 0,
                    z: 0,
                },
            ]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.dispense @"))
                .count();
            assert_eq!(
                voices_after,
                voices_before + 1,
                "each arm exactly one voice the walk keeps"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.dispense @"))
                .count();
            app.live_dispenser_dispense_events_updated(vec![]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.dispense @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the empty mailbox ships nothing"
            );
        } else {
            app.live_dispenser_dispense_events_updated(vec![]);
        }
    }


    #[test]
    fn the_wire_dispenser_pair_the_pitched_voices_wear() {
        // THE DISPENSER BAND'S OWN PITCHED PAIR (javap cases
        // 1001/1002 → offsets 755/857 over the 26.2 jar): the ONE
        // `playLocalSound(pos, <voice>, BLOCKS, 1.0F, 1.2F, false)`
        // walk each case body spends — the fail's click and the
        // launch's bow the fixed folds carry, the BLOCK CENTER the
        // playLocalSound(BlockPos, ...) resolve places, NO PARTICLES
        // the cases speak (the voices alone the broadcast renders),
        // the level ambient stream the fixed folds never spend.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the swap banks — no
        // draw the fixed 1.0/1.2 folds cost, the `fconst_1`/`ldc1`
        // pair the randoms never ride)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(4321));
        let particles_before = app.particles.len();
        app.live_dispenser_fail_launch_events_updated(vec![
            crate::world_view::WireDispenserFailLaunchEvent {
                x: -3,
                y: 64,
                z: 77,
                voice: mc_protocol::SOUND_DISPENSER_FAIL_EFFECT,
            },
            crate::world_view::WireDispenserFailLaunchEvent {
                x: 5,
                y: 30,
                z: -9,
                voice: mc_protocol::SOUND_DISPENSER_PROJECTILE_LAUNCH_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert_eq!(
            spent,
            crate::drops::Lcg::new(4321),
            "the fixed folds the ambient stream never spends"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case bodies speak"
        );
        // THE VOICES (the cases' own bodies the raised 1.2 pitch
        // speaks — the volume the `fconst_1` keeps, the pitch the
        // `ldc1` CP Float carries, the file picks the engine's own
        // LCG banks)
        if sounds_loaded() {
            let fail = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.dispenser.fail @"))
                .expect("the fail voice the case-1001 body speaks");
            let pitch: f32 = fail
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.2).abs() < 1e-6,
                "the raised pitch the ldc1 keeps: {pitch}"
            );
            let volume: f32 = fail
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 1.0).abs() < 1e-6,
                "the fixed volume the fconst_1 keeps: {volume}"
            );
            let launch = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.dispenser.launch @"))
                .expect("the launch voice the case-1002 body speaks");
            let pitch: f32 = launch
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.2).abs() < 1e-6,
                "the raised pitch the pair shares: {pitch}"
            );
            let volume: f32 = launch
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 1.0).abs() < 1e-6,
                "the fixed volume the pair keeps: {volume}"
            );
        }
        // THE SECOND ARM (the next dispenser's fail the same walk
        // wears — each word ships exactly one voice)
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.fail @"))
                .count();
            app.live_dispenser_fail_launch_events_updated(vec![
                crate::world_view::WireDispenserFailLaunchEvent {
                    x: 0,
                    y: 0,
                    z: 0,
                    voice: mc_protocol::SOUND_DISPENSER_FAIL_EFFECT,
                },
            ]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.fail @"))
                .count();
            assert_eq!(
                voices_after,
                voices_before + 1,
                "each arm exactly one voice the walk keeps"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.fail @"))
                .count();
            app.live_dispenser_fail_launch_events_updated(vec![]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.dispenser.fail @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the empty mailbox ships nothing"
            );
        } else {
            app.live_dispenser_fail_launch_events_updated(vec![]);
        }
    }


    #[test]
    fn the_wire_firework_word_the_raised_voice_wears() {
        // THE FIREWORK-SHOOT VOICE'S OWN WIRE LANE (javap case 1004 →
        // offset 836 over the 26.2 jar): the ONE `playLocalSound(pos,
        // FIREWORK_ROCKET_SHOOT, NEUTRAL, 1.0F, 1.2F, false)` walk the
        // case body spends — the raised pitch the fixed folds carry,
        // the BLOCK CENTER the playLocalSound(BlockPos, ...) resolve
        // places, NO PARTICLES the case speaks (the voice alone the
        // broadcast renders), the level ambient stream the fixed folds
        // never spend.
        let mut app = App::new(2);
        app.enter_world();
        // THE STREAM'S OWN PIN (the fresh LCG the swap banks — no
        // draw the fixed 1.0/1.2 folds cost, the `fconst_1`/`ldc1`
        // pair the randoms never ride)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(4321));
        let particles_before = app.particles.len();
        app.live_firework_shoot_events_updated(vec![
            crate::world_view::WireFireworkShootEvent {
                x: -3,
                y: 64,
                z: 77,
                voice: mc_protocol::FIREWORK_ROCKET_SHOOT_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert_eq!(
            spent,
            crate::drops::Lcg::new(4321),
            "the fixed folds the ambient stream never spends"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case body speaks"
        );
        // THE VOICE (the case's own body the raised 1.2 pitch
        // speaks — the volume the `fconst_1` keeps, the pitch the
        // `ldc1` CP Float carries, the file picks the engine's own
        // LCG banks)
        if sounds_loaded() {
            let shoot = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.firework_rocket.shoot @"))
                .expect("the shoot voice the case-1004 body speaks");
            let pitch: f32 = shoot
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.2).abs() < 1e-6,
                "the raised pitch the ldc1 keeps: {pitch}"
            );
            let volume: f32 = shoot
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 1.0).abs() < 1e-6,
                "the fixed volume the fconst_1 keeps: {volume}"
            );
        }
        // THE SECOND ARM (the next dispenser's firework the same walk
        // wears — each word ships exactly one voice)
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.firework_rocket.shoot @"))
                .count();
            app.live_firework_shoot_events_updated(vec![
                crate::world_view::WireFireworkShootEvent {
                    x: 0,
                    y: 0,
                    z: 0,
                    voice: mc_protocol::FIREWORK_ROCKET_SHOOT_EFFECT,
                },
            ]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.firework_rocket.shoot @"))
                .count();
            assert_eq!(
                voices_after,
                voices_before + 1,
                "each arm exactly one voice the walk keeps"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        if sounds_loaded() {
            let voices_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.firework_rocket.shoot @"))
                .count();
            app.live_firework_shoot_events_updated(vec![]);
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.firework_rocket.shoot @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the empty mailbox ships nothing"
            );
        } else {
            app.live_firework_shoot_events_updated(vec![]);
        }
    }


    #[test]
    fn the_wire_extinguish_word_the_pitched_voices_wear() {
        // THE FIRE-EXTINGUISH VOICE'S OWN WIRE LANE (javap case 1009 →
        // offset 4137 over the 26.2 jar): the data word the case's own
        // two-branch fold reads — the head branch the ONE
        // `playLocalSound(pos, FIRE_EXTINGUISH, BLOCKS, 0.5F, 2.6F +
        // (r - r) * 0.8F, false)` walk speaks, the tail branch the ONE
        // `playLocalSound(pos, GENERIC_EXTINGUISH_FIRE, BLOCKS, 0.7F,
        // 1.6F + (r - r) * 0.4F, false)` walk keeps, the other words
        // the case's own silence answers (no voice, no draw the
        // neither-branch ever spends), NO PARTICLES the case speaks.
        let mut app = App::new(2);
        app.enter_world();
        // THE HEAD BRANCH (data == 0): the block.fire.extinguish voice
        // the 0.5F volume the branch's own ldc keeps, the pitch the
        // 2.6 + (r1 - r2) * 0.8 fold carries — the TWO draws the
        // branch body alone spends
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xE0E1));
        let particles_before = app.particles.len();
        app.live_fire_extinguish_events_updated(vec![
            crate::world_view::WireFireExtinguishEvent {
                x: 9,
                y: 65,
                z: -3,
                data: 0,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // the exact two-draw replay (the branch's own body the only
        // spender — the engine's own stream the voice's file pick
        // banks)
        let mut replay = crate::drops::Lcg::new(0xE0E1);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(
            spent, replay,
            "the branch's own two draws the walk spends"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case body speaks"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.fire.extinguish @"))
                .expect("the head-branch voice the data-0 pick speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 2.6).abs() < 0.8,
                "the 2.6 ± 0.8 band the head branch keeps: {pitch}"
            );
            let volume: f32 = voice
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 0.5).abs() < 1e-6,
                "the 0.5 volume the head branch keeps: {volume}"
            );
        }
        // THE TAIL BRANCH (data == 1): the
        // entity.generic.extinguish_fire voice the 0.7F volume the
        // branch keeps, the 1.6 ± 0.4 band the same fold speaks
        if sounds_loaded() {
            let before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.generic.extinguish_fire @"))
                .count();
            app.live_fire_extinguish_events_updated(vec![
                crate::world_view::WireFireExtinguishEvent {
                    x: 1,
                    y: 2,
                    z: 3,
                    data: 1,
                },
            ]);
            let after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.generic.extinguish_fire @"))
                .count();
            assert_eq!(
                after,
                before + 1,
                "the tail branch exactly one voice the walk keeps"
            );
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.generic.extinguish_fire @"))
                .expect("the tail-branch voice the data-1 pick speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.6).abs() < 0.4,
                "the 1.6 ± 0.4 band the tail branch keeps: {pitch}"
            );
            let volume: f32 = voice
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 0.7).abs() < 1e-6,
                "the 0.7 volume the tail branch keeps: {volume}"
            );
        }
        // THE SILENT WORD (the neither branch the case's own guards
        // keep — no voice, no draw the word ever spends)
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0xE0E2));
        app.live_fire_extinguish_events_updated(vec![
            crate::world_view::WireFireExtinguishEvent {
                x: 0,
                y: 0,
                z: 0,
                data: 7,
            },
        ]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            spent2,
            crate::drops::Lcg::new(0xE0E2),
            "the silent word the draws never spends"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        app.live_fire_extinguish_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            particles_before,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_sculk_serve_the_gate_the_voices_wear() {
        // THE SCULK-CHARGE SERVE'S OWN WIRE LANE (javap case 3006 →
        // offset 2668 over the 26.2 jar): the data word the case's own
        // split reads — the count <= 0 arm (THE VANILLA-CALLER ARM the
        // spreader's decay feeds) the ONE fixed 1.0F/1.0F voice + the
        // pop cube burst the full-block verdict splits, the count > 0
        // arm the probability-gated voice + the face walk keeps, the
        // draw order the bytecodes pin (the gate FIRST, the volume,
        // the pitch, then the face walk's own chain).
        let mut app = App::new(2);
        app.enter_world();
        let particles_before = app.particles.len();
        // THE POP ARM (data == 0, the spreader's own word): the fixed
        // voice the fconst_1 pair speaks, the 40-pop burst the full
        // block answers — the 240 draws the walk spends (per pop the
        // cube trio + the dead life + the size + the lifetime int)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x5C01));
        app.live_sculk_charge_events_updated(vec![
            crate::world_view::WireSculkChargeEvent {
                x: 9,
                y: 65,
                z: -3,
                data: 0,
                full_block: true,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let mut replay = crate::drops::Lcg::new(0x5C01);
        for _ in 0..40 {
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_int(4);
        }
        assert_eq!(spent, replay, "the pop arm's own draws the walk spends");
        assert_eq!(
            app.particles.len(),
            particles_before + 40,
            "the 40 pops the full-block arm speaks"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.sculk.charge @"))
                .expect("the fixed voice the pop arm speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.0).abs() < 1e-6,
                "the 1.0 pitch the fconst_1 pair keeps: {pitch}"
            );
        }
        // THE GATED ARM (data = 4 << 6, the count>0 face walk): the
        // gate the first draw arms, the volume/pitch pair the pass
        // arm spends, the face walk the same stream continues — the
        // burst size the seeded replay answers
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x5C02));
        let burst_before = app.particles.len();
        app.live_sculk_charge_events_updated(vec![
            crate::world_view::WireSculkChargeEvent {
                x: 9,
                y: 65,
                z: -3,
                data: 4 << 6,
                full_block: true,
            },
        ]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        let mut replay2 = crate::drops::Lcg::new(0x5C02);
        let mut expected = 0usize;
        if replay2.next_f32() < 0.3 + 4.0 * 0.1 {
            let _ = replay2.next_f32();
            let _ = replay2.next_f32();
            expected += 0; // the voice spends no particle lane
        }
        let burst = crate::particles::sculk_charge_faces((9, 65, -3), 4, 0, &mut replay2);
        expected += burst.len();
        assert_eq!(
            spent2, replay2,
            "the gate + volume + pitch + face draws the order keeps"
        );
        assert_eq!(
            app.particles.len(),
            burst_before + expected,
            "the burst the seeded replay answers"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_sculk_charge_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_shriek_serve_the_ring_the_gate_wears() {
        // THE SCULK-SHRIEK SERVE'S OWN WIRE LANE (javap case 3007 →
        // offset 3193 over the 26.2 jar): the data word the case's
        // own body never reads, the ring the ten delayed members
        // speak (the 30 draws the provider chain spends — 3 per
        // member), the waterlogged-gated voice the arm's verdict
        // arms (the pitch draw the pass arm alone spends, the 2.0F
        // volume the armed max keeps), the empty mailbox the no-op
        // keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE DRY ARM (waterlogged = false): the ring + the voice the
        // gate arms — the 30 ring draws + the ONE pitch draw the walk
        // spends
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x5C03));
        let before = app.particles.len();
        app.live_sculk_shriek_events_updated(vec![
            crate::world_view::WireSculkShriekEvent {
                x: 9,
                y: 65,
                z: -3,
                waterlogged: false,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let mut replay = crate::drops::Lcg::new(0x5C03);
        for _ in 0..10 {
            let _ = replay.next_int(1);
            let _ = replay.next_f32();
            let _ = replay.next_f32();
        }
        let _ = replay.next_f32(); // the voice's own pitch draw
        assert_eq!(spent, replay, "the 31 draws the walk exactly spends");
        assert_eq!(
            app.particles.len(),
            before + 10,
            "the ten members the ring banks"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.sculk_shrieker.shriek @"))
                .expect("the shriek voice the dry arm speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (0.6..1.0).contains(&pitch),
                "the 0.6 + r * 0.4 band the voice keeps: {pitch}"
            );
        }
        // THE DROWNED ARM (waterlogged = true): the ring the silence
        // keeps — the voice the gate never arms, the 30 draws the
        // ring alone spends
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x5C04));
        let before2 = app.particles.len();
        let voices_before = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("block.sculk_shrieker.shriek @"))
            .count();
        app.live_sculk_shriek_events_updated(vec![
            crate::world_view::WireSculkShriekEvent {
                x: 1,
                y: 2,
                z: 3,
                waterlogged: true,
            },
        ]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        assert_eq!(
            spent2,
            {
                let mut r = crate::drops::Lcg::new(0x5C04);
                for _ in 0..10 {
                    let _ = r.next_int(1);
                    let _ = r.next_f32();
                    let _ = r.next_f32();
                }
                r
            },
            "the muted arm the pitch draw never spends"
        );
        assert_eq!(
            app.particles.len(),
            before2 + 10,
            "the ring the drowned arm still banks"
        );
        if sounds_loaded() {
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.sculk_shrieker.shriek @"))
                .count();
            assert_eq!(
                voices_after, voices_before,
                "the drowned shrieker the voice never arms"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_sculk_shriek_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_growl_serve_the_one_draw_the_1024_reach_wears() {
        // THE ENDER-DRAGON GROWL SERVE'S OWN WIRE LANE (javap case
        // 3001 → offset 5170 over the 26.2 jar): the data word the
        // case's own body never reads, the ONE voice the case body
        // speaks (the ONE pitch draw the walk spends — the
        // [0.8, 1.1] band the fold gives — the 64.0F volume the
        // armed max keeps), the NO-particles law the sound-only case
        // keeps, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE VOICE ARM: the ONE pitch draw + the ONE voice the walk
        // spends — the particles the sound-only case never banks
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x6301));
        let before = app.particles.len();
        app.live_dragon_growl_events_updated(vec![
            crate::world_view::WireDragonGrowlEvent {
                x: 41,
                y: 70,
                z: -9,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let mut replay = crate::drops::Lcg::new(0x6301);
        let _ = replay.next_f32(); // the voice's own pitch draw
        assert_eq!(spent, replay, "the ONE draw the walk exactly spends");
        assert_eq!(
            app.particles.len(),
            before,
            "the sound-only case the particles never banks"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.ender_dragon.growl @"))
                .expect("the growl voice the wire arm speaks");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (0.8..1.1).contains(&pitch),
                "the 0.8 + r * 0.3 band the voice keeps: {pitch}"
            );
        }
        // THE SECOND WORD: the fresh draw + the fresh voice the same
        // walk rides (the two words the two voices the arm ships)
        let saved2 = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x6302));
        let voices_before = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.ender_dragon.growl @"))
            .count();
        app.live_dragon_growl_events_updated(vec![
            crate::world_view::WireDragonGrowlEvent {
                x: -2,
                y: 64,
                z: 5,
            },
        ]);
        let spent2 = std::mem::replace(&mut app.level_ambient_rng, saved2);
        let mut replay2 = crate::drops::Lcg::new(0x6302);
        let _ = replay2.next_f32();
        assert_eq!(spent2, replay2, "the fresh word the ONE draw rides");
        if sounds_loaded() {
            let voices_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.ender_dragon.growl @"))
                .count();
            assert_eq!(voices_after, voices_before + 1, "the voice ships");
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        let voices = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.ender_dragon.growl @"))
            .count();
        app.live_dragon_growl_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("entity.ender_dragon.growl @"))
                .count(),
            voices,
            "the empty mailbox the voice never arms"
        );
    }


    #[test]
    fn the_brush_serve_the_voice_the_burst_wear() {
        // THE BRUSH-COMPLETE SERVE'S OWN WIRE LANE (javap case 3008 →
        // offset 1629 over the 26.2 jar): the state id the model's own
        // resolution resolves, the completed voice the brushable
        // verdict arms (the fixed 1.0/1.0 fold the `fconst_1` pair
        // keeps — NO randoms the fixed voice spends), the destroy
        // burst the BOTH branches reach (the 256 draws the 64-particle
        // terrain walk spends — the four draws the ctor chain costs
        // each), the non-brushable arm the voice never arms (the burst
        // the kind banks), the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BRUSHABLE ARM (suspicious_sand's own key): the ONE voice
        // the fixed fold speaks — the unbanked kind the burst never
        // finds, NO particles the absence banks, NO draws the fixed
        // voice and the absent burst never cost
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x6308));
        let saved_mob = std::mem::replace(&mut app.mob_rng, crate::drops::Lcg::new(0x6309));
        let before = app.particles.len();
        app.live_brush_complete_events_updated(vec![
            crate::world_view::WireBrushCompleteEvent {
                x: 3,
                y: 33,
                z: -7,
                brush_sound: Some("item.brush.brushing.sand"),
                burst_kind: None,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert_eq!(
            spent,
            crate::drops::Lcg::new(0x6308),
            "the fixed voice the ambient stream never spends"
        );
        let spent_mob = std::mem::replace(&mut app.mob_rng, saved_mob);
        assert_eq!(
            spent_mob,
            crate::drops::Lcg::new(0x6309),
            "the absent burst the terrain stream never spends"
        );
        assert_eq!(
            app.particles.len(),
            before,
            "the unbanked kind the burst never banks"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("item.brush.brushing.sand @"))
                .expect("the completed voice the verdict arms");
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (pitch - 1.0).abs() < 1e-6,
                "the fixed pitch the fconst_1 keeps: {pitch}"
            );
            let volume: f32 = voice
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 1.0).abs() < 1e-6,
                "the fixed volume the fconst_1 keeps: {volume}"
            );
        }
        // THE NON-BRUSHABLE ARM (the banked kind the burst fires
        // with): the voice the verdict never arms — the 64-particle
        // grid the terrain walk banks, the 256 draws the ctor chain
        // spends (four per particle)
        let saved_mob2 = std::mem::replace(&mut app.mob_rng, crate::drops::Lcg::new(0x6310));
        let before2 = app.particles.len();
        let voices_before = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("item.brush.brushing."))
            .count();
        app.live_brush_complete_events_updated(vec![
            crate::world_view::WireBrushCompleteEvent {
                x: -4,
                y: 20,
                z: 12,
                brush_sound: None,
                burst_kind: Some(7), // the sand tile the palette banks
            },
        ]);
        let spent_mob2 = std::mem::replace(&mut app.mob_rng, saved_mob2);
        let mut replay = crate::drops::Lcg::new(0x6310);
        for _ in 0..64 {
            let _ = replay.next_f32(); // the life draw
            let _ = replay.next_f32(); // the size draw
            let _ = replay.next_f32(); // the uo draw
            let _ = replay.next_f32(); // the vo draw
        }
        assert_eq!(
            spent_mob2, replay,
            "the 256 draws the terrain walk exactly spends"
        );
        assert_eq!(
            app.particles.len(),
            before2 + 64,
            "the 4x4x4 grid the full-cube burst banks"
        );
        if sounds_loaded() {
            assert_eq!(
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.starts_with("item.brush.brushing."))
                    .count(),
                voices_before,
                "the non-brushable verdict the voice never arms"
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_brush_complete_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_egg_serve_the_faces_the_draws_wear() {
        // THE EGG-CRACK SERVE'S OWN WIRE LANE (javap case 3009 →
        // offset 2576 over the 26.2 jar): the data word the arm never
        // carries, the six-face burst the `spawnParticlesOnBlockFaces`
        // walk speaks (the 18..36 band the `UniformInt.of(3, 6)`
        // counts answer — the nine draws the per-particle chain
        // spends: the supplier trio + the position pair + the glint
        // pick + the dead tint + the size + the lifetime), the
        // level-ambient stream the draws ride, the empty mailbox the
        // no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x6409));
        let before = app.particles.len();
        app.live_egg_crack_events_updated(vec![
            crate::world_view::WireEggCrackEvent {
                x: 12,
                y: 70,
                z: -44,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        assert!(
            app.particles.len() > before,
            "the six-face burst the serve banks"
        );
        // THE EXACT DRAW COUNT (the six count draws + the per-particle
        // nine the chain exactly spends — the next draw after the walk
        // differs from the fresh stream's same-index draw)
        let n = app.particles.len() - before;
        let mut replay = crate::drops::Lcg::new(0x6409);
        for _ in 0..(6 + n * 9) {
            let _ = replay.next();
        }
        assert_eq!(
            spent, replay,
            "the {} draws the faces walk exactly spends",
            6 + n * 9
        );
        // THE GLINT BANK (the kind the render arm's own tile law
        // wears — the burst's members the EggCrack kind carries)
        assert!(
            app.particles[before..]
                .iter()
                .all(|p| matches!(p.kind, crate::particles::ParticleKind::EggCrack)),
            "the burst's members the crack kind banks"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_egg_crack_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_trial_serve_the_burst_the_kinds_wear() {
        // THE TRIAL-SPAWNER SERVE'S OWN WIRE LANE (javap case 3011 →
        // offset 1827 over the 26.2 jar): the data word the decode
        // reads (the `FlameParticle.decode(I)` walk the case body
        // opens with), the forty-particle burst the
        // `TrialSpawner.addSpawnParticles` walk speaks (the twenty
        // SMOKE + flame pairs the `bipush 20` loop answers — the
        // twenty-seven draws the per-iteration chain spends: the
        // three nextDouble positions + the smoke's nine + the
        // flame's fifteen), the level-ambient stream the draws ride,
        // the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3011));
        let before = app.particles.len();
        app.live_trial_spawn_events_updated(vec![
            crate::world_view::WireTrialSpawnEvent {
                x: 12,
                y: 70,
                z: -44,
                data: 0,
                word: mc_protocol::PARTICLES_TRIAL_SPAWNER_SPAWN_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT FORTY (the twenty pairs the loop answers — the
        // even members the SMOKE companion, the odd members the
        // flame)
        let burst = &app.particles[before..];
        assert_eq!(burst.len(), 40, "the twenty pairs the serve banks");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(p.kind, crate::particles::ParticleKind::Smoke),
                    "pair {i} smoke first"
                );
            } else {
                assert!(
                    matches!(p.kind, crate::particles::ParticleKind::Flame),
                    "pair {i} flame"
                );
            }
        }
        // THE EXACT DRAW COUNT (the twenty-seven the per-iteration
        // chain exactly spends — the next draw after the walk differs
        // from the fresh stream's same-index draw)
        let mut replay = crate::drops::Lcg::new(0x3011);
        for _ in 0..(40 * 27 / 2) {
            let _ = replay.next();
        }
        assert_eq!(
            spent, replay,
            "the {} draws the add-spawn walk exactly spends",
            40 * 27 / 2
        );
        // THE OMINOUS WORD (the decode 1 the SoulFlame kind + the
        // FULL quad the plain Provider registration keeps — the
        // corrective byte audit the raw invokedynamic operands bank)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x640B));
        let before_soul = app.particles.len();
        app.live_trial_spawn_events_updated(vec![
            crate::world_view::WireTrialSpawnEvent {
                x: -8,
                y: 34,
                z: 90,
                data: 1,
                word: mc_protocol::PARTICLES_TRIAL_SPAWNER_SPAWN_EFFECT,
            },
        ]);
        let _ = std::mem::replace(&mut app.level_ambient_rng, saved);
        let soul_burst = &app.particles[before_soul..];
        assert_eq!(soul_burst.len(), 40, "the ominous word's own forty");
        let soul = soul_burst
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SoulFlame))
            .count();
        assert_eq!(soul, 20, "the decoded soul twin the render lane arms");
        // THE SPAWN-MOB VOICE TWIN (the 3012 word the SAME burst rides
        // beside — the case body's own draw order the pitch pair FIRST
        // keeps: TWO draws the voice spends, THEN the forty-particle
        // walk's 540 — the [0.8, 1.2] band the scale 0.2F fold gives,
        // the key the sounds.json row banks — the spawn1..4 files the
        // runtime table holds, the voice the log banks)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3012));
        let before_voice = app.particles.len();
        app.live_trial_spawn_events_updated(vec![
            crate::world_view::WireTrialSpawnEvent {
                x: 33,
                y: 41,
                z: -7,
                data: 0,
                word:
                    mc_protocol::PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_MOB_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let voice_burst = &app.particles[before_voice..];
        assert_eq!(voice_burst.len(), 40, "the voice twin's own forty");
        let mut replay = crate::drops::Lcg::new(0x3012);
        for _ in 0..(2 + 40 * 27 / 2) {
            let _ = replay.next();
        }
        assert_eq!(
            spent, replay,
            "the {} draws the voice twin exactly spends",
            2 + 40 * 27 / 2
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.trial_spawner.spawn_mob @"))
                .expect("the spawn-mob voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE SPAWN-ITEM VOICE TWIN (the 3021 word the SAME burst rides
        // beside — the OminousItemSpawner.spawnItem call the
        // `levelEvent(3021, blockPosition(), 1)` walk speaks, the data
        // word 1 the OMINOUS soul flavor the decode arms: the twenty
        // SoulFlame pairs the burst banks — the case body's own draw
        // order the pitch pair FIRST keeps: TWO draws the voice spends,
        // THEN the forty-particle walk's 540 — the [0.8, 1.2] band the
        // scale 0.2F fold gives, the key the sounds.json row banks —
        // the spawn_item1..3 files the runtime table holds, the voice
        // the log banks)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3021));
        let before_item = app.particles.len();
        app.live_trial_spawn_events_updated(vec![
            crate::world_view::WireTrialSpawnEvent {
                x: -19,
                y: 52,
                z: 61,
                data: 1,
                word:
                    mc_protocol::PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_ITEM_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let item_burst = &app.particles[before_item..];
        assert_eq!(item_burst.len(), 40, "the spawn-item twin's own forty");
        let soul = item_burst
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SoulFlame))
            .count();
        assert_eq!(
            soul, 20,
            "the OMINOUS word the caller's iconst_1 banks"
        );
        let mut replay = crate::drops::Lcg::new(0x3021);
        for _ in 0..(2 + 40 * 27 / 2) {
            let _ = replay.next();
        }
        assert_eq!(
            spent, replay,
            "the {} draws the spawn-item twin exactly spends",
            2 + 40 * 27 / 2
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.trial_spawner.spawn_item @"))
                .expect("the spawn-item voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_trial_spawn_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_detect_serve_the_voice_the_snowfall_wear() {
        // THE DETECT-PLAYER SERVE'S OWN WIRE LANE (javap case 3013 →
        // offset 1961 over the 26.2 jar): the amount word the walk
        // reads (the `addDetectPlayerParticles` amount arg the
        // detected player count carries), the voice the case body
        // plays FIRST (the TWO pitch draws the `(nf - nf) * 0.2F +
        // 1.0F` fold answers — the [0.8, 1.2] band the scale 0.2F
        // gives, the key the sounds.json row banks — the
        // detect_player1..3 files the runtime table holds), the
        // snowfall the walk's tail answers (the `30 + min(1, 10) * 5`
        // = 35 iterations, the NINE draws the per-iteration chain
        // spends: the three nextFloat positions + the six-draw
        // provider chain), the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3013));
        let before = app.particles.len();
        app.live_trial_detect_events_updated(vec![
            crate::world_view::WireDetectPlayerEvent {
                x: -12,
                y: 44,
                z: 88,
                amount: 1,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT SNOWFALL (the 35 members the loop bound arms —
        // every member the Snowflake kind the provider chain answers,
        // the near-white bluish tint the setColor constants keep)
        let snowfall = &app.particles[before..];
        assert_eq!(snowfall.len(), 35, "the 35-member snowfall");
        for p in snowfall {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Snowflake),
                "the snowflake kind the chain arms"
            );
            assert_eq!(
                p.col,
                [0.923, 0.964, 0.999],
                "the bluish-white the override keeps"
            );
        }
        // THE EXACT DRAW COUNT (the 2 voice draws the pitch pair FIRST
        // keeps + the thirty-five nine-draw iterations — the next draw
        // after the walk differs from the fresh stream's same-index
        // draw)
        let mut replay = crate::drops::Lcg::new(0x3013);
        for _ in 0..(2 + 35 * 9) {
            let _ = replay.next();
        }
        assert_eq!(
            spent, replay,
            "the {} draws the detect walk exactly spends",
            2 + 35 * 9
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.trial_spawner.detect_player @"))
                .expect("the detect-player voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE OMINOUS TWIN (the 3019 word the SAME serve answers — the
        // `isOminous()` branch the caller's sipush pair picks, the
        // SAME voice event the bytecodes pin, the SAME snowfall walk:
        // amount 2 → 40 iterations, the 2 + 360-draw replay the twin
        // exactly spends, the SAME bluish-white Snowflake kinds the
        // identical provider call site banks)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3019));
        let before_om = app.particles.len();
        app.live_trial_detect_events_updated(vec![
            crate::world_view::WireDetectPlayerEvent {
                x: 5,
                y: 66,
                z: -50,
                amount: 2,
            },
        ]);
        let spent_om = std::mem::replace(&mut app.level_ambient_rng, saved);
        let om_snow = &app.particles[before_om..];
        assert_eq!(om_snow.len(), 40, "the ominous word's own forty");
        for p in om_snow {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Snowflake),
                "the same snowflake kind the twin arms"
            );
            assert_eq!(
                p.col,
                [0.923, 0.964, 0.999],
                "the bluish-white the identical provider keeps"
            );
        }
        let mut replay_om = crate::drops::Lcg::new(0x3019);
        for _ in 0..(2 + 40 * 9) {
            let _ = replay_om.next();
        }
        assert_eq!(
            spent_om, replay_om,
            "the {} draws the ominous twin exactly spends",
            2 + 40 * 9
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_trial_detect_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_eject_serve_the_voice_the_burst_wear() {
        // THE EJECT SERVE'S OWN WIRE LANE (javap cases 3014/3017 →
        // offsets 2143/2193 over the 26.2 jar): the VOICE the 3014
        // case body plays FIRST (the TWO pitch draws the `(nf - nf) *
        // 0.2F + 1.0F` fold answers — the [0.8, 1.2] band the scale
        // 0.2F gives, the key the sounds.json row banks — the
        // eject_item files the runtime table holds), the burst BOTH
        // bodies answer (the `bipush 20` loop the forty-particle
        // answer wears: the twenty SMALL_FLAME (the zd * 0.25 fold) +
        // SMOKE pairs the per-iteration order keeps — the six walk
        // draws (three nextDouble positions + three nextGaussian
        // velocities, the polar-method rejection loop the variable
        // next() count wears) + the flame's fifteen + the smoke's
        // nine), the 3017 word the silence the case's own twelve
        // bytes keep, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3014));
        let before = app.particles.len();
        app.live_trial_eject_events_updated(vec![
            crate::world_view::WireTrialEjectEvent {
                x: -12,
                y: 44,
                z: 88,
                word: mc_protocol::PARTICLES_AND_SOUND_TRIAL_SPAWNER_EJECT_ITEM_EFFECT,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT FORTY (the twenty pairs the loop answers — the
        // even members the SMALL_FLAME companion, the odd members the
        // smoke)
        let burst = &app.particles[before..];
        assert_eq!(burst.len(), 40, "the twenty pairs the serve banks");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(p.kind, crate::particles::ParticleKind::SmallFlame),
                    "pair {i} small flame first"
                );
                assert!(
                    (0.05..0.11).contains(&p.quad_size),
                    "the scale(0.5F) half the birth band keeps: {}",
                    p.quad_size
                );
            } else {
                assert!(matches!(p.kind, crate::particles::ParticleKind::Smoke));
            }
        }
        // THE EXACT DRAW WALK (the voice's TWO draws FIRST + the
        // twenty-iteration chain the raw replay pins verbatim: per
        // iteration the three nextDouble positions + the three
        // nextGaussian velocities + the flame chain's fifteen (the
        // dead sprite pick, the dead base lifetime, the spread trio,
        // the d0 pair, the size, the six jitters, the own lifetime) +
        // the smoke chain's nine (the dead base lifetime, the spread
        // trio + d0 pair, the size, the tint, the own lifetime) — the
        // SAME stream the walk's own consumption answers, the state
        // equality the exact order banks)
        let mut replay = crate::drops::Lcg::new(0x3014);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        for _ in 0..20 {
            let _ = replay.next_f64();
            let _ = replay.next_f64();
            let _ = replay.next_f64();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            let _ = replay.next_gaussian();
            // the flame chain's fifteen
            let _ = replay.next_int(1);
            let _ = replay.next_f32();
            for _ in 0..5 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
            for _ in 0..6 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
            // the smoke chain's nine
            let _ = replay.next_f32();
            for _ in 0..8 {
                let _ = replay.next_f32();
            }
        }
        assert_eq!(
            spent, replay,
            "the walk's own exact stream the voice pair opens"
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.trial_spawner.eject_item @"))
                .expect("the eject-item voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE SILENT TWIN (the 3017 word the SAME serve answers — the
        // vault's own eject the caller's iconst_0 pins: NO pitch pair
        // the twelve-byte case keeps, the burst the same walk answers)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3017));
        let before_silent = app.particles.len();
        app.live_trial_eject_events_updated(vec![
            crate::world_view::WireTrialEjectEvent {
                x: 5,
                y: 66,
                z: -50,
                word: mc_protocol::PARTICLES_TRIAL_SPAWNER_EJECT_ITEM_EFFECT,
            },
        ]);
        let spent_silent = std::mem::replace(&mut app.level_ambient_rng, saved);
        let silent_burst = &app.particles[before_silent..];
        assert_eq!(silent_burst.len(), 40, "the silent twin's own forty");
        let small = silent_burst
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::SmallFlame))
            .count();
        assert_eq!(small, 20, "the same SMALL_FLAME pairs the twin arms");
        let mut replay_silent = crate::drops::Lcg::new(0x3017);
        for _ in 0..20 {
            let _ = replay_silent.next_f64();
            let _ = replay_silent.next_f64();
            let _ = replay_silent.next_f64();
            let _ = replay_silent.next_gaussian();
            let _ = replay_silent.next_gaussian();
            let _ = replay_silent.next_gaussian();
            // the flame chain's fifteen
            let _ = replay_silent.next_int(1);
            let _ = replay_silent.next_f32();
            for _ in 0..5 {
                let _ = replay_silent.next_f32();
            }
            let _ = replay_silent.next_f32();
            for _ in 0..6 {
                let _ = replay_silent.next_f32();
            }
            let _ = replay_silent.next_f32();
            // the smoke chain's nine
            let _ = replay_silent.next_f32();
            for _ in 0..8 {
                let _ = replay_silent.next_f32();
            }
        }
        assert_eq!(
            spent_silent, replay_silent,
            "the burst-only stream the silence keeps"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_trial_eject_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_vault_serve_the_bursts_the_voices_wear() {
        // THE VAULT WIRE LANE'S OWN SERVE (javap the case offsets
        // 2206/2306 — the burst the case body speaks FIRST, the voice
        // the case tail answers: the activation's 540-draw cage walk
        // (the twenty-seven-draw iteration chain the smoke-first pair
        // wears — the three nextDouble positions + the smoke's nine +
        // the flame's fifteen) + the deactivation's 420-draw center
        // walk (the twenty-one-draw iteration the one flame keeps),
        // the TWO pitch draws the voice tail spends AFTER, the
        // ominous gates the souls the kinds answer, the empty mailbox
        // the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3015));
        let before = app.particles.len();
        app.live_vault_events_updated(vec![crate::world_view::WireVaultEvent {
            x: -12,
            y: 44,
            z: 88,
            word: mc_protocol::PARTICLES_AND_SOUND_VAULT_ACTIVATE_EFFECT,
            soul: false,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT FORTY (the twenty pairs the loop answers — the
        // even members the SMOKE companion the walk's own call order
        // keeps FIRST, the odd members the small flame the gate's
        // data-0 pick answers)
        let burst = &app.particles[before..];
        assert_eq!(burst.len(), 40, "the twenty pairs the serve banks");
        for (i, p) in burst.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(p.kind, crate::particles::ParticleKind::Smoke),
                    "pair {i} smoke first"
                );
            } else {
                assert!(
                    matches!(p.kind, crate::particles::ParticleKind::SmallFlame),
                    "pair {i} small flame second"
                );
                assert!(
                    (0.05..0.11).contains(&p.quad_size),
                    "the scale(0.5F) half the birth band keeps: {}",
                    p.quad_size
                );
            }
        }
        // THE EXACT DRAW WALK (the burst's twenty-iteration chain
        // FIRST + the voice's TWO draws the tail answers — the burst
        // BEFORE the voice the case body's own order keeps: per
        // iteration the three nextDouble positions + the smoke
        // chain's nine + the flame chain's fifteen, then the pitch
        // pair — the SAME stream the walk's own consumption answers)
        let mut replay = crate::drops::Lcg::new(0x3015);
        for _ in 0..20 {
            let _ = replay.next_f64();
            let _ = replay.next_f64();
            let _ = replay.next_f64();
            // the smoke chain's nine
            let _ = replay.next_f32();
            for _ in 0..8 {
                let _ = replay.next_f32();
            }
            // the flame chain's fifteen
            let _ = replay.next_int(1);
            let _ = replay.next_f32();
            for _ in 0..5 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
            for _ in 0..6 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
        }
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(
            spent, replay,
            "the walk's own exact stream the burst opens"
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.vault.activate @"))
                .expect("the vault activate voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE OMINOUS DEACTIVATION (the 3016 word the SAME serve
        // answers — the soul gate the data-1 word arms, the
        // twenty-flame center walk the one-particle loop keeps, the
        // deactivate voice the case tail speaks)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3016));
        let before_decay = app.particles.len();
        app.live_vault_events_updated(vec![crate::world_view::WireVaultEvent {
            x: 5,
            y: 66,
            z: -50,
            word: mc_protocol::PARTICLES_AND_SOUND_VAULT_DEACTIVATE_EFFECT,
            soul: true,
        }]);
        let spent_decay = std::mem::replace(&mut app.level_ambient_rng, saved);
        let decay = &app.particles[before_decay..];
        assert_eq!(decay.len(), 20, "the twenty flames the twin banks");
        for p in decay.iter() {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::SoulFlame),
                "the soul word the ominous gate arms"
            );
            assert!(
                (0.1..0.22).contains(&p.quad_size),
                "the full quad the plain provider keeps: {}",
                p.quad_size
            );
        }
        let mut replay_decay = crate::drops::Lcg::new(0x3016);
        for _ in 0..20 {
            let _ = replay_decay.next_f64();
            let _ = replay_decay.next_f64();
            let _ = replay_decay.next_f64();
            let _ = replay_decay.next_gaussian();
            let _ = replay_decay.next_gaussian();
            let _ = replay_decay.next_gaussian();
            // the flame chain's fifteen
            let _ = replay_decay.next_int(1);
            let _ = replay_decay.next_f32();
            for _ in 0..5 {
                let _ = replay_decay.next_f32();
            }
            let _ = replay_decay.next_f32();
            for _ in 0..6 {
                let _ = replay_decay.next_f32();
            }
            let _ = replay_decay.next_f32();
        }
        let _ = replay_decay.next_f32();
        let _ = replay_decay.next_f32();
        assert_eq!(
            spent_decay, replay_decay,
            "the center walk's own stream the tail voice closes"
        );
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.vault.deactivate @"))
                .expect("the vault deactivate voice the lane logs");
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_vault_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_ominous_serve_the_volume_the_bursts_wear() {
        // THE OMINOUS-ACTIVATE WIRE LANE'S OWN SERVE (javap the case
        // offset 2069 — the voice the case body speaks FIRST (the
        // volume gate the `data != 0` ternary arms — the ONLY
        // 3000-band case whose volume the wire reads), the TWO pitch
        // draws the family's [0.8, 1.2] band keeps, THEN the
        // snowfall's 270-draw walk (the `iconst_0` amount word the
        // case's OWN body carries — the 30-iteration bound the
        // min-free answer keeps, the wire's data the volume gate
        // spent), THEN the become-ominous burst's 580-draw walk (the
        // twenty omen + soul pairs the omen-first order keeps — the
        // static jitter stream the super-arg pair spends), the empty
        // mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY AT THE ANCHOR (the ear the block center carries —
        // the block's own +0.5 trio the resolve speaks, the dist-0
        // ear the attenuation's own volume passthrough answers, the
        // gain the gate's raw word speaks)
        app.body = Some(crate::body::PlayerBody::new(
            -40.5,
            70.5 - crate::body::STANDING_EYE,
            123.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3020));
        let before = app.particles.len();
        app.live_trial_ominous_events_updated(vec![
            crate::world_view::WireTrialOminousEvent {
                x: -41,
                y: 70,
                z: 123,
                loud: true,
            },
        ]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT EIGHTY (the thirty-member snowfall + the fifty
        // pairs the walks answer — the snowfall FIRST the case's own
        // call order keeps, the omen puff the pair's own head wears,
        // the soul flame the tail speaks)
        let members = &app.particles[before..];
        assert_eq!(members.len(), 30 + 40, "the walks' own members");
        for p in &members[..30] {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Snowflake),
                "the snowfall the 30-iteration bound arms"
            );
        }
        for (i, p) in members[30..].iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    matches!(
                        p.kind,
                        crate::particles::ParticleKind::Spell { instant: false }
                    ),
                    "the omen first at pair {i}: {:?}",
                    p.kind
                );
            } else {
                assert!(
                    matches!(
                        p.kind,
                        crate::particles::ParticleKind::SoulFlame
                    ),
                    "the soul flame second at pair {i}"
                );
            }
        }
        // THE EXACT DRAW WALK (the voice's TWO pitch draws FIRST +
        // the snowfall's 30 x 9 chain + the burst's 20 x 29 chain —
        // the SAME order the case body's own consumption answers; the
        // static jitter stream the burst's forty-two draws ride)
        let mut replay = crate::drops::Lcg::new(0x3020);
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        for _ in 0..30 {
            // the detect walk's nine (the position pair + the height
            // + the provider chain's six)
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            for _ in 0..6 {
                let _ = replay.next_f32();
            }
        }
        for _ in 0..20 {
            // the become walk's six (the three nextDouble positions +
            // the three nextGaussian velocities)
            for _ in 0..3 {
                let _ = replay.next_f64();
            }
            for _ in 0..3 {
                let _ = replay.next_gaussian();
            }
            // the omen puff's eight level draws (the dead base
            // lifetime + the five-draw spread + the size + the
            // lifetime)
            let _ = replay.next_f32();
            for _ in 0..5 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
            let _ = replay.next_f32();
            // the flame chain's fifteen
            let _ = replay.next_int(1);
            let _ = replay.next_f32();
            for _ in 0..5 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
            for _ in 0..6 {
                let _ = replay.next_f32();
            }
            let _ = replay.next_f32();
        }
        assert_eq!(
            spent, replay,
            "the case body's own exact stream the serve opens"
        );
        if sounds_loaded() {
            // THE LOUD GATE (the dist-0 body the raw volume the gain
            // speaks — the data-1 word the 1.0F arms)
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.trial_spawner.ominous_activate @"))
                .expect("the ominous_activate voice the lane logs");
            assert!(
                line.contains(" v1.000000 p"),
                "the volume gate's own 1.0F the loud word answers: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE WHISPER (the data-0 word the SAME serve answers — the
        // 0.3F volume the gate keeps, the same 30+40 members the
        // walks answer)
        let saved = std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3021));
        let before_quiet = app.particles.len();
        app.live_trial_ominous_events_updated(vec![
            crate::world_view::WireTrialOminousEvent {
                x: -41,
                y: 70,
                z: 123,
                loud: false,
            },
        ]);
        let spent_quiet = std::mem::replace(&mut app.level_ambient_rng, saved);
        let quiet = &app.particles[before_quiet..];
        assert_eq!(quiet.len(), 30 + 40, "the same members the whisper arms");
        if sounds_loaded() {
            let line = app
                .sounds
                .events
                .iter()
                .rev()
                .find(|e| e.starts_with("block.trial_spawner.ominous_activate @"))
                .expect("the whisper voice the lane logs");
            assert!(
                line.contains(" v0.300000 p"),
                "the volume gate's own 0.3F the quiet word answers: {line}"
            );
        }
        // THE EXACT DRAW WALK THE WHISPER KEEPS (the SAME stream the
        // volume gate never changes — the pitch pair the same two
        // draws spends)
        let mut replay2 = crate::drops::Lcg::new(0x3021);
        let _ = replay2.next_f32();
        let _ = replay2.next_f32();
        for _ in 0..30 {
            for _ in 0..9 {
                let _ = replay2.next_f32();
            }
        }
        for _ in 0..20 {
            for _ in 0..3 {
                let _ = replay2.next_f64();
            }
            for _ in 0..3 {
                let _ = replay2.next_gaussian();
            }
            // the omen puff's eight level draws
            let _ = replay2.next_f32();
            for _ in 0..5 {
                let _ = replay2.next_f32();
            }
            let _ = replay2.next_f32();
            let _ = replay2.next_f32();
            // the flame chain's fifteen (the dead sprite pick the
            // next_int call answers — the SAME call shape the serve
            // keeps)
            let _ = replay2.next_int(1);
            let _ = replay2.next_f32();
            for _ in 0..5 {
                let _ = replay2.next_f32();
            }
            let _ = replay2.next_f32();
            for _ in 0..6 {
                let _ = replay2.next_f32();
            }
            let _ = replay2.next_f32();
        }
        assert_eq!(
            spent_quiet, replay2,
            "the whisper's own exact stream the same walks keep"
        );
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_trial_ominous_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_cobweb_serve_the_burst_the_voice_wear() {
        // THE COBWEB-SPAWN WIRE LANE'S OWN SERVE (javap the case
        // offset 2367 — the burst the case body speaks FIRST (the
        // `bipush 10` loop the ten-member answer wears — the corner
        // band the bare nextDouble keeps), the TWO pitch draws the
        // voice tail answers, the empty mailbox the no-op keeps.
        let mut app = App::new(2);
        app.enter_world();
        // THE BODY AT THE ANCHOR (the ear the block center carries —
        // the block's own +0.5 trio the resolve speaks, the dist-0
        // ear the attenuation's own volume passthrough answers, the
        // gain the fconst_1's raw word speaks)
        app.body = Some(crate::body::PlayerBody::new(
            11.5,
            40.5 - crate::body::STANDING_EYE,
            -19.5,
            0.0,
            0.0,
        ));
        let saved =
            std::mem::replace(&mut app.level_ambient_rng, crate::drops::Lcg::new(0x3018));
        let before = app.particles.len();
        app.live_cobweb_events_updated(vec![crate::world_view::WireCobwebEvent {
            x: 11,
            y: 40,
            z: -20,
        }]);
        let spent = std::mem::replace(&mut app.level_ambient_rng, saved);
        // THE EXACT TEN (the ten-member burst the walk's own bound
        // answers — the POOF kinds the getstatic arms, the corner
        // band the bare nextDouble keeps: [11, 12) x [40, 41) x
        // [-20, -19))
        let members = &app.particles[before..];
        assert_eq!(members.len(), 10, "the walk's own members");
        for (i, p) in members.iter().enumerate() {
            assert!(
                matches!(p.kind, crate::particles::ParticleKind::Poof),
                "the POOF the case's own getstatic arms at {i}: {:?}",
                p.kind
            );
            assert!(
                (11.0..12.0).contains(&p.x)
                    && (40.0..41.0).contains(&p.y)
                    && (-20.0..-19.0).contains(&p.z),
                "the corner band at {i}: {} {} {}",
                p.x,
                p.y,
                p.z
            );
        }
        // THE EXACT DRAW WALK (the burst's 10 x 14 chain FIRST + the
        // voice's TWO pitch draws AFTER — the SAME order the case
        // body's own consumption answers)
        let mut replay = crate::drops::Lcg::new(0x3018);
        for _ in 0..10 {
            // the walk's six level draws (the THREE gaussians FIRST —
            // the dstore trio the case body's own order keeps — the
            // THREE nextDoubles the arg walk interleaves)
            for _ in 0..3 {
                let _ = replay.next_gaussian();
            }
            for _ in 0..3 {
                let _ = replay.next_f64();
            }
            // the POOF chain's eight (the base lifetime + the THREE
            // jitters + the grey + the size's TWO + the lifetime)
            for _ in 0..8 {
                let _ = replay.next_f32();
            }
        }
        // the voice's TWO pitch draws (the `(nf - nf) * 0.2F + 1.0F`
        // fold the case tail answers — AFTER the burst the body's own
        // order keeps)
        let _ = replay.next_f32();
        let _ = replay.next_f32();
        assert_eq!(
            spent, replay,
            "the case body's own exact stream the serve opens"
        );
        if sounds_loaded() {
            // THE VOICE (the dist-0 body the raw volume the gain
            // speaks — the fconst_1 the place word arms)
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.cobweb.place @"))
                .expect("the cobweb place voice the lane logs");
            assert!(
                line.contains(" v1.000000 p"),
                "the fconst_1's own volume the place word answers: {line}"
            );
            let pitch: f32 = line
                .split(" p")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .expect("the pitch the log carries");
            assert!(
                (0.8..=1.2).contains(&pitch),
                "the 0.2F-scale pitch band: {}",
                pitch
            );
        }
        // THE SECOND ARM (the same walks the loop answers — ONE event
        // per PLACED cobweb the weaving effect's own death walk fires)
        let before2 = app.particles.len();
        app.live_cobweb_events_updated(vec![crate::world_view::WireCobwebEvent {
            x: 0,
            y: 0,
            z: 0,
        }]);
        let members2 = &app.particles[before2..];
        assert_eq!(members2.len(), 10, "the second arm's own ten");
        if sounds_loaded() {
            app.sounds
                .events
                .iter()
                .rev()
                .find(|e| e.starts_with("block.cobweb.place @"))
                .expect("the second voice the lane logs");
        }
        // THE EMPTY MAILBOX: the no-op the drained silence keeps
        let settled = app.particles.len();
        app.live_cobweb_events_updated(vec![]);
        assert_eq!(
            app.particles.len(),
            settled,
            "the empty mailbox ships nothing"
        );
    }


    #[test]
    fn the_wire_jukebox_words_the_record_and_line_wear() {
        // THE JUKEBOX PAIR'S OWN WIRE LANE (javap the cases
        // 1010/1011 → offsets 4405/4439 over the 26.2 jar): the play
        // word the ONE record instance banks (the `forJukeboxSong`
        // walk — the RECORDS 4.0/1.0 pair, the anchor's previous
        // record the `stopJukeboxSong` fold ends first), the
        // now-playing line the `setNowPlaying` walk banks (the
        // "Now Playing: %s" format the en_us table answers, the
        // animated color the `iconst_1` arm keeps), the stop word the
        // tracked record's own silence rides, the empty resolve the
        // `Optional.ifPresent` gate answers with NOTHING.
        let mut app = App::new(2);
        app.enter_world();
        let particles_before = app.particles.len();
        // THE PLAY WORD: the record instance banks + the line rides
        app.live_jukebox_events_updated(vec![
            crate::world_view::WireJukeboxEvent {
                x: 4,
                y: 60,
                z: -2,
                voice: mc_protocol::SOUND_PLAY_JUKEBOX_SONG_EFFECT,
                song: Some("minecraft:cat".to_string()),
            },
        ]);
        if sounds_loaded() {
            // the record line the positioned lanes' own shape wears
            // (the gain the 4.0F volume's own fold speaks — the armed
            // max the 64-block reach the `Math.max(v, 1.0F) *
            // getAttenuationDistance()` walk keeps)
            let line = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("music_disc.cat @"))
                .expect("the record's own play the lane logs");
            let volume: f32 = line
                .split(" v")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (volume - 4.0).abs() < 1e-6 || volume <= 1.0,
                "the 4.0 volume's gain the distance law folds: {volume}"
            );
            // the record banks (the `playingJukeboxSongs.put` walk)
            assert_eq!(
                app.sounds.records.len(),
                1,
                "the one record instance the play walk banks"
            );
            assert_eq!(
                app.sounds.records[0],
                ((4, 60, -2), "music_disc.cat".to_string()),
                "the anchor + the event key the bank keeps"
            );
        }
        // THE NOW-PLAYING LINE (the `record.nowPlaying` format the
        // description arg fills — the en_us rows the table answers)
        let msg = app.overlay_message.as_deref().unwrap_or("");
        assert!(
            msg == "Now Playing: C418 - cat"
                || msg.starts_with("Now Playing: ")
                || msg == "record.nowPlayingjukebox_song.minecraft.cat"
                || msg.contains("cat"),
            "the now-playing line the overlay banks: {msg}"
        );
        assert!(
            app.animate_overlay_message_color,
            "the animated color the iconst_1 arm keeps"
        );
        assert_eq!(
            app.overlay_message_time,
            crate::screens::OVERLAY_MESSAGE_LIFE,
            "the 60-tick life the bipush keeps"
        );
        assert_eq!(
            app.particles.len(),
            particles_before,
            "no particles the case bodies speak"
        );
        // THE SECOND PLAY AT THE SAME ANCHOR: the previous record the
        // play walk's own first fold ends (the stopJukeboxSong walk
        // the playJukeboxSong body spends) — the bank holds ONE
        if sounds_loaded() {
            let stops_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.ends_with(" stopped"))
                .count();
            app.live_jukebox_events_updated(vec![
                crate::world_view::WireJukeboxEvent {
                    x: 4,
                    y: 60,
                    z: -2,
                    voice: mc_protocol::SOUND_PLAY_JUKEBOX_SONG_EFFECT,
                    song: Some("minecraft:blocks".to_string()),
                },
            ]);
            assert_eq!(
                app.sounds.records.len(),
                1,
                "the second play the first record's death keeps"
            );
            assert_eq!(
                app.sounds.records[0].1, "music_disc.blocks",
                "the new instance the bank holds"
            );
            let stops_after = app
                .sounds
                .events
                .iter()
                .filter(|e| e.ends_with(" stopped"))
                .count();
            assert_eq!(
                stops_after,
                stops_before + 1,
                "the first fold's stop the log speaks"
            );
        }
        // THE STOP WORD: the tracked record's removal the non-null
        // gate keeps (the log line the stop speaks)
        if sounds_loaded() {
            app.live_jukebox_events_updated(vec![
                crate::world_view::WireJukeboxEvent {
                    x: 4,
                    y: 60,
                    z: -2,
                    voice: mc_protocol::SOUND_STOP_JUKEBOX_SONG_EFFECT,
                    song: None,
                },
            ]);
            assert!(
                app.sounds.records.is_empty(),
                "the stop walk the tracked record ends"
            );
            assert!(
                app.sounds
                    .events
                    .iter()
                    .any(|e| e.starts_with("music_disc.blocks @") && e.ends_with(" stopped")),
                "the stopped record the log carries"
            );
            // the stop at an anchor with no record the silent arm
            // keeps (the non-null gate — no line the nothing speaks)
            let stops_before = app.sounds.events.len();
            app.live_jukebox_events_updated(vec![
                crate::world_view::WireJukeboxEvent {
                    x: 99,
                    y: 99,
                    z: 99,
                    voice: mc_protocol::SOUND_STOP_JUKEBOX_SONG_EFFECT,
                    song: None,
                },
            ]);
            assert_eq!(
                app.sounds.events.len(),
                stops_before,
                "the absent record the silent arm keeps"
            );
        } else {
            // the no-cache boot: the walks still answer (the gate the
            // pick_file None keeps)
            app.live_jukebox_events_updated(vec![
                crate::world_view::WireJukeboxEvent {
                    x: 4,
                    y: 60,
                    z: -2,
                    voice: mc_protocol::SOUND_STOP_JUKEBOX_SONG_EFFECT,
                    song: None,
                },
            ]);
        }
        // THE EMPTY RESOLVE: the play word whose song the table never
        // named the `Optional.ifPresent` gate answers with NOTHING (no
        // record, no line the gate never walks)
        if sounds_loaded() {
            let plays_before = app
                .sounds
                .events
                .iter()
                .filter(|e| e.contains("music_disc."))
                .count();
            let msgs_before = app.overlay_message.is_some();
            app.live_jukebox_events_updated(vec![
                crate::world_view::WireJukeboxEvent {
                    x: 4,
                    y: 60,
                    z: -2,
                    voice: mc_protocol::SOUND_PLAY_JUKEBOX_SONG_EFFECT,
                    song: None,
                },
            ]);
            assert_eq!(
                app.sounds
                    .events
                    .iter()
                    .filter(|e| e.contains("music_disc."))
                    .count(),
                plays_before,
                "the empty resolve the play never spends"
            );
            assert_eq!(
                app.overlay_message.is_some(),
                msgs_before,
                "the empty resolve the line never banks"
            );
        }
        // the hsv law's own walk (javap `Mth.hsvToArgb` the six-arm
        // tableswitch keeps): h = 0.0 the case-0 arm the v/p/t
        // channels answer (the muted red the 0.7 saturation speaks —
        // the g/b pair the SAME p channel the f=0 fraction keeps),
        // h = 0.2 the case-1 arm the interpolated q the r picks
        let red = overlay_message_color(0.0);
        assert!(
            (red[0] - 0.6).abs() < 1e-6
                && red[1] < red[0]
                && red[2] < red[0]
                && (red[1] - red[2]).abs() < 1e-6,
            "the case-0 muted red the v/p pair speaks: {red:?}"
        );
        let amber = overlay_message_color(10.0);
        assert!(
            (amber[1] - 0.6).abs() < 1e-6
                && amber[2] < amber[1]
                && amber[0] > amber[2]
                && amber[0] < amber[1],
            "the case-1 sector the q interpolation keeps: {amber:?}"
        );
        // the unknown word the walk never spends (the model's own arm
        // the two words alone enqueues)
        app.live_jukebox_events_updated(vec![crate::world_view::WireJukeboxEvent {
            x: 0,
            y: 0,
            z: 0,
            voice: 999,
            song: None,
        }]);
        assert_eq!(
            app.particles.len(),
            particles_before,
            "the empty mailbox ships nothing"
        );
    }
