//! The vanilla TextureSlots resolution as a pure consumer over the
//! generated MODELS table — the quad's texture ref ("#var" or heavy_core's
//! bare "all") becomes a concrete texture id. Zero dependencies, zero
//! render-path contact.
//!
//! Every rule is javap evidence from the 26.2 client jar
//! (net.minecraft.client.resources.model.sprite.TextureSlots*):
//! - parseEntry: a textures-map value that is a string starting with '#'
//!   becomes a Reference with the '#' stripped at parse time; any other
//!   string is a Value (the full texture id, namespace AS WRITTEN — 10287
//!   carry "minecraft:", 807 are bare paths like heavy_core's
//!   "block/heavy_core"); the 26.2 inline sprite dict
//!   {"force_translucent": true, "sprite": "..."} (163 carriers, the flag
//!   ALWAYS true) is a Value whose material is the sprite field.
//! - Resolver.resolve(): one Data per model in the parent chain; the
//!   iteration walks Lists.reverse(entries) — root FIRST, child LAST — so
//!   the child OVERRIDES the ancestors PER KEY. A Value SETS the slot and
//!   REMOVES a prior reference for that key; a Reference RECORDS the
//!   target and REMOVES a prior value.
//! - Then the FIXPOINT loop: while refs remain and progress is made,
//!   resolved[target] fills resolved[key] — one hop per pass, so var->var
//!   chains of any depth resolve (the census's max is 2 passes: the
//!   furnace family's bottom/down chains through block/block). A cycle
//!   simply stops progressing.
//! - Leftover references are LOGGER.warn'ed and DROPPED — the 26.2 data
//!   has exactly 258 (mid, slot, target) triples, all in the TEMPLATE
//!   models whose children fill the slots when baked (block/button's
//!   {"particle": "#texture"} is the textbook case).
//! - getMaterial(name): strip a leading '#', look the string up in the
//!   resolved map — BARE refs behave identically to "#var" refs.

use crate::generated::models::MODELS;

/// The resolved texture slots of one model: `resolved` maps slot -> texture
/// id (as written, namespace included when the jar wrote it) in insertion
/// order; `unresolved` holds the vanilla-warned dangling references
/// (slot -> target var). The counters mirror the census ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSlots {
    pub resolved: Vec<(&'static str, &'static str)>,
    pub unresolved: Vec<(&'static str, &'static str)>,
    pub chain_depth: usize,
    pub overrides: usize,
    pub fixpoint_passes: usize,
}

impl ResolvedSlots {
    /// The texture id behind a face ref — the '#' of a "#var" ref is
    /// stripped and the bare key looked up, so BARE refs (heavy_core's
    /// "all") behave identically (TextureSlots.getMaterial bytecode).
    pub fn material(&self, texture_ref: &str) -> Option<&'static str> {
        let key = texture_ref.strip_prefix('#').unwrap_or(texture_ref);
        self.resolved
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
}

/// One parsed textures-map entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotValue {
    /// A full texture id, as written.
    Value(&'static str),
    /// A "#var" reference, '#' already stripped.
    Ref(&'static str),
}

/// The merged slots of one model, by id (linear scan over the id-sorted
/// table). None when the id is not a model file.
pub fn resolve_model(model_id: &str) -> Option<ResolvedSlots> {
    // The parent chain, root first (the generator validated the chains
    // acyclic and terminating; the depth cap is the fail-closed guard).
    let mut chain = Vec::new();
    let mut cur = model_id;
    loop {
        let (parent, _) = lookup(cur)?;
        chain.push(cur);
        match parent {
            // the vanilla built-ins are anchors, not model files
            Some(p) if !p.starts_with("builtin/") => {
                if chain.len() > 32 {
                    return None; // vanishingly defensive; the data is acyclic
                }
                cur = p;
            }
            _ => break,
        }
    }
    let chain_depth = chain.len() - 1;

    // The merge: root first, child last — the child overrides per key.
    let mut resolved: Vec<(&'static str, &'static str)> = Vec::new();
    let mut refs: Vec<(&'static str, &'static str)> = Vec::new();
    let mut overrides = 0usize;
    for mid in chain.iter().rev() {
        let (_, textures) = lookup(mid)?;
        let Some(json) = textures else { continue };
        for (key, value) in parse_textures(json) {
            let taken = resolved.iter().any(|(k, _)| *k == key)
                || refs.iter().any(|(k, _)| *k == key);
            if taken {
                overrides += 1;
            }
            match value {
                SlotValue::Ref(target) => {
                    resolved.retain(|(k, _)| *k != key);
                    upsert(&mut refs, key, target);
                }
                SlotValue::Value(id) => {
                    refs.retain(|(k, _)| *k != key);
                    upsert(&mut resolved, key, id);
                }
            }
        }
    }

    // The fixpoint: one hop per pass, until quiet. The pass ledger counts
    // every entered scan (the vanilla do-while shape) — the final futile
    // scan of a dangling-only model runs too, so it counts (the census's
    // ledger: 2063 models never enter, 1857 pass once, 8 twice).
    let mut fixpoint_passes = 0usize;
    while !refs.is_empty() {
        fixpoint_passes += 1;
        let mut progressed = false;
        let mut i = 0;
        while i < refs.len() {
            let (key, target) = refs[i];
            if let Some((_, v)) = resolved.iter().find(|(k, _)| *k == target) {
                let v = *v;
                upsert(&mut resolved, key, v);
                refs.remove(i);
                progressed = true;
            } else {
                i += 1;
            }
        }
        if !progressed {
            break;
        }
    }

    Some(ResolvedSlots {
        resolved,
        unresolved: refs,
        chain_depth,
        overrides,
        fixpoint_passes,
    })
}

/// The texture id behind a face ref of one model, by id — the
/// resolve_model(...).material(ref) shorthand.
pub fn resolve_ref(model_id: &str, texture_ref: &str) -> Option<&'static str> {
    resolve_model(model_id)?.material(texture_ref)
}

fn upsert(map: &mut Vec<(&'static str, &'static str)>, key: &'static str, value: &'static str) {
    match map.iter_mut().find(|(k, _)| *k == key) {
        Some(entry) => entry.1 = value,
        None => map.push((key, value)),
    }
}

/// (parent, textures JSON) of one MODELS row, by id.
fn lookup(model_id: &str) -> Option<(Option<&'static str>, Option<&'static str>)> {
    MODELS
        .iter()
        .find(|(id, ..)| *id == model_id)
        .map(|(_, parent, textures, ..)| (*parent, *textures))
}

/// The strict textures-JSON parser: `{"slot": value, ...}` where value is
/// a plain string (Value, or Ref when it starts with '#') or the 26.2
/// sprite dict {"force_translucent": bool, "sprite": string} (a Value).
/// Escapes refuse — the 6607 real entries are all plain paths, and the
/// registry-wide test parses every one, so a surprise fails loudly.
fn parse_textures(json: &'static str) -> Vec<(&'static str, SlotValue)> {
    let b = json.as_bytes();
    let mut i = 0usize;
    let mut out = Vec::new();
    let mut expect_key = true;
    assert_eq!(b.first(), Some(&b'{'), "textures JSON must open with a brace");
    i += 1;
    loop {
        assert!(i < b.len(), "textures JSON truncated");
        match b[i] {
            b'}' => break,
            b',' => {
                assert!(!expect_key, "textures JSON has a stray comma");
                expect_key = true;
                i += 1;
            }
            b'"' if expect_key => {
                let key = scan_string(json, &mut i);
                assert_eq!(b[i], b':', "textures JSON key must be followed by ':'");
                i += 1;
                let value = parse_value(json, &mut i);
                out.push((key, value));
                expect_key = false;
            }
            other => panic!("textures JSON: unexpected byte {other:#x} at {i}"),
        }
    }
    assert!(i + 1 == b.len(), "textures JSON must end at the closing brace");
    out
}

fn parse_value(json: &'static str, i: &mut usize) -> SlotValue {
    let b = json.as_bytes();
    assert!(*i < b.len(), "textures JSON truncated");
    match b[*i] {
        b'"' => {
            let s = scan_string(json, i);
            if let Some(target) = s.strip_prefix('#') {
                SlotValue::Ref(target)
            } else {
                SlotValue::Value(s)
            }
        }
        b'{' => {
            // {"force_translucent": true, "sprite": "..."} — the 26.2
            // inline sprite definition; a Value over the sprite field.
            *i += 1;
            let mut sprite = None;
            let mut seen_keys = 0usize;
            loop {
                assert!(*i < b.len(), "sprite dict truncated");
                match b[*i] {
                    b'}' => {
                        *i += 1;
                        break;
                    }
                    b',' => *i += 1,
                    b'"' => {
                        let key = scan_string(json, i);
                        assert_eq!(b[*i], b':');
                        *i += 1;
                        seen_keys += 1;
                        if key == "sprite" {
                            assert_eq!(b[*i], b'"', "sprite must be a string");
                            sprite = Some(scan_string(json, i));
                        } else {
                            assert_eq!(key, "force_translucent", "unknown sprite-dict key");
                            assert!(b[*i..].starts_with(b"true"), "force_translucent must be true");
                            *i += 4;
                        }
                    }
                    other => panic!("sprite dict: unexpected byte {other:#x} at {}", *i),
                }
            }
            assert_eq!(seen_keys, 2, "the sprite dict carries exactly two keys");
            SlotValue::Value(sprite.expect("the sprite dict carries a sprite"))
        }
        other => panic!("textures JSON: unexpected value byte {other:#x} at {}", *i),
    }
}

/// Scans a plain JSON string (no escapes — fail-closed on any backslash),
/// returning the &'static str slice into the MODELS table.
fn scan_string(json: &'static str, i: &mut usize) -> &'static str {
    let b = json.as_bytes();
    assert_eq!(b[*i], b'"');
    *i += 1;
    let start = *i;
    while b[*i] != b'"' {
        assert_ne!(b[*i], b'\\', "textures JSON strings carry no escapes in the real data");
        assert!(b[*i] >= 0x20, "raw control byte in a textures JSON string");
        *i += 1;
        assert!(*i < b.len(), "unterminated string");
    }
    let s = &json[start..*i];
    *i += 1;
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(map: &[(&'static str, &'static str)]) -> Vec<(&'static str, &'static str)> {
        let mut v = map.to_vec();
        v.sort_unstable();
        v
    }

    /// The button template dangles by design (particle -> #texture, no
    /// texture anywhere up its chain — the vanilla warn+drop), while the
    /// concrete child's Value fills BOTH slots in one fixpoint pass.
    #[test]
    fn resolves_the_button_template_and_fills_the_slot_in_the_child() {
        let template = resolve_model("block/button").expect("a model row");
        assert!(template.resolved.is_empty());
        assert_eq!(template.unresolved, [("particle", "texture")]);
        // the vanilla do-while runs its futile scan once even here
        assert_eq!(template.fixpoint_passes, 1);

        let child = resolve_model("block/acacia_button").expect("a model row");
        assert_eq!(sorted(&child.resolved), sorted(&[
            ("texture", "minecraft:block/acacia_planks"),
            ("particle", "minecraft:block/acacia_planks"),
        ]));
        assert!(child.unresolved.is_empty());
        assert_eq!(child.fixpoint_passes, 1);
        assert_eq!(child.material("#particle"), Some("minecraft:block/acacia_planks"));
    }

    /// The acacia shelf: the child's Value overrides the template's
    /// "#particle" reference (the census's OVERRIDE_SAMPLES pin), and the
    /// resolved map is exactly the census's.
    #[test]
    fn merges_the_chain_with_the_child_override() {
        let slots = resolve_model("block/acacia_shelf").expect("a model row");
        assert_eq!(sorted(&slots.resolved), sorted(&[
            ("all", "minecraft:block/acacia_shelf"),
            ("particle", "minecraft:block/stripped_acacia_log"),
        ]));
        assert!(slots.unresolved.is_empty());
        assert_eq!(slots.overrides, 1);
        assert_eq!(slots.chain_depth, 2);
        assert_eq!(slots.fixpoint_passes, 0);
    }

    /// The furnace family: the deepest census fixpoint (2 passes) — the
    /// inherited refs walk bottom -> particle -> front through the
    /// template chain.
    #[test]
    fn resolves_the_var_chains_to_the_fixpoint() {
        let slots = resolve_model("block/blast_furnace").expect("a model row");
        assert_eq!(slots.fixpoint_passes, 2);
        assert!(slots.unresolved.is_empty());
        assert_eq!(slots.material("#down"), Some("minecraft:block/blast_furnace_top"));
        assert_eq!(slots.material("#north"), Some("minecraft:block/blast_furnace_front"));
        assert_eq!(slots.material("#east"), Some("minecraft:block/blast_furnace_side"));
    }

    /// heavy_core: the value is a BARE path (no namespace — as written)
    /// and the faces reference it BARE — both getMaterial forms land on
    /// the same id (the TextureSlots.getMaterial bytecode: strip '#',
    /// look up).
    #[test]
    fn resolves_bare_refs_and_hash_refs_identically() {
        let slots = resolve_model("block/heavy_core").expect("a model row");
        assert_eq!(sorted(&slots.resolved), sorted(&[
            ("all", "block/heavy_core"),
            ("particle", "block/heavy_core"),
        ]));
        assert_eq!(slots.material("#all"), Some("block/heavy_core"));
        assert_eq!(slots.material("all"), Some("block/heavy_core"));
        assert_eq!(resolve_ref("block/heavy_core", "#all"), Some("block/heavy_core"));
        assert_eq!(resolve_ref("block/heavy_core", "all"), Some("block/heavy_core"));
    }

    /// The 26.2 inline sprite dict: {"force_translucent": true,
    /// "sprite": ...} is a Value over the sprite field, and the child's
    /// "all" fills cube_all's seven inherited refs in one pass.
    #[test]
    fn keeps_the_sprite_dict_values() {
        let slots = resolve_model("block/black_stained_glass").expect("a model row");
        let all_glass = "minecraft:block/black_stained_glass";
        assert_eq!(sorted(&slots.resolved), sorted(&[
            ("all", all_glass),
            ("particle", all_glass),
            ("down", all_glass),
            ("up", all_glass),
            ("north", all_glass),
            ("east", all_glass),
            ("south", all_glass),
            ("west", all_glass),
        ]));
        assert_eq!(slots.fixpoint_passes, 1);
        assert_eq!(slots.material("#north"), Some("minecraft:block/black_stained_glass"));
    }

    /// resolve_model reaches every model id and misses everything else.
    #[test]
    fn looks_up_models_by_id_and_misses_unknowns() {
        assert!(resolve_model("block/cube_all").is_some());
        assert!(resolve_model("block/cube_inner").is_none());
        assert!(resolve_model("").is_none());
        assert!(resolve_ref("nope/none", "#all").is_none());
    }

    /// The registry-wide census pin (scripts/texture_slots_census.py):
    /// every MODELS row's textures JSON parses (fail-closed strings), the
    /// chain/override/fixpoint histograms hold, and the face-ref join
    /// over the 344 element models resolves exactly 2623 of the 4238
    /// face refs — the 1615 dangling are the template models whose
    /// children fill the slots (vanilla's warn+drop ledger).
    #[test]
    fn the_registry_wide_census_pins_hold() {
        let mut with_textures = 0usize;
        let mut resolved_total = 0usize;
        let mut unresolved_total = 0usize;
        let mut depth_hist = [0usize; 5];
        let mut override_hist = [0usize; 9];
        let mut passes_hist = [0usize; 3];
        let mut values_namespaced = 0usize;
        let mut values_bare = 0usize;
        for (id, parent, textures, ..) in MODELS.iter() {
            let slots = resolve_model(id).expect("every MODELS row resolves");
            assert_eq!(slots.chain_depth, {
                let mut d = 0;
                let mut cur = *id;
                loop {
                    let (Some(p), _) = lookup(cur).unwrap() else { break };
                    match p {
                        "builtin/entity" | "builtin/generated" => break,
                        _ => {
                            cur = p;
                            d += 1;
                            assert!(d <= 32, "cycle");
                        }
                    }
                }
                d
            });
            depth_hist[slots.chain_depth.min(4)] += 1;
            override_hist[slots.overrides.min(8)] += 1;
            passes_hist[slots.fixpoint_passes.min(2)] += 1;
            resolved_total += slots.resolved.len();
            unresolved_total += slots.unresolved.len();
            if textures.is_some() {
                with_textures += 1;
            }
            for (_, v) in &slots.resolved {
                if v.contains(':') {
                    values_namespaced += 1;
                } else {
                    values_bare += 1;
                }
            }
            let _ = parent;
        }
        assert_eq!(with_textures, 3883);
        assert_eq!(resolved_total, 11094);
        assert_eq!(unresolved_total, 258);
        assert_eq!(depth_hist, [302, 2256, 874, 480, 16]);
        assert_eq!(override_hist, [3740, 110, 0, 16, 51, 9, 0, 0, 2]);
        assert_eq!(passes_hist, [2063, 1857, 8]);
        assert_eq!(values_namespaced, 10287);
        assert_eq!(values_bare, 807);

        // the face-ref join over the element models
        use crate::generated::elements::ELEMENTS;
        let mut resolved_refs = 0usize;
        let mut missing_refs = 0usize;
        let mut bare_all = 0usize;
        for (id, elements) in ELEMENTS {
            let slots = resolve_model(id).expect("an element model row");
            for element in elements.iter() {
                for face in element.faces.iter().flatten() {
                    if !face.texture.starts_with('#') {
                        bare_all += 1;
                    }
                    if slots.material(face.texture).is_some() {
                        resolved_refs += 1;
                    } else {
                        missing_refs += 1;
                    }
                }
            }
        }
        assert_eq!(bare_all, 6, "heavy_core's six bare refs");
        assert_eq!(resolved_refs, 2623);
        assert_eq!(missing_refs, 1615);
        assert_eq!(resolved_refs + missing_refs, 4238);
    }
}
