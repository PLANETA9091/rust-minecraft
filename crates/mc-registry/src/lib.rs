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
}
