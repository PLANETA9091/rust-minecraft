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
