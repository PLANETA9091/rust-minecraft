//! Skin generation: the official player skin (wide Steve) out of the client
//! jar — the data half of the first-person hand arc. The generated table
//! carries the source's integrity sha1s plus the vanilla `PlayerModel`
//! arm-box geometry (the UV unwrap the first-person arm renders with); the
//! PIXELS stream at runtime from the extracted asset root (the font-texture
//! pattern), sha1-verified against the constants the generator recorded.

use anyhow::{Context, Result};

use crate::extract::{decode_rgba, read_entry};

/// The jar entry the wide (classic Steve) skin lives at.
pub const WIDE_STEVE_JAR_PATH: &str = "assets/minecraft/textures/entity/player/wide/steve.png";
/// The jar entry the slim (Alex) skin lives at.
pub const SLIM_ALEX_JAR_PATH: &str = "assets/minecraft/textures/entity/player/slim/alex.png";
/// The jar entry the zombie's skin lives at (javap: `AbstractZombieRenderer`
/// static init — `textures/entity/zombie/zombie.png`, the same 64x64 wide
/// humanoid layout the PlayerModel geometry unwraps).
pub const ZOMBIE_JAR_PATH: &str = "assets/minecraft/textures/entity/zombie/zombie.png";

/// The flame feature's two animated strips (`FlameFeatureRenderer`'s
/// ModelBakery.FIRE_0 / FIRE_1 sprite sources) — the burn lane reads
/// their frames off the blocks atlas path the runtime cache mirrors.
pub const FIRE_0_JAR_PATH: &str = "assets/minecraft/textures/block/fire_0.png";
pub const FIRE_1_JAR_PATH: &str = "assets/minecraft/textures/block/fire_1.png";
/// The water screen's texture (javap `ScreenEffectRenderer.submitWater`:
/// the `UNDERWATER_LOCATION` static init — `textures/misc/underwater.png`,
/// the 16x16 blue the eye-under-water pass stretches over the screen
/// through `RenderTypes.blockScreenEffect`).
pub const UNDERWATER_JAR_PATH: &str = "assets/minecraft/textures/misc/underwater.png";
/// The XP orb's sprite sheet (javap `ExperienceOrbRenderer`'s
/// EXPERIENCE_ORB_LOCATION static init —
/// `textures/entity/experience/experience_orb.png`, the 64x64 sheet the
/// 4x4 grid of 16x16 icons tiles — the award orbs' pixel source the
/// getIcon band law samples).
pub const EXPERIENCE_ORB_JAR_PATH: &str =
    "assets/minecraft/textures/entity/experience/experience_orb.png";
/// The crit star's texture (javap: the `crit` particle type's own
/// sprite — the `TrackingEmitter`'s spawn draws the star the
/// `CritParticle` speaks; the jar's `textures/particle/critical_hit.png`,
/// the 8x8 the IHDR verifies).
pub const PARTICLE_CRIT_JAR_PATH: &str = "assets/minecraft/textures/particle/critical_hit.png";
/// The drown bubble's texture (javap: the `bubble` particle type's own
/// sprite — the `BubbleParticle` the broadcast-67 makeDrownParticles
/// law spawns; the jar's `textures/particle/bubble.png`, the 8x8 the
/// IHDR verifies).
pub const PARTICLE_BUBBLE_JAR_PATH: &str = "assets/minecraft/textures/particle/bubble.png";
/// The poof cloud's 8 frames (javap: the `poof` particle type's sprite
/// set — `ExplodeParticle`'s `setSpriteFromAge` walk over the 8
/// `MutableSpriteSet` frames; the jar's
/// `textures/particle/generic_0..7.png`, the 8x8s the IHDR verifies).
pub const PARTICLE_POOF_JAR_PATHS: [&str; 8] = [
    "assets/minecraft/textures/particle/generic_0.png",
    "assets/minecraft/textures/particle/generic_1.png",
    "assets/minecraft/textures/particle/generic_2.png",
    "assets/minecraft/textures/particle/generic_3.png",
    "assets/minecraft/textures/particle/generic_4.png",
    "assets/minecraft/textures/particle/generic_5.png",
    "assets/minecraft/textures/particle/generic_6.png",
    "assets/minecraft/textures/particle/generic_7.png",
];
/// The splash droplet's 4 frames (javap: the `splash` particle type's
/// sprite set — the `SplashParticle$Provider`'s `SpriteSet.get(random)`
/// pick, one frame static per particle's own life; the jar's
/// `textures/particle/splash_0..3.png`, the 8x8s the IHDR verifies).
pub const PARTICLE_SPLASH_JAR_PATHS: [&str; 4] = [
    "assets/minecraft/textures/particle/splash_0.png",
    "assets/minecraft/textures/particle/splash_1.png",
    "assets/minecraft/textures/particle/splash_2.png",
    "assets/minecraft/textures/particle/splash_3.png",
];
/// The lava drip's single sprite (javap: the `lava` particle type's
/// sprite set — the jar's `particles/lava.json` lists ONE texture,
/// `minecraft:lava`, the 8x8 the IHDR verifies; the
/// `LavaParticle$Provider`'s `SpriteSet.get(random)` returns it
/// whatever the draw).
pub const PARTICLE_LAVA_JAR_PATH: &str = "assets/minecraft/textures/particle/lava.png";
/// The damage indicator's single sprite (javap: the `damage_indicator`
/// particle type's sprite set — the jar's `particles/damage_indicator.json`
/// lists ONE texture, `minecraft:damage`, the 8x8 the IHDR verifies; the
/// `CritParticle$DamageIndicatorProvider`'s `SpriteSet.get(random)`
/// returns it whatever the draw).
pub const PARTICLE_DAMAGE_JAR_PATH: &str = "assets/minecraft/textures/particle/damage.png";
/// The runtime-cache paths the client's loader reads (the same
/// textures/ prefix strip the other lanes speak).
pub const FIRE_0_RUNTIME_PATH: &str = "block/fire_0.png";
pub const FIRE_1_RUNTIME_PATH: &str = "block/fire_1.png";
pub const UNDERWATER_RUNTIME_PATH: &str = "misc/underwater.png";
pub const EXPERIENCE_ORB_RUNTIME_PATH: &str = "entity/experience/experience_orb.png";
/// The particle lane's runtime-cache paths (the same textures/ prefix
/// strip the other lanes speak).
pub const PARTICLE_CRIT_RUNTIME_PATH: &str = "particle/critical_hit.png";
pub const PARTICLE_BUBBLE_RUNTIME_PATH: &str = "particle/bubble.png";
pub const PARTICLE_POOF_RUNTIME_PATHS: [&str; 8] = [
    "particle/generic_0.png",
    "particle/generic_1.png",
    "particle/generic_2.png",
    "particle/generic_3.png",
    "particle/generic_4.png",
    "particle/generic_5.png",
    "particle/generic_6.png",
    "particle/generic_7.png",
];
pub const PARTICLE_SPLASH_RUNTIME_PATHS: [&str; 4] = [
    "particle/splash_0.png",
    "particle/splash_1.png",
    "particle/splash_2.png",
    "particle/splash_3.png",
];
pub const PARTICLE_LAVA_RUNTIME_PATH: &str = "particle/lava.png";
pub const PARTICLE_DAMAGE_RUNTIME_PATH: &str = "particle/damage.png";
/// Where the client reads the skin at runtime (under the runtime asset root).
pub const RUNTIME_PATH: &str = "entity/player/wide/steve.png";
pub const SLIM_RUNTIME_PATH: &str = "entity/player/slim/alex.png";
pub const ZOMBIE_RUNTIME_PATH: &str = "entity/zombie/zombie.png";

/// The vanilla `PlayerModel` arm box for the wide model: texOffs (40, 16),
/// box 4x12x4 texels (w, h, d).
pub const ARM_TEX_OFFS: (u16, u16) = (40, 16);
pub const ARM_BOX_PX: [u16; 3] = [4, 12, 4];

/// The vanilla `PlayerModel` arm box for the SLIM model, javap-evidenced
/// from the official 26.2 jar: `PlayerModel.createMesh(CubeDeformation,
/// boolean slim = true)` — right_arm `texOffs(40, 16)` (UNCHANGED from the
/// wide model), `addBox(-2, -2, -2, 3, 12, 4)`, pivot `offset(-5, 2, 0)`
/// (UNCHANGED — the transform chain in the client is kind-independent).
pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];
pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];

/// The extracted skin facts the generated table carries.
pub struct SkinData {
    /// sha1 of the source PNG bytes (jar-verbatim integrity).
    pub png_sha1: String,
    /// sha1 of the decoded RGBA8 pixels (row-major, side*side*4 bytes).
    pub rgba_sha1: String,
    /// texture side (the classic skin layout is 64x64).
    pub side: u32,
    /// sha1 of the slim (Alex) source PNG bytes (jar-verbatim integrity).
    pub slim_png_sha1: String,
    /// sha1 of the decoded slim RGBA8 pixels.
    pub slim_rgba_sha1: String,
    /// sha1 of the zombie's source PNG bytes (jar-verbatim integrity) —
    /// the first mob's texture the avatar renderer sha1-verifies.
    pub zombie_png_sha1: String,
    /// sha1 of the flame feature's fire_0 strip (jar-verbatim integrity)
    /// — the burn lane's pixel source (the 16x512, 32-frame strip).
    pub fire_0_png_sha1: String,
    /// sha1 of the flame feature's fire_1 strip (jar-verbatim integrity).
    pub fire_1_png_sha1: String,
    /// sha1 of the water screen's texture (jar-verbatim integrity) —
    /// the underwater overlay's pixel source (the 16x16 blue).
    pub underwater_png_sha1: String,
    /// sha1 of the XP orb's sprite sheet (jar-verbatim integrity) —
    /// the award orbs' pixel source (the 64x64, 4x4 icon grid).
    pub experience_orb_png_sha1: String,
    /// sha1 of the crit star's texture (jar-verbatim integrity) — the
    /// particle lane's crit quad's pixel source (the 8x8 sprite).
    pub particle_crit_png_sha1: String,
    /// sha1 of the drown bubble's texture (jar-verbatim integrity) —
    /// the bubble quad's pixel source (the 8x8 sprite).
    pub particle_bubble_png_sha1: String,
    /// sha1s of the poof cloud's 8 frames (jar-verbatim integrity) —
    /// the death cloud's pixel sources (the 8x8 generic frames).
    pub particle_poof_png_sha1s: [String; 8],
    /// sha1s of the splash droplet's 4 frames (jar-verbatim integrity)
    /// — the entry splash's second ring's pixel sources (the 8x8
    /// splash frames).
    pub particle_splash_png_sha1s: [String; 4],
    /// sha1 of the lava drip's single 8x8 sprite (jar-verbatim
    /// integrity) — the pool's own drip's pixel source.
    pub particle_lava_png_sha1: String,
    /// sha1 of the damage indicator's single 8x8 sprite (jar-verbatim
    /// integrity) — the hit's own burst's pixel source.
    pub particle_damage_png_sha1: String,
}

/// One UV rect in the skin texture, in texels: (u, v, w, h).
type Rect = (u16, u16, u16, u16);

/// The arm box's six faces in vanilla `ModelPart` unwrap order and sizes:
/// top, bottom, then the side strip right -> front -> left -> back.
pub fn arm_faces() -> [Rect; 6] {
    arm_faces_for(ARM_TEX_OFFS, ARM_BOX_PX)
}

/// The unwrap for ANY box: the same `ModelPart$Cube` layout, parameterized
/// by texOffs + (w, h, d) — the wide and slim arms share it (the slim arm's
/// only geometry deltas are the box width 4 -> 3 and the origin x
/// -3 -> -2; texOffs and the pivot are unchanged, javap).
pub fn arm_faces_for(tex_offs: (u16, u16), box_px: [u16; 3]) -> [Rect; 6] {
    let (u, v) = tex_offs;
    let [w, h, d] = box_px;
    [
        (u + d, v, w, d),             // top    (w x d)
        (u + d + w, v, w, d),         // bottom (w x d)
        (u, v + d, d, h),             // right  (d x h) — outer
        (u + d, v + d, w, h),         // front  (w x h)
        (u + d + w, v + d, d, h),     // left   (d x h) — inner
        (u + d + w + d, v + d, w, h), // back   (w x h)
    ]
}

/// Read both official skins out of the jar, decode them and pin their
/// integrity — the wide (classic) and the slim (Alex) variants.
pub fn generate<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> Result<SkinData> {
    let bytes = read_entry(archive, WIDE_STEVE_JAR_PATH).context("read the wide Steve skin")?;
    let png_sha1 = crate::util::sha1_hex(&bytes);
    let (w, h, rgba) = decode_rgba(&bytes).context("decode the wide Steve skin")?;
    let side = skin_side(w, h)?;
    let slim_bytes = read_entry(archive, SLIM_ALEX_JAR_PATH).context("read the slim Alex skin")?;
    let slim_png_sha1 = crate::util::sha1_hex(&slim_bytes);
    let (sw, sh, slim_rgba) = decode_rgba(&slim_bytes).context("decode the slim Alex skin")?;
    let slim_side = skin_side(sw, sh)?;
    anyhow::ensure!(
        slim_side == side,
        "the slim skin must share the classic layout ({}x{}), got {}x{}",
        side,
        side,
        slim_side,
        slim_side
    );
    let zombie_bytes = read_entry(archive, ZOMBIE_JAR_PATH).context("read the zombie skin")?;
    let zombie_png_sha1 = crate::util::sha1_hex(&zombie_bytes);
    let (zw, zh, zombie_rgba) = decode_rgba(&zombie_bytes).context("decode the zombie skin")?;
    let zombie_side = skin_side(zw, zh)?;
    anyhow::ensure!(
        zombie_side == side,
        "the zombie skin must share the classic layout ({}x{}), got {}x{}",
        side,
        side,
        zombie_side,
        zombie_side
    );
    std::mem::drop(zombie_rgba);
    // THE FIRE STRIPS: the flame feature's two animated pixel sources
    // (javap `FlameFeatureRenderer.buildGroup`: ModelBakery.FIRE_0 /
    // FIRE_1) — 16x512 = 32 stacked 16x16 frames each, the mcmeta
    // playlist [16..31, 0..15] on fire_0, sequential on fire_1. The
    // integrity sha1 the burn lane verifies before it draws.
    let fire_0_bytes = read_entry(archive, FIRE_0_JAR_PATH).context("read fire_0")?;
    let fire_0_png_sha1 = crate::util::sha1_hex(&fire_0_bytes);
    let (fw, fh, _fire_0_rgba) = decode_rgba(&fire_0_bytes).context("decode fire_0")?;
    anyhow::ensure!(
        (fw, fh) == (16, 512),
        "fire_0 must be the 16x512 strip, got {}x{}",
        fw,
        fh
    );
    let fire_1_bytes = read_entry(archive, FIRE_1_JAR_PATH).context("read fire_1")?;
    let fire_1_png_sha1 = crate::util::sha1_hex(&fire_1_bytes);
    let (fw1, fh1, _fire_1_rgba) = decode_rgba(&fire_1_bytes).context("decode fire_1")?;
    anyhow::ensure!(
        (fw1, fh1) == (16, 512),
        "fire_1 must be the 16x512 strip, got {}x{}",
        fw1,
        fh1
    );
    // THE WATER SCREEN: the underwater overlay's pixel source (javap
    // `ScreenEffectRenderer.submitWater`'s UNDERWATER_LOCATION static) —
    // the 16x16 blue the eye-under-water pass tiles 4 wraps across the
    // screen. The integrity sha1 the screen lane verifies before it
    // blends.
    let underwater_bytes = read_entry(archive, UNDERWATER_JAR_PATH).context("read underwater")?;
    let underwater_png_sha1 = crate::util::sha1_hex(&underwater_bytes);
    let (uw, uh, _underwater_rgba) = decode_rgba(&underwater_bytes).context("decode underwater")?;
    anyhow::ensure!(
        (uw, uh) == (16, 16),
        "underwater must be the 16x16 screen texture, got {}x{}",
        uw,
        uh
    );
    // THE XP ORB: the award orbs' pixel source (javap
    // `ExperienceOrbRenderer`'s EXPERIENCE_ORB_LOCATION static) — the
    // 64x64 sheet the 4x4 grid of 16x16 icons tiles. The integrity sha1
    // the orb lane verifies before it draws.
    let orb_bytes = read_entry(archive, EXPERIENCE_ORB_JAR_PATH)
        .context("read experience_orb")?;
    let experience_orb_png_sha1 = crate::util::sha1_hex(&orb_bytes);
    let (ow, oh, _orb_rgba) = decode_rgba(&orb_bytes).context("decode experience_orb")?;
    anyhow::ensure!(
        (ow, oh) == (64, 64),
        "experience_orb must be the 64x64 sheet, got {}x{}",
        ow,
        oh
    );
    // THE PARTICLE LANE: the crit star's 8x8 and the poof cloud's 8
    // 8x8 frames (javap: the `crit` + `poof` particle types' sprites —
    // the TrackingEmitter's star and the death cloud's frame walk).
    // The integrity sha1s the particle lane verifies before it draws.
    let crit_bytes = read_entry(archive, PARTICLE_CRIT_JAR_PATH).context("read critical_hit")?;
    let particle_crit_png_sha1 = crate::util::sha1_hex(&crit_bytes);
    let (cw, ch, _crit_rgba) = decode_rgba(&crit_bytes).context("decode critical_hit")?;
    anyhow::ensure!(
        (cw, ch) == (8, 8),
        "critical_hit must be the 8x8 sprite, got {}x{}",
        cw,
        ch
    );
    // the drown bubble's 8x8 (the same shape check the crit speaks)
    let bubble_bytes = read_entry(archive, PARTICLE_BUBBLE_JAR_PATH).context("read bubble")?;
    let particle_bubble_png_sha1 = crate::util::sha1_hex(&bubble_bytes);
    let (bw, bh, _bubble_rgba) = decode_rgba(&bubble_bytes).context("decode bubble")?;
    anyhow::ensure!(
        (bw, bh) == (8, 8),
        "bubble must be the 8x8 sprite, got {}x{}",
        bw,
        bh
    );
    let mut particle_poof_png_sha1s: [String; 8] = std::array::from_fn(|_| String::new());
    for (i, path) in PARTICLE_POOF_JAR_PATHS.iter().enumerate() {
        let bytes = read_entry(archive, path).context("read generic frame")?;
        let (gw, gh, _rgba) = decode_rgba(&bytes).context("decode generic frame")?;
        anyhow::ensure!(
            (gw, gh) == (8, 8),
            "generic_{i} must be the 8x8 frame, got {}x{}",
            gw,
            gh
        );
        particle_poof_png_sha1s[i] = crate::util::sha1_hex(&bytes);
    }
    let mut particle_splash_png_sha1s: [String; 4] = std::array::from_fn(|_| String::new());
    for (i, path) in PARTICLE_SPLASH_JAR_PATHS.iter().enumerate() {
        let bytes = read_entry(archive, path).context("read splash frame")?;
        let (sw, sh, _rgba) = decode_rgba(&bytes).context("decode splash frame")?;
        anyhow::ensure!(
            (sw, sh) == (8, 8),
            "splash_{i} must be the 8x8 frame, got {}x{}",
            sw,
            sh
        );
        particle_splash_png_sha1s[i] = crate::util::sha1_hex(&bytes);
    }
    // the lava drip's single sprite (the same shape check the bubble
    // speaks — the json's one-texture set)
    let lava_bytes = read_entry(archive, PARTICLE_LAVA_JAR_PATH).context("read lava")?;
    let particle_lava_png_sha1 = crate::util::sha1_hex(&lava_bytes);
    let (lw, lh, _lava_rgba) = decode_rgba(&lava_bytes).context("decode lava")?;
    anyhow::ensure!(
        (lw, lh) == (8, 8),
        "lava must be the 8x8 sprite, got {}x{}",
        lw,
        lh
    );
    // the damage indicator's single sprite (the same shape check the
    // lava speaks — the json's one-texture set)
    let damage_bytes = read_entry(archive, PARTICLE_DAMAGE_JAR_PATH).context("read damage")?;
    let particle_damage_png_sha1 = crate::util::sha1_hex(&damage_bytes);
    let (dw2, dh2, _damage_rgba) = decode_rgba(&damage_bytes).context("decode damage")?;
    anyhow::ensure!(
        (dw2, dh2) == (8, 8),
        "damage must be the 8x8 sprite, got {}x{}",
        dw2,
        dh2
    );
    Ok(SkinData {
        png_sha1,
        rgba_sha1: crate::util::sha1_hex(&rgba),
        side,
        slim_png_sha1,
        slim_rgba_sha1: crate::util::sha1_hex(&slim_rgba),
        zombie_png_sha1,
        fire_0_png_sha1,
        fire_1_png_sha1,
        underwater_png_sha1,
        experience_orb_png_sha1,
        particle_crit_png_sha1,
        particle_bubble_png_sha1,
        particle_poof_png_sha1s,
        particle_splash_png_sha1s,
        particle_lava_png_sha1,
        particle_damage_png_sha1,
    })
}

/// The layout checks both skins must pass: square and the classic 64x64.
fn skin_side(w: u32, h: u32) -> Result<u32> {
    anyhow::ensure!(w == h, "skin texture must be square, got {w}x{h}");
    anyhow::ensure!(
        w == 64,
        "the classic skin layout is 64x64 texels, got {w}x{h}"
    );
    Ok(w)
}

/// Emit `gen_skin.rs` content: the integrity sha1s + the arm-box geometry.
pub fn emit(data: &SkinData) -> Result<String> {
    let mut out = String::new();
    out.push_str("//! GENERATED by mc2rs-gen from the official player skin (wide Steve).\n");
    out.push_str(&format!(
        "//! Source: {WIDE_STEVE_JAR_PATH} (sha1 below), decoded to RGBA8 and\n//! hashed by the generator. Do not edit: regenerate with\n//! `cargo run -p mc2rs-gen -- skin --version <ver> --jar <client.jar>`.\n\n"
    ));
    out.push_str("/// sha1 of the verbatim source PNG inside the client jar.\n");
    out.push_str(&format!(
        "pub const STEVE_WIDE_PNG_SHA1: &str = \"{}\";\n",
        data.png_sha1
    ));
    out.push_str("/// sha1 over the decoded RGBA8 pixels (row-major, side*side*4 bytes)\n/// exactly as the generator's decoder produced them.\n");
    out.push_str(&format!(
        "pub const STEVE_WIDE_RGBA_SHA1: &str = \"{}\";\n\n",
        data.rgba_sha1
    ));
    out.push_str("/// The classic skin texture is 64x64 texels.\n");
    out.push_str(&format!("pub const SKIN_SIDE: u32 = {};\n\n", data.side));
    out.push_str("/// One UV rect in the skin texture, in texels.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct FaceRect {\n    pub u: u16,\n    pub v: u16,\n    pub w: u16,\n    pub h: u16,\n}\n\n");
    out.push_str("/// The vanilla `PlayerModel` arm box for the wide model: texOffs (40, 16),\n/// box 4x12x4 texels (w, h, d).\n");
    out.push_str("pub const ARM_TEX_OFFS: (u16, u16) = (40, 16);\n");
    out.push_str("pub const ARM_BOX_PX: [u16; 3] = [4, 12, 4];\n\n");
    out.push_str("/// The arm box's six faces in vanilla `ModelPart` unwrap order:\n/// top, bottom, then the side strip right -> front -> left -> back.\n");
    out.push_str("pub static ARM_FACES: [FaceRect; 6] = [\n");
    let names = ["top", "bottom", "right (outer)", "front", "left (inner)", "back"];
    for (name, (u, v, w, h)) in names.iter().zip(arm_faces()) {
        out.push_str(&format!(
            "    FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}, // {name}\n"
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Index into ARM_FACES (vanilla cube face order).\n");
    for (i, name) in ["TOP", "BOTTOM", "RIGHT", "FRONT", "LEFT", "BACK"].iter().enumerate() {
        out.push_str(&format!("pub const FACE_{name}: usize = {i};\n"));
    }
    out.push_str("\n/// The runtime asset file the client reads (under the runtime asset root),\n/// sha1-verified against STEVE_WIDE_PNG_SHA1.\n");
    out.push_str(&format!("pub const RUNTIME_PATH: &str = \"{RUNTIME_PATH}\";\n"));
    out.push_str("\n/// The SLIM (Alex) skin, generated from the same jar:\n/// sha1s + the slim arm box (javap: `PlayerModel.createMesh` with\n/// `slim = true` — right_arm `texOffs(40, 16)` unchanged,\n/// `addBox(-2, -2, -2, 3, 12, 4)`, pivot `(-5, 2, 0)` unchanged).\n");
    out.push_str(&format!(
        "pub const SLIM_ALEX_PNG_SHA1: &str = \"{}\";\n",
        data.slim_png_sha1
    ));
    out.push_str(&format!(
        "pub const SLIM_ALEX_RGBA_SHA1: &str = \"{}\";\n",
        data.slim_rgba_sha1
    ));
    out.push_str("pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];\n");
    out.push_str("pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];\n");
    out.push_str("pub static SLIM_ARM_FACES: [FaceRect; 6] = [\n");
    for (name, (u, v, w, h)) in names.iter().zip(arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX)) {
        out.push_str(&format!(
            "    FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}, // {name}\n"
        ));
    }
    out.push_str("];\n");
    out.push_str(&format!(
        "pub const SLIM_RUNTIME_PATH: &str = \"{SLIM_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The ZOMBIE skin (the first mob's own), generated from the same jar:\n/// the source sha1 the avatar renderer verifies before drawing\n/// (javap: `AbstractZombieRenderer`'s ZOMBIE_LOCATION — the same\n/// 64x64 wide humanoid layout the AvatarModel geometry unwraps).\n");
    out.push_str(&format!(
        "pub const ZOMBIE_PNG_SHA1: &str = \"{}\";\n",
        data.zombie_png_sha1
    ));
    out.push_str(&format!(
        "pub const ZOMBIE_RUNTIME_PATH: &str = \"{ZOMBIE_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The FLAME strips (the burn lane's pixel sources), generated from\n/// the same jar: the source sha1s the burning mob's renderer verifies\n/// before it draws (javap: `FlameFeatureRenderer.buildGroup`'s\n/// ModelBakery.FIRE_0 / FIRE_1 — the 16x512, 32-frame strips; fire_0's\n/// mcmeta playlist [16..31, 0..15], fire_1 sequential).\n");
    out.push_str(&format!(
        "pub const FIRE_0_PNG_SHA1: &str = \"{}\";\n",
        data.fire_0_png_sha1
    ));
    out.push_str(&format!(
        "pub const FIRE_0_RUNTIME_PATH: &str = \"{FIRE_0_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const FIRE_1_PNG_SHA1: &str = \"{}\";\n",
        data.fire_1_png_sha1
    ));
    out.push_str(&format!(
        "pub const FIRE_1_RUNTIME_PATH: &str = \"{FIRE_1_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The WATER SCREEN texture (the underwater overlay's pixel source),\n/// generated from the same jar: the source sha1 the screen lane\n/// verifies before it blends (javap: `ScreenEffectRenderer.submitWater`'s\n/// UNDERWATER_LOCATION — `textures/misc/underwater.png`, the 16x16 blue\n/// the eye-under-water pass tiles across the screen).\n");
    out.push_str(&format!(
        "pub const UNDERWATER_PNG_SHA1: &str = \"{}\";\n",
        data.underwater_png_sha1
    ));
    out.push_str(&format!(
        "pub const UNDERWATER_RUNTIME_PATH: &str = \"{UNDERWATER_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The XP ORB's sprite sheet (the award orbs' pixel source), generated\n/// from the same jar: the source sha1 the orb lane verifies before it\n/// draws (javap: `ExperienceOrbRenderer`'s EXPERIENCE_ORB_LOCATION —\n/// `textures/entity/experience/experience_orb.png`, the 64x64 sheet the\n/// 4x4 grid of 16x16 icons tiles, the icon variant the getIcon band law\n/// speaks).\n");
    out.push_str(&format!(
        "pub const EXPERIENCE_ORB_PNG_SHA1: &str = \"{}\";\n",
        data.experience_orb_png_sha1
    ));
    out.push_str(&format!(
        "pub const EXPERIENCE_ORB_RUNTIME_PATH: &str = \"{EXPERIENCE_ORB_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The PARTICLE LANE's textures (the crit star's 8x8, the drown\n/// bubble's 8x8, the poof cloud's 8 8x8 frames, the splash droplet's\n/// 4 8x8 frames), generated from the same jar: the integrity sha1s\n/// the particle lane verifies before it draws (javap: the `crit` +\n/// `bubble` + `poof` + `splash` particle types' sprites — the\n/// TrackingEmitter's star, the broadcast-67 arm's bubbles, the death\n/// cloud's `MutableSpriteSet` frame walk, the entry splash's\n/// `SpriteSet.get` pick).\n");
    out.push_str(&format!(
        "pub const PARTICLE_CRIT_PNG_SHA1: &str = \"{}\";\n",
        data.particle_crit_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_CRIT_RUNTIME_PATH: &str = \"{PARTICLE_CRIT_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_BUBBLE_PNG_SHA1: &str = \"{}\";\n",
        data.particle_bubble_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_BUBBLE_RUNTIME_PATH: &str = \"{PARTICLE_BUBBLE_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_POOF_PNG_SHA1S: [&str; 8] = [{}];\n",
        data.particle_poof_png_sha1s
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "pub const PARTICLE_POOF_RUNTIME_PATHS: [&str; 8] = [{}];\n",
        PARTICLE_POOF_RUNTIME_PATHS
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "pub const PARTICLE_SPLASH_PNG_SHA1S: [&str; 4] = [{}];\n",
        data.particle_splash_png_sha1s
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "pub const PARTICLE_SPLASH_RUNTIME_PATHS: [&str; 4] = [{}];\n",
        PARTICLE_SPLASH_RUNTIME_PATHS
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "pub const PARTICLE_LAVA_PNG_SHA1: &str = \"{}\";\n",
        data.particle_lava_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_LAVA_RUNTIME_PATH: &str = \"{PARTICLE_LAVA_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_DAMAGE_PNG_SHA1: &str = \"{}\";\n",
        data.particle_damage_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_DAMAGE_RUNTIME_PATH: &str = \"{PARTICLE_DAMAGE_RUNTIME_PATH}\";\n"
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(entries: &[(&str, Vec<u8>)]) -> zip::ZipArchive<std::io::Cursor<Vec<u8>>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                std::io::Write::write_all(&mut w, data).unwrap();
            }
            w.finish().unwrap();
        }
        zip::ZipArchive::new(buf).unwrap()
    }

    /// A 64x64 RGBA png with the arm's top + front faces painted.
    fn skin_png() -> Vec<u8> {
        let (w, h) = (64u32, 64u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let put = |rgba: &mut [u8], x: u32, y: u32, c: [u8; 4]| {
            let o = ((y * w + x) * 4) as usize;
            rgba[o..o + 4].copy_from_slice(&c);
        };
        for y in 16..20 {
            for x in 44..48 {
                put(&mut rgba, x, y, [0, 175, 175, 255]);
            }
        }
        for y in 20..32 {
            for x in 44..48 {
                put(&mut rgba, x, y, [155, 99, 73, 255]);
            }
        }
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    /// A 16x512 RGBA png — the fire strips' shape (32 stacked 16x16
    /// frames), one opaque orange texel so the decode is non-trivial.
    fn strip_png() -> Vec<u8> {
        let (w, h) = (16u32, 512u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let o = ((8 * w + 8) * 4) as usize;
        rgba[o..o + 4].copy_from_slice(&[218, 149, 17, 255]);
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    /// A 16x16 RGBA png — the water screen's shape, one translucent blue
    /// texel so the decode is non-trivial.
    fn underwater_png() -> Vec<u8> {
        let (w, h) = (16u32, 16u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let o = ((3 * w + 5) * 4) as usize;
        rgba[o..o + 4].copy_from_slice(&[31, 85, 255, 135]);
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    /// An 8x8 RGBA png — the particle lane's sprite shape (the crit
    /// star's own, the poof frames' own), one lit texel so the decode
    /// is non-trivial.
    fn tiny_png() -> Vec<u8> {
        let (w, h) = (8u32, 8u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let o = ((2 * w + 3) * 4) as usize;
        rgba[o..o + 4].copy_from_slice(&[255, 220, 90, 255]);
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    #[test]
    fn parses_the_skins_and_emits_the_arm_geometry() {
        let png = skin_png();
        let expected_png_sha1 = crate::util::sha1_hex(&png);
        let (_, _, decoded) = crate::extract::decode_rgba(&png).unwrap();
        let expected_rgba_sha1 = crate::util::sha1_hex(&decoded);

        let strip = strip_png();
        let expected_fire0_sha1 = crate::util::sha1_hex(&strip);
        let underwater = underwater_png();
        let expected_underwater_sha1 = crate::util::sha1_hex(&underwater);
        let orb = skin_png();
        let expected_orb_sha1 = crate::util::sha1_hex(&orb);
        let crit = tiny_png();
        let expected_crit_sha1 = crate::util::sha1_hex(&crit);
        let bubble = tiny_png();
        let expected_bubble_sha1 = crate::util::sha1_hex(&bubble);
        let poofs = [tiny_png(), tiny_png(), tiny_png(), tiny_png(), tiny_png(), tiny_png(), tiny_png(), tiny_png()];
        let expected_poof_sha1s: Vec<String> = poofs.iter().map(|p| crate::util::sha1_hex(p)).collect();
        let splashes = [tiny_png(), tiny_png(), tiny_png(), tiny_png()];
        let expected_splash_sha1s: Vec<String> =
            splashes.iter().map(|p| crate::util::sha1_hex(p)).collect();
        let lava = tiny_png();
        let expected_lava_sha1 = crate::util::sha1_hex(&lava);
        let damage = tiny_png();
        let expected_damage_sha1 = crate::util::sha1_hex(&damage);

        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, png.clone()),
            (SLIM_ALEX_JAR_PATH, png.clone()),
            (ZOMBIE_JAR_PATH, png.clone()),
            (FIRE_0_JAR_PATH, strip.clone()),
            (FIRE_1_JAR_PATH, strip),
            (UNDERWATER_JAR_PATH, underwater),
            (EXPERIENCE_ORB_JAR_PATH, orb),
            (PARTICLE_CRIT_JAR_PATH, crit),
            (PARTICLE_BUBBLE_JAR_PATH, bubble),
            (PARTICLE_POOF_JAR_PATHS[0], poofs[0].clone()),
            (PARTICLE_POOF_JAR_PATHS[1], poofs[1].clone()),
            (PARTICLE_POOF_JAR_PATHS[2], poofs[2].clone()),
            (PARTICLE_POOF_JAR_PATHS[3], poofs[3].clone()),
            (PARTICLE_POOF_JAR_PATHS[4], poofs[4].clone()),
            (PARTICLE_POOF_JAR_PATHS[5], poofs[5].clone()),
            (PARTICLE_POOF_JAR_PATHS[6], poofs[6].clone()),
            (PARTICLE_POOF_JAR_PATHS[7], poofs[7].clone()),
            (PARTICLE_SPLASH_JAR_PATHS[0], splashes[0].clone()),
            (PARTICLE_SPLASH_JAR_PATHS[1], splashes[1].clone()),
            (PARTICLE_SPLASH_JAR_PATHS[2], splashes[2].clone()),
            (PARTICLE_SPLASH_JAR_PATHS[3], splashes[3].clone()),
            (PARTICLE_LAVA_JAR_PATH, lava),
            (PARTICLE_DAMAGE_JAR_PATH, damage),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.side, 64);
        assert_eq!(data.png_sha1, expected_png_sha1, "source bytes verbatim");
        assert_eq!(data.rgba_sha1, expected_rgba_sha1, "decode pinned");
        assert_eq!(data.slim_png_sha1, expected_png_sha1, "the fixture doubles as the slim skin");
        assert_eq!(data.fire_0_png_sha1, expected_fire0_sha1, "the strip's own sha1");
        assert_eq!(
            data.underwater_png_sha1, expected_underwater_sha1,
            "the water screen's own sha1"
        );
        assert_eq!(
            data.experience_orb_png_sha1, expected_orb_sha1,
            "the orb sheet's own sha1"
        );
        assert_eq!(
            data.particle_crit_png_sha1, expected_crit_sha1,
            "the crit star's own sha1"
        );
        assert_eq!(
            data.particle_bubble_png_sha1, expected_bubble_sha1,
            "the drown bubble's own sha1"
        );
        assert_eq!(
            data.particle_poof_png_sha1s,
            expected_poof_sha1s.as_slice(),
            "the poof frames' own sha1s"
        );
        assert_eq!(
            data.particle_splash_png_sha1s,
            expected_splash_sha1s.as_slice(),
            "the splash frames' own sha1s"
        );
        assert_eq!(
            data.particle_lava_png_sha1, expected_lava_sha1,
            "the lava sprite's own sha1"
        );
        assert_eq!(
            data.particle_damage_png_sha1, expected_damage_sha1,
            "the damage sprite's own sha1"
        );

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains(&format!("pub const SKIN_SIDE: u32 = 64;")));
        assert!(emitted.contains(&format!("pub const STEVE_WIDE_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains(&format!("pub const SLIM_ALEX_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains("entity/player/wide/steve.png"));
        assert!(emitted.contains("entity/player/slim/alex.png"));
        // the fire strips: the sha1s and the runtime paths land verbatim
        assert!(emitted.contains(&format!("pub const FIRE_0_PNG_SHA1: &str = \"{expected_fire0_sha1}\";")));
        assert!(emitted.contains("pub const FIRE_0_RUNTIME_PATH: &str = \"block/fire_0.png\";"));
        assert!(emitted.contains("pub const FIRE_1_RUNTIME_PATH: &str = \"block/fire_1.png\";"));
        // the water screen: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const UNDERWATER_PNG_SHA1: &str = \"{expected_underwater_sha1}\";")));
        assert!(emitted.contains("pub const UNDERWATER_RUNTIME_PATH: &str = \"misc/underwater.png\";"));
        // the xp orb: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const EXPERIENCE_ORB_PNG_SHA1: &str = \"{expected_orb_sha1}\";")));
        assert!(emitted.contains("pub const EXPERIENCE_ORB_RUNTIME_PATH: &str = \"entity/experience/experience_orb.png\";"));
        // the particle lane: the crit star's + the poof frames' sha1s
        // and the runtime paths land verbatim
        assert!(emitted.contains(&format!("pub const PARTICLE_CRIT_PNG_SHA1: &str = \"{expected_crit_sha1}\";")));
        assert!(emitted.contains("pub const PARTICLE_CRIT_RUNTIME_PATH: &str = \"particle/critical_hit.png\";"));
        assert!(emitted.contains(&format!("pub const PARTICLE_BUBBLE_PNG_SHA1: &str = \"{expected_bubble_sha1}\";")));
        assert!(emitted.contains("pub const PARTICLE_BUBBLE_RUNTIME_PATH: &str = \"particle/bubble.png\";"));
        for (i, sha) in expected_poof_sha1s.iter().enumerate() {
            assert!(emitted.contains(&format!("\"{sha}\"")), "poof frame {i}'s sha1");
            assert!(emitted.contains(&format!("\"particle/generic_{i}.png\"")));
        }
        for (i, sha) in expected_splash_sha1s.iter().enumerate() {
            assert!(
                emitted.contains(&format!("\"{sha}\"")),
                "splash frame {i}'s sha1"
            );
            assert!(emitted.contains(&format!("\"particle/splash_{i}.png\"")));
        }
        assert!(emitted.contains("pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];"));
        assert!(emitted.contains("pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];"));
        // the six faces unfold from texOffs (40,16) with the box 4x12x4
        for (u, v, w, h) in arm_faces() {
            assert!(
                emitted.contains(&format!("FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}")),
                "face rect ({u},{v},{w},{h}) must be emitted verbatim"
            );
        }
        // and they stay inside the texture
        for (u, v, w, h) in arm_faces() {
            assert!(u32::from(u) + u32::from(w) <= 64);
            assert!(u32::from(v) + u32::from(h) <= 64);
        }
        // the SLIM unwrap: the same layout, box 3x12x4 — every rect inside
        // the texture, and the strip widths shift with w (front 3 wide,
        // left/inner at u+d+w, back at u+d+w+d)
        for (u, v, w, h) in arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX) {
            assert!(emitted.contains(&format!("FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}")));
            assert!(u32::from(u) + u32::from(w) <= 64);
            assert!(u32::from(v) + u32::from(h) <= 64);
        }
        assert_eq!(
            arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX),
            [
                (44, 16, 3, 4), // top
                (47, 16, 3, 4), // bottom
                (40, 20, 4, 12), // right (outer)
                (44, 20, 3, 12), // front
                (47, 20, 4, 12), // left (inner)
                (51, 20, 3, 12), // back
            ]
        );
    }

    #[test]
    fn rejects_an_undersized_skin() {
        // a 32x64 (non-square) png must be refused: the classic layout is 64x64
        let mut rgba = vec![0u8; 32 * 64 * 4];
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, 32, 64);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&mut rgba).unwrap();
        }
        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, out),
            (SLIM_ALEX_JAR_PATH, skin_png()),
            (ZOMBIE_JAR_PATH, skin_png()),
        ]);
        assert!(generate(&mut zip).is_err(), "non-square skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_slim_skin() {
        // the slim table must not be invented: a jar missing alex.png fails
        // the generation (the data-driven rule — no fabricated defaults)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (ZOMBIE_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing slim skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_zombie_skin() {
        // the zombie table must not be invented either: a jar missing
        // zombie.png fails the generation (the same data-driven rule)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (SLIM_ALEX_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing zombie skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_underwater_screen() {
        // the water screen's table must not be invented either: a jar
        // missing underwater.png fails the generation (the same
        // data-driven rule)
        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, skin_png()),
            (SLIM_ALEX_JAR_PATH, skin_png()),
            (ZOMBIE_JAR_PATH, skin_png()),
            (FIRE_0_JAR_PATH, strip_png()),
            (FIRE_1_JAR_PATH, strip_png()),
        ]);
        assert!(generate(&mut zip).is_err(), "missing underwater.png must be refused");
    }
}
