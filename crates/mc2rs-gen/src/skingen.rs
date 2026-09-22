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

/// The jar entry the PIG's temperate skin lives at (the PigRenderer's
/// variant table's default — the plain pig in the temperate biome; the
/// 26.2 pig variants replaced the single pig.png). The quadruped mesh
/// rides `LayerDefinition.create(64, 64)` (javap PigModel).
pub const PIG_TEMPERATE_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_temperate.png";

/// The jar entry the COW's temperate skin lives at (the CowRenderer's
/// variant table's default — the plain cow; the 26.2 cow variants
/// replaced the single cow.png). The bovine mesh rides
/// `LayerDefinition.create(64, 64)` (javap CowModel).
pub const COW_TEMPERATE_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_temperate.png";

/// The jar entry the SHEEP's skin lives at (the SheepRenderer's
/// `getTextureLocation` — the plain sheep carries NO biome variants,
/// the single sheep.png the 26.2 jar keeps; the wool overlay's
/// sheep_wool.png is the fur layer's own texture the body's render
/// never reads). The ovine mesh rides `LayerDefinition.create(64, 32)`
/// (javap SheepModel — the HALF-HEIGHT layout the 64-texel-wide
/// quadrupeds' 64x64 replaced).
pub const SHEEP_JAR_PATH: &str = "assets/minecraft/textures/entity/sheep/sheep.png";

/// The jar entry the SHEEP's WOOL lives at (the SheepWoolLayer's
/// SHEEP_WOOL_LOCATION — the fur overlay's texture the layer tints by
/// the DyeColor's textureDiffuseColor the clinit's own ints speak).
pub const SHEEP_WOOL_JAR_PATH: &str = "assets/minecraft/textures/entity/sheep/sheep_wool.png";

/// The jar entry the SHEEP's WOOL UNDERCOAT lives at (the
/// SheepWoolUndercoatLayer's SHEEP_WOOL_UNDERCOAT_LOCATION static init —
/// `textures/entity/sheep/sheep_wool_undercoat.png`, the body-shaped
/// undercoat the sheared dyed sheep keeps showing: the layer's own gate
/// renders it whenever the sheep is NOT white (the DyeColor.WHITE
/// if_acmpeq the gate's bytecode speaks) — no isSheared read anywhere,
/// the sheared dyed coat the wool fur's removal never erased). The
/// mesh rides `SheepModel.createBodyLayer()` (javap LayerDefinitions:
/// the SHEEP_WOOL_UNDERCOAT entry bakes the BODY layer, NOT a
/// SheepFurModel.createFurLayer inflation — the undercoat hugs the
/// body's own shape the base skin draws).
pub const SHEEP_WOOL_UNDERCOAT_JAR_PATH: &str =
    "assets/minecraft/textures/entity/sheep/sheep_wool_undercoat.png";

/// The jar entries the BABY skins live at (the AgeableMobRenderer's
/// dedicated baby lane — the renderer's `babyModel` field the isBaby
/// word picks, javap SheepRenderer/CowRenderer/PigRenderer: the
/// `textures/entity/<kind>/<skin>_baby.png` files the baby models
/// unwrap — BabySheepModel.createBodyLayer's `LayerDefinition.create
/// (64, 32)`, BabyCowModel's `create(64, 64)`, BabyPigModel's
/// `create(32, 32)`, plus the SheepWoolLayer's SHEEP_WOOL_BABY_LOCATION
/// the `sheep_wool_baby.png` the baby's wool tints).
pub const SHEEP_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/sheep/sheep_baby.png";
pub const SHEEP_WOOL_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/sheep/sheep_wool_baby.png";
pub const COW_TEMPERATE_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_temperate_baby.png";
pub const PIG_TEMPERATE_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_temperate_baby.png";

/// The jar entry the CHICKEN's temperate skin lives at (the
/// ChickenRenderer's variant table's default — the plain hen; the 26.2
/// chicken variants replaced the single chicken.png). The avian mesh
/// rides `LayerDefinition.create(mesh, 64, 32)` (javap
/// AdultChickenModel.createBodyLayer — the HALF-HEIGHT layout).
pub const CHICKEN_TEMPERATE_JAR_PATH: &str =
    "assets/minecraft/textures/entity/chicken/chicken_temperate.png";

/// The jar entry the CHICKEN's baby skin lives at (the
/// ChickenRenderer's AdultAndBabyModelPair's baby arm —
/// `chicken_temperate_baby.png`). The chick's own mesh rides
/// `LayerDefinition.create(mesh, 16, 16)` (javap BabyChickenModel.
/// createBodyLayer — the SQUARE 16x16 layout, the smallest the baby
/// lanes keep).
pub const CHICKEN_TEMPERATE_BABY_JAR_PATH: &str =
    "assets/minecraft/textures/entity/chicken/chicken_temperate_baby.png";

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
/// The torch flame's single sprite (javap: the `flame` particle type's
/// sprite set — the jar's `particles/flame.json` lists ONE texture,
/// `minecraft:flame`, the 8x8 the IHDR verifies; the
/// `FlameParticle$Provider`'s `SpriteSet.get(random)` returns it
/// whatever the draw — the nextInt(1) the one-element list speaks).
pub const PARTICLE_FLAME_JAR_PATH: &str = "assets/minecraft/textures/particle/flame.png";
/// The damage indicator's single sprite (javap: the `damage_indicator`
/// particle type's sprite set — the jar's `particles/damage_indicator.json`
/// lists ONE texture, `minecraft:damage`, the 8x8 the IHDR verifies; the
/// `CritParticle$DamageIndicatorProvider`'s `SpriteSet.get(random)`
/// returns it whatever the draw).
pub const PARTICLE_DAMAGE_JAR_PATH: &str = "assets/minecraft/textures/particle/damage.png";
/// The heart's single sprite (javap: the `heart` particle type's sprite
/// set — the jar's `particles/heart.json` lists ONE texture,
/// `minecraft:heart`, the 8x8 the IHDR verifies; the
/// `HeartParticle$Provider`'s `SpriteSet.get(random)` returns it
/// whatever the draw — the love-mode burst the event 18 broadcast
/// speaks, the ambient drip the aiStep walks).
pub const PARTICLE_HEART_JAR_PATH: &str = "assets/minecraft/textures/particle/heart.png";
/// The happy villager's sparkle sprite (javap: the `happy_villager`
/// particle type's sprite set — the jar's `particles/happy_villager.json`
/// lists ONE texture, `minecraft:glint`, the 8x8 the IHDR verifies; the
/// `SuspendedTownParticle$HappyVillagerProvider`'s `SpriteSet.get(random)`
/// returns it whatever the draw — the feed sparkles the
/// `AgeableMob.aiStep`'s forcedAgeTimer walk spawns).
pub const PARTICLE_GLINT_JAR_PATH: &str = "assets/minecraft/textures/particle/glint.png";
/// The sweep attack's 8-frame arc (javap: the `sweep_attack` particle
/// type's sprite set — the jar's `particles/sweep_attack.json` lists
/// EIGHT textures, `minecraft:sweep_0..7` in ASCENDING order — the
/// index the `MutableSpriteSet.get` walk reads IS the file index, the
/// poof json's descending list the mirror image; each frame the 32x32
/// the IHDR verifies; the `AttackSweepParticle`'s `setSpriteFromAge`
/// rides the list the json speaks).
pub const PARTICLE_SWEEP_JAR_PATHS: [&str; 8] = [
    "assets/minecraft/textures/particle/sweep_0.png",
    "assets/minecraft/textures/particle/sweep_1.png",
    "assets/minecraft/textures/particle/sweep_2.png",
    "assets/minecraft/textures/particle/sweep_3.png",
    "assets/minecraft/textures/particle/sweep_4.png",
    "assets/minecraft/textures/particle/sweep_5.png",
    "assets/minecraft/textures/particle/sweep_6.png",
    "assets/minecraft/textures/particle/sweep_7.png",
];

/// THE VARIANT TABLE'S JAR ENTRIES (the regen-gap closure): the skins
/// the earlier rounds hand-added to the GENERATED file — the generator's
/// emit template never learned them, so every regeneration silently
/// dropped the set. Each entry the jar's own PNG carries (the sha1 the
/// emit pins), the runtime path the client's loader reads.
pub const SKELETON_JAR_PATH: &str = "assets/minecraft/textures/entity/skeleton/skeleton.png";
pub const SPIDER_JAR_PATH: &str = "assets/minecraft/textures/entity/spider/spider.png";
pub const CREEPER_JAR_PATH: &str = "assets/minecraft/textures/entity/creeper/creeper.png";
pub const CHICKEN_COLD_JAR_PATH: &str = "assets/minecraft/textures/entity/chicken/chicken_cold.png";
pub const CHICKEN_COLD_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/chicken/chicken_cold_baby.png";
pub const CHICKEN_WARM_JAR_PATH: &str = "assets/minecraft/textures/entity/chicken/chicken_warm.png";
pub const CHICKEN_WARM_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/chicken/chicken_warm_baby.png";
pub const PIG_COLD_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_cold.png";
pub const PIG_COLD_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_cold_baby.png";
pub const PIG_WARM_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_warm.png";
pub const PIG_WARM_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/pig/pig_warm_baby.png";
pub const COW_COLD_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_cold.png";
pub const COW_COLD_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_cold_baby.png";
pub const COW_WARM_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_warm.png";
pub const COW_WARM_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/cow/cow_warm_baby.png";
pub const WOLF_JAR_PATH: &str = "assets/minecraft/textures/entity/wolf/wolf.png";
pub const WOLF_BABY_JAR_PATH: &str = "assets/minecraft/textures/entity/wolf/wolf_baby.png";
/// The runtime-cache paths the client's loader reads (the same
/// textures/ prefix strip the other lanes speak).
/// THE VARIANT TABLE'S RUNTIME PATHS (the client's loader reads them
/// under the runtime asset root — the same entity/ prefix the mob
/// skins' lanes speak).
pub const SKELETON_RUNTIME_PATH: &str = "entity/skeleton/skeleton.png";
pub const SPIDER_RUNTIME_PATH: &str = "entity/spider/spider.png";
pub const CREEPER_RUNTIME_PATH: &str = "entity/creeper/creeper.png";
pub const CHICKEN_COLD_RUNTIME_PATH: &str = "entity/chicken/chicken_cold.png";
pub const CHICKEN_COLD_BABY_RUNTIME_PATH: &str = "entity/chicken/chicken_cold_baby.png";
pub const CHICKEN_WARM_RUNTIME_PATH: &str = "entity/chicken/chicken_warm.png";
pub const CHICKEN_WARM_BABY_RUNTIME_PATH: &str = "entity/chicken/chicken_warm_baby.png";
pub const PIG_COLD_RUNTIME_PATH: &str = "entity/pig/pig_cold.png";
pub const PIG_COLD_BABY_RUNTIME_PATH: &str = "entity/pig/pig_cold_baby.png";
pub const PIG_WARM_RUNTIME_PATH: &str = "entity/pig/pig_warm.png";
pub const PIG_WARM_BABY_RUNTIME_PATH: &str = "entity/pig/pig_warm_baby.png";
pub const COW_COLD_RUNTIME_PATH: &str = "entity/cow/cow_cold.png";
pub const COW_COLD_BABY_RUNTIME_PATH: &str = "entity/cow/cow_cold_baby.png";
pub const COW_WARM_RUNTIME_PATH: &str = "entity/cow/cow_warm.png";
pub const COW_WARM_BABY_RUNTIME_PATH: &str = "entity/cow/cow_warm_baby.png";
pub const WOLF_RUNTIME_PATH: &str = "entity/wolf/wolf.png";
pub const WOLF_BABY_RUNTIME_PATH: &str = "entity/wolf/wolf_baby.png";
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
/// The torch flame's runtime-cache path (the same textures/ prefix
/// strip the other lanes speak).
pub const PARTICLE_FLAME_RUNTIME_PATH: &str = "particle/flame.png";
pub const PARTICLE_DAMAGE_RUNTIME_PATH: &str = "particle/damage.png";
/// The heart's runtime-cache path (the same textures/ prefix strip the
/// other lanes speak).
pub const PARTICLE_HEART_RUNTIME_PATH: &str = "particle/heart.png";
pub const PARTICLE_GLINT_RUNTIME_PATH: &str = "particle/glint.png";
/// The sweep arc's runtime-cache paths (the same textures/ prefix
/// strip the other lanes speak — the json's ascending list IS the
/// array order).
pub const PARTICLE_SWEEP_RUNTIME_PATHS: [&str; 8] = [
    "particle/sweep_0.png",
    "particle/sweep_1.png",
    "particle/sweep_2.png",
    "particle/sweep_3.png",
    "particle/sweep_4.png",
    "particle/sweep_5.png",
    "particle/sweep_6.png",
    "particle/sweep_7.png",
];
/// Where the client reads the skin at runtime (under the runtime asset root).
pub const RUNTIME_PATH: &str = "entity/player/wide/steve.png";
pub const SLIM_RUNTIME_PATH: &str = "entity/player/slim/alex.png";
pub const ZOMBIE_RUNTIME_PATH: &str = "entity/zombie/zombie.png";
pub const PIG_TEMPERATE_RUNTIME_PATH: &str = "entity/pig/pig_temperate.png";
pub const COW_TEMPERATE_RUNTIME_PATH: &str = "entity/cow/cow_temperate.png";
pub const SHEEP_RUNTIME_PATH: &str = "entity/sheep/sheep.png";
pub const SHEEP_WOOL_RUNTIME_PATH: &str = "entity/sheep/sheep_wool.png";
pub const SHEEP_WOOL_UNDERCOAT_RUNTIME_PATH: &str = "entity/sheep/sheep_wool_undercoat.png";
pub const SHEEP_BABY_RUNTIME_PATH: &str = "entity/sheep/sheep_baby.png";
pub const SHEEP_WOOL_BABY_RUNTIME_PATH: &str = "entity/sheep/sheep_wool_baby.png";
pub const COW_TEMPERATE_BABY_RUNTIME_PATH: &str = "entity/cow/cow_temperate_baby.png";
pub const PIG_TEMPERATE_BABY_RUNTIME_PATH: &str = "entity/pig/pig_temperate_baby.png";
pub const CHICKEN_TEMPERATE_RUNTIME_PATH: &str = "entity/chicken/chicken_temperate.png";
pub const CHICKEN_TEMPERATE_BABY_RUNTIME_PATH: &str = "entity/chicken/chicken_temperate_baby.png";

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
#[expect(dead_code)] // the slim box's own origin — the generator's local copy; the emitted client line the string lane keeps
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
    /// sha1 of the pig's temperate source PNG bytes (jar-verbatim
    /// integrity) — the quadruped's texture the pig renderer verifies.
    pub pig_png_sha1: String,
    /// sha1 of the cow's temperate source PNG bytes (jar-verbatim
    /// integrity) — the bovine's texture the cow renderer verifies.
    pub cow_png_sha1: String,
    /// sha1 of the sheep's source PNG bytes (jar-verbatim integrity) —
    /// the ovine's texture the sheep renderer verifies.
    pub sheep_png_sha1: String,
    /// sha1 of the sheep's WOOL source PNG bytes (jar-verbatim
    /// integrity) — the fur overlay's texture the wool layer tints.
    pub sheep_wool_png_sha1: String,
    /// sha1 of the sheep's WOOL UNDERCOAT source PNG bytes (jar-verbatim
    /// integrity) — the body-shaped undercoat the undercoat layer tints
    /// (the sheared dyed sheep's own coat the wool fur's removal never
    /// erased).
    pub sheep_wool_undercoat_png_sha1: String,
    /// sha1s of the BABY skins (jar-verbatim integrity) — the
    /// dedicated baby lane's textures the baby models unwrap (the
    /// AgeableMobRenderer's isBaby word the model swap arms, the
    /// SheepWoolLayer's baby arm the wool_baby tints).
    pub sheep_baby_png_sha1: String,
    pub sheep_wool_baby_png_sha1: String,
    pub cow_baby_png_sha1: String,
    pub pig_baby_png_sha1: String,
    /// sha1 of the CHICKEN's temperate source PNG bytes (jar-verbatim
    /// integrity) — the avian's texture the chicken renderer verifies.
    pub chicken_png_sha1: String,
    /// sha1 of the chicken's baby source PNG bytes (jar-verbatim
    /// integrity) — the chick's texture the baby lane verifies.
    pub chicken_baby_png_sha1: String,
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
    /// sha1 of the torch flame's single 8x8 sprite (jar-verbatim
    /// integrity) — the animateTick pair's flame quad's pixel source.
    pub particle_flame_png_sha1: String,
    /// sha1 of the damage indicator's single 8x8 sprite (jar-verbatim
    /// integrity) — the hit's own burst's pixel source.
    pub particle_damage_png_sha1: String,
    pub particle_heart_png_sha1: String,
    /// sha1 of the happy villager's single 8x8 sprite (jar-verbatim
    /// integrity) — the feed sparkle's own green pixel source.
    pub particle_glint_png_sha1: String,
    /// sha1s of the sweep arc's 8 frames (jar-verbatim integrity) —
    /// the sword's own swing's pixel sources (the 32x32 sweep frames
    /// the json's ascending list orders).
    pub particle_sweep_png_sha1s: [String; 8],
    /// THE VARIANT TABLE'S SHA1S (jar-verbatim integrity): the skins
    /// the variant lanes verify before they draw — the thin/arachnid/
    /// canine pair + the 26.2 temperature variants' adult/baby rows
    /// (each the OWN layout the model family's LayerDefinition keeps).
    pub skeleton_png_sha1: String,
    pub spider_png_sha1: String,
    pub creeper_png_sha1: String,
    pub chicken_cold_png_sha1: String,
    pub chicken_cold_baby_png_sha1: String,
    pub chicken_warm_png_sha1: String,
    pub chicken_warm_baby_png_sha1: String,
    pub pig_cold_png_sha1: String,
    pub pig_cold_baby_png_sha1: String,
    pub pig_warm_png_sha1: String,
    pub pig_warm_baby_png_sha1: String,
    pub cow_cold_png_sha1: String,
    pub cow_cold_baby_png_sha1: String,
    pub cow_warm_png_sha1: String,
    pub cow_warm_baby_png_sha1: String,
    pub wolf_png_sha1: String,
    pub wolf_baby_png_sha1: String,
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
    // THE PIG SKIN (the second mob's own): the temperate variant the
    // PigRenderer's variant table defaults to (the 26.2 pig variants
    // replaced the single pig.png). The quadruped mesh rides
    // `LayerDefinition.create(64, 64)` (javap PigModel) — the same
    // 64-texel square, its own UV layout the pig model unwraps. The
    // integrity sha1 the pig lane verifies before it draws.
    let pig_bytes = read_entry(archive, PIG_TEMPERATE_JAR_PATH).context("read the pig temperate skin")?;
    let pig_png_sha1 = crate::util::sha1_hex(&pig_bytes);
    let (pig_w, pig_h, _pig_rgba) = decode_rgba(&pig_bytes).context("decode the pig temperate skin")?;
    anyhow::ensure!(
        (pig_w, pig_h) == (64, 64),
        "the pig skin must be the 64x64 layout (javap LayerDefinition.create(64, 64)), got {}x{}",
        pig_w,
        pig_h
    );
    std::mem::drop(_pig_rgba);
    // THE COW SKIN (the third mob's own): the temperate variant the
    // CowRenderer's variant table defaults to (the 26.2 cow variants
    // replaced the single cow.png). The bovine mesh rides
    // `LayerDefinition.create(64, 64)` (javap CowModel) — the same
    // 64-texel square, its own UV layout the cow model unwraps. The
    // integrity sha1 the cow lane verifies before it draws.
    let cow_bytes = read_entry(archive, COW_TEMPERATE_JAR_PATH).context("read the cow temperate skin")?;
    let cow_png_sha1 = crate::util::sha1_hex(&cow_bytes);
    let (cow_w, cow_h, _cow_rgba) = decode_rgba(&cow_bytes).context("decode the cow temperate skin")?;
    anyhow::ensure!(
        (cow_w, cow_h) == (64, 64),
        "the cow skin must be the 64x64 layout (javap LayerDefinition.create(64, 64)), got {}x{}",
        cow_w,
        cow_h
    );
    std::mem::drop(_cow_rgba);
    // THE SHEEP SKIN (the fourth mob's own): the single sheep.png the
    // SheepRenderer's getTextureLocation speaks — no biome variants
    // (the pig/cow's temperate tables the sheep never grew). The ovine
    // mesh rides `LayerDefinition.create(64, 32)` (javap SheepModel) —
    // the HALF-HEIGHT layout, its own UV layout the sheep model
    // unwraps. The integrity sha1 the sheep lane verifies before it
    // draws.
    let sheep_bytes = read_entry(archive, SHEEP_JAR_PATH).context("read the sheep skin")?;
    let sheep_png_sha1 = crate::util::sha1_hex(&sheep_bytes);
    let (sheep_w, sheep_h, _sheep_rgba) = decode_rgba(&sheep_bytes).context("decode the sheep skin")?;
    anyhow::ensure!(
        (sheep_w, sheep_h) == (64, 32),
        "the sheep skin must be the 64x32 layout (javap LayerDefinition.create(64, 32)), got {}x{}",
        sheep_w,
        sheep_h
    );
    std::mem::drop(_sheep_rgba);
    // THE SHEEP WOOL (the fur overlay's own texture — the
    // SheepWoolLayer's SHEEP_WOOL_LOCATION): the 64x32 greyscale the
    // layer tints by the dye color. The integrity sha1 the wool lane
    // verifies before it draws.
    let sheep_wool_bytes = read_entry(archive, SHEEP_WOOL_JAR_PATH).context("read the sheep wool skin")?;
    let sheep_wool_png_sha1 = crate::util::sha1_hex(&sheep_wool_bytes);
    let (wool_w, wool_h, _wool_rgba) = decode_rgba(&sheep_wool_bytes).context("decode the sheep wool skin")?;
    anyhow::ensure!(
        (wool_w, wool_h) == (64, 32),
        "the sheep wool must be the 64x32 layout (javap SheepFurModel LayerDefinition.create(64, 32)), got {}x{}",
        wool_w,
        wool_h
    );
    std::mem::drop(_wool_rgba);
    // THE SHEEP WOOL UNDERCOAT (the SheepWoolUndercoatLayer's own
    // texture — the body-shaped coat the sheared dyed sheep keeps): the
    // same 64x32 layout the body layer bakes (javap LayerDefinitions:
    // SHEEP_WOOL_UNDERCOAT = SheepModel.createBodyLayer()). The
    // integrity sha1 the undercoat lane verifies before it draws.
    let sheep_wool_undercoat_bytes =
        read_entry(archive, SHEEP_WOOL_UNDERCOAT_JAR_PATH).context("read the sheep wool undercoat skin")?;
    let sheep_wool_undercoat_png_sha1 = crate::util::sha1_hex(&sheep_wool_undercoat_bytes);
    let (ucw, uch, _undercoat_rgba) =
        decode_rgba(&sheep_wool_undercoat_bytes).context("decode the sheep wool undercoat skin")?;
    anyhow::ensure!(
        (ucw, uch) == (64, 32),
        "the sheep wool undercoat must be the 64x32 layout (the SHEEP_WOOL_UNDERCOAT layer bakes the body), got {}x{}",
        ucw,
        uch
    );
    std::mem::drop(_undercoat_rgba);
    // THE BABY SKINS (the dedicated baby lane's own textures — the
    // AgeableMobRenderer's isBaby word the model swap arms): the baby
    // sheep and its wool ride the 64x32 layout the BabySheepModel's
    // create(64, 32) speaks, the baby cow the 64x64 the BabyCowModel
    // speaks, the baby pig the 32x32 the BabyPigModel speaks — each
    // its OWN file the renderer's baby lane unwraps (javap
    // SheepRenderer's 'textures/entity/sheep/sheep_baby.png' + the
    // SheepWoolLayer's SHEEP_WOOL_BABY_LOCATION, the variant tables'
    // _baby entries the cow/pig renderers speak). The integrity sha1s
    // the baby lanes verify before they draw.
    let sheep_baby_bytes = read_entry(archive, SHEEP_BABY_JAR_PATH).context("read the sheep baby skin")?;
    let sheep_baby_png_sha1 = crate::util::sha1_hex(&sheep_baby_bytes);
    let (sby_w, sby_h, _sheep_baby_rgba) = decode_rgba(&sheep_baby_bytes).context("decode the sheep baby skin")?;
    anyhow::ensure!(
        (sby_w, sby_h) == (64, 32),
        "the sheep baby skin must be the 64x32 layout (javap BabySheepModel LayerDefinition.create(64, 32)), got {}x{}",
        sby_w,
        sby_h
    );
    std::mem::drop(_sheep_baby_rgba);
    let sheep_wool_baby_bytes = read_entry(archive, SHEEP_WOOL_BABY_JAR_PATH).context("read the sheep wool baby skin")?;
    let sheep_wool_baby_png_sha1 = crate::util::sha1_hex(&sheep_wool_baby_bytes);
    let (wbw, wbh, _wool_baby_rgba) = decode_rgba(&sheep_wool_baby_bytes).context("decode the sheep wool baby skin")?;
    anyhow::ensure!(
        (wbw, wbh) == (64, 32),
        "the sheep wool baby must be the 64x32 layout (the SHEEP_BABY_WOOL layer bakes the BabySheepModel body), got {}x{}",
        wbw,
        wbh
    );
    std::mem::drop(_wool_baby_rgba);
    let cow_baby_bytes = read_entry(archive, COW_TEMPERATE_BABY_JAR_PATH).context("read the cow temperate baby skin")?;
    let cow_baby_png_sha1 = crate::util::sha1_hex(&cow_baby_bytes);
    let (cbw, cbh, _cow_baby_rgba) = decode_rgba(&cow_baby_bytes).context("decode the cow temperate baby skin")?;
    anyhow::ensure!(
        (cbw, cbh) == (64, 64),
        "the cow baby skin must be the 64x64 layout (javap BabyCowModel LayerDefinition.create(64, 64)), got {}x{}",
        cbw,
        cbh
    );
    std::mem::drop(_cow_baby_rgba);
    let pig_baby_bytes = read_entry(archive, PIG_TEMPERATE_BABY_JAR_PATH).context("read the pig temperate baby skin")?;
    let pig_baby_png_sha1 = crate::util::sha1_hex(&pig_baby_bytes);
    let (pbw, pbh, _pig_baby_rgba) = decode_rgba(&pig_baby_bytes).context("decode the pig temperate baby skin")?;
    anyhow::ensure!(
        (pbw, pbh) == (32, 32),
        "the pig baby skin must be the 32x32 layout (javap BabyPigModel LayerDefinition.create(32, 32)), got {}x{}",
        pbw,
        pbh
    );
    std::mem::drop(_pig_baby_rgba);
    // THE CHICKEN SKIN (the fifth mob's own): the temperate variant the
    // ChickenRenderer's variant table defaults to (the 26.2 chicken
    // variants replaced the single chicken.png). The avian mesh rides
    // `LayerDefinition.create(mesh, 64, 32)` (javap
    // AdultChickenModel.createBodyLayer) — the HALF-HEIGHT layout, its
    // own UV layout the chicken model unwraps. The integrity sha1 the
    // chicken lane verifies before it draws.
    let chicken_bytes =
        read_entry(archive, CHICKEN_TEMPERATE_JAR_PATH).context("read the chicken temperate skin")?;
    let chicken_png_sha1 = crate::util::sha1_hex(&chicken_bytes);
    let (chw, chh, _chicken_rgba) = decode_rgba(&chicken_bytes).context("decode the chicken temperate skin")?;
    anyhow::ensure!(
        (chw, chh) == (64, 32),
        "the chicken skin must be the 64x32 layout (javap AdultChickenModel LayerDefinition.create(64, 32)), got {}x{}",
        chw,
        chh
    );
    std::mem::drop(_chicken_rgba);
    let chicken_baby_bytes = read_entry(archive, CHICKEN_TEMPERATE_BABY_JAR_PATH)
        .context("read the chicken temperate baby skin")?;
    let chicken_baby_png_sha1 = crate::util::sha1_hex(&chicken_baby_bytes);
    let (cbw2, cbh2, _chicken_baby_rgba) = decode_rgba(&chicken_baby_bytes)
        .context("decode the chicken temperate baby skin")?;
    anyhow::ensure!(
        (cbw2, cbh2) == (16, 16),
        "the chicken baby skin must be the 16x16 layout (javap BabyChickenModel LayerDefinition.create(16, 16)), got {}x{}",
        cbw2,
        cbh2
    );
    std::mem::drop(_chicken_baby_rgba);
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
    // the torch flame's single sprite (the same shape check the lava
    // speaks — the json's one-texture set the animateTick pair's
    // flame quad banks)
    let flame_bytes = read_entry(archive, PARTICLE_FLAME_JAR_PATH).context("read flame")?;
    let particle_flame_png_sha1 = crate::util::sha1_hex(&flame_bytes);
    let (fw, fh, _flame_rgba) = decode_rgba(&flame_bytes).context("decode flame")?;
    anyhow::ensure!(
        (fw, fh) == (8, 8),
        "flame must be the 8x8 sprite, got {}x{}",
        fw,
        fh
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
    // the heart's single sprite (the same shape check the damage
    // speaks — the json's one-texture set, the love burst's own red)
    let heart_bytes = read_entry(archive, PARTICLE_HEART_JAR_PATH).context("read heart")?;
    let particle_heart_png_sha1 = crate::util::sha1_hex(&heart_bytes);
    let (hw2, hh2, _heart_rgba) = decode_rgba(&heart_bytes).context("decode heart")?;
    anyhow::ensure!(
        (hw2, hh2) == (8, 8),
        "heart must be the 8x8 sprite, got {}x{}",
        hw2,
        hh2
    );
    // the glint's single sprite (the same shape check the heart
    // speaks — the happy_villager json's one-texture set, the feed
    // sparkle's own green the setColor(1,1,1) passes whole)
    let glint_bytes = read_entry(archive, PARTICLE_GLINT_JAR_PATH).context("read glint")?;
    let particle_glint_png_sha1 = crate::util::sha1_hex(&glint_bytes);
    let (gw2, gh2, _glint_rgba) = decode_rgba(&glint_bytes).context("decode glint")?;
    anyhow::ensure!(
        (gw2, gh2) == (8, 8),
        "glint must be the 8x8 sprite, got {}x{}",
        gw2,
        gh2
    );
    // the sweep arc's 8 frames (the same shape check the poof walk
    // speaks — the 32x32 the arc quad samples over its own life)
    let mut particle_sweep_png_sha1s: [String; 8] = std::array::from_fn(|_| String::new());
    for (i, path) in PARTICLE_SWEEP_JAR_PATHS.iter().enumerate() {
        let bytes = read_entry(archive, path).context("read sweep frame")?;
        particle_sweep_png_sha1s[i] = crate::util::sha1_hex(&bytes);
        let (sw, sh, _sweep_rgba) = decode_rgba(&bytes).context("decode sweep frame")?;
        anyhow::ensure!(
            (sw, sh) == (32, 32),
            "sweep frames must be the 32x32 sprites, got {}x{}",
            sw,
            sh
        );
    }
    // THE VARIANT TABLE READS (the regen-gap closure): each entry the
    // jar's own PNG carries, the sha1 the emit pins, the layout the
    // model family's LayerDefinition.create keeps — the thin humanoid's
    // 64x32, the arachnid's 64x32, the canine's 64x32 adult + 32x32
    // baby, the avian variants' 64x32 adult + 16x16 baby, the swine/
    // bovine variants' 64x64 adult + 32x32 baby (the dims the committed
    // ledger's doc rows speak).
    let variant_reads: [(&str, (u32, u32)); 17] = [
        (SKELETON_JAR_PATH, (64, 32)),
        (SPIDER_JAR_PATH, (64, 32)),
        (CHICKEN_COLD_JAR_PATH, (64, 32)),
        (CHICKEN_COLD_BABY_JAR_PATH, (16, 16)),
        (CHICKEN_WARM_JAR_PATH, (64, 32)),
        (CHICKEN_WARM_BABY_JAR_PATH, (16, 16)),
        (PIG_COLD_JAR_PATH, (64, 64)),
        (PIG_COLD_BABY_JAR_PATH, (32, 32)),
        (PIG_WARM_JAR_PATH, (64, 64)),
        (PIG_WARM_BABY_JAR_PATH, (32, 32)),
        (COW_COLD_JAR_PATH, (64, 64)),
        // the cold/warm CALVES the 64x64 container keeps (the 32x32 UV
        // layout the padded tile never shrinks — the jar's own bytes
        // the check verifies)
        (COW_COLD_BABY_JAR_PATH, (64, 64)),
        (COW_WARM_JAR_PATH, (64, 64)),
        (COW_WARM_BABY_JAR_PATH, (64, 64)),
        (WOLF_JAR_PATH, (64, 32)),
        (WOLF_BABY_JAR_PATH, (32, 32)),
        // THE CREEPER'S OWN SKIN (the CreeperRenderer's CREEPER_LOCATION
        // — the 64x32 layout the CreeperModel's own LayerDefinition
        // keeps, the clinit's withDefaultNamespace the path names)
        (CREEPER_JAR_PATH, (64, 32)),
    ];
    let mut variant_png_sha1s: Vec<String> = Vec::with_capacity(16);
    for (path, (ew, eh)) in &variant_reads {
        let bytes = read_entry(archive, path)
            .with_context(|| format!("read the variant skin {path}"))?;
        let (vw, vh, _v_rgba) = decode_rgba(&bytes)
            .with_context(|| format!("decode the variant skin {path}"))?;
        anyhow::ensure!(
            (vw, vh) == (*ew, *eh),
            "the variant skin {path} must be the {ew}x{eh} layout, got {vw}x{vh}"
        );
        variant_png_sha1s.push(crate::util::sha1_hex(&bytes));
    }
    let skeleton_png_sha1 = variant_png_sha1s[0].clone();
    let spider_png_sha1 = variant_png_sha1s[1].clone();
    let chicken_cold_png_sha1 = variant_png_sha1s[2].clone();
    let chicken_cold_baby_png_sha1 = variant_png_sha1s[3].clone();
    let chicken_warm_png_sha1 = variant_png_sha1s[4].clone();
    let chicken_warm_baby_png_sha1 = variant_png_sha1s[5].clone();
    let pig_cold_png_sha1 = variant_png_sha1s[6].clone();
    let pig_cold_baby_png_sha1 = variant_png_sha1s[7].clone();
    let pig_warm_png_sha1 = variant_png_sha1s[8].clone();
    let pig_warm_baby_png_sha1 = variant_png_sha1s[9].clone();
    let cow_cold_png_sha1 = variant_png_sha1s[10].clone();
    let cow_cold_baby_png_sha1 = variant_png_sha1s[11].clone();
    let cow_warm_png_sha1 = variant_png_sha1s[12].clone();
    let cow_warm_baby_png_sha1 = variant_png_sha1s[13].clone();
    let wolf_png_sha1 = variant_png_sha1s[14].clone();
    let wolf_baby_png_sha1 = variant_png_sha1s[15].clone();
    let creeper_png_sha1 = variant_png_sha1s[16].clone();
    Ok(SkinData {
        png_sha1,
        rgba_sha1: crate::util::sha1_hex(&rgba),
        side,
        slim_png_sha1,
        slim_rgba_sha1: crate::util::sha1_hex(&slim_rgba),
        zombie_png_sha1,
        pig_png_sha1,
        cow_png_sha1,
        sheep_png_sha1,
        sheep_wool_png_sha1,
        sheep_wool_undercoat_png_sha1,
        sheep_baby_png_sha1,
        sheep_wool_baby_png_sha1,
        cow_baby_png_sha1,
        pig_baby_png_sha1,
        chicken_png_sha1,
        chicken_baby_png_sha1,
        fire_0_png_sha1,
        fire_1_png_sha1,
        underwater_png_sha1,
        experience_orb_png_sha1,
        particle_crit_png_sha1,
        particle_bubble_png_sha1,
        particle_poof_png_sha1s,
        particle_splash_png_sha1s,
        particle_lava_png_sha1,
        particle_flame_png_sha1,
        particle_damage_png_sha1,
        particle_heart_png_sha1,
        particle_glint_png_sha1,
        particle_sweep_png_sha1s,
        skeleton_png_sha1,
        spider_png_sha1,
        creeper_png_sha1,
        chicken_cold_png_sha1,
        chicken_cold_baby_png_sha1,
        chicken_warm_png_sha1,
        chicken_warm_baby_png_sha1,
        pig_cold_png_sha1,
        pig_cold_baby_png_sha1,
        pig_warm_png_sha1,
        pig_warm_baby_png_sha1,
        cow_cold_png_sha1,
        cow_cold_baby_png_sha1,
        cow_warm_png_sha1,
        cow_warm_baby_png_sha1,
        wolf_png_sha1,
        wolf_baby_png_sha1,
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
    out.push_str("\n/// The SKELETON skin (the thin humanoid's own), generated from the\n/// same jar: the source sha1 the avatar renderer verifies before\n/// drawing (javap: `AbstractSkeletonRenderer`'s SKELETON_LOCATION —\n/// the SkeletonModel's own `LayerDefinition.create(64, 32)` the\n/// bytecode speaks — the HALF-HEIGHT layout the thin boxes unwrap).\n");
    out.push_str(&format!(
        "pub const SKELETON_PNG_SHA1: &str = \"{}\";\n",
        data.skeleton_png_sha1
    ));
    out.push_str(&format!(
        "pub const SKELETON_RUNTIME_PATH: &str = \"{SKELETON_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The SPIDER skin (the arachnid's own), generated from the\n/// same jar: the source sha1 the spider renderer verifies before\n/// drawing (javap: `SpiderRenderer`'s SPIDER_LOCATION — the\n/// SpiderModel's own `LayerDefinition.create(mesh, 64, 32)` the\n/// bytecode speaks — the half-height layout the eight-leg chain\n/// unwraps).\n");
    out.push_str(&format!(
        "pub const SPIDER_PNG_SHA1: &str = \"{}\";\n",
        data.spider_png_sha1
    ));
    out.push_str(&format!(
        "pub const SPIDER_RUNTIME_PATH: &str = \"{SPIDER_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The CREEPER skin (the charged-green body's own), generated from the\n/// same jar: the source sha1 the creeper renderer verifies before\n/// drawing (javap: `CreeperRenderer`'s CREEPER_LOCATION — the\n/// `textures/entity/creeper/creeper.png` the clinit's\n/// withDefaultNamespace names, the CreeperModel's own\n/// `LayerDefinition.create(mesh, 64, 32)` the bytecode speaks — the\n/// half-height layout the four-leg chain unwraps).\n");
    out.push_str(&format!(
        "pub const CREEPER_PNG_SHA1: &str = \"{}\";\n",
        data.creeper_png_sha1
    ));
    out.push_str(&format!(
        "pub const CREEPER_RUNTIME_PATH: &str = \"{CREEPER_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The PIG skin (the temperate variant the PigRenderer's variant\n/// table defaults to), generated from the same jar: the source sha1\n/// the pig renderer verifies before it draws (javap PigModel:\n/// `LayerDefinition.create(64, 64)` — the quadruped's own UV layout).\n");
    out.push_str(&format!(
        "pub const PIG_TEMPERATE_PNG_SHA1: &str = \"{}\";\n",
        data.pig_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_TEMPERATE_RUNTIME_PATH: &str = \"{PIG_TEMPERATE_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The COW skin (the temperate variant the CowRenderer's variant\n/// table defaults to), generated from the same jar: the source sha1\n/// the cow renderer verifies before it draws (javap CowModel:\n/// `LayerDefinition.create(64, 64)` — the bovine's own UV layout).\n");
    out.push_str(&format!(
        "pub const COW_TEMPERATE_PNG_SHA1: &str = \"{}\";\n",
        data.cow_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_TEMPERATE_RUNTIME_PATH: &str = \"{COW_TEMPERATE_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The SHEEP skin (the single sheep.png the SheepRenderer's\n/// getTextureLocation speaks — no biome variants), generated from the\n/// same jar: the source sha1 the sheep renderer verifies before it\n/// draws (javap SheepModel: `LayerDefinition.create(64, 32)` — the\n/// ovine's own HALF-HEIGHT UV layout).\n");
    out.push_str(&format!(
        "pub const SHEEP_PNG_SHA1: &str = \"{}\";\n",
        data.sheep_png_sha1
    ));
    out.push_str(&format!(
        "pub const SHEEP_RUNTIME_PATH: &str = \"{SHEEP_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The SHEEP WOOL (the fur overlay's own texture), generated from\n/// the same jar: the source sha1 the wool layer verifies before it\n/// tints (javap SheepFurModel: `LayerDefinition.create(64, 32)` — the\n/// inflate-cubes' own UV layout the dye color multiplies).\n");
    out.push_str(&format!(
        "pub const SHEEP_WOOL_PNG_SHA1: &str = \"{}\";\n",
        data.sheep_wool_png_sha1
    ));
    out.push_str(&format!(
        "pub const SHEEP_WOOL_RUNTIME_PATH: &str = \"{SHEEP_WOOL_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The SHEEP WOOL UNDERCOAT (the undercoat layer's own texture),\n/// generated from the same jar: the source sha1 the undercoat lane\n/// verifies before it tints (javap SheepWoolUndercoatLayer: the\n/// SHEEP_WOOL_UNDERCOAT_LOCATION static — the body-shaped coat the\n/// gate renders when the sheep's dye ISN'T the WHITE default, the\n/// sheared coat the wool fur's removal never erased; LayerDefinitions\n/// bakes the BODY layer the base skin's mesh speaks).\n");
    out.push_str(&format!(
        "pub const SHEEP_WOOL_UNDERCOAT_PNG_SHA1: &str = \"{}\";\n",
        data.sheep_wool_undercoat_png_sha1
    ));
    out.push_str(&format!(
        "pub const SHEEP_WOOL_UNDERCOAT_RUNTIME_PATH: &str = \"{SHEEP_WOOL_UNDERCOAT_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The BABY skins (the dedicated baby lane's own textures), generated\n/// from the same jar: the source sha1s the baby models verify before\n/// they draw (javap SheepRenderer's babyModel =\n/// BabySheepModel(bakeLayer(SHEEP_BABY)) + the 'sheep_baby.png'\n/// location, the SheepWoolLayer's babyModel + the\n/// SHEEP_WOOL_BABY_LOCATION, the cow/pig variant tables' _baby\n/// entries — each the OWN layout the baby layer defs speak:\n/// 64x32 sheep, 64x64 cow, 32x32 pig).\n");
    out.push_str(&format!(
        "pub const SHEEP_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.sheep_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const SHEEP_BABY_RUNTIME_PATH: &str = \"{SHEEP_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const SHEEP_WOOL_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.sheep_wool_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const SHEEP_WOOL_BABY_RUNTIME_PATH: &str = \"{SHEEP_WOOL_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const COW_TEMPERATE_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.cow_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_TEMPERATE_BABY_RUNTIME_PATH: &str = \"{COW_TEMPERATE_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PIG_TEMPERATE_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.pig_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_TEMPERATE_BABY_RUNTIME_PATH: &str = \"{PIG_TEMPERATE_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The CHICKEN skins (the avian's own textures), generated from the\n/// same jar: the source sha1s the chicken renderer verifies before it\n/// draws (javap ChickenRenderer's AdultAndBabyModelPair — the adult\n/// the AdultChickenModel bakes, the baby the BabyChickenModel — each\n/// the OWN layout the baby layer defs speak: 64x32 adult, 16x16\n/// baby).
");
    out.push_str(&format!(
        "pub const CHICKEN_TEMPERATE_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_TEMPERATE_RUNTIME_PATH: &str = \"{CHICKEN_TEMPERATE_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const CHICKEN_TEMPERATE_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_TEMPERATE_BABY_RUNTIME_PATH: &str = \"{CHICKEN_TEMPERATE_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The CHICKEN VARIANT skins (the cold/warm pairs the 26.2 variant\n/// table added — data/minecraft/chicken_variant/{cold,warm}.json's\n/// asset_id/baby_asset_id the ChickenRenderer's pair picks by the\n/// variant's model type: the cold pair rides the ColdChickenModel,\n/// the warm pair the plain adult geometry wears): the source sha1s\n/// the variant loads verify before they draw. The same 64x32 adult /\n/// 16x16 baby layouts the temperate pair keeps.\n");
    out.push_str(&format!(
        "pub const CHICKEN_COLD_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_cold_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_COLD_RUNTIME_PATH: &str = \"{CHICKEN_COLD_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const CHICKEN_COLD_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_cold_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_COLD_BABY_RUNTIME_PATH: &str = \"{CHICKEN_COLD_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const CHICKEN_WARM_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_warm_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_WARM_RUNTIME_PATH: &str = \"{CHICKEN_WARM_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const CHICKEN_WARM_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.chicken_warm_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const CHICKEN_WARM_BABY_RUNTIME_PATH: &str = \"{CHICKEN_WARM_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// THE PIG VARIANT SKINS (the cold/warm pairs the 26.2 variant table\n/// added — data/minecraft/pig_variant/{cold,warm}.json's asset_id\n/// pair the PigRenderer's model pick wears: the cold pair rides the\n/// ColdPigModel's coat walk, the warm pair the plain geometry — the\n/// pig's warm.json the model field never names): the source sha1s\n/// the variant loads verify before they draw. The same 64x64 adult /\n/// 32x32 baby layouts the temperate pair keeps.\n");
    out.push_str(&format!(
        "pub const PIG_COLD_PNG_SHA1: &str = \"{}\";\n",
        data.pig_cold_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_COLD_RUNTIME_PATH: &str = \"{PIG_COLD_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PIG_COLD_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.pig_cold_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_COLD_BABY_RUNTIME_PATH: &str = \"{PIG_COLD_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PIG_WARM_PNG_SHA1: &str = \"{}\";\n",
        data.pig_warm_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_WARM_RUNTIME_PATH: &str = \"{PIG_WARM_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PIG_WARM_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.pig_warm_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const PIG_WARM_BABY_RUNTIME_PATH: &str = \"{PIG_WARM_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// THE COW VARIANT SKINS (the cold/warm pairs the 26.2 variant table\n/// added — data/minecraft/cow_variant/{cold,warm}.json's asset_id\n/// pair the CowRenderer's model pick wears: the cold pair rides the\n/// ColdCowModel's coat walk, the warm pair the WarmCowModel's horn\n/// walk — the cow's warm.json the 'model': 'warm' field names): the\n/// source sha1s the variant loads verify before they draw. The same\n/// 64x64 adult / 32x32 baby layouts the temperate pair keeps.\n");
    out.push_str(&format!(
        "pub const COW_COLD_PNG_SHA1: &str = \"{}\";\n",
        data.cow_cold_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_COLD_RUNTIME_PATH: &str = \"{COW_COLD_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const COW_COLD_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.cow_cold_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_COLD_BABY_RUNTIME_PATH: &str = \"{COW_COLD_BABY_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const COW_WARM_PNG_SHA1: &str = \"{}\";\n",
        data.cow_warm_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_WARM_RUNTIME_PATH: &str = \"{COW_WARM_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const COW_WARM_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.cow_warm_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const COW_WARM_BABY_RUNTIME_PATH: &str = \"{COW_WARM_BABY_RUNTIME_PATH}\";\n"
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
        "pub const PARTICLE_FLAME_PNG_SHA1: &str = \"{}\";\n",
        data.particle_flame_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_FLAME_RUNTIME_PATH: &str = \"{PARTICLE_FLAME_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_DAMAGE_PNG_SHA1: &str = \"{}\";\n",
        data.particle_damage_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_DAMAGE_RUNTIME_PATH: &str = \"{PARTICLE_DAMAGE_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_HEART_PNG_SHA1: &str = \"{}\";\n",
        data.particle_heart_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_HEART_RUNTIME_PATH: &str = \"{PARTICLE_HEART_RUNTIME_PATH}\";\n"
    ));    out.push_str(&format!(
        "pub const PARTICLE_GLINT_PNG_SHA1: &str = \"{}\";\n",
        data.particle_glint_png_sha1
    ));
    out.push_str(&format!(
        "pub const PARTICLE_GLINT_RUNTIME_PATH: &str = \"{PARTICLE_GLINT_RUNTIME_PATH}\";\n"
    ));
    out.push_str(&format!(
        "pub const PARTICLE_SWEEP_PNG_SHA1S: [&str; 8] = [{}];\n",
        data.particle_sweep_png_sha1s
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "pub const PARTICLE_SWEEP_RUNTIME_PATHS: [&str; 8] = [{}];\n",
        PARTICLE_SWEEP_RUNTIME_PATHS
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str("\n/// The WOLF skin (the wild default the WolfRenderer's extractRenderState\n/// `wolf.getTexture()` answers — the variant table's PALE word the\n/// no-variant lane keeps), generated from the same jar: the source sha1\n/// the wolf renderer verifies before it draws (javap AdultWolfModel:\n/// the createBodyLayer mesh the `LayerDefinition.create(mesh, 64, 32)`\n/// layout unwraps — the head+ears+muzzle/body/upper_body/4-legs/tail\n/// chain the bytecode speaks).\n");
    out.push_str(&format!(
        "pub const WOLF_PNG_SHA1: &str = \"{}\";\n",
        data.wolf_png_sha1
    ));
    out.push_str(&format!(
        "pub const WOLF_RUNTIME_PATH: &str = \"{WOLF_RUNTIME_PATH}\";\n"
    ));
    out.push_str("\n/// The WOLF BABY skin (the dedicated baby lane's own texture), generated\n/// from the same jar: the source sha1 the baby model verifies before it\n/// draws (javap WolfRenderer's babyModel = BabyWolfModel(bakeLayer(WOLF_BABY))\n/// — the AgeableMobRenderer's babyModel field the isBaby word picks, the\n/// OWN 32x32 layout the BabyWolfModel's `createBodyLayer` create keeps).\n");
    out.push_str(&format!(
        "pub const WOLF_BABY_PNG_SHA1: &str = \"{}\";\n",
        data.wolf_baby_png_sha1
    ));
    out.push_str(&format!(
        "pub const WOLF_BABY_RUNTIME_PATH: &str = \"{WOLF_BABY_RUNTIME_PATH}\";\n"
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

    /// A 32x32 RGBA png — the sweep arc's frame shape, one lit texel so
    /// the decode is non-trivial.
    fn sweep_png() -> Vec<u8> {
        let (w, h) = (32u32, 32u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let o = ((16 * w + 16) * 4) as usize;
        rgba[o..o + 4].copy_from_slice(&[240, 240, 240, 255]);
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

    /// A 64x32 RGBA png — the sheep skin's own shape (the ovine's
    /// HALF-HEIGHT layout, javap `LayerDefinition.create(64, 32)`), one
    /// lit texel so the decode is non-trivial.
    fn sheep_png() -> Vec<u8> {
        let (w, h) = (64u32, 32u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let o = ((10 * w + 10) * 4) as usize;
        rgba[o..o + 4].copy_from_slice(&[228, 228, 228, 255]);
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
        let flame = tiny_png();
        let expected_flame_sha1 = crate::util::sha1_hex(&flame);
        let sheep = sheep_png();
        let expected_sheep_sha1 = crate::util::sha1_hex(&sheep);
        let sheep_wool = sheep_png();
        let expected_sheep_wool_sha1 = crate::util::sha1_hex(&sheep_wool);
        let sheep_wool_undercoat = sheep_png();
        let expected_sheep_wool_undercoat_sha1 = crate::util::sha1_hex(&sheep_wool_undercoat);
        let sheep_baby = sheep_png();
        let expected_sheep_baby_sha1 = crate::util::sha1_hex(&sheep_baby);
        let sheep_wool_baby = sheep_png();
        let expected_sheep_wool_baby_sha1 = crate::util::sha1_hex(&sheep_wool_baby);
        let cow_baby = skin_png();
        let expected_cow_baby_sha1 = crate::util::sha1_hex(&cow_baby);
        let pig_baby = sweep_png();
        let expected_pig_baby_sha1 = crate::util::sha1_hex(&pig_baby);
        let chicken = sheep_png();
        let expected_chicken_sha1 = crate::util::sha1_hex(&chicken);
        let chicken_baby = underwater_png();
        let expected_chicken_baby_sha1 = crate::util::sha1_hex(&chicken_baby);
        let damage = tiny_png();
        let expected_damage_sha1 = crate::util::sha1_hex(&damage);
        let heart = tiny_png();
        let expected_heart_sha1 = crate::util::sha1_hex(&heart);
        let glint = tiny_png();
        let expected_glint_sha1 = crate::util::sha1_hex(&glint);
        let sweeps = [sweep_png(), sweep_png(), sweep_png(), sweep_png(), sweep_png(), sweep_png(), sweep_png(), sweep_png()];
        let expected_sweep_sha1s: Vec<String> =
            sweeps.iter().map(|p| crate::util::sha1_hex(p)).collect();
        // THE VARIANT TABLE'S SYNTHETICS (the dims the layout asserts
        // demand — the thin/arachnid/canine 64x32, the canine baby's
        // 32x32, the avian variants' 64x32 + 16x16, the swine/bovine
        // variants' 64x64 + 32x32): the sha1s the emit pins.
        let skeleton = sheep_png();
        let expected_skeleton_sha1 = crate::util::sha1_hex(&skeleton);
        let spider = sheep_png();
        let expected_spider_sha1 = crate::util::sha1_hex(&spider);
        let chicken_cold = sheep_png();
        let expected_chicken_cold_sha1 = crate::util::sha1_hex(&chicken_cold);
        let chicken_cold_baby = underwater_png();
        let expected_chicken_cold_baby_sha1 = crate::util::sha1_hex(&chicken_cold_baby);
        let chicken_warm = sheep_png();
        let expected_chicken_warm_sha1 = crate::util::sha1_hex(&chicken_warm);
        let chicken_warm_baby = underwater_png();
        let expected_chicken_warm_baby_sha1 = crate::util::sha1_hex(&chicken_warm_baby);
        let pig_cold = skin_png();
        let expected_pig_cold_sha1 = crate::util::sha1_hex(&pig_cold);
        let pig_cold_baby = sweep_png();
        let expected_pig_cold_baby_sha1 = crate::util::sha1_hex(&pig_cold_baby);
        let pig_warm = skin_png();
        let expected_pig_warm_sha1 = crate::util::sha1_hex(&pig_warm);
        let pig_warm_baby = sweep_png();
        let expected_pig_warm_baby_sha1 = crate::util::sha1_hex(&pig_warm_baby);
        let cow_cold = skin_png();
        let expected_cow_cold_sha1 = crate::util::sha1_hex(&cow_cold);
        let cow_cold_baby = skin_png();
        let expected_cow_cold_baby_sha1 = crate::util::sha1_hex(&cow_cold_baby);
        let cow_warm = skin_png();
        let expected_cow_warm_sha1 = crate::util::sha1_hex(&cow_warm);
        let cow_warm_baby = skin_png();
        let expected_cow_warm_baby_sha1 = crate::util::sha1_hex(&cow_warm_baby);
        let wolf = sheep_png();
        let expected_wolf_sha1 = crate::util::sha1_hex(&wolf);
        let wolf_baby = sweep_png();
        let expected_wolf_baby_sha1 = crate::util::sha1_hex(&wolf_baby);
        let creeper = sheep_png();
        let expected_creeper_sha1 = crate::util::sha1_hex(&creeper);

        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, png.clone()),
            (SLIM_ALEX_JAR_PATH, png.clone()),
            (ZOMBIE_JAR_PATH, png.clone()),
            (PIG_TEMPERATE_JAR_PATH, png.clone()),
            (COW_TEMPERATE_JAR_PATH, png.clone()),
            (SHEEP_JAR_PATH, sheep.clone()),
            (SHEEP_WOOL_JAR_PATH, sheep_wool.clone()),
            (SHEEP_WOOL_UNDERCOAT_JAR_PATH, sheep_wool_undercoat.clone()),
            (SHEEP_BABY_JAR_PATH, sheep_baby.clone()),
            (SHEEP_WOOL_BABY_JAR_PATH, sheep_wool_baby.clone()),
            (COW_TEMPERATE_BABY_JAR_PATH, cow_baby.clone()),
            (PIG_TEMPERATE_BABY_JAR_PATH, pig_baby.clone()),
            (CHICKEN_TEMPERATE_JAR_PATH, sheep.clone()),
            (CHICKEN_TEMPERATE_BABY_JAR_PATH, underwater.clone()),
            (SKELETON_JAR_PATH, skeleton),
            (SPIDER_JAR_PATH, spider),
            (CHICKEN_COLD_JAR_PATH, chicken_cold),
            (CHICKEN_COLD_BABY_JAR_PATH, chicken_cold_baby),
            (CHICKEN_WARM_JAR_PATH, chicken_warm),
            (CHICKEN_WARM_BABY_JAR_PATH, chicken_warm_baby),
            (PIG_COLD_JAR_PATH, pig_cold),
            (PIG_COLD_BABY_JAR_PATH, pig_cold_baby),
            (PIG_WARM_JAR_PATH, pig_warm),
            (PIG_WARM_BABY_JAR_PATH, pig_warm_baby),
            (COW_COLD_JAR_PATH, cow_cold),
            (COW_COLD_BABY_JAR_PATH, cow_cold_baby),
            (COW_WARM_JAR_PATH, cow_warm),
            (COW_WARM_BABY_JAR_PATH, cow_warm_baby),
            (WOLF_JAR_PATH, wolf),
            (WOLF_BABY_JAR_PATH, wolf_baby),
            (CREEPER_JAR_PATH, creeper),
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
            (PARTICLE_FLAME_JAR_PATH, flame),
            (PARTICLE_DAMAGE_JAR_PATH, damage),
            (PARTICLE_HEART_JAR_PATH, heart),
            (PARTICLE_GLINT_JAR_PATH, glint),
            (PARTICLE_SWEEP_JAR_PATHS[0], sweeps[0].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[1], sweeps[1].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[2], sweeps[2].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[3], sweeps[3].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[4], sweeps[4].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[5], sweeps[5].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[6], sweeps[6].clone()),
            (PARTICLE_SWEEP_JAR_PATHS[7], sweeps[7].clone()),
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
            data.pig_png_sha1, expected_png_sha1,
            "the pig's own sha1 (the fixture doubles as the temperate skin)"
        );
        assert_eq!(
            data.cow_png_sha1, expected_png_sha1,
            "the cow's own sha1 (the fixture doubles as the temperate skin)"
        );
        assert_eq!(
            data.sheep_png_sha1, expected_sheep_sha1,
            "the sheep's own sha1 (its own 64x32 shape the fixture speaks)"
        );
        assert_eq!(
            data.sheep_wool_png_sha1, expected_sheep_wool_sha1,
            "the wool's own sha1 (the fur overlay's own 64x32 shape)"
        );
        assert_eq!(
            data.sheep_wool_undercoat_png_sha1, expected_sheep_wool_undercoat_sha1,
            "the undercoat's own sha1 (the body-shaped lane's 64x32)"
        );
        assert_eq!(
            data.sheep_baby_png_sha1, expected_sheep_baby_sha1,
            "the baby sheep's own sha1 (the dedicated baby lane's 64x32)"
        );
        assert_eq!(
            data.sheep_wool_baby_png_sha1, expected_sheep_wool_baby_sha1,
            "the baby wool's own sha1 (the SHEEP_BABY_WOOL lane's 64x32)"
        );
        assert_eq!(
            data.cow_baby_png_sha1, expected_cow_baby_sha1,
            "the baby cow's own sha1 (the baby lane's 64x64)"
        );
        assert_eq!(
            data.pig_baby_png_sha1, expected_pig_baby_sha1,
            "the baby pig's own sha1 (the baby lane's 32x32)"
        );
        assert_eq!(
            data.chicken_png_sha1, expected_chicken_sha1,
            "the chicken's own sha1 (the avian lane's 64x32)"
        );
        assert_eq!(
            data.chicken_baby_png_sha1, expected_chicken_baby_sha1,
            "the baby chicken's own sha1 (the baby lane's 16x16)"
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
            data.particle_flame_png_sha1, expected_flame_sha1,
            "the flame sprite's own sha1"
        );
        assert_eq!(
            data.particle_damage_png_sha1, expected_damage_sha1,
            "the damage sprite's own sha1"
        );
        assert_eq!(
            data.particle_heart_png_sha1, expected_heart_sha1,
            "the heart sprite's own sha1"
        );
        assert_eq!(
            data.particle_glint_png_sha1, expected_glint_sha1,
            "the glint sprite's own sha1"
        );
        assert_eq!(
            data.particle_sweep_png_sha1s,
            expected_sweep_sha1s.as_slice(),
            "the sweep frames' own sha1s"
        );
        // THE VARIANT TABLE'S SHA1S (the regen-gap closure's own pins —
        // the generator reads the same jar rows the hand-added ledger
        // carried, the emit never loses them again)
        assert_eq!(data.skeleton_png_sha1, expected_skeleton_sha1, "the skeleton skin's own sha1");
        assert_eq!(data.spider_png_sha1, expected_spider_sha1, "the spider skin's own sha1");
        assert_eq!(data.chicken_cold_png_sha1, expected_chicken_cold_sha1, "the cold chicken's own sha1");
        assert_eq!(data.chicken_cold_baby_png_sha1, expected_chicken_cold_baby_sha1, "the cold chick's own sha1");
        assert_eq!(data.chicken_warm_png_sha1, expected_chicken_warm_sha1, "the warm chicken's own sha1");
        assert_eq!(data.chicken_warm_baby_png_sha1, expected_chicken_warm_baby_sha1, "the warm chick's own sha1");
        assert_eq!(data.pig_cold_png_sha1, expected_pig_cold_sha1, "the cold pig's own sha1");
        assert_eq!(data.pig_cold_baby_png_sha1, expected_pig_cold_baby_sha1, "the cold piglet's own sha1");
        assert_eq!(data.pig_warm_png_sha1, expected_pig_warm_sha1, "the warm pig's own sha1");
        assert_eq!(data.pig_warm_baby_png_sha1, expected_pig_warm_baby_sha1, "the warm piglet's own sha1");
        assert_eq!(data.cow_cold_png_sha1, expected_cow_cold_sha1, "the cold cow's own sha1");
        assert_eq!(data.cow_cold_baby_png_sha1, expected_cow_cold_baby_sha1, "the cold calf's own sha1");
        assert_eq!(data.cow_warm_png_sha1, expected_cow_warm_sha1, "the warm cow's own sha1");
        assert_eq!(data.cow_warm_baby_png_sha1, expected_cow_warm_baby_sha1, "the warm calf's own sha1");
        assert_eq!(data.wolf_png_sha1, expected_wolf_sha1, "the wolf's own sha1");
        assert_eq!(data.wolf_baby_png_sha1, expected_wolf_baby_sha1, "the wolf pup's own sha1");
        assert_eq!(data.creeper_png_sha1, expected_creeper_sha1, "the creeper's own sha1");

        let emitted = emit(&data).unwrap();
        // THE VARIANT TABLE'S EMIT (the blocks land verbatim — the
        // regeneration now speaks the full committed set)
        assert!(emitted.contains(&format!("pub const SKELETON_PNG_SHA1: &str = \"{expected_skeleton_sha1}\";")));
        assert!(emitted.contains("pub const SKELETON_RUNTIME_PATH: &str = \"entity/skeleton/skeleton.png\";"));
        assert!(emitted.contains(&format!("pub const SPIDER_PNG_SHA1: &str = \"{expected_spider_sha1}\";")));
        assert!(emitted.contains("pub const SPIDER_RUNTIME_PATH: &str = \"entity/spider/spider.png\";"));
        assert!(emitted.contains(&format!("pub const CHICKEN_COLD_PNG_SHA1: &str = \"{expected_chicken_cold_sha1}\";")));
        assert!(emitted.contains("pub const CHICKEN_COLD_BABY_RUNTIME_PATH: &str = \"entity/chicken/chicken_cold_baby.png\";"));
        assert!(emitted.contains(&format!("pub const CHICKEN_WARM_PNG_SHA1: &str = \"{expected_chicken_warm_sha1}\";")));
        assert!(emitted.contains("pub const CHICKEN_WARM_BABY_RUNTIME_PATH: &str = \"entity/chicken/chicken_warm_baby.png\";"));
        assert!(emitted.contains(&format!("pub const PIG_COLD_PNG_SHA1: &str = \"{expected_pig_cold_sha1}\";")));
        assert!(emitted.contains("pub const PIG_COLD_BABY_RUNTIME_PATH: &str = \"entity/pig/pig_cold_baby.png\";"));
        assert!(emitted.contains(&format!("pub const PIG_WARM_PNG_SHA1: &str = \"{expected_pig_warm_sha1}\";")));
        assert!(emitted.contains("pub const PIG_WARM_BABY_RUNTIME_PATH: &str = \"entity/pig/pig_warm_baby.png\";"));
        assert!(emitted.contains(&format!("pub const COW_COLD_PNG_SHA1: &str = \"{expected_cow_cold_sha1}\";")));
        assert!(emitted.contains("pub const COW_COLD_BABY_RUNTIME_PATH: &str = \"entity/cow/cow_cold_baby.png\";"));
        assert!(emitted.contains(&format!("pub const COW_WARM_PNG_SHA1: &str = \"{expected_cow_warm_sha1}\";")));
        assert!(emitted.contains("pub const COW_WARM_BABY_RUNTIME_PATH: &str = \"entity/cow/cow_warm_baby.png\";"));
        assert!(emitted.contains(&format!("pub const WOLF_PNG_SHA1: &str = \"{expected_wolf_sha1}\";")));
        assert!(emitted.contains("pub const WOLF_RUNTIME_PATH: &str = \"entity/wolf/wolf.png\";"));
        assert!(emitted.contains(&format!("pub const WOLF_BABY_PNG_SHA1: &str = \"{expected_wolf_baby_sha1}\";")));
        assert!(emitted.contains("pub const WOLF_BABY_RUNTIME_PATH: &str = \"entity/wolf/wolf_baby.png\";"));
        assert!(emitted.contains(&format!("pub const CREEPER_PNG_SHA1: &str = \"{expected_creeper_sha1}\";")));
        assert!(emitted.contains("pub const CREEPER_RUNTIME_PATH: &str = \"entity/creeper/creeper.png\";"));
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
        // the pig: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const PIG_TEMPERATE_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains("pub const PIG_TEMPERATE_RUNTIME_PATH: &str = \"entity/pig/pig_temperate.png\";"));
        assert!(emitted.contains(&format!(
            "pub const CHICKEN_TEMPERATE_PNG_SHA1: &str = \"{expected_chicken_sha1}\";"
        )));
        assert!(emitted.contains(
            "pub const CHICKEN_TEMPERATE_RUNTIME_PATH: &str = \"entity/chicken/chicken_temperate.png\";"
        ));
        assert!(emitted.contains(&format!(
            "pub const CHICKEN_TEMPERATE_BABY_PNG_SHA1: &str = \"{expected_chicken_baby_sha1}\";"
        )));
        assert!(emitted.contains(
            "pub const CHICKEN_TEMPERATE_BABY_RUNTIME_PATH: &str = \"entity/chicken/chicken_temperate_baby.png\";"
        ));
        // the cow: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const COW_TEMPERATE_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains("pub const COW_TEMPERATE_RUNTIME_PATH: &str = \"entity/cow/cow_temperate.png\";"));
        // the sheep: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const SHEEP_PNG_SHA1: &str = \"{expected_sheep_sha1}\";")));
        assert!(emitted.contains("pub const SHEEP_RUNTIME_PATH: &str = \"entity/sheep/sheep.png\";"));
        // the sheep wool: the sha1 and the runtime path land verbatim
        assert!(emitted.contains(&format!("pub const SHEEP_WOOL_PNG_SHA1: &str = \"{expected_sheep_wool_sha1}\";")));
        assert!(emitted.contains("pub const SHEEP_WOOL_RUNTIME_PATH: &str = \"entity/sheep/sheep_wool.png\";"));
        assert!(emitted.contains("pub const SHEEP_WOOL_UNDERCOAT_RUNTIME_PATH: &str = \"entity/sheep/sheep_wool_undercoat.png\";"));
        // the baby skins: the sha1s and the runtime paths land verbatim
        assert!(emitted.contains(&format!("pub const SHEEP_BABY_PNG_SHA1: &str = \"{expected_sheep_baby_sha1}\";")));
        assert!(emitted.contains("pub const SHEEP_BABY_RUNTIME_PATH: &str = \"entity/sheep/sheep_baby.png\";"));
        assert!(emitted.contains(&format!("pub const SHEEP_WOOL_BABY_PNG_SHA1: &str = \"{expected_sheep_wool_baby_sha1}\";")));
        assert!(emitted.contains("pub const SHEEP_WOOL_BABY_RUNTIME_PATH: &str = \"entity/sheep/sheep_wool_baby.png\";"));
        assert!(emitted.contains(&format!("pub const COW_TEMPERATE_BABY_PNG_SHA1: &str = \"{expected_cow_baby_sha1}\";")));
        assert!(emitted.contains("pub const COW_TEMPERATE_BABY_RUNTIME_PATH: &str = \"entity/cow/cow_temperate_baby.png\";"));
        assert!(emitted.contains(&format!("pub const PIG_TEMPERATE_BABY_PNG_SHA1: &str = \"{expected_pig_baby_sha1}\";")));
        assert!(emitted.contains("pub const PIG_TEMPERATE_BABY_RUNTIME_PATH: &str = \"entity/pig/pig_temperate_baby.png\";"));
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
        for (i, sha) in expected_sweep_sha1s.iter().enumerate() {
            assert!(
                emitted.contains(&format!("\"{sha}\"")),
                "sweep frame {i}'s sha1"
            );
            assert!(emitted.contains(&format!("\"particle/sweep_{i}.png\"")));
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
    fn refuses_a_zip_without_the_pig_skin() {
        // the pig's table must not be invented either: a jar missing
        // pig_temperate.png fails the generation (the same data-driven
        // rule the zombie skin speaks)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (SLIM_ALEX_JAR_PATH, skin_png()), (ZOMBIE_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing pig skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_cow_skin() {
        // the cow's table must not be invented either: a jar missing
        // cow_temperate.png fails the generation (the same data-driven
        // rule the pig skin speaks)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (SLIM_ALEX_JAR_PATH, skin_png()), (ZOMBIE_JAR_PATH, skin_png()), (PIG_TEMPERATE_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing cow skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_sheep_skin() {
        // the sheep's table must not be invented either: a jar missing
        // sheep.png fails the generation (the same data-driven rule the
        // cow skin speaks)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (SLIM_ALEX_JAR_PATH, skin_png()), (ZOMBIE_JAR_PATH, skin_png()), (PIG_TEMPERATE_JAR_PATH, skin_png()), (COW_TEMPERATE_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing sheep skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_sheep_wool() {
        // the wool's table must not be invented either: a jar missing
        // sheep_wool.png fails the generation (the same data-driven
        // rule the sheep skin speaks)
        let mut zip = zip_of(&[(
            WIDE_STEVE_JAR_PATH,
            skin_png(),
        ), (SLIM_ALEX_JAR_PATH, skin_png()), (ZOMBIE_JAR_PATH, skin_png()), (PIG_TEMPERATE_JAR_PATH, skin_png()), (COW_TEMPERATE_JAR_PATH, skin_png()), (SHEEP_JAR_PATH, sheep_png())]);
        assert!(generate(&mut zip).is_err(), "missing sheep wool must be refused");
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
            (PIG_TEMPERATE_JAR_PATH, skin_png()),
            (COW_TEMPERATE_JAR_PATH, skin_png()),
            (SHEEP_JAR_PATH, sheep_png()),
            (SHEEP_WOOL_JAR_PATH, sheep_png()),
            (FIRE_0_JAR_PATH, strip_png()),
            (FIRE_1_JAR_PATH, strip_png()),
        ]);
        assert!(generate(&mut zip).is_err(), "missing underwater.png must be refused");
    }
}
