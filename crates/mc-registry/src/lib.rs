//! mc-registry — version-generated block/item registry.
//!
//! The `generated` module is produced by `mc2rs-gen generate --version <id>`.
//! Regenerating for a new version swaps the data without touching the API.

pub mod generated;

/// The hand-written consumer layer over generated::blockstates: the
/// strict JSON-subset parse of both canonical columns plus the vanilla
/// selection semantics (first-match variants, all-matching multipart
/// parts, "a|b" pipe values). See the module docs.
pub mod blockstate_lookup;

/// The vanilla FaceBakery quad derivation over generated::elements —
/// corners, default uvs, uv rotation, axis/euler element rotations with
/// rescale, computed facing, winding recalculation. Pure data, no
/// render path. See the module docs for the javap evidence.
pub mod element_quads;

/// The vanilla TextureSlots resolution over generated::models — the
/// parent-chain merge (child overrides per key), the "#var" reference
/// fixpoint, the 26.2 inline sprite dicts, and the strip-# lookup that
/// makes bare refs behave identically. Pure data, no render path.
pub mod texture_slots;

/// The vanilla sprite-uv consumer — the quads' 0..16 uv space mapped to
/// texels over generated::textures (CuboidFace.getU /16, the
/// TextureAtlasSprite span, NEAREST + CLAMP_TO_EDGE) with the vanilla
/// missingno checkerboard for the missing materials. Pure data, no
/// render path. See the module docs for the javap evidence.
pub mod sprite_uv;

/// The vanilla display-transform link — the models table's `display`
/// stanzas resolved per context through the parent chain into the
/// view-space pose over the quads (ItemTransform/ItemTransforms +
/// JOML 1.10.8 op-for-op, bit-pinned against the joml oracle). Pure
/// data, no render path. See the module docs for the javap evidence.
pub mod display;

/// The vanilla item-model bake — the ItemModelGenerator sprite extrusion
/// (the generated item models' quads baked from the layer sprites'
/// opaque-pixel outline over generated::textures). Pure data, no render
/// path. See the module docs for the javap evidence.
pub mod item_bake;

/// One block definition: identifier + sorted list of state property names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub name: &'static str,
    pub properties: &'static [&'static str],
}

/// Static, binary-searchable registry over generated data.
pub struct Registry {
    blocks: &'static [Block],
    items: &'static [&'static str],
}

impl Registry {
    pub const fn new(blocks: &'static [Block], items: &'static [&'static str]) -> Self {
        Self { blocks, items }
    }

    pub fn blocks(&self) -> &'static [Block] {
        self.blocks
    }

    pub fn items(&self) -> &'static [&'static str] {
        self.items
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Find a block by identifier (exact match, binary search).
    pub fn find_block(&self, name: &str) -> Option<&'static Block> {
        self.blocks
            .binary_search_by(|b| b.name.cmp(name))
            .ok()
            .map(|i| &self.blocks[i])
    }

    /// Does an item with this identifier exist?
    pub fn has_item(&self, name: &str) -> bool {
        self.items.binary_search(&name).is_ok()
    }

    /// All blocks that expose a given state property, e.g. "facing".
    pub fn blocks_with_property(&self, property: &str) -> Vec<&'static Block> {
        self.blocks
            .iter()
            .filter(|b| b.properties.iter().any(|p| *p == property))
            .collect()
    }
}

/// The registry for the generated target version.
pub static REGISTRY: Registry = Registry::new(generated::blocks::BLOCKS, generated::items::ITEMS);

/// Version stamp written by the generator.
pub mod meta {
    include!("generated/build.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_sorted_and_searchable() {
        let blocks = generated::blocks::BLOCKS;
        assert!(!blocks.is_empty(), "generated registry must not be empty");
        for w in blocks.windows(2) {
            assert!(w[0].name < w[1].name, "registry not sorted: {} !< {}", w[0].name, w[1].name);
        }
    }

    #[test]
    fn common_blocks_exist() {
        assert!(REGISTRY.find_block("stone").is_some());
        assert!(REGISTRY.find_block("grass_block").is_some());
    }

    #[test]
    fn unknown_block_returns_none() {
        assert!(REGISTRY.find_block("not_a_real_block_xyz").is_none());
    }

    #[test]
    fn built_in_dimension_types_carry_the_jar_layout() {
        use generated::worldgen::{dimension_type, DIMENSION_TYPES};
        assert_eq!(DIMENSION_TYPES.len(), 4);
        let ow = dimension_type("minecraft:overworld").expect("overworld");
        assert_eq!((ow.min_y, ow.height, ow.logical_height), (-64, 384, 384));
        assert!(ow.has_skylight && !ow.has_fixed_time);
        assert_eq!(ow.timelines, Some("#minecraft:in_overworld"));
        assert_eq!(ow.default_clock, Some("minecraft:overworld"));
        let nether = dimension_type("minecraft:the_nether").expect("nether");
        // the values the protocol tests pin for the synced registry: floor 0,
        // 256 tall — and the ceiling keeps logical_height BELOW height
        assert_eq!((nether.min_y, nether.height, nether.logical_height), (0, 256, 128));
        assert!(nether.has_fixed_time && !nether.has_skylight);
        assert_eq!((nether.ambient_light, nether.coordinate_scale), (0.1, 8.0));
        assert_eq!(nether.default_clock, None, "the nether names no default clock");
    }

    #[test]
    fn the_day_timeline_keyframes_come_from_the_jar() {
        use generated::worldgen::{timeline, timeline_tag, TIMELINES, TIMELINE_TAGS};
        assert_eq!(TIMELINES.len(), 4);
        assert_eq!(TIMELINE_TAGS.len(), 4);
        let day = timeline("minecraft:day").expect("day timeline");
        assert_eq!(day.clock, "minecraft:overworld");
        assert_eq!(day.period_ticks, Some(24_000));
        assert_eq!(day.tracks, 18);
        assert_eq!(
            day.sky_light_factor,
            [(730, 1.0), (11_270, 1.0), (13_140, 0.24), (22_860, 0.24)]
        );
        // the other real timelines carry no render track — empty, not invented
        let moon = timeline("minecraft:moon").expect("moon timeline");
        assert_eq!((moon.period_ticks, moon.tracks), (Some(192_000), 2));
        assert!(moon.sky_light_factor.is_empty());
        // the tag the overworld dimension type names resolves through the table
        let tag = timeline_tag("minecraft:in_overworld").expect("in_overworld tag");
        assert!(tag.contains(&"minecraft:day"));
        assert!(tag.contains(&"#minecraft:universal"), "nested tag refs ride verbatim");
    }

    #[test]
    fn the_player_skin_arm_geometry_is_the_vanilla_unwrap() {
        use generated::skin::{
            ARM_BOX_PX, ARM_FACES, ARM_TEX_OFFS, SKIN_SIDE, STEVE_WIDE_PNG_SHA1,
            STEVE_WIDE_RGBA_SHA1,
        };
        // the classic skin layout
        assert_eq!(SKIN_SIDE, 64);
        // vanilla PlayerModel (wide model): texOffs (40, 16), box 4x12x4
        assert_eq!(ARM_TEX_OFFS, (40, 16));
        assert_eq!(ARM_BOX_PX, [4, 12, 4]);
        // the six faces unfold exactly like vanilla ModelPart's cube unwrap:
        // top, bottom, then the side strip right -> front -> left -> back
        let (u, v) = ARM_TEX_OFFS;
        let [w, h, d] = ARM_BOX_PX;
        let expected = [
            (u + d, v, w, d),
            (u + d + w, v, w, d),
            (u, v + d, d, h),
            (u + d, v + d, w, h),
            (u + d + w, v + d, d, h),
            (u + d + w + d, v + d, w, h),
        ];
        for (f, e) in ARM_FACES.iter().zip(expected) {
            assert_eq!((f.u, f.v, f.w, f.h), e, "arm unwrap must be the vanilla one");
        }
        // every face stays inside the 64x64 texture
        for f in ARM_FACES.iter() {
            assert!(
                u32::from(f.u) + u32::from(f.w) <= SKIN_SIDE
                    && u32::from(f.v) + u32::from(f.h) <= SKIN_SIDE,
                "face {:?} escapes the skin texture",
                f
            );
        }
        // the source is the REAL 26.2 jar entry, pinned at BOTH layers: the
        // verbatim PNG bytes and the generator's decode of them (independently
        // cross-checked against a second decoder over all 64*64*4 bytes).
        assert_eq!(
            STEVE_WIDE_PNG_SHA1,
            "a030e0de66508e6b21e44ede14cbf71b68edf68c"
        );
        assert_eq!(
            STEVE_WIDE_RGBA_SHA1,
            "e2683b8b87d3a671d54c49377f8bcee89eeb34bf"
        );
    }

    #[test]
    fn the_wire_order_tables_match_the_built_in_tables() {
        use generated::registry_order::{BIOME_ORDER, DIMENSION_TYPE_ORDER, TIMELINE_ORDER};
        use generated::worldgen::{BIOMES, DIMENSION_TYPES, TIMELINES};
        // the evidence layer (the bootstrapped vanilla runtime dump) and the
        // data layer (the generated jar tables) must name the SAME members:
        // a claim built on an order table whose set drifts from the data
        // table would map wire ids onto wrong entries
        let same_set = |order: &[&str], table: Vec<&str>| {
            let mut a: Vec<&str> = order.to_vec();
            a.sort();
            let mut b = table;
            b.sort();
            a == b
        };
        assert!(same_set(&DIMENSION_TYPE_ORDER, DIMENSION_TYPES.iter().map(|d| d.name).collect()));
        assert!(same_set(&TIMELINE_ORDER, TIMELINES.iter().map(|t| t.name).collect()));
        assert!(same_set(&BIOME_ORDER, BIOMES.iter().map(|b| b.name).collect()));
        // the world_clock twin: the 26.2 data files are EMPTY objects, so
        // the names ARE the data — and they must name the same member set
        // the order table does (the skip-sync day-tick path maps a clock
        // name to its wire id through that order)
        assert!(same_set(
            &crate::generated::registry_order::WORLD_CLOCK_ORDER,
            generated::worldgen::WORLD_CLOCKS.to_vec()
        ));
        assert_eq!(
            generated::worldgen::WORLD_CLOCKS,
            ["minecraft:overworld", "minecraft:the_end"]
        );
        // the evidence pins: the real boot's first wire ids
        assert_eq!(DIMENSION_TYPE_ORDER[0], "minecraft:overworld");
        assert_eq!(TIMELINE_ORDER[0], "minecraft:day");
        // no duplicates inside an order table (dense, unique wire ids)
        for order in [&DIMENSION_TYPE_ORDER[..], &TIMELINE_ORDER[..], &BIOME_ORDER[..]] {
            let mut sorted = order.to_vec();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), order.len());
        }
    }

    #[test]
    fn the_tint_extras_join_the_biome_table_with_the_jar_values() {
        use generated::worldgen::{biome, biome_tint_extra, BIOME_TINT_EXTRAS};
        // the four real 26.2 carriers, verbatim from the biome JSONs
        // (effects.grass_color_modifier + effects.dry_foliage_color)
        assert_eq!(BIOME_TINT_EXTRAS.len(), 4);
        let dark = biome_tint_extra("minecraft:dark_forest").expect("dark_forest extras");
        assert_eq!(dark.grass_color_modifier, Some("dark_forest"));
        assert_eq!(dark.dry_foliage_color, Some(0x7B5334));
        let swamp = biome_tint_extra("minecraft:swamp").expect("swamp extras");
        assert_eq!(swamp.grass_color_modifier, Some("swamp"));
        assert_eq!(swamp.dry_foliage_color, Some(0x7B5334));
        let mangrove = biome_tint_extra("minecraft:mangrove_swamp").expect("mangrove extras");
        assert_eq!(mangrove.grass_color_modifier, Some("swamp"));
        let pale = biome_tint_extra("minecraft:pale_garden").expect("pale garden extras");
        assert_eq!(pale.grass_color_modifier, None);
        assert_eq!(pale.dry_foliage_color, Some(0xA0A69C));
        assert_eq!(biome_tint_extra("minecraft:plains"), None, "sparse: carriers only");
        // the join must never dangle: every extras entry names a real biome
        for e in &BIOME_TINT_EXTRAS {
            assert!(
                biome(e.biome).is_some(),
                "extras entry {} has no biome-table twin",
                e.biome
            );
        }
    }

    #[test]
    fn the_biome_tags_join_the_biome_table_and_never_dangle() {
        use generated::worldgen::{biome, biome_tag, BIOME_TAGS};
        // the jar's data/minecraft/tags/worldgen/biome/ file set, verbatim:
        // 68 tags / 390 members, no "replace" key anywhere
        assert_eq!(BIOME_TAGS.len(), 68);
        let total: usize = BIOME_TAGS.iter().map(|t| t.1.len()).sum();
        assert_eq!(total, 390, "every member the jar states is emitted");
        // is_forest's members verbatim, in the jar's stored order
        let forest = biome_tag("minecraft:is_forest").expect("is_forest");
        assert_eq!(
            forest,
            &[
                "minecraft:forest",
                "minecraft:flower_forest",
                "minecraft:birch_forest",
                "minecraft:old_growth_birch_forest",
                "minecraft:dark_forest",
                "minecraft:pale_garden",
                "minecraft:grove",
            ]
        );
        // a nested reference rides verbatim and names a KNOWN tag (the
        // structure tags live in a SUBDIRECTORY — the stem carries the
        // slash: tags/worldgen/biome/has_structure/buried_treasure.json)
        let buried = biome_tag("minecraft:has_structure/buried_treasure")
            .expect("has_structure/buried_treasure");
        assert_eq!(buried[0], "#minecraft:is_beach");
        assert!(biome_tag("#minecraft:is_beach").is_none());
        // THE JOIN NEVER DANGLES: every concrete member names a biome the
        // table carries; every nested ref names a tag the table carries
        for (tag, members) in &BIOME_TAGS {
            for m in members.iter() {
                if let Some(id) = m.strip_prefix('#') {
                    assert!(
                        biome_tag(id).is_some(),
                        "tag {tag} references unknown tag {id}"
                    );
                } else {
                    assert!(
                        biome(m).is_some(),
                        "tag {tag} references unknown biome {m}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_item_model_definitions_resolve_and_the_joins_hold() {
        use generated::item_models::{COMPLEX_ITEM_MODELS, ITEM_MODEL_DEFS, ITEM_MODEL_ROOT_TYPES};
        use generated::items::ITEMS;
        // the jar's assets/minecraft/items/ file set, verbatim: 1537
        // definitions split 1390 plain minecraft:model roots + 147
        // selector/special trees retained in the skip list
        assert_eq!(ITEM_MODEL_DEFS.len(), 1390);
        assert_eq!(COMPLEX_ITEM_MODELS.len(), 147);
        assert_eq!(ITEM_MODEL_DEFS.len() + COMPLEX_ITEM_MODELS.len(), 1537);
        let census: u32 = ITEM_MODEL_ROOT_TYPES.iter().map(|t| t.1).sum();
        assert_eq!(census, 1537, "the root-type census accounts for every item");
        assert_eq!(
            &ITEM_MODEL_ROOT_TYPES,
            &[
                ("minecraft:model", 1390),
                ("minecraft:select", 71),
                ("minecraft:special", 51),
                ("minecraft:composite", 16),
                ("minecraft:condition", 7),
                ("minecraft:range_dispatch", 2),
            ]
        );

        // verbatim spot rows — the jar's own stored strings
        let def = |id: &str| {
            ITEM_MODEL_DEFS
                .iter()
                .find(|d| d.0 == id)
                .unwrap_or_else(|| panic!("{id} must be a plain def"))
        };
        let stone = def("stone");
        assert_eq!(stone.1, "minecraft:block/stone");
        assert!(stone.2.is_empty());
        assert!(stone.3.is_none());
        assert_eq!(def("dirt").1, "minecraft:block/dirt");
        // the fence points at its INVENTORY model variant, verbatim
        assert_eq!(def("acacia_fence").1, "minecraft:block/acacia_fence_inventory");
        // grass_block: the one grass tint, canonical compact JSON, values verbatim
        let grass = def("grass_block");
        assert_eq!(grass.1, "minecraft:block/grass_block");
        assert_eq!(
            grass.2,
            &["{\"downfall\":1.0,\"temperature\":0.5,\"type\":\"minecraft:grass\"}"]
        );
        // filled_map rides TWO tints (constant + map_color), in jar order
        let map = def("filled_map");
        assert_eq!(map.1, "minecraft:item/filled_map");
        assert_eq!(
            map.2,
            &[
                "{\"type\":\"minecraft:constant\",\"value\":-1}",
                "{\"default\":4603950,\"type\":\"minecraft:map_color\"}",
            ]
        );
        // the swap-animation scale ships ONLY beside the spears' select
        // trees — zero plain def carries it (the column exists so the
        // parser drops nothing the jar states)
        assert!(ITEM_MODEL_DEFS.iter().all(|d| d.3.is_none()));
        for spear in [
            "wooden_spear",
            "stone_spear",
            "iron_spear",
            "golden_spear",
            "diamond_spear",
            "netherite_spear",
            "copper_spear",
        ] {
            assert!(ITEM_MODEL_DEFS.iter().all(|d| d.0 != spear));
            assert!(COMPLEX_ITEM_MODELS.contains(&spear), "{spear} in the skip list");
        }
        assert!(COMPLEX_ITEM_MODELS.contains(&"bow"));
        assert!(COMPLEX_ITEM_MODELS.contains(&"shield"));

        // every model path is namespaced minecraft: under block/ or item/
        let (mut block, mut item) = (0usize, 0usize);
        for (id, path, _, _) in &ITEM_MODEL_DEFS {
            let rest = path
                .strip_prefix("minecraft:")
                .unwrap_or_else(|| panic!("{id}: unnamespaced model path {path}"));
            match rest.split('/').next().unwrap() {
                "block" => block += 1,
                "item" => item += 1,
                other => panic!("{id}: model path under unexpected prefix {other}/"),
            }
        }
        assert_eq!((block, item), (731, 659), "the target split the jar states");

        // THE JOINS NEVER DANGLE: every plain def and every skipped item
        // names an item the registry table carries
        for (id, _, _, _) in &ITEM_MODEL_DEFS {
            assert!(ITEMS.contains(id), "def {id} names no known item");
        }
        for id in COMPLEX_ITEM_MODELS.iter() {
            assert!(ITEMS.contains(id), "complex {id} names no known item");
        }
    }

    #[test]
    fn the_model_table_is_the_jar_census_and_the_joins_hold() {
        use generated::item_models::ITEM_MODEL_DEFS;
        use generated::models::{ELEMENT_MODELS, MODELS};
        // the jar's assets/minecraft/models/ file set, verbatim: 3928 files
        // split 2657 block/ + 1271 item/ — cross-checked against an
        // independent python census (the itemgen dual-census precedent)
        assert_eq!(MODELS.len(), 3928);
        let blocks = MODELS.iter().filter(|m| m.0.starts_with("block/")).count();
        let items = MODELS.iter().filter(|m| m.0.starts_with("item/")).count();
        assert_eq!((blocks, items), (2657, 1271));
        // the element geometry is retained in the honest skip list
        assert_eq!(ELEMENT_MODELS.len(), 344);
        assert_eq!(ELEMENT_MODELS.first(), Some(&"block/bamboo1_age0"));
        assert_eq!(ELEMENT_MODELS.last(), Some(&"item/spyglass_in_hand"));
        // the closed census counts: gui_light carriers and the
        // ambient-occlusion overrides
        assert_eq!(MODELS.iter().filter(|r| r.5.is_some()).count(), 12);
        assert_eq!(MODELS.iter().filter(|r| r.6.is_some()).count(), 157);

        let model = |id: &str| {
            MODELS
                .iter()
                .find(|m| m.0 == id)
                .unwrap_or_else(|| panic!("{id} must be a model"))
        };

        // verbatim spot rows — the jar's own stored strings (canonical
        // compact JSON, serde key order)
        let stone = model("block/stone");
        assert_eq!(stone.1, Some("block/cube_all"), "namespace stripped");
        assert_eq!(stone.2, Some("{\"all\":\"minecraft:block/stone\"}"));
        assert_eq!(stone.4, 0);
        let apple = model("item/apple");
        assert_eq!(apple.1, Some("item/generated"));
        assert_eq!(apple.2, Some("{\"layer0\":\"minecraft:item/apple\"}"));
        let stairs = model("block/oak_stairs");
        assert_eq!(
            stairs.2,
            Some("{\"bottom\":\"minecraft:block/oak_planks\",\"side\":\"minecraft:block/oak_planks\",\"top\":\"minecraft:block/oak_planks\"}")
        );

        // the root display transforms — the held-item consumer's constants
        let block_root = model("block/block");
        assert_eq!(block_root.1, None, "block/block is a chain root");
        assert_eq!(block_root.5, Some("side"));
        let d = block_root.3.unwrap();
        assert!(d.contains("\"gui\":{\"rotation\":[30,225,0],\"scale\":[0.625,0.625,0.625],\"translation\":[0,0,0]}"), "{d}");
        // 26.2's shelf context rides the root
        assert!(d.contains("\"on_shelf\":{\"rotation\":[0,180,0],\"scale\":[1,1,1],\"translation\":[0,0,0]}"), "{d}");
        assert!(!d.contains("thirdperson_lefthand"), "the block root states only the right-hand context");
        let generated = model("item/generated");
        assert_eq!(generated.1, Some("builtin/generated"));
        assert_eq!(generated.5, Some("front"));
        let d = generated.3.unwrap();
        assert!(d.contains("\"firstperson_righthand\":{\"rotation\":[0,-90,25],\"scale\":[0.68,0.68,0.68],\"translation\":[1.13,3.2,1.13]}"), "{d}");
        let handheld = model("item/handheld");
        let d = handheld.3.unwrap();
        assert!(d.contains("\"thirdperson_righthand\":{\"rotation\":[0,-90,55],\"scale\":[0.85,0.85,0.85],\"translation\":[0,4.0,0.5]}"), "{d}");

        // the two outliers the full census caught, verbatim
        let heavy = model("block/heavy_core");
        assert_eq!(heavy.1, None, "heavy_core is its own root");
        assert_eq!(heavy.7, Some((16, 16)), "the element-UV size override");
        let fence = model("block/custom_fence_side_east");
        assert_eq!(
            fence.8,
            Some("[{\"children\":[0,1],\"color\":0,\"name\":\"east\",\"origin\":[0,0,0]}]"),
            "the Blockbench groups metadata is retained, never dropped"
        );
        // the 26.2 object-valued texture entries (redstone/glass) ride through
        let dust = model("block/redstone_dust_dot");
        assert!(dust.2.unwrap().contains("\"force_translucent\":true"));

        // the deepest chain, verbatim: the crafter family down to the root
        let chain = [
            "block/crafter_crafting_triggered",
            "block/crafter_crafting",
            "block/crafter_triggered",
            "block/crafter",
        ];
        for (a, b) in chain.iter().zip(chain.iter().skip(1)) {
            assert_eq!(model(a).1, Some(*b));
        }
        assert_eq!(model("block/crafter").1, Some("block/block"));

        // the parent graph re-verified FROM the emitted table: every parent
        // resolves to a file or a builtin/, acyclic, max depth 4
        let by_id = |id: &str| MODELS.iter().find(|m| m.0 == id);
        let mut max_depth = 0u32;
        for row in MODELS.iter() {
            let id = row.0;
            let mut cur = row.0;
            let mut seen = std::collections::BTreeSet::new();
            let mut depth = 0u32;
            loop {
                let parent = by_id(cur).expect("every walked id exists").1;
                let p = match parent {
                    None => break,
                    Some(p) if p.starts_with("builtin/") => break,
                    Some(p) => p,
                };
                assert!(by_id(p).is_some(), "{id}: dangling parent {p}");
                assert!(seen.insert(cur), "{id}: parent cycle at {cur}");
                cur = p;
                depth += 1;
            }
            max_depth = max_depth.max(depth);
        }
        assert_eq!(max_depth, 4, "the census depth (the crafter chain)");

        // THE JOIN: every plain item def's model path resolves to a file
        for (_, path, _, _) in ITEM_MODEL_DEFS.iter() {
            let bare = path.strip_prefix("minecraft:").unwrap();
            assert!(by_id(bare).is_some(), "def model {bare} resolves to no model file");
        }
        // 24 defs point directly at element-carrying models (beacon,
        // cactus, crafter...) — the consumer round's bridge set
        let elemental: std::collections::BTreeSet<&str> = ELEMENT_MODELS.iter().copied().collect();
        let hits = ITEM_MODEL_DEFS
            .iter()
            .filter(|d| elemental.contains(d.1.strip_prefix("minecraft:").unwrap()))
            .count();
        assert_eq!(hits, 24, "defs pointing directly at element-carrying models");
    }

    #[test]
    fn the_elements_geometry_is_the_jar_census_and_the_joins_hold() {
        use generated::elements::{ELEMENTS, ElementRotation};
        use generated::models::{ELEMENT_MODELS, MODELS};

        // the census, verbatim: 344 element-carrying models, 1185 elements,
        // 4238 faces — the python census, cross-checked by the Rust run
        assert_eq!(ELEMENTS.len(), 344);
        let total: usize = ELEMENTS.iter().map(|(_, e)| e.len()).sum();
        assert_eq!(total, 1185);
        let faces: usize = ELEMENTS
            .iter()
            .flat_map(|(_, e)| e.iter())
            .map(|el| el.faces.iter().flatten().count())
            .sum();
        assert_eq!(faces, 4238);

        // THE JOIN: the ELEMENTS rows match ELEMENT_MODELS id-for-id and the
        // MODELS element-count column row-for-row; the mirror direction too
        let ids: Vec<&str> = ELEMENTS.iter().map(|(id, _)| *id).collect();
        let skip: Vec<&str> = ELEMENT_MODELS.iter().copied().collect();
        assert_eq!(ids, skip);
        for (id, els) in ELEMENTS.iter() {
            let row = MODELS
                .iter()
                .find(|m| m.0 == *id)
                .unwrap_or_else(|| panic!("{id} must be a model"));
            assert_eq!(row.4 as usize, els.len(), "{id}: element-count join");
        }
        for m in MODELS.iter().filter(|m| m.4 > 0) {
            assert!(
                ELEMENTS.iter().any(|(id, _)| *id == m.0),
                "{} carries elements in MODELS but has no ELEMENTS row",
                m.0
            );
        }

        // closed domains across the whole table: bounds inside the vanilla
        // clamp [-16, 32], uvs inside [0, 16], face rotation a multiple of
        // 90 (the census domain {0, 90, 180, 270})
        for (id, els) in ELEMENTS.iter() {
            for el in els.iter() {
                for c in el.from.iter().chain(el.to.iter()) {
                    assert!((-16.0..=32.0).contains(c), "{id}: bound {c}");
                }
                for f in el.faces.iter().flatten() {
                    if let Some(uv) = f.uv {
                        for v in uv {
                            assert!((0.0..=16.0).contains(&v), "{id}: uv {v}");
                        }
                    }
                    if let Some(r) = f.rotation {
                        assert_eq!(r % 90, 0, "{id}: face rotation {r}");
                    }
                }
            }
        }

        // block/cube — the six-face box, verbatim (uv: None = the vanilla
        // bounds projection; cullface mirrors each slot's direction)
        let cube = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/cube")
            .unwrap()
            .1;
        assert_eq!(cube.len(), 1);
        let c0 = &cube[0];
        assert_eq!(c0.from, [0.0, 0.0, 0.0]);
        assert_eq!(c0.to, [16.0, 16.0, 16.0]);
        assert!(c0.shade && c0.rotation.is_none() && c0.light_emission.is_none());
        assert_eq!(c0.name, None);
        let refs = ["#down", "#up", "#north", "#south", "#west", "#east"];
        for (slot, face) in c0.faces.iter().enumerate() {
            let f = face
                .as_ref()
                .unwrap_or_else(|| panic!("cube face {slot} missing"));
            assert_eq!(f.uv, None, "the cube ships default uvs");
            assert_eq!(f.texture, refs[slot]);
            assert_eq!(f.cullface.map(|c| c as usize), Some(slot));
            assert_eq!(f.tintindex, None);
            assert_eq!(f.rotation, None);
        }

        // the 26.2 euler EXTENSION — template_hanging_sign_rot_3 ships two
        // {x,y,z,origin} rotations verbatim among its axis rotations, and
        // the mirrored uvs survive as written
        let hs = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/template_hanging_sign_rot_3")
            .unwrap()
            .1;
        assert_eq!(hs.len(), 5);
        let rotations: Vec<String> = hs
            .iter()
            .map(|el| match el.rotation {
                Some(ElementRotation::Axis { angle, axis, origin, rescale }) => format!(
                    "A({angle},{:?},{origin:?},{rescale})",
                    axis
                ),
                Some(ElementRotation::Euler { x, y, z, origin }) => {
                    format!("E({x},{y},{z},{origin:?})")
                }
                None => "none".to_string(),
            })
            .collect();
        assert_eq!(
            rotations,
            vec![
                "A(-67.5,Y,[8.0, 0.0, 8.0],false)",
                "A(-22.5,Y,[8.0, 0.0, 8.0],false)",
                "E(180,-67.5,-180,[8.0, 0.0, 8.0])",
                "A(-22.5,Y,[8.0, 0.0, 8.0],false)",
                "E(180,-67.5,-180,[8.0, 0.0, 8.0])",
            ]
        );
        assert_eq!(hs[2].faces[2].unwrap().uv, Some([15.5, 4.0, 14.0, 6.0]));
        assert_eq!(hs[2].faces[3].unwrap().uv, Some([14.0, 4.0, 15.5, 6.0]));

        // heavy_core's BARE texture refs (no '#') — six faces, as written
        let hc = ELEMENTS
            .iter()
            .find(|(id, _)| *id == "block/heavy_core")
            .unwrap()
            .1;
        assert_eq!(hc.len(), 1);
        for f in hc[0].faces.iter().flatten() {
            assert_eq!(f.texture, "all", "heavy_core's bare ref");
        }

        // the angle domain is RELAXED in 26.2 — the old {-45,-22.5,0,22.5,45}
        // five-value set is gone; the census counts of every shipped angle
        let angle_count = |want: f32| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter_map(|el| el.rotation)
                .filter(|r| matches!(r, ElementRotation::Axis { angle, .. } if *angle == want))
                .count()
        };
        assert_eq!(angle_count(45.0), 131);
        assert_eq!(angle_count(-45.0), 70);
        assert_eq!(angle_count(-22.5), 49);
        assert_eq!(angle_count(22.5), 22);
        assert_eq!(angle_count(0.0), 19);
        assert_eq!(angle_count(-67.5), 7, "the relaxed domain");
        assert_eq!(angle_count(-90.0), 2, "the relaxed domain");
        assert_eq!(angle_count(0.0001), 1, "the relaxed domain");
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| matches!(el.rotation, Some(ElementRotation::Euler { .. })))
                .count(),
            2,
            "the euler rotations ride alongside"
        );

        // the face-level census: rotation {0x1, 90x151, 180x39, 270x55},
        // tintindex {0x90, 1x32}, 531 default-uv faces, 1041 culled faces
        let frot = |want: u16| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .flat_map(|el| el.faces.iter().flatten())
                .filter_map(|f| f.rotation)
                .filter(|r| *r == want)
                .count()
        };
        assert_eq!((frot(0), frot(90), frot(180), frot(270)), (1, 151, 39, 55));
        let ftint = |want: u8| {
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .flat_map(|el| el.faces.iter().flatten())
                .filter_map(|f| f.tintindex)
                .filter(|t| *t == want)
                .count()
        };
        assert_eq!((ftint(0), ftint(1)), (90, 32));
        let all_faces = ELEMENTS
            .iter()
            .flat_map(|(_, els)| els.iter())
            .flat_map(|el| el.faces.iter().flatten());
        assert_eq!(all_faces.clone().filter(|f| f.uv.is_none()).count(), 531);
        assert_eq!(all_faces.filter(|f| f.cullface.is_some()).count(), 1041);

        // light_emission: exactly the four emissive-cross elements, 15
        let emissive: Vec<(&str, u8)> = ELEMENTS
            .iter()
            .flat_map(|(id, els)| {
                els.iter()
                    .filter_map(move |el| el.light_emission.map(|l| (*id, l)))
            })
            .collect();
        assert_eq!(emissive.len(), 4);
        assert!(
            emissive
                .iter()
                .all(|(id, l)| *l == 15 && id.ends_with("cross_emissive"))
        );

        // the shade census: 333 explicit-false elements (852 default-or-true)
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| !el.shade)
                .count(),
            333
        );
        // editor metadata retained: 94 names, 63 comments
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| el.name.is_some())
                .count(),
            94
        );
        assert_eq!(
            ELEMENTS
                .iter()
                .flat_map(|(_, els)| els.iter())
                .filter(|el| el.comment.is_some())
                .count(),
            63
        );
    }

    #[test]
    fn the_blockstate_table_is_the_jar_census_and_the_joins_hold() {
        use generated::blockstates::BLOCKSTATES;
        use generated::blocks::BLOCKS;
        use generated::models::MODELS;

        // the census, verbatim: 1198 files split EXACTLY variants (1096) or
        // multipart (102) — never both, never neither
        assert_eq!(BLOCKSTATES.len(), 1198);
        let with_variants = BLOCKSTATES.iter().filter(|r| r.1.is_some()).count();
        let with_multipart = BLOCKSTATES.iter().filter(|r| r.2.is_some()).count();
        assert_eq!((with_variants, with_multipart), (1096, 102));
        assert!(BLOCKSTATES.iter().all(|r| r.1.is_some() ^ r.2.is_some()));

        // the total applies = the "model" key occurrences across all JSON
        // columns; the distinct namespace-stripped refs join the models table
        let mut total_applies = 0usize;
        let mut refs: Vec<&str> = Vec::new();
        for (_, variants, multipart) in BLOCKSTATES.iter() {
            let json = variants.or(*multipart).unwrap();
            let needle = "\"model\":\"";
            let mut rest = json;
            while let Some(i) = rest.find(needle) {
                rest = &rest[i + needle.len()..];
                let end = rest.find('"').unwrap();
                refs.push(&rest[..end]);
                rest = &rest[end..];
                total_applies += 1;
            }
        }
        assert_eq!(total_applies, 8661);
        refs.sort_unstable();
        refs.dedup();
        assert_eq!(refs.len(), 2380, "the distinct model refs");
        for r in &refs {
            let bare = r.strip_prefix("minecraft:").unwrap_or(r);
            assert!(
                MODELS.iter().any(|m| m.0 == bare),
                "blockstate model ref {r} resolves to no model file"
            );
        }
        // the 4 vanilla BARE refs (no namespace: item_frame x2 incl. the
        // _map variant, glow_item_frame x2 likewise) ride as written — the
        // needle's leading quote keeps the namespaced refs out
        let bare_total = BLOCKSTATES
            .iter()
            .filter_map(|r| r.1.as_ref().or(r.2.as_ref()))
            .map(|j| j.matches("\"model\":\"block/").count())
            .sum::<usize>();
        assert_eq!(bare_total, 4, "the vanilla bare refs");

        // spot rows — the serde-canonical JSON fragments, byte-verbatim
        let grass = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "grass_block")
            .unwrap()
            .1
            .as_ref()
            .unwrap();
        assert!(grass.contains("\"snowy=false\":[{\"model\":\"minecraft:block/grass_block\"},"));
        assert!(grass.contains("{\"model\":\"minecraft:block/grass_block\",\"y\":270}]"));
        assert!(grass.contains("\"snowy=true\":{\"model\":\"minecraft:block/grass_block_snow\"}}"));
        let leaf = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "leaf_litter")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(leaf.starts_with(
            "[{\"apply\":{\"model\":\"minecraft:block/leaf_litter_1\"},\"when\":{\"facing\":\"north\",\"segment_amount\":\"1\"}}"
        ));
        let wire = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "redstone_wire")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(wire
            .contains("\"OR\":[{\"east\":\"none\",\"north\":\"none\",\"south\":\"none\",\"west\":\"none\"}"));
        let shelf = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "acacia_shelf")
            .unwrap()
            .2
            .as_ref()
            .unwrap();
        assert!(shelf.contains("\"AND\":[{\"facing\":\"north\"},{\"powered\":\"false\"}]"));
        // the repeater carries the census max variant count (64 states)
        let rep = BLOCKSTATES
            .iter()
            .find(|r| r.0 == "repeater")
            .unwrap()
            .1
            .as_ref()
            .unwrap();
        assert_eq!(rep.matches("{\"model\":").count(), 64);

        // THE JOIN: every blockstate id is a block in the registry (both
        // tables derive from the same jar directory)
        for (id, _, _) in BLOCKSTATES.iter() {
            assert!(
                BLOCKS.iter().any(|b| b.name == *id),
                "blockstate {id} is not a block"
            );
        }
    }

    #[test]
    fn the_texture_namespace_is_sorted_and_binary_searchable() {
        use generated::textures::{is_missing, texture_index, TEXTURE_IDS, N_TEXTURES};

        // the census: 2113 distinct concrete ids over the 3928 model files
        assert_eq!(N_TEXTURES, 2113);
        assert_eq!(TEXTURE_IDS.len(), N_TEXTURES);
        assert!(
            TEXTURE_IDS.windows(2).all(|w| w[0] < w[1]),
            "the namespace is strictly ascending (binary-search prerequisite)"
        );

        // hits: block/, item/, and a 32x32 sign sprite
        for hit in ["block/stone", "item/iron_sword", "block/oak_sign"] {
            assert!(texture_index(hit).is_some(), "{hit} must hit");
        }
        // a hit is NOT existence: the literal `missingno` (item/air.json and
        // block/air.json reference `minecraft:missingno`) is a domain member
        // pinned missing
        let air = texture_index("missingno").expect("the air models' literal ref");
        assert!(is_missing(air));
        assert!(generated::textures::texture_rgba(air).is_empty());

        // misses: the empty id, a bare stem, an unknown block, a texture
        // that EXISTS in the jar but is NOT model-referenced (the skin rides
        // the entity path — skingen's domain, not the models'), and a case
        // mismatch
        for miss in ["", "stone", "block/nonexistent", "entity/player/wide/steve", "Block/stone"] {
            assert!(texture_index(miss).is_none(), "{miss:?} must miss");
        }
    }

    #[test]
    fn every_present_tile_spans_exactly_its_size() {
        use generated::textures::{
            is_missing, texture_size, HEIGHTS, OFFSETS, TEXELS, TEXTURE_IDS, WIDTHS, N_MISSING,
            N_TEXTURES,
        };

        assert_eq!(N_MISSING, 65);
        let mut present = 0usize;
        let mut n16 = 0usize;
        let mut n32 = 0usize;
        for i in 0..N_TEXTURES {
            let (w, h) = texture_size(i as u16);
            let span = (OFFSETS[i + 1] - OFFSETS[i]) as usize;
            if w == 0 {
                // the missing invariant: all three encodings agree
                assert_eq!(h, 0, "{} must be fully empty", TEXTURE_IDS[i]);
                assert_eq!(span, 0);
                assert!(is_missing(i as u16));
                continue;
            }
            assert!(!is_missing(i as u16));
            assert_eq!(span, w as usize * h as usize * 4, "{}", TEXTURE_IDS[i]);
            match (w, h) {
                (16, 16) => n16 += 1,
                (32, 32) => n32 += 1,
                other => panic!("unexpected dims {other:?} for {}", TEXTURE_IDS[i]),
            }
            present += 1;
        }
        // the census, verbatim: 2048 present (2001 square + 47 sign-size) + 65
        // missing = 2113; the offsets tile the texels exactly
        assert_eq!(present, 2048);
        assert_eq!((n16, n32), (2001, 47));
        assert_eq!(present + N_MISSING, N_TEXTURES);
        assert_eq!(OFFSETS[N_TEXTURES] as usize, TEXELS.len());
        assert_eq!(TEXELS.len(), 2_241_536, "the census texel total");
        assert_eq!(WIDTHS.len(), N_TEXTURES);
        assert_eq!(HEIGHTS.len(), N_TEXTURES);
    }

    #[test]
    fn block_stone_pixels_are_the_world_atlas_tile_byte_for_byte() {
        use generated::atlas::{TILE_NAMES, TILES};
        use generated::textures::{texture_index, texture_rgba, texture_size};

        // the CROSS-TABLE pin: stone is untinted, so the world atlas's baked
        // tile and this table's raw tile must be byte-identical
        let stone = texture_index("block/stone").expect("stone in the namespace");
        let atlas_idx = TILE_NAMES
            .iter()
            .position(|n| *n == "stone")
            .expect("the world atlas carries stone");
        assert_eq!(texture_size(stone), (16, 16));
        assert_eq!(texture_rgba(stone), TILES[atlas_idx]);

        // the independent-decoder pins (PIL over the same jar): the corners
        // and the center of the REAL stone.png
        let px = texture_rgba(stone);
        assert_eq!(&px[0..4], &[143, 143, 143, 255], "top-left");
        assert_eq!(&px[15 * 4..16 * 4], &[127, 127, 127, 255], "top-right");
        assert_eq!(&px[15 * 16 * 4..15 * 16 * 4 + 4], &[127, 127, 127, 255], "bottom-left");
        assert_eq!(&px[16 * 16 * 4 - 4..], &[127, 127, 127, 255], "bottom-right");
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[143, 143, 143, 255], "center");
    }

    #[test]
    fn the_raw_pixels_carry_no_baked_tint() {
        use generated::atlas::{TILE_NAMES, TILES};
        use generated::textures::{texture_index, texture_rgba};

        // the TINT-BOUNDARY pair: stone (no tint) is byte-identical to the
        // atlas; grass_block_top (the 0x91BD59 plains bake in the world
        // atlas) must DIFFER — this table keeps the raw grayscale master the
        // consumer tints at draw time (the quads' tintindex)
        let grass = texture_index("block/grass_block_top").expect("grass top");
        let px = texture_rgba(grass);
        // the PIL pins: raw top-left and center
        assert_eq!(&px[0..4], &[148, 148, 148, 255]);
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[179, 179, 179, 255]);
        let atlas_idx = TILE_NAMES
            .iter()
            .position(|n| *n == "grass_block_top")
            .expect("the world atlas bakes grass_block_top");
        assert_ne!(
            px, TILES[atlas_idx],
            "the raw master must NOT equal the tint-baked atlas tile"
        );
        // the baked value the atlas carries for the same pixel: the vanilla
        // multiply (148 * 0x91 + 127) / 255 — proving the DIFF is the tint
        let baked = ((148u16 * 0x91 + 127) / 255) as u8;
        assert_eq!(TILES[atlas_idx][0], baked);

        // semi-transparency rides intact: water_still's raw alpha is 180
        let water = texture_index("block/water_still").expect("water");
        assert_eq!(&texture_rgba(water)[0..4], &[194, 194, 194, 180]);
    }

    #[test]
    fn animated_strips_ride_their_first_frame_and_signs_ride_whole() {
        use generated::textures::{texture_index, texture_rgba, texture_size};

        // water_still.png is 16x512 with an animation mcmeta: the table's
        // sprite is the first 16x16 frame (the world atlas's crop rule,
        // now mcmeta-faithful and pinned on the namespace)
        let water = texture_index("block/water_still").unwrap();
        assert_eq!(texture_size(water), (16, 16));

        // oak_sign.png is a WHOLE 32x32 sprite (no mcmeta): never cropped
        let sign = texture_index("block/oak_sign").unwrap();
        assert_eq!(texture_size(sign), (32, 32));
        let px = texture_rgba(sign);
        assert_eq!(px.len(), 32 * 32 * 4);
        assert_eq!(&px[0..4], &[194, 157, 98, 255], "sign top-left (PIL)");
        assert_eq!(&px[32 * 32 * 4 - 4..], &[0, 0, 0, 0], "sign bottom-right (transparent)");

        // transparent pixels survive decode: the iron sword's top-left is
        // fully transparent, its center the steel gray (PIL)
        let sword = texture_index("item/iron_sword").unwrap();
        assert_eq!(texture_size(sword), (16, 16));
        let px = texture_rgba(sword);
        assert_eq!(&px[0..4], &[0, 0, 0, 0]);
        assert_eq!(&px[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4], &[216, 216, 216, 255]);
    }

    #[test]
    fn the_missing_materials_are_exactly_the_census() {
        use generated::textures::{is_missing, texture_index, TEXTURE_IDS, N_MISSING};

        // 65 missing: the 64 trims/items armor-trim refs (the client jar
        // ships no PNG for them — vanilla renders the checkerboard) + the
        // literal `missingno`; NOTHING from block/ or item/ is missing
        let mut missing: Vec<&str> = Vec::new();
        for (i, id) in TEXTURE_IDS.iter().enumerate() {
            assert_eq!(
                is_missing(i as u16),
                *id == "missingno" || id.starts_with("trims/items/"),
                "{id} missingness disagrees with the census"
            );
            if is_missing(i as u16) {
                missing.push(id);
            }
        }
        assert_eq!(missing.len(), N_MISSING);
        assert_eq!(missing.len(), 65);
        assert_eq!(
            missing.iter().filter(|id| id.starts_with("trims/items/")).count(),
            64
        );
        assert!(missing.contains(&"missingno"));

        // the 57 unreferenced trims PNGs stay OUT of the namespace: the jar
        // ships trims/color_palettes/* (the RUNTIME material lookup — zero
        // overlap with the 64 missing trims/items/* refs), so only REFERENCED
        // ids land in the table — the domain is the models' domain
        assert!(texture_index("trims/color_palettes/iron").is_none());
        assert!(!TEXTURE_IDS.iter().any(|id| id.starts_with("trims/color_palettes/")));
    }
}
