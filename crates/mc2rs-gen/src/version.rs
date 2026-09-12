//! Per-version JSON (piston-meta): downloads, asset index, libraries.

use anyhow::Result;
use serde::Deserialize;

use crate::util::Cache;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionJson {
    pub id: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default)]
    pub release_time: String,
    #[serde(default)]
    pub assets: Option<String>,
    #[serde(default)]
    pub asset_index: Option<AssetIndexRef>,
    pub downloads: Downloads,
    #[serde(default)]
    pub java_version: Option<JavaVersion>,
    #[serde(default)]
    pub libraries: Vec<LibraryRef>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRef {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub total_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: DirectDownload,
    #[serde(default)]
    pub client_mappings: Option<DirectDownload>,
    #[serde(default)]
    pub server: Option<DirectDownload>,
    #[serde(default)]
    pub server_mappings: Option<DirectDownload>,
}

#[derive(Debug, Deserialize)]
pub struct DirectDownload {
    pub url: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub sha1: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JavaVersion {
    #[serde(default)]
    pub major_version: u32,
    #[serde(default)]
    pub component: String,
}

#[derive(Debug, Deserialize)]
pub struct LibraryRef {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
}

/// Fetch (cache-first) and parse the version json for a manifest entry.
pub fn resolve(agent: &ureq::Agent, cache: &Cache, entry_url: &str, entry_sha1: Option<&str>) -> Result<VersionJson> {
    let key = format!("version-json:{}", entry_sha1.unwrap_or(entry_url));
    let bytes = cache.fetch(agent, &key, entry_url)?;
    if let Some(sha) = entry_sha1 {
        crate::util::verify(&bytes, Some(sha), "version json")?;
    }
    Ok(serde_json::from_slice(&bytes)?)
}
