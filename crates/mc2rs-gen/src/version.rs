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
#[serde(rename_all = "camelCase")]
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
    /// Vanilla launcher rule list (os/action); absent = unconditional.
    #[serde(default)]
    pub rules: Option<Vec<LibraryRule>>,
    /// v2 manifest per-artifact download (authoritative url + sha1).
    #[serde(default)]
    pub downloads: Option<LibraryDownloads>,
}

impl LibraryRef {
    /// Vanilla launcher rule semantics: no rules -> allowed; with rules,
    /// the LAST matching rule wins and an unmatched-only list rejects.
    /// `os` is the host platform name ("linux", "windows", "osx").
    pub fn allowed_on(&self, os: &str) -> bool {
        let Some(rules) = &self.rules else {
            return true;
        };
        let mut allowed = false;
        for r in rules {
            let applies = match &r.os {
                None => true,
                Some(spec) => spec.name.as_deref() == Some(os),
            };
            if applies {
                allowed = r.action == "allow";
            }
        }
        allowed
    }

    /// The main artifact to put on the runtime classpath (natives and
    /// per-os classifiers are separate entries and not needed for a
    /// headless registry bootstrap).
    pub fn artifact(&self) -> Option<&DirectDownload> {
        self.downloads.as_ref().and_then(|d| d.artifact.as_ref())
    }
}

#[derive(Debug, Deserialize)]
pub struct LibraryRule {
    pub action: String,
    #[serde(default)]
    pub os: Option<LibraryOs>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryOs {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownloads {
    #[serde(default)]
    pub artifact: Option<DirectDownload>,
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
