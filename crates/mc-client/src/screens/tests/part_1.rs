use super::*;


    /// THE MANNEQUIN'S QUAD PATH RIDES THE RAW PERIMETER ORDER (the
    /// world-side entity draw's own law): the mannequin chain PRESERVES
    /// the winding — the fold's x mirror and the vp's -z mirror cancel —
    /// so the body-front quad's two fan triangles BOTH pass the raster's
    /// front-face rule and the full face paints. The old 1<->2 corner
    /// swap flipped the winding into a BOWTIE: the corner walk crossed
    /// itself, one fan triangle culled, the survivor sampled sheared —
    /// half every face missing, and the armor's holed iron texture spoke
    /// the garble the plain skin hid. The test renders the CHEST slot's
    /// body-front quad ALONE through the mannequin vp over the real
    /// iron.png layer and pins the region's own vanilla shape: the
    /// neckline notch (the top-middle texels transparent), the dense
    /// band painted across BOTH halves, and the hem taper (the bottom
    /// texel row empty).
    #[test]
    fn the_body_front_quad_paints_both_fan_halves() {
        use mc_registry::generated::equipment;
        let root = ["assets-cache/jar", "../assets-cache/jar", "../../assets-cache/jar"]
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.join(equipment::HUMANOID_DIR).exists());
        let Some(root) = root else { return };
        let entry = equipment::find("iron").expect("the iron entry speaks");
        let Some(tex) = crate::avatar::ArmorTex::load(root, equipment::HUMANOID_DIR, entry)
        else {
            return; // the sha1 gate — the honest skip
        };
        let state = crate::avatar::AvatarState::rest(180.0, 0.0, 3.5);
        let vp = mannequin_vp(232, 97, 1.0, 640.0, 360.0, 0.0);
        let quad = crate::avatar::armor_quads(
            &state,
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
            crate::avatar::ARMOR_SLOT_CHEST,
        )
        .into_iter()
        .find(|q| q.uv == [[28.0, 20.0], [20.0, 20.0], [20.0, 32.0], [28.0, 32.0]])
        .expect("the body front quad speaks");
        // the quad's screen rect (the raster's own sx/sy walk)
        let scr = |p: [f32; 3]| -> (f32, f32) {
            let c = vp.transform_point(Vec3::new(p[0], p[1], p[2]));
            (
                (c[0] / c[3] * 0.5 + 0.5) * 640.0,
                (1.0 - (c[1] / c[3] * 0.5 + 0.5)) * 360.0,
            )
        };
        let (left, top) = scr(quad.pts[1]); // the LEFT-TOP corner
        let (right, bottom) = scr(quad.pts[3]); // the RIGHT-BOTTOM corner
        assert!(
            right - left > 15.0 && bottom - top > 20.0,
            "the projected rect speaks: ({left:.1},{top:.1})..({right:.1},{bottom:.1})"
        );
        let mut fb = crate::raster::Framebuf::new(640, 360);
        fb.clear_depth();
        fb.draw_pip_sampled_quads(
            vp,
            &[crate::raster::SampledQuad {
                pts: quad.pts,
                uv: quad.uv,
                tex: "f",
                tint: [quad.shade, quad.shade, quad.shade],
            }],
            &mut |_k, u, v| Some(tex.pixel(u, v)),
        );
        let px = |x: usize, y: usize| -> [u8; 3] {
            let o = (y * 640 + x) * 3;
            [fb.color[o], fb.color[o + 1], fb.color[o + 2]]
        };
        let painted = |x: usize, y: usize| -> bool {
            let c = px(x, y);
            (c[0] as i32 - c[1] as i32).abs() < 8
                && (c[1] as i32 - c[2] as i32).abs() < 8
                && c[0] > 60
        };
        // THE DENSE BAND (texel rows ~23..27 of the rect): paint on BOTH
        // fan halves — the bowtie regression kills one whole half
        let band_top = (top + 5.0) as usize;
        let band_bot = (top + 15.0) as usize;
        let mid = ((left + right) / 2.0) as usize;
        let mut l = 0usize;
        let mut r = 0usize;
        for y in band_top..band_bot {
            for x in (left as usize..mid).rev().take(4) {
                if painted(x, y) {
                    l += 1;
                }
            }
            for x in mid..(right as usize).min(mid + 4) {
                if painted(x, y) {
                    r += 1;
                }
            }
        }
        assert!(l > 10, "the LEFT fan half paints: {l}");
        assert!(r > 10, "the RIGHT fan half paints: {r}");
        // THE NECKLINE NOTCH (the top-middle texels transparent): the
        // top rows' middle paints nothing
        let mut notch = 0usize;
        for y in (top as usize)..(top as usize + 2) {
            for x in (mid - 1)..(mid + 2) {
                if painted(x, y) {
                    notch += 1;
                }
            }
        }
        assert_eq!(notch, 0, "the neckline notch stays transparent: {notch}");
        // THE HEM (the bottom texel row fully transparent): the last
        // screen row before the rect's bottom edge paints nothing
        let mut hem = 0usize;
        for x in left as usize..right as usize {
            if painted(x, bottom as usize - 1) {
                hem += 1;
            }
        }
        assert_eq!(hem, 0, "the hem row stays transparent: {hem}");
    }


    /// THE FIXED CLOCK'S SPEND LAW (vanilla `Minecraft.runTick`'s own
    /// while loop): whole 50ms ticks spent, the sub-tick remainder
    /// carried, the 10-tick stall cap holds the spiral of death away,
    /// the backwards clock spends nothing.
    #[test]
    fn the_fixed_clock_spends_whole_ticks_and_carries_the_remainder() {
        // the empty frame: nothing accrues, nothing spends
        assert_eq!(ticks_due(0.0), (0, 0.0));
        // the backwards jump (a suspended tab's clock): spends nothing,
        // resets clean
        assert_eq!(ticks_due(-30.0), (0, 0.0));
        // the sub-tick frame: the remainder carries, no tick yet
        let (n, rem) = ticks_due(30.0);
        assert_eq!(n, 0);
        assert!((rem - 30.0).abs() < 1e-9);
        // the exact tick: one spent, nothing carried
        assert_eq!(ticks_due(50.0), (1, 0.0));
        // the drift frame: one spent, 10ms toward the next
        assert_eq!(ticks_due(60.0), (1, 10.0));
        // the multi-tick catch-up: two whole ticks, the tail carried
        assert_eq!(ticks_due(120.0), (2, 20.0));
        // the STALL CAP (vanilla's own max): 4000ms of wall time spends
        // at most ten ticks — the time dilates, the loop never spirals
        let (n, rem) = ticks_due(4000.0);
        assert_eq!(n, MAX_CATCHUP_TICKS);
        assert!((rem - 0.0).abs() < 1e-9);
        // the partial tick the render rides: the remainder's fraction
        assert!((ticks_due(60.0).1 as f32 / MILLIS_PER_TICK as f32 - 0.2).abs() < 1e-6);
    }


    #[test]
    fn chest_geometry_carries_the_generic_54_layout() {
        // the 176x166 canvas centers in the 320x180 GUI workspace
        assert_eq!(chest_panel_origin(320, 180), (72, 7));
        // the 9x3 container grid: row-major from (8, 18), the 18 pitch,
        // the row starts at the panel-relative x 8 and y 18 / 36 / 54
        let p = (0, 0);
        assert_eq!(chest_container_slot(p, 0), (8, 18));
        assert_eq!(chest_container_slot(p, 8), (8 + 8 * 18, 18));
        assert_eq!(chest_container_slot(p, 9), (8, 36));
        assert_eq!(chest_container_slot(p, 26), (8 + 8 * 18, 54));
        // the player mirror: three rows at 84 / 102 / 120, the hotbar
        // row at 142 — the gap between the grids kept (the vanilla
        // canvas's own numbers)
        assert_eq!(chest_mirror_slot(p, 0), (8, 84));
        assert_eq!(chest_mirror_slot(p, 8), (8 + 8 * 18, 84));
        assert_eq!(chest_mirror_slot(p, 26), (8 + 8 * 18, 120));
        assert_eq!(chest_mirror_slot(p, 27), (8, 142));
        assert_eq!(chest_mirror_slot(p, 35), (8 + 8 * 18, 142));
        // the panel bottom keeps the mirror's last row inside: 142 + 18
        // slot height + the bevel lands exactly on the 166 canvas
        assert_eq!(chest_mirror_slot(p, 27).1 + 18 + 6, 166);
    }


    #[test]
    fn the_chest_loadout_is_fixed_and_uses_registry_ids() {
        let mut app = App::new(20260214);
        app.open_chest();
        // the screen rode open; the world underneath came with it
        assert_eq!(app.screen, ScreenKind::Chest);
        assert_eq!(app.chest_slots.len(), 27);
        // the loadout is a CONSTANT (no RNG): two opens, one loadout
        let again = {
            let mut a2 = App::new(7); // a DIFFERENT seed — the loadout holds
            a2.open_chest();
            a2.chest_slots.clone()
        };
        assert_eq!(app.chest_slots, again);
        // the pins: the ids resolve through the registry's own order
        // and the counts ride the wire's shape (positive, the rest
        // empty via the count<=0 sentinel)
        assert_eq!(app.chest_slots[4], (926, 12)); // diamond x12
        assert_eq!(app.chest_slots[13], (932, 64)); // iron_ingot x64
        assert_eq!(mc_registry::item_name(926), Some("diamond"));
        assert_eq!(mc_registry::item_name(932), Some("iron_ingot"));
        let filled = app.chest_slots.iter().filter(|(_, c)| *c > 0).count();
        assert_eq!(filled, 8);
    }


    /// THE CHEST'S OWN PER-CELL STORE (the dev-66 round — the placed
    /// chest the 27-slot contents the session OUTLIVES: the open the
    /// entry banks, the edits the close arms the flush ride home, the
    /// reopen the same stacks load; the DIFFERENT cell the fresh grid
    /// keeps — the vanilla RandomizableContainer BlockEntity's own
    /// per-position law the map key wears).
    #[test]
    fn the_chests_own_store_the_edits_the_reopen_the_same_stacks_bank() {
        let mut app = App::new(11);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::CHEST);
        // THE FIRST OPEN the fresh grid inserts (the empty 27 the
        // ChestMenu.threeRows ctor speaks)
        app.open_chest_for(cell);
        assert_eq!(app.screen, ScreenKind::Chest);
        assert_eq!(app.chest_slots, vec![(0, 0); 27]);
        assert!(app.container_stores.contains_key(&cell));
        // THE EDITS: a grid stack + the real inventory's hotbar row
        app.chest_slots[5] = (926, 12); // diamond x12
        app.inventory.items[0] = (62, 27); // cobblestone on the hotbar
        app.chest_mirror_sync_from_inventory();
        assert_eq!(app.chest_mirror[27], (62, 27), "the hotbar row speaks");
        // THE CLOSE the flush banks
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World);
        assert_eq!(app.container_stores[&cell][5], (926, 12));
        // THE REOPEN the same stacks load
        app.open_chest_for(cell);
        assert_eq!(app.chest_slots[5], (926, 12));
        assert_eq!(app.chest_mirror[27], (62, 27), "the real rows persist");
        app.on_escape();
        // A DIFFERENT CELL the fresh grid keeps (the barrel the same
        // lane serves — the 27 the barrel's EXACT count the vanilla
        // menu wears)
        let cell2 = (c + 2, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell2.0, cell2.1, cell2.2, crate::world::BARREL);
        app.open_chest_for(cell2);
        assert_eq!(app.chest_slots[5], (0, 0), "the fresh cell empty");
        assert!(app.container_stores.contains_key(&cell2));
        assert!(app.container_stores[&cell].iter().any(|&p| p == (926, 12)));
        app.on_escape();
        assert_eq!(app.container_stores[&cell][5], (926, 12), "the barrel's close the chest's store never touches");
    }


    /// THE COMPARATOR'S OWN CONTAINER LANE (the container-analog
    /// round — the flush walk the fill banks, the front comparator the
    /// consult schedules, the beat the variant the output wears, the
    /// break the entry's death the survivor the own-half law banks:
    /// javap ComparatorBlock.getInputSignal + the stopOpen tail the
    /// 26.2 jar the whole felt behavior the offline walk replays).
    #[test]
    fn the_comparator_the_container_flush_the_fill_the_consult_and_the_break_wear() {
        // THE SINGLE'S OWN LANE (the chest the comparator's back
        // serves — the facing byte 1 the +x back the delta names)
        let mut app = App::new(11);
        app.enter_world();
        let cell = (5i32, 10i32, 5i32);
        let comp = (4i32, 10i32, 5i32);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(cell.0, cell.1, cell.2, crate::world::CHEST);
            w.set(comp.0, comp.1, comp.2, crate::world::COMPARATOR);
            w.set_variant(
                comp.0,
                comp.1,
                comp.2,
                crate::world::comparator_variant(1, false, false, 0),
            );
        }
        // THE STOCK + FLUSH WALK (the GUI's own lane the edits ride —
        // the lone stack the close the flush banks, the fill the
        // formula the +1 floor wears)
        app.open_chest_for(cell);
        app.chest_slots[0] = (926, 64);
        app.container_store_flush();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.container_fill.get(&cell).copied(),
                Some(1),
                "the lone stack the floor-0 + 1 wears"
            );
            assert!(
                !w.comparator_ticks.is_empty(),
                "the front comparator the consult schedules"
            );
        }
        // THE BEAT'S OWN ANSWER (the tick walk the refresh banks — the
        // output cache the variant wears, the powered bit the 1>0 law
        // answers)
        let due = app
            .offline_world
            .as_ref()
            .unwrap()
            .comparator_ticks
            .first()
            .map(|e| e.0)
            .expect("the due beat");
        app.frame = due;
        app.tick_comparator_beats();
        {
            let w = app.offline_world.as_ref().unwrap();
            let v = w.variant(comp.0, comp.1, comp.2);
            assert_eq!(
                crate::world::comparator_output_of(v),
                1,
                "the beat the output cache the fill banks"
            );
            assert!(
                crate::world::comparator_powered_of(v),
                "the powered bit the i>j verdict answers"
            );
        }
        // THE BREAK'S OWN DEATH RIDE (the spill the entry the air
        // drops — the store the map's removal keeps, the floor the
        // stocked stack wears)
        app.container_store_spilled(cell);
        {
            let w = app.offline_world.as_ref().unwrap();
            assert!(
                !w.container_fill.contains_key(&cell),
                "the broken chest the entry the air drops"
            );
            assert!(
                !app.container_stores.contains_key(&cell),
                "the store the removal keeps"
            );
            assert!(!app.drops.is_empty(), "the stocked stack the floor wears");
        }
        // THE DOUBLE'S OWN UNION (the two halves the one 54-slot read
        // — the partner the connected direction names, both fills the
        // same total banks, the break the survivor the own-half law
        // the 15 caps)
        let mut app2 = App::new(11);
        app2.enter_world();
        let facing = 1u8;
        let a = (5i32, 10i32, 7i32);
        let va = crate::world::chest_variant(facing, crate::world::CHEST_TYPE_LEFT);
        let dir = crate::world::chest_connected_direction(facing, crate::world::CHEST_TYPE_LEFT);
        let (dx, dz) = crate::world::stairs_facing_delta(dir);
        let b = (a.0 + dx, a.1, a.2 + dz);
        let vb = crate::world::chest_variant(facing, crate::world::CHEST_TYPE_RIGHT);
        {
            let w = app2.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::CHEST);
            w.set_variant(a.0, a.1, a.2, va);
            w.set(b.0, b.1, b.2, crate::world::CHEST);
            w.set_variant(b.0, b.1, b.2, vb);
        }
        app2.open_chest_for(a);
        assert_eq!(
            app2.chest_slots.len(),
            54,
            "the compound the 54-slot grid wears"
        );
        for slot in app2.chest_slots.iter_mut().take(27) {
            *slot = (926, 64);
        }
        app2.chest_slots[27] = (926, 64);
        app2.container_store_flush();
        {
            let w = app2.offline_world.as_ref().unwrap();
            assert_eq!(
                w.container_fill.get(&a).copied(),
                Some(8),
                "the half-full double the floor(0.5*14)+1 the 8 wears"
            );
            assert_eq!(
                w.container_fill.get(&b).copied(),
                Some(8),
                "the far face the same union the map banks"
            );
        }
        app2.container_store_spilled(a);
        {
            let w = app2.offline_world.as_ref().unwrap();
            assert!(
                !w.container_fill.contains_key(&a),
                "the broken half the entry the air drops"
            );
            assert_eq!(
                w.container_fill.get(&b).copied(),
                Some(15),
                "the survivor the own-half law the 15 caps"
            );
        }
    }


    /// THE FURNACE'S OWN TRIO THE FILL WEARS (the container-analog
    /// lane's furnace half — javap 26.2 AbstractFurnaceBlock the
    /// `getAnalogOutputSignal` walk the getRedstoneSignalFromBlock-
    /// Entity answers, the `instanceof Container` gate the furnace BE
    /// passes, the SAME lerpDiscrete formula over the THREE slots the
    /// FurnaceMenu ctor wears: the lone input the 5, the stocked trio
    /// the 8, the empty the 0 the presence keeps, the break the entry
    /// the air drops — the flush the front comparator's consult
    /// schedules, the beat the output cache the fill banks).
    #[test]
    fn the_furnace_the_trio_the_fill_the_flush_the_beat_and_the_break_wear() {
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(11);
        app.enter_world();
        let cell = (5i32, 10i32, 5i32);
        let comp = (4i32, 10i32, 5i32);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(cell.0, cell.1, cell.2, crate::world::FURNACE);
            w.set(comp.0, comp.1, comp.2, crate::world::COMPARATOR);
            w.set_variant(
                comp.0,
                comp.1,
                comp.2,
                crate::world::comparator_variant(1, false, false, 0),
            );
        }
        // THE EMPTY'S OWN PRESENCE (the fresh trio the open loads, the
        // flush the 0-fill the map banks — the entry the hasAnalog
        // arm's verdict keeps, the vanilla getRedstoneSignalFrom-
        // Container the all-empty walk the 0 speaks)
        app.open_furnace(cell);
        assert_eq!(
            app.furnace_slots.len(),
            39,
            "the fresh grid the FurnaceMenu law keeps"
        );
        app.furnace_store_flush();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.container_fill.get(&cell).copied(),
                Some(0),
                "the empty furnace the 0 the presence banks"
            );
        }
        // THE STOCKED TRIO'S OWN FILL (the reopen the stocked slots
        // wear — wait, the fresh open the empty store loads; the
        // reopen below the flushed trio reads. THE FORMULA the 131
        // walk wears: (32/64 + 64/64 + 13/64)/3 the sum the slot
        // count divides, the floor(7.947)*14 fold +1 the 8 speaks)
        app.open_furnace(cell);
        app.furnace_slots[0] = (wire_id_of("raw_iron"), 32);
        app.furnace_slots[1] = (wire_id_of("coal"), 64);
        app.furnace_slots[2] = (wire_id_of("iron_ingot"), 13);
        app.furnace_store_flush();
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                w.container_fill.get(&cell).copied(),
                Some(8),
                "the stocked trio the floor(0.5677*14)+1 the 8 wears"
            );
            assert!(
                !w.comparator_ticks.is_empty(),
                "the front comparator the consult schedules"
            );
        }
        // THE BEAT'S OWN ANSWER (the tick walk the refresh banks —
        // the output cache the variant wears, the powered bit the
        // 8>0 law answers)
        let due = app
            .offline_world
            .as_ref()
            .unwrap()
            .comparator_ticks
            .first()
            .map(|e| e.0)
            .expect("the due beat");
        app.frame = due;
        app.tick_comparator_beats();
        {
            let w = app.offline_world.as_ref().unwrap();
            let v = w.variant(comp.0, comp.1, comp.2);
            assert_eq!(
                crate::world::comparator_output_of(v),
                8,
                "the beat the output cache the trio fill banks"
            );
            assert!(
                crate::world::comparator_powered_of(v),
                "the powered bit the i>j verdict answers"
            );
        }
        // THE BREAK'S OWN DEATH RIDE (the spill the trio the floor
        // wears, the fill the entry the air drops — the words the
        // same death the removal keeps)
        app.furnace_store_spilled(cell);
        {
            let w = app.offline_world.as_ref().unwrap();
            assert!(
                !w.container_fill.contains_key(&cell),
                "the broken furnace the entry the air drops"
            );
            assert!(
                !app.furnace_stores.contains_key(&cell),
                "the trio the removal keeps"
            );
            assert!(!app.drops.is_empty(), "the stocked trio the floor wears");
        }
    }


    /// THE FILL'S OWN RIDE LANES (the furnace analog's walk rides —
    /// the boot parse the restart banks, the lit flip the tick walk
    /// re-reads [the vanilla setBlock flag-3 law the pc 399 walk
    /// speaks], the switch the prev cell's trio the re-bank serves).
    #[test]
    fn the_furnace_fill_the_boot_the_flip_and_the_switch_rides_wear() {
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE BOOT'S OWN PARSE RIDE (the disk bank the restart wears —
        // the flush the file write banks, the FRESH APP the same dir
        // the load-once parse rides, the stocked furnace's fill the
        // comparator's next consult the map serves BEFORE any open)
        let dir = std::env::temp_dir().join(format!("mc2rs-furnfill-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("furnfill-round.nbt");
        let _ = std::fs::remove_file(&file);
        let mut app = App::new(11);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        app.open_furnace(cell);
        app.furnace_slots[0] = (wire_id_of("raw_iron"), 64);
        app.furnace_store_flush();
        assert!(file.exists(), "the flush the file write banks");
        // THE RESTART: the fresh app the once-flag's parse the fills
        // bank — the lone stack (64/64)/3 the floor(4.667)+1 the 5
        // wears, the comparator's consult the map the open never rode
        let mut fresh = App::new(11);
        fresh.enter_world();
        fresh.container_stores_file = Some(file.clone());
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        fresh.container_stores_load_once();
        {
            let w = fresh.offline_world.as_ref().unwrap();
            assert_eq!(
                w.container_fill.get(&cell).copied(),
                Some(5),
                "the boot parse the stocked furnace's fill banks"
            );
        }
        // THE LIT FLIP'S OWN RE-READ (the world walk the tick serves —
        // the banked trio the raw banks ride, the update the sim tick
        // the fuel consumes, the LIT flip the setBlock flag-3 law the
        // re-read serves: the fill re-banks, the front comparator the
        // consult schedules, the beat the output cache the fresh fill
        // wears)
        let cell2 = (c, surface + 1, c + 3);
        let comp2 = (c - 1, surface + 1, c + 3);
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(cell2.0, cell2.1, cell2.2, crate::world::FURNACE);
        fresh.offline_world.as_mut().unwrap().set(
            comp2.0,
            comp2.1,
            comp2.2,
            crate::world::COMPARATOR,
        );
        fresh.offline_world.as_mut().unwrap().set_variant(
            comp2.0,
            comp2.1,
            comp2.2,
            crate::world::comparator_variant(1, false, false, 0),
        );
        fresh.furnace_stores.insert(
            cell2,
            vec![
                (wire_id_of("raw_iron"), 16),
                (wire_id_of("coal"), 1),
                (0, 0),
            ],
        );
        fresh.furnace_word_stores.insert(cell2, [0, 0, 0, 0]);
        fresh.update();
        {
            let w = fresh.offline_world.as_ref().unwrap();
            // the consumed fuel the post-tick trio wears — (16/64)/3
            // the same floor(1.167)+1 the 2 speaks either arm
            assert_eq!(
                w.container_fill.get(&cell2).copied(),
                Some(2),
                "the flip the tick-worn trio the fill re-banks"
            );
            assert!(
                w.comparator_ticks
                    .iter()
                    .any(|e| e.1 == comp2.0 && e.2 == comp2.1 && e.3 == comp2.2),
                "the flip the front comparator the consult schedules"
            );
            assert_eq!(
                w.get(cell2.0, cell2.1, cell2.2),
                crate::world::FURNACE_LIT,
                "the lit flip the world's own kind wears"
            );
        }
        // THE BEAT'S OWN ANSWER (the flip's consult the output wears)
        let due2 = fresh
            .offline_world
            .as_ref()
            .unwrap()
            .comparator_ticks
            .first()
            .map(|e| e.0)
            .expect("the due beat");
        fresh.frame = due2;
        fresh.tick_comparator_beats();
        {
            let w = fresh.offline_world.as_ref().unwrap();
            let v = w.variant(comp2.0, comp2.1, comp2.2);
            assert_eq!(
                crate::world::comparator_output_of(v),
                2,
                "the beat the output cache the flip's fill banks"
            );
        }
        // THE SWITCH'S OWN PREV RIDE (the open-another walk the banked
        // trio the fill re-banks — the edited slots the flush never
        // rode, the prev cell's fill the switch the same law banks)
        fresh.open_furnace(cell2); // the live cell the fresh trio keeps
        fresh.furnace_slots[1] = (wire_id_of("coal"), 64);
        let cell3 = (c, surface + 1, c + 6);
        fresh.offline_world.as_mut().unwrap().set(
            cell3.0,
            cell3.1,
            cell3.2,
            crate::world::BLAST_FURNACE,
        );
        fresh.open_furnace(cell3);
        {
            let a = fresh.furnace_stores.get(&cell2).expect("the prev trio");
            assert_eq!(
                a[1],
                (wire_id_of("coal"), 64),
                "the prev trio the switch banks"
            );
            let w = fresh.offline_world.as_ref().unwrap();
            // (16/64 + 64/64)/3 the floor(5.833)+1 the 6 speaks
            assert_eq!(
                w.container_fill.get(&cell2).copied(),
                Some(6),
                "the prev cell's fill the switch re-banks"
            );
        }
        let _ = std::fs::remove_file(&file);
        let _ = std::fs::remove_dir(&dir);
    }


    /// THE STORES' OWN DISK BANK (the dev-68 round — the stocked grid
    /// the RESTART OUTLIVES: javap HotbarManager the 26.2 walk the
    /// save chord's `HotbarManager.save()` pc 199 the file writes, the
    /// fresh app's `get(i)` once-flag pc 1-13 the lazy load speaks —
    /// the close walk's flush the file ride banks, the NEW app the
    /// same game dir the banked stacks the reopen wears, the break-
    /// after-boot the banked contents the spill releases — the REAL
    /// Minecraft law the restart keeps).
    #[test]
    fn the_stores_own_disk_bank_the_edits_the_restart_the_same_stacks_wear() {
        let dir = std::env::temp_dir().join(format!("mc2rs-containers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("containers-round.nbt");
        let _ = std::fs::remove_file(&file);
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE FIRST SESSION: the open the fresh grid keeps (the
        // missing file the honest empties wear), the edits the close
        // bank — AND the flush the file write rides home
        let mut app = App::new(11);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::CHEST);
        app.open_chest_for(cell);
        assert!(
            app.container_stores_loaded,
            "the once-flag the first access banks"
        );
        app.chest_slots[5] = (wire("diamond"), 12);
        app.on_escape();
        assert!(file.exists(), "the flush the file write banks");
        // THE FILE'S OWN SHAPE (the DataVersion header + the entry the
        // x/y/z triple keeps + the stocked slot the minecraft: pair
        // speaks, the empty slots the OPTIONAL empties wear)
        let bytes = std::fs::read(&file).expect("the banked file reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        assert_eq!(
            root.get("DataVersion").and_then(|v| v.as_i32()),
            Some(CONTAINERS_DATA_VERSION),
            "the header the era banks"
        );
        let entries = root
            .get("entries")
            .and_then(|v| v.as_list())
            .expect("the entries list");
        assert_eq!(entries.len(), 1, "the one stocked cell rides");
        let entry = &entries[0];
        assert_eq!(entry.get("x").and_then(|v| v.as_i32()), Some(cell.0));
        assert_eq!(entry.get("y").and_then(|v| v.as_i32()), Some(cell.1));
        assert_eq!(entry.get("z").and_then(|v| v.as_i32()), Some(cell.2));
        let items = entry.get("Items").and_then(|v| v.as_list()).expect("the Items list");
        assert_eq!(items.len(), 27, "the 27-slot grid the list keeps");
        assert!(
            items[0].get("id").is_none(),
            "the empty slot the OPTIONAL empty compound wears"
        );
        assert_eq!(
            items[5].get("id").and_then(|v| v.as_str()),
            Some("minecraft:diamond"),
            "the stocked slot the name bridge wears"
        );
        assert_eq!(items[5].get("count").and_then(|v| v.as_i32()), Some(12));
        // THE RESTART: the fresh app the same game dir carries — the
        // first open the once-flag's lazy read speaks, the banked
        // stack the reopen wears
        let mut fresh = App::new(11);
        fresh.enter_world();
        fresh.container_stores_file = Some(file.clone());
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::CHEST);
        fresh.open_chest_for(cell);
        assert_eq!(
            fresh.chest_slots[5],
            (wire("diamond"), 12),
            "the banked stack the restart wears"
        );
        fresh.on_escape();
        // THE BREAK-AFTER-RESTART (the stocked chest's death the floor
        // the items wear — the spill the banked contents releases, the
        // entry's removal the file's death ride banks)
        fresh.container_store_spilled(cell);
        assert!(
            !fresh.drops.is_empty(),
            "the banked contents the spill wears"
        );
        assert!(
            !fresh.container_stores.contains_key(&cell),
            "the store the entry's death keeps"
        );
        let bytes = std::fs::read(&file).expect("the death ride reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let entries = root
            .get("entries")
            .and_then(|v| v.as_list())
            .expect("the entries list");
        assert!(
            entries.is_empty(),
            "the death ride the emptied bank keeps"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE DISK BANK'S OWN REFUSALS — the tolerant walk the hotbar
    /// parse wears: the non-compound root the empties keep, the
    /// missing coordinate the entry skip keeps, the unknown name the
    /// honest empty stack wears (never a guessed id), the missing
    /// count the fallback-1 field keeps, the over-long Items the
    /// 27-slot cap truncates, the corrupt file the lazy load never
    /// wears, the NONE boot the file lanes never spends.
    #[test]
    fn the_stores_own_disk_parse_the_refusals_keep() {
        // THE NON-COMPOUND ROOT (the int the entries never parse)
        let stores = container_stores_from_nbt(&mc_protocol::nbt::Nbt::Int(1));
        assert!(stores.is_empty(), "the non-compound the empties keep");
        // THE TOLERANT ENTRY WALK: the coordinate-less entry skips,
        // the unknown name empties, the missing count falls back 1
        let root = mc_protocol::nbt::Nbt::Compound(vec![(
            "entries".to_string(),
            mc_protocol::nbt::Nbt::List(vec![
                // the x/y/z-less row the skip keeps
                mc_protocol::nbt::Nbt::Compound(vec![(
                    "Items".to_string(),
                    mc_protocol::nbt::Nbt::List(vec![]),
                )]),
                // the stocked row: slot 0 unknown (empty keeps), slot 1
                // missing count (fallback-1), slot 2 stocked
                mc_protocol::nbt::Nbt::Compound(vec![
                    ("x".to_string(), mc_protocol::nbt::Nbt::Int(3)),
                    ("y".to_string(), mc_protocol::nbt::Nbt::Int(4)),
                    ("z".to_string(), mc_protocol::nbt::Nbt::Int(5)),
                    (
                        "Items".to_string(),
                        mc_protocol::nbt::Nbt::List(vec![
                            mc_protocol::nbt::Nbt::Compound(vec![(
                                "id".to_string(),
                                mc_protocol::nbt::Nbt::String(
                                    "minecraft:not_an_item".to_string(),
                                ),
                            )]),
                            mc_protocol::nbt::Nbt::Compound(vec![(
                                "id".to_string(),
                                mc_protocol::nbt::Nbt::String(
                                    "minecraft:cobblestone".to_string(),
                                ),
                            )]),
                            mc_protocol::nbt::Nbt::Compound(vec![(
                                "id".to_string(),
                                mc_protocol::nbt::Nbt::String("minecraft:iron_ingot".to_string()),
                            ),
                            ("count".to_string(), mc_protocol::nbt::Nbt::Int(64))]),
                        ]),
                    ),
                ]),
            ]),
        )]);
        let stores = container_stores_from_nbt(&root);
        assert_eq!(stores.len(), 1, "the coordinate-less row the skip keeps");
        let slots = stores[&(3, 4, 5)].clone();
        assert_eq!(slots.len(), 27, "the fresh grid the shape keeps");
        let cobble = mc_registry::item_id_by_name("cobblestone").expect("the registry names");
        let iron = mc_registry::item_id_by_name("iron_ingot").expect("the registry names");
        assert_eq!(slots[0], (0, 0), "the unknown name the empty stack keeps");
        assert_eq!(slots[1], (cobble, 1), "the missing count the fallback-1 keeps");
        assert_eq!(slots[2], (iron, 64), "the stocked slot the parse wears");
        // THE OVER-LONG LIST (the 27-slot cap truncates)
        let root = mc_protocol::nbt::Nbt::Compound(vec![(
            "entries".to_string(),
            mc_protocol::nbt::Nbt::List(vec![mc_protocol::nbt::Nbt::Compound(vec![
                ("x".to_string(), mc_protocol::nbt::Nbt::Int(0)),
                ("y".to_string(), mc_protocol::nbt::Nbt::Int(0)),
                ("z".to_string(), mc_protocol::nbt::Nbt::Int(0)),
                (
                    "Items".to_string(),
                    mc_protocol::nbt::Nbt::List(
                        std::iter::repeat(mc_protocol::nbt::Nbt::Compound(vec![(
                            "id".to_string(),
                            mc_protocol::nbt::Nbt::String("minecraft:stone".to_string()),
                        )]))
                        .take(28)
                        .collect(),
                    ),
                ),
            ])]),
        )]);
        let stores = container_stores_from_nbt(&root);
        let slots = stores[&(0, 0, 0)].clone();
        assert_eq!(slots.len(), 27, "the cap the over-long list truncates");
        // THE CORRUPT FILE (the lazy load the fresh map keeps — the
        // read failure the silent keep answers) + THE NONE BOOT (the
        // flag banks, no file lane spends)
        let dir = std::env::temp_dir().join(format!("mc2rs-containers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("containers-corrupt.nbt");
        std::fs::write(&file, [0x0A, 0x00, 0x01, 0xFF]).expect("the garbage writes");
        let mut app = App::new(3);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        app.container_stores_load_once();
        assert!(
            app.container_stores.is_empty(),
            "the corrupt file the fresh map keeps"
        );
        assert!(app.container_stores_loaded, "the once-flag the failed read banks");
        let mut bare = App::new(3);
        bare.enter_world();
        bare.container_stores_load_once();
        assert!(
            bare.container_stores.is_empty() && bare.container_stores_loaded,
            "the NONE boot the flag banks, the map stays"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE FURNACES' OWN PER-CELL STORE (the dev-69 round — the same
    /// session lane the chest grid speaks: the open the cell's trio
    /// loads (the fresh 3-slot vec the first open inserts — the
    /// FurnaceMenu law INPUT 0 FUEL 1 RESULT 2), the GUI's edits the
    /// close flush banks, the reopen the SAME stacks wears (the
    /// store's trio the fresh open reads), the DIFFERENT cell the
    /// fresh trio keeps — the one furnace's stocks the other's store
    /// never touches.
    #[test]
    fn the_furnaces_own_store_the_edits_the_reopen_the_same_stacks_bank() {
        let mut app = App::new(11);
        app.enter_world();
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        // THE FIRST OPEN (the fresh trio the store inserts, the open
        // cell the key banks)
        app.open_furnace(cell);
        assert!(app.furnace_stores.contains_key(&cell), "the fresh trio banks");
        assert!(app.furnace_open_cell == Some(cell), "the open cell the key wears");
        // THE EDITS (the three block-entity slots the FurnaceMenu law
        // wears — input rides in, fuel rides in)
        app.furnace_slots[0] = (wire_id_of("raw_iron"), 5);
        app.furnace_slots[1] = (wire_id_of("coal"), 3);
        // THE CLOSE the flush banks
        app.on_escape();
        assert_eq!(app.furnace_stores[&cell][0], (wire_id_of("raw_iron"), 5));
        assert_eq!(app.furnace_stores[&cell][1], (wire_id_of("coal"), 3));
        assert!(app.furnace_open_cell.is_none(), "the close the key takes home");
        // THE DIFFERENT CELL the fresh trio keeps (the second furnace
        // the fresh 3-slot vec inserts — the first's stocks the
        // second's store never wears)
        let cell2 = (c, surface + 1, c + 2);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell2.0, cell2.1, cell2.2, crate::world::FURNACE);
        app.open_furnace(cell2);
        assert_eq!(
            app.furnace_slots[0],
            (0, 0),
            "the different cell the fresh trio keeps"
        );
        app.on_escape();
        // THE REOPEN the same stacks wear (the store's trio the fresh
        // open reads — the first furnace's stocks the bank answers)
        app.open_furnace(cell);
        assert_eq!(
            app.furnace_slots[0],
            (wire_id_of("raw_iron"), 5),
            "the banked input the reopen wears"
        );
        assert_eq!(
            app.furnace_slots[1],
            (wire_id_of("coal"), 3),
            "the banked fuel the reopen wears"
        );
        app.on_escape();
    }


    /// THE FURNACE BANK'S OWN DISK (the dev-69 round — the same disk
    /// law the chest grid speaks: the flush's file write the trio
    /// banks, the file the "furnaces" list the 3-slot compounds keeps
    /// (the stocked slot the minecraft: pair wears, the empty slots
    /// the OPTIONAL empties), the RESTART the fresh app the banked
    /// trio the reopen wears, the break-after-boot the banked
    /// contents the spill releases — the REAL Minecraft law the
    /// restart keeps).
    #[test]
    fn the_furnace_bank_the_disk_the_restart_the_same_stacks_wear() {
        let dir = std::env::temp_dir().join(format!("mc2rs-containers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("containers-furnace.nbt");
        let _ = std::fs::remove_file(&file);
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE FIRST SESSION: the edits the close bank — the flush the
        // file write rides home
        let mut app = App::new(11);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        app.open_furnace(cell);
        app.furnace_slots[0] = (wire_id_of("raw_iron"), 5);
        app.furnace_slots[1] = (wire_id_of("coal"), 3);
        app.furnace_slots[2] = (wire_id_of("iron_ingot"), 2);
        app.on_escape();
        assert!(file.exists(), "the flush the file write banks");
        // THE FILE'S OWN SHAPE (the dual root the one file wears: the
        // "furnaces" list the trio keeps, the "entries" list the
        // chest bank stays empty, the DataVersion header the era
        // pins)
        let bytes = std::fs::read(&file).expect("the banked file reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        assert_eq!(
            root.get("DataVersion").and_then(|v| v.as_i32()),
            Some(CONTAINERS_DATA_VERSION),
            "the header the era banks"
        );
        let furnaces = root
            .get("furnaces")
            .and_then(|v| v.as_list())
            .expect("the furnaces list");
        assert_eq!(furnaces.len(), 1, "the one stocked cell rides");
        let entry = &furnaces[0];
        assert_eq!(entry.get("x").and_then(|v| v.as_i32()), Some(cell.0));
        assert_eq!(entry.get("y").and_then(|v| v.as_i32()), Some(cell.1));
        assert_eq!(entry.get("z").and_then(|v| v.as_i32()), Some(cell.2));
        let items = entry.get("Items").and_then(|v| v.as_list()).expect("the Items list");
        assert_eq!(items.len(), 3, "the 3-slot trio the list keeps");
        assert!(
            items[0].get("id").is_some(),
            "the stocked input the compound wears"
        );
        assert_eq!(
            items[0].get("id").and_then(|v| v.as_str()),
            Some("minecraft:raw_iron"),
            "the input the name bridge wears"
        );
        assert_eq!(items[0].get("count").and_then(|v| v.as_i32()), Some(5));
        assert_eq!(
            items[1].get("id").and_then(|v| v.as_str()),
            Some("minecraft:coal"),
            "the fuel the name bridge wears"
        );
        assert_eq!(items[1].get("count").and_then(|v| v.as_i32()), Some(3));
        assert_eq!(
            items[2].get("id").and_then(|v| v.as_str()),
            Some("minecraft:iron_ingot"),
            "the result the name bridge wears"
        );
        let entries = root
            .get("entries")
            .and_then(|v| v.as_list())
            .expect("the entries list");
        assert!(entries.is_empty(), "the chest bank the furnace ops never wear");
        // THE RESTART: the fresh app the same game dir carries — the
        // first open the once-flag's lazy read speaks, the banked
        // trio the reopen wears
        let mut fresh = App::new(11);
        fresh.enter_world();
        fresh.container_stores_file = Some(file.clone());
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        fresh.open_furnace(cell);
        assert_eq!(
            fresh.furnace_slots[0],
            (wire_id_of("raw_iron"), 5),
            "the banked input the restart wears"
        );
        assert_eq!(
            fresh.furnace_slots[1],
            (wire_id_of("coal"), 3),
            "the banked fuel the restart wears"
        );
        fresh.on_escape();
        // THE BREAK-AFTER-RESTART (the stocked furnace's death the
        // floor the items wear — the spill the banked trio releases,
        // the entry's removal the file's death ride banks)
        fresh.furnace_store_spilled(cell);
        assert!(
            !fresh.drops.is_empty(),
            "the banked contents the spill wears"
        );
        assert!(
            !fresh.furnace_stores.contains_key(&cell),
            "the store the entry's death keeps"
        );
        let bytes = std::fs::read(&file).expect("the death ride reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let furnaces = root
            .get("furnaces")
            .and_then(|v| v.as_list())
            .expect("the furnaces list");
        assert!(
            furnaces.is_empty(),
            "the death ride the emptied bank keeps"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE FURNACE WORDS' OWN BANK (the dev-70 round — the burn/cook
    /// progress state the session AND the restart OUTLIVE: the close
    /// flush the words banks, the DIFFERENT cell the fresh zeros
    /// keeps, the reopen the banked words wears, the RESTART the
    /// mid-smelt the resume speaks, the break the words never
    /// resurrects, the tolerant parse the coordinate-less row skips
    /// and the short list the zero pad fills).
    #[test]
    fn the_furnace_words_the_disk_the_restart_the_burn_wears() {
        // THE TOLERANT PARSE (the missing coordinate the skip keeps,
        // the short list the zero pad fills, the over-long list the
        // 4-word cap truncates, the missing list the empties keep)
        let words = furnace_words_from_nbt(&mc_protocol::nbt::Nbt::Int(1));
        assert!(words.is_empty(), "the non-compound the empties keep");
        let root = mc_protocol::nbt::Nbt::Compound(vec![(
            "furnace_words".to_string(),
            mc_protocol::nbt::Nbt::List(vec![
                // the x/y/z-less row the skip keeps
                mc_protocol::nbt::Nbt::Compound(vec![(
                    "Words".to_string(),
                    mc_protocol::nbt::Nbt::List(vec![]),
                )]),
                // the stocked row: 2 words ride, the zero pad fills,
                // the 5th word the cap truncates
                mc_protocol::nbt::Nbt::Compound(vec![
                    ("x".to_string(), mc_protocol::nbt::Nbt::Int(7)),
                    ("y".to_string(), mc_protocol::nbt::Nbt::Int(8)),
                    ("z".to_string(), mc_protocol::nbt::Nbt::Int(9)),
                    (
                        "Words".to_string(),
                        mc_protocol::nbt::Nbt::List(vec![
                            mc_protocol::nbt::Nbt::Int(5),
                            mc_protocol::nbt::Nbt::Int(80),
                            mc_protocol::nbt::Nbt::Int(12),
                            mc_protocol::nbt::Nbt::Int(200),
                            mc_protocol::nbt::Nbt::Int(999),
                        ]),
                    ),
                ]),
            ]),
        )]);
        let words = furnace_words_from_nbt(&root);
        assert_eq!(words.len(), 1, "the coordinate-less row the skip keeps");
        assert_eq!(
            words[&(7, 8, 9)],
            [5, 80, 12, 200],
            "the stocked words the parse wears, the pad the zeros keep"
        );
        // THE SESSION LANE (the flush the close banks, the DIFFERENT
        // cell the fresh zeros keep, the reopen the banked words wear)
        let mut app = App::new(11);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        app.open_furnace(cell);
        assert_eq!(app.furnace_data, [0, 0, 0, 0], "the fresh cell the zeros keep");
        app.furnace_data = [5, 80, 12, 200];
        app.on_escape();
        assert_eq!(
            app.furnace_word_stores[&cell],
            [5, 80, 12, 200],
            "the flush the words bank rides"
        );
        let cell2 = (c, surface + 1, c + 2);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell2.0, cell2.1, cell2.2, crate::world::FURNACE);
        app.open_furnace(cell2);
        assert_eq!(
            app.furnace_data,
            [0, 0, 0, 0],
            "the different cell the fresh zeros keep"
        );
        app.on_escape();
        app.open_furnace(cell);
        assert_eq!(
            app.furnace_data,
            [5, 80, 12, 200],
            "the banked words the reopen wears"
        );
        app.on_escape();
        // THE DISK LANE (the flush's write the third list banks, the
        // RESTART the mid-smelt the resume speaks, the break the
        // words never resurrects)
        let dir = std::env::temp_dir().join(format!("mc2rs-containers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("containers-furnace-words.nbt");
        let _ = std::fs::remove_file(&file);
        let mut app = App::new(11);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        app.open_furnace(cell);
        app.furnace_data = [30, 200, 7, 200];
        app.furnace_slots[0] = (62, 8);
        app.on_escape();
        let bytes = std::fs::read(&file).expect("the banked file reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let word_rows = root
            .get("furnace_words")
            .and_then(|v| v.as_list())
            .expect("the furnace_words list");
        assert_eq!(word_rows.len(), 1, "the one stocked cell rides");
        let words = word_rows[0]
            .get("Words")
            .and_then(|v| v.as_list())
            .expect("the Words list");
        assert_eq!(words.len(), 4, "the 4-word width the list keeps");
        assert_eq!(words[0].as_i32(), Some(30), "the lit remaining rides");
        assert_eq!(words[1].as_i32(), Some(200), "the lit total rides");
        assert_eq!(words[2].as_i32(), Some(7), "the cook progress rides");
        assert_eq!(words[3].as_i32(), Some(200), "the cook total rides");
        let mut fresh = App::new(11);
        fresh.enter_world();
        fresh.container_stores_file = Some(file.clone());
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::FURNACE);
        fresh.open_furnace(cell);
        assert_eq!(
            fresh.furnace_data,
            [30, 200, 7, 200],
            "the banked burn the restart resumes"
        );
        fresh.on_escape();
        // THE BREAK-AFTER-RESTART (the words the entry's death wears)
        fresh.furnace_store_spilled(cell);
        assert!(
            !fresh.furnace_word_stores.contains_key(&cell),
            "the words the spill never resurrects"
        );
        let bytes = std::fs::read(&file).expect("the death ride reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let word_rows = root
            .get("furnace_words")
            .and_then(|v| v.as_list())
            .expect("the furnace_words list");
        assert!(
            word_rows.is_empty(),
            "the death ride the emptied words bank keeps"
        );
        let _ = std::fs::remove_file(&file);
    }


    /// THE FURNACES' OWN WORLD WALK (the dev-72 round — the vanilla
    /// law the block entity's serverTick EVERY stocked furnace
    /// speaks: the closed cell the bank's own tick serves — the coal
    /// the bank burns, the ore the bank smelts, the ingot the bank
    /// lands, the LIT flip the world's kind wears while the player
    /// the OTHER furnace's screen wears).
    #[test]
    fn the_furnaces_own_world_walk_the_banked_cells_the_tocks_wear() {
        let mut app = App::new(11);
        app.enter_world();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface + 1, c);
        let b = (c, surface + 1, c + 2);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::FURNACE);
            w.set(b.0, b.1, b.2, crate::world::FURNACE);
        }
        // THE STOCK (the opened cell's trio the close flush banks —
        // the raw iron the input wears, the coal the fuel keeps)
        app.open_furnace(a);
        app.furnace_slots[0] = (raw_iron, 5);
        app.furnace_slots[1] = (coal, 3);
        app.on_escape();
        // THE OTHER CELL (the screen the walkaway speaks — the
        // furnace_pos the second cell the retention keeps)
        app.open_furnace(b);
        app.on_escape();
        assert_eq!(app.furnace_pos, Some(b), "the walkaway the cell keeps");
        // THE FIRST WALK TICK (the bank the ignite speaks — the coal
        // the bank consumes, the LIT flip the world's kind wears)
        app.update();
        assert_eq!(
            app.furnace_stores[&a][1],
            (coal, 2),
            "the bank's own coal the walk consumes"
        );
        assert_eq!(
            app.furnace_word_stores[&a],
            [1600, 1600, 1, 200],
            "the banked words the walk ignites"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(a.0, a.1, a.2),
            crate::world::FURNACE_LIT,
            "the walk's own flip the world wears"
        );
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(b.0, b.1, b.2),
            crate::world::FURNACE,
            "the empty cell the unlit kind keeps"
        );
        // THE FULL SMELT (the walk the 200-tick arc serves — the
        // ingot the bank lands, the input the bank shrinks)
        for _ in 0..199 {
            app.update();
        }
        assert_eq!(
            app.furnace_stores[&a][2],
            (iron_ingot, 1),
            "the ingot the bank's own burn lands"
        );
        assert_eq!(
            app.furnace_stores[&a][0],
            (raw_iron, 4),
            "the input the bank's own burn shrinks"
        );
    }


    /// THE WORLD WALK'S OWN PRIME (the dev-72 round — the banked
    /// mid-smelt the closed cell's tick NEVER resets: the scratch
    /// sim's memory the banked input wears, the vanilla law the
    /// setItem reset the input CHANGE alone answers — the closed
    /// cell's slots never change, no setItem ever speaks; the
    /// reopened live lane the same prime wears, the loaded input the
    /// fresh sim's memory keeps).
    #[test]
    fn the_furnaces_own_world_walk_the_mid_smelt_never_resets() {
        let mut app = App::new(11);
        app.enter_world();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface + 1, c);
        let b = (c, surface + 1, c + 2);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::FURNACE);
            w.set(b.0, b.1, b.2, crate::world::FURNACE);
        }
        // THE MID-SMELT STOCK (the banked words the close flush banks)
        app.open_furnace(a);
        app.furnace_slots[0] = (raw_iron, 5);
        app.furnace_slots[1] = (coal, 3);
        app.furnace_data = [10, 80, 57, 200];
        app.on_escape();
        app.open_furnace(b);
        app.on_escape();
        // THE WALK TICK (the prime the progress keeps — 58 the walk
        // speaks, the reset the zero the unprimed memory would wear)
        app.update();
        assert_eq!(
            app.furnace_word_stores[&a],
            [9, 80, 58, 200],
            "the banked mid-smelt the walk continues"
        );
        // THE REOPEN (the live lane the same prime wears — the first
        // live tick the banked arc never resets)
        app.open_furnace(a);
        app.update();
        assert_eq!(
            app.furnace_data,
            [8, 80, 59, 200],
            "the reopened live tick the arc continues"
        );
    }


    /// THE RETAINED CELL'S OWN RIDE HOME (the dev-72 round — the
    /// ticks the close the live arm kept walking the store banks at
    /// the switch: the retention the live trio walks, the open the
    /// OTHER cell the walked ticks ride home, the world walk the
    /// bank continues, the reopen the FULL arc the resume wears —
    /// the continuity the old switch the store's stale copy broke).
    #[test]
    fn the_retained_cell_the_switch_the_ride_home_the_continuity_wears() {
        let mut app = App::new(11);
        app.enter_world();
        let coal = mc_registry::item_id_by_name("coal").unwrap();
        let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface + 1, c);
        let b = (c, surface + 1, c + 2);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::FURNACE);
            w.set(b.0, b.1, b.2, crate::world::FURNACE);
        }
        // THE STOCK + THE CLOSE (the flush the close-time state banks)
        app.open_furnace(a);
        app.furnace_slots[0] = (raw_iron, 5);
        app.furnace_slots[1] = (coal, 3);
        app.on_escape();
        assert_eq!(
            app.furnace_word_stores[&a],
            [0, 0, 0, 0],
            "the close the zeros banks"
        );
        // THE RETENTION WALK (the live arm the retained trio serves —
        // the ignite the first tick speaks, the arc the ticks grow)
        app.update();
        assert_eq!(
            app.furnace_data,
            [1600, 1600, 1, 200],
            "the retention the ignite wears"
        );
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(
            app.furnace_data,
            [1595, 1600, 6, 200],
            "the retention the arc keeps"
        );
        // THE SWITCH (the ride home the walked ticks banks — the
        // store the live state the same beat wears)
        app.open_furnace(b);
        assert_eq!(
            app.furnace_word_stores[&a],
            [1595, 1600, 6, 200],
            "the ride home the walked words banks"
        );
        assert_eq!(
            app.furnace_stores[&a][1],
            (coal, 2),
            "the ride home the trio banks"
        );
        app.on_escape();
        // THE BANK'S OWN WALK (the world walk the banked arc serves)
        app.update();
        app.update();
        assert_eq!(
            app.furnace_word_stores[&a],
            [1593, 1600, 8, 200],
            "the bank the walk continues"
        );
        // THE REOPEN (the full arc the resume wears — the loaded
        // input the prime the live tick never resets)
        app.open_furnace(a);
        app.update();
        assert_eq!(
            app.furnace_data,
            [1592, 1600, 9, 200],
            "the continuity the full arc wears"
        );
    }


    /// THE DOUBLE CHEST'S OWN COMBINED GRID (the dev-71 round — the
    /// paired halves the ONE 54-slot compound the menu wears: the
    /// opened chest's variant type the connected direction reads, the
    /// partner cell the SAME kind the compound answers, the
    /// acceptDouble(neighbor, this) order the
    /// CompoundContainer(first, second) shape wears — the grid's
    /// 0..27 the PARTNER's own store hold, the 27..54 the opened
    /// cell's own rows keep). The flush the halves SPLIT (each half's
    /// own store the rows ride home to), the reopen from the OTHER
    /// half the order flips (the neighbor the compound's head keeps),
    /// the lone chest the 27-slot shape never leaves.
    #[test]
    fn the_double_chest_the_halves_the_combined_grid_wears() {
        let mut app = App::new(11);
        app.enter_world();
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface + 1, c); // the LEFT half
        let b = (c + 1, surface + 1, c); // the RIGHT half (the east neighbor)
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::CHEST);
            w.set(b.0, b.1, b.2, crate::world::CHEST);
            w.set_variant(
                a.0,
                a.1,
                a.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_LEFT,
                ),
            );
            w.set_variant(
                b.0,
                b.1,
                b.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_RIGHT,
                ),
            );
        }
        // the partner's own store stocks first (the bank the direct
        // walk wears — the first open the stocked rows load)
        let mut b_store = vec![(0i32, 0i32); 27];
        b_store[1] = (wire_id_of("diamond"), 12);
        b_store[5] = (wire_id_of("coal"), 3);
        app.container_stores.insert(b, b_store);
        // THE OPEN FROM THE LEFT HALF (the compound the pair speaks —
        // the neighbor's rows the head keeps, the opened cell's own
        // rows the tail rides)
        app.open_chest_for(a);
        assert_eq!(
            app.chest_slots.len(),
            54,
            "the double the compound's 54 slots wears"
        );
        assert_eq!(app.container_open_cell, Some(a), "the opened cell the key keeps");
        assert_eq!(app.container_open_cell_b, Some(b), "the partner the second key banks");
        assert_eq!(
            app.chest_slots[1],
            (wire_id_of("diamond"), 12),
            "the neighbor half the compound's head keeps"
        );
        assert_eq!(
            app.chest_slots[5],
            (wire_id_of("coal"), 3),
            "the neighbor's stocked row the head wears"
        );
        // THE EDIT the opened half's own rows (the 27..54 the tail
        // keeps)
        app.chest_slots[27 + 3] = (wire_id_of("iron_ingot"), 7);
        // THE CLOSE the halves SPLIT (the flush the per-cell stores
        // ride home — the partner's rows the partner banks, the
        // opened cell's rows the opened cell keeps)
        app.on_escape();
        assert_eq!(
            app.container_stores[&b][1],
            (wire_id_of("diamond"), 12),
            "the partner's half the partner store banks"
        );
        assert_eq!(
            app.container_stores[&b][5],
            (wire_id_of("coal"), 3),
            "the partner's stocked row the partner keeps"
        );
        assert_eq!(
            app.container_stores[&a][3],
            (wire_id_of("iron_ingot"), 7),
            "the opened half's rows the opened cell banks"
        );
        assert!(app.container_open_cell.is_none(), "the close the keys take home");
        assert!(app.container_open_cell_b.is_none(), "the partner key the close takes");
        // THE REOPEN FROM THE OTHER HALF (the order flips — the
        // acceptDouble(neighbor, this) walk the opened cell speaks:
        // now the LEFT half the compound's head wears)
        app.open_chest_for(b);
        assert_eq!(app.chest_slots.len(), 54, "the pair the compound keeps");
        assert_eq!(app.container_open_cell, Some(b));
        assert_eq!(app.container_open_cell_b, Some(a));
        assert_eq!(
            app.chest_slots[27 + 1],
            (wire_id_of("diamond"), 12),
            "the partner's rows the tail keeps now"
        );
        assert_eq!(
            app.chest_slots[3],
            (wire_id_of("iron_ingot"), 7),
            "the opened half's rows the head keeps now"
        );
        app.on_escape();
        // THE LONE CHEST the 27-slot shape never leaves (the SINGLE
        // type the connected walk never speaks — no partner, no
        // compound)
        let lone = (c, surface + 1, c + 5);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(lone.0, lone.1, lone.2, crate::world::CHEST);
        app.offline_world.as_mut().unwrap().set_variant(
            lone.0,
            lone.1,
            lone.2,
            crate::world::chest_variant(
                crate::world::CHEST_FACING_NORTH,
                crate::world::CHEST_TYPE_SINGLE,
            ),
        );
        app.open_chest_for(lone);
        assert_eq!(app.chest_slots.len(), 27, "the lone chest the single keeps");
        assert!(app.container_open_cell_b.is_none(), "no partner the None keeps");
        app.on_escape();
    }


    /// THE DOUBLE CHEST'S OWN DISK (the dev-71 round — the same disk
    /// law the per-cell stores wear: the flush's file write the TWO
    /// halves' entries banks, the RESTART the fresh app the banked
    /// halves the reopen wears, the break-after-boot the partner's
    /// own store the spill releases — the per-cell
    /// preRemoveSideEffects walk the BE's own 27 slots drops — and
    /// the surviving half the SINGLE shape reopens).
    #[test]
    fn the_double_chest_the_disk_the_restart_the_halves_keep() {
        let dir = std::env::temp_dir().join(format!("mc2rs-containers-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the temp dir");
        let file = dir.join("containers-double.nbt");
        let _ = std::fs::remove_file(&file);
        let wire_id_of = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE FIRST SESSION: the pair the compound's edits bank — the
        // flush the halves' file write rides home
        let mut app = App::new(11);
        app.enter_world();
        app.container_stores_file = Some(file.clone());
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface + 1, c);
        let b = (c + 1, surface + 1, c);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::CHEST);
            w.set(b.0, b.1, b.2, crate::world::CHEST);
            w.set_variant(
                a.0,
                a.1,
                a.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_LEFT,
                ),
            );
            w.set_variant(
                b.0,
                b.1,
                b.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_RIGHT,
                ),
            );
        }
        app.open_chest_for(a);
        assert_eq!(app.chest_slots.len(), 54, "the pair the compound speaks");
        app.chest_slots[1] = (wire_id_of("diamond"), 12); // the partner's rows
        app.chest_slots[27 + 3] = (wire_id_of("iron_ingot"), 7); // the opened half
        app.on_escape();
        assert!(file.exists(), "the flush the file write banks");
        // THE FILE'S OWN SHAPE (the entries list the TWO halves' rows
        // keeps — the BTree walk the deterministic order speaks)
        let bytes = std::fs::read(&file).expect("the banked file reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let entries = root
            .get("entries")
            .and_then(|v| v.as_list())
            .expect("the entries list");
        assert_eq!(entries.len(), 2, "the two halves the file banks");
        // THE RESTART (the fresh app the banked halves the reopen
        // wears — the world the same pair the placement walk re-wears,
        // the disk the stores' own truth keeps)
        let mut fresh = App::new(11);
        fresh.enter_world();
        fresh.container_stores_file = Some(file.clone());
        {
            let w = fresh.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::CHEST);
            w.set(b.0, b.1, b.2, crate::world::CHEST);
            w.set_variant(
                a.0,
                a.1,
                a.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_LEFT,
                ),
            );
            w.set_variant(
                b.0,
                b.1,
                b.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_RIGHT,
                ),
            );
        }
        fresh.open_chest_for(a);
        assert_eq!(
            fresh.chest_slots.len(),
            54,
            "the banked pair the compound wears"
        );
        assert_eq!(
            fresh.chest_slots[1],
            (wire_id_of("diamond"), 12),
            "the partner's half the disk wears"
        );
        assert_eq!(
            fresh.chest_slots[27 + 3],
            (wire_id_of("iron_ingot"), 7),
            "the opened half's rows the disk keeps"
        );
        fresh.on_escape();
        // THE BREAK-AFTER-RESTART (the dig the cell AIR rides first —
        // the removal walk the spill speaks — the partner's own store
        // the item entities release, the entry the cell's death rides)
        fresh
            .offline_world
            .as_mut()
            .unwrap()
            .set(b.0, b.1, b.2, crate::world::AIR);
        fresh.container_store_spilled(b);
        assert!(
            !fresh.container_stores.contains_key(&b),
            "the spilled half the entry dies with"
        );
        assert!(
            !fresh.drops.is_empty(),
            "the stocked half the item entities release"
        );
        assert_eq!(
            fresh.container_stores[&a][3],
            (wire_id_of("iron_ingot"), 7),
            "the surviving half's contents the surviving cell keeps"
        );
        // THE REOPEN THE SINGLE (the stale LEFT type the missing
        // partner the honest single keeps — the 27-slot shape the
        // opened cell's own rows wear)
        fresh.open_chest_for(a);
        assert_eq!(
            fresh.chest_slots.len(),
            27,
            "the surviving half the single shape wears"
        );
        assert!(
            fresh.container_open_cell_b.is_none(),
            "the missing partner the None keeps"
        );
        assert_eq!(
            fresh.chest_slots[3],
            (wire_id_of("iron_ingot"), 7),
            "the surviving half's rows the reopen wears"
        );
        fresh.on_escape();
        // THE DEATH RIDE (the file the emptied partner never banks —
        // the one entry the surviving half keeps)
        let bytes = std::fs::read(&file).expect("the death ride reads");
        let (_, root) =
            mc_protocol::nbt::read_file_nbt(&mut bytes.as_slice()).expect("the parse serves");
        let entries = root
            .get("entries")
            .and_then(|v| v.as_list())
            .expect("the entries list");
        assert_eq!(entries.len(), 1, "the spilled half the file never resurrects");
        let _ = std::fs::remove_file(&file);
    }


    #[test]
    fn the_dig_broken_half_the_survivor_the_single_shape_wears() {
        // THE PAIR'S OWN BREAK (the dev-73 round — javap
        // ChestBlock.updateShape pc 138-160 the 26.2 jar the removal
        // chain serves): the hold mines the LEFT half to air, the
        // destroy_block walk the demote beat speaks — the survivor the
        // SINGLE wears, the facing the byte keeps, the survivor's own
        // stock the world keeps (the vanilla law: the spill the broken
        // half's own store alone releases), the reopen the single
        // shape the 27 rows wears.
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let a = (c, surface, c);
        let b = (c - 1, surface, c);
        {
            let w = app.offline_world.as_mut().unwrap();
            w.set(a.0, a.1, a.2, crate::world::CHEST);
            w.set_variant(
                a.0,
                a.1,
                a.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_RIGHT,
                ),
            );
            w.set(b.0, b.1, b.2, crate::world::CHEST);
            w.set_variant(
                b.0,
                b.1,
                b.2,
                crate::world::chest_variant(
                    crate::world::CHEST_FACING_NORTH,
                    crate::world::CHEST_TYPE_LEFT,
                ),
            );
        }
        // THE OPEN THE COMPOUND SPEAKS (the pair the 54 rows wears) —
        // the stock the survivor's own store banks
        app.open_chest_for(a);
        assert_eq!(app.chest_slots.len(), 54, "the pair the compound speaks");
        app.chest_slots[27 + 3] = (926, 12); // the opened RIGHT half's own row
        app.on_escape();
        assert_eq!(
            app.container_stores[&a][3],
            (926, 12),
            "the flush the survivor's half banks"
        );
        // THE DIG: the LEFT half the hold mines to air — the real lane
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(
                b.0 as f32 + 0.5,
                app.spawn_y as f32 + 4.0,
                b.2 as f32 + 0.5,
            ),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(b.0, b.1, b.2) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        // THE DEMOTE (the updateShape arm the removal chain serves):
        // the survivor the SINGLE wears, the facing the byte keeps
        {
            let w = app.offline_world.as_ref().unwrap();
            assert_eq!(
                crate::world::chest_variant_type(w.variant(a.0, a.1, a.2)),
                crate::world::CHEST_TYPE_SINGLE,
                "the survivor the SINGLE wears"
            );
            assert_eq!(
                crate::world::chest_variant_facing(w.variant(a.0, a.1, a.2)),
                crate::world::CHEST_FACING_NORTH,
                "the facing the byte keeps"
            );
        }
        // THE SURVIVOR'S OWN STOCK THE WORLD KEEPS (the vanilla law —
        // the spill the broken half's own store alone releases)
        assert_eq!(
            app.container_stores.get(&a).map(|s| s[3]),
            Some((926, 12)),
            "the survivor's own store the world keeps"
        );
        assert!(
            !app.container_stores.contains_key(&b),
            "the broken half's entry the spill releases"
        );
        // THE REOPEN (the single shape the 27 rows wears — the partner
        // key the demote takes)
        app.open_chest_for(a);
        assert_eq!(
            app.chest_slots.len(),
            27,
            "the survivor the single shape reopens"
        );
        assert!(
            app.container_open_cell_b.is_none(),
            "the partner key the demote takes"
        );
        assert_eq!(
            app.chest_slots[3],
            (926, 12),
            "the survivor's stock the reopen wears"
        );
    }


    /// THE OFFLINE CHEST CLICK'S OWN WALK (the dev-66 round — the
    /// transfer clicks the REAL inventory mutates: the hotbar row the
    /// cursor takes, the grid seat the stack lands, the shift-click
    /// the quick-move home rides; the flush the close banks — the
    /// vanilla doClick's own tables the offline belief answers).
    #[test]
    fn the_offline_chest_click_the_transfer_the_real_inventory_mutates() {
        let mut app = App::new(11);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::CHEST);
        app.open_chest_for(cell);
        // the player's own stock: the hotbar row's cobblestone
        app.inventory.items[0] = (62, 27);
        app.chest_mirror_sync_from_inventory();
        // THE CLICK ON THE HOTBAR ROW (menu 54): the stack rides the
        // cursor, the REAL inventory empties, the HUD row syncs
        app.chest_slot_rects = vec![(100, 100, 54)];
        app.mouse_pos = Some((108, 108));
        app.click_chest_slot(100, 100, 0);
        assert_eq!(app.chest_carried, (62, 27), "the take rides");
        assert_eq!(app.inventory.items[0], (0, 0), "the real row emptied");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD row synced");
        assert_eq!(app.chest_mirror[27], (0, 0), "the mirror row follows");
        // THE CLICK ON THE GRID SEAT (menu 3): the stack lands
        app.chest_slot_rects = vec![(160, 100, 3)];
        app.mouse_pos = Some((168, 108));
        app.click_chest_slot(160, 100, 0);
        assert_eq!(app.chest_slots[3], (62, 27), "the stack landed");
        assert_eq!(app.chest_carried, (0, 0), "the cursor emptied");
        // THE CLOSE the flush banks — the inventory the honest shape
        // keeps (the cobblestone the CHEST holds now)
        app.on_escape();
        assert_eq!(app.container_stores[&cell][3], (62, 27));
        assert_eq!(app.inventory.items[0], (0, 0));
        // THE REOPEN + THE SHIFT-CLICK HOME: the quick-move the REAL
        // rows speak (the whole stack the OTHER section takes — the
        // grid->mirror order the rev scan keeps, the HOTBAR row the
        // first free seat the walk answers: menu 62 the items[8] wears)
        app.open_chest_for(cell);
        assert_eq!(app.chest_slots[3], (62, 27), "the store remembered");
        app.chest_slot_rects = vec![(160, 100, 3)];
        app.mouse_pos = Some((168, 108));
        app.shift_down = true;
        app.click_chest_slot(160, 100, 0);
        assert_eq!(app.chest_slots[3], (0, 0), "the grid emptied");
        assert_eq!(app.inventory.items[8], (62, 27), "the home ride");
        assert_eq!(app.inventory.items[0], (0, 0), "the main scan never ran");
        app.shift_down = false;
        app.on_escape();
        assert_eq!(app.container_stores[&cell][3], (0, 0));
        assert_eq!(app.inventory.items[8], (62, 27));
        // THE CARRIED'S OWN HOME RIDE (the vanilla removed()'s
        // placeItemBackInInventory): the cursor's held stack the close
        // returns through the add walk (the free-slot scan the items[0]
        // the first empty wears)
        app.open_chest_for(cell);
        app.chest_slot_rects = vec![(100, 100, 62)];
        app.mouse_pos = Some((108, 108));
        app.click_chest_slot(100, 100, 0); // the hotbar stack back on the cursor
        assert_eq!(app.chest_carried, (62, 27));
        app.on_escape();
        assert_eq!(app.inventory.items[0], (62, 27), "the carried rode home");
        assert_eq!(app.chest_carried, (0, 0), "the cursor the close clears");
        assert_eq!(app.container_stores[&cell][3], (0, 0), "the grid the take emptied");
    }


    /// THE DISPENSER'S OWN 3x3 CANVAS (the offline round — the
    /// DispenserScreen the DispenserMenu's own geometry wears): the
    /// door the use lane opens, the nine the per-cell store banks,
    /// the transfer clicks the REAL inventory mutates, the flush the
    /// close banks (the shot beat the SAME map consumes — the lever's
    /// flip the GUI's own stock dispenses), the comparator's analog
    /// read the fill ride serves, the reopen the same stacks wears.
    #[test]
    fn the_dispensers_own_canvas_the_nine_the_store_the_flush_banks() {
        let mut app = App::new(11);
        app.enter_world();
        let cobble = mc_registry::item_id_by_name("cobblestone").unwrap();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface + 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
        // THE FIRST OPEN (the fresh nine the store inserts, the open
        // cell the key banks, the kind the title lane banks)
        app.open_dispenser_for(cell);
        assert_eq!(app.screen, ScreenKind::Dispenser, "the door opened the screen");
        assert_eq!(app.dispenser_open_cell, Some(cell), "the open cell the key wears");
        assert_eq!(app.dispenser_open_kind, crate::world::DISPENSER, "the kind the title banks");
        assert_eq!(app.dispenser_slots.len(), 9, "the fresh nine the menu wears");
        assert!(app.dispenser_slots.iter().all(|&s| s == (0, 0)), "the fresh zeros");
        // THE STANDARD ROWS' OWN PROJECTION (the player's own stock
        // the 36 mirror rows speak — the hotbar head the menu 36
        // wears)
        app.inventory.items[0] = (cobble, 27);
        app.dispenser_mirror_sync_from_inventory();
        assert_eq!(app.dispenser_mirror[27], (cobble, 27), "the hotbar head rode");
        // THE CLICK ON THE HOTBAR ROW (menu 36): the stack rides the
        // cursor, the REAL inventory empties, the HUD row syncs
        app.dispenser_slot_rects = vec![(100, 100, 36)];
        app.mouse_pos = Some((108, 108));
        app.click_dispenser_slot(100, 100, 0);
        assert_eq!(app.dispenser_carried, (cobble, 27), "the take rides");
        assert_eq!(app.inventory.items[0], (0, 0), "the real row emptied");
        assert_eq!(app.hotbar_slots[0], (0, 0), "the HUD row synced");
        assert_eq!(app.dispenser_mirror[27], (0, 0), "the mirror row follows");
        // THE CLICK ON THE GRID SEAT (menu 3): the stack lands
        app.dispenser_slot_rects = vec![(160, 100, 3)];
        app.mouse_pos = Some((168, 108));
        app.click_dispenser_slot(160, 100, 0);
        assert_eq!(app.dispenser_slots[3], (cobble, 27), "the stack landed");
        assert_eq!(app.dispenser_carried, (0, 0), "the cursor emptied");
        // THE CLOSE the flush banks — the store the shot beat
        // consumes, the analog fill the comparator's back read banks
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World, "the close the world wears");
        assert_eq!(app.container_stores[&cell][3], (cobble, 27));
        assert!(app.dispenser_open_cell.is_none(), "the close the key takes home");
        assert_eq!(app.dispenser_open_kind, 0, "the kind the close clears");
        assert_eq!(
            app.offline_world.as_ref().unwrap().container_fill.get(&cell).copied(),
            Some(1),
            "the analog fill the nine's own total banks"
        );
        // THE REOPEN + THE SHIFT-CLICK HOME: the quick-move the REAL
        // rows speak (the grid->mirror order the rev scan keeps, the
        // HOTBAR row the first free seat the walk answers: menu 44
        // the items[8] wears — the moveItemStackTo(9, 45, true) the
        // DispenserMenu's own quickMoveStack banks)
        app.open_dispenser_for(cell);
        assert_eq!(app.dispenser_slots[3], (cobble, 27), "the store remembered");
        app.dispenser_slot_rects = vec![(160, 100, 3)];
        app.mouse_pos = Some((168, 108));
        app.shift_down = true;
        app.click_dispenser_slot(160, 100, 0);
        assert_eq!(app.dispenser_slots[3], (0, 0), "the grid emptied");
        assert_eq!(app.inventory.items[8], (cobble, 27), "the home ride");
        assert_eq!(app.inventory.items[0], (0, 0), "the main scan never ran");
        app.shift_down = false;
        app.on_escape();
        assert_eq!(app.container_stores[&cell][3], (0, 0));
        assert_eq!(app.inventory.items[8], (cobble, 27));
        // THE CARRIED'S OWN HOME RIDE (the vanilla removed()'s
        // placeItemBackInInventory): the cursor's held stack the close
        // returns through the add walk
        app.open_dispenser_for(cell);
        app.dispenser_slot_rects = vec![(100, 100, 44)];
        app.mouse_pos = Some((108, 108));
        app.click_dispenser_slot(100, 100, 0); // the hotbar stack back on the cursor
        assert_eq!(app.dispenser_carried, (cobble, 27));
        app.on_escape();
        assert_eq!(app.inventory.items[0], (cobble, 27), "the carried rode home");
        assert_eq!(app.dispenser_carried, (0, 0), "the cursor the close clears");
        assert_eq!(
            app.container_stores[&cell][3], (0, 0),
            "the grid the take emptied"
        );
    }


    /// THE DISPENSER'S OWN DOOR (the use lane the block-use walk
    /// serves — javap DispenserBlock.useWithoutItem the openMenu +
    /// the awardStat(INSPECT_DISPENSER) + the SUCCESS verdict speaks)
    /// AND THE DROPPER'S OWN TITLE (the same menu class the two
    /// BlockEntities answer — the DEFAULT_NAME the container.* rows
    /// wear): the placed cell's right-press the 3x3 screen opens, the
    /// per-cell store the two kinds keep, the sneaking player the arm
    /// skips (the vanilla's own use walk the sneaking gate wears).
    #[test]
    fn the_dispensers_own_door_the_two_kinds_the_titles_wear() {
        let mut app = App::new(11);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        // THE PLACED CELLS THE LOOK SEAM WEARS (the straight-down aim
        // the hit's own cell reads — the dispenser cell the ground
        // layer keeps, the dropper the sibling column answers: the
        // terrain the direct set never guesses)
        let dcell = (c, top - 1, c);
        let rcell = (c + 2, top - 1, c);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(dcell.0, dcell.1, dcell.2, crate::world::DISPENSER);
        app.offline_world
            .as_mut()
            .unwrap()
            .set(rcell.0, rcell.1, rcell.2, crate::world::DROPPER);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE DOOR (the block-use arm the held item's lane PRECEDES —
        // the empty hand the press keeps, the clip the dispenser's
        // top face reads)
        assert!(app.on_right_press(0, 0), "the door consumed the press");
        assert_eq!(app.screen, ScreenKind::Dispenser, "the door opened the screen");
        assert_eq!(app.dispenser_open_cell, Some(dcell), "the cell the key banks");
        assert_eq!(app.dispenser_open_kind, crate::world::DISPENSER, "the title's kind");
        assert_eq!(app.dispenser_slots.len(), 9, "the fresh nine banks");
        // THE PER-CELL LAW (the dispenser's nine the store banks — the
        // shot beat the SAME map consumes)
        let dstore = app.container_stores.get(&dcell).expect("the dispenser's nine");
        assert!(dstore.iter().all(|&s| s == (0, 0)), "the fresh nine the door keeps");
        // THE SNEAKING PLAYER'S OWN GATE (the shift press the
        // block-use arm never reaches — the vanilla's own use walk
        // the sneaking skip wears)
        app.on_escape();
        app.shift_down = true;
        assert!(!app.on_right_press(0, 0), "the sneaking press the door never wears");
        assert_eq!(app.screen, ScreenKind::World, "the sneaking press the world keeps");
        app.shift_down = false;
        // THE DROPPER'S OWN DOOR (the sibling column the aim moves to
        // — the same menu class the two kinds answer, the
        // container.dropper row the title banks)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(
                (c + 2) as f32 + 0.5,
                top as f32 + 4.0,
                c as f32 + 0.5,
            ),
            yaw: std::f32::consts::PI,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the dropper's door consumed");
        assert_eq!(app.screen, ScreenKind::Dispenser, "the same screen the kind picks");
        assert_eq!(app.dispenser_open_cell, Some(rcell), "the dropper cell the key banks");
        assert_eq!(app.dispenser_open_kind, crate::world::DROPPER, "the dropper title");
        assert_eq!(app.dispenser_slots.len(), 9, "the dropper's own fresh nine");
        let rstore = app.container_stores.get(&rcell).expect("the dropper's nine");
        assert!(rstore.iter().all(|&s| s == (0, 0)), "the fresh nine the dropper wears");
    }


    /// THE BROKEN CONTAINER'S OWN SPILL (the dev-67 round — javap
    /// `BlockEntity.preRemoveSideEffects` the dropContents arm the
    /// removal walk speaks: the dug chest cell the stocked slot the
    /// item entity releases, the per-cell entry dies with, the reopen
    /// the fresh grid keeps; the DIG lane's own completion the break
    /// speaks).
    #[test]
    fn the_broken_container_the_contents_the_spill_wears() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::CHEST);
        let cell = (c, surface, c);
        // THE PRE-SEEDED STORE (the first open + the stack + the close)
        app.open_chest_for(cell);
        app.chest_slots[1] = (926, 5);
        app.on_escape();
        assert_eq!(app.container_stores[&cell][1], (926, 5));
        // THE DIG: the hold mines the chest cell to air
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        // THE STORE DIED WITH THE CELL
        assert!(
            !app.container_stores.contains_key(&cell),
            "the broken container the store never keeps"
        );
        // THE SPILL (the dropContents arm the same beat speaks): the
        // stocked (926, 5) the item entity the dig releases — the
        // spawn-point bands the drops.rs unit test pins, the one-tick
        // physics the dig's completion tick walks the wide cell
        // margin keeps
        let spill = app
            .drops
            .iter()
            .find(|d| d.item == 926 && d.count == 5)
            .expect("the stocked slot the spill releases");
        assert!(
            spill.x >= c as f64 - 0.5 && spill.x < c as f64 + 1.5,
            "the cell's own x reach: {}",
            spill.x
        );
        assert!(
            spill.y >= surface as f64 - 0.5 && spill.y < surface as f64 + 1.5,
            "the cell's own y reach: {}",
            spill.y
        );
        assert!(
            spill.z >= c as f64 - 0.5 && spill.z < c as f64 + 1.5,
            "the cell's own z reach: {}",
            spill.z
        );
        app.on_left_release();
        // THE REBUILD + THE REOPEN: the fresh grid the new container
        // wears (the old stacks the break spent)
        app.offline_world
            .as_mut()
            .unwrap()
            .set(c, surface, c, crate::world::CHEST);
        app.open_chest_for(cell);
        assert_eq!(app.chest_slots[1], (0, 0), "the fresh store empty");
        app.on_escape();
    }


    /// THE CREATIVE BREAK'S OWN SPILL (the vanilla gamemode's own
    /// split — the abilities the block-loot walk ALONE refuse: the
    /// instabuild gate `spawn_drops_for` speaks, the spill the removal
    /// event keeps — the stocked chest the creative hand breaks the
    /// contents the floor wears).
    #[test]
    fn the_creative_break_the_contents_the_spill_keeps() {
        let mut app = App::new(9);
        app.enter_world();
        app.game_mode = GameMode::Creative;
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let cell = (c, surface, c);
        // THE STOCKED STORE (the direct entry — the GUI walk the
        // dev-66 round's own tests speak): the cobblestone x64 the
        // chunk split drains, the diamonds x5 the ONE-chunk small
        // stack wears, the (0,0) slots the walk's isEmpty guard skips
        let mut slots = vec![(0i32, 0i32); 27];
        slots[0] = (62, 64);
        slots[2] = (926, 5);
        app.container_stores.insert(cell, slots);
        app.container_store_spilled(cell);
        // THE ENTRY DIED WITH THE CELL
        assert!(
            !app.container_stores.contains_key(&cell),
            "the spilled store the map never keeps"
        );
        // THE SPILL: the 64 the chunks split (the SUM the stack
        // keeps), the 5 ONE chunk wears, the empty slots the silence
        // keeps (the sums the exact counts answer)
        let cobble: Vec<_> = app.drops.iter().filter(|d| d.item == 62).collect();
        let total: i32 = cobble.iter().map(|d| d.count).sum();
        assert_eq!(total, 64, "the chunks the whole stack drain");
        assert!(cobble.len() > 1, "the 64 the 10..30 rolls split");
        assert_eq!(
            app.drops.iter().filter(|d| d.item == 926).count(),
            1,
            "the small stack ONE chunk wears"
        );
        // THE CREATIVE GATE: the block loot the abilities refuse (the
        // instabuild the spawn_drops_for arm speaks — no chest ITEM
        // drop rides the same break), the spill the removal keeps
        assert!(
            app.drops.iter().all(|d| d.item == 62 || d.item == 926),
            "the block loot the instabuild arm never rolls"
        );
    }


    #[test]
    fn the_hud_row_lands_verbatim_from_the_store() {
        // the offline world carries no store: the row is the all-empty
        // default, the hotbar renders its chrome only (the honest shape
        // the goldens pin)
        let mut app = App::new(3);
        assert_eq!(app.hotbar_slots, [(0, 0); 9]);
        // the live row lands WHOLESALE: every slot verbatim, the (0, 0)
        // slots the wire's own emptiness riding untouched — the HUD
        // skips them at draw time (the count<=0 sentinel)
        let row = [
            (0, 0),
            (921, 5),   // apple x5
            (0, 0),
            (926, 1),   // diamond x1 — the count's own hiding rule
            (62, 27),   // cobblestone x27
            (0, 0),
            (0, 0),
            (932, 64),  // iron_ingot x64
            (0, 0),
        ];
        app.live_hotbar_updated(row);
        assert_eq!(app.hotbar_slots, row);
        // the replacement is wholesale: a fresh row replaces the old
        // belief entirely (the store's refresh re-speaks every slot —
        // a stale icon never outlives its store)
        app.live_hotbar_updated([(0, 0); 9]);
        assert_eq!(app.hotbar_slots, [(0, 0); 9]);
        // the ids resolve through the registry's own order (the same
        // table the flat icon textures walk)
        assert_eq!(mc_registry::item_name(921), Some("apple"));
        assert_eq!(mc_registry::item_name(926), Some("diamond"));
    }


    #[test]
    fn the_held_name_the_pop_the_decay_the_swap_wears() {
        let mut app = App::new(3);
        app.enter_world();
        // the row: slot 0 the diamond sword, slot 1 the dirt stack, the
        // rest the wire's emptiness (the (0, 0) sentinel)
        let mut row = [(0, 0); 9];
        row[0] = (964, 1);
        row[1] = (55, 64);
        app.live_hotbar_updated(row);
        // the display-name resolution the pop renders (the en_us lang
        // table's own hover names; the unresolvable id the empty string
        // keeps — the draw the empty width never arms)
        assert_eq!(app.held_item_display_name(964), "Diamond Sword");
        assert_eq!(app.held_item_display_name(55), "Dirt");
        assert_eq!(app.held_item_display_name(-1), "");
        assert_eq!(app.held_item_display_name(20000), "");
        // THE FRESH POP (the change arm): the first in-world tick the
        // sword's selection the 40-tick store arms
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS);
        assert_eq!(app.held_name_last, Some((964, "Diamond Sword".to_string())));
        // THE SAME-ITEM DECAY (the `is(getItem())` + hover-name equal
        // arm): every quiet tick one step down, the zero the floor
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS - 1);
        for _ in 0..50 {
            app.update();
        }
        assert_eq!(app.held_name_timer, 0, "the decay the zero floor keeps");
        app.update();
        assert_eq!(app.held_name_timer, 0, "the `> 0` gate no negative keeps");
        // THE SWAP RE-ARMS (the number-key lane the state compare feeds —
        // no event hook, the pure per-tick walk the honest read keeps)
        app.on_number_key(1);
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS);
        assert_eq!(app.held_name_last, Some((55, "Dirt".to_string())));
        // the SAME-slot re-select (the on_number_key gate eats the
        // repeat) — the quiet tick decays, never re-arms
        app.on_number_key(1);
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS - 1);
        // THE SWAP BACK (the item change the 40 store re-arms)
        app.on_number_key(0);
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS);
        assert_eq!(app.held_name_last, Some((964, "Diamond Sword".to_string())));
        // THE EMPTY SELECTION (the empty arm): the timer zeroes, the
        // last stack the EMPTY the copy keeps
        app.on_number_key(2);
        app.update();
        assert_eq!(app.held_name_timer, 0);
        assert_eq!(app.held_name_last, None);
        // the empty → non-empty (the pc 121-124 isEmpty gate): the
        // change arm re-arms
        app.on_number_key(0);
        app.update();
        assert_eq!(app.held_name_timer, HELD_NAME_POP_TICKS);
    }


    #[test]
    fn the_offline_command_the_gamemode_the_self_word_wears() {
        let mut app = App::new(3);
        app.enter_world();
        assert_eq!(app.game_mode, GameMode::Survival);
        // THE APPLY (the GameModeCommand walk — the mode the local
        // authority banks, the previous the change alone earns, the
        // abilities the re-grant owns)
        app.chat_open = true;
        app.chat_input = "/gamemode creative".into();
        app.on_chat_submit();
        assert_eq!(app.game_mode, GameMode::Creative);
        assert_eq!(app.previous_game_mode, Some(GameMode::Survival));
        // THE FEEDBACK (commands.gamemode.success.self the
        // gameMode.creative display arg wears)
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set own game mode to Creative Mode"),
        );
        assert!(!app.chat_open, "the submit the chat closes");
        assert!(app.recent_chat.last().map(String::as_str) == Some("/gamemode creative"));
        // THE UNCHANGED MODE the silence keeps (the setGameMode false
        // return — no second message, the previous bank unmoved)
        app.chat_open = true;
        app.chat_input = "/gamemode creative".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set own game mode to Creative Mode"),
        );
        assert_eq!(app.previous_game_mode, Some(GameMode::Survival));
        // THE INVALID WORD the error wears (the GameModeArgument
        // ERROR_INVALID verdict — the red-prefixed unknown command)
        app.chat_open = true;
        app.chat_input = "/gamemode xyz".into();
        app.on_chat_submit();
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(err.starts_with("\u{a7}c"), "the error the RED prefix rides: {err:?}");
        assert!(err.contains("Unknown or incomplete command"));
        // the plain (non-command) text the history keeps — the dispatch
        // the slash-less message never touches
        app.chat_open = true;
        app.chat_input = "hello world".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("hello world"),
        );
    }


    #[test]
    fn the_offline_command_the_tp_the_body_reanchor_keeps() {
        let mut app = App::new(3);
        app.enter_world();
        app.body = Some(crate::body::PlayerBody::new(8.5, 5.0, 8.5, 180.0, 0.0));
        // the fall the teleport resets (the fresh land the zero keeps)
        app.body.as_mut().unwrap().fall_distance = 12.0;
        app.chat_open = true;
        app.chat_input = "/tp 100.5 64 -200".into();
        app.on_chat_submit();
        let b = app.body.as_ref().unwrap();
        assert_eq!((b.x, b.y, b.z), (100.5, 64.0, -200.0));
        assert_eq!(b.fall_distance, 0.0, "the fresh land the fall resets");
        assert!(b.fall_event.is_none());
        // THE FEEDBACK (the formatDouble %f law — the six decimals the
        // Locale.ROOT format keeps)
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Teleported mc2rs to 100.500000, 64.000000, -200.000000"),
        );
        // THE ALIAS (the literal("teleport") the same walk wears)
        app.chat_open = true;
        app.chat_input = "/teleport 0 70 0".into();
        app.on_chat_submit();
        let b = app.body.as_ref().unwrap();
        assert_eq!((b.x, b.y, b.z), (0.0, 70.0, 0.0));
        // THE BAD PARSE the error arms (the two-arg form the three the
        // law keeps — the non-number the finite verdict rejects)
        app.chat_open = true;
        app.chat_input = "/tp 100 64".into();
        app.on_chat_submit();
        assert!(app.chat_messages.last().map(|(m, _)| m.starts_with("\u{a7}c")) == Some(true));
        // the momentum the re-anchor zeroes (the teleport the velocity
        // the fresh body keeps)
        app.chat_open = true;
        app.chat_input = "/tp 5 65 5".into();
        app.on_chat_submit();
        let b = app.body.as_ref().unwrap();
        assert_eq!((b.vx, b.vy, b.vz), (0.0, 0.0, 0.0));
    }


    #[test]
    fn the_offline_command_the_give_the_inventory_add_wears() {
        let mut app = App::new(3);
        app.enter_world();
        assert_eq!(app.hotbar_slots, [(0, 0); 9]);
        // THE GRANT (the Inventory.add walk — the first empty slot the
        // fresh stack fills)
        app.chat_open = true;
        app.chat_input = "/give dirt 32".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[0], (55, 32));
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gave 32 Dirt to mc2rs"),
        );
        // THE DEFAULT COUNT (the missing arg the 1 the integer
        // argument's floor keeps)
        app.chat_open = true;
        app.chat_input = "/give diamond_sword".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[1], (964, 1));
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gave 1 Diamond Sword to mc2rs"),
        );
        // THE NAMESPACE FORM (the minecraft: prefix the strip keeps)
        app.chat_open = true;
        app.chat_input = "/give minecraft:cobblestone 3".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[2], (62, 3));
        // THE TOP-UP PASS (the same-id partial stack the merge fills —
        // the 64 the maxStackSize cap keeps, the remainder the fresh
        // slot carries)
        app.chat_open = true;
        app.chat_input = "/give dirt 40".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[0], (55, 64), "the partial the merge tops up");
        assert_eq!(app.hotbar_slots[3], (55, 8), "the remainder the fresh stack keeps");
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gave 40 Dirt to mc2rs"),
        );
        // THE UNKNOWN ITEM the error wears (the honest None the
        // vocabulary keeps)
        app.chat_open = true;
        app.chat_input = "/give unobtainium 1".into();
        app.on_chat_submit();
        assert!(app.chat_messages.last().map(|(m, _)| m.starts_with("\u{a7}c")) == Some(true));
        // THE COUNT FLOOR (the integer argument's 1 — the zero the
        // parse rejects)
        app.chat_open = true;
        app.chat_input = "/give dirt 0".into();
        app.on_chat_submit();
        assert!(app.chat_messages.last().map(|(m, _)| m.starts_with("\u{a7}c")) == Some(true));
        // THE NO-ROOM the silence keeps (the full row the grant never
        // earns — the vanilla floor-drop the offline world lacks)
        let mut full = [(0, 0); 9];
        for (i, slot) in full.iter_mut().enumerate() {
            *slot = (62 + i as i32, 64);
        }
        app.live_hotbar_updated(full);
        let before = app.chat_messages.len();
        app.chat_open = true;
        app.chat_input = "/give dirt 10".into();
        app.on_chat_submit();
        assert_eq!(app.chat_messages.len(), before, "the no-room the silence keeps");
    }


    #[test]
    fn the_offline_command_the_give_the_item_max_the_chunks_wear() {
        let mut app = App::new(3);
        app.enter_world();
        let snowball = mc_registry::item_id_by_name("snowball").expect("the registry names");
        let sword = mc_registry::item_id_by_name("iron_sword").expect("the registry names");
        // THE 16-CAP CHUNKS (the snowball's own max the give ride wears —
        // the 64 the flat cap never spoke): 100 snowballs the six 16s +
        // the 4 remainder land, the seventh slot the 4 carries
        app.chat_open = true;
        app.chat_input = "/give snowball 100".into();
        app.on_chat_submit();
        for slot in app.hotbar_slots.iter().take(6) {
            assert_eq!(*slot, (snowball, 16), "the full chunk the cap wears");
        }
        assert_eq!(app.hotbar_slots[6], (snowball, 4), "the remainder the fresh slot keeps");
        assert_eq!(app.hotbar_slots[7], (0, 0), "the exhausted grant the silence keeps");
        // THE PARTIAL TOP-UP (the 4-count slot the merge rides — the
        // cap the top-up's own ceiling, the 64 never reached)
        app.chat_open = true;
        app.chat_input = "/give snowball 2".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[6], (snowball, 6), "the partial the merge tops up");
        assert_eq!(app.hotbar_slots[7], (0, 0), "the no fresh slot the merge spoke");
        // THE CAP REFUSAL (the full 16 slot the merge never partners —
        // the next grant the fresh slot answers)
        app.chat_open = true;
        app.chat_input = "/give snowball 16".into();
        app.on_chat_submit();
        assert_eq!(app.hotbar_slots[6], (snowball, 16), "the partial the cap reaches");
        assert_eq!(app.hotbar_slots[7], (snowball, 6), "the fresh slot the 6 remainder keeps");
        // THE UNSTACKABLE LANE (the sword's 1 the merge gate never
        // opens — every sword the own slot wears)
        let mut app2 = App::new(3);
        app2.enter_world();
        app2.chat_open = true;
        app2.chat_input = "/give iron_sword 3".into();
        app2.on_chat_submit();
        for slot in app2.hotbar_slots.iter().take(3) {
            assert_eq!(*slot, (sword, 1), "the one-per-slot the sword's cap wears");
        }
        assert_eq!(app2.hotbar_slots[3], (0, 0), "the exact count the silence keeps");
        app2.chat_open = true;
        app2.chat_input = "/give iron_sword 2".into();
        app2.on_chat_submit();
        assert_eq!(app2.hotbar_slots[0], (sword, 1), "the full slot the merge refuses");
        assert_eq!(app2.hotbar_slots[3], (sword, 1), "the fresh slots the singles wear");
        assert_eq!(app2.hotbar_slots[4], (sword, 1), "the second single the fresh slot keeps");
    }


    #[test]
    fn the_belief_add_the_item_max_the_landing_wears() {
        let snowball = mc_registry::item_id_by_name("snowball").expect("the registry names");
        let sword = mc_registry::item_id_by_name("iron_sword").expect("the registry names");
        let stone = mc_registry::item_id_by_name("stone").expect("the registry names");
        // THE SPLIT LANDING (the window-0 belief add — the selected
        // hotbar the 16-cap landing wears, the remainder the next slot
        // keeps, the changed rows the landings name)
        let mut slots = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        let mut changed: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_inventory_add(&mut slots, 0, snowball, 20, &mut changed);
        assert_eq!(leftover, 0, "the empty belief the whole grant lands");
        assert_eq!(slots[36], (snowball, 16), "the selected hotbar the cap wears");
        assert_eq!(slots[37], (snowball, 4), "the remainder the next slot keeps");
        assert_eq!(
            changed,
            vec![(36, (snowball, 16)), (37, (snowball, 4))],
            "the changed rows the landings name"
        );
        // THE CAP REFUSAL (the all-16 belief the merge never partners —
        // the pass stalls, the leftover the caller's honest tail)
        for s in slots.iter_mut() {
            *s = (snowball, 16);
        }
        let mut changed2: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_inventory_add(&mut slots, 0, snowball, 5, &mut changed2);
        assert_eq!(leftover, 5, "the full belief the leftover returns");
        assert!(changed2.is_empty(), "the stalled pass the changed map never names");
        // THE UNSTACKABLE LANE (the sword's 1 the merge gate never
        // opens — every sword the own slot wears)
        let mut slots2 = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        slots2[36] = (sword, 1);
        let mut changed3: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_inventory_add(&mut slots2, 0, sword, 3, &mut changed3);
        assert_eq!(leftover, 0, "the empty slots the singles take");
        assert_eq!(slots2[36], (sword, 1), "the 1-cap slot the merge refuses");
        assert_eq!(slots2[37], (sword, 1), "the first fresh slot the single wears");
        assert_eq!(slots2[38], (sword, 1), "the second fresh slot the single wears");
        // THE 64 KEEP (the stone the plain cap wears — the single
        // landing the whole stack takes)
        let mut slots3 = [(0, 0); crate::world_view::INVENTORY_MENU_SLOTS];
        let mut changed4: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_inventory_add(&mut slots3, 0, stone, 64, &mut changed4);
        assert_eq!(leftover, 0, "the 64 stack the one slot keeps");
        assert_eq!(slots3[36], (stone, 64), "the plain cap the landing wears");
    }


    #[test]
    fn the_chest_furnace_table_adds_the_item_max_the_landing_wears() {
        let snowball = mc_registry::item_id_by_name("snowball").expect("the registry names");
        let sword = mc_registry::item_id_by_name("iron_sword").expect("the registry names");
        // THE CHEST MIRROR'S OWN SPLIT (the selected hotbar row the 16
        // cap tops up, the remainder the next row keeps, the changed
        // rows the mirror's menu ids name — 27+idx the hotbar band)
        let mut mirror = [(0, 0); 36];
        mirror[27] = (snowball, 12);
        let mut changed: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_chest_inventory_add(&mut mirror, 0, snowball, 8, &mut changed);
        assert_eq!(leftover, 0, "the mirror's room the grant takes");
        assert_eq!(mirror[27], (snowball, 16), "the partial the cap tops up");
        assert_eq!(mirror[28], (snowball, 4), "the remainder the next row keeps");
        assert_eq!(
            changed,
            vec![(54, (snowball, 16)), (55, (snowball, 4))],
            "the mirror rows the menu ids name"
        );
        // THE FURNACE MENU'S OWN SPLIT (the 15-count partial the one
        // take the cap reaches, the fresh slot the 4 carries)
        let mut fslots = [(0, 0); 39];
        fslots[30] = (snowball, 15);
        let mut changed2: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_furnace_inventory_add(&mut fslots, 0, snowball, 5, &mut changed2);
        assert_eq!(leftover, 0, "the furnace belief's room the grant takes");
        assert_eq!(fslots[30], (snowball, 16), "the partial the cap reaches");
        assert_eq!(fslots[31], (snowball, 4), "the remainder the fresh slot keeps");
        // THE TABLE'S OWN SPLIT (the crafting menu's player rows the
        // sword's 1 the one-per-slot law wears)
        let mut tslots = [(0, 0); 46];
        tslots[37] = (sword, 1);
        let mut changed3: Vec<(i16, (i32, i32))> = Vec::new();
        let leftover = belief_table_inventory_add(&mut tslots, 0, sword, 2, &mut changed3);
        assert_eq!(leftover, 0, "the table belief's room the grant takes");
        assert_eq!(tslots[37], (sword, 1), "the 1-cap slot the merge refuses");
        assert_eq!(tslots[38], (sword, 1), "the first fresh slot the single wears");
        assert_eq!(tslots[39], (sword, 1), "the second fresh slot the single wears");
    }


    #[test]
    fn the_offline_command_the_time_the_clock_set_absolute_wears() {
        let mut app = App::new(3);
        app.enter_world();
        // THE SET (the TimeCommand's `set <time>` walk — the clock the
        // new total banks, the frame offset the base answers)
        app.chat_open = true;
        app.chat_input = "/time set 6000".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 6000);
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set minecraft:overworld to 6000 tick(s)"),
        );
        assert!(!app.chat_open, "the submit the chat closes");
        // THE UNITS WALK (the TimeArgument's UNITS map — the `s` the
        // x20 the `d` the x24000 the bare number the ticks keeps)
        app.chat_open = true;
        app.chat_input = "/time set 90s".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 1800);
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set minecraft:overworld to 1800 tick(s)"),
        );
        app.chat_open = true;
        app.chat_input = "/time set 3d".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 72000);
        // THE ADD (the same set.absolute key the NEW total the arg
        // carries — the negative rewind the int-min floor allows)
        app.chat_open = true;
        app.chat_input = "/time add 1000".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 73000);
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set minecraft:overworld to 73000 tick(s)"),
        );
        app.chat_open = true;
        app.chat_input = "/time add -2000".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 71000);
        // THE QUERIES (the bare query the clock.absolute wording the
        // gametime sibling the game-time lane wears)
        app.chat_open = true;
        app.chat_input = "/time query".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Clock minecraft:overworld is at 71000 tick(s)"),
        );
        app.chat_open = true;
        app.chat_input = "/time query gametime".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("The game time is 71000 tick(s)"),
        );
        // THE PARSE FAILURES (the negative set the zero floor rejects,
        // the invalid unit the UNITS map's miss keeps, the bare
        // literal and the unknown query arg the dispatch's error
        // render)
        let ok = app.day_time_now();
        for bad in ["/time set -5", "/time set 5x", "/time", "/time query daytime"] {
            app.chat_open = true;
            app.chat_input = bad.into();
            app.on_chat_submit();
            let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
            assert!(err.starts_with("\u{a7}c"), "the {bad} the RED error rides: {err:?}");
            assert_eq!(app.day_time_now(), ok, "the failure the clock unmoved keeps");
        }
    }


    #[test]
    fn the_offline_command_the_clock_pause_resume_rate_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        // THE PAUSE (javap lambda$addClockNodes$4 — the setPaused(true)
        // arm the `commands.time.pause` feedback wears): the day clock
        // FREEZES while the world's own ticks keep speaking — the
        // frame counter the advance still earns, the mob/walk lanes
        // the world's own beat rides
        app.chat_open = true;
        app.chat_input = "/time pause".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Paused clock minecraft:overworld"),
        );
        assert!(app.clock_paused);
        let frozen_at = app.day_time_now();
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(app.day_time_now(), frozen_at, "the pause the day advance gates");
        assert_eq!(app.frame, 10, "the world's own ticks the pause never gates");
        // THE SET UNDER THE PAUSE (the frozen walk the manual write
        // still arms — the clock the set arm re-anchors, the pause
        // the advance keeps gating)
        app.chat_open = true;
        app.chat_input = "/time set 6000".into();
        app.on_chat_submit();
        assert_eq!(app.day_time_now(), 6000);
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6000, "the re-frozen clock the set cannot thaw");
        // THE RESUME (javap lambda$addClockNodes$5 — the
        // setPaused(false) arm the `commands.time.resume` feedback
        // wears): the advance the walk re-arms, the default rate the
        // one-tick step keeps
        app.chat_open = true;
        app.chat_input = "/time resume".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Resumed clock minecraft:overworld"),
        );
        assert!(!app.clock_paused);
        for _ in 0..7 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6007, "the resumed walk the 1:1 step keeps");
        // THE RATE 2.0 (javap lambda$addClockNodes$6 — the
        // floatArg(1.0E-5, 1000.0) bound the setRate walk serves, the
        // `commands.time.rate` feedback the Float.toString word
        // wears: the `2.0x` the integral float's own law keeps)
        app.chat_open = true;
        app.chat_input = "/time rate 2".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Clock minecraft:overworld will now advance at 2.0x normal rate"),
        );
        assert_eq!(app.clock_rate, 2.0);
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6017, "the doubled rate the step doubles");
        // THE RATE 0.5 (the sub-tick carry the floor folds — the
        // every-other-tick step the fractional remainder banks)
        app.chat_open = true;
        app.chat_input = "/time rate 0.5".into();
        app.on_chat_submit();
        assert_eq!(app.clock_rate, 0.5);
        for _ in 0..6 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6020, "the half rate the carry fold walks");
        // THE PAUSE+RATE INTERPLAY (the frozen gate the rate never
        // thaws — the pause the advance's FIRST gate keeps)
        app.chat_open = true;
        app.chat_input = "/time pause".into();
        app.on_chat_submit();
        for _ in 0..9 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6020, "the pause the scaled walk also gates");
        app.chat_open = true;
        app.chat_input = "/time resume".into();
        app.on_chat_submit();
        for _ in 0..4 {
            app.update();
        }
        assert_eq!(app.day_time_now(), 6022, "the resumed half-rate the carry resumes");
        // THE BOUNDS (the floatArg(1.0E-5, 1000.0) the parse verdict
        // keeps — the non-numeric word, the zero, the over-max and
        // the below-min float the RED error rides, the rate AND the
        // carry the rejection never touches; the bounds themselves
        // the INCLUSIVE arms accept)
        let rate_ok = app.clock_rate;
        let total_ok = app.day_time_now();
        for bad in ["/time rate abc", "/time rate 0", "/time rate 1000.1", "/time rate 9e-6", "/time rate"] {
            app.chat_open = true;
            app.chat_input = bad.into();
            app.on_chat_submit();
            let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
            assert!(err.starts_with("\u{a7}c"), "the {bad} the RED error rides: {err:?}");
            assert_eq!(app.clock_rate, rate_ok, "the failed parse the rate unmoved keeps");
        }
        app.chat_open = true;
        app.chat_input = "/time rate 1000".into();
        app.on_chat_submit();
        assert_eq!(app.clock_rate, 1000.0, "the upper bound the inclusive arm keeps");
        app.chat_open = true;
        app.chat_input = "/time rate 0.00001".into();
        app.on_chat_submit();
        assert_eq!(app.clock_rate, 0.00001, "the lower bound the inclusive arm keeps");
        assert_eq!(app.day_time_now(), total_ok, "the parse alone the clock never moves");
        // THE DEFAULT RESTORE (the 1.0 rate the bit-exact one-tick
        // step the old fold wore)
        app.chat_open = true;
        app.chat_input = "/time rate 1".into();
        app.on_chat_submit();
        assert_eq!(app.clock_rate, 1.0);
        let base_now = app.day_time_now();
        for _ in 0..3 {
            app.update();
        }
        assert_eq!(app.day_time_now(), base_now + 3, "the default rate the exact step keeps");
    }


    #[test]
    fn the_offline_command_the_weather_the_levels_speak() {
        let mut app = App::new(3);
        app.enter_world();
        // THE DRY BOOT (the procedural scene the dry world keeps)
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (0.0, 0.0));
        // THE RAIN (the setRain walk — the rain level alone arms, the
        // weather render lane the level pair reads)
        app.chat_open = true;
        app.chat_input = "/weather rain".into();
        app.on_chat_submit();
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (1.0, 0.0));
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set the weather to rain"),
        );
        assert!(!app.chat_open, "the submit the chat closes");
        // THE THUNDER (the setThunder walk — the rain AND the thunder
        // both armed, the `rain & thunder` wording the table speaks)
        app.chat_open = true;
        app.chat_input = "/weather thunder".into();
        app.on_chat_submit();
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (1.0, 1.0));
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set the weather to rain & thunder"),
        );
        // THE CLEAR (the setClear walk — the dry sky the zero pair
        // wears)
        app.chat_open = true;
        app.chat_input = "/weather clear".into();
        app.on_chat_submit();
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (0.0, 0.0));
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Set the weather to clear"),
        );
        // THE DURATION FORM (the `duration` TimeArgument.time(1)
        // sibling — the valid the same apply earns)
        app.chat_open = true;
        app.chat_input = "/weather rain 5000".into();
        app.on_chat_submit();
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (1.0, 0.0));
        // THE PARSE-FIRST LAW (the unknown word AND the below-one
        // duration the apply never reaches — the levels the failure
        // unmoved keeps)
        app.chat_open = true;
        app.chat_input = "/weather xyz".into();
        app.on_chat_submit();
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(err.starts_with("\u{a7}c"), "the error the RED prefix rides: {err:?}");
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (1.0, 0.0), "the bad word the levels unmoved keeps");
        app.chat_open = true;
        app.chat_input = "/weather clear 0".into();
        app.on_chat_submit();
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(err.starts_with("\u{a7}c"), "the zero duration the RED error rides: {err:?}");
        let w = app.offline_world.as_ref().unwrap();
        assert_eq!((w.rain_level, w.thunder_level), (1.0, 0.0), "the bad duration the levels unmoved keeps");
    }


    #[test]
    fn the_offline_command_the_kill_the_death_screen_arms() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(!app.dead, "the fresh boot the alive body keeps");
        // THE KILL (the LivingEntity.kill's genericKill walk — the
        // Float.MAX the actuallyHurt serves, the health floor the
        // death tail arms)
        app.chat_open = true;
        app.chat_input = "/kill".into();
        app.on_chat_submit();
        assert!(app.dead, "the kill the death screen arms");
        assert_eq!(app.health, 0.0);
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed mc2rs"),
        );
        assert!(!app.chat_open, "the submit the chat closes");
        // THE ALREADY-DEAD KILL (the !self.dead guard the death tail
        // keeps — the feedback the command layer still speaks)
        app.chat_open = true;
        app.chat_input = "/kill".into();
        app.on_chat_submit();
        assert!(app.dead);
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed mc2rs"),
        );
        // THE RESPAWN (the DeathScreen's own button walk — the fresh
        // body the kill the spawn pose re-anchors)
        app.respawn();
        assert!(!app.dead);
        assert_eq!(app.health, MAX_HEALTH);
        // THE SECOND KILL (the respawned body the kill the same walk
        // serves — the death the fresh body answers again)
        app.chat_open = true;
        app.chat_input = "/kill".into();
        app.on_chat_submit();
        assert!(app.dead);
        assert_eq!(app.health, 0.0);
    }

    #[test]
    fn the_offline_command_the_kill_targets_the_selector_walk_speaks() {
        // THE PURE PARSE (the EntitySelectorParser's own word law —
        // the five heads, the options region, the failures)
        assert_eq!(
            parse_entity_selector_word("@e").map(|p| p.head),
            Some(SelectorHead::AllEntities),
        );
        assert_eq!(
            parse_entity_selector_word("@s").map(|p| p.head),
            Some(SelectorHead::SelfEntity),
        );
        assert_eq!(
            parse_entity_selector_word("@p").map(|p| p.head),
            Some(SelectorHead::NearestPlayer),
        );
        assert_eq!(
            parse_entity_selector_word("@a").map(|p| p.head),
            Some(SelectorHead::AllPlayers),
        );
        assert_eq!(
            parse_entity_selector_word("@r").map(|p| p.head),
            Some(SelectorHead::RandomPlayer),
        );
        // THE FAILURES (the non-selector word, the unknown head, the
        // trailing junk the options region never opens, the unclosed
        // bracket, the below-min limit the offline walk refuses, the
        // other registered keys the offline walk never serves, the
        // tag form)
        for bad in [
            "pig", "@", "@x", "@ee", "@e[", "@e[type=pig", "@e[limit=0]",
            "@e[name=Steve]", "@e[type=#minecraft:pig]", "@e[type=]",
        ] {
            assert!(
                parse_entity_selector_word(bad).is_none(),
                "the {bad} the parse failure keeps"
            );
        }
        // THE TYPE OPTIONS (the bare, the negated, the multiple, the
        // minecraft:-prefixed, the player id)
        let parsed = parse_entity_selector_word("@e[type=pig]").unwrap();
        assert_eq!(
            parsed.types,
            vec![(false, "minecraft:pig".to_string())],
        );
        let parsed = parse_entity_selector_word("@e[type=!skeleton]").unwrap();
        assert_eq!(
            parsed.types,
            vec![(true, "minecraft:skeleton".to_string())],
        );
        let parsed = parse_entity_selector_word("@a[type=minecraft:pig,type=cow]").unwrap();
        assert_eq!(
            parsed.types,
            vec![
                (false, "minecraft:pig".to_string()),
                (false, "minecraft:cow".to_string()),
            ],
        );
        let parsed = parse_entity_selector_word("@e[type=player]").unwrap();
        assert_eq!(parsed.types, vec![(false, "player".to_string())]);
        // THE APP WALK (the world's own roster the matcher answers —
        // the staged roster the clear first keeps, the counts the
        // ctor's own spawn never drifts)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.y, b.z)
        };
        // TWO PIGS + ONE SKELETON the kind re-arm speaks
        let mut pig_a = crate::mob::Zombie::new(101, bx + 1.0, by, bz, 5);
        pig_a.kind = crate::mob::MobKind::Pig;
        let mut pig_b = crate::mob::Zombie::new(102, bx + 2.0, by, bz, 6);
        pig_b.kind = crate::mob::MobKind::Pig;
        let mut skeleton = crate::mob::Zombie::new(103, bx - 1.0, by, bz, 7);
        skeleton.kind = crate::mob::MobKind::Skeleton;
        app.mobs.push(pig_a);
        app.mobs.push(pig_b);
        app.mobs.push(skeleton);
        // THE @e WALK (the mobs AND the player the all-entities query
        // answers — the count the multiple key wears)
        app.chat_open = true;
        app.chat_input = "/kill @e".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 4 entities"),
        );
        assert!(app.dead, "the @e the player includes");
        assert_eq!(app.health, 0.0);
        for mob in &app.mobs {
            assert!(mob.health <= 0.0, "the dose the every mob lands");
        }
        // THE RESPAWN (the fresh body the type walks the alive body
        // stages)
        app.respawn();
        app.spawn_offline_body();
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the respawned body");
            (b.x, b.y, b.z)
        };
        let mut pig = crate::mob::Zombie::new(104, bx + 1.0, by, bz, 8);
        pig.kind = crate::mob::MobKind::Pig;
        app.mobs.clear();
        app.mobs.push(pig);
        // THE TYPE FILTER (the pig alone the positive set answers —
        // the ONE match the single key's display name wears)
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed Pig"),
        );
        assert!(!app.dead, "the filter the player excludes");
        assert!(app.mobs.last().unwrap().health <= 0.0);
        // THE NEGATION (the skeleton the exclusion spares — the dead
        // pig the corpse the collection STILL counts, the player the
        // negation the all-entities head includes: the pair the
        // multiple key wears)
        let mut skel = crate::mob::Zombie::new(105, bx + 2.0, by, bz, 9);
        skel.kind = crate::mob::MobKind::Skeleton;
        app.mobs.push(skel);
        app.chat_open = true;
        app.chat_input = "/kill @e[type=!skeleton]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 2 entities"),
        );
        assert!(app.dead, "the negation the player spares never");
        assert!(
            app.mobs.last().unwrap().health > 0.0,
            "the excluded skeleton the dose never lands"
        );
        // THE RESPAWN + THE PLAYER-ONLY WALK (the type=player filter
        // the mob roster excludes)
        app.respawn();
        app.spawn_offline_body();
        app.chat_open = true;
        app.chat_input = "/kill @e[type=player]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed mc2rs"),
        );
        assert!(app.dead);
        // THE PLAYER HEADS (the @s/@p/@a/@r the single-player source
        // the same ONE player answers)
        app.respawn();
        for head in ["@s", "@p", "@a", "@r"] {
            app.chat_open = true;
            app.chat_input = format!("/kill {head}");
            app.on_chat_submit();
            assert_eq!(
                app.chat_messages.last().map(|(m, _)| m.as_str()),
                Some("Killed mc2rs"),
                "the {head} head the single player answers"
            );
            assert!(app.dead);
            app.respawn();
        }
        // THE PARSE FAILURES (the invalid word the brigadier verdict
        // keeps — the red error rides, the roster untouched)
        for bad in ["pig", "@x", "@e[limit=0]", "@e[type=dragon]"] {
            app.chat_open = true;
            app.chat_input = format!("/kill {bad}");
            app.on_chat_submit();
            let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
            assert!(err.starts_with("\u{a7}c"), "the {bad} the RED error rides: {err:?}");
        }
        assert!(!app.dead);
        // THE EMPTY MATCH (the valid command the zero answers — the
        // world's pig-free roster the type walk the empty collection
        // the count still speaks)
        app.mobs.clear();
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 0 entities"),
        );
    }


    /// THE DISTANCE/LIMIT/SORT OPTIONS' OWN WALK (javap
    /// `MinMaxBounds$Bounds.fromReader` the range the parse wears,
    /// `lambda$bootStrap$23` the below-min limit refuses,
    /// `lambda$bootStrap$25` the four order words switch,
    /// `EntitySelector.sortAndLimit` the order-then-cap the match
    /// applies, `lambda$getPredicate$2` the squared distance answers):
    /// the parse the pure law keeps, the app walk the staged roster
    /// the bodies the distances wear (the pigs the 1.0/2.0 blocks the
    /// skeleton the mirrored 1.0 speaks).
    #[test]
    fn the_offline_command_the_kill_targets_the_distance_limit_sort_speak() {
        // THE PURE PARSE (the fromReader law the range wears)
        assert_eq!(
            parse_entity_selector_word("@e[distance=..8]").unwrap().distance,
            Some((f64::NEG_INFINITY, 8.0)),
        );
        assert_eq!(
            parse_entity_selector_word("@e[distance=4..]").unwrap().distance,
            Some((4.0, f64::INFINITY)),
        );
        assert_eq!(
            parse_entity_selector_word("@e[distance=4..8]").unwrap().distance,
            Some((4.0, 8.0)),
        );
        assert_eq!(
            parse_entity_selector_word("@e[distance=5]").unwrap().distance,
            Some((5.0, 5.0)),
        );
        assert_eq!(
            parse_entity_selector_word("@e[distance=5.5]").unwrap().distance,
            Some((5.5, 5.5)),
        );
        // THE LIMIT + THE SORT (the below-1 the maxResults keeps, the
        // four words the order enum answers)
        assert_eq!(
            parse_entity_selector_word("@e[limit=3]").unwrap().limit,
            Some(3),
        );
        assert_eq!(
            parse_entity_selector_word("@e[sort=nearest]").unwrap().sort,
            Some(SelectorSort::Nearest),
        );
        assert_eq!(
            parse_entity_selector_word("@e[sort=furthest]").unwrap().sort,
            Some(SelectorSort::Furthest),
        );
        assert_eq!(
            parse_entity_selector_word("@e[sort=random]").unwrap().sort,
            Some(SelectorSort::Random),
        );
        assert_eq!(
            parse_entity_selector_word("@e[sort=arbitrary]").unwrap().sort,
            Some(SelectorSort::Arbitrary),
        );
        // THE COMBINED REGION (the four keys the one options region
        // answers — the type lane the old law keeps)
        let parsed =
            parse_entity_selector_word("@e[type=pig,distance=..10,limit=2,sort=nearest]").unwrap();
        assert_eq!(parsed.types, vec![(false, "minecraft:pig".to_string())]);
        assert_eq!(parsed.distance, Some((f64::NEG_INFINITY, 10.0)));
        assert_eq!(parsed.limit, Some(2));
        assert_eq!(parsed.sort, Some(SelectorSort::Nearest));
        // THE FAILURES (the both-empty `..` the ERROR_EMPTY keeps, the
        // swapped the ERROR_SWAPPED wears, the garbage number the
        // readNumber refuses, the limit-0 the ERROR_LIMIT_TOO_SMALL
        // answers, the unknown word the ERROR_SORT_UNKNOWN speaks)
        for bad in [
            "@e[distance=..]",
            "@e[distance=]",
            "@e[distance=8..4]",
            "@e[distance=abc]",
            "@e[distance=4..8..2]",
            "@e[limit=0]",
            "@e[limit=-1]",
            "@e[limit=abc]",
            "@e[sort=bogus]",
            "@e[sort=]",
        ] {
            assert!(
                parse_entity_selector_word(bad).is_none(),
                "the {bad} the parse failure keeps"
            );
        }
        // THE APP WALK (the staged roster the distance law answers —
        // the body the source position wears, the pigs the 1.0/2.0
        // blocks ride, the skeleton the mirrored 1.0 keeps)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let stage = |app: &mut App, base: i32| {
            app.mobs.clear();
            let (bx, by, bz) = {
                let b = app.body.as_ref().expect("the staged body");
                (b.x, b.y, b.z)
            };
            let mut pig_a = crate::mob::Zombie::new(base, bx + 1.0, by, bz, 5);
            pig_a.kind = crate::mob::MobKind::Pig;
            let mut pig_b = crate::mob::Zombie::new(base + 1, bx + 2.0, by, bz, 6);
            pig_b.kind = crate::mob::MobKind::Pig;
            let mut skeleton = crate::mob::Zombie::new(base + 2, bx - 1.0, by, bz, 7);
            skeleton.kind = crate::mob::MobKind::Skeleton;
            app.mobs.push(pig_a);
            app.mobs.push(pig_b);
            app.mobs.push(skeleton);
        };
        // THE MAX SIDE (distance=..1.5 — the player 0.0, the pigs the
        // 1.0/2.0 split, the skeleton the 1.0 keeps: THREE the count
        // wears, the far pig the dose never lands)
        stage(&mut app, 301);
        app.chat_open = true;
        app.chat_input = "/kill @e[distance=..1.5]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 3 entities"),
        );
        assert!(app.dead, "the zero distance the player includes");
        assert!(app.mobs[0].health <= 0.0, "the 1.0 pig the range lands");
        assert!(app.mobs[1].health > 0.0, "the 2.0 pig the range spares");
        assert!(app.mobs[2].health <= 0.0, "the 1.0 skeleton the range lands");
        // THE MIN SIDE (distance=1.5.. — the far pig ALONE the range
        // answers: the player the 0.0 the min excludes)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 311);
        app.chat_open = true;
        app.chat_input = "/kill @e[distance=1.5..]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed Pig"),
        );
        assert!(!app.dead, "the min side the 0.0 player excludes");
        assert!(app.mobs[0].health > 0.0, "the near pig the min spares");
        assert!(app.mobs[1].health <= 0.0, "the far pig the min lands");
        // THE BOUNDED SIDES (distance=1.5..2.5 — the far pig the band
        // answers, the 0.0 the 1.0s the band spares)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 321);
        app.chat_open = true;
        app.chat_input = "/kill @e[distance=1.5..2.5]".into();
        app.on_chat_submit();
        assert!(app.mobs[1].health <= 0.0, "the 2.0 pig the band lands");
        assert!(app.mobs[0].health > 0.0 && app.mobs[2].health > 0.0);
        assert!(!app.dead);
        // THE BOUNDARY (distance=1.. — the EXACT 1.0 the min-inclusive
        // law lands, the 0.0 player the band spares)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 331);
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig,distance=1..]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 2 entities"),
        );
        assert!(!app.dead, "the exact min the boundary includes");
        // THE ZERO BAND (distance=..0 — the AT-SOURCE entities alone
        // the band answers: the player the single key wears)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 341);
        app.chat_open = true;
        app.chat_input = "/kill @e[distance=..0]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed mc2rs"),
        );
        assert!(app.dead, "the zero band the player alone answers");
        // THE LIMIT (the arbitrary order the FIRST candidates keeps —
        // the roster order the pigs the first two wear, the skeleton
        // the player the cap spares)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 351);
        app.chat_open = true;
        app.chat_input = "/kill @e[limit=2]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed 2 entities"),
        );
        assert!(!app.dead, "the cap the player never reaches");
        assert!(app.mobs[0].health <= 0.0 && app.mobs[1].health <= 0.0);
        assert!(app.mobs[2].health > 0.0, "the skeleton the cap spares");
        // THE NEAREST ORDER (sort=nearest,limit=1 — the 1.0 pig the
        // ascending comparator answers, the 2.0 pig the cap spares)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 361);
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig,sort=nearest,limit=1]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed Pig"),
        );
        assert!(app.mobs[0].health <= 0.0, "the near pig the sort lands");
        assert!(app.mobs[1].health > 0.0, "the far pig the cap spares");
        // THE FURTHEST ORDER (sort=furthest,limit=1 — the 2.0 pig the
        // descending comparator answers, the 1.0 pig the cap spares)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 371);
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig,sort=furthest,limit=1]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed Pig"),
        );
        assert!(app.mobs[1].health <= 0.0, "the far pig the sort lands");
        assert!(app.mobs[0].health > 0.0, "the near pig the cap spares");
        // THE RANDOM ORDER (the seeded stream the SAME victim wears —
        // the two identical worlds the same draw answers: the exact
        // ONE pig the cap keeps, the pair the same side loses)
        app.mob_rng = crate::drops::Lcg::new(77);
        stage(&mut app, 381);
        app.chat_open = true;
        app.chat_input = "/kill @e[type=pig,sort=random,limit=1]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Killed Pig"),
        );
        let app_dead_a = app.mobs[0].health <= 0.0;
        let app_dead_b = app.mobs[1].health <= 0.0;
        assert!(
            app_dead_a != app_dead_b,
            "the random order the EXACT one pig keeps"
        );
        let mut app2 = App::new(3);
        app2.enter_world();
        app2.spawn_offline_body();
        app2.mob_rng = crate::drops::Lcg::new(77);
        stage(&mut app2, 381);
        app2.chat_open = true;
        app2.chat_input = "/kill @e[type=pig,sort=random,limit=1]".into();
        app2.on_chat_submit();
        assert_eq!(
            app.mobs[0].health <= 0.0,
            app2.mobs[0].health <= 0.0,
            "the seeded stream the same victim wears"
        );
        // THE PARSE FAILURE (the limit-0 the dispatch refuses — the
        // red error rides, the roster untouched)
        app.respawn();
        app.spawn_offline_body();
        stage(&mut app, 391);
        app.chat_open = true;
        app.chat_input = "/kill @e[limit=0]".into();
        app.on_chat_submit();
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(err.starts_with("\u{a7}c"), "the RED error rides: {err:?}");
        assert!(app.mobs.iter().all(|m| m.health > 0.0));
        assert!(!app.dead);
    }


    /// THE EFFECT FAMILY'S OWN SPEECH (javap `EffectCommands` — the
    /// clear/give tree the dispatch arms wear, the ResourceArgument
    /// word the registry lookup answers, the IntegerArgumentType
    /// ranges the parse verdicts keep, the `infinite` literal the -1
    /// sentinel carries, the feedback keys the size-1 collection law
    /// wears, the failed verdicts the red exceptions render): the
    /// parse the pure law keeps, the app walk the stores the chat
    /// both speak.
    #[test]
    fn the_offline_command_the_effect_family_speaks() {
        // THE PURE PARSE (the ResourceArgument word law)
        let speed = crate::effects::effect_id_by_name("minecraft:speed").unwrap();
        assert_eq!(crate::effects::effect_id_by_name("minecraft:slowness"), Some(1));
        assert_eq!(super::App::parse_effect_word("speed"), Some(speed));
        assert_eq!(super::App::parse_effect_word("minecraft:speed"), Some(speed));
        assert_eq!(
            super::App::parse_effect_word("saturation"),
            Some(crate::effects::MOB_EFFECT_SATURATION),
        );
        assert_eq!(
            super::App::parse_effect_word("instant_health"),
            Some(crate::effects::MOB_EFFECT_INSTANT_HEALTH),
        );
        assert_eq!(
            super::App::parse_effect_word("instant_damage"),
            Some(crate::effects::MOB_EFFECT_INSTANT_DAMAGE),
        );
        // THE FAILURES (the upper-case word, the unknown key, the
        // empty namespace, the empty path, the bare colon)
        for bad in ["Speed", "haste_speed_unknown", "minecraft:", ":speed", ""] {
            assert!(
                super::App::parse_effect_word(bad).is_none(),
                "the {bad:?} the parse failure keeps"
            );
        }
        // THE INT RANGES (the seconds 1..=1000000, the amplifier
        // 0..=255 — the below/above/non-numeric the failures keep)
        assert_eq!(super::App::parse_int_arg("1", 1, 1_000_000), Some(1));
        assert_eq!(
            super::App::parse_int_arg("1000000", 1, 1_000_000),
            Some(1_000_000),
        );
        assert_eq!(super::App::parse_int_arg("0", 1, 1_000_000), None);
        assert_eq!(super::App::parse_int_arg("1000001", 1, 1_000_000), None);
        assert_eq!(super::App::parse_int_arg("abc", 1, 1_000_000), None);
        assert_eq!(super::App::parse_int_arg("255", 0, 255), Some(255));
        assert_eq!(super::App::parse_int_arg("256", 0, 255), None);
        // THE BOOL WORD (the exact pair the reader answers)
        assert_eq!(super::App::parse_bool_arg("true"), Some(true));
        assert_eq!(super::App::parse_bool_arg("false"), Some(false));
        assert_eq!(super::App::parse_bool_arg("maybe"), None);
        // THE APP WALK (the staged roster the matcher answers)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.y, b.z)
        };
        let mut pig = crate::mob::Zombie::new(111, bx + 1.0, by, bz, 5);
        pig.kind = crate::mob::MobKind::Pig;
        app.mobs.push(pig);
        // THE DEFAULT GIVE (the 600-tick default the visible flags
        // wear, the single target the entity name speaks)
        app.chat_open = true;
        app.chat_input = "/effect give @s speed".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Applied effect Speed to mc2rs"),
        );
        let inst = app
            .effects
            .get(speed)
            .expect("the speed the store carries");
        assert_eq!(inst.duration, 600);
        assert_eq!(inst.amplifier, 0);
        assert_eq!(inst.flags, crate::effects::DEFAULT_EFFECT_FLAGS);
        // THE SECONDS FOLD (the *20 the non-instant body wears — the
        // upgrade the strictly-greater amplifier replaces)
        app.chat_open = true;
        app.chat_input = "/effect give @s minecraft:speed 30 1".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Applied effect Speed to mc2rs"),
        );
        let inst = app.effects.get(speed).expect("the upgrade rides");
        assert_eq!(inst.duration, 600, "the 30 seconds the 600 ticks wear");
        assert_eq!(inst.amplifier, 1);
        // THE WEAKER REFUSAL (the equal-amp SHORTER duration the
        // update law keeps — the count the false return empties, the
        // failed verdict the red exception renders)
        app.chat_open = true;
        app.chat_input = "/effect give @s speed 10".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some(
                "\u{a7}cUnable to apply this effect (target is either immune to effects, or has something stronger)"
            ),
        );
        assert_eq!(app.effects.get(speed).unwrap().amplifier, 1);
        // THE HIDE FLAGS (the hidden pair the zero pack wears)
        let jump = crate::effects::effect_id_by_name("minecraft:jump_boost").unwrap();
        app.chat_open = true;
        app.chat_input = "/effect give @s jump_boost 30 0 true".into();
        app.on_chat_submit();
        let inst = app.effects.get(jump).expect("the jump rides");
        assert_eq!(inst.flags, 0, "the hideParticles the zero flags wear");
        // THE INFINITE LITERAL (the -1 sentinel the never-expiry wears)
        app.chat_open = true;
        app.chat_input = "/effect give @s speed infinite 2".into();
        app.on_chat_submit();
        let inst = app.effects.get(speed).expect("the infinite rides");
        assert_eq!(inst.duration, -1);
        assert_eq!(inst.amplifier, 2);
        assert!(inst.is_infinite());
        // THE MOB LANE (the pig the store carries — the single-target
        // collection the entity name speaks)
        app.chat_open = true;
        app.chat_input = "/effect give @e[type=pig] speed".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Applied effect Speed to Pig"),
        );
        assert!(app.mobs[0].effects.has_effect(speed));
        // THE ROSTER COUNT (the @e the player AND the mobs the
        // multiple key wears — the 2-target collection the count
        // speaks EVEN THOUGH the player's standing amp-2 infinite the
        // weaker dose refuses — the collection-size law the
        // feedback's own bytecode keeps)
        app.chat_open = true;
        app.chat_input = "/effect give @e speed 45".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Applied effect Speed to 2 targets"),
        );
        // THE PARSE FAILURES (the bad word, the range verdicts, the
        // bool word — the RED error rides, the stores untouched)
        for bad in [
            "/effect give @s haste_speed_unknown",
            "/effect give @s speed 0",
            "/effect give @s speed 1000001",
            "/effect give @s speed 30 256",
            "/effect give @s speed 30 0 maybe",
            "/effect give @s",
            "/effect",
        ] {
            app.chat_open = true;
            app.chat_input = bad.into();
            app.on_chat_submit();
            let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
            assert!(err.starts_with("\u{a7}c"), "the {bad} the RED error rides: {err:?}");
        }
        // THE CLEAR WALKS (the everything and the specific lanes, the
        // self and the target collections)
        app.chat_open = true;
        app.chat_input = "/effect clear @s speed".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Removed effect Speed from mc2rs"),
        );
        assert!(!app.effects.has_effect(speed));
        // THE MOB'S OWN CLEAR (the pig's store the everything lane
        // empties)
        app.chat_open = true;
        app.chat_input = "/effect clear @e[type=pig]".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Removed every effect from Pig"),
        );
        assert!(app.mobs[0].effects.is_empty());
        // THE EVERYTHING SELF CLEAR (the standing jump_boost the
        // every-lane removes — the ONE body the single key wears)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Removed every effect from mc2rs"),
        );
        assert!(app.effects.is_empty());
        // THE FAILED VERDICTS (the effect-less body the every lane
        // refuses — the red exceptions render)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("\u{a7}cTarget has no effects to remove"),
        );
        app.chat_open = true;
        app.chat_input = "/effect clear @s speed".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("\u{a7}cTarget doesn't have the requested effect"),
        );
    }


    /// THE EFFECT DOSES' OWN TICK (javap the applyEffectTick laws —
    /// the HealOrHarm body the heal/harm doses the tick speaks, the
    /// Saturation's eat the food walk runs, the RegenerationMobEffect
    /// beat the mob's own body heals, the undead's refusal the store
    /// never fills): the doses the bodies the one update each wear.
    #[test]
    fn the_effect_doses_the_bodies_wear() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // THE BODY'S OWN GATES (the food lane the below-18 walk never
        // heals — the doses the assertions the alone effects own)
        app.food_data.set_food_level(1);
        app.food_data.set_saturation(0.0);
        // THE INSTANT HEAL (the default 1-tick duration the ONE
        // application the same law keeps, the 4<<amp dose the health
        // lift speaks, the expiry the same-tick removal answers)
        app.health = 10.0;
        app.chat_open = true;
        app.chat_input = "/effect give @s instant_health".into();
        app.on_chat_submit();
        assert_eq!(
            app.effects
                .get(crate::effects::MOB_EFFECT_INSTANT_HEALTH)
                .unwrap()
                .duration,
            1,
            "the instant body the 1-tick default wears"
        );
        app.update();
        assert_eq!(app.health, 14.0, "the 4<<0 dose the health lifts");
        assert!(
            !app.effects.has_effect(crate::effects::MOB_EFFECT_INSTANT_HEALTH),
            "the spent instant the store leaves"
        );
        // THE SECONDS-AS-TICKS LAW (the instant body the seconds the
        // TICKS speaks — the 3-tick duration the THREE doses wear)
        app.health = 6.0;
        app.chat_open = true;
        app.chat_input = "/effect give @s instant_health 3".into();
        app.on_chat_submit();
        for _ in 0..3 {
            app.update();
        }
        assert_eq!(app.health, 18.0, "the three 4-tick doses the 12 lift");
        assert!(!app.effects.has_effect(crate::effects::MOB_EFFECT_INSTANT_HEALTH));
        // THE INSTANT HARM (the 6<<amp dose the hurt pipeline runs)
        app.health = 20.0;
        app.chat_open = true;
        app.chat_input = "/effect give @s instant_damage".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(app.health, 14.0, "the 6<<0 dose the pipeline lands");
        // THE SATURATION'S EAT (the amplifier+1 dose the food walk
        // runs, the two ticks the two lifts wear)
        app.food_data.set_food_level(10);
        app.food_data.set_saturation(0.0);
        app.chat_open = true;
        app.chat_input = "/effect give @s saturation 2".into();
        app.on_chat_submit();
        for _ in 0..2 {
            app.update();
        }
        assert_eq!(app.food_data.food_level(), 12, "the 1+amp doses land");
        assert!(!app.effects.has_effect(crate::effects::MOB_EFFECT_SATURATION));
        // THE MOB LANE (the pig's regen beat the pig's own body
        // heals, the instant harm the pig's health drops, the
        // undead's refusal the store never fills)
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the body");
            (b.x, b.y, b.z)
        };
        let mut pig = crate::mob::Zombie::new(121, bx + 1.0, by, bz, 15);
        pig.kind = crate::mob::MobKind::Pig;
        pig.health = 4.0;
        app.mobs.push(pig);
        app.chat_open = true;
        app.chat_input = "/effect give @e[type=pig] regeneration".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Applied effect Regeneration to Pig"),
        );
        app.update();
        assert_eq!(
            app.mobs[0].health, 5.0,
            "the pig's regen beat the pig's body heals"
        );
        app.chat_open = true;
        app.chat_input = "/effect give @e[type=pig] instant_damage".into();
        app.on_chat_submit();
        app.update();
        assert!(
            app.mobs[0].health <= 0.0,
            "the 6<<0 dose the 5.0 body the kill tail arms"
        );
        // THE UNDEAD'S REFUSAL (the skeleton the regen the
        // canBeAffected gate refuses — the failed verdict the red
        // exception renders, the store the empty law keeps; the
        // FAR stage the shoot-range the idle body keeps)
        let mut skel = crate::mob::Zombie::new(122, bx + 30.0, by, bz, 25);
        skel.kind = crate::mob::MobKind::Skeleton;
        app.mobs.push(skel);
        app.chat_open = true;
        app.chat_input = "/effect give @e[type=skeleton] regeneration".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some(
                "\u{a7}cUnable to apply this effect (target is either immune to effects, or has something stronger)"
            ),
        );
        assert!(
            app.mobs[1]
                .effects
                .is_empty(),
            "the refused dose the skeleton's store never fills"
        );
        // THE INFINITE NEVER EXPIRES (the -1 sentinel the tick-down
        // never touches, the 200 updates the store the same effect
        // keeps — the namespaced form the walk rides)
        app.chat_open = true;
        app.chat_input = "/effect give @s minecraft:speed infinite".into();
        app.on_chat_submit();
        let speed = crate::effects::effect_id_by_name("minecraft:speed").unwrap();
        for _ in 0..200 {
            app.update();
        }
        let inst = app.effects.get(speed).expect("the infinite never expires");
        assert_eq!(inst.duration, -1);
        assert_eq!(inst.amplifier, 0, "the no-amplifier arm the zero wears");
    }


    /// THE PERIODIC DOSES' OWN LANES (javap PoisonMobEffect/
    /// WitherMobEffect — the `25/40 >> amp` periods the pre-dec
    /// duration the remainder speaks, the poison's `getHealth()
    /// > 1.0F` gate the half-heart floor wears, the wither's
    /// ungated dose the kill lands): the beats the folds the
    /// bodies speak.
    #[test]
    fn the_poison_wither_doses_the_bodies_wear() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // the food lane's own silence (the below-18 walk never
        // heals — the doses the assertions the alone effects own)
        app.food_data.set_food_level(1);
        app.food_data.set_saturation(0.0);
        // THE POISON'S FIRST BEAT (the 600-tick body the /effect
        // seconds-as-ticks law wears: the 600 the 25-multiple the
        // FIRST tick's pre-dec duration speaks — the beat the 1.0
        // dose the pipeline lands)
        app.health = 10.0;
        app.chat_open = true;
        app.chat_input = "/effect give @s poison 30".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.health, 9.0,
            "the first-tick beat the 1.0 dose lands"
        );
        // THE 25-TICK GAP THE PERIOD WEARS (the next multiple the
        // 575 pre-dec duration waits for: the 24 more updates the
        // health the 9.0 the no-beat window keeps, the 25th the
        // 575-multiple beat the 8.0 speaks)
        for _ in 0..24 {
            app.update();
        }
        assert_eq!(app.health, 9.0, "the gap the no-beat window wears");
        app.update();
        assert_eq!(app.health, 8.0, "the 575-multiple the second beat wears");
        // THE FLOOR THE POISON NEVER BREAKS (the 1.0 body the
        // `getHealth() > 1.0F` fcmpl gate refuses: the fresh give
        // the first-tick beat the gate keeps — the health the
        // 1.0 the dose never spends)
        app.health = 1.0;
        app.chat_open = true;
        app.chat_input = "/effect give @s poison 30".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.health, 1.0,
            "the half-heart floor the gate refuses"
        );
        assert!(
            app.effects
                .has_effect(crate::effects::MOB_EFFECT_POISON),
            "the refused dose the effect still stands"
        );
        // THE MOB LANE (the pig's poison beat the pig's body the
        // same 1.0 dose wears, the > 1.0F gate the pig's own
        // health reads — the 4.0 body the armor fold the shared
        // hurt pipeline keeps: the 1.0 dose the CombatRules walk
        // the 0.94 lands, the 3.06 the f64 health speaks)
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the body");
            (b.x, b.y, b.z)
        };
        let mut pig = crate::mob::Zombie::new(121, bx + 1.0, by, bz, 15);
        pig.kind = crate::mob::MobKind::Pig;
        pig.health = 4.0;
        app.mobs.push(pig);
        app.chat_open = true;
        app.chat_input = "/effect give @e[type=pig] poison 30".into();
        app.on_chat_submit();
        app.update();
        let absorbed = 1.0_f32
            * (1.0
                - (crate::mob::ZOMBIE_ARMOR as f32 - 1.0_f32 / 2.0_f32).max(
                    crate::mob::ZOMBIE_ARMOR as f32 * 0.2,
                )
                    / 25.0_f32);
        assert_eq!(
            app.mobs[0].health,
            4.0 - absorbed,
            "the pig's first-tick beat the armor-fold dose lands"
        );
        // THE COOLDOWN'S OWN DECAY (the 20-tick window the 9→8 beat
        // armed: the 11 more updates the invulnerable_time the >10
        // gate drains — the wither dose the fresh hurt the closed
        // window serves; the standing poison's next 575-multiple
        // beat the window never crosses, the 1.0 body the gate
        // refuses regardless)
        for _ in 0..11 {
            app.update();
        }
        // THE WITHER'S UNGATED KILL (the 40-tick body the 40 the
        // multiple the first tick speaks: the 1.0 body the
        // `hurtServer(wither, 1.0F)` the health read the bytecode
        // never makes — the dose KILLS, the health the zero arms)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        app.chat_open = true;
        app.chat_input = "/effect give @s wither 2".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.health, 0.0,
            "the ungated wither dose the 1.0 body kills"
        );
    }


    /// THE HUNGER'S OWN DRAIN (javap HungerMobEffect — the
    /// every-tick `causeFoodExhaustion(0.005f × (amp+1))` the
    /// exhaustion bank rides): the amp-0 walk the bank the exact
    /// f32 accumulation keeps, the amp-3 walk the > 4.0 fold the
    /// saturation drains (the FoodData.tick's own spend the mirror
    /// law walks), the bar the fold's own drop wears.
    #[test]
    fn the_hunger_the_food_bank_drains() {
        // THE AMP-0 WALK (the 10 updates the 0.005-each feed banks —
        // the fold the sub-4.0 bank never fires, the exhaustion the
        // exact accumulation speaks)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.chat_open = true;
        app.chat_input = "/effect give @s hunger 30".into();
        app.on_chat_submit();
        app.update();
        assert!(
            app.effects
                .has_effect(crate::effects::MOB_EFFECT_HUNGER),
            "the give the store banks"
        );
        let mut bank = 0.0_f32;
        for _ in 0..10 {
            // the effects beat's own feed (the add_exhaustion the
            // bank's own min walk wears)
            bank = (bank + crate::effects::HUNGER_EXHAUSTION_PER_TICK).min(40.0);
        }
        // the first update's feed rode the give-tick beat — the nine
        // more updates the same walk wears (the bank the 10 feeds
        // speak)
        for _ in 0..9 {
            app.update();
        }
        assert_eq!(
            app.food_data.exhaustion(),
            bank,
            "the 10 feeds the exact f32 accumulation wears"
        );
        assert_eq!(
            app.food_data.saturation(),
            5.0,
            "the sub-4.0 bank the fold never fires"
        );

        // THE AMP-3 WALK (the 0.02-per-tick feed the > 4.0 fold
        // crosses: the saturation the fold's own -1.0 drains, the
        // bar the saturation's after wears — the mirror the FoodData
        // tick's own spend walk keeps)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.chat_open = true;
        app.chat_input = "/effect give @s hunger 30 3".into();
        app.on_chat_submit();
        app.update();
        let mut bank = 0.0_f32;
        let mut saturation = 5.0_f32;
        let mut food = 20_i32;
        for _ in 0..210 {
            // the FoodData.tick's own fold (the strictly-over 4.0 the
            // fcmpl's own shape wears)
            if bank > 4.0 {
                bank -= 4.0;
                if saturation > 0.0 {
                    saturation = (saturation - 1.0).max(0.0);
                } else {
                    food = (food - 1).max(0);
                }
            }
            // the effects beat's own feed (the amp-3 the ×4 product
            // wears)
            bank = (bank + crate::effects::HUNGER_EXHAUSTION_PER_TICK * 4.0).min(40.0);
        }
        for _ in 0..209 {
            app.update();
        }
        assert_eq!(
            app.food_data.exhaustion(),
            bank,
            "the crossed fold the exact bank speaks"
        );
        assert_eq!(
            app.food_data.saturation(),
            saturation,
            "the fold's own -1.0 the saturation drains"
        );
        assert_eq!(app.food_data.food_level(), food, "the fed body the bar keeps");
    }


    /// THE FIRE IMMUNITY'S OWN VERDICT (javap hurtServer pcs 20-41 —
    /// the `source.is(IS_FIRE) && hasEffect(FIRE_RESISTANCE)` pair
    /// the 0-return answers): the six burning doses the immune body
    /// refuses — the health the refusal never spends, the clear the
    /// immunity lifts, the bare body the dose lands again.
    #[test]
    fn the_fire_resistance_the_burning_doses_refuse() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // THE BARE BODY'S BURN (the lava dose the pipeline lands: the
        // 4.0 the health drop speaks, the true return the landed hurt
        // wears)
        app.health = 20.0;
        assert!(
            app.hurt_burning(crate::body::LAVA_DAMAGE),
            "the bare body the lava dose lands"
        );
        assert_eq!(app.health, 20.0 - crate::body::LAVA_DAMAGE);
        // THE IMMUNITY'S OWN HEAD (the give the store banks: the six
        // burning sources the verdict refuses — the false return the
        // landed verdict keeps, the health the flash the voice the
        // refusal never touches)
        app.chat_open = true;
        app.chat_input = "/effect give @s fire_resistance".into();
        app.on_chat_submit();
        assert!(
            app.effects
                .has_effect(crate::effects::MOB_EFFECT_FIRE_RESISTANCE),
            "the give the store banks"
        );
        app.health = 20.0;
        assert!(!app.hurt_burning(crate::body::LAVA_DAMAGE));
        assert!(!app.hurt_burning(crate::body::ON_FIRE_DAMAGE));
        assert!(!app.hurt_burning(crate::body::IN_FIRE_DAMAGE));
        assert!(!app.hurt_burning(crate::body::IN_SOUL_FIRE_DAMAGE));
        assert!(!app.hurt_burning(crate::body::CAMPFIRE_DAMAGE));
        assert!(!app.hurt_burning(crate::body::SOUL_CAMPFIRE_DAMAGE));
        assert_eq!(app.health, 20.0, "the six refusals the health keeps");
        // THE CLEAR LIFTS THE IMMUNITY (the milk the store empties:
        // the 11 updates the 20-tick window the bare dose admits —
        // the cooldown the six refusals never re-armed, the bare
        // body the on-fire dose lands again)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        assert!(
            !app
                .effects
                .has_effect(crate::effects::MOB_EFFECT_FIRE_RESISTANCE),
            "the clear the store empties"
        );
        for _ in 0..11 {
            app.update();
        }
        assert!(
            app.hurt_burning(crate::body::ON_FIRE_DAMAGE),
            "the bare body the dose lands again"
        );
        assert_eq!(
            app.health,
            20.0 - crate::body::ON_FIRE_DAMAGE,
            "the bare body's own drop the landed dose speaks"
        );
    }


    /// THE VOID'S OWN DOSE (javap `LivingEntity.onBelowWorld` — the
    /// hurtServer(fellOutOfWorld(), 4.0F) the pipeline's own law
    /// keeps): the 4.0 the health drop speaks, the 20-tick window the
    /// same-dose repeat buries, the 10-tick re-arm the next speaks,
    /// the BYPASSES_INVULNERABILITY roster the creative body the void
    /// kills (the plain hurt the head refuses), the exhaustion the
    /// type's own JSON 0.0 the food bank never feeds.
    #[test]
    fn the_void_the_dose_the_bypass_the_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.health = 20.0;
        // THE FIRST DOSE (the 4.0 the health drop speaks, the true
        // the landed hurt wears)
        assert!(
            app.hurt_void(crate::body::VOID_DAMAGE),
            "the void dose the survival body lands"
        );
        assert_eq!(app.health, 20.0 - crate::body::VOID_DAMAGE);
        // THE WINDOW'S OWN BURY (the same 4.0 the comparison the
        // repeat never exceeds — the false the health keeps)
        assert!(
            !app.hurt_void(crate::body::VOID_DAMAGE),
            "the window the same-dose repeat buries"
        );
        assert_eq!(app.health, 20.0 - crate::body::VOID_DAMAGE);
        // THE RE-ARM'S OWN SPEAK (the window's tail the <=10 the else
        // arm answers — the fresh window the dose re-arms)
        app.invulnerable_time = 10;
        assert!(app.hurt_void(crate::body::VOID_DAMAGE), "the re-arm the dose lands");
        assert_eq!(app.health, 20.0 - 2.0 * crate::body::VOID_DAMAGE);
        // THE BYPASS'S OWN VERDICT (the creative the plain head
        // refuses — the void the roster turns: the /kill's own law
        // the pair wears, the void the creative kills)
        app.game_mode = GameMode::Creative;
        app.health = 20.0;
        app.invulnerable_time = 0;
        assert!(
            !app.hurt(4.0),
            "the plain head the creative body refuses"
        );
        assert_eq!(app.health, 20.0, "the refusal the health keeps");
        assert!(
            app.hurt_void(crate::body::VOID_DAMAGE),
            "the roster the invulnerable head turns"
        );
        assert_eq!(
            app.health,
            20.0 - crate::body::VOID_DAMAGE,
            "the void the creative body kills"
        );
        // THE EXHAUSTION'S OWN SILENCE (the out_of_world JSON's 0.0 —
        // the food bank the dose never feeds)
        assert_eq!(
            app.food_data.exhaustion(),
            0.0,
            "the 0.0 the type's own JSON keeps"
        );
    }


    /// THE VOID'S OWN DRAIN (the e2e — the staged fall the tick head
    /// banks, the App's drain the post-borrow tail answers): the one
    /// update the dose lands, the next update the window the repeat
    /// buries, the fall bank the groundless void never folds (the
    /// landing the void never serves — the dose the below-world walk
    /// answers instead).
    #[test]
    fn the_void_the_drain_the_dose_the_wears() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.health = 20.0;
        // THE REGEN'S OWN SILENCE (the food under 18 the regen lane
        // never arms — the cadence's own arithmetic the heal the
        // count never sways)
        app.food_data.set_food_level(6);
        // THE STAGED FALL (the body below the margin — the tick-head
        // read the crossing the previous tick's settled position
        // speaks)
        {
            let b = app.body.as_mut().unwrap();
            b.y = -64.5;
            b.vy = 0.0;
            b.on_ground = false;
        }
        app.update();
        assert_eq!(
            app.health,
            20.0 - crate::body::VOID_DAMAGE,
            "the drain the dose the pipeline serves"
        );
        // THE REPEAT'S OWN BURY (the next update the window the same
        // dose never exceeds — the flag re-banks, the hurt the
        // cooldown refuses)
        app.update();
        assert_eq!(
            app.health,
            20.0 - crate::body::VOID_DAMAGE,
            "the window the repeat buries"
        );
        // THE CADENCE'S OWN SPEAK (the nine more updates the 20-tick
        // window's decay walks — the 10-tick re-arm the every-tick
        // flag the next dose admits: the 4.0 the second speaks, the
        // vanilla's own void-death cadence the pipeline keeps)
        for _ in 0..9 {
            app.update();
        }
        assert_eq!(
            app.health,
            20.0 - 2.0 * crate::body::VOID_DAMAGE,
            "the re-arm the second dose lands"
        );
        {
            let b = app.body.as_ref().unwrap();
            assert!(
                b.y < -crate::body::VOID_MARGIN - 1.0,
                "the body the void's own descent keeps ({:.3})",
                b.y
            );
            assert_eq!(
                b.fall_event, None,
                "the groundless fall the void never lands"
            );
        }
        // THE EXHAUSTION'S OWN SILENCE (the 0.0 the JSON keeps)
        assert_eq!(app.food_data.exhaustion(), 0.0);
    }


    /// THE DEATH'S OWN MESSAGE (the vanilla die's CombatTracker walk —
    /// the damage source's own lang key the death screen names, the
    /// %1$s the player's own name answers): the void's outOfWorld row,
    /// the fall's fell.accident.generic row, the lava/inFire/drown/
    /// starve rows the en_us table carries, the UNNAMED sources the
    /// None keeps (the title-only law the generic lanes wear), the
    /// /kill's own genericKill row, the respawn the message clears.
    #[test]
    fn the_death_the_message_the_source_names() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // THE VOID'S OWN NAME (the crossing dose the health zero
        // banks — the outOfWorld row the message speaks)
        app.health = 4.0;
        assert!(
            app.hurt_void(crate::body::VOID_DAMAGE),
            "the void dose the crossing lands"
        );
        assert!(app.dead, "the zero the death crossing arms");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs fell out of the world"),
            "the outOfWorld row the vanilla lang carries"
        );
        // THE RESPAWN CLEARS (the fresh vitals the message None
        // answers)
        app.respawn();
        assert!(!app.dead);
        assert_eq!(app.death_message, None, "the respawn the message clears");
        // THE FALL'S OWN NAME (the single 20.0 blow the crossing
        // banks — the fell.accident.generic row)
        app.apply_fall_damage(20);
        assert!(app.dead, "the 20 the fall crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs fell from a high place"),
            "the fell.accident.generic row speaks"
        );
        app.respawn();
        // THE BURNING SOURCES' OWN NAMES (the lava row, the inFire
        // family row the campfires' own message_id carries)
        assert!(app.hurt_burning_keyed(20.0, "death.attack.lava"));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs tried to swim in lava"),
            "the lava row speaks"
        );
        app.respawn();
        assert!(app.hurt_burning_keyed(20.0, "death.attack.inFire"));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs went up in flames"),
            "the inFire row speaks"
        );
        app.respawn();
        // THE DROWN'S OWN NAME
        assert!(app.hurt_drowning(20.0));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs drowned"),
            "the drown row speaks"
        );
        app.respawn();
        // THE STARVE'S OWN NAME
        assert!(app.hurt_starving(20.0));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs starved to death"),
            "the starve row speaks"
        );
        app.respawn();
        // THE UNNAMED SOURCES' OWN LAW (the None the title-only
        // screen wears — the doses the caller's own lane the killer
        // word never passes: the magic/poison family the no-name
        // sources keep)
        assert!(app.hurt(20.0));
        assert!(app.dead);
        assert_eq!(
            app.death_message, None,
            "the unnamed source the title-only law keeps"
        );
        app.respawn();
        // THE /KILL'S OWN NAME (the genericKill row the direct walk
        // speaks — the command's own dose the crossing banks)
        app.actually_hurt(
            f32::MAX,
            HurtVoice::GenericKill,
            Some("death.attack.genericKill"),
            None,
        );
        assert!(app.dead, "the /kill dose the crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was killed"),
            "the genericKill row speaks"
        );
    }


    /// THE DEATH NAMES ITS KILLER (javap the 26.2 jar's own
    /// getDeathMessage walks — the mobAttack source the %2$s the
    /// attacker's display name answers, the sonic_boom row the
    /// shriek alone wears, the explosion's own indirect entity the
    /// explosion.player row speaks, the ownerless blast the generic
    /// row keeps; the display names the en_us entity rows carry).
    #[test]
    fn the_death_names_its_killer() {
        let mut app = App::new(3);
        app.enter_world();
        // THE SLAIN ROW (death.attack.mob — the zombie's melee the
        // mobAttack source names, the en_us row the pair fills:
        // "mc2rs was slain by Zombie")
        assert!(app.hurt_by_named(
            20.0,
            "death.attack.mob",
            &mob_kind_display_name(crate::mob::MobKind::Zombie),
        ));
        assert!(app.dead);
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was slain by Zombie"),
            "the slain row the killer word wears"
        );
        app.respawn();
        // THE SONIC ROW (death.attack.sonic_boom — the 26.2 row the
        // %2$s never carries: the shriek the warden name never
        // appends; the lethal dose the crossing banks — the lane's
        // own 10.0 the fresh body survives)
        assert!(app.hurt_keyed(20.0, "death.attack.sonic_boom"));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was obliterated by a sonically-charged shriek"),
            "the sonic row the shriek alone wears"
        );
        app.respawn();
        // THE WARDEN'S MELEE ROW (the same mobAttack source — the
        // Warden the slain row's own display name answers)
        assert!(app.hurt_by_named(
            20.0,
            "death.attack.mob",
            &mob_kind_display_name(crate::mob::MobKind::Warden),
        ));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was slain by Warden"),
            "the warden row speaks"
        );
        app.respawn();
        // THE CREEPER'S OWN ROW (death.attack.explosion.player — the
        // explosion's own source entity the indirect read names:
        // "mc2rs was blown up by Creeper")
        assert!(app.hurt_by_named(
            20.0,
            "death.attack.explosion.player",
            &mob_kind_display_name(crate::mob::MobKind::Creeper),
        ));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was blown up by Creeper"),
            "the blown-up row the creeper names"
        );
        app.respawn();
        // THE OWNERLESS BLAST'S OWN ROW (death.attack.explosion — the
        // null indirect entity the generic row keeps: "mc2rs blew up")
        assert!(app.hurt_keyed(20.0, "death.attack.explosion"));
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs blew up"),
            "the generic blast row the name never carries"
        );
        app.respawn();
        // THE DISPLAY NAMES' OWN TABLE (the en_us entity rows the
        // helper answers — the killer word each drawn kind speaks)
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Zombie), "Zombie");
        assert_eq!(
            mob_kind_display_name(crate::mob::MobKind::Skeleton),
            "Skeleton"
        );
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Spider), "Spider");
        assert_eq!(
            mob_kind_display_name(crate::mob::MobKind::Creeper),
            "Creeper"
        );
        assert_eq!(
            mob_kind_display_name(crate::mob::MobKind::Warden),
            "Warden"
        );
        // THE PASSIVE KINDS' OWN ROWS (the animals the helper's own
        // walk answers — the display name the same entity key speaks)
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Pig), "Pig");
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Cow), "Cow");
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Sheep), "Sheep");
        assert_eq!(
            mob_kind_display_name(crate::mob::MobKind::Chicken),
            "Chicken"
        );
        assert_eq!(mob_kind_display_name(crate::mob::MobKind::Wolf), "Wolf");
        // THE COOLDOWN'S OWN WINDOW (the 20-tick law — the landed dose
        // arms the raw word, the weaker repeat the window's comparison
        // refuses; the respawn the fresh window banks)
        assert!(app.hurt_by_named(
            4.0,
            "death.attack.mob",
            &mob_kind_display_name(crate::mob::MobKind::Zombie),
        ));
        assert!(!app.dead, "the arming dose the alive body keeps");
        assert!(!app.hurt_by_named(
            2.0,
            "death.attack.mob",
            &mob_kind_display_name(crate::mob::MobKind::Spider),
        ));
        assert!(!app.dead, "the refused dose the alive body keeps");
    }


    /// THE SHOT'S OWN ARCHER (the arrow damage source's own
    /// getLocalizedDeathMessage walk — the 26.2 AbstractArrow the
    /// override NEVER carries, the constant pool's death-key absence
    /// the generic DamageSource law serves: "death.attack.arrow" the
    /// msgId the en_us row wears "%1$s was shot by %2$s", the %2$s
    /// the causing-vs-direct read answers — the owner-present walk
    /// the shooter's display name fills, the ownerless walk the
    /// DIRECT entity the arrow's own en_us display name speaks).
    #[test]
    fn the_shot_names_its_archer() {
        let mut app = App::new(5);
        app.enter_world();
        app.spawn_offline_body();
        // THE OPEN FLANK (the world's own terrain the probe answers —
        // the head-height cell one block out the solid step the
        // spawn's own ground sometimes keeps, the arrow the AIR lane
        // the head check demands: the west the probe prefers, the
        // solid verdict the east flank the same 1.5-speed shot wears)
        let open_lane = |app: &App| -> (f64, f64, f64, f64, f64) {
            let b = app.body.as_ref().unwrap();
            let w = app.offline_world.as_ref().unwrap();
            let (px, py, pz) = (b.x, b.y, b.z);
            let west = crate::body::solid(
                w,
                px.floor() as i32 - 1,
                (py + 1.0).floor() as i32,
                pz.floor() as i32,
            );
            if west {
                (px + 1.0, py + 1.0, pz, -1.5, -90.0)
            } else {
                (px - 1.0, py + 1.0, pz, 1.5, 90.0)
            }
        };
        // THE ARCHER (the owner ref the flight lane carries — the id
        // the death's walk the mobs slice answers)
        let mut sk = crate::mob::Zombie::new(
            40,
            app.body.as_ref().unwrap().x + 6.0,
            app.body.as_ref().unwrap().y,
            app.body.as_ref().unwrap().z,
            41,
        );
        sk.kind = crate::mob::MobKind::Skeleton;
        sk.on_ground = true;
        app.mobs = vec![sk];
        // THE LETHAL BODY (the arrow's own ceil(|v|·2.0) dose the
        // two-point health never survives — the death the lane's own
        // walk speaks, the fresh window the first hit banks)
        app.health = 2.0;
        // THE OWNED SHOT (the segment the player's own inflated box
        // crosses — the hand-placed pose the spread's draw never
        // speaks, the flight the one tick's own walk answers)
        let (sx, sy, sz, svx, syr) = open_lane(&app);
        app.arrows.push(crate::mob::Arrow::from_spawn(&crate::mob::ArrowShot {
            id: 9001,
            x: sx,
            y: sy,
            z: sz,
            vx: svx,
            vy: 0.0,
            vz: 0.0,
            y_rot: syr as f32,
            x_rot: 0.0,
            crit: false,
            pitch: 1.0,
            seed: 11,
            owner: Some(40),
        }));
        app.update();
        for _ in 0..8 {
            if app.dead {
                break;
            }
            app.update();
        }
        assert!(
            app.dead,
            "the lethal dose the body takes (health={} arrows={} msg={:?} ax={:?})",
            app.health,
            app.arrows.len(),
            app.death_message,
            app.arrows.first().map(|a| (a.x, a.y, a.z))
        );
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was shot by Skeleton"),
            "the shot row the archer's own name wears"
        );
        // THE SHOVER'S OWN ENTRY (the arrow's causing entity the
        // shooter names — the landed shot the (kind, stamp) pair
        // banks, the fall's own word the assist row the skeleton
        // later speaks)
        assert_eq!(
            app.last_hurt_by_mob.map(|(k, _, _)| k),
            Some(crate::mob::MobKind::Skeleton),
            "the shot the shover entry banks"
        );
        // THE OWNERLESS ARM'S OWN WORD (the causing entity the null
        // keeps — the DIRECT entity the %2$s answers: the arrow's own
        // en_us display name the generic walk fills; the flight the
        // sim's own walk never speaks — the ownerless arrows the
        // client's tick the player sweep NEVER answers, the wire's
        // visual lane the server's authority keeps — so the row the
        // lang table's own answer pins)
        assert_eq!(
            tr("entity.minecraft.arrow"),
            Some("Arrow"),
            "the ownerless row the direct entity's own name wears"
        );
    }


    /// THE BLAST'S OWN PRIMER (javap `PrimedTnt.getOwner` the
    /// getIndirectSourceEntity walk reads — the flint arm's own prime
    /// the player names: "mc2rs was blown up by mc2rs" the pair fills
    /// EVEN WHEN the owner IS the killed body, the fire walk's None
    /// the generic row keeps; the chain the wasExploded walk the
    /// parent's own word inherits — the chained cell the SAME row
    /// speaks).
    #[test]
    fn the_blast_names_its_primer() {
        let mut app = App::new(5);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // THE PLAYER-PRIMED SHOT (the owner word the flint lane's own
        // prime banks — the dose the point-blank body takes)
        app.health = 1.0;
        let (px, py, pz) = (
            app.body.as_ref().unwrap().x,
            app.body.as_ref().unwrap().y,
            app.body.as_ref().unwrap().z,
        );
        let mut t = crate::mob::PrimedTnt::new(9, px + 1.5, py, pz, 7);
        t.fuse = 2;
        t.owner = Some(crate::mob::BlastSource::Player);
        app.primed_tnts.push(t);
        for _ in 0..40 {
            if app.dead {
                break;
            }
            app.update();
        }
        assert!(
            app.dead,
            "the point-blank dose the body takes (health={} msg={:?})",
            app.health, app.death_message
        );
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was blown up by mc2rs"),
            "the primer row the owner's own name wears"
        );
        // THE FIRE-PRIMED SHOT (the ctor's own None the fire walk
        // keeps — the getIndirectSourceEntity null the generic row
        // answers)
        app.respawn();
        app.health = 1.0;
        let (px, py, pz) = (
            app.body.as_ref().unwrap().x,
            app.body.as_ref().unwrap().y,
            app.body.as_ref().unwrap().z,
        );
        let mut t2 = crate::mob::PrimedTnt::new(10, px + 1.5, py, pz, 7);
        t2.fuse = 2;
        app.primed_tnts.push(t2);
        for _ in 0..40 {
            if app.dead {
                break;
            }
            app.update();
        }
        assert!(
            app.dead,
            "the ownerless dose the body takes too (msg={:?})",
            app.death_message
        );
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs blew up"),
            "the fire walk's row the name never carries"
        );
        // THE CHAIN'S OWN INHERITANCE (javap `TntBlock.wasExploded` —
        // the chained cell the explosion's own indirect source the
        // ctor arg speaks: the player-primed boom the chained body's
        // word wears; the WALL the both booms survive — the words the
        // chain alone proves)
        app.respawn();
        app.health = 40.0;
        let (px, py, pz) = (
            app.body.as_ref().unwrap().x,
            app.body.as_ref().unwrap().y,
            app.body.as_ref().unwrap().z,
        );
        // the chained cell the radius answers (3 cells east — the
        // entity the 1.5 gap the blast walk reaches)
        app.offline_world.as_mut().unwrap().set(
            px.floor() as i32 + 3,
            py.floor() as i32,
            pz.floor() as i32,
            crate::world::TNT,
        );
        let mut t3 = crate::mob::PrimedTnt::new(11, px + 1.5, py, pz, 7);
        t3.fuse = 2;
        t3.owner = Some(crate::mob::BlastSource::Player);
        app.primed_tnts.push(t3);
        let mut chain_named = false;
        for _ in 0..120 {
            app.update();
            if app
                .primed_tnts
                .iter()
                .any(|t| t.id != 11 && t.owner == Some(crate::mob::BlastSource::Player))
            {
                chain_named = true;
            }
            if chain_named && app.primed_tnts.is_empty() {
                break; // the chain body's own boom the walk spent
            }
        }
        assert!(
            chain_named,
            "the chain the parent's own word inherits (tnts left: {})",
            app.primed_tnts.len()
        );
    }


    /// THE FALL NAMES ITS SHOVER (the 26.2 CombatTracker's die walk —
    /// the getMostSignificantFall entry the IS_FALL read answers, the
    /// attacker's own name the assist row speaks "was doomed to fall
    /// by %2$s", the expired shelf the kill credit the escape row
    /// wears "hit the ground too hard while trying to escape %2$s", the
    /// creditless fall the accident row keeps "fell from a high
    /// place"; the entry's own shelf the
    /// RESET_COMBAT_STATUS_TIME 300 the class-file constant walk
    /// pins, the fall's own landing the lastDamageTime re-stamp
    /// wears): the REAL melee lane banks the (kind, stamp, mid-fall)
    /// triple (the zombie's bite the addRecord arm walks), the fall's lethal
    /// dose the shover's own name the death row wears, the respawn
    /// the fresh tracker the None clears, the expired window the
    /// kill credit the escape row wears, the creditless fall the
    /// accident row keeps.
    #[test]
    fn the_fall_names_its_shover() {
        // THE REAL MELEE'S OWN BANK (the zombie's bite the hit_events
        // walk lands — the hurt_by_named success the addRecord arm
        // rides, the (kind, stamp) pair the memory banks)
        let mut app = App::new(5);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        let (px, pz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.z)
        };
        // THE SKY PAD (the spawn's enclosure the generated terrain
        // keeps — the flat stand the test's own world-set builds: the
        // stone row the y=60 plane wears, the body and the zombie the
        // same stand share, the sky the eye-to-eye ray the open air
        // answers — the melee's canUse the range+LOS pair the pad
        // serves)
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -2i32..=2 {
                for dz in -2i32..=2 {
                    world.set(px.floor() as i32 + dx, 60, pz.floor() as i32 + dz, crate::world::STONE);
                }
            }
        }
        if let Some(b) = app.body.as_mut() {
            b.y = 61.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.fall_distance = 0.0;
            b.on_ground = true;
        }
        let mut zombie = crate::mob::Zombie::new(60, px + 1.0, 61.0, pz, 61);
        zombie.on_ground = true;
        app.mobs.push(zombie);
        let mut bitten = false;
        for _ in 0..200 {
            app.update();
            if app.last_hurt_by_mob.is_some() && app.health < 20.0 {
                bitten = true;
                break;
            }
        }
        assert!(
            bitten,
            "the bite the memory banks (health: {}, zombie at {:.1},{:.1},{:.1}, player at {:.1},{:.1}, fire: {}, aggr: {}, mobs: {}, diff: {:?})",
            app.health,
            app.mobs.first().map(|m| m.x).unwrap_or(-1.0),
            app.mobs.first().map(|m| m.y).unwrap_or(-1.0),
            app.mobs.first().map(|m| m.z).unwrap_or(-1.0),
            px, pz,
            app.mobs.first().map(|m| m.fire_ticks).unwrap_or(-1),
            app.mobs.first().map(|m| m.aggressive).unwrap_or(false),
            app.mobs.len(),
            app.offline_world.as_ref().map(|w| w.difficulty),
        );
        assert_eq!(
            app.last_hurt_by_mob.map(|(k, _, _)| k),
            Some(crate::mob::MobKind::Zombie),
            "the zombie's own kind the entry banks"
        );
        // THE FALL'S OWN WORD (the open window the assist row wears —
        // the shover's own display name the %2$s fills; the zombie
        // the cleared mobs the repeat bites never land, the lethal
        // dose the walk's own unit arms the established shape drives)
        app.mobs.clear();
        app.apply_fall_damage(20);
        assert!(app.dead, "the lethal fall the crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was doomed to fall by Zombie"),
            "the assist row the shover names"
        );
        // THE RESPAWN'S OWN CLEAR (the vanilla respawn re-creates the
        // LivingEntity — the fresh CombatTracker's empty entries the
        // new body carries)
        app.respawn();
        assert!(
            app.last_hurt_by_mob.is_none(),
            "the respawn the tracker clears"
        );
        // THE EXPIRED SHELF'S OWN ROW (the 300-tick retention the
        // stamp long past speaks — the entries the recheckStatus
        // walk cleared, the kill credit the memory keeps the escape
        // row wears; the tick walk the frame far ahead rides, the
        // stamp the expired shelf reads)
        app.frame = 1000;
        app.last_hurt_by_mob = Some((crate::mob::MobKind::Zombie, 0, false));
        app.apply_fall_damage(20);
        assert!(app.dead, "the second fall the crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs hit the ground too hard while trying to escape Zombie"),
            "the expired shelf the kill credit the escape row wears"
        );
        assert!(
            app.last_hurt_by_mob.is_some(),
            "the expired shelf the kill credit keeps"
        );
        app.respawn();
        // THE CREDITLESS FALL'S OWN ROW (the no-credit lane the
        // accident row keeps — the vanilla getKillCredit null the
        // source's own walk refuses; the respawn the memory cleared,
        // the fresh body the fall the plain accident row wears)
        app.frame = 2000;
        app.apply_fall_damage(20);
        assert!(app.dead, "the third fall the crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs fell from a high place"),
            "the creditless fall the accident row keeps"
        );
        app.respawn();
        // THE KIND'S OWN FLOW (the skeleton's shot the same pair
        // banks — the display name the assist row's %2$s the
        // shooter's own kind wears)
        app.last_hurt_by_mob = Some((crate::mob::MobKind::Skeleton, app.frame, false));
        app.apply_fall_damage(20);
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was doomed to fall by Skeleton"),
            "the assist row the skeleton names"
        );
    }


    /// THE FALL THE MID-AIR BLOW THE FINISH ROW WEARS (the 26.2
    /// CombatTracker's getMessageForAssistedFall the assist/finish
    /// pair rides — the entry's own mid-fall flavor the row pick
    /// answers): the hit DURING the fall the finish row speaks
    /// ("death.fell.finish" = "%1$s fell too far and was finished by
    /// %2$s" the en_us carries), the grounded shove the assist row
    /// keeps ("death.fell.assist" = "was doomed to fall by %2$s" the
    /// autopush-622 walk serves), the sim's bare-handed mobs the
    /// .item row the honest absence never speaks.
    #[test]
    fn the_fall_is_finished_by_its_hitter() {
        // THE MID-AIR BITE'S OWN BANK (the float shape the race-free
        // read keeps — the body the pad's own open sky hovers, the
        // melee gate the horizontal read the dy never reads
        // (dist2_xz the within_melee_range_width law walks), the
        // airborne the fall distance carries the flag the TRUE
        // banks)
        let mut app = App::new(5);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        let (px, pz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.z)
        };
        // THE SKY PAD (the shover test's own shape — the flat stand
        // the test's world-set builds)
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -2i32..=2 {
                for dz in -2i32..=2 {
                    world.set(
                        px.floor() as i32 + dx,
                        60,
                        pz.floor() as i32 + dz,
                        crate::world::STONE,
                    );
                }
            }
        }
        if let Some(b) = app.body.as_mut() {
            b.y = 62.5;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.fall_distance = 0.5;
            b.on_ground = false;
        }
        let mut zombie = crate::mob::Zombie::new(60, px + 1.0, 61.0, pz, 61);
        zombie.on_ground = true;
        app.mobs.push(zombie);
        let mut bitten_mid_fall = false;
        for _ in 0..200 {
            // THE HOVER'S OWN RE-ARM (the airborne state the tick's
            // own step re-arms — the gravity's tick-pull the next arm
            // answers, the pad 1.5 below the collision never reaches)
            if let Some(b) = app.body.as_mut() {
                b.on_ground = false;
                b.vy = 0.0;
                b.fall_distance = b.fall_distance.max(0.5);
            }
            app.update();
            if app.last_hurt_by_mob.map(|(_, _, m)| m) == Some(true)
                && app.health < 20.0
            {
                bitten_mid_fall = true;
                break;
            }
        }
        assert!(
            bitten_mid_fall,
            "the mid-air bite the flag banks (health={} entry={:?})",
            app.health, app.last_hurt_by_mob
        );
        assert_eq!(
            app.last_hurt_by_mob.map(|(k, _, _)| k),
            Some(crate::mob::MobKind::Zombie),
            "the zombie's own kind the entry banks"
        );
        // THE FINISH'S OWN ROW (the open window the mid-fall flavor
        // the finish row wears — the shover's own display name the
        // %2$s fills, the mobs cleared the repeat bites never land)
        app.mobs.clear();
        app.apply_fall_damage(20);
        assert!(app.dead, "the lethal fall the crossing banks");
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs fell too far and was finished by Zombie"),
            "the finish row the mid-fall blow's own word wears"
        );
        // THE LANG ROW'S OWN PIN (the 26.2 en_us the walk's own word
        // keeps; the .item row the sim's bare hands the honest
        // absence never speaks)
        assert_eq!(
            tr("death.fell.finish"),
            Some("%1$s fell too far and was finished by %2$s"),
            "the finish row the lang table carries"
        );
        // THE GROUNDED SHOVE'S OWN CONTRAST (the same bite the ground
        // lands — the flag the FALSE banks, the assist row the same
        // walk wears: the split the hit's own instant decides)
        app.respawn();
        app.mobs.clear();
        let (px2, pz2) = {
            let b = app.body.as_ref().expect("the respawned body");
            (b.x, b.z)
        };
        if let Some(b) = app.body.as_mut() {
            b.y = 61.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.fall_distance = 0.0;
            b.on_ground = true;
        }
        let mut z2 = crate::mob::Zombie::new(62, px2 + 1.0, 61.0, pz2, 63);
        z2.on_ground = true;
        app.mobs.push(z2);
        let mut bitten_ground = false;
        for _ in 0..200 {
            app.update();
            if app.last_hurt_by_mob.map(|(_, _, m)| m) == Some(false)
                && app.health < 20.0
            {
                bitten_ground = true;
                break;
            }
        }
        assert!(
            bitten_ground,
            "the grounded bite the FALSE flag banks (health={} entry={:?})",
            app.health, app.last_hurt_by_mob
        );
        app.mobs.clear();
        app.apply_fall_damage(20);
        assert_eq!(
            app.death_message.as_deref(),
            Some("mc2rs was doomed to fall by Zombie"),
            "the assist row the grounded shove wears"
        );
    }


    /// THE EGG'S OWN THROW (javap `EggItem.use` + `ThrownEgg` the 26.2
    /// law speaks): the press the arc wears — the power 1.5 the
    /// gaussian spread's tiny band keeps, the eye minus 0.1 the spawn
    /// pose wears, the survival stack the consume wears; the hit the
    /// discard speaks — the wall the clip ray banks, the ten shards
    /// the broadcast-3 wears, the rolls the chicks hatch (the
    /// 1/8 the 1/32 family's own draw), the baby the negative age
    /// wears; the mob the 0-dose word speaks (the health the fold
    /// never spends, the flash the walk re-arms, the thrower's own
    /// word the aggro arms).
    #[test]
    fn the_egg_the_throw_the_arc_the_hit_wears() {
        // THE THROW'S OWN ARM (the use lane the RMB press wears — the
        // sky range the shover test's own shape builds, the wall the
        // throw's own target: the stone column the ray crosses)
        let mut app = App::new(5);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        let (px, pz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.z)
        };
        // THE SKY PAD'S OWN LEVEL (the world's CHUNK_Y 48 the terrain's
        // tops the spawn's enclosure the eye-lines refuse — the gotcha
        // the flank probes banked: the pad the open sky the flight's
        // own lane wears, the level inside the ceiling the cells
        // answer)
        let gy = 30i32;
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4i32..=4 {
                for dz in -4i32..=4 {
                    world.set(
                        px.floor() as i32 + dx,
                        gy - 1,
                        pz.floor() as i32 + dz,
                        crate::world::STONE,
                    );
                }
            }
        }
        // THE THROW'S OWN LOOK FIRST (the body's yaw the wall's own
        // probe reads — the spawn's own yaw the east-facing stand-in
        // wears, the re-set the yaw-0 look the throw's ray walks)
        if let Some(b) = app.body.as_mut() {
            b.y = gy as f64;
            b.yaw = 0.0;
            b.pitch = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.on_ground = true;
        }
        {
            let world = app.offline_world.as_mut().unwrap();
            // THE WALL ALONG THE ACTUAL LOOK (the camera's own forward
            // the throw's ray walks — the convention the probe pins,
            // the 3-wide panel the spread's tiny band covers)
            let fwd = {
                let b = app.body.as_ref().expect("the body");
                let c = b.camera();
                (c.forward().x as f64, c.forward().z as f64)
            };
            let wx = (px + fwd.0 * 4.0).floor() as i32;
            let wz = (pz + fwd.1 * 4.0).floor() as i32;
            for ddx in -1i32..=1 {
                for dy in 0..=2 {
                    world.set(wx + ddx, gy + dy, wz, crate::world::STONE);
                }
            }
        }
        let egg_id = crate::mob::wire_id("egg").expect("the egg's own id");
        app.inventory.items[app.selected_hotbar] = (egg_id, 3);
        assert!(
            app.on_right_press_main_item_lane(),
            "the egg's own throw the lane speaks"
        );
        assert_eq!(app.eggs.len(), 1, "the egg the press spawns");
        assert_eq!(
            app.inventory.items[app.selected_hotbar].1,
            2,
            "the survival stack the consume wears"
        );
        // THE ARC'S OWN WORD (the power 1.5 the spread's own band
        // keeps, the eye minus 0.1 the spawn pose wears)
        {
            let e = &app.eggs[0];
            let speed = (e.vx * e.vx + e.vy * e.vy + e.vz * e.vz).sqrt();
            assert!(
                (speed - crate::mob::EGG_THROW_POWER).abs() < 0.05,
                "the power the arc keeps ({speed})"
            );
            assert!(
                (e.y - (gy as f64 + 1.52)).abs() < 1e-9,
                "the eye minus 0.1 the spawn pose wears"
            );
        }
        // THE HIT'S OWN WALK (the wall the ray crosses — the discard
        // the break speaks, the ten shards the broadcast-3 wears)
        for _ in 0..10 {
            app.update();
            if app.eggs.is_empty() {
                break;
            }
        }
        assert!(
            app.eggs.is_empty(),
            "the hit the discard speaks (eggs={:?})",
            app.eggs
                .iter()
                .map(|e| (e.x, e.y, e.z, e.vx, e.vy, e.vz))
                .collect::<Vec<_>>()
        );
        let shards = app
            .particles
            .iter()
            .filter(|p| matches!(p.kind, crate::particles::ParticleKind::EggItem))
            .count();
        assert_eq!(shards, 10, "the shatter the ten shards wear");

        // THE MOB HIT'S OWN WORD (the 0-dose the ThrowableProjectile
        // speaks — the health the fold never spends, the flash the
        // walk re-arms, the thrower's own word the aggro arms)
        app.respawn();
        app.mobs.clear();
        app.particles.clear();
        let (px2, pz2) = {
            let b = app.body.as_ref().expect("the respawned body");
            (b.x, b.z)
        };
        if let Some(b) = app.body.as_mut() {
            b.y = gy as f64;
            b.yaw = 0.0;
            b.pitch = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.on_ground = true;
        }
        let (fx, fz) = {
            let b = app.body.as_ref().expect("the body");
            let c = b.camera();
            (c.forward().x as f64, c.forward().z as f64)
        };
        let mut z = crate::mob::Zombie::new(900, px2 + fx * 2.5, gy as f64, pz2 + fz * 2.5, 901);
        z.on_ground = true;
        app.mobs.push(z);
        app.inventory.items[app.selected_hotbar] = (egg_id, 1);
        let health_before = app.mobs.iter().find(|m| m.id == 900).unwrap().health;
        assert!(
            app.on_right_press_main_item_lane(),
            "the egg's own throw the lane speaks"
        );
        for _ in 0..10 {
            app.update();
            if app.eggs.is_empty() {
                break;
            }
        }
        assert!(app.eggs.is_empty(), "the mob hit the discard speaks");
        let zombie = app.mobs.iter().find(|m| m.id == 900).unwrap();
        assert_eq!(
            zombie.health, health_before,
            "the 0-dose the health never spends"
        );
        assert!(zombie.hurt_time > 0, "the flash the walk re-arms");

        // THE CHICK'S OWN ROLLS (the 40 throws the deterministic
        // stream spends — the exact count the fixed seed pins, the
        // 1/8 roll the chick banks, the nested 1/32 the four-family
        // speaks, the baby the negative age wears, the variant the
        // world's own biome word picks)
        app.respawn();
        app.mobs.clear();
        app.particles.clear();
        let (px3, pz3) = {
            let b = app.body.as_ref().expect("the respawned body");
            (b.x, b.z)
        };
        {
            let world = app.offline_world.as_mut().unwrap();
            for dx in -4i32..=4 {
                for dz in -4i32..=4 {
                    world.set(
                        px3.floor() as i32 + dx,
                        gy - 1,
                        pz3.floor() as i32 + dz,
                        crate::world::STONE,
                    );
                }
            }
        }
        if let Some(b) = app.body.as_mut() {
            b.y = gy as f64;
            b.yaw = 0.0;
            b.pitch = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.on_ground = true;
        }
        {
            let world = app.offline_world.as_mut().unwrap();
            // THE WALL ALONG THE ACTUAL LOOK (the first arm's own
            // probe the re-set keeps — the yaw the same 0.0 wears)
            let fwd = {
                let b = app.body.as_ref().expect("the body");
                let c = b.camera();
                (c.forward().x as f64, c.forward().z as f64)
            };
            let wx = (px3 + fwd.0 * 4.0).floor() as i32;
            let wz = (pz3 + fwd.1 * 4.0).floor() as i32;
            for ddx in -1i32..=1 {
                for dy in 0..=2 {
                    world.set(wx + ddx, gy + dy, wz, crate::world::STONE);
                }
            }
        }
        if let Some(b) = app.body.as_mut() {
            b.y = gy as f64;
            b.yaw = 0.0;
            b.pitch = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.on_ground = true;
        }
        app.inventory.items[app.selected_hotbar] = (egg_id, 50);
        for _ in 0..40 {
            if let Some(b) = app.body.as_mut() {
                b.y = gy as f64;
                b.yaw = 0.0;
                b.pitch = 0.0;
                b.on_ground = true;
            }
            let _ = app.on_right_press_main_item_lane();
            for _ in 0..6 {
                app.update();
            }
        }
        for _ in 0..40 {
            app.update();
        }
        let chicks: Vec<&crate::mob::Zombie> = app
            .mobs
            .iter()
            .filter(|m| m.kind == crate::mob::MobKind::Chicken)
            .collect();
        assert!(
            chicks.iter().all(|m| m.age < 0),
            "the baby the negative age wears"
        );
        assert!(
            chicks
                .iter()
                .all(|m| m.farm_variant == Some(crate::mob::chicken_variant_for_biome(spawn_biome(app.seed)))),
            "the variant the world's own biome word picks"
        );
        let pinned = chicks.len();
        assert!(pinned > 0, "the chicks the rolls hatch ({pinned})");
    }


    /// THE RESISTANCE'S OWN FOLD (javap getDamageAfterMagicAbsorb pcs
    /// 0-73 — the fold the actuallyHurt's magic lane wears): the amp-0
    /// body the 5.0 blow the 4.0 survivor spends, the cooldown's
    /// last_hurt the RAW word keeps (the 4.5 window arm the raw
    /// comparison refuses, the 6.0 the 1.0 difference the fold wears),
    /// the amp-4 the j-zero the full immunity speaks, the STARVE the
    /// BYPASSES_EFFECTS member the fold never wears, the /kill the
    /// generic_kill identity rides.
    #[test]
    fn the_resistance_the_doses_the_raw_word_wears() {
        // THE AMP-0 WALK (the fold the 5.0 blow wears, the raw word
        // the window's own comparison reads)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        assert!(app.effects.add_effect(
            crate::effects::MOB_EFFECT_RESISTANCE,
            0,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS
        ));
        app.health = 20.0;
        assert!(app.hurt(5.0), "the fresh dose lands");
        assert_eq!(
            app.health,
            20.0 - crate::effects::resistance_scale(0, 5.0),
            "the folded survivor the health spends"
        );
        // THE RAW WORD (the window arm the UNFOLDED comparison reads —
        // the 4.5 folded would pass the 3.2 survivor, the 4.5 raw the
        // 5.0 last_hurt refuses)
        assert!(!app.hurt(4.5), "the raw word the window keeps");
        assert_eq!(app.health, 20.0 - crate::effects::resistance_scale(0, 5.0));
        // THE STRONGER RAW DOSE (the 1.0 difference the fold wears —
        // the last_hurt the 6.0 raw banks)
        assert!(app.hurt(6.0), "the stronger raw dose the window admits");
        assert_eq!(
            app.health,
            20.0 - crate::effects::resistance_scale(0, 5.0)
                - crate::effects::resistance_scale(0, 1.0),
            "the difference arm the fold wears"
        );

        // THE AMP-4 WALK (the j-zero the max(0) clamp — the full
        // immunity the health the dose never spends)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_RESISTANCE,
            4,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.health = 20.0;
        assert!(app.hurt(5.0), "the landed hurt the zero dose wears");
        assert_eq!(app.health, 20.0, "the zero survivor the health keeps");

        // THE STARVE WALK (the BYPASSES_EFFECTS member — the 1.0 bite
        // the fold never wears, the unscaled 1.0 the health speaks)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_RESISTANCE,
            0,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.health = 20.0;
        assert!(app.hurt_starving(crate::food::STARVE_DAMAGE));
        assert_eq!(
            app.health,
            20.0 - crate::food::STARVE_DAMAGE,
            "the starve the unscaled bite wears"
        );

        // THE /KILL WALK (the generic_kill the BYPASSES_RESISTANCE
        // member — the Float.MAX_VALUE the identity pass speaks, the
        // death the health-floor tail arms)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_RESISTANCE,
            0,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.health = 20.0;
        app.chat_open = true;
        app.chat_input = "/kill".into();
        app.on_chat_submit();
        assert!(app.dead, "the generic_kill the resistance never gates");
        assert_eq!(app.health, 0.0, "the kill walk the health floors");
    }


    /// THE WEAKNESS'S OWN MELEE WALK (javap MobEffects clinit pcs
    /// 469-503 the addAttributeModifier(ATTACK_DAMAGE, "effect.weakness",
    /// -4.0d, ADD_VALUE) row + the AttributeTemplate.create(int) the
    /// amount × (amp+1) product + the RangedAttribute's sanitizeValue
    /// the 0.0 floor keeps): the amp-0 fist the zero attribute the
    /// full charge the 0.0 product speaks (the dose the delivery
    /// never carries, the onAttack reset the gate still walks), the
    /// diamond blade the 2.0+6.0−4.0 the 4.0 survivor wears (the
    /// armor fold the zombie's own 2.0 the ×0.94 speaks), the bare
    /// contrast the 8.0 attribute the identity rides.
    #[test]
    fn the_weakness_the_fist_the_floor_wears() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE FLOOR WALK (the amp-0 fist the 2.0−4.0 the clamp floors)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_WEAKNESS,
            0,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        app.mobs.push(crate::mob::Zombie::new(
            7,
            c as f64 + 3.0,
            top as f64 + 1.62,
            c as f64 + 0.5,
            3,
        ));
        {
            let (tx, ty, tz) = {
                let m = &app.mobs[0];
                (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
            };
            let b = app.body.as_mut().unwrap();
            let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
            let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
            let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
            b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees()
                as f32;
            b.pitch = -(dy / len3).asin().to_degrees() as f32;
        }
        // the fist's own charge window (the 5.0 delay the 5 ticker the
        // full charge speaks)
        app.attack_ticker = 5;
        app.attack_mob(0);
        assert_eq!(
            app.mobs[0].health,
            crate::mob::ZOMBIE_MAX_HEALTH,
            "the zero attribute the dose never carries"
        );
        // the swing still re-armed (onAttack's own reset the dmg gate
        // never blocks)
        assert_eq!(app.attack_ticker, 0, "the reset the floor walk keeps");

        // THE BLADE WALK (the 2.0+6.0−4.0 the 4.0 survivor wears — the
        // armor fold the zombie's own law speaks)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_WEAKNESS,
            0,
            400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.inventory.items[app.selected_hotbar] = (wire("diamond_sword"), 1);
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        app.mobs.push(crate::mob::Zombie::new(
            7,
            c as f64 + 3.0,
            top as f64 + 1.62,
            c as f64 + 0.5,
            3,
        ));
        {
            let (tx, ty, tz) = {
                let m = &app.mobs[0];
                (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
            };
            let b = app.body.as_mut().unwrap();
            let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
            let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
            let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
            b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees()
                as f32;
            b.pitch = -(dy / len3).asin().to_degrees() as f32;
        }
        // the sword's own charge window (the 12.5 delay the 13 ticker
        // the full charge speaks)
        app.attack_ticker = 13;
        app.attack_mob(0);
        // the exact f32 mirror: the 4.0 dose the armor law's own f6
        // fold (the f6 the DOSE's own word wears — armor − amount/2
        // the 0.4 floor clamps, the magic lane the unworn identity
        // rides)
        let f6_for = |amount: f32| {
            (crate::mob::ZOMBIE_ARMOR as f32 - amount / 2.0_f32)
                .clamp(crate::mob::ZOMBIE_ARMOR as f32 * 0.2, 20.0)
        };
        assert_eq!(
            app.mobs[0].health,
            crate::mob::ZOMBIE_MAX_HEALTH - 4.0_f32 * (1.0 - f6_for(4.0) / 25.0),
            "the weakened blade the 4.0 survivor wears"
        );

        // THE BARE CONTRAST (the unworn body the 8.0 attribute the
        // identity rides — the same blade the weakness's absence
        // speaks)
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        app.inventory.items[app.selected_hotbar] = (wire("diamond_sword"), 1);
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        app.mobs.push(crate::mob::Zombie::new(
            7,
            c as f64 + 3.0,
            top as f64 + 1.62,
            c as f64 + 0.5,
            3,
        ));
        {
            let (tx, ty, tz) = {
                let m = &app.mobs[0];
                (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
            };
            let b = app.body.as_mut().unwrap();
            let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
            let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
            let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
            b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees()
                as f32;
            b.pitch = -(dy / len3).asin().to_degrees() as f32;
        }
        app.attack_ticker = 13;
        app.attack_mob(0);
        assert_eq!(
            app.mobs[0].health,
            crate::mob::ZOMBIE_MAX_HEALTH - 8.0_f32 * (1.0 - f6_for(8.0) / 25.0),
            "the bare blade the 8.0 attribute wears"
        );
    }


    /// THE EFFECT DOSES' OWN MOTION WIRING (javap the attribute lanes
    /// — the speed/slowness rows' ×(1+amount×(amp+1)) product the
    /// movement fold banks, the jump_boost's 0.1×(amp+1) power the
    /// jump floor reads, the SAFE_FALL_DISTANCE's +1.0×(amp+1) lift
    /// the fall grace wears): the commands the store speaks, the
    /// body's own fields the one update each carries.
    #[test]
    fn the_effect_doses_the_motion_wiring_speaks() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.mobs.clear();
        // THE SPEED DOSE (the no-amp give the ×1.2 product banks)
        app.chat_open = true;
        app.chat_input = "/effect give @s speed".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.body.as_ref().unwrap().effect_speed_mult,
            1.0 + crate::body::SPEED_EFFECT_MOVEMENT_MODIFIER,
        );
        // THE AMPLIFIER LIFT (the amp-1 upgrade the ×1.4 product wears)
        app.chat_open = true;
        app.chat_input = "/effect give @s speed 30 1".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.body.as_ref().unwrap().effect_speed_mult,
            1.0 + crate::body::SPEED_EFFECT_MOVEMENT_MODIFIER * 2.0,
        );
        // THE PAIR FOLD (the slowness III the ×0.4 step the product
        // multiplies — the two ×(1+amount) steps the calculate law
        // keeps)
        app.chat_open = true;
        app.chat_input = "/effect give @s slowness 30 3".into();
        app.on_chat_submit();
        app.update();
        let expected = (1.0 + crate::body::SPEED_EFFECT_MOVEMENT_MODIFIER * 2.0)
            * (1.0 + crate::body::SLOWNESS_EFFECT_MOVEMENT_MODIFIER * 4.0);
        let got = app.body.as_ref().unwrap().effect_speed_mult;
        assert!(
            (got - expected).abs() < 1e-6,
            "the pair fold speaks: {got} vs {expected}"
        );
        // THE SANITIZE FLOOR (the extreme slowness the ZERO speed
        // wears — the negative product the [0, ..] clamp refuses)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        app.chat_open = true;
        app.chat_input = "/effect give @s slowness 30 6".into();
        app.on_chat_submit();
        app.update();
        assert_eq!(
            app.body.as_ref().unwrap().effect_speed_mult,
            0.0,
            "the amp-6 slowness the zero speed the clamp wears"
        );
        // THE JUMP BOOST'S OWN LANES (the power the jump floor reads,
        // the threshold the fall grace lifts — the amp-3 dose the
        // 0.4/7.0 pair wears)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        app.chat_open = true;
        app.chat_input = "/effect give @s jump_boost 30 3".into();
        app.on_chat_submit();
        app.update();
        let b = app.body.as_ref().unwrap();
        assert_eq!(
            b.jump_boost_power,
            crate::body::JUMP_BOOST_POWER_PER_LEVEL * 4.0,
        );
        assert_eq!(
            b.safe_fall_distance,
            crate::body::SAFE_FALL_DISTANCE + 4.0,
        );
        // THE CLEAR RESTORE (the empty store the defaults the body
        // wears — the 1.0/0.0/3.0 the no-effect body carries)
        app.chat_open = true;
        app.chat_input = "/effect clear".into();
        app.on_chat_submit();
        app.update();
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.effect_speed_mult, 1.0);
        assert_eq!(b.jump_boost_power, 0.0);
        assert_eq!(b.safe_fall_distance, crate::body::SAFE_FALL_DISTANCE);
    }


    #[test]
    fn splash_selection_is_deterministic_and_in_range() {
        let a = App::new(20260214);
        let b = App::new(20260214);
        assert_eq!(a.splash_index, b.splash_index);
        // THE PICK'S OWN LAW (the cache-absent boot the CI's pre-generate
        // test job speaks 0 — the empty-table arm the fallback renders;
        // the cache-present boot the splitmix index rides)
        let expected = if a.splashes.is_empty() {
            0
        } else {
            (splitmix(20260214) % a.splashes.len() as u64) as usize
        };
        assert_eq!(a.splash_index, expected, "the pick the list's own length speaks");
    }


    #[test]
    fn the_splash_loader_speaks_the_parse_law() {
        // THE MIGRATION'S OWN LAW (the parse the old generator's walk
        // spoke, now the loader's): the '\n' split, the '\r' trim, the
        // empty lines' filter, the empty bytes the honest empty keeps
        let lines = parse_splashes(b"As seen on TV!\nAwesome!\n\n100% pure!\r\nMore polygons!\n");
        assert_eq!(
            lines,
            vec!["As seen on TV!", "Awesome!", "100% pure!", "More polygons!"],
            "the parse law the cache file's own lines keep"
        );
        assert!(
            parse_splashes(b"").is_empty(),
            "the empty file the honest empty list keeps"
        );
    }


    #[test]
    fn the_lang_loader_speaks_the_parse_law() {
        // THE MIGRATION'S OWN LAW (the parse the old generator's collect
        // walk spoke, now the loader's): the flat keys the direct entries
        // keep, the nested objects the dotted-key join walks, the escapes
        // the serde_json read answers, the broken/non-object bytes the
        // honest empty map keeps
        let map = parse_lang(
            br#"{"menu.singleplayer":"Singleplayer","menu":{"multiplayer":"Multiplayer"},"quote":"He said \"hi\""}"#,
        );
        assert_eq!(
            map.get("menu.singleplayer").map(String::as_str),
            Some("Singleplayer"),
            "the flat key the direct entry keeps"
        );
        assert_eq!(
            map.get("menu.multiplayer").map(String::as_str),
            Some("Multiplayer"),
            "the nested object the dotted-key join walks"
        );
        assert_eq!(
            map.get("quote").map(String::as_str),
            Some("He said \"hi\""),
            "the escapes the json read answers"
        );
        assert_eq!(map.len(), 3, "the non-string entries the skip law keeps");
        assert!(
            parse_lang(b"not json").is_empty(),
            "the broken file the honest empty map keeps"
        );
        assert!(
            parse_lang(b"[]").is_empty(),
            "the non-object the honest empty map keeps"
        );
    }


    #[test]
    fn panorama_offset_is_deterministic_and_wraps() {
        let strip = 540.0;
        assert_eq!(panorama_offset(0, strip), 0.0);
        assert_eq!(panorama_offset(30, strip), 7.5);
        assert_eq!(panorama_offset(30, strip), panorama_offset(30, strip));
        // wraps: frame (strip/0.25) is exactly one full strip
        let wrap_frame = (strip / 0.25) as u64;
        assert!(panorama_offset(wrap_frame, strip) < 0.25);
        // degenerate strip never NaNs; offset stays within [0, strip)
        let o = panorama_offset(123, 0.0);
        assert!(o.is_finite() && (0.0..1.0).contains(&o));
    }


    #[test]
    fn boot_progress_transitions_to_title() {
        let mut app = App::new(7);
        assert_eq!(app.screen, ScreenKind::Loading);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.screen, ScreenKind::Title, "boot completes after 20 ticks");
        assert_eq!(app.boot_progress, 1.0);
    }


    #[test]
    fn singleplayer_click_enters_world_with_mesh() {
        let mut app = App::new(3);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.screen, ScreenKind::Title);
        app.render_title_buttons_for_test();
        let sp = app.buttons.iter().find(|b| b.id == "singleplayer").expect("singleplayer button").clone();
        app.on_click(sp.x + sp.w / 2, sp.y + sp.h / 2);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(!app.paused);
        let quads = app.world_quads.as_ref().expect("world meshed on entry");
        assert!(!quads.is_empty(), "mesh produced faces");
        assert!(app.spawn_y > 0, "spawn eye above bedrock");
    }


    #[test]
    fn world_music_follows_the_official_spawn_biome() {
        // the situation is resolved from the seeded spawn biome through the
        // generated biome-music table: the playing event is either the
        // biome's official background-music event or the vanilla in-game
        // default for biomes without one; deterministic per seed either way
        use mc_registry::generated::worldgen as wm;
        let seed = 3;
        let biome = spawn_biome(seed);
        assert!(wm::biome(biome).is_some(), "{biome} must be an official biome");
        let expect = wm::biome_music(biome, "default")
            .map(|m| m.event)
            .unwrap_or("music.game");
        let mut app = App::new(seed);
        app.enter_world();
        assert_eq!(app.music.mode(), crate::music::MusicMode::Game);
        assert_eq!(
            app.music.game_tracks(),
            &[expect] as &[&str],
            "seed {seed} biome {biome} must resolve to {expect}"
        );
        // determinism: the same seed resolves the same situation
        let mut again = App::new(seed);
        again.enter_world();
        assert_eq!(again.music.game_tracks(), app.music.game_tracks());
    }


    #[test]
    fn escape_toggles_pause_over_world() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(!app.paused);
        app.on_escape();
        assert!(app.paused);
        app.on_escape();
        assert!(!app.paused);
        // escape on menus is a no-op
        app.open(ScreenKind::Title);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::Title);
    }


    #[test]
    fn the_f3_words_the_vanilla_entries_speak() {
        let mut app = App::new(3);
        app.enter_world();
        // THE TOGGLE (the F3 key's own flip — the rising edge the
        // press answers, the next press re-answers)
        assert!(!app.debug_held);
        app.on_debug_key();
        assert!(app.debug_held);
        app.on_debug_key();
        assert!(!app.debug_held);
        app.on_debug_key();
        assert!(app.debug_held);

        // THE BODY'S OWN LINES (the offline body the spawn law
        // materializes — the entries the camera entity's own words
        // speak, each format the javap ldc verbatim keeps): the pose
        // the ZERO rotation re-arms (the spawn yaw the world's own
        // facing wears — the quadrant rows the canonical zeros speak)
        app.spawn_offline_body();
        {
            let w = app.body.as_mut().unwrap();
            w.yaw = 0.0;
            w.pitch = 0.0;
        }
        let b = app.body.as_ref().unwrap().clone();
        let lines = app.debug_lines();
        // DebugEntryVersion's `Minecraft \u0001 (\u0001/\u0001)` — the
        // tracked version the name and the launched word both keep
        assert_eq!(lines[0], "Minecraft 26.2 (26.2/vanilla)");
        // DebugEntryFps's `%d fps T: %s%s` — the no-tracker lane the
        // inf word keeps (the window never closed the "--" the
        // pre-first-window row wears)
        assert!(lines[1].ends_with(" fps T: inf"), "{}", lines[1]);
        // `XYZ: %.3f / %.5f / %.3f` — the Y the five-decimal word keeps
        assert_eq!(
            lines[2],
            format!("XYZ: {:.3} / {:.5} / {:.3}", b.x, b.y, b.z)
        );
        // `Block: %d %d %d` — the blockPosition's own floor
        let (bx, by, bz) =
            (b.x.floor() as i32, b.y.floor() as i32, b.z.floor() as i32);
        assert_eq!(lines[3], format!("Block: {bx} {by} {bz}"));
        // `Chunk: %d %d %d [%d %d in r.%d.%d.mca]` — the region math
        // the (& 31, >> 5) pair keeps
        let (cx, cz) = (bx >> 4, bz >> 4);
        assert_eq!(
            lines[4],
            format!(
                "Chunk: {cx} {} {cz} [{} {} in r.{}.{}.mca]",
                by >> 4,
                cx & 31,
                cz & 31,
                cx >> 5,
                cz >> 5
            )
        );
        // `Facing: %s (%s) (%.1f / %.1f)` — the spawn yaw the south
        // quadrant wears, the wrapDegrees pair the 0.0 the zero
        // rotation speaks
        assert!(
            lines[5].starts_with("Facing: south (Towards positive Z) (0.0 / 0.0)"),
            "{}",
            lines[5]
        );
        // `Biome: \u0001` — the seed word the full registry id keeps
        assert!(lines[6].starts_with("Biome: minecraft:"), "{}", lines[6]);
        // `Client Light: \u0001 (\u0001 sky, \u0001 block)` — the eye
        // cell's own pair the open-sky column walk speaks
        assert!(lines[7].starts_with("Client Light: "), "{}", lines[7]);

        // THE FACING QUADRANTS (Direction.fromYRot's own four-way fold
        // — `floor(yaw/90 + 0.5) & 3` over the HORIZONTALS table: the
        // 90/180/-90 yaws the west/north/east words wear, the Towards
        // words the switchmap's own table banks)
        for (yaw, word) in [
            (90.0f32, "west (Towards negative X)"),
            (180.0, "north (Towards negative Z)"),
            (-90.0, "east (Towards positive X)"),
        ] {
            app.body.as_mut().unwrap().yaw = yaw;
            let line = app.debug_lines()[5].clone();
            assert!(
                line.starts_with(&format!("Facing: {word}")),
                "yaw {yaw}: {line}"
            );
        }
    }


    #[test]
    fn the_mood_percent_folds_the_math_round_law() {
        // THE FOLD'S OWN PINS (javap Math.round(F)I — the
        // floor(x + 0.5f) half-up walk the f32 width keeps): the zero
        // the fresh accumulator speaks, the sub-half the floor
        // answers, the .5 tie the positive-infinity law rounds up,
        // the hundred the fired machine's own ceiling wears (the
        // 1.0f32 * 100 + 0.5 the floor's exact 100.5 wears — the int
        // cast the floor answers)
        assert_eq!(mood_percent(0.0), 0);
        assert_eq!(mood_percent(0.004_9), 0);
        assert_eq!(mood_percent(0.123_4), 12);
        assert_eq!(mood_percent(0.5), 50);
        // the .5-percent tie: 0.005f32 * 100.0f32 rounds the f32
        // nearest the product speaks — 0.5 exactly (0.005f32 is
        // 0.00499999988..., the f32 multiply the half it rounds to),
        // the +0.5 the exact 1.0 wears, the floor the 1 speaks — the
        // JVM's own Math.round verdict the same walk answers
        assert_eq!(mood_percent(0.005), 1);
        assert_eq!(mood_percent(0.999), 100);
        assert_eq!(mood_percent(1.0), 100);
    }


    #[test]
    fn the_f3_wears_the_cave_moods_own_row() {
        // THE BODY-LESS ARM (the vanilla entry's null-player walk —
        // `if player == null return` the line the handler-less client
        // never speaks): the fresh App's left column ends before any
        // mood row the body-less walk could wear
        let mut app = App::new(3);
        app.enter_world();
        let bare = app.debug_lines();
        assert!(
            !bare.iter().any(|l| l.contains("Mood ")),
            "{bare:?}"
        );

        // THE DRIVE (the complete-darkness walk the sound.rs fire walk
        // keeps — every sample banks +1/6000, the f32 accumulation the
        // honest walk rides): 3000 banks a half — the moodiness the
        // accessor answers, the row the same value's fold wears. The
        // two-field drive the disjoint borrows answer (the machine and
        // the engine the same tick walks, the same split the world
        // tick's own call site keeps).
        app.spawn_offline_body();
        let dark = |_x: i32, _y: i32, _z: i32| -> (u8, u8) { (0u8, 0u8) };
        for i in 0..3000u64 {
            app.cave_mood
                .tick(i, &dark, 10.3, 20.7, 30.9, 0.0, &mut app.sounds);
        }
        let moodiness = app.cave_mood.moodiness();
        // the bank's own bound: 3000/6000 the half the f32 walk keeps
        // (the drift the tiny relative error wears — the fire's own
        // 6000-bank exactness the same accumulator answers)
        assert!(
            moodiness > 0.49 && moodiness < 0.51,
            "the 3000-bank half the walk keeps: {moodiness}"
        );
        let lines = app.debug_lines();
        // THE PLACEMENT (the vanilla entry's own neighborhood — the
        // light row the ride follows, the LocalDifficulty row the
        // 26.2 registration order wears between Light and DayCount, the
        // Day row the next index keeps, the mood row the one after
        // answers)
        assert!(
            lines[7].starts_with("Client Light: "),
            "{}",
            lines[7]
        );
        // THE LOCAL DIFFICULTY ROW (the entry between Light and Day
        // the registration order banks — the fresh world's dawn the
        // 1.50 base effective wears: the day ramp the negative clamp
        // zeroes, the moon term the zeroed ramp's own ceiling keeps,
        // the id-2 the 0.75 base multiplies; the special the below-2
        // fcmpg arm keeps at 0.00 — the fresh world's inhabited bank
        // the zero the same word wears)
        assert_eq!(lines[8], "Local Difficulty: 1.50 // 0.00", "{}", lines[8]);
        // THE DAY ROW (the DebugEntryDayCount walk — the fresh world's
        // dawn clock the zero day speaks)
        assert_eq!(lines[9], "Day #0", "{}", lines[9]);
        // THE SPAWN COUNTS ROW (the entry between DayCount and
        // SoundMood the registration order banks — the fresh world's
        // census the zero counts wears: the 5x5 grid the 25 spawnable
        // chunks answers (every chunk the chessboard-8 radius keeps),
        // no mobs the categories the default-0 getInt wears)
        assert_eq!(
            lines[10],
            "SC: 25, MO: 0, C: 0, AM: 0, AX: 0, UWC: 0, WC: 0, WA: 0, MI: 0",
            "{}",
            lines[10]
        );
        // THE ROW (the vanilla concat's mood half verbatim — the
        // leading space the ldc keeps, the percent the fold answers):
        // the exact word the half-machine speaks
        assert_eq!(lines[11], format!(" (Mood {}%)", mood_percent(moodiness)));
        assert_eq!(lines[11], " (Mood 50%)");
        // the row the SoundCache row's LEFT ride keeps (the 26.2
        // registration order the mood's right neighbor the tail banks —
        // the last-line word the cache row's own test pins)
        assert_eq!(
            lines[12],
            "Sound cache: 0 buffers, 0 MiB",
            "{}",
            lines[12]
        );
    }


    #[test]
    fn the_sound_cache_megabytes_wear_the_ceil_law() {
        // THE CEIL'S OWN LAW (javap `DebugEntrySoundCache.
        // bytesToMegabytes` — the `l2d` + two `1024.0` ddivs the
        // `Mth.ceilLong` cast banks): the partial megabyte the NEXT
        // MiB answers, the exact boundary the same word keeps
        assert_eq!(bytes_to_megabytes(0), 0);
        assert_eq!(bytes_to_megabytes(1), 1);
        assert_eq!(bytes_to_megabytes(1_048_575), 1);
        assert_eq!(bytes_to_megabytes(1_048_576), 1);
        assert_eq!(bytes_to_megabytes(1_048_577), 2);
        assert_eq!(bytes_to_megabytes(2_097_152), 2);
        assert_eq!(bytes_to_megabytes(5 * 1_048_576), 5);

        // THE WORD (the display walk's `String.format(Locale.ROOT,
        // "Sound cache: %d buffers, %d MiB", ...)` — the CP's #33
        // utf8 verbatim, the count the first slot, the megabyte ceil
        // the second): the empty cache the 0/0 word wears, the
        // nonzero census the ceil's own answer rides
        assert_eq!(sound_cache_line(0, 0), "Sound cache: 0 buffers, 0 MiB");
        assert_eq!(
            sound_cache_line(3, 3 * 1_048_576 + 5),
            "Sound cache: 3 buffers, 4 MiB"
        );
    }


    #[test]
    fn the_f3_sound_cache_row_closes_the_registration_tail() {
        // THE PLACEMENT (the DebugScreenEntries clinit's own order —
        // SoundMood step 391 the left ride, SoundCache step 406 the
        // right ride the left column's tail closes): the fresh
        // offline world the row the mood's right neighbor wears, the
        // LAST line the tail keeps, the empty cache the 0/0 word the
        // one-shot backend honestly answers
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let lines = app.debug_lines();
        let mood_idx = lines
            .iter()
            .position(|l| l.starts_with(" (Mood "))
            .expect("the mood row the body wears");
        assert_eq!(
            lines[mood_idx + 1],
            "Sound cache: 0 buffers, 0 MiB",
            "{}",
            lines[mood_idx + 1]
        );
        assert_eq!(lines.last().unwrap(), "Sound cache: 0 buffers, 0 MiB");

        // THE NO-GATE LAW (the entry's own bytecode — `isAllowed`
        // return-1, the display the `Minecraft.getInstance()` direct
        // read: the camera/ServerLevel arms the SC/LD rows open this
        // entry never keeps): the NETWORKED world the row still
        // wears — the server-side gates the SC/LD rows the same walk
        // refuses, the cache row the client's own sound manager any
        // world's lane answers, the tail the same word closes
        app.world_is_network = true;
        let lines = app.debug_lines();
        assert!(
            !lines.iter().any(|l| l.starts_with("SC: ")),
            "{lines:?}"
        );
        assert!(
            !lines
                .iter()
                .any(|l| l.starts_with("Local Difficulty: ")),
            "{lines:?}"
        );
        assert_eq!(lines.last().unwrap(), "Sound cache: 0 buffers, 0 MiB");
    }


    #[test]
    fn the_f3_day_row_divides_the_world_clock() {
        // THE DIVISION'S OWN LAW (DebugEntryDayCount's getPeriodCount
        // walk — the getTotalTicks / periodTicks ldiv the bytecode
        // keeps, the period the DAY_PERIOD_TICKS constant mirrors):
        // the base the known-day dawn rides, the frame the same
        // clock's ticks wear
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.frame = 23_000;
        app.clock_total = 3 * crate::timeline::DAY_PERIOD_TICKS as u64 + 500 + 23_000;
        // the total (72_000 + 500 + 23_000) the 95_500 wears — the
        // floor division the 3 days + 23_500 ticks the row speaks (the
        // index the LocalDifficulty row's own insertion the +1 shift
        // wears — the row between Light and Day the vanilla order
        // banks)
        assert_eq!(app.debug_lines()[9], "Day #3");
        // the EXACT period boundary: the total the 4 full days the
        // count 4 wears (the next dawn the day rollover speaks; the
        // clock the frame's step rides — the old base+frame fold the
        // pair moved together)
        app.frame = 24_000;
        app.clock_total = 3 * crate::timeline::DAY_PERIOD_TICKS as u64 + 500 + 24_000;
        assert_eq!(app.debug_lines()[9], "Day #4");
        // the dawn's own zero: the fresh clock the day 0 keeps
        app.clock_total = 0;
        app.frame = 0;
        assert_eq!(app.debug_lines()[9], "Day #0");
    }


    #[test]
    fn the_f3_local_difficulty_row_folds_the_dwell_bank() {
        // THE SERVER-SIDE GATE (the entry's `instanceof ServerLevel`
        // walk — the networked world the vanilla multiplayer F3 keeps
        // silent, the row the offline/host world alone wears): the
        // networked world's rows never speak even with a body present
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.update();
        // THE DRIVE (the host world's own accumulator — the
        // tick_inhabited walk the world-tick lane rides): the first
        // update the body's own chunk the +1 banks (the fresh world
        // the zero bank wore before)
        let banked = app
            .offline_world
            .as_ref()
            .and_then(|w| w.inhabited.as_ref())
            .expect("the host world's own lane");
        assert!(
            banked.iter().any(|t| *t >= 1),
            "the world tick the dwell bank drives: {banked:?}"
        );

        // THE ROW (the entry's own walk — the clock the dawn keeps,
        // the day ramp the negative clamp zeroes, the moon term the
        // zeroed ramp's ceiling keeps, the inhabited bank the ramp's
        // own 2.08e-7-per-tick wear (the 1/3600000 × 0.75 the NORMAL
        // arm keeps): the id-2 the 1.5000004+ effective speaks — the
        // %.2f HALF_UP the `1.50` word keeps; the special the below-2
        // fcmpg arm the 0.00 keeps)
        let lines = app.debug_lines();
        let row = lines
            .iter()
            .find(|l| l.starts_with("Local Difficulty: "))
            .expect("the host world's own row");
        assert_eq!(row, "Local Difficulty: 1.50 // 0.00", "{}", row);

        // THE PLACEMENT (the vanilla registration order — the row
        // between the Client Light and the Day rows the 26.2 census
        // banks)
        let idx = lines.iter().position(|l| l.starts_with("Local Difficulty: ")).unwrap();
        assert!(lines[idx - 1].starts_with("Client Light: "), "{}", lines[idx - 1]);
        assert!(lines[idx + 1].starts_with("Day #"), "{}", lines[idx + 1]);

        // THE DAWN'S OWN TAIL (the 72000+1440000 total the full day
        // ramp wears — the base 0.75+0.25 = 1.0 the id-2 multiplies;
        // the total the moon period's phase-7 span rides (168000 the
        // 24000×7 answer) — the 0.75-brightness moon the
        // clamp(0.1875, 0, 0.25) term speaks, the id-2 the 2.375+
        // effective wears (the banked tick the tiny ramp adds): the
        // %.2f HALF_UP the `2.38` word keeps; the special the
        // (eff-2)/2 the `0.19` half-step answers)
        app.clock_total = 72000 + 1_440_000;
        app.frame = 0;
        let lines = app.debug_lines();
        let idx = lines.iter().position(|l| l.starts_with("Local Difficulty: ")).unwrap();
        // the banked tick the same walk keeps — the inhabited term the
        // ramp's own span rides (the moon-0.75 span the 2.38 word
        // wears — the word the exact f32 fold the half-up round banks)
        assert!(
            lines[idx].starts_with("Local Difficulty: 2.38"),
            "the full-ramp word the row wears: {}",
            lines[idx]
        );
        // the special multiplier the half-step speaks (the (eff-2)/2
        // the 0.19 word keeps — the %.2f the fold answers)
        assert!(
            lines[idx].ends_with("// 0.19"),
            "the half-step the special wears: {}",
            lines[idx]
        );
    }


    #[test]
    fn the_f3_spawn_counts_row_censuses_the_mobs() {
        // THE CENSUS'S OWN WALK (NaturalSpawner.createState the Mob
        // instances alone counts — the kind's own category the
        // EntityTypes clinit pair banks, the joining the values() order
        // wears): the fresh app the zero census keeps (the exact word
        // the mood walk's own placement already pins), the pushed mobs
        // the per-category counts arm
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let (bx, by, bz) = {
            let b = app.body.as_ref().expect("the spawned body");
            (b.x, b.y, b.z)
        };
        // THREE MONSTERS (the EntityTypes clinit's MONSTER arm — one
        // zombie the sim's own ctor spawns, one skeleton and one spider
        // the kind re-arm speaks) + ONE CREATURE (the sheep the
        // CREATURE arm keeps)
        let mut skeleton = crate::mob::Zombie::new(101, bx + 1.0, by, bz, 5);
        skeleton.kind = crate::mob::MobKind::Skeleton;
        let mut spider = crate::mob::Zombie::new(102, bx - 1.0, by, bz, 6);
        spider.kind = crate::mob::MobKind::Spider;
        let mut sheep = crate::mob::Zombie::new(103, bx + 2.0, by, bz, 7);
        sheep.kind = crate::mob::MobKind::Sheep;
        app.mobs.push(crate::mob::Zombie::new(100, bx, by, bz, 4));
        app.mobs.push(skeleton);
        app.mobs.push(spider);
        app.mobs.push(sheep);

        let lines = app.debug_lines();
        let idx = lines
            .iter()
            .position(|l| l.starts_with("SC: "))
            .expect("the host world's own row");
        // THE WORD (the census the kinds speak: MO 3 — the zombie, the
        // skeleton, the spider — C 1 — the sheep — every other category
        // the default-0 keeps; the spawnable count the 25 the whole
        // 5x5 grid the chessboard-8 radius of the centered body wears)
        assert_eq!(
            lines[idx],
            "SC: 25, MO: 3, C: 1, AM: 0, AX: 0, UWC: 0, WC: 0, WA: 0, MI: 0",
            "{}",
            lines[idx]
        );
        // THE PLACEMENT (the registration order — the row the Day
        // row's right ride keeps)
        assert!(lines[idx - 1].starts_with("Day #"), "{}", lines[idx - 1]);

        // THE NON-MOB SKIP (the createState walk the items/orbs/
        // projectiles never counts — the client's own drops the
        // `instanceof Mob` check refuses; the drops lane the census
        // never reads)
        let before = lines[idx].clone();
        app.drops
            .push(crate::drops::ItemDrop::spawn_entity_loot(
                901,
                crate::world::STONE as i32,
                1,
                bx,
                by + 1.0,
                bz,
                &mut app.mob_rng,
            ));
        let lines = app.debug_lines();
        let idx = lines.iter().position(|l| l.starts_with("SC: ")).unwrap();
        assert_eq!(lines[idx], before, "the drop the census never counts");
    }


    #[test]
    fn the_f3_right_column_the_system_rows_speak() {
        // THE TOGGLE (the same F3 flip the left column's test keeps —
        // the right column rides the same panel's own visibility)
        let mut app = App::new(3);
        app.enter_world();
        assert!(!app.debug_held);
        app.on_debug_key();
        assert!(app.debug_held);

        // THE MEMORY ROWS' OWN FORMATS (DebugEntryMemory's ldc pair —
        // the used/allocated/max triple the integer percent fold and
        // the bytesToMebibytes' two ldiv-by-1024 speak: 224 MiB used,
        // 500 MiB allocated, 1638 MiB max → `Mem: 13% 224/1638MiB`
        // (224*100/1638 = 13) and `Allocated: 30% 500MiB`
        // (500*100/1638 = 30); the %2d min-width and the %03d
        // zero-pad the small-percent word wears — 5% the ` 5` space
        // pad keeps)
        let (mem, alloc) = mem_rows(
            224 * 1048576,
            500 * 1048576,
            1638 * 1048576,
        );
        assert_eq!(mem, "Mem: 13% 224/1638MiB");
        assert_eq!(alloc, "Allocated: 30% 500MiB");
        let (small_mem, _) = mem_rows(5 * 1048576, 0, 100 * 1048576);
        assert_eq!(small_mem, "Mem:  5% 005/100MiB");

        // THE PROBE'S OWN HONESTY (the /proc lanes the Linux process
        // really keeps — the Some verdict, the max > 0, the used <=
        // max, and the allocated >= used bound the VmSize/VmRSS pair
        // the kernel keeps)
        let probe = system_memory_probe();
        assert!(probe.is_some(), "the /proc lanes the test env keeps");
        let (used, allocated, max) = probe.unwrap();
        assert!(max > 0, "the max the cgroup/meminfo lane keeps");
        assert!(used <= max, "the resident the limit's own bound keeps");
        assert!(allocated >= used, "the address space the resident bounds");

        // THE SEAM'S OWN DETERMINISM (the drawn rows the caller's OWN
        // probe reading re-speak — the SAME triple the mem_rows fold
        // and the rows pin: the double-read race the VmRSS drift
        // between two /proc walks banks RETIRES — the live wrapper
        // the one probe per frame the draw path wears, the exact
        // numbers the seam's same-reading law pins whole)
        let right = debug_right_lines_from(
            Some((used, allocated, max)),
            cpu_word(8, "Test(R)   CPU(R)"),
            854,
            480,
        );
        let (want_mem, want_alloc) = mem_rows(used, allocated, max);
        assert_eq!(right[0], want_mem, "{}", right[0]);
        assert_eq!(right[1], want_alloc, "{}", right[1]);
        assert_eq!(
            right[2], "CPU: 8x Test(R) CPU(R)",
            "the collapse the whitespace fold wears"
        );
        assert_eq!(right[3], "Display: 854x480 (software)");

        // THE SEAM'S NONE ARM (the no-proc lane the memory rows skip
        // — the CPU row the vec's own head wears, the display row the
        // tail keeps)
        let none_right = debug_right_lines_from(
            None,
            "<unknown>".to_string(),
            854,
            480,
        );
        assert_eq!(
            none_right,
            vec!["CPU: <unknown>", "Display: 854x480 (software)"]
        );

        // THE LIVE WRAPPER'S OWN HONESTY (the drawn rows the ONE
        // probe the frame wears — the row count the Some verdict
        // keeps, the prefixes the live sources speak, the exact
        // numbers the seam's same-reading law pinned above; the
        // /proc/cpuinfo word the env really keeps)
        let live = debug_right_lines(854, 480);
        assert_eq!(live.len(), 4, "{live:?}");
        assert!(live[0].starts_with("Mem: "), "{}", live[0]);
        assert!(live[1].starts_with("Allocated: "), "{}", live[1]);
        assert!(live[2].starts_with("CPU: "), "{}", live[2]);
        assert_ne!(
            live[2], "CPU: <unknown>",
            "the /proc/cpuinfo the test env really keeps"
        );
        assert_eq!(live[3], "Display: 854x480 (software)");

        // THE CPU WORD'S OWN FOLD (GLX._getCpuInfo's law — the
        // `%dx %s` count + name pair and the `.replaceAll('\s+', ' ')`
        // collapse: the multi-space model the single-space word wears)
        assert_eq!(
            cpu_word(2, "Intel(R)   Xeon(R)  Processor"),
            "2x Intel(R) Xeon(R) Processor"
        );

        // THE RIGHT X ANCHOR (extractLines' right branch —
        // `guiWidth - 2 - lineWidth` the else lane keeps: 854-2-100
        // = 752, the narrow gui the same law answers)
        assert_eq!(right_column_x(854, 100), 752);
        assert_eq!(right_column_x(320, 300), 18);
    }


    /// THE F3+G CHUNK BORDER LINES' OWN LAW (the audited 26.2
    /// ChunkBorderRenderer.emitGizmos walk the pure builder speaks —
    /// the seven layers the counts, the endpoints and the ARGB
    /// constants pin, the SectionPos fold the negative-coordinate
    /// floor law keeps).
    #[test]
    fn the_chunk_border_lines_the_seven_layers_speak() {
        // THE CAMERA SECTION (the >> 4 fold): block (5, 20, 7) rides
        // section (0, 1, 0) — the min corner (0, 16, 0)
        let lines = chunk_border_lines((5, 20, 7), 0, 48);
        // THE COUNT (16 neighbors + 7*4 interior verticals + 25*4
        // rings + 4 corners + 12 cuboid edges + 4*4 section rings)
        assert_eq!(lines.len(), 16 + 28 + 100 + 4 + 12 + 16, "the layers");
        // LAYER 1 (the neighbor grid, red alpha 0.5): the first line
        // the (-16, -16) corner speaks, the last the (+32, +32) one
        assert_eq!(
            (lines[0].a.x, lines[0].a.y, lines[0].a.z),
            (-16.0, 0.0, -16.0)
        );
        assert_eq!(lines[0].b.y, 48.0, "the vertical runs minY..maxY");
        assert_eq!(lines[0].rgb, CHUNK_NEIGHBOR_RED);
        assert!((lines[0].alpha - 0.5).abs() < 1e-6);
        assert!(!lines[0].always_on_top);
        let l15 = &lines[15];
        assert_eq!((l15.a.x, l15.a.z), (32.0, 32.0), "the grid's own tail");
        // LAYER 2 (the interior verticals): x = 2 the yellow speaks,
        // x = 4 the teal every-4th wears, both the minZ and maxZ
        // faces ride; the Z walk the mirrored pair keeps
        let x2_min_z = &lines[16]; // i = 2, the minZ face
        assert_eq!((x2_min_z.a.x, x2_min_z.a.z), (2.0, 0.0));
        assert_eq!(x2_min_z.rgb, CHUNK_CELL_YELLOW);
        let x4_min_z = &lines[20]; // i = 4 (the second walk), the minZ face
        assert_eq!((x4_min_z.a.x, x4_min_z.a.z), (4.0, 0.0));
        assert_eq!(x4_min_z.rgb, CHUNK_CELL_TEAL, "the every-4th teal");
        let x2_max_z = &lines[17];
        assert_eq!((x2_max_z.a.x, x2_max_z.a.z), (2.0, 16.0), "the maxZ face");
        let z2_min_x = &lines[18]; // i = 2, the Z walk's own minX face
        assert_eq!((z2_min_x.a.x, z2_min_x.a.z), (0.0, 2.0));
        let z2_max_x = &lines[19];
        assert_eq!((z2_max_x.a.x, z2_max_x.a.z), (16.0, 2.0), "the maxX face");
        // LAYER 3 (the horizontal rings): y = 0 the teal the %8 law
        // speaks, y = 2 the yellow, the step-2 walk the 25 rings keeps
        let ring_y0 = &lines[44]; // 16 + 28 = 44 — the first ring
        assert_eq!(ring_y0.a.y, 0.0);
        assert_eq!(ring_y0.rgb, CHUNK_CELL_TEAL);
        let ring_y2 = &lines[48];
        assert_eq!(ring_y2.a.y, 2.0);
        assert_eq!(ring_y2.rgb, CHUNK_CELL_YELLOW);
        let ring_y8 = &lines[60];
        assert_eq!(ring_y8.a.y, 8.0);
        assert_eq!(ring_y8.rgb, CHUNK_CELL_TEAL);
        // LAYER 4 (the corner verticals, the major blue): the four
        // {0,16}² corners the camera chunk's own edges wear
        let corner0 = &lines[144]; // 16 + 28 + 100 = 144
        assert_eq!((corner0.a.x, corner0.a.z), (0.0, 0.0));
        assert_eq!(corner0.rgb, CHUNK_MAJOR_BLUE);
        let corner3 = &lines[147];
        assert_eq!((corner3.a.x, corner3.a.z), (16.0, 16.0));
        // LAYER 5 (the section cuboid, always-on-top): the camera's
        // own 16³ box the min corner (0, 16, 0) keeps
        let cuboid = &lines[148..160];
        assert!(
            cuboid.iter().all(|l| l.always_on_top),
            "the setAlwaysOnTop stroke"
        );
        assert!(cuboid.iter().all(|l| l.rgb == CHUNK_MAJOR_BLUE));
        let secs: Vec<(f32, f32, f32)> = cuboid.iter().map(|l| (l.a.x, l.a.y, l.a.z)).collect();
        assert!(secs.contains(&(0.0, 16.0, 0.0)), "the section min corner");
        assert!(
            cuboid.iter().any(|l| (l.b.x, l.b.y, l.b.z) == (16.0, 32.0, 16.0)),
            "the section max corner the +16 box wears"
        );
        // LAYER 6 (the section-y rings): y = 0, 16, 32, 48 the four
        // blue rings the every-16th walk keeps
        let rings = &lines[160..176];
        let ys: Vec<f32> = rings.iter().step_by(4).map(|l| l.a.y).collect();
        assert_eq!(ys, vec![0.0, 16.0, 32.0, 48.0]);
        assert!(rings.iter().all(|l| l.rgb == CHUNK_MAJOR_BLUE));
        assert!(rings.iter().all(|l| !l.always_on_top));

        // THE NEGATIVE SECTION (the >> 4 arithmetic shift the floor
        // law keeps): block (-3, 20, -3) rides section (-1, 1, -1) —
        // the min corner (-16, 16, -16)
        let neg = chunk_border_lines((-3, 20, -3), 0, 48);
        assert_eq!((neg[0].a.x, neg[0].a.z), (-32.0, -32.0), "the -16 offset");
        let neg_cuboid = &neg[148..160];
        assert!(
            neg_cuboid
                .iter()
                .any(|l| (l.a.x, l.a.y, l.a.z) == (-16.0, 16.0, -16.0)),
            "the negative section's own min corner"
        );
    }


    /// THE F3+G TOGGLE'S OWN LAW (the debug-entry flip the audited
    /// refreshRendererList gate serves — the persistence the entry
    /// system keeps: no overlay raise, the modifier-spent flag the
    /// bare F3 release gate reads).
    #[test]
    fn the_f3_g_chord_toggles_the_chunk_border_entry() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(!app.chunk_borders, "the fresh entry sleeps");
        app.on_chunk_borders_key();
        assert!(app.chunk_borders, "the chord raises");
        assert!(app.debug_modifier_used, "the F3 modifier spent");
        assert!(!app.debug_held, "the overlay the entry never raises");
        app.on_chunk_borders_key();
        assert!(!app.chunk_borders, "the chord lowers");
    }


    #[test]
    fn the_gamemode_switcher_layout_the_audited_geometry_speaks() {
        // THE GEOMETRY'S OWN LAW (javap 26.2 GameModeSwitcherScreen —
        // the extractBackground's isub-62/isub-31-then-27 chain, the
        // init walk's ALL_SLOTS_WIDTH/2 halving + the imul-31 row, the
        // render walk's isub-31-then-20 name line and the iadd-5 hint)
        let l = gamemode_switcher_layout(854, 480);
        assert_eq!(l.panel, (854 / 2 - 62, 480 / 2 - 31 - 27), "the 125x75 blit");
        assert_eq!(l.panel.0, 365, "the panel x the fold speaks");
        assert_eq!(l.panel.1, 182, "the panel y the fold speaks");
        // the slot row: x = width/2 - 59 + i*31 (the 119 halving), the
        // y the height/2 - 31 row keeps
        let xs: Vec<i32> = l.slots.iter().map(|&(x, _)| x).collect();
        assert_eq!(xs, vec![368, 399, 430, 461], "the step-31 row");
        assert!(l.slots.iter().all(|&(_, y)| y == 209), "the slot y");
        assert_eq!(l.name_y, 189, "the name line");
        assert_eq!(l.hint_y, 245, "the hint line");

        // THE ODD-WIDTH CENTER'S OWN FLOOR (the integer division the
        // negative side the vanilla's own math keeps — 853/2 = 426)
        let l = gamemode_switcher_layout(853, 481);
        assert_eq!(l.panel.0, 426 - 62, "the odd width's floor");
        assert_eq!(l.slots[0].0, 426 - 59, "the row's odd-width x");
        assert_eq!(l.name_y, 240 - 51, "the odd height's floor");

        // THE ICON ORDER'S OWN LAW (the audited $values array — the
        // ordinals the getNext walk cycles, the slot row's left-to-right)
        let order = GameMode::ICON_ORDER;
        assert_eq!(
            order,
            [
                GameMode::Creative,
                GameMode::Survival,
                GameMode::Adventure,
                GameMode::Spectator,
            ],
            "the audited enum order"
        );
        // the cycle: creative -> survival -> adventure -> spectator ->
        // creative (the wrap the mod-4 ordinal law keeps)
        let nexts: Vec<GameMode> = order.iter().map(|&m| m.next_icon()).collect();
        assert_eq!(nexts, [order[1], order[2], order[3], order[0]], "the getNext law");
        // the name keys and the icon items (the GameModeIcon fields —
        // the translatable keys the en_us table answers, the ItemStack
        // ctor's own four the registry resolves)
        assert_eq!(GameMode::Creative.name_key(), "gameMode.creative");
        assert_eq!(GameMode::Survival.name_key(), "gameMode.survival");
        assert_eq!(GameMode::Adventure.name_key(), "gameMode.adventure");
        assert_eq!(GameMode::Spectator.name_key(), "gameMode.spectator");
        assert_eq!(GameMode::Creative.icon_item(), "grass_block");
        assert_eq!(GameMode::Survival.icon_item(), "iron_sword");
        assert_eq!(GameMode::Adventure.icon_item(), "map");
        assert_eq!(GameMode::Spectator.icon_item(), "ender_eye");
        for mode in order {
            assert!(
                mc_registry::item_id_by_name(mode.icon_item()).is_some(),
                "the icon id the registry carries: {}",
                mode.icon_item()
            );
        }
        // the survival default (the vanilla world creation's own law)
        assert_eq!(GameMode::Survival.icon_index(), 1, "the survival slot");
    }


    #[test]
    fn the_f4_switcher_lanes_the_audited_hold_law_speaks() {
        // THE OPEN GATE'S OWN LAW (the handleDebugKeys chain — the
        // in-world, alive, unpaused world the walk never refuses)
        let mut app = App::new(3);
        app.enter_world();
        assert_eq!(app.game_mode, GameMode::Survival, "the vanilla default");
        assert!(app.previous_game_mode.is_none(), "the fresh bank sleeps");
        app.on_open_game_mode_switcher();
        assert_eq!(app.screen, ScreenKind::GameModeSwitcher, "the screen opens");
        assert_eq!(app.gm_hovered, 1, "the default hover the current mode speaks");
        assert!(app.debug_modifier_used, "the chord consumes the modifier");
        // THE CYCLE WALK (the keyPressed law — the getNext the F4 tap
        // serves, the wrap the mod-4 ordinal keeps)
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 2, "adventure");
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 3, "spectator");
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 0, "creative");
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 1, "the wrap lands survival");
        // THE F3 RELEASE'S OWN APPLY (the keyReleased law — the apply
        // the change earns, the previous bank the old mode fills, the
        // close the same release keeps)
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 2, "adventure hovered");
        app.on_debug_modifier_released();
        assert_eq!(app.screen, ScreenKind::World, "the release closes");
        assert_eq!(app.game_mode, GameMode::Adventure, "the apply lands");
        assert_eq!(
            app.previous_game_mode,
            Some(GameMode::Survival),
            "the bank the old mode fills"
        );
        // THE REOPEN'S OWN DEFAULT (the getDefaultSelected law — the
        // PREVIOUS mode the bank prefers: the flick the two-mode toggle
        // feel speaks)
        app.on_open_game_mode_switcher();
        assert_eq!(app.gm_hovered, 1, "the previous mode the reopen hovers");
        app.on_debug_modifier_released();
        assert_eq!(app.game_mode, GameMode::Survival, "the flick returns");
        assert_eq!(
            app.previous_game_mode,
            Some(GameMode::Adventure),
            "the bank the return fills"
        );
        // THE SAME-MODE ARM (the `icon.mode != current` gate — the
        // silent apply the bank never moves)
        app.on_open_game_mode_switcher(); // hovers the previous
        assert_eq!(app.gm_hovered, 2, "the adventure the reopen hovers");
        // cycle thrice to land BACK on survival
        app.on_game_mode_switcher_cycle();
        app.on_game_mode_switcher_cycle();
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 1, "survival hovered again");
        app.on_debug_modifier_released();
        assert_eq!(app.screen, ScreenKind::World, "the same-mode closes");
        assert_eq!(app.game_mode, GameMode::Survival, "the mode never moves");
        assert_eq!(
            app.previous_game_mode,
            Some(GameMode::Adventure),
            "the bank the same-mode arm keeps"
        );
        // THE GATES' OWN REFUSALS (the paused world the walk never
        // passes — the screen the refusal keeps)
        app.paused = true;
        app.on_open_game_mode_switcher();
        assert_eq!(app.screen, ScreenKind::World, "the pause refuses the open");
        app.paused = false;
        app.dead = true;
        app.on_open_game_mode_switcher();
        assert_eq!(app.screen, ScreenKind::World, "the death refuses the open");
        app.dead = false;
        // THE CLOSED-STATE SILENCE (the screen's own key walk — the
        // cycle and the apply the non-switcher screens never reach)
        app.gm_hovered = 0;
        app.on_game_mode_switcher_cycle();
        assert_eq!(app.gm_hovered, 0, "the closed state the cycle never moves");
        app.on_debug_modifier_released();
        assert_eq!(app.screen, ScreenKind::World, "the closed release walks the toggle");
    }


    #[test]
    fn the_f4_switcher_the_wire_session_the_server_belief_keeps() {
        // THE WIRE'S OWN SEND SHAPE (the mode the SERVER's belief owns
        // — the ServerboundChangeGameModePacket the wire apply speaks,
        // the sink-less App the send's own silence keeps: the packet
        // lane the encoder-less refusal retired — and the LOCAL
        // prediction the switcher's walk never keeps regardless)
        let mut app = App::new(3);
        app.enter_world();
        app.world_is_network = true;
        app.on_open_game_mode_switcher();
        assert_eq!(app.screen, ScreenKind::GameModeSwitcher, "the open the wire keeps");
        app.on_game_mode_switcher_cycle();
        app.on_game_mode_switcher_cycle();
        app.on_debug_modifier_released();
        assert_eq!(app.screen, ScreenKind::World, "the release closes");
        assert_eq!(app.game_mode, GameMode::Survival, "the mode the server owns");
        assert!(app.previous_game_mode.is_none(), "the bank the wire never fills");
    }


    /// THE GLIDE START'S OWN WIRE WORD (javap LocalPlayer.aiStep
    /// pcs 644-694 the 26.2 walk — the jump chord's success the
    /// ServerboundPlayerCommandPacket rides, the live session the
    /// word speaks, the offline lane the wire never spends): the
    /// elytra chest the verdict grants, the airborne body the start
    /// speaks, the drain the command banks and clears.
    #[test]
    fn the_glide_start_the_wire_word_rides() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // THE LIVE LANE (the net world the body walks, the sink the
        // word rides — the same harness the F4 toggle's wire test
        // wears): the chest MENU slot 6 the elytra stack wears (the
        // ctor's SLOT_IDS walk: 5 head, 6 chest, 7 legs, 8 feet), the
        // airborne body the jump key holds
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
        app.attach_live_clicks(tx);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        // THE START + THE WORD (the flag the verdict speaks, the
        // drain the command banks and clears, the 26.2 ordinal 6 the
        // Action enum's clinit banks)
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the glide chord the start speaks"
        );
        assert!(
            !app.body.as_ref().expect("the body rides").glide_started_due,
            "the echo the drain clears"
        );
        // the input packet the jump key's sync lane rides FIRST — the
        // command word the drain speaks after it (the try-recv walk
        // the input rows skip)
        let mut saw_word = None;
        while let Ok(cmd) = rx.try_recv() {
            if let crate::world_view::ContainerCommand::PlayerCommand { action } = cmd {
                saw_word = Some(action);
                break;
            }
        }
        assert_eq!(
            saw_word,
            Some(mc_protocol::play::PLAYER_COMMAND_START_FALL_FLYING),
            "the START_FALL_FLYING ordinal the word carries"
        );
        // THE OFFLINE LANE (the gameplay inventory the chest row
        // wears): the same chord the glide starts and the wire the
        // lane never spends
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
        app.attach_live_clicks(tx);
        app.inventory.armor[1] = (wire("elytra"), 1);
        {
            let b = app.body.as_mut().expect("the offline body spawns");
            b.y = 40.0;
            b.on_ground = false;
        }
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the offline glide the same chord starts"
        );
        // the input sync the jump press still rides (the sink the
        // input lane reads) — the COMMAND word the offline drain
        // never spends
        let mut saw_word = false;
        while let Ok(cmd) = rx.try_recv() {
            if let crate::world_view::ContainerCommand::PlayerCommand { .. } = cmd {
                saw_word = true;
            }
        }
        assert!(
            !saw_word,
            "the offline lane the wire never spends"
        );
    }


    /// THE FALL-FLYING WEAR'S OWN RIDE (javap `updateFallFlying` the
    /// 26.2 walk — the gliding ride the live lane's air world keeps
    /// airborne): the 40-tick glide the clock keeps, the every-20
    /// gates the TWO damage points speak, the break the 2-point wear
    /// never triggers, the survival stack the infinite-materials
    /// gate never shields (the feed walk the creative mode alone
    /// arms).
    #[test]
    fn the_elytra_wear_the_glide_banks_the_damage() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the glide chord the start speaks"
        );
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 0..39 {
            app.update();
        }
        let b = app.body.as_ref().expect("the body rides");
        assert_eq!(b.fall_fly_ticks, 40, "the clock the 40 glide ticks keep");
        assert_eq!(b.elytra_damage, 2, "the every-20 wear the 40 ticks speak");
        assert!(!b.elytra_broke_due, "the 2-point wear the break never speaks");
        assert!(b.fall_flying, "the air world the glide never retires");
    }


    /// THE SCULK LANE'S OWN RECORD (javap Entity.gameEvent →
    /// Level.gameEvent → ServerLevel.gameEvent →
    /// GameEventDispatcher.post the 26.2 walk): the every-10 gate
    /// the world's hearing speaks — the record the lane banks, the
    /// PRE-travel position the Entity's own `getfield position`
    /// walk carries (the wear walk the tick's head runs, the move
    /// the position never touched yet), the r16 the register(name)'s
    /// own default-radius walk banks, the f4 the VibrationSystem's
    /// frequency map answers.
    #[test]
    fn the_sculk_lane_the_glide_event_the_world_hears() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the glide chord the start speaks"
        );
        app.on_movement_input(0.0, 0.0, false, false, false);
        // ticks 2..=9: the gate silent, the lane empty
        for _ in 2..=9 {
            app.update();
        }
        assert!(
            app.game_events.is_empty(),
            "the 9 count the gate keeps silent"
        );
        // the event's own position: the PRE-travel field of tick 10 =
        // the post-tick-9 answer the body banks
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        app.update(); // tick 10: the first gate
        assert_eq!(
            app.game_events.len(),
            1,
            "the 10th tick the gate speaks"
        );
        assert_eq!(
            app.game_events[0],
            format!(
                "elytra_glide @10 x{:.6} y{:.6} z{:.6} r16 f4",
                pos9.0, pos9.1, pos9.2
            ),
            "the record the pre-travel position + the register's r16 + the vibration's f4 banks"
        );
        // ticks 11..=19: the rhythm's silence again
        for _ in 11..=19 {
            app.update();
        }
        let pos19 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        app.update(); // tick 20: the second gate (the HURT parity's own —
                      // the (i/10)%2 walk the hurtAndBreak rides, the
                      // record the gate NEVER reads)
        assert_eq!(
            app.game_events[1],
            format!(
                "elytra_glide @20 x{:.6} y{:.6} z{:.6} r16 f4",
                pos19.0, pos19.1, pos19.2
            ),
            "the hurt tick the record ALIKE speaks — the parity the gate never reads"
        );
        // ticks 21..=40: the every-10 rhythm rides (i=30 the hurtless,
        // i=40 the hurt pair — the count the rhythm's own answer)
        for _ in 21..=40 {
            app.update();
        }
        assert_eq!(
            app.game_events.len(),
            4,
            "the every-10 rhythm the 40 ticks bank"
        );
        assert!(
            app.game_events[2].starts_with("elytra_glide @30 "),
            "the 30th stamp the rhythm banks"
        );
        assert!(
            app.game_events[3].starts_with("elytra_glide @40 "),
            "the 40th stamp the rhythm banks"
        );
        // tick 41: the drain the lane keeps — the no-gate tick the
        // record never speaks
        app.update();
        assert_eq!(
            app.game_events.len(),
            4,
            "the 41st tick the gate refuses"
        );
    }


    /// THE SCULK SENSOR'S OWN HEARING (the dispatch walk the App
    /// serves): the glide event the registered sensor cells answer —
    /// the hearing record the lane banks (the exact line the
    /// reference folds pin), the out-of-range sensor the gate
    /// refuses, the phase window the re-arm law keeps (the tick-20
    /// event the ACTIVE/COOLDOWN walk refuses, the tick-50 event the
    /// window's tail still refuses, the post-window tick-60 event the
    /// INACTIVE phase hears again) — the arrival's own broadcast the
    /// activation banks (the record the sweep serves, the lane the
    /// walk's own tail wears).
    #[test]
    fn the_sculk_sensor_the_glide_hears() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        // the near sensor rides the fall path, the far sensor the
        // gate's own radius refuses
        app.place_sculk_sensor(0, 56, 0);
        app.place_sculk_sensor(10, 56, 0);
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        assert!(
            app.body.as_ref().expect("the body rides").fall_flying,
            "the glide chord the start speaks"
        );
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        assert!(app.game_events.is_empty());
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        // the reference folds the expected hearing's own fields pin —
        // the distance the Vec3→CENTER fold, the arrival the floor,
        // the strength the integer-sqrt + the fold pair
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 56.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let eb = (
            pos9.0.floor() as i32,
            pos9.1.floor() as i32,
            pos9.2.floor() as i32,
        );
        let dsq = ((eb.0 - 0) * (eb.0 - 0)
            + (eb.1 - 56) * (eb.1 - 56)
            + (eb.2 - 0) * (eb.2 - 0)) as f64;
        let p = std::cmp::max(1, 15 - ((15.0f64 / 8.0) * dsq.sqrt()).floor() as i32);
        app.update(); // tick 10: the first gate — the post + ONE hearing
        assert_eq!(
            app.game_events.len(),
            2,
            "the post + the near sensor's hearing — the far sensor the gate refuses"
        );
        assert!(
            app.game_events[0].starts_with("elytra_glide @10 "),
            "the post the lane banks first"
        );
        assert_eq!(
            app.game_events[1],
            format!(
                "sculk_hearing @10 x0 y56 z0 d{:.6} t{} f4 p{}",
                d, t, p
            ),
            "the hearing the reference folds pin bit-for-bit"
        );
        // the tick-20 event the phase window refuses (the sensor the
        // ACTIVE/COOLDOWN walk keeps — the arrival + 40 the free-at
        // folds). THE ARRIVAL'S OWN BROADCAST the window's middle
        // banks — the activation the record lane wears (the walk the
        // sweep serves), the hearing the tag gate never repeats
        let click_at = 10 + t;
        for _ in 11..click_at {
            app.update();
        }
        assert_eq!(
            app.game_events.len(),
            2,
            "the interim ticks the lane keeps"
        );
        app.update(); // the arrival: the sweep the broadcast banks
        assert_eq!(
            app.game_events.len(),
            3,
            "the arrival the broadcast the lane banks"
        );
        assert_eq!(
            app.game_events[2],
            format!(
                "sculk_sensor_tendrils_clicking @{} x0.500000 y56.500000 z0.500000 r16 f0",
                click_at
            ),
            "the broadcast the walk's own line pins"
        );
        for _ in (click_at + 1)..=19 {
            app.update();
        }
        assert_eq!(
            app.game_events.len(),
            3,
            "the interim tail the lane keeps"
        );
        app.update(); // tick 20
        assert_eq!(
            app.game_events.len(),
            4,
            "the post banks, the hearing the phase refuses, the arrival broadcast the lane keeps"
        );
        assert!(
            app.game_events[3].starts_with("elytra_glide @20 "),
            "the second post the lane banks"
        );
        // the tick-50 event the window's tail STILL refuses (the
        // free-at the arrival + 40 the re-arm law keeps) — the body
        // the teleport parks in range first. The tick-30/40 events
        // the walk posts alike (the glide clock the gate never
        // pauses) — the posts the lane banks, the hearings the phase
        // refuses
        for _ in 21..=45 {
            app.update();
        }
        assert_eq!(
            app.game_events.len(),
            6,
            "the tick-30/40 posts bank — the hearings the phase refuses"
        );
        {
            let b = app.body.as_mut().expect("the body rides");
            b.x = 0.0;
            b.y = 64.0;
            b.z = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
        }
        for _ in 46..=49 {
            app.update();
        }
        app.update(); // tick 50
        assert_eq!(
            app.game_events.len(),
            7,
            "the tick-50 post banks, the hearing the window still refuses"
        );
        assert!(
            app.game_events[6].starts_with("elytra_glide @50 "),
            "the tick-50 post the lane banks"
        );
        // the post-window event the re-armed sensor hears again (the
        // INACTIVE phase the window's tail restores)
        {
            let b = app.body.as_mut().expect("the body rides");
            b.y = 64.0;
            b.vy = 0.0;
        }
        for _ in 51..=56 {
            app.update();
        }
        // the post-window event the re-armed sensor hears again (the
        // INACTIVE phase the window's tail restores): the teleport
        // parks the body in range — the look-steered drift the glide
        // keeps the z fold resets too
        {
            let b = app.body.as_mut().expect("the body rides");
            b.x = 0.0;
            b.y = 64.0;
            b.z = 0.0;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
        }
        for _ in 57..=59 {
            app.update();
        }
        let pos59 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        let dx2 = pos59.0 - 0.5;
        let dy2 = pos59.1 - 56.5;
        let dz2 = pos59.2 - 0.5;
        let d2 = ((dx2 * dx2 + dy2 * dy2 + dz2 * dz2).sqrt()) as f32;
        let t2 = d2.floor() as i32;
        let eb2 = (
            pos59.0.floor() as i32,
            pos59.1.floor() as i32,
            pos59.2.floor() as i32,
        );
        let dsq2 = ((eb2.0 - 0) * (eb2.0 - 0)
            + (eb2.1 - 56) * (eb2.1 - 56)
            + (eb2.2 - 0) * (eb2.2 - 0)) as f64;
        let p2 = std::cmp::max(1, 15 - ((15.0f64 / 8.0) * dsq2.sqrt()).floor() as i32);
        app.update(); // tick 60
        assert_eq!(
            app.game_events.len(),
            9,
            "the tick-60 post + the SECOND hearing the re-armed sensor banks"
        );
        assert!(
            app.game_events[7].starts_with("elytra_glide @60 "),
            "the tick-60 post the lane banks"
        );
        assert_eq!(
            app.game_events[8],
            format!(
                "sculk_hearing @60 x0 y56 z0 d{:.6} t{} f4 p{}",
                d2, t2, p2
            ),
            "the re-armed hearing the reference folds pin bit-for-bit"
        );
    }


    /// THE SENSOR'S OWN VOICE LANE (javap SculkSensorBlock — the
    /// activation's scheduled window the sweep speaks): the hearing
    /// the schedule arms (the CLICKING the arrival tick answers — the
    /// post tick the voice never wears), the CLICKING the arrival
    /// speaks (the v1.0F / 0.8F + nextFloat() * 0.2F pair the level's
    /// own stream keeps), the CLICKING_STOP the re-arm tail answers
    /// (the free-at's own twin), the cleared Options the double-voice
    /// refusal wears.
    #[test]
    fn the_sculk_voice_the_activation_the_sensors_speak() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 56, 0);
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        // the reference fold the arrival pins — the Vec3→CENTER
        // distance the travel law floors
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 56.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        let stop_at = click_at + crate::sculk::SCULK_SENSOR_REARM_TICKS as i32;
        app.update(); // tick 10: the post + the hearing — the schedule arms
        // THE SCHEDULE the hearing arms, the POST tick the voice
        // never wears (the arrival the vibration's own travel speaks)
        {
            let cell = &app.sculk_sensors[0];
            assert_eq!(
                cell.clicking_at,
                Some(click_at as u64),
                "the arrival tick the schedule banks"
            );
            assert_eq!(
                cell.stop_at,
                Some(stop_at as u64),
                "the re-arm tail the free-at's twin banks"
            );
            assert_eq!(
                cell.free_at,
                stop_at as u64,
                "the free-at the same fold the window keeps"
            );
        }
        assert!(
            !app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("block.sculk_sensor.clicking")),
            "the post tick the voice never wears"
        );
        // the ticks between the post and the arrival: the window holds
        for _ in 11..click_at {
            app.update();
        }
        assert_eq!(
            app.sculk_sensors[0].clicking_at,
            Some(click_at as u64),
            "the pending window the interim ticks keep"
        );
        // THE ARRIVAL TICK: the CLICKING speaks, the window's head drains
        app.update();
        {
            let cell = &app.sculk_sensors[0];
            assert_eq!(
                cell.clicking_at, None,
                "the cleared Option the double-voice refusal wears"
            );
            assert_eq!(
                cell.stop_at,
                Some(stop_at as u64),
                "the stop the window's tail still banks"
            );
        }
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.sculk_sensor.clicking @"))
                .expect("the CLICKING voice the arrival speaks");
            assert!(
                voice.starts_with(&format!("block.sculk_sensor.clicking @{click_at} ")),
                "the arrival tick the stamp keeps: {voice}"
            );
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|tk| tk[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (0.8..1.0).contains(&pitch),
                "the 0.8 + r * 0.2 band the draw keeps: {pitch}"
            );
        }
        // THE RE-ARM TAIL: the CLICKING_STOP speaks, the window drains
        for _ in (click_at + 1)..stop_at {
            app.update();
        }
        assert_eq!(
            app.sculk_sensors[0].stop_at,
            Some(stop_at as u64),
            "the tail the interim ticks keep"
        );
        app.update();
        {
            let cell = &app.sculk_sensors[0];
            assert_eq!(cell.stop_at, None, "the tail the window drains");
            assert_eq!(
                cell.free_at,
                stop_at as u64,
                "the phase the INACTIVE walks home — the cell hears again"
            );
        }
        if sounds_loaded() {
            let stop = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("block.sculk_sensor.clicking_stop @"))
                .expect("the CLICKING_STOP voice the re-arm tail speaks");
            assert!(
                stop.starts_with(&format!("block.sculk_sensor.clicking_stop @{stop_at} ")),
                "the re-arm tail the stamp keeps: {stop}"
            );
            let pitch: f32 = stop
                .split(" p")
                .nth(1)
                .and_then(|tk| tk[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (0.8..1.0).contains(&pitch),
                "the same band the stop's draw keeps: {pitch}"
            );
        }
        // THE DRAINED WINDOW: a further tick the silence keeps (no
        // double voice the cleared Options wear)
        let (clicks, stops) = (
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.sculk_sensor.clicking @"))
                .count(),
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.sculk_sensor.clicking_stop @"))
                .count(),
        );
        app.update();
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.sculk_sensor.clicking @"))
                .count(),
            clicks,
            "the drained window the clicking silence keeps"
        );
        assert_eq!(
            app.sounds
                .events
                .iter()
                .filter(|e| e.starts_with("block.sculk_sensor.clicking_stop @"))
                .count(),
            stops,
            "the drained window the stop silence keeps"
        );
    }


    /// THE SENSOR'S OWN BROADCAST (javap activate pc 64-72 — the
    /// Level.gameEvent(entity, SCULK_SENSOR_TENDRILS_CLICKING, pos) the
    /// activation's own walk posts): the record line the lane banks (the
    /// atCenterOf +0.5 center, the register(name)'s bipush 16 radius, the
    /// f0 the map's defaultReturnValue answers — the tendrils row the 41
    /// pairs never carry), the dispatch's tag gate the second-order
    /// hearing refuses (the vibrations json the 56 rows the event never
    /// carries — the in-range cell the gates pass, the tag alone refuses,
    /// no cascade the lane ever banks), the double-post the cleared
    /// Option refuses, the glide clock the phase window keeps silent.
    #[test]
    fn the_broadcast_the_sensors_never_hear() {
        // the vocabulary the map's own default keeps — the event the
        // frequency table and the vibrations tag alike never carry
        assert_eq!(
            crate::sculk::vibration_frequency("sculk_sensor_tendrils_clicking"),
            0,
            "the map's defaultReturnValue the lookup answers"
        );
        assert!(
            !crate::sculk::is_valid_vibration("sculk_sensor_tendrils_clicking"),
            "the vibrations tag the row never carries"
        );
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
        // the near sensor rides the fall path, the second sensor the
        // broadcast's own radius keeps (the tendrils center the in-range
        // gate passes — the tag alone the hearing refuses; the glide's
        // own events the radius never grants it)
        app.place_sculk_sensor(0, 56, 0);
        app.place_sculk_sensor(2, 50, 2);
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        // the reference fold the arrival pins — the Vec3→CENTER distance
        // the travel law floors
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 56.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        let stop_at = click_at + crate::sculk::SCULK_SENSOR_REARM_TICKS as i32;
        app.update(); // tick 10: the post + ONE hearing
        assert_eq!(
            app.game_events.len(),
            2,
            "the post + the near sensor's hearing — the second sensor the glide's own radius refuses"
        );
        assert!(
            app.game_events[0].starts_with("elytra_glide @10 "),
            "the post the lane banks first"
        );
        assert!(
            app.game_events[1].starts_with("sculk_hearing @10 x0 y56 z0 "),
            "the near sensor's hearing the lane banks"
        );
        // the interim ticks the window keeps — the lane the glide clock
        // never disturbs before the first gate
        for _ in 11..click_at {
            app.update();
        }
        assert_eq!(
            app.game_events.len(),
            2,
            "the interim ticks the lane keeps"
        );
        // THE ARRIVAL: the sweep the activation walks — the broadcast the
        // record lane banks (the gameEvent the playSound precedes)
        app.update();
        assert_eq!(
            app.game_events.len(),
            3,
            "the broadcast the lane banks at the arrival"
        );
        assert_eq!(
            app.game_events[2],
            format!(
                "sculk_sensor_tendrils_clicking @{} x0.500000 y56.500000 z0.500000 r16 f0",
                click_at
            ),
            "the broadcast the walk's own line pins — the center the atCenterOf fold keeps, the r16 the register(name)'s bipush banks, the f0 the map's default speaks"
        );
        // THE DISPATCH'S TAG GATE the second-order hearing refuses — the
        // in-range cell the gates pass, the tag alone the refusal wears
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("sculk_hearing"))
                .count(),
            1,
            "the tick-10 hearing the only one — the broadcast the tag refuses"
        );
        {
            let near = &app.sculk_sensors[0];
            assert_eq!(
                near.clicking_at, None,
                "the cleared Option the double-voice refusal wears"
            );
            assert_eq!(
                near.stop_at,
                Some(stop_at as u64),
                "the stop the window's tail still banks — the broadcast the phase never disturbs"
            );
            assert_eq!(
                near.free_at,
                stop_at as u64,
                "the free-at the same fold keeps"
            );
        }
        {
            let second = &app.sculk_sensors[1];
            assert_eq!(
                second.clicking_at, None,
                "the in-range cell the window never arms — the tag the hearing refuses"
            );
            assert_eq!(
                second.stop_at, None,
                "the tail the refusal keeps empty"
            );
            assert_eq!(
                second.free_at, 0,
                "the phase the refusal never spends"
            );
        }
        // the glide clock the next gate posts — the phase window the
        // hearing refuses, the second-order silence the walk keeps
        for _ in (click_at + 1)..=20 {
            app.update();
        }
        assert!(
            app.game_events[3].starts_with("elytra_glide @20 "),
            "the glide clock the next gate posts"
        );
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("sculk_hearing"))
                .count(),
            1,
            "the phase window the tick-20 hearing refuses — the second-order silence keeps"
        );
        // the i%10 gate the odd tick refuses — the broadcast the cleared
        // Option never doubles
        app.update();
        assert_eq!(
            app.game_events.len(),
            4,
            "the 21st tick the gates refuse — the broadcast the double-post refusal wears"
        );
        assert_eq!(
            app.sculk_sensors[1].clicking_at, None,
            "the whole walk the in-range cell the refusal keeps"
        );
    }


    /// THE WOOL'S OWN REFUSAL (the occlusion walk the hearing gates):
    /// the wool cell the ray path keeps — the hearing the isOccluded
    /// walk refuses (the post the lane banks, the phase the sensor
    /// never arms), the moved wool the ray path leaves — the tick-20
    /// event the re-opened path hears (the exact hearing line the
    /// reference folds pin), the arrival the broadcast the re-opened
    /// activation banks.
    #[test]
    fn the_wool_the_hearing_refuses() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let wool_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:white_wool")
            .map(|(s, _, _)| *s as u16)
            .expect("white_wool rides the registry");
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 32, 0);
        // THE WOOL COLUMN the ray path keeps — the cell between the
        // event's block and the sensor's own (the chunk (0,0) the
        // chunk-major index serves; the world the 48-high grid keeps,
        // the scene the 32-40 band rides)
        app.net_world.as_mut().expect("the world rides").chunks[0]
            [36 * crate::world::CHUNK_X * crate::world::CHUNK_Z] = wool_id;
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        app.update(); // tick 10: the post banks, the hearing the wool refuses
        assert_eq!(
            app.game_events.len(),
            1,
            "the post the lane banks — the hearing the wool refuses"
        );
        assert!(
            app.game_events[0].starts_with("elytra_glide @10 "),
            "the post the lane banks"
        );
        {
            let cell = &app.sculk_sensors[0];
            assert_eq!(
                cell.clicking_at, None,
                "the phase the wool's refusal never arms"
            );
            assert_eq!(
                cell.free_at, 0,
                "the window the refusal never spends"
            );
        }
        // THE WOOL MOVES — the ray path re-opens (the off-path cell the
        // selectivity keeps: (5,60,5) the column walk never enters)
        {
            let w = app.net_world.as_mut().expect("the world rides");
            w.chunks[0][36 * crate::world::CHUNK_X * crate::world::CHUNK_Z] =
                crate::world::AIR;
            w.chunks[0][5
                + 5 * crate::world::CHUNK_X
                + 36 * crate::world::CHUNK_X * crate::world::CHUNK_Z] = wool_id;
        }
        for _ in 11..=19 {
            app.update();
        }
        let pos19 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        // the reference folds the tick-20 hearing's own fields pin
        let dx = pos19.0 - 0.5;
        let dy = pos19.1 - 32.5;
        let dz = pos19.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let eb = (
            pos19.0.floor() as i32,
            pos19.1.floor() as i32,
            pos19.2.floor() as i32,
        );
        let dsq = ((eb.0 - 0) * (eb.0 - 0)
            + (eb.1 - 32) * (eb.1 - 32)
            + (eb.2 - 0) * (eb.2 - 0)) as f64;
        let p = std::cmp::max(1, 15 - ((15.0f64 / 8.0) * dsq.sqrt()).floor() as i32);
        app.update(); // tick 20: the post + the hearing — the path re-opened
        assert_eq!(
            app.game_events.len(),
            3,
            "the tick-20 post + the hearing the re-opened path banks"
        );
        assert!(
            app.game_events[1].starts_with("elytra_glide @20 "),
            "the tick-20 post the lane banks"
        );
        assert_eq!(
            app.game_events[2],
            format!(
                "sculk_hearing @20 x0 y32 z0 d{:.6} t{} f4 p{}",
                d, t, p
            ),
            "the hearing the reference folds pin bit-for-bit"
        );
        // THE ARRIVAL: the broadcast the re-opened activation banks —
        // the wool-refused walk the voice never wore, the re-opened
        // hearing's window the record speaks
        let click_at = 20 + t;
        for _ in 21..click_at {
            app.update();
        }
        app.update();
        assert!(
            app.game_events.iter().any(|l| l.starts_with(&format!(
                "sculk_sensor_tendrils_clicking @{} x0.500000 y32.500000 z0.500000 r16 f0",
                click_at
            ))),
            "the arrival the broadcast the re-opened activation banks"
        );
    }


    /// THE RESONATE WALK (javap tryResonateVibration — the activation's
    /// own pc 56-63 the six-neighbour scan serves): the amethyst the
    /// UP neighbour keeps — the resonate_4 record the resonator's own
    /// center banks (the exact line the reference folds pin), the
    /// broadcast the pc order follows, the amethyst voice the bend
    /// pitch wears (the tritone-down the glide's own frequency keeps),
    /// the re-arm tail the resonate never repeats.
    #[test]
    fn the_resonate_the_amethyst_the_sensor_answers() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let amethyst_id = mc_registry::generated::block_ids::BLOCK_RANGES
            .iter()
            .find(|(_, _, n)| *n == "minecraft:amethyst_block")
            .map(|(s, _, _)| *s as u16)
            .expect("amethyst_block rides the registry");
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 32, 0);
        // the fresh cell the frequency never carries
        assert_eq!(
            app.sculk_sensors[0].resonate_frequency, 0,
            "the fresh cell the hearing never armed"
        );
        // THE AMETHYST the UP neighbour keeps — the resonator the
        // scan's own tag answers
        app.net_world.as_mut().expect("the world rides").chunks[0]
            [33 * crate::world::CHUNK_X * crate::world::CHUNK_Z] = amethyst_id;
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        app.update(); // tick 10: the post + the hearing — the frequency the cell banks
        assert_eq!(
            app.game_events.len(),
            2,
            "the post + the hearing the lane banks — the resonate the arrival speaks"
        );
        assert_eq!(
            app.sculk_sensors[0].resonate_frequency, 4,
            "the glide's own frequency the hearing banks"
        );
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 32.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        let stop_at = click_at + crate::sculk::SCULK_SENSOR_REARM_TICKS as i32;
        for _ in 11..click_at {
            app.update();
        }
        // THE ARRIVAL: the resonate record the walk banks FIRST (the
        // pc order the broadcast's own head keeps), the broadcast the
        // tail wears
        app.update();
        assert_eq!(
            app.game_events.len(),
            4,
            "the resonate + the broadcast the arrival banks — the tick-10 pair the lane keeps"
        );
        assert_eq!(
            app.game_events[2],
            format!(
                "resonate_4 @{} x0.500000 y33.500000 z0.500000 r16 f4",
                click_at
            ),
            "the resonate the walk's own line pins — the resonator's center the atCenterOf fold keeps, the r16 the register(name)'s bipush, the f4 the clinit's resonate row answers"
        );
        assert_eq!(
            app.game_events[3],
            format!(
                "sculk_sensor_tendrils_clicking @{} x0.500000 y32.500000 z0.500000 r16 f0",
                click_at
            ),
            "the broadcast the pc order follows"
        );
        if sounds_loaded() {
            let events = &app.sounds.events;
            let amethyst_idx = events
                .iter()
                .position(|e| e.starts_with("block.amethyst_block.resonate @"))
                .expect("the amethyst voice the bend speaks");
            let voice = &events[amethyst_idx];
            assert!(
                voice.starts_with(&format!("block.amethyst_block.resonate @{click_at} ")),
                "the arrival tick the stamp keeps: {voice}"
            );
            assert!(
                voice.contains(&format!(
                    "p{:.6}",
                    crate::sculk::resonance_pitch_bend(4)
                )),
                "the bend pitch the deterministic fold keeps: {voice}"
            );
            let clicking_idx = events
                .iter()
                .position(|e| e.starts_with("block.sculk_sensor.clicking @"))
                .expect("the CLICKING voice the activation speaks");
            assert!(
                amethyst_idx < clicking_idx,
                "the amethyst the walk's own order keeps — the tryResonate the CLICKING precedes"
            );
        }
        // THE RE-ARM TAIL: the stop the resonate never repeats (the
        // tick walk the gameEvent never posts, the tryResonate the
        // activate's own walk keeps)
        for _ in (click_at + 1)..stop_at {
            app.update();
        }
        app.update();
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("resonate_"))
                .count(),
            1,
            "the re-arm tail the resonate never repeats"
        );
    }


    /// THE SHRIEKER THE CLICKING THE SHRIEK ANSWERS (javap the
    /// dispatch walk — the tendrils-clicking event the
    /// shrieker_can_listen tag ALONE grants, the r8 the user's own
    /// radius keeps, the arrival the tryShriek walk answers): the app
    /// walk — the fresh cell the window never arms, the broadcast the
    /// SHRIEKER hearing follows (the exact line the reference folds
    /// pin — the d the 2.0 fold, the t2 the travel law, the f0 the
    /// map's default, the p0 the no-POWER arm), the arrival the
    /// shriek walk serves (the ring the ten members banks, the v2.0F
    /// voice the pitch band wears, the r32 record the register(name,
    /// 32)'s own bipush keeps, the f0 the map the shriek row never
    /// carries), the SHRIEKING window the in-window hearing refuses
    /// (the re-armed sensor's second broadcast the gate never hears).
    #[test]
    fn the_shrieker_the_clicking_the_shriek_answers() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 32, 0);
        // the SECOND sensor the glide path's own +z drift serves (the
        // elytra walk the fall powers forward) — the fresh ear the
        // window's middle re-arms, the broadcast the SHRIEKING gate's
        // own proof the refusal carries
        app.place_sculk_sensor(0, 32, 6);
        app.place_sculk_shrieker(2, 32, 0, false);
        // the fresh cell the window never arms
        assert_eq!(
            app.sculk_shrikers[0].free_at, 0,
            "the fresh cell the hearing never armed"
        );
        assert!(app.sculk_shrikers[0].shriek_at.is_none());
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 32.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        app.update(); // tick 10: the post + the sensor's own hearing
        assert_eq!(
            app.game_events.len(),
            2,
            "the post + the sensor's hearing — the shrieker the glide's own tag refuses"
        );
        for _ in 11..click_at {
            app.update();
        }
        // THE BROADCAST TICK: the broadcast + the SHRIEKER hearing —
        // the sensor's clicking arm the shrieker's walk follows (the
        // pc order the dispatch keeps)
        app.update();
        assert_eq!(
            app.game_events.len(),
            4,
            "the broadcast + the shrieker's hearing the lane banks"
        );
        assert_eq!(
            app.game_events[2],
            format!(
                "sculk_sensor_tendrils_clicking @{} x0.500000 y32.500000 z0.500000 r16 f0",
                click_at
            ),
            "the broadcast the pc order follows"
        );
        assert_eq!(
            app.game_events[3],
            format!("sculk_hearing @{} x2 y32 z0 d2.000000 t2 f0 p0", click_at),
            "the shrieker's hearing the exact line pins — the d the 2.0 fold, the t2 the travel law, the f0 the map's default, the p0 the no-POWER arm"
        );
        assert_eq!(
            app.sculk_shrikers[0].shriek_at,
            Some((click_at + 2) as u64),
            "the scheduled arrival the travel law banks"
        );
        // THE ARRIVAL: the shriek walk — the ring, the voice, the r32
        // record (the pc order the shriek walk keeps: the levelEvent
        // the gameEvent post precedes)
        let before = app.particles.len();
        for _ in (click_at + 1)..=(click_at + 2) {
            app.update();
        }
        let arrival = click_at + 2;
        assert_eq!(
            app.sculk_shrikers[0].shriek_at,
            None,
            "the arrival the pending walk clears"
        );
        assert_eq!(
            app.sculk_shrikers[0].free_at,
            arrival as u64 + crate::sculk::SHRIEKER_SHRIEK_TICKS,
            "the SHRIEKING window the scheduleTick 90 banks"
        );
        assert_eq!(
            app.particles.len(),
            before + 10,
            "the ten members the ring banks"
        );
        assert_eq!(
            app.game_events[4],
            format!(
                "shriek @{} x2.500000 y32.500000 z0.500000 r32 f0",
                arrival
            ),
            "the GameEvent.SHRIEK post the exact line pins — the r32 the register(name, 32)'s bipush, the f0 the map the shriek row never carries"
        );
        if sounds_loaded() {
            let voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with(&format!("block.sculk_shrieker.shriek @{arrival} ")))
                .expect("the shriek voice the dry arm speaks");
            // the log the FOLDED gain records (the attenuate law the
            // positioned lane speaks — the 2.0F the fold's own input
            // keeps), the pitch band the draw's own answer wears
            let pitch: f32 = voice
                .split(" p")
                .nth(1)
                .and_then(|t| t[..6].parse().ok())
                .unwrap_or(0.0);
            assert!(
                (0.6..1.0).contains(&pitch),
                "the 0.6 + r * 0.4 band the voice keeps: {voice}"
            );
        }
        // THE SHRIEKING WINDOW THE REFUSAL KEEPS: the SECOND sensor's
        // own broadcast (the fresh ear the glide path's +z drift wakes
        // — the event the shrieker's r8 keeps in range) the in-window
        // shrieker never hears (the canReceiveVibration gate the
        // free-at walk serves)
        let hearing_count = app
            .game_events
            .iter()
            .filter(|l| l.starts_with("sculk_hearing @") && l.contains(" x2 y32 z0 "))
            .count();
        assert_eq!(hearing_count, 1, "the first hearing the lane banks alone");
        for _ in (arrival + 1)..=(click_at + 40) {
            app.update();
        }
        let second = app
            .game_events
            .iter()
            .filter(|l| l.starts_with("sculk_sensor_tendrils_clicking @"))
            .any(|l| {
                l.split('@')
                    .nth(1)
                    .and_then(|r| r.split(' ').next())
                    .and_then(|v| v.parse::<i64>().ok())
                    .map(|t| t > click_at as i64)
                    .unwrap_or(false)
            });
        assert!(
            second,
            "the second sensor's broadcast the re-arm walk banks"
        );
        let new_hearings = app
            .game_events
            .iter()
            .filter(|l| l.starts_with("sculk_hearing @") && l.contains(" x2 y32 z0 "))
            .count();
        assert_eq!(
            new_hearings, hearing_count,
            "the SHRIEKING window the in-window hearing refuses"
        );
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("shriek @"))
                .count(),
            1,
            "the window the second shriek refuses"
        );
    }


    /// THE DRAIN THE WARDEN LADDER THE CAN_SUMMON WEARS (javap the
    /// 26.2 jar SculkShriekerBlockEntity.tryShriek pc 0-56 +
    /// WardenSpawnTracker.tryWarn — the arrival walk the tryShriek
    /// law serves: the SHRIEKER'S OWN warningLevel the reset keeps,
    /// the canRespond gate the CAN_SUMMON + the PEACEFUL acmp pair
    /// answers, the failed warn the pc 46-49 return the NO-shriek
    /// tail keeps, the passed warn the OptionalInt the ifPresent put
    /// banks): the app walk — the can-summon cell the ladder arms
    /// (the warden_warn record the walk's own line banks BEFORE the
    /// shriek record the pc order keeps, the tracker the cooldown-200
    /// state wears), the window's re-hearing the cooldown refusal
    /// keeps (the NO-shriek arrival the reset wears — the
    /// warning_level zero the fresh walk banks, the free-at the
    /// refusal never re-arms), the drained window the second warn
    /// wears (the lvl2 line the ladder's own rung speaks), the
    /// PEACEFUL world the can-summon cell the shriek tail wears (the
    /// acmp the gate refuses, NO warn the tracker never draws).
    #[test]
    fn the_drain_the_warden_ladder_the_can_summon_wears() {
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        // === WALK A: the Normal world the ladder arms ===
        let mut app = App::new(3);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 32, 0);
        app.place_sculk_shrieker(2, 32, 0, true);
        assert!(
            app.sculk_shrikers[0].can_summon,
            "the can-summon bit the placement banks"
        );
        assert_eq!(
            app.warden_tracker,
            crate::sculk::WardenSpawnTracker::fresh(),
            "the fresh tracker the ServerPlayer ctor keeps"
        );
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 32.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        app.update(); // tick 10: the post + the sensor's own hearing
        for _ in 11..click_at {
            app.update();
        }
        app.update(); // the broadcast + the shrieker's hearing
        assert_eq!(
            app.game_events.len(),
            4,
            "the broadcast + the hearing the lane banks"
        );
        // THE ARRIVAL: the warn ladder the Some(1) arm — the
        // warden_warn record the shriek record PRECEDES (the
        // tryToWarn walk the shriek tail serves)
        for _ in (click_at + 1)..=(click_at + 2) {
            app.update();
        }
        let arrival = click_at + 2;
        assert_eq!(
            app.game_events[4],
            format!("warden_warn @{} x2 y32 z0 lvl1", arrival),
            "the warn record the exact line pins — the Some the ifPresent put banks"
        );
        assert_eq!(
            app.game_events[5],
            format!(
                "shriek @{} x2.500000 y32.500000 z0.500000 r32 f0",
                arrival
            ),
            "the shriek record the warn's tail wears"
        );
        assert_eq!(
            app.sculk_shrikers[0].warning_level, 1,
            "the OptionalInt the cell's own level banks"
        );
        assert_eq!(
            app.warden_tracker.warning_level, 1,
            "the tracker the ladder's own rung wears"
        );
        assert_eq!(
            app.warden_tracker.cooldown_ticks,
            crate::sculk::WARDEN_WARNING_LEVEL_INCREASE_COOLDOWN,
            "the increase the cooldown-200 window banks"
        );
        assert_eq!(
            app.warden_tracker.ticks_since_last_warning, 0,
            "the increase the counter's own reset wears"
        );
        // THE WINDOW'S RE-HEARING THE COOLDOWN REFUSAL KEEPS: the
        // player's own return the r8 ear's reach keeps (the body the
        // walk re-banks every tick — the elytra drift the return
        // undoes, the glide's own count the return never resets) —
        // the first glide beat the hearing arms again, the arrival
        // walks the ladder — the on-cooldown tracker the None speaks,
        // the pc 46-49 return the NO-shriek tail wears (the reset the
        // fresh walk banks — the cell's own level the zero keeps, the
        // free-at the refusal never re-arms)
        for f in (arrival + 1)..=214 {
            if f >= 95 {
                let b = app.body.as_mut().expect("the body rides");
                b.x = 0.0;
                b.y = 34.0;
                b.z = 0.0;
            }
            app.update();
        }
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("warden_warn @"))
                .count(),
            1,
            "the cooldown the second warn refuses"
        );
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("shriek @"))
                .count(),
            1,
            "the failed warn the second shriek refuses"
        );
        assert_eq!(
            app.sculk_shrikers[0].warning_level, 0,
            "the second arrival the reset wears — the warn failed"
        );
        assert_eq!(
            app.sculk_shrikers[0].free_at,
            arrival as u64 + crate::sculk::SHRIEKER_SHRIEK_TICKS,
            "the refusal the free-at never re-arms"
        );
        // THE DRAINED WINDOW THE SECOND WARN WEARS: the return the
        // hearing's reach keeps again — the first arrival past the
        // 214 drain the ladder's second rung banks (the lvl2 line the
        // walk's own answer keeps) — the walk the +90 window's own
        // reach never crosses (the third arrival the re-armed free-at
        // refuses before the walk ends)
        for _ in 215..=260 {
            let b = app.body.as_mut().expect("the body rides");
            b.x = 0.0;
            b.y = 34.0;
            b.z = 0.0;
            app.update();
        }
        let warns: Vec<&String> = app
            .game_events
            .iter()
            .filter(|l| l.starts_with("warden_warn @"))
            .collect();
        assert_eq!(
            warns.len(),
            2,
            "the drained window the second warn wears"
        );
        assert!(
            warns[1].contains(" x2 y32 z0 lvl2"),
            "the second rung the exact level pins: {}",
            warns[1]
        );
        assert_eq!(
            app.sculk_shrikers[0].warning_level, 2,
            "the cell the second rung banks"
        );
        assert_eq!(
            app.warden_tracker.warning_level, 2,
            "the tracker the second rung wears"
        );
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("shriek @"))
                .count(),
            2,
            "the passed warn the second shriek wears"
        );

        // === WALK B: the Peaceful world the acmp the gate refuses ===
        let mut app = App::new(3);
        let mut world = air_net_world(2, 0);
        world.difficulty = crate::difficulty::Difficulty::Peaceful;
        app.net_world = Some(world);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        app.live_inv_slots[6] = (wire("elytra"), 1);
        app.body = Some(crate::body::PlayerBody::new(0.0, 40.0, 0.0, 0.0, 0.0));
        app.place_sculk_sensor(0, 32, 0);
        app.place_sculk_shrieker(2, 32, 0, true);
        app.on_movement_input(0.0, 0.0, true, false, false);
        app.update();
        app.on_movement_input(0.0, 0.0, false, false, false);
        for _ in 2..=9 {
            app.update();
        }
        let pos9 = {
            let b = app.body.as_ref().expect("the body rides");
            (b.x, b.y, b.z)
        };
        let dx = pos9.0 - 0.5;
        let dy = pos9.1 - 32.5;
        let dz = pos9.2 - 0.5;
        let d = ((dx * dx + dy * dy + dz * dz).sqrt()) as f32;
        let t = d.floor() as i32;
        let click_at = 10 + t;
        app.update();
        for _ in 11..click_at {
            app.update();
        }
        app.update();
        for _ in (click_at + 1)..=(click_at + 2) {
            app.update();
        }
        // THE SHRIEK TAIL THE PLAIN CELL KEEPS: the acmp the
        // canRespond refuses — the shriek wears, NO warn the ladder
        // never arms, the tracker the fresh trio keeps
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("shriek @"))
                .count(),
            1,
            "the peaceful world the shriek tail wears"
        );
        assert_eq!(
            app.game_events
                .iter()
                .filter(|l| l.starts_with("warden_warn @"))
                .count(),
            0,
            "the PEACEFUL acmp the warn never arms"
        );
        assert_eq!(
            app.sculk_shrikers[0].warning_level, 0,
            "the gate the cell's own level keeps"
        );
        assert_eq!(
            app.warden_tracker.warning_level, 0,
            "the tracker the peaceful walk never draws"
        );
        assert_eq!(
            app.warden_tracker.cooldown_ticks, 0,
            "the cooldown the increase never arms"
        );
        assert_eq!(
            app.warden_tracker.ticks_since_last_warning, arrival as i32,
            "the tick walk the doTick slot keeps — the warn the counter never resets"
        );
    }

    /// THE GAMERULE LITERAL'S OWN WALK (javap GameRuleCommand — the
    /// keepInventory row the query/set pair serves, the raw rule id +
    /// the serialized bool the 26.2 lang rows wear, the unknown rule
    /// and the bad value the parse failures keep).
    #[test]
    fn the_offline_command_the_gamerule_the_keepinventory_walks() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(
            !app.keep_inventory,
            "the vanilla default the false boot wears"
        );
        // THE SET TRUE (commands.gamerule.set the two-arg translatable
        // wears — the raw rule id + the serialized bool)
        app.chat_open = true;
        app.chat_input = "/gamerule keepInventory true".into();
        app.on_chat_submit();
        assert!(app.keep_inventory, "the set the row arms");
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gamerule keepInventory is now set to: true"),
        );
        // THE QUERY (commands.gamerule.query the two-arg translatable
        // wears)
        app.chat_open = true;
        app.chat_input = "/gamerule keepInventory".into();
        app.on_chat_submit();
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gamerule keepInventory is currently set to: true"),
        );
        // THE SET FALSE (the row the rule disarms)
        app.chat_open = true;
        app.chat_input = "/gamerule keepInventory false".into();
        app.on_chat_submit();
        assert!(!app.keep_inventory, "the set the row disarms");
        assert_eq!(
            app.chat_messages.last().map(|(m, _)| m.as_str()),
            Some("Gamerule keepInventory is now set to: false"),
        );
        // THE BAD VALUE (the BoolArgumentType's parse the failure
        // keeps — the row unmoved, the RED error rides)
        app.chat_open = true;
        app.chat_input = "/gamerule keepInventory maybe".into();
        app.on_chat_submit();
        assert!(!app.keep_inventory, "the bad value the row unmoved keeps");
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(
            err.starts_with("\u{a7}c"),
            "the parse failure the RED error rides: {err:?}"
        );
        // THE UNKNOWN RULE (the GameRuleArgument's own walk — the
        // parse failure the RED error keeps)
        app.chat_open = true;
        app.chat_input = "/gamerule doDaylightCycle true".into();
        app.on_chat_submit();
        let err = app.chat_messages.last().map(|(m, _)| m.clone()).unwrap();
        assert!(
            err.starts_with("\u{a7}c"),
            "the unknown rule the RED error rides: {err:?}"
        );
        // THE SET THE DEATH ARC ANSWERS (the armed row the death
        // lanes' gates read — the same spare the part_8 law wears)
        app.chat_open = true;
        app.chat_input = "/gamerule keepInventory true".into();
        app.on_chat_submit();
        assert!(app.keep_inventory, "the set the row arms");
    }
