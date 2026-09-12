//! mc-assets — official asset index + parallel sha1-verified downloader.
//!
//! Assets are fetched at runtime from `resources.download.minecraft.net`
//! and cached locally. Nothing is committed to the repository.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

/// Read the full body of a ureq response (no `into_vec` in ureq 2.12).
pub fn body_bytes(resp: ureq::Response) -> Result<Vec<u8>> {
    use std::io::Read;
    let mut reader = resp.into_reader();
    let mut out = Vec::new();
    reader.read_to_end(&mut out)?;
    Ok(out)
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    #[serde(default)]
    pub map_to_resources: bool,
    pub objects: BTreeMap<String, AssetObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

impl AssetIndex {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).context("asset index json parse")
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        Self::parse(&bytes)
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Cache path for an object: `<root>/objects/<h[0..2]>/<hash>`.
    pub fn cache_path(&self, root: &Path, hash: &str) -> PathBuf {
        root.join("objects").join(&hash[..2]).join(hash)
    }
}

const BASE_URL: &str = "https://resources.download.minecraft.net";

/// Download every object of the index into `root/objects/...` (16 parallel
/// workers). `progress` is called as (done, total) from worker threads.
pub fn download_all(
    index: &AssetIndex,
    root: &Path,
    agent: &ureq::Agent,
    progress: impl Fn(usize, usize) + Sync,
) -> Result<()> {
    let tasks: Vec<(&String, &AssetObject)> = index.objects.iter().collect();
    let total = tasks.len();
    let next = AtomicUsize::new(0);
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());

    std::thread::scope(|s| {
        for _ in 0..16 {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= total {
                    break;
                }
                let (path, obj) = tasks[i];
                if let Err(e) = download_object(root, agent, path, obj) {
                    errors.lock().unwrap().push(format!("{path}: {e}"));
                }
                if i % 50 == 0 {
                    progress(i, total);
                }
            });
        }
    });

    let errs = errors.into_inner().unwrap();
    if !errs.is_empty() {
        let head = errs.iter().take(5).map(|s| s.as_str()).collect::<Vec<_>>().join("; ");
        anyhow::bail!(
            "{} asset download(s) failed, first few: {}",
            errs.len(),
            head
        );
    }
    progress(total, total);
    Ok(())
}

fn download_object(root: &Path, agent: &ureq::Agent, _path: &str, obj: &AssetObject) -> Result<()> {
    let dest = root.join("objects").join(&obj.hash[..2]).join(&obj.hash);
    if dest.is_file() {
        if let Ok(meta) = fs::metadata(&dest) {
            if meta.len() == obj.size {
                return Ok(()); // already cached, size matches
            }
        }
    }
    let prefix = &obj.hash[..2];
    let hash = &obj.hash;
    let url = format!("{BASE_URL}/{prefix}/{hash}");
    let bytes = fetch_with_retry(agent, &url, 3)?;
    verify_sha1(&bytes, &obj.hash)?;
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::write(&dest, &bytes)?;
    Ok(())
}

fn fetch_with_retry(agent: &ureq::Agent, url: &str, retries: u32) -> Result<Vec<u8>> {
    let mut last_err = None;
    for _ in 0..retries {
        match agent.get(url).call() {
            Ok(resp) => match body_bytes(resp) {
                Ok(b) => return Ok(b),
                Err(e) => last_err = Some(anyhow!("read body: {e}")),
            },
            Err(e) => last_err = Some(anyhow!("http: {e}")),
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    Err(last_err.unwrap_or_else(|| anyhow!("fetch failed")))
}

fn verify_sha1(bytes: &[u8], expected: &str) -> Result<()> {
    use sha1::{Digest, Sha1};
    let digest = hex::encode(Sha1::digest(bytes));
    if digest != expected {
        anyhow::bail!("sha1 mismatch: got {digest}, expected {expected}");
    }
    Ok(())
}
