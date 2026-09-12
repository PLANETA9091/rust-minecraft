//! Client jar analysis: obfuscation detection + extraction of functional
//! registry metadata (blockstates, items) — NO Mojang code is read into the
//! repository, only names and structural data.

use std::io::{Cursor, Read};

use anyhow::{Context, Result};
use serde_json::Value;
use zip::ZipArchive;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockGen {
    pub name: String,
    pub properties: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct JarReport {
    pub obfuscated: bool,
    pub obfuscation_ratio: f64,
    pub class_count: usize,
    pub net_minecraft_classes: usize,
    pub blockstate_count: usize,
    pub item_count: usize,
    pub texture_count: usize,
    pub sound_count: usize,
    pub has_lang_en_us: bool,
    /// Protocol version from the jar's embedded version.json (26.2: 776);
    /// None when the version ships no embedded manifest.
    pub protocol_version: Option<i32>,
    /// World/data format version from version.json (26.2: 4903).
    pub world_version: Option<i32>,
    pub blocks: Vec<BlockGen>,
    pub items: Vec<String>,
}

/// Open the jar bytes and classify everything the generator needs.
pub fn analyze(jar_bytes: &[u8]) -> Result<JarReport> {
    let cursor = Cursor::new(jar_bytes);
    let mut archive =
        ZipArchive::new(cursor).with_context(|| "open client jar as zip")?;

    let mut class_count = 0usize;
    let mut short_named = 0usize;
    let mut net_minecraft_classes = 0usize;
    let mut blockstate_paths: Vec<String> = Vec::new();
    let mut item_paths: Vec<String> = Vec::new();
    let mut texture_count = 0usize;
    let mut sound_count = 0usize;
    let mut has_lang_en_us = false;

    let names: Vec<String> = archive.file_names().map(String::from).collect();
    for name in &names {
        if let Some(rest) = name.strip_prefix("assets/minecraft/") {
            if let Some(bs) = rest.strip_prefix("blockstates/") {
                if bs.ends_with(".json") {
                    blockstate_paths.push(bs.trim_end_matches(".json").to_string());
                }
            } else if let Some(it) = rest.strip_prefix("items/") {
                if it.ends_with(".json") {
                    item_paths.push(it.trim_end_matches(".json").to_string()); // 1.21.4+ layout
                }
            } else if let Some(it) = rest.strip_prefix("models/item/") {
                if it.ends_with(".json") {
                    item_paths.push(it.trim_end_matches(".json").to_string()); // legacy layout
                }
            } else if rest.starts_with("textures/") && name.ends_with(".png") {
                texture_count += 1;
            } else if rest.starts_with("sounds/") {
                sound_count += 1;
            } else if rest == "lang/en_us.json" {
                has_lang_en_us = true;
            }
        }
        if name.ends_with(".class") && !name.starts_with("META-INF/") {
            class_count += 1;
            if name.starts_with("net/minecraft/") {
                net_minecraft_classes += 1;
            }
            let stem = name.trim_end_matches(".class");
            let base = stem.rsplit('/').next().unwrap_or(stem);
            if base.len() <= 2 && !base.contains('$') {
                short_named += 1;
            }
        }
    }

    let obfuscation_ratio = if class_count > 0 {
        short_named as f64 / class_count as f64
    } else {
        0.0
    };
    let obfuscated = obfuscation_ratio > 0.5 && net_minecraft_classes == 0;

    // The jar embeds its own manifest: the protocol number the LOGIN
    // handshake must speak (launchermeta version.json no longer carries it
    // in the 26.x era).
    let (protocol_version, world_version) = match archive.by_name("version.json") {
        Ok(mut f) => {
            let mut bytes = Vec::new();
            f.read_to_end(&mut bytes)?;
            let v: Value = serde_json::from_slice(&bytes)
                .with_context("parse embedded version.json")?;
            (
                v.get("protocol_version").and_then(Value::as_i64).map(|x| x as i32),
                v.get("world_version").and_then(Value::as_i64).map(|x| x as i32),
            )
        }
        Err(_) => (None, None),
    };

    // Parse blockstate JSON files for state properties.
    let mut blocks: Vec<BlockGen> = Vec::with_capacity(blockstate_paths.len());
    for path in &blockstate_paths {
        let mut file = archive
            .by_name(&format!("assets/minecraft/blockstates/{path}.json"))
            .with_context(|| format!("read blockstate {path}"))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let value: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("parse blockstate {path}"))?;
        let mut props = Vec::new();
        collect_state_properties(&value, &mut props);
        props.sort();
        props.dedup();
        blocks.push(BlockGen { name: path.clone(), properties: props });
    }
    blocks.sort_by(|a, b| a.name.cmp(&b.name));

    item_paths.sort();
    item_paths.dedup();

    Ok(JarReport {
        obfuscated,
        obfuscation_ratio,
        class_count,
        net_minecraft_classes,
        blockstate_count: blocks.len(),
        item_count: item_paths.len(),
        texture_count,
        sound_count,
        has_lang_en_us,
        protocol_version,
        world_version,
        blocks,
        items: item_paths,
    })
}

/// Pull property names out of a blockstate JSON (variants + multipart/when).
fn collect_state_properties(v: &Value, out: &mut Vec<String>) {
    if let Value::Object(root) = v {
        if let Some(Value::Object(variants)) = root.get("variants") {
            for key in variants.keys() {
                for part in key.split(',') {
                    let prop = part.split('=').next().unwrap_or("");
                    if !prop.is_empty() && !out.iter().any(|s| s == prop) {
                        out.push(prop.to_string());
                    }
                }
            }
        }
        if let Some(Value::Array(multipart)) = root.get("multipart") {
            for entry in multipart {
                if let Some(when) = entry.get("when") {
                    when_properties(when, out);
                }
            }
        }
    }
}

fn when_properties(w: &Value, out: &mut Vec<String>) {
    match w {
        Value::Object(map) => {
            for (k, v) in map {
                if k == "OR" || k == "AND" {
                    if let Value::Array(terms) = v {
                        for t in terms {
                            when_properties(t, out);
                        }
                    }
                } else if !out.iter().any(|s| s == k) {
                    out.push(k.clone());
                }
            }
        }
        Value::Array(terms) => {
            for t in terms {
                when_properties(t, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_properties() {
        let v: Value = serde_json::json!({
            "variants": {
                "facing=north,half=lower": {},
                "facing=south,half=upper": {}
            }
        });
        let mut out = Vec::new();
        collect_state_properties(&v, &mut out);
        out.sort();
        assert_eq!(out, vec!["facing", "half"]);
    }

    #[test]
    fn multipart_when_or_and() {
        let v: Value = serde_json::json!({
            "multipart": [
                { "when": { "OR": [ {"north": "true"}, {"south": "true"} ] }, "apply": {} },
                { "when": { "AND": [ {"waterlogged": "true"}, {"logged": "false"} ] }, "apply": {} }
            ]
        });
        let mut out = Vec::new();
        collect_state_properties(&v, &mut out);
        out.sort();
        assert_eq!(out, vec!["logged", "north", "south", "waterlogged"]);
    }
}
