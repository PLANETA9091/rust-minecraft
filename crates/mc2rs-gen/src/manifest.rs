//! Mojang official version manifest (version_manifest_v2.json).

use anyhow::Result;
use serde::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<VEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    #[expect(dead_code)] // the manifest's upload timestamp — the json's own column
    pub time: String,
    pub release_time: String,
    #[serde(default)]
    pub sha1: Option<String>,
}

pub fn load(agent: &ureq::Agent) -> Result<Manifest> {
    let bytes = util::http_bytes(agent, util::MANIFEST_URL)?;
    Ok(serde_json::from_slice(&bytes)?)
}

impl Manifest {
    pub fn find(&self, id: &str) -> Option<&VEntry> {
        self.versions.iter().find(|v| v.id == id)
    }

    pub fn latest_release(&self) -> &str {
        &self.latest.release
    }
}
