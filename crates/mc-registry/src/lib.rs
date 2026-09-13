//! mc-registry — version-generated block/item registry.
//!
//! The `generated` module is produced by `mc2rs-gen generate --version <id>`.
//! Regenerating for a new version swaps the data without touching the API.

pub mod generated;

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
}
