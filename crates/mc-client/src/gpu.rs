//! GPU fast path (feature `gpu`) — placeholder wired for CI.
//! The deterministic CPU renderer stays the pixel-perfect reference; the GPU
//! backend (wgpu instanced quads + frustum culling) must produce identical
//! frames per `mc2rs-pixel`, then replaces it as the realtime renderer.

pub fn gpu_path_status() -> &'static str {
    "gpu backend: scaffolded (wgpu instancing + frustum culling planned); CPU reference renderer active"
}
