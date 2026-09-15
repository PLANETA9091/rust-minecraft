# shots/ — the client's visual evidence

Every lane that ships pixels owes the folder a frame. The gallery is the
proof the features exist as rendered images, not as test names alone.
94 frames and counting — grouped below by the lane that spoke them.

## The F5 lane (the third-person avatar) — v0.3.154+

The CameraType cycle, the detached camera (the eight-ray getMaxZoom
shrink), the full avatar (the HumanoidModel geometry, the walk cosines,
the crouch, the swing), the F5 key.

| frame | the scene |
|---|---|
| `f5_back.png` | the avatar's back mid-stride in seed 42's pond, the sun glare |
| `f5_front.png` | the face, full body |
| `f5_crouch.png` | the sneak hunch |
| `f5_wade.png` | THE WATER-STEP LANE's own scene (v0.3.160): the wader knee-deep in the pond, mid-stride, the sun glare on the water — every fired step speaks the swim voice + the muffled block step |
| `f5_wade_front.png` | the same wade, front view — the water line across the legs |
| `s_f5_third_back_walk.png`, `s_f5_third_front.png`, `s_f5_crouch_walk.png`, `s_f5_swing.png` | the twin agent's static-lane frames (the walk, the front, the crouch walk, the swing freeze) |

Regenerate the family (the flags the CLI speaks today):

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --perspective third-back --body --body-walk 0.9,5.0 --out shots/f5_wade.png
# --perspective first|third-back|third-front, --body-yaw/--body-pitch,
# --body-crouch, --body-walk "speed,position" (the WalkAnimationState pair)
```

## The lava lane (the player's fire arc) — v0.3.162

The pool carve (--lava-pool: the 3x3 dry basin at the spawn column, the
saturated layer turned to lava), the REAL contact laws (the ignite, the
4.0 through the cooldown, the flame rings the fire clock drives), the
lava_still pixels the atlas lane landed (tile 28, the first-frame crop).

| frame | the scene |
|---|---|
| `lava_wade.png` | THE MONEY SHOT: the wader chest-deep in the pool, the flame rings climbing, the health the 4.0s took — every law the contact fired is real (no scripted fire) |
| `lava_wade_back.png` | the back view — the flames over the shoulders, the wall backdrop |
| `lava_zombie.png` | the alternate angle (pitch -10) — the pool's face fills the foreground |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --perspective third-front --body --body-pitch=-15 --lava-pool --walk-ticks 2 \
  --out shots/lava_wade.png
# --lava-pool (the dry basin carve), --body-fire N (the seeded brand —
# the flame feature's own render clock), --walk-ticks (the sink + the
# contact)
```

## The fire-screen lane (the first-person burn) — v0.3.163

The ScreenEffectRenderer overlay (javap the 26.2 jar): the
GameRenderer's `cameraType.isFirstPerson()` gate, the `isOnFire` arm,
the TWO camera quads `translate(±0.24, -0.3, 0)·rotateY(∓10°)` at
z=-0.5, the FIRE_1 sprite's full frame through buildQuad's own uv walk
(the left edge rides u1 — the horizontal mirror), the vertex alpha
0xE5FFFFFF (229/255) the translucent blend speaks, the depth test
without the write.

| frame | the scene |
|---|---|
| `fire_screen.png` | THE BURNING SCREEN: the two flame sheets tilted over the lower view, the lava pool glowing beneath, the hand and the HUD riding over — the first-person burn the vanilla screen speaks |

Regenerate:

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --body --lava-pool --body-fire 250 --perspective first \
  --out shots/fire_screen.png
```

## The water-screen lane (the first-person dive) — v0.3.164

The ScreenEffectRenderer's water arm (javap the 26.2 jar): the
`isEyeInFluid(WATER)` gate behind the spectator check, ONE camera quad
on the [-1,1]² plane at z=-0.5 (no translate, no rotate — the quad
overflows any frustum), the uv corners (yawOff ± 4, pitchOff ± 4) with
`yawOff = -getYRot()/64`, `pitchOff = getXRot()/64` — the 16x16
underwater.png tiles REPEAT, 4 wraps span the screen and the pattern
scrolls as the look turns. The vertex color `colorFromFloat(0.1, br,
br, br)` — the eye block's own brightness through the Lightmap ramp
`f/(4-3f)`, the alpha 25/255 the translucent blend scales the texel's
own 135/146 by. The water fog (the FogRenderer's own lane) sleeps —
this pass is the submit's water arm verbatim.

| frame | the scene |
|---|---|
| `underwater_dive.png` | THE BLUE WASH: the eye one cell under the surface, the 16x16 tile's blue over every pixel (the mean Δb +10 the diff control spoke), the basin walls and the sandy floor through it |
| `underwater_dive_back.png` | the detached view — the avatar in the basin, NO wash (the submit's first-person gate sleeps the whole block) |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --body --water-pool --perspective first --out shots/underwater_dive.png
# --water-pool (the 3x3 basin carve AT the body, the body sunk to the
# floor — the eye under the surface, the REAL isEyeInFluid geometry),
# --perspective third-back shows the gate's sleep
```

## The water-fog lane (the murk that hides the distance) — v0.3.165

The FogRenderer's own lane (javap the 26.2 jar): the fog type rides
`Camera.getFluidInCamera`'s own camera-position query (the detached
view's honest law too), the `WaterFogEnvironment.setupFog` pair — the
start the `visual/water_fog_start_distance` default -8.0 speaks, the
end `visual/water_fog_end_distance`'s 96.0 x max(0.25, getWaterVision())
— and the color the `visual/water_fog_color` default -16448205
(0x050533) speaks. The vision is `LocalPlayer.getWaterVision`'s own
two-segment ramp over the `waterVisionTime` clock the tick feeds
(+1/tick under, -10/tick out, the clamp [0, 600]): the fresh dive
speaks 24.0, the 100-tick soak 57.6, the full 600-tick soak 96.0. The
setupFog's own pair-copy (skyEnd = cloudEnd = environmentalEnd) folds
the sky bands into the murk and sleeps the celestial pass inside it.

| frame | the scene |
|---|---|
| `underwater_fog.png` | THE FRESH DIVE: the eye one cell under the surface, the clock at 0 — the end 24.0 murk pulls the basin walls toward the navy and the sky bands fold into the 0x050533 clear |
| `underwater_fog_settled.png` | THE SOAKED EYE: `--water-vision 600` — the ramp's full open, the end 96.0 — the same basin reads bright, the far walls clear of the murk |
| `underwater_fog_back.png` | the detached camera behind the avatar — NO murk (the fog type rides the camera cell, and the detached camera's cell is dry) |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --screen world --body --water-pool \
  --perspective first --out shots/underwater_fog.png
# --water-vision 600 rides the soak clock's seed (the --zombie-fire
# discipline: the deterministic stand-in for the soak time the live
# drift can't hold — the water's own push walks the body out of a
# staged pool), --perspective third-back shows the camera-cell gate
```

## The air-bubbles lane (the drowning arc's HUD) — v0.3.166

The air clock's own row (javap the 26.2 jar): the body's airSupply the
LivingEntity.baseTick branch drains under the eye (one a tick, the -20
floor folding the 2.0 drown through the DROWNING voice) and refills dry
(+4/tick to the 300 the getMaxAirSupply sipush names); the HUD's
extractAirBubbles walk rides the eye-in-water verdict — the gate
`!underwater && air >= max` sleeps the row, the ceil walk
`ceil((air + offset) * 10 / max)` speaks the full / bursting / empty
sprites right-aligned above the hunger (the x = right edge - (i-1)*8 - 9
the 1-based loop folds), the air_empty's 1px jitter rides the drained-dry
frame's even ticks, the bursting's pop voice rides the
lastBubblePopSoundPlayed dedup.

| frame | the scene |
|---|---|
| `air_bubbles.png` | THE MID-DRAIN FRAME: `--air-supply 121` — four full bubbles, the bursting ring at the boundary (the changed window's own sprite), five empty outlines behind |
| `air_bubbles_drained.png` | THE DRAINED-DRY DIVER: `--air-supply 0` — the whole row speaks the empty outlines (popping 10, the jitter arm's own frame) |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --screen world --seed 42 --body \
  --water-pool --perspective first --air-supply 121 --out shots/air_bubbles.png
# --air-supply N rides the air clock's seed (the --water-vision
# discipline: the mid-drain frames the live dive can't hold — the pool
# walks the body out), clamped [-20, 300]
```

## The block-screen lane (the suffocation view) — v0.3.168

The submit chain's own FIRST arm (javap `ScreenEffectRenderer.submit`
+ `getViewBlockingState`, the 26.2 jar): the eight sample points
around the player's box (x ± 0.4*bbWidth, the eye ± 0.05) read
`getRenderShape() != INVISIBLE && isViewBlocking(...)` — the
registration default `isViewBlocking = isSuffocating` =
`blocksMotion() && isCollisionShapeFullBlock()` (BootstrapMethods
#5/#6), the full-cube solids (stone, dirt, sand, the ores, the log)
block, the fluids and the leaf kinds never do. The first hit submits
`getParticleMaterial(state).sprite()` — the block model's own
PARTICLE entry (grass_block's is dirt, the log's its side) — through
`lambda$submitBlockSprite$0`: the [-1, 1]² plane at z = -0.5 (the
water arm's own plane), the (getU1, getV1, getU0, getV0) uv walk (the
left edge rides u1 — the mirrored-upright law the fire arm speaks),
the vertex color -15132391 = 0xFF191919 (the rgb 25/255 dark
multiply, the alpha 0xFF keeps the quad OPAQUE — the world fully
hidden). The draw rides the submit order: BEFORE the water arm's 93
and the fire arm's 120.

| frame | the scene |
|---|---|
| `block_screen.png` | THE BURY: the eye's own cell holds dirt — the eight samples all read the solid, the full frame speaks the dark dirt sprite (the 0xFF191919 tint the bytecode speaks), the HUD rows floating over the suffocation view |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --body --bury --out shots/block_screen.png
# --bury fills the eye's own cell with DIRT (feet + 1.62, the vanilla
# eye geometry); the eight-sample walk reads it, the arm submits the
# block sprite — the --screen lane owns no ticks
```

## The experience-bar lane (the contextual bar) — v0.3.169

The HUD's own empty slot filled (javap the 26.2 jar — the bar MOVED
out of `Hud` into `net.minecraft.client.gui.contextualbar.ExperienceBar`):
the `gameMode.hasExperience()` gate the survival client speaks, then
`extractBackground`'s own law — the `xpNeededForNextLevel > 0` arm
lifts the row (level 0's needed is 7 under the 26.2 curve `7 +
level*2`, the drift the old `2 + level*7` shape never spoke — the
dark background shows from the first frame), the background 182x5 at
the ContextualBar defaults `left = (w-182)/2`, `top = h-24-5`, the
fill `(int)(progress * 183.0f)` riding the TEN-INT region blit — the
leftmost `fill` columns at 1:1, the CROP the never-a-squish law
carries. The level digits render past level 0 only: the bare digits
(the "%s" translation the en_us lang speaks) centered `(w -
textWidth)/2` at `h-24-9-2`, the FOUR black neighbor draws (the
-16777216) + the green center (the -8323296 = 0xFF80FF20), the
dropShadow-less outline `extractExperienceLevel` speaks. The grant
lanes: the bare zombie's flat `xpReward` 5 through
`dropExperience`'s `lastHurtByPlayerMemoryTime` gate (the fist kill's
own reward — the equipment walk adds nothing to the bare body), the
ores' `UniformInt` rolls through the harvest gate (coal 0-2, redstone
1-5, lapis 2-5, diamond/emerald 3-7; copper/iron/gold silent — the
smelting lane's own reward), both folded into `giveExperiencePoints`'s
three loops (the f32 divide, the borrow, the spend on the old/new
curve pair).

| frame | the scene |
|---|---|
| `xp_bar.png` | THE SEEDED STAT: level 27 at 85% — the green fill walks 155 columns of the progress sprite, the dark background rides past it, the "27" in the experience green between the hearts and the food row |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --screen world --seed 20260214 \
  --ui-frame 30 --xp 27,0.85 --out shots/xp_bar.png
# --xp seeds "level,progress" (the --zombie-fire discipline: the
# deterministic stand-in for the mining/killing grants the live lane
# ticks) — the bar reads the seeded store through the SAME gates
```

## The xp-orb lane (the award's physical truth) — v0.3.170

The XP lane's own body (javap `ExperienceOrb` over the 26.2 jar —
`net.minecraft.world.entity.ExperienceOrb`): the award's own split —
`awardWithDirection`'s while loop steps `getExperienceValue`'s
descending bands (2477, 1237, 617, 307, 149, 73, 37, 17, 7, 3, else 1
— the zombie's flat 5 fragments into the [3, 1, 1] walk the vanilla's
own no-2-band verdict speaks), each step ONE orb entity at the
position, the delta the `Vec3.ZERO` the award path passes. The orb's
own physics: the `getDefaultGravity` 0.03, the `followNearbyPlayer`
scan (the 8.0 radius the `getNearestPlayer` call speaks, the keep gate
the 64.0d squared fold holds) — the accel `scale(e * e * 0.1)` over
`e = 1.0 - sqrt(lenSqr) / 8.0` toward the `getY() + getEyeHeight() /
2.0` midpoint, the water's buoyant lift, the drag the ground-friction
fold speaks, the 6000-tick LIFETIME the age discards. The touch:
`playerTouch`'s plain bbox intersect, the `takeXpDelay = 2` throttle
one orb per window, the grant riding `giveExperiencePoints` (the
pickup's own site the level-up chime's %5 gate now owns). The render
(javap `ExperienceOrbRenderer.submit`): the camera-facing quad at
`translate(0, 0.1, 0)` then the camera basis then the 0.3 scale over
the [-0.5, 0.5] x [-0.25, 0.75] plane, the uv walk the icon's 16x16
window rides over the 64x64 `textures/entity/experience/experience_orb.png`
sheet (`(icon % 4) * 16 / 64` the left, `(icon / 4) * 16 / 64` the
top — the `getIcon` band law's variant index 0..10), the vertex color
the pulse law speaks — `r = (sin(age/2) + 1) * 0.5 * 255`, `g = 255`,
`b = (sin(age/2 + 4.1887903) + 1) * 0.1 * 255` (the 4.1887903 the
4π/3 phase shift), alpha 128 the translucent blend the
`entityTranslucentCullItemTarget` render type carries, the block
light +7 the clamp 0..15 speaks. The pickup voice:
`handleTakeItemEntity`'s orb branch — `entity.experience_orb.pickup`
at volume 0.1, pitch `0.9 + (r - r) * 0.35` (the `random/orb.ogg` the
asset index sha1-verifies).

| frame | the scene |
|---|---|
| `xp_orbs.png` | THE AWARD'S OWN FLIGHT: the zombie kill's [3, 1, 1] orbs floating at chest height two blocks ahead — the camera-facing quads ride the pulse (the age-staggered phase spread reads: bright green, mid green, the olive lean the 4π/3-shifted blue band speaks), the sand path and the tree the backdrop |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --screen world --seed 20260214 \
  --body --orbs --body-pitch 15 --ui-frame 3 --out shots/xp_orbs.png
# --orbs seeds the flat 5's own split ahead of the body (the
# --zombie-fire discipline: the deterministic stand-in for the orbs
# the kill's award spawns live — the real lane rides the touch)
```

## The lava-fog lane (the ember murk) — v0.3.167

The fog type's sibling arm (javap the 26.2 jar): the camera-in-lava
cell arms the `LavaFogEnvironment` — the bare arm's 0.25/1.0 pair (the
fconst_0.25 / fconst_1 the bytecode speaks, one block of sight), the
fire-resistance arm's 0.0/5.0 (the hasEffect(FIRE_RESISTANCE) branch
the store's truth opens), the spectator's -8.0 / renderDistance*0.5
(the honest absence — no spectator yet), and the color
`getBaseColor`'s own fixed fold -6743808 = 0xFF991900 — the deep
red-orange murk. The setupFog's own pair-copy (skyEnd = cloudEnd =
environmentalEnd) folds the sky bands into the ember, sleeps the
celestial pass, rides the clouds' fog end — the murk lane's own
treatment, the lava's voice.

| frame | the scene |
|---|---|
| `lava_fog.png` | THE LAVA DIVE: the eye IN the lava cell — the whole frame speaks the ember murk, the burning overlay (the same-tick ignite's staged brand) layers the flame sheets over it, the sky bands folded into the 0xFF991900 clear |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --screen world --seed 42 --body \
  --lava-dive --perspective first --out shots/lava_fog.png
# --lava-dive carves the 3x3 lava basin AT the body and sinks it (the
# water dive's own sibling); the brand rides the stage (the same-tick
# ignite the contact law speaks — the --screen lane owns no ticks)
```

## The zombie lane (the first live mob) — v0.3.155+

The night spawner, the chase, the charged fist, the knockback, the
hurt-flash (the OverlayTexture 0.7 mix), the death flip, the daylight
burn (the flame rings), the loot, the footsteps, the voices.

| frame | the scene |
|---|---|
| `zombie_burn.png` | THE FLAME RINGS on the terrace — the sun's brand burning while the body watches |
| `zombie_night.png` | the stars, the moonlit approach at the terrace lip |
| `zombie_death.png` | the death flip mid-clock |
| `zombie_fire_hurt.png` | THE HURT-FLASH TINT on the burning zombie (v0.3.161): the 0.7 mix darkens the skin while the flame rings wrap it, the wader watching from the pond |
| `s_zombie_burning_day.png`, `s_zombie_night_chase.png` | the twin agent's frames (the burning day, the night chase) |

Regenerate the family:

```sh
cargo run -p mc-client --release -- --headless --screen world --seed 42 \
  --body --mobs 1 --ignite 8 --out shots/zombie_burn.png
# --zombie (single deterministic spawn), --zombie-fire/--zombie-death/
# --zombie-aggro/--zombie-hurt, --mobs N, --walk-ticks/--settle-ticks
```

## The golden matrix (the pixel discipline)

The 13-gate matrix lives in `goldens/` (rms 0.000, regenerated ONLY with
the render change explained in the commit). These `shots/` frames are the
same renders at the gallery's size — the first-person world, the boot
chain, the CPU/GPU duality:

- `world_day.png`, `world_night.png`, `world_clouds.png` — the first-person world (day/night/clouds)
- `verify_*.png` — the boot chain + world verification frames (loading/title/servers/options/pause/chest)
- `v_*.png`, `render.png`, `render640.png`, `preview640.png` — the early golden-matrix runs at gallery sizes
- `held_sword.png`, `held_diff.png` — the held-item display arc

## The renderer archaeology (the early rounds)

The CPU-raster verification and the comparison frames the first render
rounds spoke — kept as the lane's own history:

- `d*.png`, `df_*.png`, `diff*.png`, `f*.png`, `g_*.png`, `n*.png`, `probe*.png`, `v1.png`, `v2.png`, `f.png`, `f480.png` — the CPU vs GPU lane, the boot-chain debug passes, the sky/celestial probes (the exact commands are lost with the pre-reset worklog; the lanes they verified live in the git log)

## The discipline

- every NEW gameplay lane owes the folder its frame in the same round it ships
- the headless shots are deterministic: the same flags, the same pixels
- the shot flags never leak into the goldens (every default absent = the
  golden scenes bit-identical)
