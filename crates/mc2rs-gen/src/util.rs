//! HTTP + hashing + cache helpers shared by the generator.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use sha1::{Digest, Sha1};

pub const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(15))
        .timeout(Duration::from_secs(1800))
        .user_agent("mc2rs-generator/0.2 (+https://github.com/PLANETA9091/rust-minecraft)")
        .build()
}

/// GET a URL and return the raw body bytes.
pub fn http_bytes(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    let resp = agent
        .get(url)
        .call()
        .with_context(|| format!("GET {url}"))?;
    let mut reader = resp.into_reader();
    let mut out = Vec::new();
    reader.read_to_end(&mut out)?;
    Ok(out)
}

pub fn sha1_hex(bytes: &[u8]) -> String {
    hex::encode(Sha1::digest(bytes))
}

/// Verify bytes against an expected sha1 (if provided).
pub fn verify(bytes: &[u8], expected: Option<&str>, label: &str) -> Result<()> {
    if let Some(expected) = expected {
        let got = sha1_hex(bytes);
        anyhow::ensure!(
            got == expected,
            "{label} sha1 mismatch: got {got}, expected {expected}"
        );
    }
    Ok(())
}

/// Content-addressed cache directory (default `.mc2rs/cache`).
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        self.root.join(hex::encode(Sha1::digest(key.as_bytes())))
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        fs::read(self.path_for(key)).ok()
    }

    pub fn put(&self, key: &str, bytes: &[u8]) -> Result<()> {
        let p = self.path_for(key);
        fs::create_dir_all(p.parent().unwrap())?;
        fs::write(p, bytes)?;
        Ok(())
    }

    /// Fetch through the cache (cache-first).
    pub fn fetch(&self, agent: &ureq::Agent, key: &str, url: &str) -> Result<Vec<u8>> {
        if let Some(b) = self.get(key) {
            return Ok(b);
        }
        let b = http_bytes(agent, url)?;
        self.put(key, &b)?;
        Ok(b)
    }
}

/// Default cache location next to CWD.
pub fn default_cache() -> Cache {
    Cache::new(Path::new(".mc2rs").join("cache"))
}
