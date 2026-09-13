//! The hand-written consumer half of generated::blockstates::BLOCKSTATES:
//! a strict zero-dependency JSON-subset parser for the two canonical
//! columns plus the vanilla selection semantics — the state-id ->
//! model+rotation link the mesher's palette will consume (the
//! when-condition homework named by the blockstates round).
//!
//! Written against a fresh independent census of the real jar
//! (scripts/blockstate_when_census.py, autopush-69): variant keys are
//! always the block's FULL property set in BLOCKS order (7296 keys over
//! 1096 files, 549 empty, zero pipes), when values are strings only
//! (1874 plain + 32 pipe-carriers — exactly 4 distinct "a|b" sets, max
//! 3), the OR/AND wrappers never nest (13 OR / 768 AND map elements),
//! the closed apply key set is {model, x, y, uvlock, weight} (weight
//! only on the six chorus_plant applies, always 2), and no when map is
//! empty. The parser accepts the semantically-defined supersets the
//! census merely didn't refute (an empty when map = always-true,
//! recursive wrappers) and refuses everything MEANINGLESS — unknown
//! keys, non-string when values, escapes, floats, duplicates, empty
//! apply lists — the extend-parser-first posture at the consumer layer.
//!
//! Selection semantics (vanilla): variants pick the FIRST key (file
//! order) whose property pairs all match the state — on real data every
//! non-empty key is a full property set, so this equals exact-key
//! lookup (both pinned by tests); multipart collects EVERY part whose
//! when matches, in part order. A when value "a|b" matches if the
//! state's value is any of the pipe alternatives; a property absent
//! from the state fails the condition (fail-closed).

/// One apply entry: the model ref exactly as written (namespaced, or one
/// of the 4 vanilla bare refs) with its rotations and random weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apply<'a> {
    pub model: &'a str,
    pub x: u32,
    pub y: u32,
    pub uvlock: bool,
    pub weight: Option<u16>,
}

/// A multipart when selector. [`When::Map`] is the plain AND of
/// (property -> value) pairs; a value may carry "a|b" or-lists.
/// [`When::Or`] / [`When::And`] are the two wrapper forms — the parser
/// recurses, so a future nesting parses, though 26.2 ships none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum When<'a> {
    Map(Vec<(&'a str, &'a str)>),
    Or(Vec<When<'a>>),
    And(Vec<When<'a>>),
}

/// One multipart part: the condition (None = always) and its applies
/// (the apply object as a singleton, or the apply list in jar order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part<'a> {
    pub when: Option<When<'a>>,
    pub apply: Vec<Apply<'a>>,
}

/// The parsed definition of one blockstate row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition<'a> {
    Variants(Vec<(&'a str, Vec<Apply<'a>>)>),
    Multipart(Vec<Part<'a>>),
}

/// Which canonical column a blockstate row carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Variants,
    Multipart,
}

/// The raw JSON column for a block id, straight from the generated table.
pub fn column(block_id: &str) -> Option<(Shape, &'static str)> {
    crate::generated::blockstates::BLOCKSTATES
        .iter()
        .find(|r| r.0 == block_id)
        .map(|r| match (r.1, r.2) {
            (Some(v), _) => (Shape::Variants, v),
            (_, Some(m)) => (Shape::Multipart, m),
            _ => unreachable!("the table's XOR invariant"),
        })
}

/// Parse the definition for a block id from the generated table.
pub fn parse(block_id: &str) -> Option<Result<Definition<'static>, String>> {
    column(block_id).map(|(shape, json)| parse_definition(json, shape))
}

/// Parse one canonical column with the strict typed layer.
pub fn parse_definition<'a>(json: &'a str, shape: Shape) -> Result<Definition<'a>, String> {
    let mut p = Parser::new(json);
    let v = p.value()?;
    p.end()?;
    match shape {
        Shape::Variants => variants_from(v),
        Shape::Multipart => multipart_from(v),
    }
}

/// The vanilla variant selection: the FIRST key (file order) whose
/// property pairs all match the state; the key's apply list (the
/// weighted/equal random candidates) comes back. The empty key ""
/// matches every state.
pub fn select_variant<'a>(
    variants: &'a [(&'a str, Vec<Apply<'a>>)],
    state: &[(&str, &str)],
) -> Option<&'a [Apply<'a>]> {
    variants
        .iter()
        .find(|(key, _)| key_matches(key, state))
        .map(|(_, applies)| applies.as_slice())
}

fn key_matches(key: &str, state: &[(&str, &str)]) -> bool {
    if key.is_empty() {
        return true;
    }
    key.split(',').all(|pair| match pair.split_once('=') {
        Some((p, v)) => state.iter().any(|(sp, sv)| *sp == p && *sv == v),
        None => false,
    })
}

/// The vanilla multipart selection: EVERY part whose when matches (or
/// that carries no when) contributes its applies, in part order.
pub fn select_parts<'a>(parts: &'a [Part<'a>], state: &[(&str, &str)]) -> Vec<&'a Apply<'a>> {
    parts
        .iter()
        .filter(|p| p.when.as_ref().map_or(true, |w| when_matches(w, state)))
        .flat_map(|p| p.apply.iter())
        .collect()
}

/// Does a when selector match the state? A Map pair matches iff the
/// state carries the property with one of the pipe alternatives; an
/// absent property fails the pair (fail-closed).
pub fn when_matches(when: &When, state: &[(&str, &str)]) -> bool {
    match when {
        When::Map(pairs) => pairs.iter().all(|(p, allowed)| {
            state
                .iter()
                .any(|(sp, sv)| sp == p && allowed.split('|').any(|a| a == *sv))
        }),
        When::Or(ws) => ws.iter().any(|w| when_matches(w, state)),
        When::And(ws) => ws.iter().all(|w| when_matches(w, state)),
    }
}

// ---------- the strict JSON-subset parser ----------

enum Val<'a> {
    Str(&'a str),
    Bool(bool),
    Num(u32),
    Obj(Vec<(&'a str, Val<'a>)>),
    Arr(Vec<Val<'a>>),
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Parser { b: s.as_bytes(), i: 0 }
    }

    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }

    fn end(&mut self) -> Result<(), String> {
        self.ws();
        if self.i == self.b.len() {
            Ok(())
        } else {
            Err(format!("trailing garbage at byte {}", self.i))
        }
    }

    fn expect(&mut self, c: u8) -> Result<(), String> {
        self.ws();
        if self.peek() == Some(c) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected '{}' at byte {}", c as char, self.i))
        }
    }

    fn value(&mut self) -> Result<Val<'a>, String> {
        self.ws();
        match self.peek() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Val::Str),
            Some(b't') => self.literal("true").map(|_| Val::Bool(true)),
            Some(b'f') => self.literal("false").map(|_| Val::Bool(false)),
            Some(c) if c.is_ascii_digit() => self.uint().map(Val::Num),
            Some(c) => Err(format!("unexpected byte '{}' at {}", c as char, self.i)),
            None => Err("unexpected end of input".to_string()),
        }
    }

    fn literal(&mut self, lit: &str) -> Result<(), String> {
        if self.b[self.i..].starts_with(lit.as_bytes()) {
            self.i += lit.len();
            Ok(())
        } else {
            Err(format!("bad literal at byte {}", self.i))
        }
    }

    /// A JSON string as a borrowed slice — escapes and raw control
    /// bytes refuse (the canonical columns carry none; the charset
    /// census proved it).
    fn string(&mut self) -> Result<&'a str, String> {
        self.expect(b'"')?;
        let start = self.i;
        while let Some(c) = self.peek() {
            match c {
                b'"' => {
                    let s = &self.b[start..self.i];
                    self.i += 1;
                    return std::str::from_utf8(s).map_err(|e| format!("bad utf-8: {e}"));
                }
                b'\\' => return Err(format!("string escapes refuse at byte {}", self.i)),
                c if c < 0x20 => {
                    return Err(format!("raw control byte {c:#04x} at byte {}", self.i))
                }
                _ => self.i += 1,
            }
        }
        Err("unterminated string".to_string())
    }

    /// Non-negative integers only — the columns' closed numeric domain
    /// (rotations 0/90/180/270, weight u16). No sign, no fraction.
    fn uint(&mut self) -> Result<u32, String> {
        let start = self.i;
        let mut n: u32 = 0;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                n = n
                    .checked_mul(10)
                    .and_then(|x| x.checked_add((c - b'0') as u32))
                    .ok_or_else(|| format!("number overflow at byte {}", self.i))?;
                self.i += 1;
            } else {
                break;
            }
        }
        if self.i == start {
            return Err(format!("expected a number at byte {}", self.i));
        }
        Ok(n)
    }

    fn object(&mut self) -> Result<Val<'a>, String> {
        self.expect(b'{')?;
        let mut out: Vec<(&'a str, Val<'a>)> = Vec::new();
        self.ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(Val::Obj(out));
        }
        loop {
            let k = self.string()?;
            self.expect(b':')?;
            let v = self.value()?;
            if out.iter().any(|(ek, _)| *ek == k) {
                return Err(format!("duplicate object key {k:?} at byte {}", self.i));
            }
            out.push((k, v));
            self.ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Val::Obj(out));
                }
                _ => return Err(format!("expected ',' or '}}' at byte {}", self.i)),
            }
        }
    }

    fn array(&mut self) -> Result<Val<'a>, String> {
        self.expect(b'[')?;
        let mut out = Vec::new();
        self.ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(Val::Arr(out));
        }
        loop {
            out.push(self.value()?);
            self.ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Val::Arr(out));
                }
                _ => return Err(format!("expected ',' or ']' at byte {}", self.i)),
            }
        }
    }
}

// ---------- the typed layer over the parsed values ----------

/// The census-verified rotation domain: 0/90/180/270 only.
fn rotation(n: u32) -> Result<u32, String> {
    if n <= 270 && n % 90 == 0 {
        Ok(n)
    } else {
        Err(format!("rotation {n} outside the census domain 0/90/180/270"))
    }
}

fn apply_from(v: Val) -> Result<Apply, String> {
    let Val::Obj(fields) = v else {
        return Err("an apply must be an object".to_string());
    };
    let mut a = Apply { model: "", x: 0, y: 0, uvlock: false, weight: None };
    for (k, v) in fields {
        match k {
            "model" => match v {
                Val::Str(s) => a.model = s,
                _ => return Err("model must be a string".to_string()),
            },
            "x" => a.x = rotation(match v {
                Val::Num(n) => n,
                _ => return Err("x must be a number".to_string()),
            })?,
            "y" => a.y = rotation(match v {
                Val::Num(n) => n,
                _ => return Err("y must be a number".to_string()),
            })?,
            "uvlock" => match v {
                Val::Bool(b) => a.uvlock = b,
                _ => return Err("uvlock must be a boolean".to_string()),
            },
            "weight" => match v {
                Val::Num(n) => a.weight = Some(u16::try_from(n).map_err(|_| "weight overflows u16".to_string())?),
                _ => return Err("weight must be a number".to_string()),
            },
            other => {
                return Err(format!(
                    "unknown apply key {other:?} (the closed set is model/x/y/uvlock/weight)"
                ))
            }
        }
    }
    if a.model.is_empty() {
        return Err("an apply requires its model".to_string());
    }
    Ok(a)
}

fn when_from(v: Val) -> Result<When, String> {
    let Val::Obj(fields) = v else {
        return Err("a when must be an object".to_string());
    };
    // the wrappers are ALWAYS sole-key arrays (the census: OR x5 / AND
    // x288 files, never mixed with plain keys, never a non-array value)
    let wrapper_kind = fields
        .iter()
        .find_map(|(k, _)| (*k == "OR" || *k == "AND").then_some(*k));
    if let Some(kind) = wrapper_kind {
        if fields.len() != 1 {
            return Err(format!(
                "the {kind} wrapper refuses to share its object with other keys"
            ));
        }
        let Val::Arr(els) = fields.into_iter().next().unwrap().1 else {
            return Err(format!("the {kind} wrapper must be an array"));
        };
        let ws = els
            .into_iter()
            .map(when_from)
            .collect::<Result<Vec<_>, String>>()?;
        return Ok(if kind == "OR" { When::Or(ws) } else { When::And(ws) });
    }
    let mut pairs = Vec::with_capacity(fields.len());
    for (k, v) in fields {
        let Val::Str(s) = v else {
            return Err(format!(
                "when value for {k:?} must be a string (the census: no arrays, no bools)"
            ));
        };
        pairs.push((k, s));
    }
    Ok(When::Map(pairs))
}

fn part_from(v: Val) -> Result<Part, String> {
    let Val::Obj(fields) = v else {
        return Err("a multipart part must be an object".to_string());
    };
    let mut when: Option<When> = None;
    let mut apply: Option<Vec<Apply>> = None;
    for (k, v) in fields {
        match k {
            "apply" => {
                let els = match v {
                    Val::Obj(o) => vec![Val::Obj(o)],
                    Val::Arr(a) => a,
                    _ => return Err("apply must be an object or an array".to_string()),
                };
                if els.is_empty() {
                    return Err("an apply list refuses to be empty".to_string());
                }
                apply = Some(
                    els.into_iter()
                        .map(apply_from)
                        .collect::<Result<Vec<_>, String>>()?,
                );
            }
            "when" => when = Some(when_from(v)?),
            other => return Err(format!("unknown part key {other:?} (the closed set is apply/when)")),
        }
    }
    Ok(Part { when, apply: apply.ok_or("a multipart part requires its apply")? })
}

fn variants_from(v: Val) -> Result<Definition, String> {
    let Val::Obj(fields) = v else {
        return Err("the variants column must be an object".to_string());
    };
    let mut out = Vec::with_capacity(fields.len());
    for (k, v) in fields {
        let els = match v {
            Val::Obj(o) => vec![Val::Obj(o)],
            Val::Arr(a) => {
                if a.is_empty() {
                    return Err(format!("variant {k:?} refuses an empty apply array"));
                }
                a
            }
            _ => return Err("a variant value must be an object or an array".to_string()),
        };
        let applies = els
            .into_iter()
            .map(apply_from)
            .collect::<Result<Vec<_>, String>>()?;
        out.push((k, applies));
    }
    Ok(Definition::Variants(out))
}

fn multipart_from(v: Val) -> Result<Definition, String> {
    let Val::Arr(els) = v else {
        return Err("the multipart column must be an array".to_string());
    };
    if els.is_empty() {
        return Err("the multipart column refuses to be empty".to_string());
    }
    let parts = els
        .into_iter()
        .map(part_from)
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Definition::Multipart(parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::blockstates::BLOCKSTATES;
    use crate::generated::blocks::BLOCKS;

    fn props_of(id: &str) -> &'static [&'static str] {
        BLOCKS
            .iter()
            .find(|b| b.name == id)
            .unwrap_or_else(|| panic!("{id} is no block"))
            .properties
    }

    #[test]
    fn the_columns_split_and_every_blockstate_row_looks_up() {
        let (mut variants, mut multipart) = (0usize, 0usize);
        for (id, v, mp) in BLOCKSTATES {
            let (shape, json) = column(id).unwrap_or_else(|| panic!("{id} does not look up"));
            assert!(!json.is_empty());
            match shape {
                Shape::Variants => {
                    variants += 1;
                    assert!(v.is_some() && mp.is_none(), "{id}");
                }
                Shape::Multipart => {
                    multipart += 1;
                    assert!(mp.is_some() && v.is_none(), "{id}");
                }
            }
        }
        assert_eq!((variants, multipart), (1096, 102));
        assert_eq!(column("acacia_button").unwrap().0, Shape::Variants);
        assert_eq!(column("acacia_fence").unwrap().0, Shape::Multipart);
        assert!(column("not_a_block").is_none());
    }

    #[test]
    fn selects_variants_first_match_with_rotations_and_arrays() {
        // acacia_button: the wall/east/false row carries uvlock + x90 + y90
        let Some(Ok(Definition::Variants(vs))) = parse("acacia_button") else { panic!() };
        let a = select_variant(
            &vs,
            &[("face", "wall"), ("facing", "east"), ("powered", "false")],
        )
        .unwrap();
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].model, "minecraft:block/acacia_button");
        assert_eq!((a[0].x, a[0].y, a[0].uvlock, a[0].weight), (90, 90, true, None));
        // the pressed neighbor one property flip away
        let b = select_variant(
            &vs,
            &[("face", "wall"), ("facing", "east"), ("powered", "true")],
        )
        .unwrap();
        assert_eq!(b[0].model, "minecraft:block/acacia_button_pressed");
        assert_eq!((b[0].x, b[0].y, b[0].uvlock), (90, 90, true));

        // the empty key matches ANY state (549 property-less files)
        let Some(Ok(Definition::Variants(pl))) = parse("acacia_planks") else { panic!() };
        assert_eq!(pl.len(), 1);
        assert!(pl[0].0.is_empty());
        assert_eq!(
            select_variant(&pl, &[("anything", "goes")]).unwrap()[0].model,
            "minecraft:block/acacia_planks"
        );

        // the apply ARRAY: grass_block snowy=false is the four equal
        // y-rotations in jar order (the random candidates)
        let Some(Ok(Definition::Variants(g))) = parse("grass_block") else { panic!() };
        let ga = select_variant(&g, &[("snowy", "false")]).unwrap();
        assert_eq!(ga.len(), 4);
        assert_eq!(
            ga.iter().map(|a| a.y).collect::<Vec<_>>(),
            vec![0, 90, 180, 270]
        );
        assert!(ga.iter().all(|a| a.model == "minecraft:block/grass_block"
            && !a.uvlock
            && a.weight.is_none()));
        // snowy=true is a plain singleton
        assert_eq!(select_variant(&g, &[("snowy", "true")]).unwrap()[0].model,
            "minecraft:block/grass_block_snow");
        // unmentioned properties never block: an extra state property is
        // a wildcard for the key matcher (grass_block has no empty key,
        // so the empty state picks nothing at all)
        assert!(select_variant(&g, &[]).is_none());
        assert_eq!(
            select_variant(&g, &[("snowy", "false"), ("extra", "x")]).unwrap().len(),
            4
        );
    }

    #[test]
    fn selects_multipart_parts_with_or_and_wrappers_and_pipe_values() {
        // acacia_fence: the always-post + the north side
        let Some(Ok(Definition::Multipart(f))) = parse("acacia_fence") else { panic!() };
        assert_eq!(f.len(), 5);
        let none = select_parts(
            &f,
            &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false")],
        );
        assert_eq!(none.len(), 1);
        assert_eq!(none[0].model, "minecraft:block/acacia_fence_post");
        let north = select_parts(
            &f,
            &[("north", "true"), ("east", "false"), ("south", "false"), ("west", "false")],
        );
        assert_eq!(north.len(), 2);
        assert_eq!(north[1].model, "minecraft:block/acacia_fence_side");
        assert_eq!((north[1].y, north[1].uvlock), (0, true));

        // redstone_wire: the OR wrapper's five maps + the "side|up" pipes.
        // all-none picks only the dot (via the OR's first map)...
        let Some(Ok(Definition::Multipart(w))) = parse("redstone_wire") else { panic!() };
        assert_eq!(w.len(), 9);
        let dot = select_parts(
            &w,
            &[("east", "none"), ("north", "none"), ("south", "none"), ("west", "none"), ("power", "0")],
        );
        assert_eq!(dot.len(), 1);
        assert_eq!(dot[0].model, "minecraft:block/redstone_dust_dot");
        // east=side picks exactly the east side piece (via the "side|up" pipe)
        let side = select_parts(
            &w,
            &[("east", "side"), ("north", "none"), ("south", "none"), ("west", "none"), ("power", "0")],
        );
        assert_eq!(side.len(), 1);
        assert_eq!(side[0].model, "minecraft:block/redstone_dust_side_alt1");
        assert_eq!(side[0].y, 270);
        // east=up + north=side: the dot (OR map 2), the north side0, the
        // east side_alt1, and the east up piece — four parts stack
        let up_side = select_parts(
            &w,
            &[("east", "up"), ("north", "side"), ("south", "none"), ("west", "none"), ("power", "0")],
        );
        let mut models: Vec<&str> = up_side.iter().map(|a| a.model).collect();
        models.sort_unstable();
        assert_eq!(models, vec![
            "minecraft:block/redstone_dust_dot",
            "minecraft:block/redstone_dust_side0",
            "minecraft:block/redstone_dust_side_alt1",
            "minecraft:block/redstone_dust_up",
        ]);

        // leaf_litter: segment 3 matches BOTH the "2|3" pipe and the
        // exact "3" part — vanilla really stacks both models
        let Some(Ok(Definition::Multipart(l))) = parse("leaf_litter") else { panic!() };
        assert_eq!(l.len(), 16);
        let seg3 = select_parts(&l, &[("facing", "north"), ("segment_amount", "3")]);
        assert_eq!(
            seg3.iter().map(|a| a.model).collect::<Vec<_>>(),
            vec!["minecraft:block/leaf_litter_2", "minecraft:block/leaf_litter_3"]
        );

        // acacia_shelf: the AND wrappers chain facing+powered+side_chain
        let Some(Ok(Definition::Multipart(s))) = parse("acacia_shelf") else { panic!() };
        assert_eq!(s.len(), 24);
        let unpowered = select_parts(
            &s,
            &[("facing", "north"), ("powered", "false"), ("side_chain", "left")],
        );
        assert_eq!(
            unpowered.iter().map(|a| a.model).collect::<Vec<_>>(),
            vec![
                "minecraft:block/acacia_shelf",
                "minecraft:block/acacia_shelf_unpowered",
            ]
        );
        let left = select_parts(
            &s,
            &[("facing", "north"), ("powered", "true"), ("side_chain", "left")],
        );
        assert_eq!(
            left.iter().map(|a| a.model).collect::<Vec<_>>(),
            vec![
                "minecraft:block/acacia_shelf",
                "minecraft:block/acacia_shelf_left",
            ]
        );
    }

    #[test]
    fn the_registry_wide_census_pins_hold_and_every_key_selects_itself() {
        // counts walked straight off the parsed definitions — the python
        // census (scripts/blockstate_when_census.py) re-derived independently
        let (mut total_keys, mut empty_keys) = (0usize, 0usize);
        let (mut when_maps, mut or_elems, mut and_elems) = (0usize, 0usize, 0usize);
        let mut pipe_values = 0usize;
        let mut weighted = 0usize;
        fn walk<'a>(
            w: &'a When<'a>,
            when_maps: &mut usize,
            or_elems: &mut usize,
            and_elems: &mut usize,
            pipe_values: &mut usize,
            nested: &mut usize,
        ) -> Vec<&'a str> {
            let mut props = Vec::new();
            match w {
                When::Map(pairs) => {
                    *when_maps += 1;
                    for (p, v) in pairs {
                        props.push(*p);
                        if v.contains('|') {
                            *pipe_values += 1;
                        }
                    }
                }
                When::Or(ws) => {
                    *or_elems += ws.len();
                    for e in ws {
                        if !matches!(e, When::Map(_)) {
                            *nested += 1;
                        }
                        props.extend(walk(e, when_maps, or_elems, and_elems, pipe_values, nested));
                    }
                }
                When::And(ws) => {
                    *and_elems += ws.len();
                    for e in ws {
                        if !matches!(e, When::Map(_)) {
                            *nested += 1;
                        }
                        props.extend(walk(e, when_maps, or_elems, and_elems, pipe_values, nested));
                    }
                }
            }
            props
        }
        for (id, _, _) in BLOCKSTATES {
            let props = props_of(id);
            let def = parse(id).unwrap().unwrap_or_else(|e| panic!("{id}: {e}"));
            match def {
                Definition::Variants(vs) => {
                    for (key, applies) in &vs {
                        total_keys += 1;
                        if key.is_empty() {
                            empty_keys += 1;
                        } else {
                            let pairs: Vec<(&str, &str)> = key
                                .split(',')
                                .map(|p| p.split_once('=').unwrap())
                                .collect();
                            // the full property set, in BLOCKS order
                            assert_eq!(pairs.len(), props.len(), "{id}: key {key}");
                            for (i, (p, _)) in pairs.iter().enumerate() {
                                assert_eq!(*p, props[i], "{id}: key {key}");
                            }
                            // the key selects ITSELF (first-match == exact
                            // lookup on full-set keys)
                            let sel = select_variant(&vs, &pairs)
                                .unwrap_or_else(|| panic!("{id}: {key} selects nothing"));
                            assert_eq!(sel, applies.as_slice(), "{id}: {key}");
                        }
                        for a in applies {
                            if a.weight.is_some() {
                                weighted += 1;
                            }
                        }
                    }
                }
                Definition::Multipart(ps) => {
                    for p in &ps {
                        if let Some(w) = &p.when {
                            let mut nested = 0usize;
                            let wprops = walk(
                                w,
                                &mut when_maps,
                                &mut or_elems,
                                &mut and_elems,
                                &mut pipe_values,
                                &mut nested,
                            );
                            assert_eq!(nested, 0, "{id}: wrappers never nest");
                            for wp in wprops {
                                assert!(
                                    props.contains(&wp),
                                    "{id}: when property {wp} is not a block property"
                                );
                            }
                        }
                        for a in &p.apply {
                            if a.weight.is_some() {
                                weighted += 1;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!((total_keys, empty_keys), (7296, 549));
        assert_eq!((when_maps, or_elems, and_elems), (1574, 13, 768));
        assert_eq!(pipe_values, 32);
        // the only weighted applies: chorus_plant's six noside entries, all 2
        assert_eq!(weighted, 6);
        for a in select_parts(
            &match parse("chorus_plant").unwrap().unwrap() {
                Definition::Multipart(ps) => ps,
                _ => unreachable!(),
            },
            &[("north", "true"), ("east", "true"), ("south", "true"), ("west", "true"), ("up", "true"), ("down", "true")],
        ) {
            if let Some(w) = a.weight {
                assert_eq!(w, 2);
                assert!(a.model.starts_with("minecraft:block/chorus_plant_noside"));
            }
        }
    }

    #[test]
    fn the_parser_refuses_shapes_outside_the_census() {
        let refuses = |json: &str, shape: Shape| {
            assert!(
                parse_definition(json, shape).is_err(),
                "expected refusal: {json}"
            );
        };
        // --- the variants column ---
        refuses("{\"\":{\"model\":\"a\"},\"\":{\"model\":\"b\"}}", Shape::Variants); // dup key
        refuses("{\"\":{\"modell\":\"a\"}}", Shape::Variants); // unknown apply key
        refuses("{\"\":{\"x\":45}}", Shape::Variants); // off the 90-degree domain
        refuses("{\"\":{\"x\":-90}}", Shape::Variants); // signed
        refuses("{\"\":{\"y\":360}}", Shape::Variants); // over 270
        refuses("{\"\":{\"model\":1}}", Shape::Variants); // non-string model
        refuses("{\"\":{}}", Shape::Variants); // model required
        refuses("{\"\":[]}", Shape::Variants); // empty apply array
        refuses("{\"\":{\"model\":\"a\",\"weight\":65536}}", Shape::Variants); // weight u16 overflow
        refuses("{\"\":{\"model\":\"a\",\"weight\":1.5}}", Shape::Variants); // fraction
        refuses("[]", Shape::Variants); // wrong top shape
        refuses("null", Shape::Variants); // null
        refuses("{\"\":{\"model\":\"a\"}} trailing", Shape::Variants); // trailing garbage
        refuses("{\"a\\tb\":{\"model\":\"a\"}}", Shape::Variants); // escapes refuse
        refuses("{\"\":{\"model\":\"a\",\"uvlock\":\"yes\"}}", Shape::Variants); // non-bool uvlock
        // a fully-formed apply with every closed key parses
        assert_eq!(
            parse_definition(
                "{\"\":{\"model\":\"a\",\"x\":180,\"y\":270,\"uvlock\":true,\"weight\":2}}",
                Shape::Variants,
            )
            .unwrap(),
            Definition::Variants(vec![("", vec![Apply {
                model: "a", x: 180, y: 270, uvlock: true, weight: Some(2),
            }])])
        );
        // --- the multipart column ---
        refuses("{}", Shape::Multipart); // wrong top shape
        refuses("[]", Shape::Multipart); // empty multipart
        refuses("[{\"apply\":{\"model\":\"a\"},\"when\":{\"p\":true}}]", Shape::Multipart); // bool when value
        refuses("[{\"apply\":{\"model\":\"a\"},\"when\":{\"p\":[\"a\"]}}]", Shape::Multipart); // list when value
        refuses("[{\"apply\":{\"model\":\"a\"},\"when\":{\"OR\":\"x\"}}]", Shape::Multipart); // wrapper non-array
        refuses("[{\"apply\":{\"model\":\"a\"},\"when\":{\"OR\":{}}}]", Shape::Multipart); // wrapper object
        refuses("[{\"apply\":{\"model\":\"a\"},\"when\":{\"p\":\"a\",\"q\":\"b\",\"OR\":[]}}]", Shape::Multipart); // mixed map+wrapper
        refuses("[{\"apply\":{\"model\":\"a\"},\"extra\":1}]", Shape::Multipart); // unknown part key
        refuses("[{\"when\":{\"p\":\"a\"}}]", Shape::Multipart); // apply required
        refuses("[{\"apply\":\"x\"}]", Shape::Multipart); // apply non-object
        refuses("[null]", Shape::Multipart); // null part
        refuses("[{\"apply\":[{\"model\":\"a\"},{}]}]", Shape::Multipart); // empty apply element
        // nested wrappers parse (the future-proofing the parser allows)
        assert!(parse_definition(
            "[{\"apply\":{\"model\":\"a\"},\"when\":{\"OR\":[{\"AND\":[{\"p\":\"1|2\"}]}]}}]",
            Shape::Multipart,
        )
        .is_ok());
    }
}
