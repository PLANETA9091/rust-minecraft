//! Culled face meshing with vanilla-style smooth lighting (per-vertex AO),
//! parallel across chunks with `rayon`.
//!
//! The network world's light layer (when the `World` carries one) rides
//! the same per-vertex shading: each corner averages the 4 face-plane
//! cells' light the vanilla smooth-lighting way (the face cell plus the
//! 3 AO diagonals), known cells only, through the vanilla brightness
//! ramp. A world WITHOUT a light layer (the procedural scene) meshes
//! bit-identically to the legacy path — no opinion, no change.

use rayon::prelude::*;

use mc_registry::blockstate_lookup::{select_parts, Part};
use mc_registry::element_quads::{derive_model_inherited, Quad as DerivedQuad};
use mc_registry::generated::atlas;

use crate::colors::{default_color, FACE_SHADE};
use crate::textures::TextureLut;
use crate::tint;
use crate::world::*;

/// One shaded world-space quad: 4 CCW corner points, per-vertex shading and
/// texture mapping. When `tile` is a real atlas index the vertex colors carry
/// the shade×AO multiplier and the rasterizer samples the official texture;
/// when `tile == TILE_NONE` the vertex colors carry the full flat base color.
#[derive(Debug, Clone, PartialEq)]
pub struct Quad {
    pub pts: [[f32; 3]; 4],
    pub colors: [[f32; 3]; 4],
    pub uvs: [[f32; 2]; 4],
    pub tile: u16,
    pub translucent: bool,
    /// THE PER-TEXEL ALPHA ARM (the autopush-599 round — the honey
    /// wake): the translucent pass the FIXED water opacity the
    /// texture-less quads keep, the FLAGGED quads the TEXEL's own
    /// alpha the vanilla translucent shader composites (SRC_ALPHA /
    /// ONE_MINUS_SRC_ALPHA over the framebuffer — the honey textures'
    /// uniform 191/189/129 the arm reads). The water quads the flag
    /// never sets (the fixed seed the bit-exact law keeps — the
    /// goldens the same pixels speak).
    pub per_texel_alpha: bool,
}

/// Brightness multiplier per ambient-occlusion level (reference curve).
/// Level 3 = fully open corner, 0 = corner enclosed by two opaque walls.
/// Reproduces the darkened-corner look of vanilla "Smooth Lighting".
pub const AO_CURVE: [f32; 4] = [0.55, 0.70, 0.85, 1.00];

/// Classic Minecraft vertex AO: a corner is darkened by the opaque blocks
/// touching it on the face plane (side1, side2, corner). Two facing sides
/// fully occlude regardless of the corner block.
#[inline]
pub fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u8 {
    if side1 && side2 {
        0
    } else {
        3 - (side1 as u8 + side2 as u8 + corner as u8)
    }
}

/// Offsets of the 6 neighbors: +X -X +Y -Y +Z -Z.
const NEIGHBOR: [(i32, i32, i32); 6] = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

/// CCW corner offsets per face (unit cube at origin).
const FACE_CORNERS: [[[f32; 3]; 4]; 6] = [
    // +X
    [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
    // -X
    [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
    // +Y
    [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
    // -Y
    [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    // +Z
    [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
    // -Z
    [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
];

/// THE END PORTAL'S OWN SENTINEL (the dev-29 round — the tile id the
/// starry quads carry: the render pass partitions on it, the shader
/// law the sampled walk serves; the ATLAS never banks a tile here —
/// the 16x16 tile the 256x256 noise cannot carry, the two textures
/// the loader's own lane reads)
pub const TILE_END_PORTAL: u16 = 65534;

/// THE HEAD'S OWN MODEL-FACE TABLE (the dev-82 round — javap
/// ModelPart$Cube/<Polygon> the 26.2 ctor law the probe
/// scripts/skull_probe_82.py decodes): per model face the four corner
/// POSITIONS in model px (the head box x -4..4, y -8..0, z -4..4 —
/// SkullModel.createHeadModel's addBox(-4,-8,-4, 8,8,8) law) + the
/// four tile-space uvs (the Polygon ctor's rect-corner assignment —
/// vertex[0] the (u1, v0) top-right corner wears, vertex[1] the
/// (u0, v0), vertex[2] the (u0, v1), vertex[3] the (u1, v1), the
/// NO-inset fold the fconst_0 speak) + the ENTITY_HEAD_FACES slot
/// the tile row reads. The model-face order [down, up, west, north,
/// east, south] the generated table's own law keeps — the DOWN face
/// the model y0=-8 row carries (the head TOP the +Y-down convention
/// speaks, the world top the flip answers), the NORTH face the skin
/// FRONT region banks.
const HEAD_MODEL_FACES: [([[f32; 3]; 4], [[f32; 2]; 4], usize); 6] = [
    // DOWN (slot 0 — the [8,0] region, the head top the flip wears):
    // v24 (maxX, y0, maxZ), v23 (x0, y0, maxZ), v19 (x0, y0, z0),
    // v20 (maxX, y0, z0)
    (
        [[4.0, -8.0, 4.0], [-4.0, -8.0, 4.0], [-4.0, -8.0, -4.0], [4.0, -8.0, -4.0]],
        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        0,
    ),
    // UP (slot 1 — the [16,0] region, the neck cut): v21 (maxX, maxY,
    // z0), v22 (x0, maxY, z0), v26 (x0, maxY, maxZ), v25 (maxX, maxY,
    // maxZ)
    (
        [[4.0, 0.0, -4.0], [-4.0, 0.0, -4.0], [-4.0, 0.0, 4.0], [4.0, 0.0, 4.0]],
        [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
        1,
    ),
    // WEST (slot 2 — the [0,8] region): v19, v23, v26, v22
    (
        [[-4.0, -8.0, -4.0], [-4.0, -8.0, 4.0], [-4.0, 0.0, 4.0], [-4.0, 0.0, -4.0]],
        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        2,
    ),
    // NORTH — THE SKIN'S OWN FACE (slot 3 — the [8,8] region): v20,
    // v19, v22, v21
    (
        [[4.0, -8.0, -4.0], [-4.0, -8.0, -4.0], [-4.0, 0.0, -4.0], [4.0, 0.0, -4.0]],
        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        3,
    ),
    // EAST (slot 4 — the [16,8] region): v24, v20, v21, v25
    (
        [[4.0, -8.0, 4.0], [4.0, -8.0, -4.0], [4.0, 0.0, -4.0], [4.0, 0.0, 4.0]],
        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        4,
    ),
    // SOUTH (slot 5 — the [24,8] region, the back): v23, v24, v25, v26
    (
        [[-4.0, -8.0, 4.0], [4.0, -8.0, 4.0], [4.0, 0.0, 4.0], [-4.0, 0.0, 4.0]],
        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        5,
    ),
];

fn is_transparent(kind: u16) -> bool {
    // the leaves FAMILY: oak (foliage-tinted) and pale oak (the audited
    // no-tint verdict — still cutout leaves, non-occluding, no AO);
    // leaf_litter is the flat ground cover (the 13th audit — non-cube,
    // non-occluding, casts no AO, exactly the blockstate models' verdict);
    // LAVA is the SECOND fluid member (the lava lane) — non-occluding
    // like the water it mirrors (the pool walls' faces draw against it);
    // WHEAT is the crop (the 15th audit — the four-plane cross the
    // block/crop model speaks, `.noCollision()` the Blocks clinit's own
    // row keeps): the farmland's top face under it draws, the crop casts
    // no AO (the is_opaque walk the same verdict keeps)
    kind == AIR
        || kind == WATER
        || kind == LEAVES
        || kind == PALE_LEAVES
        || kind == LEAF_LITTER
        || kind == LAVA
        // THE END PORTAL'S OWN ROW (the dev-29 round — the starry cell
        // the ring's inner walls draw against: the noCollision law the
        // collision shape keeps, the AO the emissive field never casts,
        // the portal's own cube the special arm the sentinel tile
        // emits — the generic tile walk the NO-elements model cannot
        // resolve)
        || kind == crate::world::END_PORTAL
        // the whole AGE LADDER the non-occluding verdict keeps (the
        // farmland's top face under ANY stage draws — the same
        // noCollision family the 15th audit and the growth round speak)
        || crate::world::is_wheat(kind)
        // the FLOWER FAMILY the same verdict keeps (the autopush-411
        // round — the cross planes non-occluding, no AO, the grass
        // under the bloom draws)
        || crate::world::is_flower(kind)
        // THE GRASS TUFT FAMILY the same verdict keeps (the autopush-412
        // round — the tinted_cross planes non-occluding, no AO, the
        // noCollision + replaceable rows the Blocks clinit speaks)
        || crate::world::is_tuft(kind)
        // THE DOUBLE PLANT FAMILY the same verdict keeps (the
        // autopush-413 round — the tinted_cross planes per HALF, the
        // noCollision + replaceable rows the Blocks clinit pc 17244/
        // 17297 speaks, the two cells the pair wears)
        || crate::world::is_double_plant(kind)
        // THE TORCH the same verdict keeps (the dev-32 round — the
        // stick box the neighbors' faces draw against, the noCollision
        // row the Properties chain speaks)
        || crate::world::is_torch(kind)
        // THE REDSTONE TORCH the same verdict keeps (the autopush-666
        // round — the stick box the neighbors' faces draw against, the
        // noCollision row the blocks clinit's REDSTONE_TORCH factory
        // pc 10081 speaks)
        || crate::world::is_redstone_torch(kind)
        // THE LEVER the same verdict keeps (the dev-45 round — the
        // base plate + the tilted stick the neighbors' faces draw
        // against, the noCollision row the Properties chain speaks)
        || crate::world::is_lever(kind)
        // THE BUTTON the same verdict keeps (the autopush-660 round —
        // the thin plate the neighbors' faces draw against, the
        // noCollision row the Properties chain speaks)
        || crate::world::is_button(kind)
        // THE PLATE FAMILY the same verdict keeps (the dev-49 round —
        // the thin floor plate the neighbors' faces draw against, the
        // noCollision row the two factories' Properties chains speak
        // (the pressure-plate pc 15721/15761 windows the weighted rows
        // the same call wear), the non-occluding verdict the vanilla's
        // own canOcclude=false registration banks): the floor's top
        // face under ANY plate draws, the plate casts no AO, the
        // element geometry the attach lane's own walk renders
        || crate::world::is_pressure_plate(kind)
        // THE REDSTONE WIRE the same verdict keeps (the autopush-665
        // round — the y=0.25 flat plane the neighbors' faces draw
        // against, the noCollision row the REDSTONE_WIRE factory's own
        // Properties chain speaks, the non-occluding verdict the
        // vanilla's own canOcclude=false registration banks): the
        // floor's top face under ANY wire draws, the wire casts no AO,
        // the flat geometry the dust lane's own walk renders
        || crate::world::is_redstone_wire(kind)
        // THE RAILS the same verdict keeps (the dev-86 round — the
        // Blocks clinit's noCollision() row the four rail kinds the
        // probe scripts/rail_probe_86.py banks: the y=1/16 flat plane
        // the neighbors' faces draw against, the AO the opaque walk
        // never casts, the ground's top face under ANY rail draws,
        // the flat geometry the rail lane's own walk renders)
        || crate::world::is_rail(kind)
        // THE WEB the same verdict keeps (the dev-22 round — the
        // noCollision() pc 4247 the WebBlock registration speaks, the
        // cross planes non-occluding, no AO, the neighbors' faces the
        // web's own diagonals draw against)
        || crate::world::is_web(kind)
        // THE FIRE KINDS the same verdict keeps (the blocks clinit's
        // replaceable().noCollision() rows the registrations speak —
        // the ground's top face under the flame draws, the flame casts
        // no AO, the multipart planes the cell's own lane renders)
        || crate::world::is_fire(kind)
        // THE CAMPFIRE FAMILY the same verdict keeps (the dev-6 round —
        // the blocks clinit's noOcclusion() row the two registrations
        // speak: the non-occluding column the neighbors' faces draw
        // against, the cullface verdicts the campfire lane's own
        // neighbor reads keep, the fire planes the cell's own light
        // the lane renders)
        || crate::world::is_campfire(kind)
        // THE FENCE FAMILY the same verdict keeps (the dev-55 round
        // — the blocks clinit's forceSolidOn() row the thirteen
        // registrations speak: the non-occluding compound the
        // neighbors' faces draw against, the AO the opaque walk
        // never casts, the post + the side arms the cell's own
        // multipart lane renders — the ground's top face under ANY
        // fence draws, the wall behind the connected arms shows)
        || crate::world::is_fence(kind)
        // THE WALL FAMILY the same verdict keeps (the dev-56 round —
        // the blocks clinit's forceSolidOn() row the 32 registrations
        // speak: the non-occluding compound the neighbors' faces draw
        // against, the AO the opaque walk never casts, the post + the
        // side strips the cell's own multipart lane renders — the
        // ground's top face under ANY wall draws, the wall behind the
        // connected strips shows)
        || crate::world::is_wall(kind)
        // THE HONEY BLOCK the same verdict keeps (the autopush-599
        // round — the blocks clinit's noOcclusion() pc 26888 the
        // registration speaks: the non-occluding cube the neighbors'
        // faces draw against, the AO the opaque walk never casts, the
        // two-element model the honey arm's own translucent quads
        // render)
        || kind == crate::world::HONEY_BLOCK
        // THE PORTAL the same verdict keeps (the autopush-605 round —
        // the blocks clinit's noCollision() pc 11091 the NETHER_PORTAL
        // registration speaks: the non-occluding plane the neighbors'
        // faces draw against, the AO the opaque walk never casts, the
        // thin-plane models the portal arm's own translucent quads
        // render)
        || kind == crate::world::NETHER_PORTAL
        // THE BAMBOO PAIR the same verdict keeps (the autopush-608
        // round — the blocks clinit's noOcclusion() pc 23180 the
        // BAMBOO registration speaks (the non-occluding stalk the
        // neighbors' faces draw against, the skylight the
        // propagatesSkylightDown override's iconst_1 keeps) and the
        // noCollision() pc 23110 the BAMBOO_SAPLING registration
        // speaks (the pass-through cross the flowers' own law keeps,
        // the AO the opaque walk never casts)
        || crate::world::is_bamboo_stalk(kind)
        || crate::world::is_bamboo_sapling(kind)
        // THE DOOR FAMILY the same verdict keeps (the autopush-655
        // round — the 3/16 panel the noOcclusion law the DoorBlock's
        // own isTransparent-true registration speaks: the non-
        // occluding thin box the neighbors' faces draw against, the
        // AO the opaque walk never casts, the element geometry the
        // door arm's own lane renders)
        || crate::world::is_door(kind)
        // THE FENCE GATE'S OWN ROW (the dev-53 round — the forceSolidOn
        // law the blocks clinit row pc 12890-12892 speaks: the gate
        // the panel the neighbors' faces draw against [the
        // getOcclusionShape walk pc 0-48 the two-post pair the SHAPE_
        // OCCLUSION map answers — the panel the occlusion never
        // banks], the AO the opaque walk never casts, the element
        // geometry the gate arm's own lane renders)
        || crate::world::is_fence_gate(kind)
        // THE TRAPDOOR FAMILY the same verdict keeps (the autopush-668
        // round — the 3/16 panel the noOcclusion law the blocks
        // clinit's own chain speaks: the non-occluding thin box the
        // neighbors' faces draw against, the AO the opaque walk never
        // casts, the element geometry the trapdoor arm's own lane
        // renders)
        || crate::world::is_trapdoor(kind)
        // THE GLASS FAMILY the same verdict keeps (the dev-57 round —
        // the blocks clinit's noOcclusion() pc 3782 the plain glass's
        // chain speaks, the stained ladder the collection lambda's own
        // chain the same row wears: the non-occluding cube the
        // neighbors' faces draw against, the AO the opaque walk never
        // casts, the full-cube quads the generic lane's cutout /
        // translucent arms render — the same-kind cull the
        // HalfTransparentBlock.skipRendering law the face loop's own
        // glass arm speaks)
        || crate::world::is_glass(kind)
        // THE TINTED GLASS the same verdict keeps (the dev-81 round —
        // javap TintedGlassBlock the 26.2 jar: the TransparentBlock
        // super the HalfTransparentBlock noOcclusion chain banks [the
        // same law the glass family's own row keeps]: the non-occluding
        // full cube the neighbors' faces draw against, the AO the
        // opaque walk never casts, the full-cube quads the generic
        // lane's translucent arm the per-texel blend serves — the
        // same-kind cull the super's own skipRendering pc 0-17 the
        // glass arm's tinted extension answers)
        || crate::world::is_tinted_glass(kind)
        // THE FLOOR HEADS the same verdict keeps (the dev-82 round —
        // javap SkullBlock the 26.2 jar the noOcclusion registration
        // the Properties chain speaks, the 8-px box the full cube
        // never is: the neighbors' faces draw against the skull cell,
        // the AO the opaque walk never casts, the six model faces the
        // mini-cube lane's own walk renders — the BE-rendered family
        // the chest precedent's own verdict shares)
        || crate::world::is_floor_head(kind)
        // THE WALL HEADS the same verdict keeps (the dev-83 round —
        // javap WallSkullBlock the 26.2 jar the probe
        // scripts/skull_probe_83.py banks: the wallVariant(floor)
        // registration the Properties full-copy the noOcclusion
        // verdict carries, the 8-px wall box the full cube never is —
        // the neighbors' faces draw against the wall cell, the wall
        // mini-cube lane's own walk the six model faces renders)
        || crate::world::is_wall_head_kind(kind)
        // THE PANE FAMILY the same verdict keeps (the dev-58 round —
        // the blocks clinit's noOcclusion() rows the glass_pane and
        // iron_bars chains speak [pc 12459 / pc 12322], the stained
        // ladder the collection lambda's own chain the same row
        // wears: the non-occluding thin strip the neighbors' faces
        // draw against, the AO the opaque walk never casts, the
        // multipart quads the pane lane's own arms render — the
        // stone behind the window draws, the same-kind cull the
        // skipRendering law the lane's own arm speaks)
        || crate::world::is_glass_pane(kind)
        || kind == crate::world::IRON_BARS
        // THE COPPER BARS FAMILY the same verdict keeps (the dev-59
        // round — javap lambda$static$163 the Properties chain the
        // noOcclusion() row pc 21 speaks: the eight oxidization rungs
        // the non-occluding thin strip the neighbors' faces draw
        // against, the AO the opaque walk never casts, the multipart
        // quads the pane lane's own arms render — the iron seam's own
        // law the copper rungs the class instances bank)
        || crate::world::is_copper_bars(kind)
        // THE CHAIN FAMILY the same verdict keeps (the dev-60 round —
        // javap lambda$static$165 the Properties chain the
        // noOcclusion() row pc 24 speaks: the nine links the
        // non-occluding thin band the neighbors' faces draw against,
        // the AO the opaque walk never casts, the element quads the
        // chain lane's own walk renders — the hanging link the light
        // passes through)
        || crate::world::is_chain(kind)
        // THE LANTERN FAMILY the same verdict keeps (the dev-61 round
        // — javap the Blocks clinit the LANTERN row pc 24639 the
        // noOcclusion() call the chain speaks [the SOUL_LANTERN row
        // pc 24694 the same call wears, the copper collection the
        // lambda$static$308 chain pc 32 the same row banks]: the ten
        // kinds the non-occluding compound the neighbors' faces draw
        // against, the AO the opaque walk never casts, the element
        // quads the lantern lane's own walk renders — the standing
        // body + top + hooks the cell's own light serves)
        || crate::world::is_lantern(kind)
        // THE STAIRS FAMILY the same verdict keeps (the dev-62 round —
        // the 64 kinds the canOcclude refusal the vanilla light law
        // speaks: the StairBlock rows the shape the FULL cube never is
        // (`isShapeFullBlock` the canOcclude verdict reads — the
        // slab + step union the false answer gives), the
        // propagatesSkylightDown law the same verdict keeps: the
        // half-open stair cell the neighbors' faces draw against, the
        // AO the opaque walk never casts, the element quads the stairs
        // lane's own walk renders)
        || crate::world::is_stairs(kind)
        // THE CHEST FAMILY the same verdict keeps (the dev-64 round —
        // the 11 kinds the BE-rendered boxes the FULL cube never is:
        // the 14/16-tall inset box the canOcclude verdict the false
        // answer gives, the propagatesSkylightDown law the same
        // verdict keeps: the chest cell the neighbors' faces draw
        // against, the AO the opaque walk never casts, the hand-built
        // quads the chest lane's own walk renders)
        || crate::world::is_chest(kind)
        // THE SLAB FAMILY the same verdict keeps (the dev-63 round —
        // the 68 kinds the useShapeForLightOcclusion law the vanilla
        // light speaks: the BOTTOM/TOP states the shape the FULL cube
        // never is (`isShapeFullBlock` the canOcclude verdict reads —
        // the half band the false answer gives), the propagatesSkylight
        // Down law the same verdict keeps: the half-open slab cell the
        // neighbors' faces draw against, the AO the opaque walk never
        // casts, the element quads the slab lane's own walk renders —
        // the DOUBLE state the full cube the same verdict the mesher's
        // is_transparent kind-only vocabulary keeps, the exact light
        // verdict the [`crate::light::sky_opacity_at`] variant-aware
        // walk serves)
        || crate::world::is_slab(kind)
}

/// Only opaque blocks cast ambient occlusion (air/water/leaves never darken
/// a corner — same policy as vanilla).
#[inline]
fn is_opaque(kind: u16) -> bool {
    !is_transparent(kind)
}

#[inline]
fn axis_delta(axis: usize, sign: i32) -> (i32, i32, i32) {
    match axis {
        0 => (sign, 0, 0),
        1 => (0, sign, 0),
        _ => (0, 0, sign),
    }
}

/// Per-corner AO levels (order matches `FACE_CORNERS[face]`) for the face of
/// the block at (wx, wy, wz). Samples the three blocks diagonally adjacent to
/// each corner in the plane one step in front of the face.
fn ao_levels(world: &World, wx: i32, wy: i32, wz: i32, face: usize) -> [u8; 4] {
    let (nx, ny, nz) = NEIGHBOR[face];
    let normal_axis = if nx != 0 { 0 } else if ny != 0 { 1 } else { 2 };
    let (t1, t2) = match normal_axis {
        0 => (1usize, 2usize),
        1 => (0usize, 2usize),
        _ => (0usize, 1usize),
    };
    let (bx, by, bz) = (wx + nx, wy + ny, wz + nz);
    let mut out = [3u8; 4];
    for (i, c) in FACE_CORNERS[face].iter().enumerate() {
        let s1 = if c[t1] >= 0.5 { 1 } else { -1 };
        let s2 = if c[t2] >= 0.5 { 1 } else { -1 };
        let d1 = axis_delta(t1, s1);
        let d2 = axis_delta(t2, s2);
        let side1 = is_opaque(world.get(bx + d1.0, by + d1.1, bz + d1.2));
        let side2 = is_opaque(world.get(bx + d2.0, by + d2.1, bz + d2.2));
        let corner = is_opaque(world.get(bx + d1.0 + d2.0, by + d1.1 + d2.1, bz + d1.2 + d2.2));
        out[i] = vertex_ao(side1, side2, corner);
    }
    out
}

/// Choose the vertex order whose fan diagonal (v0-v2) carries the brighter
/// AO pair — the classic anti-crease fix for interpolated smooth lighting.
fn flip_order(ao: &[u8; 4]) -> [usize; 4] {
    if ao[0] + ao[2] < ao[1] + ao[3] {
        [1, 2, 3, 0]
    } else {
        [0, 1, 2, 3]
    }
}

/// The vanilla LightTexture brightness ramp: `f = level/15`, `f/(4 - 3f)`.
/// Level 15 maps to EXACTLY 1.0 (open-sky faces keep the legacy look) and
/// level 0 to 0.0 (a known-dark cell is black — the vanilla cave look);
/// the curve's aggressive low end (level 8 ≈ 0.22) is the original's.
#[inline]
pub fn light_brightness(level: f32) -> f32 {
    let f = level / 15.0;
    f / (4.0 - 3.0 * f)
}

/// The vanilla lightmap's `BlockLightTint` — the `visual/block_light_tint`
/// environment attribute's DEFAULT, pinned in the jar's bytecode
/// (`EnvironmentAttributes.<clinit>`: `defaultValue(sipush -10100)`; the
/// attribute codec is `ExtraCodecs.STRING_RGB_COLOR` = `hexColor(6)`
/// xmap'd with `ARGB.opaque`/`ARGB.transparent`, so the canonical int is
/// the ARGB `0xFFFFD88C`, unpacked by `ARGB.vector3fFromRGB24` (`red =
/// (i>>16)&255, green = (i>>8)&255, blue = i&255` — the alpha bits fall
/// off the masks): R 255 / G 216 / B **140**. An earlier round shipped
/// 172 for the blue — a one-hex-digit misread of 0x8C as 0xAC
/// (`0xFFFFD8AC` would be -10068, not -10100); this round's audit
/// re-derived the whole chain from the bytecode and corrected it. No
/// dimension JSON in the registry's data and no timeline track overrides
/// it, so the default IS the whole value in every dimension the
/// registry carries — a STATED attribute (a custom server's) replaces
/// it verbatim (the model's `block_light_tint()`).
const DEFAULT_BLOCK_LIGHT_TINT: [f32; 3] = [1.0, 216.0 / 255.0, 140.0 / 255.0];

/// The lightmap's BYTECODE-default `NightVisionColor` —
/// `EnvironmentAttributes.<clinit>` defaultValue -6710887 =
/// 0xFF999999 = #999999 gray (javap; the attribute is
/// `spatiallyInterpolated().syncable()`, so a stated custom-server
/// value replaces it through the wire). The shader consumes it as
/// `NightVisionColor * NightVisionFactor` maxed against the ambient
/// base.
const DEFAULT_NIGHT_VISION_COLOR: [f32; 3] = [153.0 / 255.0; 3];

/// The lightmap's `NightVisionFactor` — the extractor's bytecode
/// (`LightmapRenderStateExtractor.extract`): the player's EFFECT
/// state, not a timeline and not the attribute. `hasEffect(NIGHT_VISION)`
/// -> `GameRenderer.nightVisionScale` (1.0, or `0.7 + sin((duration -
/// partialTick) · π · 0.2) · 0.3` while the effect ends within 200
/// ticks — the fade-out flicker); `waterVision > 0 &&
/// hasEffect(CONDUIT_POWER)` -> `getWaterVision()`; else exactly 0.0.
/// THE FACTOR NOW RIDES THE SNAPSHOT (`World::night_vision_factor`,
/// the model's own-effect store through the Update/Remove Mob Effect
/// wire transport) — this const is the FLOOR the effect-absent state
/// rides: every golden/procedural scene answers 0.0, so their
/// arithmetic is this value's identity (the term
/// `max(AmbientColor, NightVisionColor · 0.0)` is the ambient base
/// bit for bit for ANY tint — a color is non-negative). The flicker
/// is the documented non-goal (same verdict as the torch shimmer);
/// the conduit-power branch needs the underwater state the model
/// does not track and stays 0.0.
#[cfg_attr(not(test), expect(dead_code))]
const NIGHT_VISION_FACTOR: f32 = 0.0;

/// The vanilla lightmap's `BlockFactor` — the extractor's bytecode
/// (`LightmapRenderStateExtractor.extract`): `blockFactor =
/// blockLightFlicker + 1.4`, where the flicker is a mean-zero random
/// walk ((r1−r2)·r3·r4·0.1 decayed ×0.9 per tick — the torch shimmer).
/// A deterministic client cannot carry the random walk; it pins the
/// flicker's EXPECTATION (0) and keeps the constant 1.4 — the mean of
/// the vanilla range, the honest stand-in. The shimmer itself is the
/// documented non-goal (same verdict as the random end-flash).
const BLOCK_FACTOR: f32 = 1.4;

/// The lightmap shader's `parabolicMixFactor`: `(2*level − 1)^2` on the
/// RAW normalized block level — 1.0 at BOTH ends (level 0 and 15), a
/// dip to 0.0 at mid levels. The mix toward white therefore weakens at
/// mid-range (the pure `#ffd8ac` torch warmth) and strengthens at the
/// extremes (full torch light is nearly white-hot; the level-0 mix is
/// irrelevant in the sum since its brightness is 0).
#[inline]
fn parabolic_mix_factor(block_level: f32) -> f32 {
    let t = 2.0 * block_level - 1.0;
    t * t
}

/// The lightmap shader's `BlockLightColor` — `mix(BlockLightTint,
/// vec3(1.0), 0.9 * parabolicMixFactor(block_level))` with
/// `block_level = block / 15`, lightmap.fsh verbatim. The parabola
/// rides the RAW level, NOT the brightness ramp: the warmth peaks at
/// mid levels and the extremes whiten. The red channel is 1.0 for
/// every level (the tint's red is already 1.0). The tint is the
/// dimension's STATED `visual/block_light_tint` when one rides the
/// sync (the model's `block_light_tint()`, normalized to 0..1), the
/// bytecode default otherwise — `None` is the default's verdict,
/// never a guess.
#[inline]
fn block_light_color(block: f32, tint: Option<[f32; 3]>) -> [f32; 3] {
    let tint = tint.unwrap_or(DEFAULT_BLOCK_LIGHT_TINT);
    let mix = 0.9 * parabolic_mix_factor(block / 15.0);
    [
        tint[0] + mix * (1.0 - tint[0]),
        tint[1] + mix * (1.0 - tint[1]),
        tint[2] + mix * (1.0 - tint[2]),
    ]
}

/// The lightmap shader's `max(AmbientColor, nightVisionColor)` base
/// — lightmap.fsh verbatim: `vec3 nightVisionColor =
/// lightmapInfo.NightVisionColor * lightmapInfo.NightVisionFactor;
/// vec3 color = max(lightmapInfo.AmbientColor, nightVisionColor);`.
/// The tint is the dimension's STATED `visual/night_vision_color`
/// when one rides the sync (the model's `night_vision_color()`,
/// normalized to 0..1), the bytecode default otherwise — `None` is
/// the default's verdict, never a guess. The factor rides as a
/// parameter so the term's contract is pinnable at a lifted state
/// (the effect system's arrival); at this client's floor 0.0 the
/// product is exactly zero and the max returns the ambient base bit
/// for bit for ANY tint.
#[inline]
fn night_vision_base(ambient: [f32; 3], tint: Option<[f32; 3]>, factor: f32) -> [f32; 3] {
    let t = tint.unwrap_or(DEFAULT_NIGHT_VISION_COLOR);
    [
        ambient[0].max(t[0] * factor),
        ambient[1].max(t[1] * factor),
        ambient[2].max(t[2] * factor),
    ]
}

/// The vanilla lightmap combine, PER CHANNEL — `lightmap.fsh`'s
/// `color = max(AmbientColor, nightVisionColor); color += SkyLightColor *
/// get_brightness(sky) * SkyFactor + BlockLightColor *
/// get_brightness(block) * BlockFactor`, under the shader's single final
/// `clamp(color, 0, 1)` (the lightmap texture's range; the terms are
/// non-negative so only the 1.0 side can bite). `ambient` is the shader's
/// AmbientColor BASE the sky/block terms are added ON TOP of — the
/// dimension's `visual/ambient_light_color` attribute (overworld
/// #0a0a0a, nether #302821, end #3f473f; the bytecode default black when
/// no opinion — a no-op base, so `None` keeps the pre-ambient arithmetic
/// bit for bit). `NightVisionColor` is the dimension's STATED
/// `visual/night_vision_color` (or the `#999999` bytecode default,
/// gray) UNDER the night-vision factor — the player's EFFECT state,
/// [`NIGHT_VISION_FACTOR`] 0.0 at this client's state, so the base is
/// `max(A, C·0.0) = A` bit for bit (the term rides for the day the
/// effect system lifts the factor — [`night_vision_base`]).
/// `SkyLightColor` is the
/// timeline's `visual/sky_light_color` track (`timeline::sky_light_color`:
/// the white default by day, `#7a7aff` — the moonlight blue — through the
/// night plateau). `BlockLightColor` is [`block_light_color`] — the
/// stated tint (or the `#ffd88c` bytecode default, the corrected blue
/// 140) mixed 0.9-parabola toward white on the raw
/// level — scaled by `BlockFactor` 1.4, the flicker's expectation (the
/// random walk itself is the documented non-goal). The remaining shader
/// terms: the DarknessScale arm the player's own effect state carries —
/// the shader's `color = color - vec3(lightmapInfo.DarknessScale);`
/// subtract (javap `LightmapRenderStateExtractor` pc 304-344 + the
/// 26.2 jar's `shaders/core/lightmap.fsh` verbatim) BEFORE the clamp —
/// the 0.0 no-effect floor keeps the fold bit for bit (the `x - 0.0`
/// identity, the bottom clamp no-op on the non-negative terms) —, the
/// BossOverlay 0 identity, and the
/// brightness (gamma) mix is an identity at gamma 0. A layer the model
/// never answered for is `None`, not zero; no layer at all is the legacy
/// unshaded look.
#[inline]
pub(crate) fn lightmap_combine(
    sky: Option<f32>,
    block: Option<f32>,
    sky_factor: f32,
    sky_tint: [f32; 3],
    ambient: [f32; 3],
    block_tint: Option<[f32; 3]>,
    night_vision: Option<[f32; 3]>,
    night_vision_factor: f32,
    darkness_scale: f32,
) -> [f32; 3] {
    let sky_term = sky.map(|v| light_brightness(v) * sky_factor);
    let block_term = block.map(|v| {
        let b = light_brightness(v) * BLOCK_FACTOR;
        let c = block_light_color(v, block_tint);
        [b * c[0], b * c[1], b * c[2]]
    });
    if sky_term.is_none() && block_term.is_none() {
        return [1.0; 3]; // no light layer — the legacy look
    }
    // the shader's base: max(AmbientColor, NightVisionColor *
    // NightVisionFactor) — the factor rides the snapshot (the effect
    // state's transport); at the effect-absent floor 0.0 it is the
    // ambient color bit for bit
    let mut color = night_vision_base(ambient, night_vision, night_vision_factor);
    if let Some(s) = sky_term {
        color[0] += s * sky_tint[0];
        color[1] += s * sky_tint[1];
        color[2] += s * sky_tint[2];
    }
    if let Some(b) = block_term {
        color[0] += b[0];
        color[1] += b[1];
        color[2] += b[2];
    }
    // THE DARKNESS ARM (lightmap.fsh verbatim: `color = color -
    // vec3(lightmapInfo.DarknessScale);` — the subtract the shader
    // speaks BEFORE the clamp; the 0.0 floor the `x - 0.0` f32
    // identity keeps bit for bit)
    color[0] -= darkness_scale;
    color[1] -= darkness_scale;
    color[2] -= darkness_scale;
    // the shader's own clamp BOTH sides (`clamp(color, 0.0, 1.0)`) —
    // the bottom floor the subtraction's negatives land on (the
    // no-effect floor never wears it — every term above non-negative)
    [
        color[0].min(1.0).max(0.0),
        color[1].min(1.0).max(0.0),
        color[2].min(1.0).max(0.0),
    ]
}

/// The cell's two RAW light layers, as the engine reported them — sky
/// and block are kept apart through the whole smooth-lighting walk so
/// the night sky factor can scale the sky term AFTER the brightness
/// ramp (and the timeline's moonlight tint can COLOR it — the
/// `visual/sky_light_color` track), exactly where the vanilla lightmap
/// shader applies SkyFactor and SkyLightColor
/// (`get_brightness(sky) * SkyFactor`, additive with the block term;
/// 26.2 jar: `assets/minecraft/shaders/core/lightmap.fsh` +
/// `LightmapRenderState.skyFactor`). A layer the model never answered
/// for is `None`, not zero.
#[inline]
fn cell_layers(light: &LightGrid, side: usize, x: i32, y: i32, z: i32) -> (Option<u8>, Option<u8>) {
    light.cell(side, x, y, z)
}

/// Per-corner smooth-light levels (FACE_CORNERS order): the average of
/// the 4 face-plane cells around each corner — the face cell plus the 3
/// AO diagonals, the SAME cells [`ao_levels`] samples. SKY and BLOCK
/// are averaged SEPARATELY (each over its own known cells — the
/// vanilla client averages the packed light's two halves independently;
/// the old combined-cell max was the pre-night simplification). The
/// average is float (smoother than the vanilla packed int average, a
/// deliberate minimal departure). A corner with no known cell at all
/// in BOTH layers keeps the legacy unshaded look — unsent neighbors
/// never darken the faces they touch. Returns `([sky; 4], [block; 4])`.
fn corner_light(
    light: &LightGrid,
    side: usize,
    wx: i32,
    wy: i32,
    wz: i32,
    face: usize,
) -> ([Option<f32>; 4], [Option<f32>; 4]) {
    let (nx, ny, nz) = NEIGHBOR[face];
    let normal_axis = if nx != 0 { 0 } else if ny != 0 { 1 } else { 2 };
    let (t1, t2) = match normal_axis {
        0 => (1usize, 2usize),
        1 => (0usize, 2usize),
        _ => (0usize, 1usize),
    };
    let (bx, by, bz) = (wx + nx, wy + ny, wz + nz);
    let mut out_sky = [None; 4];
    let mut out_block = [None; 4];
    for (i, c) in FACE_CORNERS[face].iter().enumerate() {
        let s1 = if c[t1] >= 0.5 { 1 } else { -1 };
        let s2 = if c[t2] >= 0.5 { 1 } else { -1 };
        let d1 = axis_delta(t1, s1);
        let d2 = axis_delta(t2, s2);
        let cells = [
            (bx, by, bz),
            (bx + d1.0, by + d1.1, bz + d1.2),
            (bx + d2.0, by + d2.1, bz + d2.2),
            (bx + d1.0 + d2.0, by + d1.1 + d2.1, bz + d1.2 + d2.2),
        ];
        let mut sum_sky = 0.0f32;
        let mut n_sky = 0u32;
        let mut sum_block = 0.0f32;
        let mut n_block = 0u32;
        for &(x, y, z) in &cells {
            let (s, b) = cell_layers(light, side, x, y, z);
            if let Some(v) = s {
                sum_sky += v as f32;
                n_sky += 1;
            }
            if let Some(v) = b {
                sum_block += v as f32;
                n_block += 1;
            }
        }
        out_sky[i] = (n_sky > 0).then(|| sum_sky / n_sky as f32);
        out_block[i] = (n_block > 0).then(|| sum_block / n_block as f32);
    }
    (out_sky, out_block)
}

/// Vanilla face->UV orientation. Texture row 0 is the top; side faces keep
/// the block's +Y at v=0; each side is unmirrored when viewed from outside;
/// top maps u=x, v=z (north at texture top), bottom mirrors v.
pub fn face_uv(face: usize, c: [f32; 3]) -> [f32; 2] {
    match face {
        0 => [1.0 - c[2], 1.0 - c[1]], // +X east
        1 => [c[2], 1.0 - c[1]],       // -X west
        2 => [c[0], c[2]],             // +Y up
        3 => [c[0], 1.0 - c[2]],       // -Y down
        4 => [c[0], 1.0 - c[1]],       // +Z south
        _ => [1.0 - c[0], 1.0 - c[1]], // -Z north
    }
}

/// THE DIG LANE's breaking overlay quads — the six faces of the targeted
/// cell sampling the destroy_stage crack tile the progress law names
/// (the vanilla `DestroyStage.getDestructor` index). The faces inflate
/// by the 0.0015 epsilon (the vanilla crumbling render type's polygon
/// offset stand-in — the crack renders in front of the block's own
/// surface, never z-fighting it), the UVs ride the exact `face_uv`
/// orientation the terrain speaks, the vertex colors stay 1.0 (the
/// crack's own grays blend over the block's lit surface — the vanilla
/// crumbling pass's own shape).
pub fn destroy_overlay_quads(pos: (i32, i32, i32), tile: u16) -> Vec<Quad> {
    let (px, py, pz) = (pos.0 as f32, pos.1 as f32, pos.2 as f32);
    const EPS: f32 = 0.0015;
    let infl = |v: f32| if v <= 0.0 { -EPS } else { 1.0 + EPS };
    let mut out = Vec::with_capacity(6);
    for face in 0..6usize {
        let mut pts = [[0f32; 3]; 4];
        let mut uvs = [[0f32; 2]; 4];
        for (i, c) in FACE_CORNERS[face].iter().enumerate() {
            pts[i] = [px + infl(c[0]), py + infl(c[1]), pz + infl(c[2])];
            uvs[i] = face_uv(face, *c);
        }
        out.push(Quad {
            pts,
            colors: [[1.0; 3]; 4],
            uvs,
            tile,
            translucent: false,
                            per_texel_alpha: false,
        });
    }
    out
}

/// THE RAIL'S OWN CRACK BAND (the dev-88 round — the vanilla destroy
/// overlay the rail's SHAPE_FLAT `Block.column(16.0, 0.0, 2.0)` speaks):
/// the six faces the band top's own height wraps — the top plane the
/// band's own top carries, the bottom the cell floor keeps, the four
/// sides the strip wears, the full tile uv the every-face sample keeps,
/// the same 0.0015 epsilon the crumbling render type's polygon offset
/// speaks. THE ASCENT'S OWN SPAN (the dev-94 round — the vanilla
/// SHAPE_ASCENDING box the ascent bytes wear): the caller the shape's
/// own top passes — the flat bytes the 2-px the SHAPE_FLAT speaks, the
/// ascent bytes the 8-px the same getShape's isAscending() arm serves.
pub fn destroy_overlay_quads_band(pos: (i32, i32, i32), tile: u16, band_top: f32) -> Vec<Quad> {
    let (px, py, pz) = (pos.0 as f32, pos.1 as f32, pos.2 as f32);
    const EPS: f32 = 0.0015;
    // the y fold the band's own span names: the low corners the
    // floor's below keeps, the high corners the band top's above wears
    let yfold = |v: f32| if v <= 0.0 { -EPS } else { band_top + EPS };
    let infl = |v: f32| if v <= 0.0 { -EPS } else { 1.0 + EPS };
    let mut out = Vec::with_capacity(6);
    for face in 0..6usize {
        let mut pts = [[0f32; 3]; 4];
        let mut uvs = [[0f32; 2]; 4];
        for (i, c) in FACE_CORNERS[face].iter().enumerate() {
            pts[i] = [px + infl(c[0]), py + yfold(c[1]), pz + infl(c[2])];
            uvs[i] = face_uv(face, *c);
        }
        out.push(Quad {
            pts,
            colors: [[1.0; 3]; 4],
            uvs,
            tile,
            translucent: false,
            per_texel_alpha: false,
        });
    }
    out
}

/// The flat ground-cover plane: leaf_litter's model elements sit at
/// y = 0.25 model units (of 16) — a quarter pixel above the floor
/// (the 13th audit: template_leaf_litter_1..4, all `from.y == to.y == 0.25`).
const FLAT_Y: f32 = 0.25 / 16.0;

/// Mesh one chunk: all faces where a solid block touches a transparent one.
pub fn mesh_chunk(world: &World, cx: usize, cz: usize, lut: Option<&TextureLut>) -> Vec<Quad> {
    let mut out = Vec::with_capacity(2048);
    let light = world.light.as_ref();
    let bx0 = (cx * CHUNK_X) as i32;
    let bz0 = (cz * CHUNK_Z) as i32;
    for ly in 0..CHUNK_Y {
        for lz in 0..CHUNK_Z {
            for lx in 0..CHUNK_X {
                let id = world.chunks[cx + cz * world.chunks_per_side]
                    [lx + lz * CHUNK_X + ly * CHUNK_X * CHUNK_Z];
                if id == AIR {
                    continue;
                }
                let wx = bx0 + lx as i32;
                let wy = ly as i32;
                let wz = bz0 + lz as i32;
                let water_self = id == WATER;
                // THE SECOND FLUID (the lava lane): the same fluid shape
                // the water lane speaks — the surface faces draw only
                // against air, the body's own neighbors stay silent —
                // but OPAQUE (the lava_still pixels never ride the
                // translucent pass the water speaks)
                let lava_self = id == LAVA;
                // THE REDSTONE DUST'S OWN FLAT LANE (the autopush-665
                // round — javap the blockstates/redstone_wire.json
                // multipart the 9 parts the pixel probes speak): the
                // zero-thickness plane the y=0.25 quarter-pixel height
                // the leaf_litter's own FLAT_Y wears, the up+down faces
                // BOTH the model elements speak (no cullface — both
                // faces draw unconditionally; the raster backface-culls
                // each to its own side). THE SHAPE THE PARTS' OWN OR
                // ARMS BANK: the dot center part[0] the all-none OR the
                // perpendicular-pair arms serve ((east+north) or
                // (east+south) or (south+west) or (north+west) — the
                // corner turn the center the arms' own meeting wears),
                // the side arms parts[1..4] the connected sides' half
                // rects serve (north z 0..8 the line0 master's
                // [0,0,16,8] window u=x/v=z, south z 8..16 the [0,8,16,
                // 16] window u=x/v=z, east x 8..16 the line1 master's
                // [0,8,16,16] window u=z/v=x — the y=270 rotation the
                // transposed map the side_alt1 row wears, west x 0..8
                // the [0,0,16,8] window u=z/v=x — the same rotation the
                // side1 row keeps). THE CONNECTION VERDICT (javap
                // shouldConnectTo pc 0-96: the REDSTONE_WIRE.is() arm +
                // the isSignalSource arm the lever/button/plate
                // families the same verdict wears — the arm the
                // adjacent wire OR source kind answers). The UP
                // parts[5..8] the climbing wall arms the binary lane
                // never banks (the same-level run the first lane walks).
                // The overlay the translucent blend the alpha lane a
                // later round wears (the dust renders as authored minus
                // the rim). NO variant byte the wire reads (the state
                // the kind pair alone banks), ambientocclusion false —
                // shade and light only, NO AO_CURVE term.
                if crate::world::is_redstone_wire(id) {
                    let connected = |dx: i32, dz: i32| -> bool {
                        let cell = world.get(wx + dx, wy, wz + dz);
                        crate::world::is_redstone_wire(cell)
                            || crate::world::is_lever(cell)
                            || crate::world::is_button(cell)
                            || crate::world::is_pressure_plate(cell)
                    };
                    let (north, south) = (connected(0, -1), connected(0, 1));
                    let (west, east) = (connected(-1, 0), connected(1, 0));
                    // THE DOT CENTER'S OWN VERDICT (part[0]'s OR arms —
                    // the all-none arm OR any perpendicular pair the
                    // center wears; the straight N-S / E-W runs the
                    // arms the center never shares)
                    let perp_pair = (east && north) || (east && south)
                        || (west && north) || (west && south);
                    let show_dot = (!north && !south && !west && !east) || perp_pair;
                    // THE TILES' OWN SLOTS (the kind row's dot/N-S/E-W
                    // seats the generator's wire bank slots 0..2 carry —
                    // the OFF row the power-0 colored masters, the ON
                    // row the power-15 tiles)
                    let tiles = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize]
                    } else {
                        [atlas::TILE_NONE; 6]
                    };
                    let dot_tile = tiles[0];
                    let ns_tile = tiles[1];
                    let ew_tile = tiles[2];
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let fy = wy as f32 + FLAT_Y;
                    // THE PIECES' OWN TABLE (the rect the block units
                    // speak, the uv window the master's pixels wear,
                    // the transposed flag the E-W arms' rotation map
                    // keeps): (x0, z0, x1, z1, u0, v0, u1, v1, tile,
                    // transposed)
                    let mut pieces: Vec<(f32, f32, f32, f32, f32, f32, f32, f32, u16, bool)> =
                        Vec::with_capacity(3);
                    if show_dot {
                        pieces.push((0.0, 0.0, 16.0, 16.0, 0.0, 0.0, 16.0, 16.0, dot_tile, false));
                    }
                    if north {
                        pieces.push((0.0, 0.0, 16.0, 8.0, 0.0, 0.0, 16.0, 8.0, ns_tile, false));
                    }
                    if south {
                        pieces.push((0.0, 8.0, 16.0, 16.0, 0.0, 8.0, 16.0, 16.0, ns_tile, false));
                    }
                    if east {
                        pieces.push((8.0, 0.0, 16.0, 16.0, 0.0, 8.0, 16.0, 16.0, ew_tile, true));
                    }
                    if west {
                        pieces.push((0.0, 0.0, 8.0, 16.0, 0.0, 0.0, 16.0, 8.0, ew_tile, true));
                    }
                    for face in [2usize, 3] {
                        // THE TINT CARRIER (the pre-colored pixels the
                        // TINTS row's zeros keep — the resolved tint the
                        // no-op law the wire rows never speak)
                        let base = world
                            .face_tint(id, face, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3]);
                        let shade = FACE_SHADE[face];
                        let clight = light
                            .map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                        for &(px0, pz0, px1, pz1, u0, v0, u1, v1, tile, transposed) in
                            pieces.iter()
                        {
                            if tile == atlas::TILE_NONE {
                                continue;
                            }
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                                // THE CORNER'S OWN POSITION INSIDE THE
                                // PIECE RECT (block units), the uv the
                                // WINDOW's own map serves (the N-S arms
                                // u=x/v=z, the E-W arms the transposed
                                // u=z/v=x the y=270 rotation's rigid
                                // ride keeps; the down face the
                                // vanilla's own flip the mirrored
                                // within-window v wears)
                                let bx = px0 + c[0] * (px1 - px0);
                                let bz = pz0 + c[2] * (pz1 - pz0);
                                let (mut uu, mut vv) = if transposed {
                                    (bz, bx)
                                } else {
                                    (bx, bz)
                                };
                                uu = uu.clamp(u0, u1) / 16.0;
                                vv = vv.clamp(v0, v1) / 16.0;
                                uvs[i] = if face == 2 {
                                    [uu, vv]
                                } else {
                                    [uu, (v0 + v1) / 16.0 - vv]
                                };
                                pts[i] = [
                                    wx as f32 + bx / 16.0,
                                    fy,
                                    wz as f32 + bz / 16.0,
                                ];
                                let lm = match clight.as_ref() {
                                    Some((sky, block)) => lightmap_combine(
                                        sky[i],
                                        block[i],
                                        sky_factor,
                                        sky_tint,
                                        ambient,
                                        block_tint,
                                        night_vision,
                                        world.night_vision_factor,
                                        world.darkness_scale,
                                    ),
                                    None => [1.0; 3], // no light layer — legacy look
                                };
                                let m = [shade * lm[0], shade * lm[1], shade * lm[2]];
                                colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                                per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE RAIL'S OWN FLAT LANE (the dev-86 round — the
                // blockstates/rail.json's shape rows the models the
                // probe scripts/rail_probe_86.py banks): the rail_flat
                // parent's zero-thickness element the [0,1,0]->
                // [16,1,16] span speaks — the full-footprint plane at
                // y=1/16, the up face uv [0,0,16,16] + the down face
                // the flipped v the model's own [0,16,16,0] window
                // wears, ambientocclusion false — shade and light
                // only, NO AO_CURVE term; no cullface — both faces
                // draw unconditionally (the raster backface-culls
                // each to its own side). THE SHAPE THE VARIANT BYTE
                // READS: the 0 byte the NORTH_SOUTH default (the
                // registerDefaultState pin the placement's Z-axis arm
                // stamps), the 1 byte the EAST_WEST quarter — the
                // blockstate's y=90 row the model rotation the
                // transposed uv fold answers (the u the z-axis the
                // quarter turns wear — the wire lane's own E-W law
                // the same transpose keeps; the vanilla rail texture
                // the symmetric ties the mirror direction the honest
                // approximation keeps). The four kinds the lane's own
                // tile row serves (the FACE_TILES [2] up seat the
                // flat arm banks — the OFF masters the placement
                // wears, the _on pair the powered swap the redstone
                // rounds keep).
                if crate::world::is_rail(id) {
                    // THE CORNER SEAT (the dev-92 round — the vanilla
                    // blockstates/rail.json's corner rows the model
                    // rotation the byte resolves: the curve bytes the
                    // plain RAIL's perpendicular pair banks — the
                    // [`crate::world::rail_curve_byte`] verdict — the
                    // rail_corner texture the pool tail seats, the
                    // y=0/90/180/270 arms the uv rotation folds
                    // below wear; the straight bytes the kind's own
                    // row the flat arm keeps). THE SHAPE NIBBLE (the
                    // dev-96 round — the variant's own bit layout the
                    // mask reads: the low nibble the shape byte, the
                    // 0x80 the POWERED bit the powered rails wear —
                    // the shape arms the masked byte alone answer)
                    let shape = world.variant(wx, wy, wz) & crate::world::RAIL_SHAPE_MASK;
                    let corner = matches!(shape, 6..=9);
                    // THE POWERED RAIL'S OWN ON SEAT (the dev-96 round
                    // — the vanilla blockstates/powered_rail.json's
                    // powered=true rows the model swap the bit reads:
                    // the POWERED bit the powered_rail_on texture the
                    // seat banks, the powered=false rows the kind's
                    // own row the off master keeps)
                    // THE ACTIVATOR RAIL'S OWN TWIN (the dev-97 round
                    // — the vanilla blockstates/activator_rail.json's
                    // powered=true rows the same bit the seat reads:
                    // the activator's own _on texture the kind's own
                    // const banks, the powered kind the dev-96 seat
                    // keeps, the powered=false rows the off master)
                    // THE DETECTOR RAIL'S OWN SEAT (the dev-98 round
                    // — the vanilla blockstates/detector_rail.json's
                    // powered=true rows the same bit the lane reads:
                    // the detector_rail_on texture the kind's own
                    // const banks; the WORLD law the cart detection
                    // its own lane serves, the neighbor signal the
                    // detector's bit never arms — the mesher renders
                    // whatever bit the world banks)
                    let powered_on = matches!(
                        id,
                        crate::world::POWERED_RAIL
                            | crate::world::DETECTOR_RAIL
                            | crate::world::ACTIVATOR_RAIL
                    ) && world.variant(wx, wy, wz) & crate::world::RAIL_POWERED_BIT != 0;
                    let tile = if corner {
                        atlas::TILE_RAIL_CORNER
                    } else if powered_on && id == crate::world::ACTIVATOR_RAIL {
                        atlas::TILE_ACTIVATOR_RAIL_ON
                    } else if powered_on && id == crate::world::DETECTOR_RAIL {
                        atlas::TILE_DETECTOR_RAIL_ON
                    } else if powered_on {
                        atlas::TILE_POWERED_RAIL_ON
                    } else if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let transposed = shape == 1;
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let fy = wy as f32 + 1.0 / 16.0;
                    for face in [2usize, 3] {
                        let base = world
                            .face_tint(id, face, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3]);
                        let shade = FACE_SHADE[face];
                        let clight = light
                            .map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                        if tile == atlas::TILE_NONE {
                            continue;
                        }
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                            // THE CORNER'S OWN POSITION INSIDE THE FULL
                            // RECT (block units), the uv the model's
                            // own map serves (the N-S default u=x/v=z,
                            // the E-W quarter the transposed u=z/v=x
                            // the y=90 rotation's rigid ride keeps;
                            // the down face the vanilla's own flip the
                            // mirrored v wears)
                            // THE CORNER'S OWN ROTATION FOLD (the dev-92
                            // round — the blockstate's y-rotated model
                            // the uv the inverse mapping samples: the
                            // SE byte the identity wears, the SW byte
                            // the y=90 arm (u=z, v=16-x), the NW byte
                            // the y=180 arm (16-x, 16-z), the NE byte
                            // the y=270 arm (16-z, x) — the texture's
                            // own east+south bend the edges the decoded
                            // PNG confirms, the down face the vanilla's
                            // own v-flip the same fold keeps)
                            let bx = c[0] * 16.0;
                            let bz = c[2] * 16.0;
                            let (uu, vv) = if transposed {
                                (bz, bx)
                            } else {
                                match shape {
                                    7 => (bz, 16.0 - bx),
                                    8 => (16.0 - bx, 16.0 - bz),
                                    9 => (16.0 - bz, bx),
                                    // THE ASCENT'S OWN UV (the dev-93
                                    // round — the y=90-rotated raised
                                    // template the E-W bytes wear: the u
                                    // the z-axis tracks, the v the x-axis
                                    // runs the slope along — the 16-x
                                    // mirror the y=90 mapping keeps)
                                    2 | 3 => (bz, 16.0 - bx),
                                    _ => (bx, bz),
                                }
                            };
                            let (uu, vv) = (uu / 16.0, vv / 16.0);
                            uvs[i] = if face == 2 {
                                [uu, vv]
                            } else {
                                [uu, 1.0 - vv]
                            };
                            // THE ASCENT'S OWN SLOPE (the dev-93 round —
                            // the raised_ne/sw template's 45-degree
                            // rotation the rescale the span keeps: the
                            // plane the flat seat at the low edge wears,
                            // the y=17/16 seam at the high edge reaches
                            // — the upper cell's own rail seat the one
                            // plane connects, the low/high arms the
                            // byte's direction answers)
                            let py = match shape {
                                2 => wy as f32 + 1.0 / 16.0 + bx / 16.0,
                                3 => wy as f32 + 17.0 / 16.0 - bx / 16.0,
                                4 => wy as f32 + 17.0 / 16.0 - bz / 16.0,
                                5 => wy as f32 + 1.0 / 16.0 + bz / 16.0,
                                _ => fy,
                            };
                            pts[i] = [
                                wx as f32 + bx / 16.0,
                                py,
                                wz as f32 + bz / 16.0,
                            ];
                            let lm = match clight.as_ref() {
                                Some((sky, block)) => lightmap_combine(
                                    sky[i],
                                    block[i],
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                ),
                                None => [1.0; 3], // no light layer — legacy look
                            };
                            let m = [shade * lm[0], shade * lm[1], shade * lm[2]];
                            colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // The 13th-audit FLAT path: leaf_litter is a zero-thickness
                // ground cover, not a cube. Its blockstate is a multipart
                // (leaf_litter_1..4 x facing rotations) and every element
                // has EXACTLY up+down faces at y=0.25, both tintindex 0 ->
                // the dry fold (DRY_SLOT), with `ambientocclusion: false`
                // (no corner AO — the model verdict) and no cullface (both
                // faces draw unconditionally; the raster backface-culls
                // each to its own side).
                // THE 14TH AUDIT: the state's properties ride the variants
                // grid — `segment_amount` picks the coverage rect(s) (the
                // template elements' exact geometry; the multipart's model
                // sets are 1->{m1}, 2->{m2}, 3->{m2,m3}, 4->{m4}) and
                // `facing` the y-rotation (north=0 / east=90 / south=180 /
                // west=270, the blockstate's own table). No opinion
                // (variant 0 — the procedural scenes, the pre-variants
                // look) = the full footprint at y-rot 0, BIT FOR BIT the
                // 13th-audit emission: segment 4 IS the full rect and the
                // template uv arithmetic collapses to the old `face_uv`
                // exactly at that rect.
                if id == LEAF_LITTER {
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // The attribute-system fold (javap EnvironmentAttributeSystem:
                    // the dimension's constant layer OVERRIDES the default, the
                    // timeline's multiply track applies last): the stated factor
                    // is the BASE, the day tick's curve value scales it. None =
                    // the default 1.0 — an exact f32 identity, the golden
                    // scenes' bit-stability. Constant per mesh — the shader's
                    // SkyFactor uniform equivalent.
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    // The shader's SkyLightColor uniform: the
                    // attribute-system fold for the COLOR attribute
                    // (javap ColorModifier.MULTIPLY_RGB ->
                    // ARGB.multiply(int, int): the -1 white default
                    // short-circuits to the other operand, else
                    // per-channel imul / idiv 255 — the probe's folded
                    // value IS an Integer the extractor unpacks). None
                    // = the -1 default's shape — the track verbatim,
                    // the exact expression this path shipped before the
                    // stated attribute; a stated base (the end #ac60cd,
                    // the nether #7a7aff, a custom server's wire)
                    // multiplies the byte-space track per channel.
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's BlockLightTint: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#FFD88C, the corrected blue 140).
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's NightVisionColor: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#999999 gray) — consumed UNDER the
                    // night-vision factor (the effect state, 0.0 here).
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let fy = wy as f32 + FLAT_Y;
                    let variant = world
                        .variants
                        .as_ref()
                        .and_then(|g| g.get(cx + cz * world.chunks_per_side))
                        .map(|g| g[lx + lz * CHUNK_X + ly * CHUNK_X * CHUNK_Z])
                        .unwrap_or(0);
                    let (rot_idx, segment) = if variant == 0 {
                        (0usize, 4u8)
                    } else {
                        let v = variant - 1;
                        ((v & 3) as usize, (v >> 2) + 1)
                    };
                    // (x0, z0, x1, z1) in TEXTURE PIXELS — the audited
                    // template elements: model_1 the NW quarter, model_2
                    // the west strip, model_3 the SE quarter, model_4 the
                    // full cover.
                    let rects: &[(f32, f32, f32, f32)] = match segment {
                        1 => &[(0.0, 0.0, 8.0, 8.0)],
                        2 => &[(0.0, 0.0, 8.0, 16.0)],
                        3 => &[(0.0, 0.0, 8.0, 16.0), (8.0, 8.0, 16.0, 16.0)],
                        _ => &[(0.0, 0.0, 16.0, 16.0)],
                    };
                    let theta = rot_idx as f32 * std::f32::consts::FRAC_PI_2;
                    let (sn, cs) = theta.sin_cos();
                    for face in [2usize, 3] {
                        let base = if tile == atlas::TILE_NONE {
                            // The blockstate-driven palette, VARIANT-AWARE
                            // (v0.3.80): segment_amount selects the template
                            // model(s) and each samples a different uv rect,
                            // so the flat color follows the variant — the
                            // same precedence as the cube path below (the
                            // palette first, then the runtime-texture LUT,
                            // then the hand-written defaults).
                            crate::colors::palette_leaf_litter_color(segment, face)
                                .or_else(|| lut.and_then(|l| l.color(id, face)))
                                .unwrap_or_else(|| default_color(id, face))
                        } else {
                            // The same tint carrier as the cube path: the
                            // synced biome's dry-fold color when it differs
                            // from the baked plains sample (the no-op rule),
                            // multiplied into the vertex colors.
                            world
                                .face_tint(id, face, wx, wy, wz)
                                .map(tint::unpack)
                                .unwrap_or([1.0; 3])
                        };
                        let shade = FACE_SHADE[face];
                        // per-corner light in front of the face plane (the
                        // client-wide convention); `None` keeps legacy look
                        let clight =
                            light.map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                        for &(px0, pz0, px1, pz1) in rects {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                                // The corner's UNROTATED position inside the
                                // rect (block units), uv GLUED to it — the
                                // template's per-face uv arithmetic (up:
                                // u = x, v = z; down mirrors v WITHIN the
                                // rect, v = (z0 + z1 - z)/16 — exactly the
                                // old `face_uv` 1 - z at the full rect).
                                let ux = (px0 + c[0] * (px1 - px0)) / 16.0;
                                let uz = (pz0 + c[2] * (pz1 - pz0)) / 16.0;
                                uvs[i] = if face == 2 {
                                    [ux, uz]
                                } else {
                                    [ux, (pz0 + pz1) / 16.0 - uz]
                                };
                                // The blockstate's y-rotation about the cell
                                // center; the uv stays glued (the element
                                // turns, the texture turns with it).
                                let rx = 0.5 + (ux - 0.5) * cs - (uz - 0.5) * sn;
                                let rz = 0.5 + (ux - 0.5) * sn + (uz - 0.5) * cs;
                                pts[i] = [wx as f32 + rx, fy, wz as f32 + rz];
                                // The vanilla lightmap combine, VERBATIM
                                // (the shared per-channel core both paths
                                // call — see `lightmap_combine`), riding the
                                // dimension's ambient base.
                                let lm = match clight.as_ref() {
                                    Some((sky, block)) => lightmap_combine(
                                        sky[i],
                                        block[i],
                                        sky_factor,
                                        sky_tint,
                                        ambient,
                                        block_tint,
                                        night_vision,
                                        world.night_vision_factor,
                                        world.darkness_scale,
                                    ),
                                    None => [1.0; 3], // no light layer — legacy look
                                };
                                // `ambientocclusion: false` — shade and light
                                // only, NO AO_CURVE term.
                                let m = [shade * lm[0], shade * lm[1], shade * lm[2]];
                                colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE 15TH-AUDIT WHEAT PATH: the crop is the SECOND
                // non-cube member. blockstates/wheat.json age=0 ->
                // block/wheat_stage0 -> block/crop, whose FOUR
                // zero-thickness vertical planes (x=4 and x=12 spanning
                // z 0..16, z=4 and z=12 spanning x 0..16, each y -1..15
                // — the sprite sinks one pixel into the farmland below)
                // carry `shade: false` faces under a model-level
                // `ambientocclusion: false` — the vanilla tesselateWithoutAO
                // shape: no AO, no FACE_SHADE, every corner sampling the
                // CELL's own light (getLightColor(state, pos) the
                // withoutAO walk speaks). No cullface anywhere in the
                // model — the planes draw unconditionally; each plane's
                // two faces ride the cube path's own winding convention
                // (FACE_CORNERS per face direction, the raster
                // backface-culls each to its own side). The explicit uvs
                // pair per plane (west/east and north/south, one row
                // [0,0,16,16] the other [16,0,0,16]) folds to ONE u
                // convention per plane — the mirrored row cancels the
                // mirrored face walk, the texture reads the same from
                // both sides; v stays the side-face law (1 - y).
                // THE WHOLE AGE LADDER the path keeps — the per-kind
                // FACE_TILES row the stage's own wheat_stageN tile
                // speaks (the growth walk's setBlock the re-mesh
                // answers, the stage's pixels the ladder's art carries)
                if crate::world::is_wheat(id)
                    || crate::world::is_flower(id)
                    || crate::world::is_tuft(id)
                    || crate::world::is_double_plant(id)
                    // THE WEB the same path keeps (the dev-22 round —
                    // the block/cobweb parent block/cross the two
                    // diagonal planes the flowers' own bake carries,
                    // the tint the pre-colored master never rides,
                    // the offset the NONE row the web never speaks)
                    || crate::world::is_web(id)
                    // THE SAPLING the same path keeps (the autopush-608
                    // round — the block/bamboo_sapling parent
                    // block/tinted_cross the flowers' own bake carries,
                    // the stage0 master the PRE-COLORED pixels speak,
                    // the XZ offset the offsetType(XZ) pc 23123 row
                    // wears)
                    || crate::world::is_bamboo_sapling(id)
                {
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        // the no-atlas boot's own defaults (the palette
                        // speaks no wheat row — the kind's pixels the
                        // generated atlas alone carries)
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else if crate::world::is_tuft(id) || crate::world::is_double_plant(id) {
                        // THE TUFT'S OWN TINT CARRIER (the autopush-412
                        // round — the same shape the flat path and the
                        // cube path speak): the synced biome's grass
                        // color when it differs from the baked plains
                        // sample (the no-op rule — the offline world's
                        // no-biome lane and the plains==baked verdict
                        // both ride the identity), multiplied into the
                        // vertex colors. One color per BLOCK (the
                        // audited BiomeColors.getAverageGrassColor
                        // walk — the block pos the source samples), the
                        // face-2 row the whole-cell verdict reads.
                        // THE DOUBLE PLANT the same carrier keeps (the
                        // autopush-413 round — the doubleTallGrass()
                        // source the same biome color answers, the
                        // UPPER half's pos.below() read the face_tint
                        // query's own below-shift folds)
                        world
                            .face_tint(id, 2, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3])
                    } else {
                        [1.0; 3] // the tile's own pixels speak (no tintindex — pre-colored)
                    };
                    // The attribute-system fold (the shared per-mesh
                    // uniforms both existing paths speak — see the flat
                    // path's own walk for the javap-proven arithmetic).
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // THE CELL'S OWN LIGHT — the withoutAO walk's
                    // getLightColor(state, pos): both layers sampled AT
                    // the crop cell, all four corners the same sample
                    // (no smoothing, no AO diagonals). The grid absent
                    // (the procedural lane) keeps the legacy unshaded
                    // look — the same None convention both paths speak.
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // The audited model rows: (axis, fixed_pos/16,
                    // u-flipped) — x=4 speaks u=z, x=12 speaks u=1-z,
                    // z=4 speaks u=1-x, z=12 speaks u=x. Each plane's
                    // faces: the pair of cube-face directions its normal
                    // axis carries (x-planes: west+east; z-planes:
                    // north+south), the y extent the -1/16 shift keeps.
                    // THE FLOWER PATH'S OWN CROSS (the autopush-411 round
                    // — the block/cross bake the 45° rotY + rescale fold
                    // speaks): the two diagonal planes the element's own
                    // from/to inset keeps — (0.8, ·, 0.8) -> (15.2, ·,
                    // 15.2) and its mirror — each emitting BOTH windings
                    // (the raster culls one per view side, the vanilla's
                    // own two faces per plane draw). `shade: false` —
                    // no FACE_SHADE term, the cell light alone
                    // multiplies; the uv the full sprite rides (the
                    // mirrored side cancels the reversed walk — the
                    // texture reads the same from both sides). THE TUFT
                    // FAMILY the same bake keeps (the autopush-412
                    // round — the tinted_cross parent the flowers' own
                    // cross geometry repeats, the tint the base carrier
                    // rides), the fold the baked plains look keeps
                    // no-op.
                    if crate::world::is_flower(id)
                        || crate::world::is_tuft(id)
                        || crate::world::is_double_plant(id)
                        // THE WEB the same path keeps (the dev-22 round
                        // — the cross geometry the flowers' own bake
                        // repeats, the offset the plant walk's NONE row
                        // zeroes — the rigid diagonal planes the vanilla
                        // model renders)
                        || crate::world::is_web(id)
                        // THE SAPLING the same path keeps (the
                        // autopush-608 round — the tinted_cross parent
                        // the cross geometry the flowers' own bake
                        // repeats, the XZ offset the sapling's own row
                        // wears)
                        || crate::world::is_bamboo_sapling(id)
                    {
                        // THE PLANT'S OWN RENDER OFFSET (the dev-27 round
                        // — the javap 26.2 law the world.rs walk keeps):
                        // the OffsetFunction's Vec3 the tesselate walk's
                        // FFF triple carries, the model plane the offset
                        // + inset fold rides (the innermost addition —
                        // the section origin adds later, the vanilla's
                        // own block-local bake keeps). The Y-ZERO seed
                        // the pair's two cells shares — the rigid pair
                        // the tall grass wears.
                        let (dx, dy, dz) = crate::world::plant_offset(id, wx, wy, wz);
                        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
                        let a = 0.8 / 16.0;
                        let b = 15.2 / 16.0;
                        let diagonals: [[(f32, f32); 2]; 2] = [
                            [(a, a), (b, b)], // the first element's bake
                            [(a, b), (b, a)], // the second element's mirror
                        ];
                        for &[(px0, pz0), (px1, pz1)] in &diagonals {
                            for &rev in &[false, true] {
                                let mut pts = [[0f32; 3]; 4];
                                let mut colors = [[0f32; 3]; 4];
                                let mut uvs = [[0f32; 2]; 4];
                                let mut corners = [
                                    (px0, 0.0f32, pz0, 0.0f32),
                                    (px1, 0.0f32, pz1, 1.0f32),
                                    (px1, 1.0f32, pz1, 1.0f32),
                                    (px0, 1.0f32, pz0, 0.0f32),
                                ];
                                if rev {
                                    corners.reverse();
                                }
                                for (i, &(fx, fy, fz, u)) in corners.iter().enumerate() {
                                    pts[i] = [
                                        wx as f32 + (fx + dx),
                                        wy as f32 + (fy + dy),
                                        wz as f32 + (fz + dz),
                                    ];
                                    uvs[i] = [u, 1.0 - fy];
                                    let lm = lightmap_combine(
                                        cell_sky,
                                        cell_block,
                                        sky_factor,
                                        sky_tint,
                                        ambient,
                                        block_tint,
                                        night_vision,
                                        world.night_vision_factor,
                                        world.darkness_scale,
                                    );
                                    colors[i] = [
                                        base[0] * lm[0],
                                        base[1] * lm[1],
                                        base[2] * lm[2],
                                    ];
                                }
                                out.push(Quad {
                                    pts,
                                    colors,
                                    uvs,
                                    tile,
                                    translucent: false,
                            per_texel_alpha: false,
                                });
                            }
                        }
                        continue;
                    }
                    let planes: [(u8, f32, bool); 4] = [
                        (0, 4.0 / 16.0, false),
                        (0, 12.0 / 16.0, true),
                        (2, 4.0 / 16.0, true),
                        (2, 12.0 / 16.0, false),
                    ];
                    for &(axis, fixed, flip) in &planes {
                        let (fa, fb) = if axis == 0 { (1usize, 0usize) } else { (5usize, 4usize) };
                        for &face in &[fa, fb] {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                                let (fx, fy, fz) = if axis == 0 {
                                    (fixed, c[1] - 1.0 / 16.0, c[2])
                                } else {
                                    (c[0], c[1] - 1.0 / 16.0, fixed)
                                };
                                pts[i] = [wx as f32 + fx, wy as f32 + fy, wz as f32 + fz];
                                let u_raw = if axis == 0 { c[2] } else { c[0] };
                                uvs[i] = [
                                    if flip { 1.0 - u_raw } else { u_raw },
                                    1.0 - c[1],
                                ];
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                // `shade: false` — no FACE_SHADE term,
                                // the lightmap alone multiplies
                                colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE TORCH'S OWN ELEMENT LANE (the dev-32 round — javap
                // the 26.2 jar: blockstates/torch.json's "" variant ->
                // block/torch -> block/template_torch, the ONE element box
                // from [7,0,7] to [9,10,9] the generated elements table
                // banks, `shade: false` faces under model-level
                // `ambientocclusion: false` — the tesselateWithoutAO shape
                // the crop family wears: the CELL's own light every corner
                // samples, no AO diagonals, no FACE_SHADE term). The quads
                // the registry's derive walk bakes (the same chain the TNT
                // render speaks — the faceinfo corners, the vanilla
                // winding recalculation), positioned at the cell; the
                // down face's `cullface: down` the below cell's opaque
                // verdict culls (the same isSolidRender law the cube
                // path's draw read speaks); the UVs the model's texel-space
                // rows the derive walk keeps verbatim (down [7,13,9,15],
                // up [7,6,9,8], the four sides [7,6,9,16]) — the /16 fold
                // the tile's 0..1 space wears, the sub-sprite windows the
                // stick's sides and the flame tip sample.
                if crate::world::is_torch(id) || crate::world::is_redstone_torch(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // `shade: false` + the no-atlas boot's own default —
                    // the tile's own pixels speak, the flat palette the
                    // missing row keeps (the crop path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface law:
                    // the neighbor's face the solid render answers)
                    let below = world.get(wx, wy - 1, wz);
                    // THE ELEMENT BAKES' OWN CHOICE (the autopush-666
                    // round — the redstone torch's own pair the
                    // per-kind derive wears: the LIT kind the
                    // template_redstone_torch chain the stick + the SIX
                    // glow planes the 7-element bake answers, the OFF
                    // kind the template_torch_unlit parent the stick
                    // box alone keeps; the regular torch the ONE
                    // element chain the lane's own bake answers; the
                    // autopush-667 round the WALL pair the same
                    // per-kind derive wears — the LIT kind the
                    // template_redstone_torch_wall chain the tilted
                    // stick box [-1,3.5,7]->[1,13.5,9] the -22.5° rotZ
                    // the element-rotation lane bakes + the SIX glow
                    // planes the 7-element chain keeps, the OFF kind
                    // the template_torch_wall_unlit parent the stick
                    // box ALONE answers)
                    let element_quads = if crate::world::is_torch(id) {
                        torch_model_quads()
                    } else if crate::world::is_redstone_wall_torch(id) {
                        redstone_wall_torch_model_quads(crate::world::redstone_torch_lit(id))
                    } else {
                        redstone_torch_model_quads(crate::world::redstone_torch_lit(id))
                    };
                    // THE WALL TORCH'S OWN FACING ROTATION (the
                    // autopush-667 round — the blockstates'
                    // redstone_wall_torch.json the y-table the 8-variant
                    // map banks: facing=east the base model the NO-y row
                    // answers, north 270 / south 90 / west 180 the
                    // rotated rows speak; the facing the variant byte
                    // decodes, the theta the cell-center rotation the
                    // leaf_litter lane's own formula wears [rx = 0.5 +
                    // (x-0.5)·cos - (z-0.5)·sin, rz = 0.5 + (x-0.5)·sin
                    // + (z-0.5)·cos] — the vanilla model y-rotation the
                    // same walk answers, the stick's embedded half the
                    // attached block's own body hides [the wall block's
                    // opaque faces the depth test answers], the uv the
                    // rotation never touches — the texture turns WITH
                    // the element the glued law keeps). The standing
                    // kinds the identity pair the no-variant lane
                    // keeps.
                    let (wall_sn, wall_cs) = if crate::world::is_redstone_wall_torch(id) {
                        let facing =
                            crate::world::wall_torch_facing_of(world.variant(wx, wy, wz));
                        let y_deg = match facing {
                            crate::world::WALL_TORCH_FACING_EAST => 0.0f32,
                            crate::world::WALL_TORCH_FACING_SOUTH => 90.0,
                            crate::world::WALL_TORCH_FACING_WEST => 180.0,
                            _ => 270.0,
                        };
                        y_deg.to_radians().sin_cos()
                    } else {
                        (0.0f32, 1.0f32)
                    };
                    for q in element_quads {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the stick's bottom the solid ground never shows
                        }
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE WALL ROTATION'S OWN FOLD (the facing
                            // theta the cell-center walk wears — the
                            // standing kinds the identity walk answers,
                            // the (sn, cs) pair the (0, 1) no-op keeps)
                            let rx = 0.5 + (v[0] - 0.5) * wall_cs - (v[2] - 0.5) * wall_sn;
                            let rz = 0.5 + (v[0] - 0.5) * wall_sn + (v[2] - 0.5) * wall_cs;
                            pts[i] = [
                                wx as f32 + rx,
                                wy as f32 + v[1],
                                wz as f32 + rz,
                            ];
                            // the texel-space uv the /16 fold normalizes
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            // `shade: false` — no FACE_SHADE term, the
                            // lightmap alone multiplies
                            colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE COMPARATOR'S OWN ELEMENT LANE (the autopush-124
                // round — the four per-state model bakes the jar's own
                // blockstate rows wear: block/comparator{,_on,
                // _subtract,_on_subtract} the (subtract, powered) pair
                // the variant byte picks. THE PER-QUAD TILE VERDICT the
                // merged texture map's own names answer: the #top ref
                // the TILE_COMPARATOR(_ON) swap banks, the #slab ref
                // the smooth_stone tile keeps, the #unlit/#lit refs the
                // redstone torch kinds' own tiles wear, the particle
                // ref the quads never carry. THE FACING'S OWN Y-TABLE
                // the blockstates/comparator.json rows speak — facing
                // south the y=0 row answers, west y=90, north y=180,
                // east y=270 (the turns the 90° steps the cell-center
                // rotation fold wears — the wall torch lane's own
                // formula the same walk serves). THE DOWN CULLFACE the
                // slab's ground verdict keeps, the side cullfaces the
                // rotated model the lane never consults [the touching
                // slabs' opposite orientations the depth test settles].
                // `shade: true` the models carry — the FACE_SHADE term
                // the quad's own direction reads, the lightmap the
                // per-corner fold answers).
                if crate::world::is_comparator(id) || crate::world::is_repeater(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the candle path's own verdict)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface law:
                    // the neighbor's face the solid render answers)
                    let below = world.get(wx, wy - 1, wz);
                    let variant = world.variant(wx, wy, wz);
                    // THE SHARED LANE'S OWN DECODE (the kind's own
                    // variant law the pair speaks — the comparator the
                    // (subtract, powered) pair the four bakes pick, the
                    // repeater the (powered, delay, locked) triple the
                    // sixteen chains answer)
                    let (quads, facing) = if crate::world::is_comparator(id) {
                        let subtract = crate::world::comparator_subtract_of(variant);
                        let powered = crate::world::comparator_powered_of(variant);
                        (
                            comparator_model_quads(subtract, powered),
                            crate::world::comparator_facing_of(variant),
                        )
                    } else {
                        let powered = crate::world::repeater_powered_of(variant);
                        let delay = crate::world::repeater_delay_of(variant);
                        let locked = crate::world::repeater_locked_of(variant);
                        (
                            repeater_model_quads(powered, delay, locked),
                            crate::world::repeater_facing_of(variant),
                        )
                    };
                    let turns = match facing {
                        1 => 3.0f32, // east the y=270 row
                        2 => 0.0,    // south the y=0 row
                        3 => 1.0,    // west the y=90 row
                        _ => 2.0,    // north the y=180 row
                    };
                    let (sn, cs) = (turns * std::f32::consts::FRAC_PI_2).sin_cos();
                    for q in quads {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the slab's bottom the solid ground never shows
                        }
                        // THE PER-QUAD TILE'S OWN VERDICT (the resolved
                        // texture name the derive walk carries — the
                        // #top swap the powered bit's own seat banks)
                        let tile = match q.texture {
                            "block/comparator" => atlas::TILE_COMPARATOR,
                            "block/comparator_on" => atlas::TILE_COMPARATOR_ON,
                            "block/repeater" => atlas::TILE_REPEATER,
                            "block/repeater_on" => atlas::TILE_REPEATER_ON,
                            "block/smooth_stone" => {
                                atlas::FACE_TILES[crate::world::SMOOTH_STONE as usize][2]
                            }
                            "block/redstone_torch" => {
                                atlas::FACE_TILES[crate::world::REDSTONE_TORCH as usize][2]
                            }
                            "block/redstone_torch_off" => {
                                atlas::FACE_TILES[crate::world::REDSTONE_TORCH_OFF as usize][2]
                            }
                            // THE LOCK BAR'S OWN TILE (the autopush-126
                            // round — the #lock ref the jar's own
                            // block/bedrock texture the freeze bar wears)
                            "block/bedrock" => atlas::FACE_TILES[crate::world::BEDROCK as usize][2],
                            _ => tile,
                        };
                        // THE SHADE'S OWN SEAT (the model's shade:true
                        // the face direction's own multiplier reads —
                        // the mesher's face order [east, west, top,
                        // bottom, south, north] the Cull enum order
                        // [Down, Up, North, South, West, East] the map
                        // wears)
                        let shade = match q.direction {
                            mc_registry::element_quads::Direction::East => FACE_SHADE[0],
                            mc_registry::element_quads::Direction::West => FACE_SHADE[1],
                            mc_registry::element_quads::Direction::Up => FACE_SHADE[2],
                            mc_registry::element_quads::Direction::Down => FACE_SHADE[3],
                            mc_registry::element_quads::Direction::South => FACE_SHADE[4],
                            mc_registry::element_quads::Direction::North => FACE_SHADE[5],
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE Y-ROTATION'S OWN FOLD (the facing turns
                            // the cell-center walk wears — the vanilla
                            // blockstate y the clockwise order speaks)
                            let rx = 0.5 + (v[0] - 0.5) * cs - (v[2] - 0.5) * sn;
                            let rz = 0.5 + (v[0] - 0.5) * sn + (v[2] - 0.5) * cs;
                            pts[i] = [wx as f32 + rx, wy as f32 + v[1], wz as f32 + rz];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * shade * lm[0],
                                base[1] * shade * lm[1],
                                base[2] * shade * lm[2],
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE OBSERVER'S OWN ELEMENT LANE (the autopush-127
                // round — the ONE full-cube bake the jar's own
                // blockstates/observer.json rows wear: block/observer
                // the twelve rows' model, the (powered) bit the
                // per-quad verdict's #bottom seat swaps to the _on
                // back tile [the observer_on parent-override the bake
                // never derives — the parent's ONE texture swap the
                // verdict reads]). THE PER-QUAD TILE VERDICT the
                // merged texture map's own names answer: the #top ref
                // the observer_top tile banks [the down/up faces the
                // model's own rows speak], the #side ref the
                // observer_side keeps, the #front ref the eye tile the
                // FACING face wears, the #bottom ref the back(_on)
                // tile the output face swaps. THE FACING'S OWN Y-TABLE
                // the blockstates/observer.json rows speak — facing
                // north the y=0 row answers [the model's #front the
                // NORTH face at rest the comparator's south-anchored
                // table the mirror wears], east y=90, south y=180,
                // west y=270. THE X-ROWS the vertical pair speaks —
                // facing down the x=90 row [the model's north face the
                // DOWN face the -90° X-fold answers], facing up the
                // x=270 [the +90° fold the eye up wears], the folds'
                // own order the vertical rows the x-fold alone answers
                // [the blockstate rows never mix the pair]. THE
                // ALL-SIX CULLFACES the full-cube law keeps — every
                // face the neighbor's solid verdict culls [the model's
                // own cullface rows the jar speaks, the comparator's
                // plate the down-only law the cube lane extends]. The
                // `shade: true` the model carries — the FACE_SHADE
                // term the quad's own direction reads, the lightmap
                // the per-corner fold answers.
                if crate::world::is_observer(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the candle path's own verdict)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    let variant = world.variant(wx, wy, wz);
                    let powered = crate::world::observer_powered_of(variant);
                    let facing = crate::world::observer_facing_of(variant);
                    let quads = observer_model_quads();
                    // THE Y-TABLE THE HORIZONTAL ROWS SPEAK (the
                    // blockstate's own turns the quarter folds keep) —
                    // THE X-FOLD THE VERTICAL PAIR (the down the -90°
                    // the x=90 row answers, the up the +90° the x=270
                    // row wears)
                    let (turns, x_fold): (f32, Option<f32>) = match facing {
                        0 => (0.0, Some(-std::f32::consts::FRAC_PI_2)), // down x=90
                        1 => (0.0, Some(std::f32::consts::FRAC_PI_2)),  // up x=270
                        2 => (0.0, None),                               // north y=0
                        3 => (2.0, None),                               // south y=180
                        4 => (3.0, None),                               // west y=270
                        _ => (1.0, None),                               // east y=90
                    };
                    let (sn, cs) = (turns * std::f32::consts::FRAC_PI_2).sin_cos();
                    let (xs, xc) = match x_fold {
                        Some(a) => a.sin_cos(),
                        None => (0.0f32, 1.0f32),
                    };
                    for q in quads {
                        // THE ALL-SIX CULL'S OWN READ (the full-cube
                        // law the neighbor's face the solid render
                        // answers — the cullface the quad's own
                        // direction names)
                        if let Some(cf) = q.cullface {
                            let (ox, oy, oz) = match cf {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + ox, wy + oy, wz + oz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TILE'S OWN VERDICT (the resolved
                        // texture name the derive walk carries — the
                        // #bottom swap the powered bit's own seat banks)
                        let tile = match q.texture {
                            "block/observer_top" => atlas::TILE_OBSERVER_TOP,
                            "block/observer_side" => atlas::TILE_OBSERVER_SIDE,
                            "block/observer_front" => atlas::TILE_OBSERVER_FRONT,
                            "block/observer_back" => {
                                if powered {
                                    atlas::TILE_OBSERVER_BACK_ON
                                } else {
                                    atlas::TILE_OBSERVER_BACK
                                }
                            }
                            _ => tile,
                        };
                        // THE SHADE'S OWN SEAT (the model's shade:true
                        // the face direction's own multiplier reads —
                        // the mesher's face order the Cull enum order
                        // the map wears)
                        let shade = match q.direction {
                            mc_registry::element_quads::Direction::East => FACE_SHADE[0],
                            mc_registry::element_quads::Direction::West => FACE_SHADE[1],
                            mc_registry::element_quads::Direction::Up => FACE_SHADE[2],
                            mc_registry::element_quads::Direction::Down => FACE_SHADE[3],
                            mc_registry::element_quads::Direction::South => FACE_SHADE[4],
                            mc_registry::element_quads::Direction::North => FACE_SHADE[5],
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE FOLDS' OWN ORDER (the vertical rows
                            // the x-fold alone answers — the cell-center
                            // X-rotation the (y, z) pair wears — then
                            // the horizontal y-rot the facing turns
                            // speak, the vanilla x-then-y compose the
                            // blockstate rows keep)
                            let mut py = v[1];
                            let mut pz = v[2];
                            if x_fold.is_some() {
                                let dy = v[1] - 0.5;
                                let dz = v[2] - 0.5;
                                py = 0.5 + dy * xc - dz * xs;
                                pz = 0.5 + dy * xs + dz * xc;
                            }
                            let rx = 0.5 + (v[0] - 0.5) * cs - (pz - 0.5) * sn;
                            let rz = 0.5 + (v[0] - 0.5) * sn + (pz - 0.5) * cs;
                            pts[i] = [wx as f32 + rx, wy as f32 + py, wz as f32 + rz];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * shade * lm[0],
                                base[1] * shade * lm[1],
                                base[2] * shade * lm[2],
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE PISTON LANE'S OWN ELEMENT WALK (the autopush-128
                // round — the THREE kinds the family's own bake picks:
                // the retracted base the template_piston full-cube
                // chain [the sticky top the kind's own model answers],
                // the extended base the piston_extended body the
                // inside face the piston_inner tile wears, the head
                // the plate + shaft chain the overhang the base
                // cell's own border serves). THE JAR'S OWN Y-TABLE
                // the blockstates rows speak — the down x=90 / up
                // x=270 / east y=90 / south y=180 / west y=270 / north
                // y=0 rows the observer's own table the mirror keeps,
                // the folds' own order the vertical rows the x-fold
                // alone answers. THE MODEL'S OWN CULLFACES the rows
                // carry — the full cube's six the template_piston
                // speaks, the extended body's five the open front
                // keeps, the head plate's five + the shaft's none the
                // overhang serves. The `shade: true` the model
                // carries — the FACE_SHADE term the per-quad direction
                // reads, the lightmap the per-corner fold answers.
                if crate::world::is_piston_kind(id)
                    || crate::world::is_piston_head(id)
                    || crate::world::is_moving_piston(id)
                {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the candle path's own verdict)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    let variant = world.variant(wx, wy, wz);
                    // THE MOVING ARM'S OWN STASH READ (the pushed
                    // state the mid-state cell wears — the slide's
                    // own offset the variant byte's quarters AND
                    // extending bit carry: the extend the source
                    // BEHIND the destination the -delta arm keeps,
                    // the retract pull the source AHEAD the +delta
                    // arm wears; the full-cell offset the quarters'
                    // 0 arm speaks, the half the 2 arm answers)
                    let moving = if crate::world::is_moving_piston(id) {
                        world.moving_state_of(wx, wy, wz)
                    } else {
                        None
                    };
                    // THE THREE BAKES' OWN PICK (the kind + the state
                    // bits the decode serves — the base the retracted
                    // pair the kind's own model wears, the extended
                    // ONE chain both kinds speak, the head the type
                    // bit the pair answers; the MOVING cell the
                    // pushed state's own bake picks — the family
                    // members the sibling bakes answer, the plain
                    // kinds the generic cube wears)
                    let (quads, facing, offset) = if let Some((pkind, pvariant, pflags)) = moving {
                        let quarters = crate::world::moving_progress_quarters_of(variant).min(4);
                        // THE SLIDE'S OWN CURVE (the flag the shape
                        // speaks — the land rows the shrinking
                        // fraction the source-to-dest slide wears,
                        // the HEAD_RETRACT row the GROWING fraction
                        // the sink the base the head renders)
                        let (sign, frac) = if pflags == crate::world::MOVING_FLAG_HEAD_RETRACT {
                            (-1.0f32, quarters as f32 / 4.0)
                        } else if crate::world::moving_extending_of(variant) {
                            (-1.0f32, 1.0 - quarters as f32 / 4.0)
                        } else {
                            (1.0f32, 1.0 - quarters as f32 / 4.0)
                        };
                        let (sdx, sdy, sdz) = crate::world::piston_facing_delta6(
                            crate::world::moving_facing_of(variant),
                        );
                        let offset = [
                            sign * frac * sdx as f32,
                            sign * frac * sdy as f32,
                            sign * frac * sdz as f32,
                        ];
                        if crate::world::is_piston_head(pkind) {
                            (
                                piston_head_model_quads(crate::world::piston_head_sticky_of(
                                    pvariant,
                                )),
                                crate::world::piston_head_facing_of(pvariant),
                                offset,
                            )
                        } else if crate::world::is_piston_kind(pkind) {
                            (
                                piston_model_quads(
                                    pkind == crate::world::STICKY_PISTON,
                                    crate::world::piston_extended_of(pvariant),
                                ),
                                crate::world::piston_facing_of(pvariant),
                                offset,
                            )
                        } else {
                            // the IDENTITY table the generic cube
                            // wears (the north row the no-fold keep —
                            // the family's own folds the authored
                            // bakes alone answer)
                            (moving_cube_model_quads(), 2, offset)
                        }
                    } else if crate::world::is_moving_piston(id) {
                        // THE STASH-LESS MID-STATE (the stale cell the
                        // honest absence keeps — the finalize's take
                        // the row cleared): nothing renders.
                        (&[][..], 0, [0.0f32; 3])
                    } else if crate::world::is_piston_head(id) {
                        (
                            piston_head_model_quads(crate::world::piston_head_sticky_of(variant)),
                            crate::world::piston_head_facing_of(variant),
                            [0.0f32; 3],
                        )
                    } else {
                        (
                            piston_model_quads(
                                id == crate::world::STICKY_PISTON,
                                crate::world::piston_extended_of(variant),
                            ),
                            crate::world::piston_facing_of(variant),
                            [0.0f32; 3],
                        )
                    };
                    // THE GENERIC ROW'S OWN SEAT (the pushed kind's
                    // own FACE_TILES row the per-direction verdict
                    // reads — the piston-family pushes the name
                    // verdict keeps, the plain kinds the row serves)
                    let pushed_row: Option<[u16; 6]> = match moving {
                        Some((pkind, _, _))
                            if !crate::world::is_piston_kind(pkind)
                                && !crate::world::is_piston_head(pkind) =>
                        {
                            if (pkind as usize) < atlas::N_KINDS {
                                Some(atlas::FACE_TILES[pkind as usize])
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    // THE Y-TABLE THE HORIZONTAL ROWS SPEAK — THE
                    // X-FOLD THE VERTICAL PAIR (the down the -90° the
                    // x=90 row answers, the up the +90° the x=270 row
                    // wears — the observer's own table the mirror
                    // keeps)
                    let (turns, x_fold): (f32, Option<f32>) = match facing {
                        0 => (0.0, Some(-std::f32::consts::FRAC_PI_2)), // down x=90
                        1 => (0.0, Some(std::f32::consts::FRAC_PI_2)),  // up x=270
                        2 => (0.0, None),                               // north y=0
                        3 => (2.0, None),                               // south y=180
                        4 => (3.0, None),                               // west y=270
                        _ => (1.0, None),                               // east y=90
                    };
                    let (sn, cs) = (turns * std::f32::consts::FRAC_PI_2).sin_cos();
                    let (xs, xc) = match x_fold {
                        Some(a) => a.sin_cos(),
                        None => (0.0f32, 1.0f32),
                    };
                    for q in quads {
                        // THE MODEL'S OWN CULL READ (the cullface the
                        // quad's own direction names — the neighbor's
                        // solid verdict the face culls)
                        if let Some(cf) = q.cullface {
                            let (ox, oy, oz) = match cf {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + ox, wy + oy, wz + oz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TILE'S OWN VERDICT (the resolved
                        // texture name the derive walk carries — the
                        // five tiles the family wears; the MOVING
                        // generic arm the pushed kind's own row the
                        // per-direction pick serves)
                        let tile = if let Some(row) = pushed_row {
                            use mc_registry::element_quads::Direction as Dir;
                            match q.direction {
                                Dir::East => row[0],
                                Dir::West => row[1],
                                Dir::Up => row[2],
                                Dir::Down => row[3],
                                Dir::South => row[4],
                                Dir::North => row[5],
                            }
                        } else {
                            match q.texture {
                                "block/piston_top" => atlas::TILE_PISTON_TOP,
                                "block/piston_top_sticky" => atlas::TILE_PISTON_TOP_STICKY,
                                "block/piston_side" => atlas::TILE_PISTON_SIDE,
                                "block/piston_bottom" => atlas::TILE_PISTON_BOTTOM,
                                "block/piston_inner" => atlas::TILE_PISTON_INNER,
                                _ => tile,
                            }
                        };
                        // THE SHADE'S OWN SEAT (the model's shade:true
                        // the face direction's own multiplier reads)
                        let shade = match q.direction {
                            mc_registry::element_quads::Direction::East => FACE_SHADE[0],
                            mc_registry::element_quads::Direction::West => FACE_SHADE[1],
                            mc_registry::element_quads::Direction::Up => FACE_SHADE[2],
                            mc_registry::element_quads::Direction::Down => FACE_SHADE[3],
                            mc_registry::element_quads::Direction::South => FACE_SHADE[4],
                            mc_registry::element_quads::Direction::North => FACE_SHADE[5],
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE FOLDS' OWN ORDER (the vertical rows
                            // the x-fold alone answers — then the
                            // horizontal y-rot the facing turns speak)
                            let mut py = v[1];
                            let mut pz = v[2];
                            if x_fold.is_some() {
                                let dy = v[1] - 0.5;
                                let dz = v[2] - 0.5;
                                py = 0.5 + dy * xc - dz * xs;
                                pz = 0.5 + dy * xs + dz * xc;
                            }
                            let rx = 0.5 + (v[0] - 0.5) * cs - (pz - 0.5) * sn;
                            let rz = 0.5 + (v[0] - 0.5) * sn + (pz - 0.5) * cs;
                            // THE SLIDE'S OWN OFFSET (the mid-state's
                            // quarters the variant byte carries — the
                            // world-space step the pts assembly adds,
                            // the settled cells the zero offset keeps)
                            pts[i] = [
                                wx as f32 + rx + offset[0],
                                wy as f32 + py + offset[1],
                                wz as f32 + rz + offset[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * shade * lm[0],
                                base[1] * shade * lm[1],
                                base[2] * shade * lm[2],
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE CANDLE PAIR'S OWN ELEMENT LANE (the autopush-522
                // round — the canLight consult's own kinds finally
                // RENDERS). blockstates/candle.json's one-candle rows:
                // "candles=1,lit=false" -> block/candle_one_candle and
                // "candles=1,lit=true" -> block/candle_one_candle_lit,
                // BOTH parents block/template_candle — the stick box
                // [7,0,7]->[9,6,9] (six faces, the `#all` ref the kind's
                // own tile answers) PLUS the two ±45° rotY wick planes
                // the generated ELEMENTS table carries (the derive walk
                // bakes the rotations into the vertices the fire lane's
                // rotated planes speak). The elements carry `shade: true`
                // — the vanilla tesselation's FACE_SHADE term the cube
                // path's own table wears (up 1.0, down 0.5, the four
                // horizontals 0.8/0.6), the term the torch/fire lanes'
                // `shade: false` elements never pay. The cell's own
                // light every corner samples (no AO diagonals — the
                // same withoutAO fold the element lanes keep), the down
                // face's `cullface: down` the below cell's opaque
                // verdict culls (the torch lane's own verdict).
                if crate::world::is_candle(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the torch path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface law:
                    // the neighbor's face the solid render answers)
                    let below = world.get(wx, wy - 1, wz);
                    for q in candle_model_quads(id == crate::world::CANDLE_LIT) {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the stick's bottom the solid ground never shows
                        }
                        // THE SHADE TERM'S OWN READ — the element's
                        // `shade: true` the vanilla tesselation's
                        // direction table answers (the FACE_SHADE
                        // constants the cube path wears; the quad's own
                        // direction the derive walk's facing verdict
                        // carries)
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            // the texel-space uv the /16 fold normalizes
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE LEVER'S OWN ELEMENT LANE (the dev-45 round — the
                // toggle's own block finally renders): the two kinds
                // the OFF/ON bit the model pair answers, the facing
                // the variant byte wears (the placement's raw-look
                // stamp the wire lane banks — the blockstate's own
                // FLOOR rows north y=0 / east y=90 / south y=180 /
                // west y=270 the step fold reads, the SAME y-rot
                // convention the door lane's steps walk serves), the
                // per-quad TEXTURE VERDICT the two-tile model demands
                // (the #lever refs the kind's own row tile — the
                // FACE_TILES row the lever arm banks, the #base refs
                // the cobblestone sidecar the TILE_COBBLESTONE const
                // answers). The DOWN CULLFACE the base plate carries
                // the below cell the cull verdict reads (the rotated
                // direction the same turn the vertices take — the
                // DOWN the y-rot never moves), the non-occluding
                // verdict the neighbors' own faces the stick never
                // culls. The cell's own light every corner samples
                // (no AO diagonals — the element lanes' own fold), the
                // shade term the rotated direction's table answers
                // (the shade:true elements the FACE_SHADE walk pays),
                // the stick's own ±45° tilt the derive walk's rotated
                // vertices already wear (the Rodrigues fold the
                // element-rotation lane speaks).
                if crate::world::is_lever(id) || crate::world::is_button(id) || crate::world::is_pressure_plate(id) {
                    // (the autopush-660 round the BUTTON the same lane
                    // wears — the FaceAttachedHorizontalDirectionalBlock
                    // parent the two families share, the element lane
                    // the same decode + the same x/y turns the button's
                    // ONE-plate model rides: derive quads the pair
                    // answers, the tile verdict the same fold keeps)
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the lever quads'
                    // no-atlas fallback (the #lever ref the palette row
                    // banks)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE VARIANT'S OWN TWO-FIELD DECODE (the dev-45
                    // round the facing byte banked, this round the face
                    // seat the HIGH nibble banks — the placement's
                    // two-field stamp the decode reads, the plain 1..4
                    // rows the floor face the nibble-0 wear): the
                    // blockstate's own rows the step laws speak — the
                    // FLOOR rows north y=0 / east y=90 / south y=180 /
                    // west y=270 the index steps keep, the WALL rows
                    // the x=90 turn + the SAME y steps wear, the
                    // CEILING rows the x=180 turn + the y-remap the
                    // south y=0 / west y=90 / north y=180 / east y=270
                    // rows speak (the (idx+2)%4 fold the +2 offset's
                    // own law banks — the JOML rotationXYZ the x
                    // arm the y arm the order keeps). The PLATE the
                    // decode never reads (the floor-only plate the
                    // variant byte never consumes — the no-turn identity
                    // the sensor's own floor lane wears).
                    let (x_turns, steps) = if crate::world::is_pressure_plate(id) {
                        (0usize, 0usize)
                    } else {
                    let variant = world.variant(wx, wy, wz);
                    let lever_face = crate::world::lever_face_of(variant);
                    let facing_raw = crate::world::lever_facing_of(variant);
                    let idx = if facing_raw == 0 { 0usize } else { facing_raw as usize - 1 };
                    match lever_face {
                        crate::world::LEVER_FACE_WALL => (1usize, idx),
                        crate::world::LEVER_FACE_CEILING => (2usize, (idx + 2) % 4),
                        _ => (0usize, idx),
                    }
                    };
                    for q in if crate::world::is_button(id) {
                        button_model_quads(id, crate::world::button_powered(id))
                    } else if crate::world::is_pressure_plate(id) {
                        plate_model_quads(id, crate::world::plate_powered(id))
                    } else {
                        lever_model_quads(crate::world::lever_powered(id))
                    } {
                        // THE ROTATED CULLFACE VERDICT — the six-
                        // direction law the candle lane wears, the
                        // direction the SAME turns the geometry takes
                        // (the x-arm first the wall/ceiling lanes bank,
                        // the down cullface the y-rot never moves — the
                        // x-turn the flush face's own verdict rotates)
                        if let Some(dir) = q.cullface {
                            let rotated =
                                rotate_direction_cw(rotate_direction_x(dir, x_turns), steps);
                            let (dx, dy, dz) = match rotated {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // lever's own refs the kind row and the
                        // sidecar const bank, the unknown ref the
                        // kind's own row keeps)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            // the lever's own #lever ref the kind row
                            // banks (the #base sidecar the cobblestone
                            // const answers); the button's #texture ref
                            // block/stone the row's own tile banks — the
                            // stone master the FACE_TILES row already
                            // banks (the autopush-660 round the same
                            // single-tile verdict the family wears)
                            "lever" | "stone" => fallback_tile,
                            "cobblestone" => atlas::TILE_COBBLESTONE,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the element's
                        // shade:true the ROTATED direction's table
                        // answers (the world-facing term the turn
                        // keeps), the never-shaded the 1.0 keeps
                        let rotated_dir =
                            rotate_direction_cw(rotate_direction_x(q.direction, x_turns), steps);
                        let face_shade = if q.shade {
                            match rotated_dir {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE X-TURN'S OWN ARM (the blockstate x
                            // values the JOML -90° fold serves — each
                            // turn the (y,z)->(z,1-y) map about the
                            // cell center answers: one turn the floor
                            // plate the wall's own flush reads, two
                            // turns the ceiling's own top keeps — the
                            // wall row's x=90 the single turn banks,
                            // the ceiling's x=180 the double wears)
                            let (mut vy, mut vz) = (v[1], v[2]);
                            for _ in 0..x_turns {
                                let ny = 0.5 + (vz - 0.5);
                                let nz = 1.0 - vy;
                                vy = ny;
                                vz = nz;
                            }
                            // THE STEPS' OWN TURN (the blockstate y
                            // values the CW walk folds: each step the
                            // (x,z)->(-z,x) map about the cell center
                            // serves — the north row the raw model
                            // keeps, the east row one step wears)
                            let (dx, dz) = (v[0] - 0.5, vz - 0.5);
                            let (rx, rz) = rotate_xz_cw(dx, dz, steps);
                            pts[i] = [
                                wx as f32 + 0.5 + rx,
                                wy as f32 + vy,
                                wz as f32 + 0.5 + rz,
                            ];
                            // the texel-space uv the /16 fold
                            // normalizes (the rotated rows the UVs
                            // never move — the door lane's own law the
                            // symmetrical masters keep)
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE CAMPFIRE FAMILY'S OWN ELEMENT LANE (the dev-6
                // round — the douse lane's own kinds finally RENDER).
                // blockstates/campfire.json's "facing=south,lit=false"
                // row -> block/campfire_off (the five-log geometry BOTH
                // unlit kinds share — the SOUL_CAMPFIRE kind the same
                // model the blockstate's own rows point, the unlit soul
                // the plain-log look the vanilla itself renders), the
                // "facing=south,lit=true" rows -> block/campfire /
                // block/soul_campfire (BOTH parents
                // block/template_campfire: the five-log boxes the lit
                // faces swap, PLUS the two ±45° rotY rescale fire
                // planes the derive walk bakes — the candle lane's own
                // rotated-plane fold speaks). The per-quad TEXTURE
                // VERDICT the multi-texture template demands (the
                // candle lane's single-tile fold the campfire never
                // wears — the merged texture map's #log/#lit_log/#fire
                // refs the resolved quad texture names carry): the log
                // faces the named tile consts read, the fire faces the
                // strip tables' FRAME0 bases — the draw-time remap the
                // campfire_fire/soul_campfire_fire frametime-2
                // animations swaps (the mesher never re-meshes for the
                // flicker). The cullfaces the five-log boxes carry
                // (north/south/east/west/down) the six-direction
                // neighbor read culls (the vanilla cullface law: the
                // neighbor's face the solid render answers — the
                // candle lane's down-only verdict the campfire's four
                // side rows generalize). The log boxes' `shade: true`
                // elements the FACE_SHADE table pays (up 1.0, down
                // 0.5, the horizontals 0.8/0.6), the fire planes'
                // `shade: false` the lightmap alone multiplies. The
                // cell's own light every corner samples (no AO
                // diagonals — the same withoutAO fold the element
                // lanes keep); the fire planes' y 1..17 span the cell
                // above pokes into, the vanilla's own lick-above the
                // mesh never clips.
                if crate::world::is_campfire(id) {
                    let model_id = match id {
                        crate::world::CAMPFIRE_LIT => "block/campfire",
                        crate::world::SOUL_CAMPFIRE_LIT => "block/soul_campfire",
                        _ => "block/campfire_off",
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the fire quads'
                    // no-atlas fallback (the fire ref the palette row
                    // banks — the resolved texture verdict the strip
                    // base duplicates)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in campfire_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the five-
                        // log boxes' side + down rows the neighbor read
                        // culls (the vanilla cullface law the candle
                        // lane's down arm generalizes)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // fire bases the draw-time remap animates, the
                        // unknown ref the kind's own row keeps)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "campfire_log" => atlas::TILE_CAMPFIRE_LOG,
                            "campfire_log_lit" => atlas::TILE_CAMPFIRE_LOG_LIT,
                            "campfire_fire" => atlas::CAMPFIRE_FIRE_FRAME0,
                            "soul_campfire_log_lit" => atlas::TILE_SOUL_CAMPFIRE_LOG_LIT,
                            "soul_campfire_fire" => atlas::SOUL_CAMPFIRE_FIRE_FRAME0,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the log boxes'
                        // shade:true elements the direction table
                        // answers, the fire planes the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE HONEY BLOCK'S OWN ELEMENT LANE (the autopush-599
                // round — javap the 26.2 jar: blockstates/honey_block.
                // json's "" variant -> block/honey_block, the
                // TWO-ELEMENT model the generated elements table banks:
                // the FULL outer cube [0,0,0]->[16,16,16] all six faces
                // the #down ref (honey_block_bottom) with cullface rows
                // + the 1..15 INSET cube the #up/#side/#down skins
                // wear, uv [1,1,15,15] the 14-texel crop the inset's
                // own texel-space rows keep). The textures' UNIFORM
                // PARTIAL ALPHA (the PNG alpha maps: top 191 / side 189
                // / bottom 129 — every texel the same value speaks) the
                // per-texel translucent arm composites (the vanilla
                // translucent layer's SRC_ALPHA / ONE_MINUS_SRC_ALPHA
                // fold — the DOUBLE-LAYER stack the combined
                // 1-(1-0.506)*(1-0.749) ~= 0.876 opacity the outside
                // view wears). The SUBMIT ORDER the depth-write law
                // speaks: the translucent pass the framebuffer
                // overdraw composites, the depth the pass writes — the
                // INSET quads the back-to-front order FIRST banks (the
                // derive walk's element order the outer cube first
                // keeps — the reversed vec the inset quads the outer
                // layer's own depth-write the closer plane never
                // erases), the camera-inside cell the backface cull the
                // walls never shows (the one-sided faces the vanilla
                // cull keeps — the neighbors' faces the non-occluding
                // verdict draws). The elements carry `shade: true` —
                // the FACE_SHADE term the cube path's own table wears
                // (up 1.0, down 0.5, the four horizontals 0.8/0.6), the
                // noOcclusion verdict the AO never casts (the
                // is_transparent row the walk keeps). The cullfaces the
                // outer cube's six rows carry the six-direction
                // neighbor read culls (the campfire lane's general
                // verdict — the honey-against-stone shared face the
                // cull speaks, the rim the neighbor's own face
                // answers).
                if id == crate::world::HONEY_BLOCK {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let base = [1.0f32; 3];
                    for q in honey_model_quads() {
                        // THE CULLFACE'S OWN READ — the neighbor cell the
                        // solid-render verdict samples (the campfire lane's
                        // general walk — the shared plane the opaque
                        // neighbor culls)
                        if let Some(cf) = q.cullface {
                            let (dx, dy, dz) = match cf {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the honey masters' tiles answer
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "honey_block_bottom" => atlas::TILE_HONEY_BLOCK_BOTTOM,
                            "honey_block_top" => atlas::TILE_HONEY_BLOCK_TOP,
                            "honey_block_side" => atlas::TILE_HONEY_BLOCK_SIDE,
                            _ => atlas::FACE_TILES[crate::world::HONEY_BLOCK as usize][2],
                        };
                        // THE SHADE TERM'S OWN READ — the model's
                        // shade:true elements the direction table answers
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            // the texel-space uv the /16 fold normalizes
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: true,
                            per_texel_alpha: true,
                        });
                    }
                    continue;
                }
                // THE PORTAL'S OWN ELEMENT LANE (the autopush-605 round
                // — the dimension gate finally RENDERS). The axis
                // verdict the NEIGHBOR reads derive (javap
                // NetherPortalBlock.animateTick pc 190-217 the same
                // short-circuit the sampler arm speaks: the west/east
                // portal pair the X-EXTENDING gate answers) — the
                // blockstates' own axis vocabulary the kind-only
                // palette never stores, the plane's own thin axis the
                // gate's neighbors answer: the x-extending gate the ns
                // model's thin-in-Z plane [0,0,6]->[16,16,10] wears,
                // the else arm the ew model's thin-in-X plane
                // [6,0,0]->[10,16,16] keeps. The TWO faces each
                // element carries the #portal ref (the merged texture
                // map block/nether_portal resolves — the strip's
                // FRAME0 base the draw-time remap animates, the
                // 32-frame sequential playlist the mcmeta's empty
                // animation wears). The texels' PARTIAL alpha (the
                // PNG's own alpha map the 120-distinct-value noise
                // speaks) the per-texel translucent arm composites
                // (the vanilla translucent layer's SRC_ALPHA /
                // ONE_MINUS_SRC_ALPHA fold — the honey lane's own
                // verdict). The elements carry `shade: true` — the
                // FACE_SHADE term the cube path's own table wears (up
                // 1.0, down 0.5, the four horizontals 0.8/0.6), the
                // cullfaces the elements never carry (the walk the
                // None arms skips — the neighbors' faces the
                // non-occluding verdict draws), the cell's own light
                // every corner samples (no AO diagonals — the same
                // withoutAO fold the element lanes keep).
                if id == crate::world::NETHER_PORTAL {
                    let x_extending = world.get(wx - 1, wy, wz) == crate::world::NETHER_PORTAL
                        || world.get(wx + 1, wy, wz) == crate::world::NETHER_PORTAL;
                    let model_id = if x_extending {
                        "block/nether_portal_ns"
                    } else {
                        "block/nether_portal_ew"
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the unknown ref's
                    // fallback (the portal ref the palette row banks —
                    // the resolved texture verdict the strip base
                    // duplicates)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in portal_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the
                        // elements' own None rows the walk skips (the
                        // campfire lane's general verdict the portal's
                        // cullface-less faces never consults)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the #portal
                        // ref the strip's FRAME0 base answers (the
                        // unknown ref the kind's own row keeps)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "nether_portal" => atlas::NETHER_PORTAL_FRAME0,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the model's
                        // shade:true element the direction table answers
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: true,
                            per_texel_alpha: true,
                        });
                    }
                    continue;
                }
                // THE CANDLE CAKE PAIR'S OWN ELEMENT LANE (the
                // autopush-523 round — the canLight consult's third row
                // finally RENDERS). blockstates/candle_cake.json's two
                // rows: "lit=false" -> block/candle_cake and "lit=true"
                // -> block/candle_cake_lit, BOTH parents
                // block/template_cake_with_candle — the cake slab box
                // [1,0,1]->[15,8,15] (the #bottom/#top/#side refs the
                // cake masters answer) PLUS the candle stick box
                // [7,8,7]->[9,14,9] (the #candle ref block/candle the
                // unlit row names, block/candle_lit the lit row — the
                // vanilla's own texture overrides the per-quad verdict
                // reads through the PALETTE's own candle tiles) PLUS
                // the two ±45° rotY wick planes the derive walk bakes
                // (the campfire lane's rotated planes speak). The
                // elements carry `shade: true` — the vanilla
                // tesselation's FACE_SHADE term the cube path's own
                // table wears (up 1.0, down 0.5, the four horizontals
                // 0.8/0.6), the term the torch/fire lanes' `shade:
                // false` elements never pay. The cell's own light every
                // corner samples (no AO diagonals — the same withoutAO
                // fold the element lanes keep); the cullfaces the slab
                // and the stick's own down rows carry the six-direction
                // neighbor read culls (the campfire lane's general
                // verdict the candle lane's down-only arm generalizes).
                if crate::world::is_candle_cake(id) {
                    let model_id = if id == crate::world::CANDLE_CAKE_LIT {
                        "block/candle_cake_lit"
                    } else {
                        "block/candle_cake"
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the no-atlas
                    // boot's own default (the cake_side row the
                    // palette arm banks — the resolved texture verdict
                    // the named consts duplicate)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in cake_candle_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the
                        // slab's and the stick's own down rows the
                        // neighbor read culls (the vanilla cullface
                        // law: the neighbor's face the solid render
                        // answers — the campfire lane's own walk)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // cake masters the named consts bank, the
                        // candle faces the palette's own candle tiles
                        // answer — the same block/candle and
                        // block/candle_lit masters the kind rows keep,
                        // the unknown ref the kind's own row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "cake_bottom" => atlas::TILE_CAKE_BOTTOM,
                            "cake_top" => atlas::TILE_CAKE_TOP,
                            "cake_side" => atlas::TILE_CAKE_SIDE,
                            "candle" => atlas::FACE_TILES[crate::world::CANDLE as usize][2],
                            "candle_lit" => {
                                atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2]
                            }
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the template's
                        // shade:true elements the direction table
                        // answers, the never-shaded the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE CAKE LADDER'S OWN ELEMENT LANE (the autopush-524
                // round — the full cake and the six bitten stages the
                // eat lane's kind swaps wear, the candle-cake lane's
                // own single-box law the ladder generalizes). The
                // blockstate's "bites=N" rows block/cake and
                // block/cake_sliceN resolve — each a SINGLE box element
                // [1+2N, 0, 1]->[15, 8, 15] (the lambda$static$0 shape
                // the x-start walk the models exactly mirror), #bottom/
                // #top/#side the cake masters name, the west face
                // #inside the bitten cut speaks (block/cake_inner the
                // bank's own fourth tile answers). The elements carry
                // `shade: true` — the vanilla tesselation's FACE_SHADE
                // term the cube path's own table wears (up 1.0, down
                // 0.5, the four horizontals 0.8/0.6), the term the
                // torch/fire lanes' `shade: false` elements never pay.
                // The cell's own light every corner samples (no AO
                // diagonals — the same withoutAO fold the element lanes
                // keep); the down face's `cullface: down` the below
                // cell's opaque verdict culls (the candle-cake lane's
                // own six-direction verdict the single row keeps).
                if crate::world::is_cake(id) {
                    let model_id = match id {
                        crate::world::CAKE_SLICE1 => "block/cake_slice1",
                        crate::world::CAKE_SLICE2 => "block/cake_slice2",
                        crate::world::CAKE_SLICE3 => "block/cake_slice3",
                        crate::world::CAKE_SLICE4 => "block/cake_slice4",
                        crate::world::CAKE_SLICE5 => "block/cake_slice5",
                        crate::world::CAKE_SLICE6 => "block/cake_slice6",
                        _ => "block/cake",
                    };
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the kind's own FACE_TILES row — the no-atlas
                    // boot's own default (the cake_side row the
                    // palette arm banks — the resolved texture verdict
                    // the named consts duplicate)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if fallback_tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in cake_model_quads(model_id) {
                        // THE SIX-DIRECTION CULLFACE VERDICT — the
                        // box's own down row the neighbor read culls
                        // (the vanilla cullface law: the neighbor's
                        // face the solid render answers — the
                        // candle-cake lane's own walk)
                        if let Some(dir) = q.cullface {
                            let (dx, dy, dz) = match dir {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // cake masters the named consts bank, the
                        // bitten cut the TILE_CAKE_INNER const answers
                        // — the unknown ref the kind's own row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let tile = match tex_stem {
                            "cake_bottom" => atlas::TILE_CAKE_BOTTOM,
                            "cake_top" => atlas::TILE_CAKE_TOP,
                            "cake_side" => atlas::TILE_CAKE_SIDE,
                            "cake_inner" => atlas::TILE_CAKE_INNER,
                            _ => fallback_tile,
                        };
                        // THE SHADE TERM'S OWN READ — the template's
                        // shade:true elements the direction table
                        // answers, the never-shaded the 1.0 keeps
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE DOOR FAMILY'S OWN ELEMENT LANE (the autopush-655
                // round — the vocabulary's next frontier the flip walk
                // renders: the oak door the two-halves x two-open-bits
                // ladder wears). The blockstates oak_door.json's rows
                // the four offline kinds resolve (the hinge-LEFT rows
                // the ctor's registerDefaultState pin speaks — the
                // closed rows oak_door_bottom_left/top_left with the
                // y table north 270 / east 0 / south 90 / west 180,
                // the open rows the _open models with north 0 / east
                // 90 / south 180 / west 270), each model ONE box
                // element [0,0,0]->[3,16,16] (the base west strip x
                // 0..3 — the panel edge the OPPOSITE facing the closed
                // state wears, the facing's LEFT the left-hinge swing
                // answers: the SHAPES clinit's boxZ(16.0, 13.0, 16.0)
                // box(0,0,16,16,16,13) the facing=north panel at z
                // 13..16 the model's own y-270 rotation the SAME edge
                // the consistency check answers). The derive walk the
                // real geometry resolves (`derive_model_inherited` —
                // the parent chain the #bottom/#top refs the resolved
                // textures answer, the down/up faces' uv rotation 90
                // the Quadrant.rotateVertexIndex law bakes,
                // ambientocclusion false the withoutAO fold keeps —
                // the cell's own light every corner samples, the
                // element's shade:true the direction table pays).
                // THE FACING the variant byte carries (the orientable
                // vocabulary's 1..=4 law — the placement lane stamps,
                // the byte's no-opinion answer the north default the
                // y=0 row the kind resolution banks): the theta tables
                // the blockstate's own y values fold — the closed
                // panel's steps CW about the cell center (north 3,
                // east 0, south 1, west 2), the open's swing (north 0,
                // east 1, south 2, west 3) — the vertices AND the
                // face directions rotate together (the west strip
                // under one CW step the north edge the (x,z)->(-z,x)
                // map serves, the cullface the same rotation the
                // neighbor reads keep). THE CULLFACE VERDICT the six-
                // direction law the cake lane wears (the rotated
                // direction the neighbor read answers — the vanilla
                // cullface law the solid render answers), the per-quad
                // texture verdict the named consts read (the lower's
                // #bottom / the upper's #top the two door masters
                // bank).
                if crate::world::is_door(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE MODEL + STEPS THE KIND ANSWERS (the closed
                    // y table north 270 / east 0 / south 90 / west 180
                    // -> the steps [3, 0, 1, 2], the open's north 0 /
                    // east 90 / south 180 / west 270 -> [0, 1, 2, 3] —
                    // the 90-degree CW walk the blockstate y values
                    // fold; the variant byte's no-opinion answer the
                    // north default the idx-0 keeps). THE HINGE-RIGHT
                    // OPEN'S OWN ROW (the autopush-658 round — the
                    // blockstate hinge=right open rows north 180 /
                    // east 270 / south 0 / west 90 -> [2, 3, 0, 1] the
                    // same fold speaks; the closed rows the hinge the
                    // tables never wear — the LEFT table both hinges
                    // the closed state serves). THE MODEL ROW THE
                    // GENERATED TABLE WEARS (the autopush-656 round —
                    // the DOOR_MODEL_BY_ROW ref the blockstate's own
                    // facing=north row resolves: the LEFT quartets the
                    // hinge=left rows the _left models speak, the
                    // RIGHT tail the hinge=right rows the _right
                    // models serve [the autopush-658 round] — the
                    // WAXED ladder the UNWAXED models the jar's own
                    // rows answer, the real chain the bake keeps)
                    let facing = world.variant(wx, wy, wz);
                    let idx = if facing == 0 {
                        0usize // the unpropertied default: north
                    } else {
                        (facing - 1) as usize // 1..=4 the N/E/S/W rows
                    };
                    let open_steps = [0usize, 1, 2, 3];
                    let open_steps_right = [2usize, 3, 0, 1];
                    let closed_steps = [3usize, 0, 1, 2];
                    let steps = if crate::world::is_door_open(id) {
                        if crate::world::door_hinge_right(id) {
                            open_steps_right[idx]
                        } else {
                            open_steps[idx]
                        }
                    } else {
                        closed_steps[idx]
                    };
                    for q in door_model_quads(id) {
                        // THE ROTATED CULLFACE VERDICT — the six-
                        // direction law the cake lane wears, the
                        // direction the SAME steps the geometry turns
                        // (the west cullface under one CW step the
                        // north neighbor read answers)
                        if let Some(dir) = q.cullface {
                            let rotated = rotate_direction_cw(dir, steps);
                            let (dx, dy, dz) = match rotated {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // per-wood door masters the named consts bank,
                        // the unknown ref the kind's own row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        // THE PER-QUAD TEXTURE VERDICT — the resolved
                        // texture name the tile mapping answers (the
                        // per-wood door masters the named consts bank,
                        // the unknown ref the kind's own row serves).
                        // THE WOOD WALK THE STEM'S OWN BASE READS (the
                        // dev-43 round — the {wood}_door base the
                        // twelve-pair table keys, the half suffix the
                        // pair's own bottom/top picks)
                        let fallback_tile = if (id as usize) < atlas::N_KINDS {
                            atlas::FACE_TILES[id as usize][2]
                        } else {
                            atlas::TILE_NONE
                        };
                        let tile = door_master_tile(tex_stem).unwrap_or(fallback_tile);
                        let base = if tile == atlas::TILE_NONE {
                            crate::colors::palette_color(id, 2)
                                .or_else(|| lut.and_then(|l| l.color(id, 2)))
                                .unwrap_or_else(|| default_color(id, 2))
                        } else {
                            [1.0; 3]
                        };
                        // THE SHADE TERM'S OWN READ — the element's
                        // shade:true the direction table answers (the
                        // ROTATED direction the world-facing term
                        // keeps), the never-shaded the 1.0 keeps
                        let rotated_dir = rotate_direction_cw(q.direction, steps);
                        let face_shade = if q.shade {
                            match rotated_dir {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE STEPS' OWN TURN (the blockstate y
                            // values the CW walk folds: each step the
                            // (x,z)->(-z,x) map about the cell center
                            // serves — the west strip under one step
                            // the north edge answers)
                            let (dx, dz) = (v[0] - 0.5, v[2] - 0.5);
                            let (rx, rz) = rotate_xz_cw(dx, dz, steps);
                            pts[i] = [
                                wx as f32 + 0.5 + rx,
                                wy as f32 + v[1],
                                wz as f32 + 0.5 + rz,
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE FENCE GATE'S OWN ELEMENT LANE (the dev-53 round
                // — the gate family the derive walk renders: the
                // blockstates oak_fence_gate.json's facing=south,
                // in_wall=false rows the two offline kinds resolve —
                // the closed rows block/oak_fence_gate, the open rows
                // block/oak_fence_gate_open, each model the template
                // chain's EIGHT elements speaks (the two posts + the
                // two inner verticals + the four bars — the derive
                // walk the real geometry resolves, `derive_model_
                // inherited` the parent chain the #texture refs the
                // resolved textures answer — the ONE block/oak_planks
                // master every face wears, ambientocclusion default
                // the AO-less fold the door lane's own law keeps —
                // the cell's own light every corner samples, the
                // element's shade:true the direction table pays).
                // THE FACING the variant byte carries (the orientable
                // vocabulary's 1..=4 law — the placement lane stamps,
                // the byte's no-opinion answer the north default the
                // idx-0 row keeps): the theta tables the blockstate's
                // own y values fold — the closed AND open rows the
                // SAME y table wear (north 180, east 270, south 0,
                // west 90 — the 90-degree CW walk the blockstate y
                // values fold, the vertices AND the face directions
                // rotate together — the south-facing gate the y=0
                // authoring the as-is render speaks). THE CULLFACE
                // VERDICT the six-direction law the door lane wears
                // (the rotated direction the neighbor read answers —
                // the vanilla cullface law the solid render answers),
                // the per-quad texture verdict the ONE planks master
                // reads (the unknown ref the kind's own row serves).
                if crate::world::is_fence_gate(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE MODEL + STEPS THE KIND ANSWERS (the y table
                    // north 180 / east 270 / south 0 / west 90 -> the
                    // steps [2, 3, 0, 1] — the 90-degree CW walk the
                    // blockstate y values fold; the variant byte's
                    // no-opinion answer the north default the idx-0
                    // keeps. THE MODEL ROW THE KIND'S OWN HALF PICKS
                    // (the closed rows block/oak_fence_gate, the open
                    // rows block/oak_fence_gate_open — the derive walk
                    // the EIGHT-element geometry resolves)
                    let facing = world.variant(wx, wy, wz);
                    let idx = if facing == 0 {
                        0usize // the unpropertied default: north
                    } else {
                        (facing - 1) as usize // 1..=4 the N/E/S/W rows
                    };
                    let gate_steps = [2usize, 3, 0, 1];
                    let steps = gate_steps[idx];
                    for q in gate_model_quads(id) {
                        // THE ROTATED CULLFACE VERDICT — the six-
                        // direction law the door lane wears, the
                        // direction the SAME steps the geometry turns
                        if let Some(dir) = q.cullface {
                            let rotated = rotate_direction_cw(dir, steps);
                            let (dx, dy, dz) = match rotated {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE PER-QUAD TEXTURE VERDICT — the gate's own
                        // #texture ref block/oak_planks the row's own
                        // tile banks — the planks master the FACE_TILES
                        // row already banks (the generator's gate arm
                        // the six slots the ONE master wears — the
                        // lever/button lanes' own single-tile verdict
                        // the family keeps; the derive quads' per-quad
                        // stems the ONE ref the whole chain resolves,
                        // the kind's own row the honest mapping keeps)
                        let fallback_tile = if (id as usize) < atlas::N_KINDS {
                            atlas::FACE_TILES[id as usize][2]
                        } else {
                            atlas::TILE_NONE
                        };
                        let tile = fallback_tile;
                        let base = if tile == atlas::TILE_NONE {
                            crate::colors::palette_color(id, 2)
                                .or_else(|| lut.and_then(|l| l.color(id, 2)))
                                .unwrap_or_else(|| default_color(id, 2))
                        } else {
                            [1.0; 3]
                        };
                        // THE SHADE TERM'S OWN READ — the element's
                        // shade:true the direction table answers (the
                        // ROTATED direction the world-facing term
                        // keeps), the never-shaded the 1.0 keeps
                        let rotated_dir = rotate_direction_cw(q.direction, steps);
                        let face_shade = if q.shade {
                            match rotated_dir {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE STEPS' OWN TURN (the blockstate y
                            // values the CW walk folds: each step the
                            // (x,z)->(-z,x) map about the cell center
                            // serves — the door lane's own walk shares)
                            let (dx, dz) = (v[0] - 0.5, v[2] - 0.5);
                            let (rx, rz) = rotate_xz_cw(dx, dz, steps);
                            pts[i] = [
                                wx as f32 + 0.5 + rx,
                                wy as f32 + v[1],
                                wz as f32 + 0.5 + rz,
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE FENCE FAMILY'S OWN MULTIPART LANE (the dev-55
                // round — the enclosure rung the palette finally
                // renders: the blockstates {wood}_fence.json's
                // MULTIPART the UNCONDITIONAL post apply + the four
                // side arms the when-rows wear, the select_parts walk
                // the CONNECTION BITS the mesh-time law computes the
                // props answer — the variant byte the no-opinion 0
                // keeps, the four neighbor reads the same verdict
                // every frame answers, the placement/break updates
                // the propagation never needs). The models the
                // template fence_post/fence_side parents the derive
                // walk resolves (the post element [6,0,6]->[10,16,10]
                // the center post speaks, the side's TWO bar elements
                // the [7,12,0]->[9,15,9] top + [7,6,0]->[9,9,9] lower
                // pair wear), the ONE #texture ref the per-wood
                // planks master every face banks (the FACE_TILES row
                // the same master the tile keeps). THE UVLOCK the
                // side arms' y quarter the rotation ride — the uvs
                // the planks master's own grain the as-authored
                // orientation keeps (the fire lane's own no-uvlock
                // precedent the same rotation walk wears — the
                // uniform plank grain the quarter-turn delta the
                // honest deviation notes). THE CULLFACE VERDICT the
                // six-direction law the gate lane wears (the rotated
                // direction the neighbor read answers — the post's
                // up/down cullfaces the solid roof/floor the culled
                // verdict keeps), the shade:true faces the direction
                // table the gate's own walk pays.
                if crate::world::is_fence(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE CONNECTION BITS the mesh-time law computes —
                    // the N/E/S/W when-rows the props answer
                    let bits = crate::world::fence_connection_bits(world, wx, wy, wz);
                    let props = [
                        ("north", if bits & 0b0001 != 0 { "true" } else { "false" }),
                        ("east", if bits & 0b0010 != 0 { "true" } else { "false" }),
                        ("south", if bits & 0b0100 != 0 { "true" } else { "false" }),
                        ("west", if bits & 0b1000 != 0 { "true" } else { "false" }),
                    ];
                    // THE ONE PLANKS MASTER the FACE_TILES row the tile
                    // banks (the gate lane's own fallback law the
                    // family keeps)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    let (post_quads, side_quads) = fence_models(id);
                    for apply in select_parts(fence_parts(id), &props) {
                        let model_id = apply.model.strip_prefix("minecraft:").unwrap_or(&apply.model);
                        // the side model's y quarter the blockstate's
                        // own y values speak (north 0 / east 90 /
                        // south 180 / west 270 — the steps the CW
                        // count answers); the post the y=0 row the
                        // unconditional apply wears
                        let (steps, model) = if model_id.ends_with("_post") {
                            (0usize, post_quads)
                        } else if model_id.ends_with("_side") {
                            (((apply.y as usize) / 90) % 4, side_quads)
                        } else {
                            (((apply.y as usize) / 90) % 4, post_quads)
                        };
                        for q in model {
                            // THE ROTATED CULLFACE VERDICT — the six-
                            // direction law the gate lane wears, the
                            // direction the SAME steps the geometry
                            // turns
                            if let Some(dir) = q.cullface {
                                let rotated = rotate_direction_cw(dir, steps);
                                let (dx, dy, dz) = match rotated {
                                    mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                    mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                    mc_registry::element_quads::Direction::North => (0, 0, -1),
                                    mc_registry::element_quads::Direction::South => (0, 0, 1),
                                    mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                    mc_registry::element_quads::Direction::East => (1, 0, 0),
                                };
                                if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                    continue;
                                }
                            }
                            // THE SHADE TERM'S OWN READ — the element's
                            // shade:true the direction table answers
                            // (the ROTATED direction the world-facing
                            // term keeps), the never-shaded the 1.0
                            // keeps
                            let rotated_dir = rotate_direction_cw(q.direction, steps);
                            let face_shade = if q.shade {
                                match rotated_dir {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                // THE STEPS' OWN TURN (the blockstate y
                                // values the CW walk folds — the fire
                                // lane's own quarter walk the same map
                                // wears)
                                let (vx, vz) = match steps {
                                    1 => (1.0 - v[2], v[0]),
                                    2 => (1.0 - v[0], 1.0 - v[2]),
                                    3 => (v[2], 1.0 - v[0]),
                                    _ => (v[0], v[2]),
                                };
                                pts[i] = [
                                    wx as f32 + vx,
                                    wy as f32 + v[1],
                                    wz as f32 + vz,
                                ];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                colors[i] = [
                                    base[0] * lm[0] * face_shade,
                                    base[1] * lm[1] * face_shade,
                                    base[2] * lm[2] * face_shade,
                                ];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                                per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE WALL FAMILY'S OWN MULTIPART LANE (the dev-56
                // round — the rampart rung the palette finally
                // renders: the blockstates {base}_wall.json's
                // MULTIPART the up=true CONDITIONAL post apply + the
                // four side arms the when-rows wear, the select_parts
                // walk the CONNECTION STATE the mesh-time law computes
                // the props answer — the variant byte the no-opinion 0
                // keeps, the four neighbor reads + the above-face
                // cover reads + the raise law the same verdict every
                // frame answers, the placement/break updates the
                // propagation never needs). The models the template
                // wall_post/wall_side/wall_side_tall parents the
                // derive walk resolves (the post element [4,0,4]->
                // [12,16,12] the center post speaks, the side's ONE
                // bar element the [5,0,0]->[11,14,8] low band wears,
                // the tall's [5,0,0]->[11,16,8] the full-height bar
                // answers), the ONE #texture ref the per-wall BASE
                // master every face banks (the FACE_TILES row the
                // same master the tile keeps). THE UVLOCK the side
                // arms' y quarter the rotation ride — the uvs the
                // master's own grain the as-authored orientation
                // keeps (the fence lane's own precedent the same
                // rotation walk wears — the uniform stone grain the
                // quarter-turn delta the honest deviation notes). THE
                // CULLFACE VERDICT the six-direction law the fence
                // lane wears (the rotated direction the neighbor read
                // answers — the post's up/down cullfaces the solid
                // roof/floor the culled verdict keeps), the shade:true
                // faces the direction table the fence's own walk pays.
                if crate::world::is_wall(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE CONNECTION STATE the mesh-time law computes —
                    // the N/E/S/W when-rows the LOW/TALL verdicts the
                    // props answer, the UP row the post apply's own
                    // gate wears
                    let bits = crate::world::wall_connection_bits(world, wx, wy, wz);
                    let dir_of = |d: u8| match d {
                        0b0001 => 1u8,
                        0b0010 => 2,
                        0b0100 => 3,
                        _ => 4,
                    };
                    let tall_for = |d: u8| {
                        bits & d != 0
                            && crate::world::wall_above_covers(
                                world,
                                wx,
                                wy + 1,
                                wz,
                                dir_of(d),
                            )
                    };
                    let side_word = |d: u8| {
                        if bits & d == 0 {
                            "none"
                        } else if tall_for(d) {
                            "tall"
                        } else {
                            "low"
                        }
                    };
                    let up = crate::world::wall_up(world, wx, wy, wz);
                    let props = [
                        ("north", side_word(0b0001)),
                        ("east", side_word(0b0010)),
                        ("south", side_word(0b0100)),
                        ("west", side_word(0b1000)),
                        ("up", if up { "true" } else { "false" }),
                    ];
                    // THE ONE BASE MASTER the FACE_TILES row the tile
                    // banks (the fence lane's own fallback law the
                    // family keeps)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    let (post_quads, side_quads, tall_quads) = wall_models(id);
                    for apply in select_parts(wall_parts(id), &props) {
                        let model_id = apply.model.strip_prefix("minecraft:").unwrap_or(&apply.model);
                        // the side model's y quarter the blockstate's
                        // own y values speak (north 0 / east 90 /
                        // south 180 / west 270 — the steps the CW
                        // count answers); the post the y=0 row the
                        // conditional apply wears
                        let (steps, model) = if model_id.ends_with("_post") {
                            (0usize, post_quads)
                        } else if model_id.ends_with("_side_tall") {
                            (((apply.y as usize) / 90) % 4, tall_quads)
                        } else if model_id.ends_with("_side") {
                            (((apply.y as usize) / 90) % 4, side_quads)
                        } else {
                            (((apply.y as usize) / 90) % 4, post_quads)
                        };
                        for q in model {
                            // THE ROTATED CULLFACE VERDICT — the six-
                            // direction law the fence lane wears, the
                            // direction the SAME steps the geometry
                            // turns
                            if let Some(dir) = q.cullface {
                                let rotated = rotate_direction_cw(dir, steps);
                                let (dx, dy, dz) = match rotated {
                                    mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                    mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                    mc_registry::element_quads::Direction::North => (0, 0, -1),
                                    mc_registry::element_quads::Direction::South => (0, 0, 1),
                                    mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                    mc_registry::element_quads::Direction::East => (1, 0, 0),
                                };
                                if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                    continue;
                                }
                            }
                            // THE SHADE TERM'S OWN READ — the element's
                            // shade:true the direction table answers
                            // (the ROTATED direction the world-facing
                            // term keeps), the never-shaded the 1.0
                            // keeps
                            let rotated_dir = rotate_direction_cw(q.direction, steps);
                            let face_shade = if q.shade {
                                match rotated_dir {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                // THE STEPS' OWN TURN (the blockstate y
                                // values the CW walk folds — the fence
                                // lane's own quarter walk the same map
                                // wears)
                                let (vx, vz) = match steps {
                                    1 => (1.0 - v[2], v[0]),
                                    2 => (1.0 - v[0], 1.0 - v[2]),
                                    3 => (v[2], 1.0 - v[0]),
                                    _ => (v[0], v[2]),
                                };
                                pts[i] = [
                                    wx as f32 + vx,
                                    wy as f32 + v[1],
                                    wz as f32 + vz,
                                ];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                colors[i] = [
                                    base[0] * lm[0] * face_shade,
                                    base[1] * lm[1] * face_shade,
                                    base[2] * lm[2] * face_shade,
                                ];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                                per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE TRAPDOOR'S OWN ELEMENT LANE (the autopush-668
                // round — the door family's thin-panel sibling the
                // interactive lane finally renders). The blockstates'
                // oak_trapdoor.json rows the THREE models resolve (the
                // closed bottom block/oak_trapdoor_bottom the HALF=
                // bottom state wears, the closed top block/oak_trapdoor_
                // top the HALF=top state keeps, the open block/oak_
                // trapdoor_open the swung panel answers), each ONE box
                // element (the closed bottom [0,0,0]->[16,3,16], the
                // closed top [0,13,0]->[16,16,16], the open [0,0,13]->
                // [16,16,16] the north strip the base shape the SHAPES
                // map's identity arm banks — the facing=east closed rows
                // the NO-y rows the symmetric slab never turns, the open
                // rows the y table north 0 / east 90 / south 180 /
                // west 270 the per-facing steps speak). The derive walk
                // the real geometry resolves (`derive_model_inherited` —
                // the parent chain the #texture ref the merged map
                // answers, the oak_trapdoor master tile the kind's own
                // seat banks, ambientocclusion default the AO-less fold
                // the door lane's own law keeps — the cell's own light
                // every corner samples, the element's shade:true the
                // direction table pays). THE FACING + HALF bits the
                // variant byte carries (the wall torch's own vocabulary
                // the low nibble keeps, the +8 bit the half word banks).
                // THE OPEN'S OWN STEPS (the blockstate y values the CW
                // walk folds: north 0, east 1, south 2, west 3 — the
                // south-strip panel under one step the west edge the
                // (x,z)->(-z,x) map serves, the panel the OPPOSITE
                // facing the vanilla's own SHAPES map banks) — the
                // vertices AND the cullfaces rotate together (the
                // door lane's own walk shares), the closed models the
                // identity steps the slab never turns.
                if crate::world::is_trapdoor(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let variant = world.variant(wx, wy, wz);
                    let open = crate::world::trapdoor_open(id);
                    let half_top = crate::world::trapdoor_half_top(variant);
                    let facing = crate::world::trapdoor_facing_of(variant);
                    // THE LADDER'S OWN STEM (the kind's own wood the
                    // stem walk answers — the per-wood models the bake
                    // reads, the unknown id the empty slice keeps)
                    let stem = crate::world::trapdoor_stem_of(id).unwrap_or("oak_trapdoor");
                    let element_quads = trapdoor_model_quads(stem, half_top, open);
                    // THE STEPS' OWN TABLE (the open's swing the
                    // blockstate y values fold — north 0, east 1, south
                    // 2, west 3 — the closed slabs the identity the
                    // symmetric geometry keeps)
                    let steps = if open {
                        match facing {
                            crate::world::WALL_TORCH_FACING_EAST => 1usize,
                            crate::world::WALL_TORCH_FACING_SOUTH => 2,
                            crate::world::WALL_TORCH_FACING_WEST => 3,
                            _ => 0,
                        }
                    } else {
                        0
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // `shade: true` faces the direction table pays —
                    // the tile's own pixels the white base keeps
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for q in element_quads {
                        // THE ROTATED CULLFACE VERDICT — the six-
                        // direction law the cake lane wears, the
                        // direction the SAME steps the geometry turns
                        if let Some(dir) = q.cullface {
                            let rotated = rotate_direction_cw(dir, steps);
                            let (dx, dy, dz) = match rotated {
                                mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                mc_registry::element_quads::Direction::North => (0, 0, -1),
                                mc_registry::element_quads::Direction::South => (0, 0, 1),
                                mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                mc_registry::element_quads::Direction::East => (1, 0, 0),
                            };
                            if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                continue;
                            }
                        }
                        // THE SHADE TERM'S OWN READ — the element's
                        // shade:true the direction table answers (the
                        // ROTATED direction the world-facing term
                        // keeps), the never-shaded the 1.0 keeps
                        let rotated_dir = rotate_direction_cw(q.direction, steps);
                        let face_shade = if q.shade {
                            match rotated_dir {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE STEPS' OWN TURN (the blockstate y
                            // values the CW walk folds: each step the
                            // (x,z)->(-z,x) map about the cell center
                            // serves — the south strip under one step
                            // the west edge answers)
                            let (dx, dz) = (v[0] - 0.5, v[2] - 0.5);
                            let (rx, rz) = rotate_xz_cw(dx, dz, steps);
                            pts[i] = [
                                wx as f32 + 0.5 + rx,
                                wy as f32 + v[1],
                                wz as f32 + 0.5 + rz,
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE FIRE KINDS' OWN MULTIPART LANE (the flint's and the
                // charge's own product finally RENDERS — the honest hole
                // the glass precedent wore, the fan the vanilla blockstate
                // speaks). blockstates/fire.json is a MULTIPART over the
                // five wall booleans (north/east/south/west/up): the floor
                // pair under the all-false condition, the four side
                // groups each under OR[dir=true, all-false], the up pair
                // under up=true; blockstates/soul_fire.json carries NO
                // when — the full fan always, the up group never (the
                // soul flame the directional properties never wears).
                // THE PROPS' OWN DERIVATION — javap `FireBlock.
                // getStateForPlacement(BlockGetter, BlockPos)` (the 26.2
                // jar): the below cell's `canBurn(belowState) ||
                // belowState.isFaceSturdy(level, below, UP)` gate pc 14-33
                // serves the DEFAULT state (all five booleans false) when
                // it answers, else the `Direction.values()` walk's
                // `setValue(PROPERTY_BY_DIRECTION.get(d),
                // canBurn(getBlockState(pos.relative(d))))` pc 47-128
                // reads the DIRECT neighbor per direction (DOWN rides no
                // property — the map's own row the null check skips).
                // `updateShape` re-runs the SAME walk (`getStateWithAge`
                // -> `getState` -> the placement's own law) — the live
                // neighbor read the offline single-kind world wears as
                // its steady state. The sturdy arm the collision table's
                // Full row answers (the same offline equivalence the
                // survival law's doc keeps), the canBurn odds the
                // placement round's own `fire_can_burn` table speaks.
                // THE SELECTION rides the real blockstate engine (the
                // parse + select_parts walk the fence/wire tests pin) —
                // the jar's own when-conditions, the jar's own y rotations.
                // THE MODEL QUADS ride the derive walk (the torch lane's
                // own bake): template_fire_floor's ±22.5° rotX/rotZ +
                // rescale planes, template_fire_side's flat z=0.01 walls,
                // template_fire_up's rotated ceil planes with their uv
                // rotations — the census-derived ELEMENTS table banks all
                // ten fire and six soul models, `derive_model_inherited`
                // resolves the parent chain and the #fire refs. The
                // second-frame carriers (block/fire_1, block/soul_fire_1
                // — fire_floor1/fire_up1/soul_fire_floor1's own rows) map
                // to the kind's SINGLE atlas tile (the static-frame
                // policy the fluids wear — the animation strips the
                // palette's frame-0 banking keeps). `shade: false` +
                // model-level `ambientocclusion: false` on every fire
                // element — the tesselateWithoutAO shape the crop and
                // torch lanes wear: the CELL's own light, no AO diagonals,
                // no FACE_SHADE term. No cullface anywhere — the planes
                // draw unconditionally; the multipart's `y` rotation
                // (no uvlock on any fire apply) turns the side walls
                // about the cell's own center: javap
                // `Variant$SimpleModelState.asModelState` ->
                // `Quadrant.fromXYZAngles` -> `OctahedralGroup.
                // BLOCK_ROT_Y_90` = `ROT_90_Y_NEG` = (P321, flipX) —
                // the (x, z) -> (1-z, x) quarter the north-to-east turn
                // speaks, 180 the double, 270 the inverse.
                if crate::world::is_fire(id) {
                    let soul = id == crate::world::SOUL_FIRE;
                    let below = world.get(wx, wy - 1, wz);
                    let gate = crate::world::fire_can_burn(below)
                        || crate::world::collision_shape(below)
                            == crate::world::CollisionShape::Full;
                    let prop = |read: bool| {
                        // the gate's own verdict — the default state's
                        // all-false shape the below cell's gate keeps
                        if gate { "false" } else if read { "true" } else { "false" }
                    };
                    let props = [
                        ("north", prop(crate::world::fire_can_burn(world.get(wx, wy, wz - 1)))),
                        ("east", prop(crate::world::fire_can_burn(world.get(wx + 1, wy, wz)))),
                        ("south", prop(crate::world::fire_can_burn(world.get(wx, wy, wz + 1)))),
                        ("west", prop(crate::world::fire_can_burn(world.get(wx - 1, wy, wz)))),
                        ("up", prop(crate::world::fire_can_burn(world.get(wx, wy + 1, wz)))),
                    ];
                    // The attribute-system fold (the shared per-mesh
                    // uniforms both existing paths speak — see the flat
                    // path's own walk for the javap-proven arithmetic).
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // THE CELL'S OWN LIGHT — the withoutAO walk's
                    // getLightColor(state, pos): both layers sampled AT
                    // the flame cell, all corners the same sample (the
                    // grid absent keeps the legacy unshaded look — the
                    // same None convention both paths speak).
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // THE SECOND FRAME'S OWN FAMILY — the model id's
                    // trailing digit the blockstate's own apply pairs
                    // speak (fire_floor0 AND fire_floor1, the side0/1 +
                    // side_alt0/1 fours, the up pair — both families the
                    // SAME group applies, the two overlapped quads the
                    // independent sprite flicker speaks): the *_1 models
                    // the fire_1/soul_fire_1 strip's frame-0 tile bake
                    // (the remap's own base), the *_0 family the kind's
                    // single atlas tile keeps (the fire_0/soul_fire_0
                    // strip's first frame the block palette banks). The
                    // frames themselves the DRAW-TIME remap swaps
                    // (`atlas::anim_tile` — the mesher never
                    // re-meshes for the flicker).
                    let tile_of_family = |model_id: &str| -> u16 {
                        if (id as usize) >= atlas::N_KINDS {
                            return atlas::TILE_NONE;
                        }
                        if model_id.ends_with('1') {
                            if soul {
                                atlas::SOUL_FIRE_1_FRAMES[0]
                            } else {
                                atlas::FIRE_1_FRAMES[0]
                            }
                        } else {
                            tile
                        }
                    };
                    // `shade: false` + the no-atlas boot's own default —
                    // the tile's own pixels speak, the flat palette the
                    // missing row keeps (the torch path's own verdict)
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    for apply in select_parts(fire_parts(soul), &props) {
                        // the namespaced ref the blockstate writes the
                        // vanilla ModelResourceLocation parse resolves —
                        // the bare `builtin/` refs the four special forms
                        // keep ride the same strip (the fire applies all
                        // carry the minecraft: prefix, the honest default)
                        let model_id = apply.model.strip_prefix("minecraft:").unwrap_or(apply.model);
                        let Some(model) = fire_model_quads().get(model_id) else {
                            continue;
                        };
                        let tile = tile_of_family(model_id);
                        for q in model {
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                // the multipart's y quarter about the cell's
                                // own center (no uvlock — the uvs ride)
                                let (vx, vz) = match (apply.y / 90) % 4 {
                                    1 => (1.0 - v[2], v[0]),
                                    2 => (1.0 - v[0], 1.0 - v[2]),
                                    3 => (v[2], 1.0 - v[0]),
                                    _ => (v[0], v[2]),
                                };
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                // `shade: false` — no FACE_SHADE term,
                                // the lightmap alone multiplies
                                pts[i] = [wx as f32 + vx, wy as f32 + v[1], wz as f32 + vz];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                colors[i] = [base[0] * lm[0], base[1] * lm[1], base[2] * lm[2]];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                            per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE BAMBOO STALK'S OWN MULTIPART LANE (the autopush-
                // 608 round — the sword's f32::MAX instant lane the
                // palette finally wakes, the render the multipart's own
                // walk serves). blockstates/bamboo.json is a MULTIPART
                // over the AGE and the LEAVES properties: the age arms
                // apply FOUR overlapping stalk models each (the
                // bamboo1..4_age0 fours the SAME [7,0,7]->[9,16,9] box
                // wear — the side UV strips alone differ: [0..2]/[3..5]/
                // [6..8]/[9..11] of the bamboo_stalk master; the age1
                // fours the [6.5,0,6.5]->[9.5,16,9.5] box the same four
                // strips wear), the leaves arms the crossed-plane pairs
                // (the flowers' own 0.8..15.2 inset geometry the
                // shade:false faces speak).
                // THE PROPS' OWN DERIVATION — the kind-only vocabulary
                // the AGE/LEAVES/STAGE properties never STORES, but the
                // tick-grown steady state the GEOMETRY answers (the
                // autopush-616 growth walk the store's net state wears):
                // the contiguous bamboo-below count b and the
                // bamboo-above count k the capped reads speak (the
                // thresholds the laws demand the caps never sway — the
                // age law b>=3 the 3-cap pins, the leaves law the
                // k∈{0,1,2,3+} bands the 3-cap answers), h = b+k+1 the
                // plant's own height. THE LEAVES LAW the grown plants'
                // growth trace wears (the growBamboo cascade the
                // below->SMALL / twoBelow->NONE pair the large branch
                // fires): the TOP segment (k=0) the LARGE pair renders
                // once the plant reaches 4 (the below-SMALL verdict the
                // large branch serves), the h=2/3 tops the SMALL pair
                // (the sapling's own growBamboo the SMALL-leaved top
                // places); k=1 the SMALL pair (the h>=3 plants the
                // cascade's own below arm banks), k=2 the SMALL pair
                // (the h>=4 plants — the h=3 base the NONE keeps),
                // k>=3 NONE (the steady state's own floor — the
                // twoBelow->NONE arm the fourth-from-top segment
                // answers). THE H=1 LONE STALK the DEFAULT state keeps
                // (the placement law's own product — the tick-grown
                // plants the 2-tall minimum wear, the lone stalk the
                // hand-staged cell the none verdict renders). THE AGE
                // LAW the growBamboo flag speaks (the twoBelow-bamboo
                // OR the ticked-state's own age-1 the flag wears): the
                // tick-grown plants the b>=3 segments the thick age1
                // fours render, the younger the thin age0 keeps. The
                // STAGE property the fold keeps (every stalk stage 0 —
                // the bone-meal lane the client never serves).
                // THE FOUR-MODEL OVERLAP'S OWN DRAW LAW — the vanilla
                // chunk mesh draws with the GL LEQUAL depth func (the
                // RenderSystem.depthFunc(515) law), so the depth-equal
                // coplanar quads the LAST-declared apply's fragments
                // WIN — the framebuffer holds bamboo4's UV strip. This
                // raster's depth law is STRICT LESS (the first-drawn
                // quad wins — the fire lane's own floor0/floor1
                // precedent), so the lane EMITS THE AGE GROUP'S MODELS
                // IN REVERSED DECLARATION ORDER: the last-declared
                // model first, the raster's LESS the same winner the
                // vanilla LEQUAL keeps, the visible strip the vanilla
                // framebuffer's own verdict.
                // THE SHADE TERM the stalk elements' default TRUE
                // carries (the FACE_SHADE constants the cube path wears
                // — the derive quads' own direction the term reads);
                // the leaves pairs' shade:false the lightmap alone
                // multiplies. NO tint anywhere (the textures PRE-
                // COLORED — the BlockColors bamboo absence the jar's
                // own registry speaks, the tintindex the resolver never
                // serves). The XZ offset the plant walk's own row keeps
                // (the offsetType(XZ) pc 23186 the clinit arm speaks —
                // the stalk the getOffset move the vanilla getShape
                // rides). The cullface verdicts the down/up faces carry
                // (the element tables' own cullface Down/Up rows — the
                // torch lane's own law the neighbor reads keep).
                if crate::world::is_bamboo_stalk(id) {
                    // THE CAPPED NEIGHBOR COUNTS (the thresholds the
                    // caps never sway — the doc above): b the below
                    // count the age law pins at 3, k the above count
                    // the leaves bands demand at 3
                    let b = (1..4)
                        .take_while(|&d| world.get(wx, wy - d, wz) == crate::world::BAMBOO)
                        .count();
                    let k = (1..4)
                        .take_while(|&d| world.get(wx, wy + d, wz) == crate::world::BAMBOO)
                        .count();
                    let h = b + k + 1;
                    let leaves = if k == 0 {
                        if h >= 4 {
                            "large"
                        } else if h >= 2 {
                            "small"
                        } else {
                            "none"
                        }
                    } else if k == 1 {
                        if h >= 3 {
                            "small"
                        } else {
                            "none"
                        }
                    } else if k == 2 {
                        if h >= 4 {
                            "small"
                        } else {
                            "none"
                        }
                    } else {
                        "none"
                    };
                    let age = if b >= 3 { "1" } else { "0" };
                    let props = [("age", age), ("leaves", leaves)];
                    // The attribute-system fold (the shared per-mesh
                    // uniforms the fire lane's own walk speaks — the
                    // javap-proven arithmetic).
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // THE CELL'S OWN LIGHT — the withoutAO walk's
                    // getLightColor(state, pos): both layers sampled AT
                    // the stalk cell, all corners the same sample (the
                    // fire lane's own law the thin geometry keeps).
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE PLANT'S OWN RENDER OFFSET — the XZ row the
                    // plant walk keeps (the stalk the getOffset move
                    // the vanilla shape law rides).
                    let (off_x, off_y, off_z) = crate::world::plant_offset(id, wx, wy, wz);
                    let (off_x, off_y, off_z) =
                        (off_x as f32, off_y as f32, off_z as f32);
                    for apply in select_parts(bamboo_parts(), &props)
                        .into_iter()
                        .rev()
                    {
                        let model_id =
                            apply.model.strip_prefix("minecraft:").unwrap_or(apply.model);
                        let Some(model) = bamboo_model_quads().get(model_id) else {
                            continue;
                        };
                        // the per-quad texture verdict the names speak:
                        // the stalk masters the kind's own atlas tile
                        // (the FACE_TILES row the palette arm banks —
                        // every face the #all ref answers), the leaves
                        // masters the appended consts (the palette walk
                        // never references them — the tile_names walk
                        // the append banks)
                        let stalk_tile = if (id as usize) < atlas::N_KINDS {
                            atlas::FACE_TILES[id as usize][2]
                        } else {
                            atlas::TILE_NONE
                        };
                        let tile_of_texture = |texture: &str| -> u16 {
                            if texture.ends_with("bamboo_small_leaves") {
                                atlas::TILE_BAMBOO_SMALL_LEAVES
                            } else if texture.ends_with("bamboo_large_leaves") {
                                atlas::TILE_BAMBOO_LARGE_LEAVES
                            } else {
                                stalk_tile
                            }
                        };
                        // THE REVERSED EMISSION (the four-model
                        // overlap's own draw law — the vanilla LEQUAL's
                        // last-pass verdict the LESS raster the reversed
                        // order serves; the within-model quads the
                        // derive order keeps — the coplanar fight the
                        // applies' own fours alone speak)
                        for q in model.iter() {
                            let tile = tile_of_texture(q.texture);
                            // THE SHADE TERM'S OWN READ — the element's
                            // `shade` the vanilla tesselation's
                            // direction table answers (the stalk's
                            // default-true elements the cube path's
                            // constants keep, the leaves' shade:false
                            // the 1.0 keeps)
                            let face_shade = if q.shade {
                                match q.direction {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            // THE CULLFACE VERDICTS the down/up faces
                            // carry (the torch lane's own law — the
                            // neighbor the solid render answers)
                            let below = world.get(wx, wy - 1, wz);
                            if q.cullface
                                == Some(mc_registry::element_quads::Direction::Down)
                                && !is_transparent(below)
                            {
                                continue;
                            }
                            let above = world.get(wx, wy + 1, wz);
                            if q.cullface
                                == Some(mc_registry::element_quads::Direction::Up)
                                && !is_transparent(above)
                            {
                                continue;
                            }
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                // the model plane the offset + inset
                                // fold rides (the flower lane's own law
                                // — the section origin adds later, the
                                // block-local bake keeps)
                                pts[i] = [
                                    wx as f32 + v[0] + off_x,
                                    wy as f32 + v[1] + off_y,
                                    wz as f32 + v[2] + off_z,
                                ];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                colors[i] = [
                                    lm[0] * face_shade,
                                    lm[1] * face_shade,
                                    lm[2] * face_shade,
                                ];
                            }
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile,
                                translucent: false,
                                per_texel_alpha: false,
                            });
                        }
                    }
                    continue;
                }
                // THE PANE FAMILY'S OWN MULTIPART LANE (the dev-58
                // round — the thin-strip rung the palette finally
                // renders: the blockstates {name}.json's MULTIPART the
                // UNCONDITIONAL post apply + the side/noside arms (the
                // panes) or the post_ends/post/cap/side arms (the
                // bars) the select_parts walk the CONNECTION STATE the
                // mesh-time law computes the props answer — the
                // [`pane_connection_bits`] walk the four neighbor reads
                // the SAME verdict every frame answers, the
                // placement/break updates the propagation never
                // needs). The dev-59 round the COPPER BARS the lane
                // JOINS — the eight oxidization rungs the SAME lane
                // the class law renders (javap lambda$static$162 the
                // plain IronBarsBlock instances the collection walk
                // fabricates, the entry condition the
                // [`is_iron_bars_class`] range the whole thin-strip
                // family answers). The models the pane templates the derive
                // walk resolves (the post's down/up cap pair + the
                // side's END face the cullface row carries + the
                // west/east #pane long faces — the ZERO-inner-face law
                // the connected seam the z-fight never banks; the bars
                // templates the ZERO-WIDTH planes the crossed-sheet
                // look renders — the post pair x=8/z=8 planes, the
                // side's plane + the END face + the y=0.001/15.999
                // edge strips), the ambientocclusion: false the
                // templates ALL carry (the AO the quads never cast).
                // THE PER-QUAD TEXTURE VERDICT — the derived quads'
                // resolved stems the tile mapping answers (the #edge
                // refs the kind's own FACE_TILES row the pane-top
                // master banks, the #pane refs the SIBLING glass kind's
                // row the id-17 offset answers — the pane kind its
                // glass block the SAME dye the −17 seat wears, the
                // bars' #bars/#edge refs the kind's own iron_bars
                // master the row banks). THE CULLFACE VERDICT the
                // fence lane's own rotated law + THE SAME-KIND CULL
                // ARM the skipRendering law banks (javap
                // IronBarsBlock.skipRendering pc 0-124: the
                // neighborState.is(this) arm pc 0-5 + the vertical
                // axis arm pc 50-61 the stacked caps' shared plane
                // culls + the mutual-connection arm pc 62-116 the
                // connected seam faces cull — the instanceof
                // universality the adjacent same-kind panes the
                // ALWAYS-mutual verdict wears, the different-color
                // pair the DIFFERENT blocks the draw keeps). THE
                // STAINED HALF'S OWN TRANSLUCENT FLAGS the dyed cloths
                // the per-texel blend serves (the #pane side refs the
                // stained glass masters' 0.4 body band carry, the
                // force_translucent: true refs the render-layer
                // signal speaks), the plain pane and the bars the
                // cutout lane's binary alphas wear.
                if crate::world::is_iron_bars_class(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE CONNECTION BITS the mesh-time law computes —
                    // the N/E/S/W when-rows the props answer
                    let bits = crate::world::pane_connection_bits(world, wx, wy, wz);
                    let props = [
                        ("north", if bits & 0b0001 != 0 { "true" } else { "false" }),
                        ("east", if bits & 0b0010 != 0 { "true" } else { "false" }),
                        ("south", if bits & 0b0100 != 0 { "true" } else { "false" }),
                        ("west", if bits & 0b1000 != 0 { "true" } else { "false" }),
                    ];
                    // THE PER-QUAD TILE WALK — the derived quads'
                    // resolved stems the tile mapping answers (the
                    // #edge refs the kind's own row, the #pane refs
                    // the sibling glass kind's row the id-17 offset
                    // answers, the bars' own master the kind row
                    // keeps)
                    let models = pane_models(id);
                    for apply in select_parts(pane_parts(id), &props) {
                        let model_id = apply.model.strip_prefix("minecraft:").unwrap_or(&apply.model);
                        // the model's y quarter the blockstate's own
                        // y values speak (north/south the y0 rows the
                        // side/side_alt models wear, east/west the
                        // y90 turns — the steps the CW count answers)
                        let steps = ((apply.y as usize) / 90) % 4;
                        let Some(quads) = models.get(model_id) else {
                            continue;
                        };
                        for q in quads {
                            // THE ROTATED FACE DIRECTION the steps the
                            // same walk folds (the same-kind arm and
                            // the shade term the rotated direction
                            // read)
                            let rotated_dir = rotate_direction_cw(q.direction, steps);
                            // THE SAME-KIND CULL ARM (the dev-58
                            // round — javap IronBarsBlock.skipRendering
                            // pc 0-124 the 26.2 jar: the
                            // neighborState.is(this) arm pc 0-5 the
                            // same instance the shared face culls, the
                            // vertical axis arm pc 50-61 the stacked
                            // caps' shared plane the TRUE verdict
                            // wears, the mutual-connection arm pc
                            // 62-116 the connected seam faces cull —
                            // the instanceof universality the
                            // adjacent same-kind panes the
                            // ALWAYS-mutual verdict wears [the
                            // attachsTo instance arm every pane pair
                            // connects], the different-color pair the
                            // DIFFERENT blocks the draw keeps). THE
                            // BARS-TAG PAIR ARM the dev-59 round the
                            // palette ARMS — javap pc 8-47 the tag
                            // pair the TRUE verdict the crossing
                            // planes' shared faces culls (the neighbor
                            // AND the state BOTH in the BARS tag —
                            // iron_bars + the eight copper rungs the
                            // probe's tags/block/bars.json banks, the
                            // panes NEVER in): the iron|copper pair
                            // the DIFFERENT blocks the tag law culls,
                            // the copper|exposed pair the ladder
                            // neighbors the same law culls — the
                            // connected bars lattice the ONE
                            // continuous band renders, the z-fight the
                            // cull never banks.
                            {
                                let (dx, dy, dz) = match rotated_dir {
                                    mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                    mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                    mc_registry::element_quads::Direction::North => (0, 0, -1),
                                    mc_registry::element_quads::Direction::South => (0, 0, 1),
                                    mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                    mc_registry::element_quads::Direction::East => (1, 0, 0),
                                };
                                let neighbor = world.get(wx + dx, wy + dy, wz + dz);
                                if neighbor == id
                                    || (crate::world::is_bars_tag(id)
                                        && crate::world::is_bars_tag(neighbor))
                                {
                                    continue;
                                }
                            }
                            // THE ROTATED CULLFACE VERDICT — the
                            // six-direction law the fence lane wears
                            if let Some(dir) = q.cullface {
                                let rotated = rotate_direction_cw(dir, steps);
                                let (dx, dy, dz) = match rotated {
                                    mc_registry::element_quads::Direction::Down => (0, -1, 0),
                                    mc_registry::element_quads::Direction::Up => (0, 1, 0),
                                    mc_registry::element_quads::Direction::North => (0, 0, -1),
                                    mc_registry::element_quads::Direction::South => (0, 0, 1),
                                    mc_registry::element_quads::Direction::West => (-1, 0, 0),
                                    mc_registry::element_quads::Direction::East => (1, 0, 0),
                                };
                                if !is_transparent(world.get(wx + dx, wy + dy, wz + dz)) {
                                    continue;
                                }
                            }
                            // THE SHADE TERM'S OWN READ — the pane
                            // templates' shade defaults the direction
                            // table answers (the ambientocclusion:
                            // false the AO the quads never cast, the
                            // shade:true the faces the world-facing
                            // term keeps)
                            let face_shade = if q.shade {
                                match rotated_dir {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            let mut pts = [[0f32; 3]; 4];
                            let mut colors = [[0f32; 3]; 4];
                            let mut uvs = [[0f32; 2]; 4];
                            for (i, v) in q.vertices.iter().enumerate() {
                                // THE STEPS' OWN TURN (the blockstate y
                                // values the CW walk folds — the fire
                                // lane's own quarter walk the same map
                                // wears)
                                let (vx, vz) = match steps {
                                    1 => (1.0 - v[2], v[0]),
                                    2 => (1.0 - v[0], 1.0 - v[2]),
                                    3 => (v[2], 1.0 - v[0]),
                                    _ => (v[0], v[2]),
                                };
                                pts[i] = [
                                    wx as f32 + vx,
                                    wy as f32 + v[1],
                                    wz as f32 + vz,
                                ];
                                uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                                let lm = lightmap_combine(
                                    cell_sky,
                                    cell_block,
                                    sky_factor,
                                    sky_tint,
                                    ambient,
                                    block_tint,
                                    night_vision,
                                    world.night_vision_factor,
                                    world.darkness_scale,
                                );
                                colors[i] = [
                                    lm[0] * face_shade,
                                    lm[1] * face_shade,
                                    lm[2] * face_shade,
                                ];
                            }
                            // THE PER-QUAD TEXTURE VERDICT — the
                            // resolved stem the tile mapping answers
                            // (the #edge refs the kind's own
                            // pane-top/iron_bars master the FACE_TILES
                            // row banks, the #pane refs the SIBLING
                            // glass kind's row the id-17 offset
                            // answers — the pane kind its glass block
                            // the SAME dye the −17 seat wears)
                            let tex_stem = q
                                .texture
                                .trim_start_matches("minecraft:")
                                .trim_start_matches("block/");
                            let pane_tile = if tex_stem == "glass" {
                                atlas::FACE_TILES[crate::world::GLASS as usize][2]
                            } else if tex_stem.ends_with("_stained_glass") {
                                atlas::FACE_TILES[(id - 17) as usize][2]
                            } else if tex_stem == "iron_bars" {
                                atlas::FACE_TILES[crate::world::IRON_BARS as usize][2]
                            } else {
                                // the #edge refs (glass_pane_top /
                                // {color}_stained_glass_pane_top) the
                                // kind's own row banks
                                atlas::FACE_TILES[id as usize][2]
                            };
                            out.push(Quad {
                                pts,
                                colors,
                                uvs,
                                tile: pane_tile,
                                // THE STAINED HALF'S OWN TRANSLUCENT
                                // LANE (the 16 dyed cloths the
                                // per-texel blend serves — the #pane
                                // side refs the stained masters' 0.4
                                // body band carry, the edge tiles'
                                // 0.90 band the same law wears), the
                                // plain pane and the bars the OPAQUE
                                // pass the binary cutouts wear
                                translucent: crate::world::is_stained_glass_pane(id),
                                per_texel_alpha: crate::world::is_stained_glass_pane(id),
                            });
                        }
                    }
                    continue;
                }
                // THE CHAIN FAMILY'S OWN ELEMENT LANE (the dev-60
                // round — the hanging link the element walk finally
                // renders): javap ChainBlock the 26.2 jar the probe
                // scripts/jar_work/chain_688.py banks — the
                // RotatedPillarBlock super the AXIS vocabulary the
                // variant byte wears, the blockstates the NINE the
                // variants carry (the axis=y entry the IDENTITY model
                // wears, the axis=z the x=90 turn banks, the axis=x
                // the x=90 + y=90 pair — the probe's walk speaks, the
                // waxed twins the BASE models the references re-wear),
                // the template_chain parent the TWO zero-width planes
                // the ±45° y-axis element rotations speak (the
                // Rodrigues walk the derive fold answers, shade: false
                // every face keeps — the chain's own light the cell
                // samples, the AO the element never casts). THE FOLD
                // ORDER THE GEOMETRY DEMANDS: the x=90 turn the
                // vertical shaft the horizontal rung maps FIRST (the
                // (x, y, z) -> (x, 1-z, y) quarter the y-span the
                // z-span answers), the y-steps the CW walk the
                // z-rung the x-rung turns SECOND — the axis=x entry's
                // x=90+y=90 pair the along-x chain banks, the axis=z
                // the x=90 alone the along-z chain wears. The
                // lightmap the cell's own answer the pane lane's own
                // setup serves, the per-quad tile the resolved stem
                // the kind's own master banks (the waxed twins the
                // BASE masters re-wear — the texture-sharing law the
                // vanilla waxed look keeps), the OPAQUE flags the
                // binary cutout rides (the {0: 200, 255: 56} alpha
                // every link wears).
                if crate::world::is_chain(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE AXIS'S OWN VARIANT (the placed byte the
                    // ROTATED-PILLAR vocabulary wears — the default 0
                    // the registerDefaultState's AXIS=Y pin keeps)
                    let axis = world.variant(wx, wy, wz);
                    let (chain_quads, bs_x_rot, bs_y_rot) = chain_model_quads(id, axis);
                    for q in chain_quads {
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE X-FOLD'S OWN QUARTER (the axis=z/x
                            // entries the x=90 turn the blockstate
                            // carries: the (y, z) -> (1-z, y) fold the
                            // vertical shaft the horizontal rung maps —
                            // the right-handed quarter the +90 the
                            // Rodrigues walk answers)
                            let (fx, fy, fz) = if bs_x_rot == 90 {
                                (v[0], 1.0 - v[2], v[1])
                            } else {
                                (v[0], v[1], v[2])
                            };
                            // THE Y-STEPS' OWN TURN (the axis=x entry
                            // the y=90 the x=90 carries: the CW walk
                            // the z-rung the x-rung turns — the pane
                            // lane's own table the folded pair wears)
                            let (vx, vz) = match (bs_y_rot as usize) / 90 % 4 {
                                1 => (1.0 - fz, fx),
                                2 => (1.0 - fx, 1.0 - fz),
                                3 => (fz, 1.0 - fx),
                                _ => (fx, fz),
                            };
                            pts[i] = [
                                wx as f32 + vx,
                                wy as f32 + fy,
                                wz as f32 + vz,
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            // THE SHADE TERM'S OWN READ — the chain
                            // elements' shade:false the 1.0 keeps (the
                            // template's own law the flat link wears)
                            let face_shade = 1.0f32;
                            colors[i] = [
                                lm[0] * face_shade,
                                lm[1] * face_shade,
                                lm[2] * face_shade,
                            ];
                        }
                        // THE PER-QUAD TILE VERDICT — the resolved
                        // stem the kind's own master answers (the
                        // #texture refs the base names the waxed
                        // twins' own rows re-wear — the texture-sharing
                        // law the vanilla waxed look keeps)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let chain_tile = if tex_stem == "iron_chain" {
                            atlas::FACE_TILES[crate::world::IRON_CHAIN as usize][2]
                        } else {
                            atlas::FACE_TILES[id as usize][2]
                        };
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile: chain_tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE LANTERN'S OWN ELEMENT LANE (the dev-61 round —
                // javap LanternBlock the 26.2 jar the probe
                // scripts/jar_work/lantern_689.py banks): the
                // blockstate the hanging pair carries (hanging=false
                // the block/{name} standing model banks, hanging=true
                // the block/{name}_hanging the ceiling model wears,
                // the waxed twins the BASE models the references
                // re-wear), the template pair the body [5,0,5]->
                // [11,7,11] + top [6,7,6]->[10,9,10] boxes the SHAPE
                // pair mirrors + the two hook planes the ±45° y-axis
                // rotations speak (shade: false every hook face
                // keeps), the derive walk the element quads bakes.
                // The lightmap the cell's own answer the chain lane's
                // own setup serves, the per-quad tile the resolved
                // stem the kind's own master banks (the waxed twins
                // the BASE masters re-wear — the texture-sharing law
                // the vanilla waxed look keeps), the OPAQUE flags the
                // binary cutout rides (the {0: 450, 255: 318} alpha
                // every lantern wears), the DOWN CULLFACE the body's
                // bottom face carries the below cell the cull verdict
                // reads (the candle lane's own law the ground seam
                // keeps).
                if crate::world::is_lantern(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE HANGING BYTE'S OWN VARIANT (the placed byte
                    // the HANGING vocabulary wears — the default 0 the
                    // registerDefaultState's HANGING=false pin keeps)
                    let hanging = world.variant(wx, wy, wz);
                    let lantern_quads = lantern_model_quads(id, hanging);
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the candle path's own verdict)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        crate::colors::palette_color(id, 2)
                            .or_else(|| lut.and_then(|l| l.color(id, 2)))
                            .unwrap_or_else(|| default_color(id, 2))
                    } else {
                        [1.0; 3]
                    };
                    // THE DOWN CULL'S OWN READ — the below cell the
                    // cullface verdict samples (the vanilla cullface
                    // law: the neighbor's face the solid render
                    // answers)
                    let below = world.get(wx, wy - 1, wz);
                    for q in lantern_quads {
                        if q.cullface == Some(mc_registry::element_quads::Direction::Down)
                            && !is_transparent(below)
                        {
                            continue; // the body's bottom the solid ground never shows
                        }
                        // THE SHADE TERM'S OWN READ — the body
                        // elements' shade:true the vanilla tesselation's
                        // direction table answers (the FACE_SHADE
                        // constants the cube path wears; the hooks'
                        // shade:false the flat light keeps — the
                        // template's own law the rotated plane wears)
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            // the texel-space uv the /16 fold normalizes
                            // (the atlas tile the top 16x16 frame the
                            // first-frame read banks — the model UVs
                            // the v<=15 band the frame never leaves)
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                base[0] * lm[0] * face_shade,
                                base[1] * lm[1] * face_shade,
                                base[2] * lm[2] * face_shade,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE CHEST'S OWN ELEMENT LANE (the dev-64 round — the
                // BE-rendered family the blockstate's dummy model never
                // carries: the HAND-BUILT boxes the SHAPES law's own px
                // vocabulary wears — the SINGLE the [1,0,1]->[15,14,15]
                // symmetric inset keeps, the LEFT/RIGHT halves the
                // connected side the flush edge wears [the
                // chest_connected_direction walk the connected quarter
                // answers], the per-face tile the FACE_TILES row the
                // composite arm banks [the front the NORTH slot wears,
                // the facing face the tile assignment rotates], the
                // shade term the direction table the cube path keeps,
                // the NO cullface the 1/16 inset the neighbor faces
                // behind the box never hides — the vanilla chest law
                // the isSuffocating false answer keeps). The lightmap
                // the cell's own answer the lantern lane's own setup
                // serves.
                if crate::world::is_chest(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // the variant byte the facing/type pair the walk
                    // unpacks (the default 0 the registerDefaultState's
                    // SINGLE+north pin keeps)
                    let v = world.variant(wx, wy, wz);
                    let facing = crate::world::chest_variant_facing(v);
                    let chest_type = crate::world::chest_variant_type(v);
                    // the FACE_TILES row the composite arm banks (the
                    // front the NORTH slot wears — the atlas order the
                    // [east, west, up, down, south, north] law keeps),
                    // the FACING the tile assignment rotates (the
                    // front tile the facing face wears, the NORTH
                    // slot the back face the side tile serves — the
                    // swap the facing byte's own quarter answers)
                    let tiles = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize]
                    } else {
                        [atlas::TILE_NONE; 6]
                    };
                    let front_face = match facing {
                        crate::world::CHEST_FACING_EAST => 0usize,
                        crate::world::CHEST_FACING_WEST => 1usize,
                        crate::world::CHEST_FACING_SOUTH => 4usize,
                        _ => 5usize,
                    };
                    let face_tile = |face: usize| -> u16 {
                        if face == front_face {
                            tiles[5]
                        } else if face == 5 && facing != crate::world::CHEST_FACING_NORTH {
                            tiles[0]
                        } else {
                            tiles[face]
                        }
                    };
                    for (pts_local, uvs, face) in chest_model_quads(facing, chest_type) {
                        // the FACE_SHADE table the cube path keeps (the
                        // direction the face's own slot answers)
                        let face_shade = match face {
                            2 => 1.0f32,
                            3 => 0.5,
                            4 | 5 => 0.8,
                            _ => 0.6,
                        };
                        let tile = face_tile(face);
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        for (i, pv) in pts_local.iter().enumerate() {
                            pts[i] = [
                                wx as f32 + pv[0],
                                wy as f32 + pv[1],
                                wz as f32 + pv[2],
                            ];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            colors[i] = [
                                lm[0] * face_shade,
                                lm[1] * face_shade,
                                lm[2] * face_shade,
                            ];
                        }
                        // the texel-space uv the /16 fold normalizes
                        // (the hand-built px space the lane's own law
                        // speaks — the lantern lane's own consumption
                        // the same fold keeps)
                        let uvs = [
                            [uvs[0][0] / 16.0, uvs[0][1] / 16.0],
                            [uvs[1][0] / 16.0, uvs[1][1] / 16.0],
                            [uvs[2][0] / 16.0, uvs[2][1] / 16.0],
                            [uvs[3][0] / 16.0, uvs[3][1] / 16.0],
                        ];
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE MOB HEADS' OWN MINI-CUBE LANE (the dev-82 round —
                // javap SkullModel.createHeadModel +
                // SkullBlockRenderer.createGroundTransformation +
                // ModelPart$Cube/<Polygon> the 26.2 jar the probe
                // scripts/skull_probe_82.py banks): the 8x8x8 head box
                // the model space speaks (addBox(-4,-8,-4, 8,8,8) — x
                // -4..4, y -8..0, z -4..4 px), the createGround-
                // Transformation's own compose the vertex walk answers
                // (translation(0.5, 0, 0.5) · rotateY(-convertToDegrees(
                // rot)) · scale(-1,-1,1) — the Y flip the model's own
                // +Y-down convention converts, the rotation the ROTATION
                // segment's 22.5-degree quarter negates, the translate
                // the cell center the box lands on), the per-face rect
                // corners the Polygon ctor assigns (NO half-texel inset
                // the 26.2 walk banks — the fconst_0 fold the decode
                // speaks), the entity sheets the ENTITY_HEAD_FACES
                // table's own crops carry. The entity law the light
                // lane keeps (entityCutoutNoCull — the uniform cell
                // light every face wears, NO face shade, NO AO the
                // entity pipeline never samples), the six faces the
                // BE pass always emits (the neighbor cull the BE layer
                // never runs).
                if crate::world::is_floor_head(id) || crate::world::is_wall_head(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE ROTATION SEGMENT'S OWN QUARTER (the
                    // convertToDegrees law the transform negates — the
                    // 22.5-degree step the ROTATION_16 property wears)
                    // THE WALL HEAD'S OWN FACING QUARTER (the dev-83
                    // round — javap SkullBlockRenderer.createWall-
                    // Transformation the 26.2 jar the probe
                    // scripts/skull_probe_83.py banks: the compose
                    // T(0.5 - stepX/4, 0.25, 0.5 - stepZ/4) ·
                    // R_y(-toYRot(opposite(facing))) · scale(-1,-1,1)
                    // the vertex walk answers — the toYRot table [north
                    // 180, east -90, south 0, west +90] the opposite
                    // fold the negation speaks [north 0°, east -90°,
                    // south -180°, west +90°], the translate the hugging
                    // offset the facing's own step wears [8 - 4·step px,
                    // the y the 4px lift the 0.25 the wall band speaks])
                    let (alpha, tx_px, ty_px, tz_px) = if crate::world::is_floor_head(id) {
                        let rot = world.variant(wx, wy, wz) & 15;
                        (
                            (-(rot as f32) * 22.5f32).to_radians(),
                            8.0f32,
                            0.0f32,
                            8.0f32,
                        )
                    } else {
                        let facing = crate::world::wall_torch_facing_of(world.variant(wx, wy, wz));
                        let (sx, _sy, sz) = crate::world::wall_torch_facing_delta(facing);
                        let theta = match facing {
                            crate::world::WALL_TORCH_FACING_EAST => -90.0f32,
                            crate::world::WALL_TORCH_FACING_SOUTH => -180.0f32,
                            crate::world::WALL_TORCH_FACING_WEST => 90.0f32,
                            _ => 0.0f32,
                        };
                        (
                            theta.to_radians(),
                            8.0 - 4.0 * sx as f32,
                            4.0f32,
                            8.0 - 4.0 * sz as f32,
                        )
                    };
                    let (sin_a, cos_a) = alpha.sin_cos();
                    let faces = atlas::ENTITY_HEAD_FACES[id as usize];
                    let lm = lightmap_combine(
                        cell_sky,
                        cell_block,
                        sky_factor,
                        sky_tint,
                        ambient,
                        block_tint,
                        night_vision,
                        world.night_vision_factor,
                        world.darkness_scale,
                    );
                    for (corners, uvs, slot) in HEAD_MODEL_FACES {
                        let tile = faces[slot];
                        if tile == atlas::TILE_NONE {
                            continue; // the unserved kinds the honest skip keeps
                        }
                        let mut pts = [[0f32; 3]; 4];
                        for (i, c) in corners.iter().enumerate() {
                            // THE FLIP'S OWN PAIR (scale(-1,-1,1) — the
                            // model's +Y-down convention the world's
                            // +Y-up speaks)
                            let (fx, fy, fz) = (-c[0], -c[1], c[2]);
                            // THE ROTATE'S OWN PAIR (rotateY(alpha) —
                            // the right-handed +Y law: x' = x cos + z
                            // sin, z' = -x sin + z cos)
                            let rx = fx * cos_a + fz * sin_a;
                            let rz = -fx * sin_a + fz * cos_a;
                            // THE TRANSLATE'S OWN PAIR (the ground law's
                            // translation(8px, 0, 8px) — the cell center
                            // the box lands on; THE WALL LAW'S OWN PAIR
                            // (the dev-83 round — the (8 - 4·step, 4,
                            // 8 - 4·step) px the createWallTransformation
                            // compose wears — the hugging side the
                            // facing's own step answers, the y the 4px
                            // lift the wall band speaks))
                            pts[i] = [
                                wx as f32 + (tx_px + rx) / 16.0,
                                wy as f32 + (ty_px + fy) / 16.0,
                                wz as f32 + (tz_px + rz) / 16.0,
                            ];
                        }
                        out.push(Quad {
                            pts,
                            colors: [lm; 4], // the entity law's uniform light
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE STAIRS' OWN ELEMENT LANE (the dev-62 round —
                // javap StairBlock the 26.2 jar the probe
                // scripts/jar_work/stairs_690.py banks): the
                // blockstate the 40-key facing/half/shape map carries
                // (the SAME rotation table every kind wears — the waxed
                // twins the BASE copper models the references re-wear),
                // the template trio the slab + step boxes speak (the
                // straight the [0,0,0]->[16,8,16] + [8,8,0]->[16,16,16]
                // pair keeps, the inner the SW-quarter arm adds, the
                // outer the SE-quarter step wears), the SHAPE the
                // [`world::stairs_shape_of`] walk the neighbors answer
                // (the vanilla's updateShape-maintained state the same
                // verdict the mesh-time walk keeps — the fence's
                // connection-bits precedent the lane wears). The
                // lightmap the cell's own answer the chain lane's own
                // setup serves, the per-quad tile the resolved stem the
                // kind's own master banks (the parent masters the
                // TILE_NAMES census the reverse index answers — the
                // sandstone walk the top/side split keeps), the shade
                // term the ROTATED direction's table answers (the x=180
                // top-half pair the Up/Down flip the turn keeps, the
                // y-steps the quarter rotation the cw walk serves — the
                // lever lane's own composition the fold wears), the
                // OPAQUE flags the binary cutout rides (the parent
                // masters' alpha every stair wears).
                if crate::world::is_stairs(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE VARIANT'S OWN READ (the facing byte + the half
                    // bit the placement walk stamps, the SHAPE the
                    // neighbor walk the same verdict the collision
                    // answers)
                    let sv = world.variant(wx, wy, wz);
                    let facing = crate::world::stairs_variant_facing(sv);
                    let half_top = crate::world::stairs_variant_half(sv);
                    let shape = crate::world::stairs_shape_of(world, wx, wy, wz, facing, half_top);
                    let (stairs_quads, bs_x_turns, bs_y_steps) =
                        stairs_model_quads(id, facing, half_top, shape);
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the lantern path's own verdict)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    for q in stairs_quads {
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE X-FOLD'S OWN HALF TURN (the top-half
                            // entries the x=180 turn the blockstate
                            // carries: the (y, z) -> (1-y, 1-z) fold the
                            // slab the upper band flips — the chain
                            // lane's own x=90 quarter the walk family
                            // keeps)
                            let (fx, fy, fz) = if bs_x_turns == 2 {
                                (v[0], 1.0 - v[1], 1.0 - v[2])
                            } else {
                                (v[0], v[1], v[2])
                            };
                            // THE Y-STEPS' OWN TURN (the facing quarter
                            // the y=90..270 walk the CW cycle speaks —
                            // the chain lane's own table the folded pair
                            // wears)
                            let (vx, vz) = match bs_y_steps % 4 {
                                1 => (1.0 - fz, fx),
                                2 => (1.0 - fx, 1.0 - fz),
                                3 => (fz, 1.0 - fx),
                                _ => (fx, fz),
                            };
                            pts[i] = [
                                wx as f32 + vx,
                                wy as f32 + fy,
                                wz as f32 + vz,
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            // THE SHADE TERM'S OWN READ — the elements'
                            // shade:true the vanilla tesselation's
                            // direction table answers (the ROTATED
                            // direction the x=180 + the y-steps the
                            // composition serves — the lever lane's own
                            // pair the fold wears)
                            let rotated_dir = rotate_direction_cw(
                                rotate_direction_x(q.direction, bs_x_turns),
                                bs_y_steps,
                            );
                            let face_shade = if q.shade {
                                match rotated_dir {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            colors[i] = [
                                lm[0] * face_shade,
                                lm[1] * face_shade,
                                lm[2] * face_shade,
                            ];
                        }
                        // THE PER-QUAD TILE VERDICT — the resolved stem
                        // the kind's own master answers (the #top/#side/
                        // #bottom refs the parent masters name — the
                        // sandstone walk the split rows keep, the
                        // unknown stem the fallback row serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let stair_tile = stairs_stem_tile(tex_stem).unwrap_or(fallback_tile);
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile: stair_tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE SLAB'S OWN ELEMENT LANE (the dev-63 round —
                // javap SlabBlock the 26.2 jar the probe
                // scripts/jar_work/slab_691.py banks): the blockstate
                // the THREE-key type map carries (type=bottom/top/
                // double — the waxed twins the BASE copper models the
                // references re-wear, the smooth_stone_slab double the
                // block/smooth_stone_slab_double special model wears),
                // the template pair the band boxes speak (block/slab
                // the [0,0,0]->[16,8,16] bottom band the down/up +
                // four side cullfaces banks, block/slab_top the
                // [0,8,0]->[16,16,16] mirrored pair wears — the uv
                // bands the v 8..16 bottom / v 0..8 top rows keep),
                // the DOUBLE the parent's full-cube model the same
                // lane renders (the parent textures the same stem
                // walk serves, the cullfaces the six directions the
                // full cube banks). The lightmap the cell's own
                // answer the stairs lane's own setup serves, the
                // per-quad tile the resolved stem the TILE_NAMES
                // census the reverse index answers (the sandstone/
                // quartz/blackstone walks the split rows keep, the
                // smooth_stone_slab the slab_side special master
                // banks), the shade term the direction table answers
                // (the slab template's shade:true elements the same
                // vanilla tesselation walk keeps), the cullfaces the
                // vanilla cullface law the neighbor's solid-render
                // verdict reads (the down face the below cell culls,
                // the up face the above answers, the four sides the
                // same law speaks), the OPAQUE flags the binary
                // cutout rides (the parent masters' alpha every slab
                // wears).
                if crate::world::is_slab(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    // THE VARIANT'S OWN READ (the type byte the
                    // placement walk stamps — the double the parent
                    // model the same lane serves)
                    let slab_type = crate::world::slab_variant_type(world.variant(wx, wy, wz));
                    let slab_quads = slab_model_quads(id, slab_type);
                    // the no-atlas boot's own default — the tile's own
                    // pixels speak, the flat palette the missing row
                    // keeps (the stairs path's own verdict)
                    let fallback_tile = if (id as usize) < atlas::N_KINDS {
                        atlas::FACE_TILES[id as usize][2]
                    } else {
                        atlas::TILE_NONE
                    };
                    // THE CULLFACE'S OWN NEIGHBOR READ (the vanilla
                    // cullface law the six directions the verdict
                    // keeps — the solid-render neighbor the face
                    // culls, the non-occluding the same face draws)
                    let cull_solid = |dir: mc_registry::element_quads::Direction| -> bool {
                        let (dx, dy, dz) = match dir {
                            mc_registry::element_quads::Direction::Down => (0, -1, 0),
                            mc_registry::element_quads::Direction::Up => (0, 1, 0),
                            mc_registry::element_quads::Direction::North => (0, 0, -1),
                            mc_registry::element_quads::Direction::South => (0, 0, 1),
                            mc_registry::element_quads::Direction::West => (-1, 0, 0),
                            mc_registry::element_quads::Direction::East => (1, 0, 0),
                        };
                        let nb = world.get(wx + dx, wy + dy, wz + dz);
                        !is_transparent(nb)
                    };
                    for q in slab_quads {
                        // THE CULLFACE'S OWN SKIP (the neighbor's
                        // solid-render verdict the face never draws)
                        if let Some(dir) = q.cullface {
                            if cull_solid(dir) {
                                continue;
                            }
                        }
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, v) in q.vertices.iter().enumerate() {
                            // THE NO-TURN'S OWN IDENTITY (the slab
                            // blockstates the x=0 y=0 rows carry — the
                            // quads the cell's own frame the identity
                            // walk serves, the y the band's own span
                            // the template pair banks)
                            pts[i] = [
                                wx as f32 + v[0],
                                wy as f32 + v[1],
                                wz as f32 + v[2],
                            ];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            let lm = lightmap_combine(
                                cell_sky,
                                cell_block,
                                sky_factor,
                                sky_tint,
                                ambient,
                                block_tint,
                                night_vision,
                                world.night_vision_factor,
                                world.darkness_scale,
                            );
                            // THE SHADE TERM'S OWN READ — the
                            // elements' shade:true the vanilla
                            // tesselation's direction table answers
                            // (the unturned quads the identity
                            // direction the same walk keeps)
                            let face_shade = if q.shade {
                                match q.direction {
                                    mc_registry::element_quads::Direction::Up => 1.0f32,
                                    mc_registry::element_quads::Direction::Down => 0.5,
                                    mc_registry::element_quads::Direction::North
                                    | mc_registry::element_quads::Direction::South => 0.8,
                                    mc_registry::element_quads::Direction::West
                                    | mc_registry::element_quads::Direction::East => 0.6,
                                }
                            } else {
                                1.0
                            };
                            colors[i] = [
                                lm[0] * face_shade,
                                lm[1] * face_shade,
                                lm[2] * face_shade,
                            ];
                        }
                        // THE PER-QUAD TILE VERDICT — the resolved stem
                        // the kind's own master answers (the #top/#side/
                        // #bottom refs the parent masters name — the
                        // sandstone/quartz/blackstone walks the split
                        // rows keep, the unknown stem the fallback row
                        // serves)
                        let tex_stem = q
                            .texture
                            .trim_start_matches("minecraft:")
                            .trim_start_matches("block/");
                        let slab_tile = slab_stem_tile(tex_stem).unwrap_or(fallback_tile);
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile: slab_tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE END PORTAL'S OWN ARM (the dev-29 round — the
                // starry volume the special pass draws: the
                // EndPortalRenderer's own cube the shader law serves,
                // the faces the transparent-neighbor gate the terrain
                // walk speaks, the EPS inset the coplanar fight the
                // BE's own polygon offset answers — the sentinel tile
                // the render pass partitions on, the fullbright the
                // render type's packedLight ignore keeps)
                if id == crate::world::END_PORTAL {
                    const PORTAL_EPS: f32 = 0.001;
                    for face in 0..6usize {
                        let (dx, dy, dz) = NEIGHBOR[face];
                        let n = world.get(wx + dx, wy + dy, wz + dz);
                        if !is_transparent(n) {
                            continue; // the terrain gate the same law keeps
                        }
                        let mut pts = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for (i, c) in FACE_CORNERS[face].iter().enumerate() {
                            pts[i] = [
                                wx as f32
                                    + (if c[0] < 0.5 { PORTAL_EPS } else { 1.0 - PORTAL_EPS }),
                                wy as f32
                                    + (if c[1] < 0.5 { PORTAL_EPS } else { 1.0 - PORTAL_EPS }),
                                wz as f32
                                    + (if c[2] < 0.5 { PORTAL_EPS } else { 1.0 - PORTAL_EPS }),
                            ];
                            uvs[i] = face_uv(face, *c);
                        }
                        out.push(Quad {
                            pts,
                            colors: [[1.0; 3]; 4], // the fullbright the BE pass keeps
                            uvs,
                            tile: TILE_END_PORTAL,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                for face in 0..6usize {
                    let (dx, dy, dz) = NEIGHBOR[face];
                    let n = world.get(wx + dx, wy + dy, wz + dz);
                    // THE GLASS FAMILY'S OWN CULL ARM (the dev-57 round —
                    // javap HalfTransparentBlock.skipRendering pc 0-17 the
                    // 26.2 jar: the neighborState.is(this) verdict true the
                    // shared face the SAME block never draws — the
                    // glass|glass pair the cull keeps, the glass|stained
                    // pair the DIFFERENT blocks the draw wears, the coplanar
                    // z-fight the two quads' same plane never speaks)
                    // THE TINTED EXTENSION (the dev-81 round — the
                    // TintedGlassBlock the super's own law inherits: javap
                    // the probe scripts/tinted_glass_probe_81.py the NO
                    // skipRendering override banks — the tinted|tinted pair
                    // the shared face the same verdict never draws, the
                    // tinted|glass pair the DIFFERENT blocks the draw
                    // wears)
                    let glass_self =
                        crate::world::is_glass(id) || crate::world::is_tinted_glass(id);
                    let draw = if water_self || lava_self {
                        n == AIR // the fluids draw only against air (surface)
                    } else if glass_self && n == id {
                        false // the same-block cull the skipRendering law banks
                    } else {
                        is_transparent(n)
                    };
                    if !draw {
                        continue;
                    }
                    // Real texture tile from the generated atlas when present;
                    // otherwise the flat palette path (avg color or defaults).
                    // THE FARMLAND'S OWN WET FACE (the blockstates/farmland.
                    // json's moisture=7 arm the tile atlas speaks — javap
                    // FarmlandBlock.isNearWater the hydration verdict the
                    // vanilla texture the moisture property picks): the top
                    // face the near-water cell the farmland_moist master
                    // wears, the dry the farmland keeps (the sides/bottom
                    // the dirt the both models name)
                    let tile = if (id as usize) < atlas::N_KINDS {
                        if id == crate::world::FARMLAND
                            && face == 2
                            && world.is_near_water(wx, wy, wz)
                        {
                            atlas::TILE_FARMLAND_MOIST
                        } else if (water_self || lava_self) && face != 2 && face != 3 {
                            // THE VANILLA SIDE-FACE LAW (javap FluidRenderer,
                            // the 26.2 jar pc 1458-1466): the horizontal
                            // sides the HORIZONTAL loop walks the FLOWING
                            // sprite ALWAYS wear — the still pick the top
                            // face's calm-pool verdict (pc 671-691, the
                            // getFlow==(0,0) the stillMaterial answers) and
                            // the bottom face's own stillMaterial walk
                            // (pc 1040-1048) keep, the side quads the flow
                            // master's top-left 16x16 quadrant the
                            // generator banks (the corner UVs pc 1515-1567
                            // the getU(0.0)..getU(0.5) x
                            // getV((1-h)*0.5)..getV(0.5) window the
                            // full-height cell the exact quadrant answers —
                            // the tile the side quad's whole UV rect maps).
                            // The draw-time remap the flow strips' own
                            // frametimes the anim_tile arms speak (water 1,
                            // lava 3 — the mcmeta's own keys).
                            if water_self {
                                atlas::WATER_FLOW_FRAME0
                            } else {
                                atlas::LAVA_FLOW_FRAME0
                            }
                        } else if crate::world::is_barrel(id) {
                            // THE BARREL'S OWN LID TRAVEL (the dev-65
                            // round — the FACING(6)+OPEN blockstate the
                            // x/y turns the model law speaks: the lid
                            // tile (the FACE_TILES row's UP entry — the
                            // cube_bottom_top's own #top authoring)
                            // MOVES to the facing direction's face
                            // index [the blockstate's x=90/x=180 turns
                            // the up face the model law rotates — the
                            // [`world::barrel_lid_face`] table the byte
                            // answers], the OPPOSITE face the BOTTOM
                            // master wears, the REST the side tile
                            // keeps; the OPEN bit the lid tile the
                            // barrel_top_open master swaps [the
                            // open=true arm's own model the atlas's
                            // named const banks]). The furnace trio's
                            // own front-travel law the family wears.
                            let v = world.variant(wx, wy, wz);
                            let facing = crate::world::barrel_variant_facing(v);
                            let open = crate::world::barrel_variant_open(v);
                            let row = atlas::FACE_TILES[id as usize];
                            let lid = if open {
                                atlas::TILE_BARREL_TOP_OPEN
                            } else {
                                row[2]
                            };
                            let lid_face = crate::world::barrel_lid_face(facing);
                            let base_face = match lid_face {
                                2 => 3,
                                3 => 2,
                                5 => 4,
                                4 => 5,
                                1 => 0,
                                _ => 1,
                            };
                            if face == lid_face {
                                lid
                            } else if face == base_face {
                                row[3]
                            } else {
                                row[0]
                            }
                        } else if crate::world::is_orientable_furnace(id) {
                            // THE ORIENTABLE TRIO'S OWN FRONT-TILE SWAP
                            // (the autopush-601 furnace arm, the dev-26
                            // round GENERALIZED to the blast furnace and
                            // the smoker — the orientable model's
                            // one textured direction the facing byte
                            // rotates): the variant byte 1..=4 the
                            // north/east/south/west order the blockstate's
                            // y-rotation keeps (world_view's
                            // orientable_variant decode, the materialize walk
                            // stamps), the front tile (the FACE_TILES
                            // row's north entry — the model's own
                            // north=#front authoring) MOVES to the facing
                            // direction's face index, the vacated north
                            // slot the side tile wears (the three side
                            // entries the same tile banks — east/west/
                            // south symmetric, only the front travels).
                            // The byte's no-opinion answer (the offline
                            // scenes, the unpropertied state) the
                            // north-facing default keeps — the blockstate's
                            // own y=0 row the kind resolution banks. The
                            // face-to-rotation law the leaf_litter path's
                            // own theta walk speaks: facing=east (y=90)
                            // the model's north face lands at +X (east).
                            let facing = world.variant(wx, wy, wz);
                            let front = atlas::FACE_TILES[id as usize][5];
                            match face {
                                0 if facing == 2 => front,  // east-facing: +X
                                4 if facing == 3 => front,  // south-facing: +Z
                                1 if facing == 4 => front,  // west-facing: -X
                                5 if facing != 2 && facing != 3 && facing != 4 => front, // north (default)
                                5 => atlas::FACE_TILES[id as usize][0], // the vacated north
                                _ => atlas::FACE_TILES[id as usize][face],
                            }
                        } else {
                            atlas::FACE_TILES[id as usize][face]
                        }
                    } else {
                        atlas::TILE_NONE
                    };
                    let base = if tile == atlas::TILE_NONE {
                        // The blockstate-driven palette FIRST (the real
                        // model -> texture chain, v0.3.77's data half), then
                        // the runtime-texture LUT, then the hand-written
                        // defaults. The palette cannot color air/water —
                        // exactly the kinds the LUT/defaults still serve.
                        crate::colors::palette_color(id, face)
                            .or_else(|| lut.and_then(|l| l.color(id, face)))
                            .unwrap_or_else(|| default_color(id, face))
                    } else {
                        // The multiplier carrier for the textured path, PLUS
                        // the synced biome tint when it differs from the
                        // baked default (the atlas pixels carry the vanilla
                        // plains constants — a matching tint is a no-op by
                        // construction, the existing scenes stay bit for
                        // bit). The tint rides the SAME vertex carrier as
                        // shade×AO×light: the vanilla vertex-color multiply
                        // (`vertexColor * texelColor`).
                        world
                            .face_tint(id, face, wx, wy, wz)
                            .map(tint::unpack)
                            .unwrap_or([1.0; 3])
                    };
                    let shade = FACE_SHADE[face];
                    // Smooth lighting: per-corner AO from the 3 diagonal
                    // neighbors in front of this face.
                    let ao = ao_levels(world, wx, wy, wz, face);
                    // Per-corner light sample — only when the world
                    // carries a light layer; `None` keeps the legacy path.
                    let clight =
                        light.map(|l| corner_light(l, world.chunks_per_side, wx, wy, wz, face));
                    // The attribute-system fold (see the flat path): the
                    // stated factor OVERRIDES the default 1.0, the day
                    // tick's curve value scales it; None = the exact f32
                    // identity. Constant per mesh — the shader's
                    // SkyFactor uniform equivalent.
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    // The shader's SkyLightColor uniform: the timeline's
                    // visual/sky_light_color track (white day, the #7a7aff
                    // moonlight blue at night); None = day = white.
                    // The shader's SkyLightColor uniform: the
                    // attribute-system fold for the COLOR attribute
                    // (javap ColorModifier.MULTIPLY_RGB ->
                    // ARGB.multiply(int, int): the -1 white default
                    // short-circuits to the other operand, else
                    // per-channel imul / idiv 255 — the probe's folded
                    // value IS an Integer the extractor unpacks). None
                    // = the -1 default's shape — the track verbatim,
                    // the exact expression this path shipped before the
                    // stated attribute; a stated base (the end #ac60cd,
                    // the nether #7a7aff, a custom server's wire)
                    // multiplies the byte-space track per channel.
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    // The shader's AmbientColor base: the dimension's
                    // visual/ambient_light_color attribute as raw bytes
                    // (overworld #0a0a0a); None = the bytecode-default
                    // black — a no-op base, the pre-ambient arithmetic.
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's BlockLightTint: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#FFD88C, the corrected blue 140).
                    let block_tint = world.block_light_tint.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    // The shader's NightVisionColor: the dimension's STATED
                    // attribute (raw bytes -> 0..1), None = the bytecode
                    // default's verdict (#999999 gray) — consumed UNDER the
                    // night-vision factor (the effect state, 0.0 here).
                    let night_vision = world.night_vision_color.map(|a| {
                        [a[0] as f32 / 255.0, a[1] as f32 / 255.0, a[2] as f32 / 255.0]
                    });
                    let order = flip_order(&ao);
                    let mut pts = [[0f32; 3]; 4];
                    let mut colors = [[0f32; 3]; 4];
                    let mut uvs = [[0f32; 2]; 4];
                    for (i, &src) in order.iter().enumerate() {
                        let c = FACE_CORNERS[face][src];
                        pts[i] = [wx as f32 + c[0], wy as f32 + c[1], wz as f32 + c[2]];
                        // The vanilla lightmap shader's combine, VERBATIM
                        // (the shared per-channel core both paths call):
                        // `SkyLightColor * get_brightness(sky) * SkyFactor
                        // + BlockLightColor * get_brightness(block) *
                        // BlockFactor` — the additive SUM, clamped at 1.0
                        // per channel (the shader's single final clamp).
                        // With one term absent it is the other term
                        // exactly; at day factor 1.0 a sky-15 corner is
                        // bit-identical (1.0 + b clamps back to 1.0, the
                        // old max's verdict). No known cell -> legacy look.
                        let lm = match clight.as_ref() {
                            Some((sky, block)) => {
                                lightmap_combine(sky[src], block[src], sky_factor, sky_tint, ambient, block_tint, night_vision, world.night_vision_factor, world.darkness_scale)
                            }
                            None => [1.0; 3], // no light layer — legacy look
                        };
                        let m = [
                            shade * AO_CURVE[ao[src] as usize] * lm[0],
                            shade * AO_CURVE[ao[src] as usize] * lm[1],
                            shade * AO_CURVE[ao[src] as usize] * lm[2],
                        ];
                        colors[i] = [base[0] * m[0], base[1] * m[1], base[2] * m[2]];
                        uvs[i] = face_uv(face, c);
                    }
                    out.push(Quad {
                        pts,
                        colors,
                        uvs,
                        tile,
                        // THE STAINED HALF'S OWN TRANSLUCENT LANE (the
                        // dev-57 round — the 26.2 models' own
                        // force_translucent: true ref the render-layer
                        // signal the ladder speaks): the 16 dyed cloths the
                        // translucent pass the per-texel alpha blend
                        // serves (the textures' {102: 191, 155: 5, 163: 60}
                        // uniform map the SRC_ALPHA / ONE_MINUS_SRC_ALPHA
                        // law composites — the honey arm's own machinery
                        // the water seed keeps), the plain glass the
                        // OPAQUE pass the binary cutout wears (the
                        // textures' {0: 191, 255: 65} map the discard law
                        // answers — the alpha-255 frame texels the depth
                        // write keeps, the alpha-0 interior the hole never
                        // occludes)
                        translucent: water_self
                            || crate::world::is_stained_glass(id)
                            || crate::world::is_tinted_glass(id),
                        per_texel_alpha: crate::world::is_stained_glass(id)
                            || crate::world::is_tinted_glass(id),
                    });
                }
            }
        }
    }
    out
}

/// THE TORCH'S OWN MODEL BAKE — the block/torch chain the registry's
/// derive walk resolves (the same lane the primed TNT render speaks:
/// the faceinfo corners, the vanilla UV rect rows, the winding
/// recalculation). The one bake the OnceLock caches — the element
/// data the generated tables bank, the failed derive the honest
/// empty slice keeps (the stick the nothing renders).
fn torch_model_quads() -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        mc_registry::element_quads::derive_model_inherited("block/torch")
            .unwrap_or_default()
    })
}

/// THE REDSTONE TORCH'S OWN MODEL BAKES (the autopush-666 round — the
/// block/redstone_torch and block/redstone_torch_off chains the
/// registry's derive walk resolves, the torch lane's own cache
/// pattern): the LIT kind walks the parent link to
/// block/template_redstone_torch's SEVEN-element geometry (the stick
/// box [7,0,7]->[9,10,9] the six-face row the cullface-down read
/// wears + the SIX tiny glow planes the tip's cross the lit tile's
/// own windows bank), the OFF kind walks the block/template_torch_
/// unlit parent the stick box ALONE keeps — the merged texture map's
/// `#torch` ref the per-quad texture name answers (the
/// block/redstone_torch and block/redstone_torch_off masters the
/// kind tiles bank — the lane reads the KIND's own tile, the quads'
/// texture name the lane never consults). The failed derive the
/// honest empty slice keeps (the stick the nothing renders).
fn redstone_torch_model_quads(lit: bool) -> &'static [mc_registry::element_quads::Quad] {
    static LIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static UNLIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = if lit { &LIT } else { &UNLIT };
    cache.get_or_init(|| {
        let model = if lit {
            "block/redstone_torch"
        } else {
            "block/redstone_torch_off"
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE REDSTONE WALL TORCH'S OWN MODEL BAKES (the autopush-667 round —
/// the block/redstone_wall_torch and block/redstone_wall_torch_off
/// chains the registry's derive walk resolves, the torch lane's own
/// cache pattern): the LIT kind walks the parent link to
/// block/template_redstone_torch_wall's SEVEN-element geometry (the
/// tilted stick box [-1,3.5,7]->[1,13.5,9] the -22.5° rotZ about
/// origin [0,3.5,8] the element-rotation lane bakes + the SIX tiny
/// glow planes the lit tile's own windows bank), the OFF kind walks
/// the block/template_torch_wall_unlit parent the stick box ALONE
/// keeps — the merged texture map's `#torch` ref the per-quad
/// texture name answers (the block/redstone_wall_torch and
/// block/redstone_wall_torch_off masters the kind tiles bank — the
/// lane reads the KIND's own tile, the quads' texture name the lane
/// never consults; the SAME 149/150 sprites the standing pair's
/// seats wear — the atlas's tile set the wall pair never grows). The
/// per-facing y-rotation the MESHER's own walk wears (the blockstate
/// y-table the 8-variant map banks). The failed derive the honest
/// empty slice keeps (the stick the nothing renders).
fn redstone_wall_torch_model_quads(lit: bool) -> &'static [mc_registry::element_quads::Quad] {
    static LIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static UNLIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = if lit { &LIT } else { &UNLIT };
    cache.get_or_init(|| {
        let model = if lit {
            "block/redstone_wall_torch"
        } else {
            "block/redstone_wall_torch_off"
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE COMPARATOR'S OWN MODEL BAKES (the autopush-124 round — the
/// four per-state chains the registry's derive walk resolves, the
/// torch lane's own cache pattern the pair of state bits wears):
/// the (subtract, powered) pair the blockstates/comparator.json's
/// four rows name — block/comparator, block/comparator_on, block/
/// comparator_subtract, block/comparator_on_subtract. The resolved
/// texture refs the per-quad verdict reads (#top the TILE_COMPARATOR
/// swap banks, #slab the smooth_stone, #unlit/#lit the torch kinds).
/// The failed derive the honest empty slice keeps.
fn comparator_model_quads(
    subtract: bool,
    powered: bool,
) -> &'static [mc_registry::element_quads::Quad] {
    static OFF: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static SUBTRACT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_SUBTRACT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = match (subtract, powered) {
        (false, false) => &OFF,
        (false, true) => &ON,
        (true, false) => &SUBTRACT,
        (true, true) => &ON_SUBTRACT,
    };
    cache.get_or_init(|| {
        let model = match (subtract, powered) {
            (false, false) => "block/comparator",
            (false, true) => "block/comparator_on",
            (true, false) => "block/comparator_subtract",
            (true, true) => "block/comparator_on_subtract",
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE REPEATER'S OWN MODEL BAKES (the autopush-126 round — the
/// sixteen per-state chains the jar's own blockstates/repeater.json
/// rows name: the (powered, delay, locked) triple the variant byte
/// picks — block/repeater_{1..4}tick the dark rows, the _on pair the
/// lit torch + the twelve halo faces the glow star carries, the
/// _locked rows the bedrock bar the freeze wears). The resolved
/// texture refs the per-quad verdict reads (#top the TILE_REPEATER
/// swap banks, #slab the smooth_stone, #unlit/#lit the torch kinds'
/// own tiles, #lock the bedrock tile). The failed derive the honest
/// empty slice keeps.
fn repeater_model_quads(
    powered: bool,
    delay: u8,
    locked: bool,
) -> &'static [mc_registry::element_quads::Quad] {
    static OFF_1: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static OFF_2: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static OFF_3: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static OFF_4: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_1: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_2: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_3: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_4: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LOCK_1: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LOCK_2: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LOCK_3: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LOCK_4: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_LOCK_1: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_LOCK_2: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_LOCK_3: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON_LOCK_4: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let step = delay.clamp(1, 4) as usize;
    let cache = match (powered, locked, step) {
        (false, false, 1) => &OFF_1,
        (false, false, 2) => &OFF_2,
        (false, false, 3) => &OFF_3,
        (false, false, _) => &OFF_4,
        (false, true, 1) => &LOCK_1,
        (false, true, 2) => &LOCK_2,
        (false, true, 3) => &LOCK_3,
        (false, true, _) => &LOCK_4,
        (true, false, 1) => &ON_1,
        (true, false, 2) => &ON_2,
        (true, false, 3) => &ON_3,
        (true, false, _) => &ON_4,
        (_, true, 1) => &ON_LOCK_1,
        (_, true, 2) => &ON_LOCK_2,
        (_, true, 3) => &ON_LOCK_3,
        (_, true, _) => &ON_LOCK_4,
    };
    cache.get_or_init(|| {
        let model = match (powered, locked, step) {
            (false, false, 1) => "block/repeater_1tick",
            (false, false, 2) => "block/repeater_2tick",
            (false, false, 3) => "block/repeater_3tick",
            (false, false, _) => "block/repeater_4tick",
            (false, true, 1) => "block/repeater_1tick_locked",
            (false, true, 2) => "block/repeater_2tick_locked",
            (false, true, 3) => "block/repeater_3tick_locked",
            (false, true, _) => "block/repeater_4tick_locked",
            (true, false, 1) => "block/repeater_1tick_on",
            (true, false, 2) => "block/repeater_2tick_on",
            (true, false, 3) => "block/repeater_3tick_on",
            (true, false, _) => "block/repeater_4tick_on",
            (_, true, 1) => "block/repeater_1tick_on_locked",
            (_, true, 2) => "block/repeater_2tick_on_locked",
            (_, true, 3) => "block/repeater_3tick_on_locked",
            (_, true, _) => "block/repeater_4tick_on_locked",
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE OBSERVER'S OWN MODEL BAKE (the autopush-127 round — the ONE
/// full-cube chain the jar's own blockstates/observer.json rows name:
/// block/observer the twelve rows' model the dark AND lit states
/// wear, the observer_on parent-override the bake never derives —
/// the parent's ONE texture swap [#bottom -> block/observer_back_on]
/// the per-quad verdict's own (powered) seat reads). The resolved
/// texture refs the per-quad verdict reads (#top the observer_top
/// tile, #side the observer_side, #front the eye, #bottom the
/// back(_on)). The failed derive the honest empty slice keeps.
fn observer_model_quads() -> &'static [mc_registry::element_quads::Quad] {
    static MODEL: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    MODEL.get_or_init(|| {
        mc_registry::element_quads::derive_model_inherited("block/observer").unwrap_or_default()
    })
}

/// THE PISTON'S OWN MODEL BAKES (the autopush-128 round — the three
/// per-state chains the jar's own blockstates/piston.json +
/// sticky_piston.json rows name: the retracted pair block/piston +
/// block/sticky_piston the template_piston full-cube parent the
/// platform's own sticky swap wears, the extended ONE chain
/// block/piston_base the piston_extended body the INSIDE face the
/// piston_inner tile answers [the blockstate's extended=true rows
/// BOTH kinds the same model speak — the sticky top the retracted
/// state alone wears]). The resolved texture refs the per-quad
/// verdict reads (#platform the piston_top(_sticky) swap, #side the
/// piston_side, #bottom the piston_bottom, #inside the
/// piston_inner). The failed derive the honest empty slice keeps.
fn piston_model_quads(sticky: bool, extended: bool) -> &'static [mc_registry::element_quads::Quad] {
    static NORMAL: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static STICKY: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static EXTENDED: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = match (sticky, extended) {
        (_, true) => &EXTENDED,
        (false, false) => &NORMAL,
        (_, false) => &STICKY,
    };
    cache.get_or_init(|| {
        let model = match (sticky, extended) {
            (_, true) => "block/piston_base",
            (false, false) => "block/piston",
            (_, false) => "block/sticky_piston",
        };
        piston_bake(model)
    })
}

/// THE PISTON BAKES' OWN RESOLVE (the derive walk the prefix the jar's
/// own rows wear — the block/piston chain's slots the "minecraft:"
/// prefix the bare path the family lanes' rows never carry: the
/// comparator/observer rows the bare stems the verdict matches, the
/// piston rows the prefixed values the derive verbatim carries — the
/// strip the per-quad verdict's bare arms demand, the consumption
/// site's own strip_prefix law the mesher's walk keeps).
fn piston_bake(model: &'static str) -> Vec<mc_registry::element_quads::Quad> {
    mc_registry::element_quads::derive_model_inherited(model)
        .unwrap_or_default()
        .into_iter()
        .map(|mut q| {
            if let Some(bare) = q.texture.strip_prefix("minecraft:") {
                q.texture = bare;
            }
            q
        })
        .collect()
}

/// THE MOVING PISTON'S OWN GENERIC BAKE (the autopush-129 round — the
/// pushed state the piston family's own bakes never cover the plain
/// full-cube walk serves: the block/cube element the jar's own six
/// faces the derive wears — the same parent chain the template_piston
/// bakes walk — the per-face tile the PUSHED kind's own FACE_TILES
/// row picks at the emission site [the texture refs the derive never
/// resolves — the cube's #down..#east the per-direction verdict
/// reads]). The failed derive the honest empty slice keeps.
fn moving_cube_model_quads() -> &'static [mc_registry::element_quads::Quad] {
    static CUBE: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    CUBE.get_or_init(|| piston_bake("block/cube"))
}

/// THE PISTON HEAD'S OWN MODEL BAKES (the autopush-128 round — the
/// blockstates/piston_head.json's type rows the pair of chains name:
/// block/piston_head the normal arm, block/piston_head_sticky the
/// sticky arm — BOTH parents block/template_piston_head the plate
/// [0,0,0]->[16,16,4] + the shaft [6,6,4]->[10,10,20] geometry the
/// overhang the base cell's own border the z=1.25 walk serves). The
/// resolved texture refs the per-quad verdict reads (#platform the
/// piston_top(_sticky), #side the piston_side, #unsticky the
/// piston_top). The failed derive the honest empty slice keeps.
fn piston_head_model_quads(sticky: bool) -> &'static [mc_registry::element_quads::Quad] {
    static NORMAL: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static STICKY: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = if sticky { &STICKY } else { &NORMAL };
    cache.get_or_init(|| {
        let model = if sticky {
            "block/piston_head_sticky"
        } else {
            "block/piston_head"
        };
        piston_bake(model)
    })
}

/// THE TRAPDOOR LADDER'S OWN MODEL BAKES (the autopush-669 round —
/// the per-stem chains the registry's derive walk resolves, the oak
/// round's own cache pattern the wood rungs wear): the CLOSED kinds
/// walk the parent links to block/{stem}_bottom's and block/{stem}_
/// top's ONE-element geometry (the bottom slab [0,0,0]->[16,3,16]
/// the HALF=bottom state wears, the top slab [0,13,0]->[16,16,16]
/// the HALF=top state keeps), the OPEN kinds walk the block/{stem}_
/// open parents the [0,0,13]->[16,16,16] north-strip panel answers —
/// the merged texture map's `#texture` ref the per-quad texture name
/// answers (the per-wood master tiles the kinds' own seats bank —
/// the lane reads the KIND's own tile, the quads' texture name the
/// lane never consults; the SAME sprite the three models of each
/// wood share). The template parents the SAME geometry both families
/// bank (the ten orientable woods the template_orientable_trapdoor_
/// * chains wear, the oak/dark_oak pair the template_trapdoor_*
/// chain keeps — the probe scripts/jar_work/trapdoor_woods_669.txt
/// the census banks; the metal rungs the SAME template_trapdoor_*
/// chain the iron/copper rows wear — the probe
/// scripts/jar_work/iron_copper_trapdoors_670.txt banks). The open models' per-facing y-rotation the
/// MESHER's own walk wears. The failed derive the honest empty slice
/// keeps (the panel the nothing renders).
fn trapdoor_model_quads(
    stem: &'static str,
    half_top: bool,
    open: bool,
) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<(&'static str, u8), Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let state = if open {
        2u8
    } else if half_top {
        1u8
    } else {
        0u8
    };
    // THE LADDER'S OWN PREBAKE (the 51 derives the ONE init serves —
    // the seventeen distinct stems' three states the {stem}_bottom/
    // {stem}_top/{stem}_open chains resolve [the twenty-one pairs the
    // waxed aliases the unwaxed stems answer, the map the dedup
    // keeps], the absent model the honest skip keeps, the map the
    // lookup's own answer banks)
    let map = QUADS.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        for (st, suffix) in [(0u8, "bottom"), (1u8, "top"), (2u8, "open")] {
            for s in crate::world::TRAPDOOR_KINDS.chunks(2).filter_map(|pair| {
                crate::world::trapdoor_stem_of(pair[0])
            }) {
                let Some(wood) = s.strip_suffix("_trapdoor") else {
                    continue;
                };
                let name = format!("block/{wood}_trapdoor_{suffix}");
                let Some(model) = lookup_model(&name) else {
                    continue;
                };
                m.entry((s, st))
                    .or_insert_with(|| {
                        mc_registry::element_quads::derive_model_inherited(model)
                            .unwrap_or_default()
                    });
            }
        }
        m
    });
    map.get(&(stem, state))
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

/// THE MODEL REF'S OWN STATIC LIFT (the per-stem names the derive
/// walk reads — the 'static borrow the table's own rows answer, the
/// absent name the None the honest keep serves).
fn lookup_model(name: &str) -> Option<&'static str> {
    mc_registry::generated::models::MODELS
        .iter()
        .find(|(id, ..)| *id == name)
        .map(|(id, ..)| *id)
}

/// THE CANDLE PAIR'S OWN MODEL BAKES — the block/candle_one_candle and
/// block/candle_one_candle_lit chains the registry's derive walk
/// resolves (the torch lane's own cache pattern): BOTH walk the parent
/// link to block/template_candle's element geometry, the merged
/// texture map's `#all` ref the per-quad texture name answers (the
/// block/candle and block/candle_lit masters the kind tiles bank —
/// the lane reads the KIND's own tile, the quads' texture name the
/// lane never consults). The failed derive the honest empty slice
/// keeps (the stick the nothing renders).
fn candle_model_quads(lit: bool) -> &'static [mc_registry::element_quads::Quad] {
    static UNLIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static LIT: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    let cache = if lit { &LIT } else { &UNLIT };
    cache.get_or_init(|| {
        let model = if lit {
            "block/candle_one_candle_lit"
        } else {
            "block/candle_one_candle"
        };
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE LEVER'S OWN MODEL BAKES (the dev-45 round — the jar's own
/// REVERSED authoring the resolution wears verbatim): the
/// blockstates/lever.json's FLOOR rows name the MODELS crosswise
/// (powered=false the "block/lever_on" row answers, powered=true the
/// "block/lever" row — the models the LOOK the stick's own tilt
/// speaks, the blockstate the verdict's own mapping keeps), the two
/// element boxes the derive walk serves (the cobblestone base plate
/// [5,-0.02,4] -> [11,2.98,12] + the lever stick [7,1,7] -> [9,11,9]
/// the ±45° rotX turn the jar's own rotation field banks — the
/// Rodrigues walk the element-rotation lane serves), the per-quad
/// texture refs the two-tile verdict the branch reads (#lever the
/// named row tile, #base the cobblestone sidecar), tinted = false
/// (no tintindex anywhere). The failed derive the honest empty slice
/// keeps (the lever the nothing renders).
fn lever_model_quads(powered: bool) -> &'static [mc_registry::element_quads::Quad] {
    static OFF: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    static ON: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    // THE JAR'S OWN REVERSED ROWS (the blockstate's verbatim mapping
    // — powered=false the "_on" model, powered=true the plain): the
    // names the LOOKS carry, the verdict the blockstate speaks.
    let model = if powered { "block/lever" } else { "block/lever_on" };
    let cache = if powered { &ON } else { &OFF };
    cache.get_or_init(|| {
        mc_registry::element_quads::derive_model_inherited(model).unwrap_or_default()
    })
}

/// THE BUTTON'S OWN MODEL BAKES (the autopush-660 round — the jar's
/// DIRECT authoring the lever's REVERSED rows never share): the
/// blockstates/{wood}_button.json's FLOOR rows name the MODELS straight
/// (powered=false the "block/{wood}_button" row answers, powered=true
/// the "block/{wood}_button_pressed" row — the verbatim rows the
/// resolution speaks), the ONE element box the template parent serves
/// (the unpressed plate [5,0,6] -> [11,2,10] the six faces the down
/// cullface carries, the pressed [5,0,6] -> [11,1.02,10] the thinner
/// plate the same six faces wear — the parent chain block/button the
/// derive walk resolves, the #texture ref the per-wood planks master
/// the merged map banks — the mesher's per-quad verdict the row's own
/// tile reads), tinted = false (no tintindex anywhere). The
/// autopush-662 round the FULL LADDER the (id, powered)-keyed map
/// banks — the fourteen base names the OFF kind's NAME_OF row answers,
/// the 28 models the derive walk bakes once (the failed derive the
/// honest empty slice keeps — the button the nothing renders).
fn button_model_quads(id: u16, powered: bool) -> &'static [mc_registry::element_quads::Quad] {
    // THE LADDER'S OWN BASE SEATS (the OFF kind every pair wears — the
    // NAME_OF row the ON kind the same name speaks, the model names
    // the {name}/{name}_pressed pair the jar's blockstates bank)
    const BASE_IDS: [u16; 14] = [
        crate::world::BUTTON,
        crate::world::OAK_BUTTON,
        crate::world::SPRUCE_BUTTON,
        crate::world::BIRCH_BUTTON,
        crate::world::JUNGLE_BUTTON,
        crate::world::ACACIA_BUTTON,
        crate::world::DARK_OAK_BUTTON,
        crate::world::MANGROVE_BUTTON,
        crate::world::CHERRY_BUTTON,
        crate::world::PALE_OAK_BUTTON,
        crate::world::BAMBOO_BUTTON,
        crate::world::CRIMSON_BUTTON,
        crate::world::WARPED_BUTTON,
        crate::world::POLISHED_BLACKSTONE_BUTTON,
    ];
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<(u16, bool), Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let map = QUADS.get_or_init(|| {
        BASE_IDS
            .iter()
            .flat_map(|&base| {
                let name = crate::world::NAME_OF[base as usize];
                [
                    (
                        (base, false),
                        mc_registry::element_quads::derive_model_inherited(&format!(
                            "block/{name}"
                        ))
                        .unwrap_or_default(),
                    ),
                    (
                        (base, true),
                        mc_registry::element_quads::derive_model_inherited(&format!(
                            "block/{name}_pressed"
                        ))
                        .unwrap_or_default(),
                    ),
                ]
            })
            .collect()
    });
    // the ON kind's NAME_OF row the same base name speaks — the lookup
    // the toggled kind's own seat the base key normalizes (the OFF/ON
    // bit the ±1 mate the ladder's own law wears)
    let base = if powered { id - 1 } else { id };
    map.get(&(base, powered))
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

/// THE PRESSURE PLATES' OWN MODEL BAKES (the dev-48 round — the jar's
/// DIRECT authoring the lever's REVERSED rows never share): the
/// blockstates/{base}_pressure_plate.json's POWERED rows name the
/// MODELS straight (powered=false the "block/{base}_pressure_plate"
/// row answers, powered=true the "block/{base}_pressure_plate_down" —
/// the verbatim rows the resolution speaks), the ONE element box the
/// template parent serves (the resting plate [1,0,1] -> [15,1,15] the
/// six faces the down cullface carries, the pressed [1,0,1] ->
/// [15,0.5,15] the thinner band the same six faces wear — the parent
/// chain block/thin_block the derive walk resolves, the #texture ref
/// the per-set-type planks/stone master the merged map banks — the
/// mesher's per-quad verdict the row's own tile reads), tinted =
/// false (no tintindex anywhere). The (id, powered)-keyed map the
/// autopush-663 round the FULL LADDER bakes — the FOURTEEN base seats
/// the TWENTY-EIGHT models once bank (the failed derive the honest
/// empty slice keeps — the plate the nothing renders); the dev-49
/// round the weighted pair-pair the SIXTEEN base seats the THIRTY-TWO
/// models the same derive walk banks (the gold/iron rungs the block/
/// light|heavy_weighted_pressure_plate{,_down} chains the generated
/// MODELS rows already speak, the #texture refs block/gold_block /
/// block/iron_block the metal masters the tile table's 144/145 seats
/// bank).
fn plate_model_quads(id: u16, powered: bool) -> &'static [mc_registry::element_quads::Quad] {
    // THE SENSOR RUNGS' OWN BASE SEATS (the OFF kind every pair wears —
    // the NAME_OF row the ON kind the same name speaks, the model names
    // the {name}/{name}_down pair the jar's blockstates bank; the
    // autopush-663 round the FULL LADDER the fourteen base seats bake —
    // the twelve ladder rungs the same derive walk wears; the dev-49
    // round the weighted pair-pair the gold/iron seats the same walk
    // banks — the ONE #texture ref the metal masters the tile table
    // banks, the per-quad verdict the kind's own FACE_TILES row reads)
    const BASE_IDS: [u16; 16] = [
        crate::world::OAK_PRESSURE_PLATE,
        crate::world::STONE_PRESSURE_PLATE,
        crate::world::SPRUCE_PRESSURE_PLATE,
        crate::world::BIRCH_PRESSURE_PLATE,
        crate::world::JUNGLE_PRESSURE_PLATE,
        crate::world::ACACIA_PRESSURE_PLATE,
        crate::world::DARK_OAK_PRESSURE_PLATE,
        crate::world::MANGROVE_PRESSURE_PLATE,
        crate::world::CHERRY_PRESSURE_PLATE,
        crate::world::PALE_OAK_PRESSURE_PLATE,
        crate::world::BAMBOO_PRESSURE_PLATE,
        crate::world::CRIMSON_PRESSURE_PLATE,
        crate::world::WARPED_PRESSURE_PLATE,
        crate::world::POLISHED_BLACKSTONE_PRESSURE_PLATE,
        crate::world::LIGHT_WEIGHTED_PRESSURE_PLATE,
        crate::world::HEAVY_WEIGHTED_PRESSURE_PLATE,
    ];
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<(u16, bool), Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let map = QUADS.get_or_init(|| {
        BASE_IDS
            .iter()
            .flat_map(|&base| {
                let name = crate::world::NAME_OF[base as usize];
                [
                    (
                        (base, false),
                        mc_registry::element_quads::derive_model_inherited(&format!(
                            "block/{name}"
                        ))
                        .unwrap_or_default(),
                    ),
                    (
                        (base, true),
                        mc_registry::element_quads::derive_model_inherited(&format!(
                            "block/{name}_down"
                        ))
                        .unwrap_or_default(),
                    ),
                ]
            })
            .collect()
    });
    // the ON kind's NAME_OF row the same base name speaks — the lookup
    // the toggled kind's own seat the base key normalizes (the OFF/ON
    // bit the ±1 mate the ladder's own law wears)
    let base = if powered { id - 1 } else { id };
    map.get(&(base, powered))
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

/// THE HONEY MODEL'S OWN BAKE — the block/honey_block chain the
/// registry's derive walk resolves (the candle lane's own cache
/// pattern): the TWO-ELEMENT model the generated elements table banks
/// — the full outer cube the #down texture carries + the 1..15 inset
/// cube the #up/#side/#down skins wear — the REVERSED vec the
/// back-to-front submit order serves (the derive walk's element order
/// the outer cube first keeps; the reversed vec the inset quads the
/// translucent depth-write the outside view's own overdraw admits —
/// the inner layer composites BEFORE the outer the closer plane
/// writes).
fn honey_model_quads() -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<Vec<mc_registry::element_quads::Quad>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        let mut quads =
            mc_registry::element_quads::derive_model_inherited("block/honey_block")
                .unwrap_or_default();
        quads.reverse();
        quads
    })
}

/// THE CAMPFIRE FAMILY'S OWN MODEL BAKES — the block/campfire_off,
/// block/campfire and block/soul_campfire chains the registry's
/// derive walk resolves (the candle lane's own cache pattern): the
/// unlit kind's five-log geometry and the lit kinds' template
/// inheritance (the lit_log face swaps + the two 45-degree rotY
/// rescale fire planes the rotation walk bakes), the per-quad
/// texture names RESOLVED (the parent-chain merge the texture walk
/// serves — the lane's per-quad tile verdict the names read). The
/// failed derive the honest empty slice keeps (the logs the nothing
/// renders).
fn campfire_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = ["block/campfire_off", "block/campfire", "block/soul_campfire"];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE PORTAL'S OWN MODEL BAKES — the block/nether_portal_ns and
/// block/nether_portal_ew chains the registry's derive walk resolves
/// (the campfire lane's own cache pattern): BOTH the standalone
/// one-element models the blockstates axis variants name (the ns the
/// axis=x row — the thin-in-Z plane [0,0,6]->[16,16,10] the
/// west/east-portal verdict wears, the ew the axis=z row — the
/// thin-in-X plane [6,0,0]->[10,16,16]), the TWO faces each element
/// carries the #portal ref (block/nether_portal the merged texture
/// map resolves — the strip's per-quad verdict the FRAME0 base
/// answers). The failed derive the honest empty slice keeps (the
/// portal the nothing renders).
fn portal_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = ["block/nether_portal_ns", "block/nether_portal_ew"];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE CANDLE CAKE PAIR'S OWN MODEL BAKES — the block/candle_cake and
/// block/candle_cake_lit chains the registry's derive walk resolves
/// (the campfire lane's own cache pattern): BOTH parents
/// block/template_cake_with_candle — the cake slab box + the candle
/// stick box + the two ±45° rotY wick planes the rotation walk bakes,
/// the per-quad texture names RESOLVED (the parent-chain merge the
/// texture walk serves — the cake masters the named consts bank, the
/// candle refs the per-quad verdict reads). The failed derive the
/// honest empty slice keeps (the cake the nothing renders).
fn cake_candle_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = ["block/candle_cake", "block/candle_cake_lit"];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE DOOR FAMILY'S OWN MODEL BAKES — the per-kind model refs the
/// generated DOOR_MODEL_BY_ROW table carries (the autopush-656 round
/// the blockstate's own hinge-LEFT facing=north row the regen
/// resolved — the WAXED ladder the UNWAXED models the jar's own rows
/// serve, the dev-43 round's NAME_OF-derived word the waxed kinds
/// the absent models never answered): each a SINGLE box element
/// [0,0,0]->[3,16,16] (the base west strip the blockstate's y=0
/// authoring banks — the panel edge the theta tables rotate), the
/// lower's faces all #bottom (the per-oxidization master the TILE
/// consts bank), the upper's all #top — the per-quad texture names
/// RESOLVED (the parent-chain merge the texture walk serves). The
/// failed derive the honest empty slice keeps (the door the nothing
/// renders).
fn door_model_quads(kind: u16) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<u16, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let map = QUADS.get_or_init(|| {
        crate::world::DOOR_KINDS
            .iter()
            .map(|&k| {
                let model = atlas::DOOR_MODEL_BY_ROW[k as usize];
                (
                    k,
                    mc_registry::element_quads::derive_model_inherited(model)
                        .unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(&kind).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE FENCE GATE'S OWN MODEL BAKES — the two kinds the template
/// chain's models resolve (the dev-53 round): the closed kind the
/// block/oak_fence_gate model wears, the open kind the
/// block/oak_fence_gate_open model keeps — each the template
/// chain's EIGHT elements speaks (the two posts + the two inner
/// verticals + the four bars — the derive walk the real geometry
/// resolves, the #texture ref block/oak_planks the ONE master the
/// per-quad verdict reads). The failed derive the honest empty
/// slice keeps (the gate the nothing renders).
fn gate_model_quads(kind: u16) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<u16, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let map = QUADS.get_or_init(|| {
        [
            (
                crate::world::OAK_FENCE_GATE,
                "block/oak_fence_gate",
            ),
            (
                crate::world::OAK_FENCE_GATE_OPEN,
                "block/oak_fence_gate_open",
            ),
            // THE LADDER'S OWN MODEL ROWS (the autopush-681 round): the
            // eleven wooden pairs the derive walk every wood serves —
            // the template_fence_gate parents the ten planks masters
            // resolve, the BAMBOO pair the template_custom_fence_gate
            // parents its OWN bamboo_fence_gate master wears (the
            // jar's own per-wood models the generated MODELS rows
            // bank — the parent chain the derive resolves).
            (
                crate::world::SPRUCE_FENCE_GATE,
                "block/spruce_fence_gate",
            ),
            (
                crate::world::SPRUCE_FENCE_GATE_OPEN,
                "block/spruce_fence_gate_open",
            ),
            (
                crate::world::BIRCH_FENCE_GATE,
                "block/birch_fence_gate",
            ),
            (
                crate::world::BIRCH_FENCE_GATE_OPEN,
                "block/birch_fence_gate_open",
            ),
            (
                crate::world::JUNGLE_FENCE_GATE,
                "block/jungle_fence_gate",
            ),
            (
                crate::world::JUNGLE_FENCE_GATE_OPEN,
                "block/jungle_fence_gate_open",
            ),
            (
                crate::world::ACACIA_FENCE_GATE,
                "block/acacia_fence_gate",
            ),
            (
                crate::world::ACACIA_FENCE_GATE_OPEN,
                "block/acacia_fence_gate_open",
            ),
            (
                crate::world::CHERRY_FENCE_GATE,
                "block/cherry_fence_gate",
            ),
            (
                crate::world::CHERRY_FENCE_GATE_OPEN,
                "block/cherry_fence_gate_open",
            ),
            (
                crate::world::DARK_OAK_FENCE_GATE,
                "block/dark_oak_fence_gate",
            ),
            (
                crate::world::DARK_OAK_FENCE_GATE_OPEN,
                "block/dark_oak_fence_gate_open",
            ),
            (
                crate::world::PALE_OAK_FENCE_GATE,
                "block/pale_oak_fence_gate",
            ),
            (
                crate::world::PALE_OAK_FENCE_GATE_OPEN,
                "block/pale_oak_fence_gate_open",
            ),
            (
                crate::world::MANGROVE_FENCE_GATE,
                "block/mangrove_fence_gate",
            ),
            (
                crate::world::MANGROVE_FENCE_GATE_OPEN,
                "block/mangrove_fence_gate_open",
            ),
            (
                crate::world::BAMBOO_FENCE_GATE,
                "block/bamboo_fence_gate",
            ),
            (
                crate::world::BAMBOO_FENCE_GATE_OPEN,
                "block/bamboo_fence_gate_open",
            ),
            (
                crate::world::CRIMSON_FENCE_GATE,
                "block/crimson_fence_gate",
            ),
            (
                crate::world::CRIMSON_FENCE_GATE_OPEN,
                "block/crimson_fence_gate_open",
            ),
            (
                crate::world::WARPED_FENCE_GATE,
                "block/warped_fence_gate",
            ),
            (
                crate::world::WARPED_FENCE_GATE_OPEN,
                "block/warped_fence_gate_open",
            ),
        ]
        .into_iter()
        .map(|(k, model)| {
            (
                k,
                mc_registry::element_quads::derive_model_inherited(model)
                    .unwrap_or_default(),
            )
        })
        .collect()
    });
    map.get(&kind).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE DOOR MASTERS' OWN TILE PAIRS (the dev-43 round, the
/// autopush-656 round the copper ladder + iron complete — the
/// seventeen textured bases' #bottom/#top masters the generated
/// TILE consts bank): the resolved texture stem the per-wood pair
/// answers (the `_bottom`/`_top` suffix the pair's own half picks;
/// the WAXED kinds the UNWAXED stems the derived quads carry — the
/// resolved chain the unwaxed rows the same pairs answer), the
/// non-door stems the None the caller's fallback keeps.
fn door_master_tile(tex_stem: &str) -> Option<u16> {
    const PAIRS: [(&str, u16, u16); 17] = [
        ("oak_door", atlas::TILE_OAK_DOOR_BOTTOM, atlas::TILE_OAK_DOOR_TOP),
        ("spruce_door", atlas::TILE_SPRUCE_DOOR_BOTTOM, atlas::TILE_SPRUCE_DOOR_TOP),
        ("birch_door", atlas::TILE_BIRCH_DOOR_BOTTOM, atlas::TILE_BIRCH_DOOR_TOP),
        ("jungle_door", atlas::TILE_JUNGLE_DOOR_BOTTOM, atlas::TILE_JUNGLE_DOOR_TOP),
        ("acacia_door", atlas::TILE_ACACIA_DOOR_BOTTOM, atlas::TILE_ACACIA_DOOR_TOP),
        (
            "dark_oak_door",
            atlas::TILE_DARK_OAK_DOOR_BOTTOM,
            atlas::TILE_DARK_OAK_DOOR_TOP,
        ),
        (
            "mangrove_door",
            atlas::TILE_MANGROVE_DOOR_BOTTOM,
            atlas::TILE_MANGROVE_DOOR_TOP,
        ),
        ("cherry_door", atlas::TILE_CHERRY_DOOR_BOTTOM, atlas::TILE_CHERRY_DOOR_TOP),
        (
            "pale_oak_door",
            atlas::TILE_PALE_OAK_DOOR_BOTTOM,
            atlas::TILE_PALE_OAK_DOOR_TOP,
        ),
        ("bamboo_door", atlas::TILE_BAMBOO_DOOR_BOTTOM, atlas::TILE_BAMBOO_DOOR_TOP),
        (
            "crimson_door",
            atlas::TILE_CRIMSON_DOOR_BOTTOM,
            atlas::TILE_CRIMSON_DOOR_TOP,
        ),
        ("warped_door", atlas::TILE_WARPED_DOOR_BOTTOM, atlas::TILE_WARPED_DOOR_TOP),
        (
            "copper_door",
            atlas::TILE_COPPER_DOOR_BOTTOM,
            atlas::TILE_COPPER_DOOR_TOP,
        ),
        (
            "exposed_copper_door",
            atlas::TILE_EXPOSED_COPPER_DOOR_BOTTOM,
            atlas::TILE_EXPOSED_COPPER_DOOR_TOP,
        ),
        (
            "weathered_copper_door",
            atlas::TILE_WEATHERED_COPPER_DOOR_BOTTOM,
            atlas::TILE_WEATHERED_COPPER_DOOR_TOP,
        ),
        (
            "oxidized_copper_door",
            atlas::TILE_OXIDIZED_COPPER_DOOR_BOTTOM,
            atlas::TILE_OXIDIZED_COPPER_DOOR_TOP,
        ),
        ("iron_door", atlas::TILE_IRON_DOOR_BOTTOM, atlas::TILE_IRON_DOOR_TOP),
    ];
    let base = tex_stem.strip_suffix("_top").unwrap_or(tex_stem);
    let base = base.strip_suffix("_bottom").unwrap_or(base);
    PAIRS
        .iter()
        .find(|(name, _, _)| *name == base)
        .map(|&(_, bottom, top)| {
            if tex_stem.ends_with("_top") {
                top
            } else {
                bottom
            }
        })
}

/// THE CW QUARTER-TURN'S OWN MAP (the blockstate y rotation's own
/// walk the door lane's theta tables fold): one 90-degree CW step
/// about the cell center the (x,z)->(-z,x) map serves — the west
/// strip under one step the north edge answers (the furnace arm's
/// own face law the same direction order keeps: y=90 the model's
/// north face at +X). The steps the caller folds (the closed/open
/// tables the blockstate's own y values bank).
fn rotate_xz_cw(dx: f32, dz: f32, steps: usize) -> (f32, f32) {
    match steps % 4 {
        0 => (dx, dz),
        1 => (-dz, dx),
        2 => (-dx, -dz),
        _ => (dz, -dx),
    }
}

/// THE X-AXIS'S OWN TURN (the blockstate x values the -90° fold
/// rotates: down->south->up->north->down the same cycle the
/// (y,z)->(z,1-y) vertex map speaks, the east/west axes the turn
/// never touches). The wall lane's single turn the floor plate the
/// wall's flush reads, the ceiling lane's double the top keeps, the
/// y-rot steps the turn's own order the JOML rotationXYZ walk banks
/// (the x arm first, the y steps after).
fn rotate_direction_x(
    dir: mc_registry::element_quads::Direction,
    turns: usize,
) -> mc_registry::element_quads::Direction {
    use mc_registry::element_quads::Direction as D;
    match turns % 4 {
        0 => dir,
        1 => match dir {
            D::Down => D::South,
            D::South => D::Up,
            D::Up => D::North,
            D::North => D::Down,
            flat => flat,
        },
        2 => match dir {
            D::Down => D::Up,
            D::Up => D::Down,
            D::North => D::South,
            D::South => D::North,
            flat => flat,
        },
        _ => match dir {
            D::Down => D::North,
            D::North => D::Up,
            D::Up => D::South,
            D::South => D::Down,
            flat => flat,
        },
    }
}

/// THE CW DIRECTION'S OWN TURN (the face directions the geometry's
/// own steps rotate: north->east->south->west->north the same CW
/// order the vertex map speaks, the up/down axes the turn never
/// touches). The cullface verdict and the shade table the rotated
/// direction both read (the world-facing term the turn's own law
/// keeps).
fn rotate_direction_cw(
    dir: mc_registry::element_quads::Direction,
    steps: usize,
) -> mc_registry::element_quads::Direction {
    use mc_registry::element_quads::Direction as D;
    match steps % 4 {
        0 => dir,
        1 => match dir {
            D::North => D::East,
            D::East => D::South,
            D::South => D::West,
            D::West => D::North,
            flat => flat,
        },
        2 => match dir {
            D::North => D::South,
            D::East => D::West,
            D::South => D::North,
            D::West => D::East,
            flat => flat,
        },
        _ => match dir {
            D::North => D::West,
            D::West => D::South,
            D::South => D::East,
            D::East => D::North,
            flat => flat,
        },
    }
}

/// THE CAKE LADDER'S OWN MODEL BAKES — the block/cake and
/// block/cake_slice1..6 chains the registry's derive walk resolves
/// (the campfire lane's own cache pattern the seven models walk):
/// each a SINGLE box element [1+2N, 0, 1]->[15, 8, 15] (the
/// lambda$static$0 shape the x-start walk the models exactly mirror),
/// the west face's #inside ref the bitten cut speaks — the per-quad
/// texture names RESOLVED (the parent-chain merge the texture walk
/// serves — the named consts the verdict reads). The failed derive
/// the honest empty slice keeps (the cake the nothing renders).
fn cake_model_quads(model_id: &str) -> &'static [mc_registry::element_quads::Quad] {
    static QUADS: std::sync::OnceLock<
        std::collections::HashMap<&'static str, Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    let models = [
        "block/cake",
        "block/cake_slice1",
        "block/cake_slice2",
        "block/cake_slice3",
        "block/cake_slice4",
        "block/cake_slice5",
        "block/cake_slice6",
    ];
    let map = QUADS.get_or_init(|| {
        models
            .into_iter()
            .map(|m| {
                (
                    m,
                    mc_registry::element_quads::derive_model_inherited(m).unwrap_or_default(),
                )
            })
            .collect()
    });
    map.get(model_id).map(|v| v.as_slice()).unwrap_or(&[])
}

/// THE FIRE MULTIPART'S OWN CACHED PARTS — the parsed blockstate
/// definition the selection rides, derived ONCE (the jar's own when
/// conditions and y rotations; `parse` returns 'static data, the
/// clone the cache keeps is the same borrows). The soul half's own
/// column carries NO when — the full fan the selection always speaks.
fn fire_parts(soul: bool) -> &'static [Part<'static>] {
    static FIRE: std::sync::OnceLock<Vec<Part<'static>>> = std::sync::OnceLock::new();
    static SOUL: std::sync::OnceLock<Vec<Part<'static>>> = std::sync::OnceLock::new();
    let cache = if soul { &SOUL } else { &FIRE };
    cache.get_or_init(|| {
        let block = if soul { "soul_fire" } else { "fire" };
        match mc_registry::blockstate_lookup::parse(block) {
            Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) => {
                parts.to_vec()
            }
            _ => Vec::new(),
        }
    })
}

/// THE FIRE MODELS' OWN CACHED BAKES — the derive walk's quads per
/// model id, derived ONCE for the whole ten-model fire and six-model
/// soul set (the floor pair, the four side templates, the up pair's
/// two alt forms; the second-frame carriers fire_1/soul_fire_1 ride
/// the same bake — the texture ref the kind's tile mapping folds).
fn fire_model_quads() -> &'static std::collections::HashMap<&'static str, Vec<DerivedQuad>> {
    static QUADS: std::sync::OnceLock<std::collections::HashMap<&'static str, Vec<DerivedQuad>>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        let models = [
            "block/fire_floor0",
            "block/fire_floor1",
            "block/fire_side0",
            "block/fire_side1",
            "block/fire_side_alt0",
            "block/fire_side_alt1",
            "block/fire_up0",
            "block/fire_up1",
            "block/fire_up_alt0",
            "block/fire_up_alt1",
            "block/soul_fire_floor0",
            "block/soul_fire_floor1",
            "block/soul_fire_side0",
            "block/soul_fire_side1",
            "block/soul_fire_side_alt0",
            "block/soul_fire_side_alt1",
        ];
        models
            .into_iter()
            .map(|m| (m, derive_model_inherited(m).unwrap_or_default()))
            .collect()
    })
}

/// THE BAMBOO MULTIPART'S OWN CACHED PARTS — the parsed blockstate
/// definition the selection rides, derived ONCE (the fire lane's own
/// cache pattern): the blockstates/bamboo.json's multipart the
/// generated BLOCKSTATES row banks (the age arms apply FOUR overlapping
/// stalk models each, the leaves arms the crossed-plane pairs — the
/// jar's own when-conditions).
fn bamboo_parts() -> &'static [Part<'static>] {
    static PARTS: std::sync::OnceLock<Vec<Part<'static>>> = std::sync::OnceLock::new();
    PARTS.get_or_init(|| {
        match mc_registry::blockstate_lookup::parse("bamboo") {
            Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) => {
                parts.to_vec()
            }
            _ => Vec::new(),
        }
    })
}

/// THE FENCE FAMILY'S OWN CACHED PARTS (the dev-55 round — the
/// parsed blockstate definition the selection rides, the fire
/// lane's own cache pattern): the blockstates/{wood}_fence.json's
/// MULTIPART the generated BLOCKSTATES row banks (the
/// UNCONDITIONAL post apply + the four side arms the
/// north/east/south/west when-rows — the jar's own when-conditions
/// the select_parts walk the fire/bamboo tests pin), the thirteen
/// kinds the contiguous run the palette tail banks.
fn fence_parts(kind: u16) -> &'static [Part<'static>] {
    static PARTS: std::sync::OnceLock<std::collections::HashMap<u16, Vec<Part<'static>>>> =
        std::sync::OnceLock::new();
    PARTS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE THIRTEEN KINDS the explicit seats the palette tail
        // banks (the constant rows the range law reads — the seat
        // ladder the enclosures' own walk keeps)
        for k in [
            crate::world::OAK_FENCE,
            crate::world::SPRUCE_FENCE,
            crate::world::BIRCH_FENCE,
            crate::world::JUNGLE_FENCE,
            crate::world::ACACIA_FENCE,
            crate::world::CHERRY_FENCE,
            crate::world::DARK_OAK_FENCE,
            crate::world::PALE_OAK_FENCE,
            crate::world::MANGROVE_FENCE,
            crate::world::BAMBOO_FENCE,
            crate::world::CRIMSON_FENCE,
            crate::world::WARPED_FENCE,
            crate::world::NETHER_BRICK_FENCE,
        ] {
            if let Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) =
                mc_registry::blockstate_lookup::parse(crate::world::NAME_OF[k as usize])
            {
                map.insert(k, parts.to_vec());
            }
        }
        map
    })
    .get(&kind)
    .map(|v| v.as_slice())
    .unwrap_or(&[])
}

/// THE FENCE MODELS' OWN CACHED BAKES — the derive walk's quads per
/// kind, derived ONCE for the thirteen-kind set (the post pair the
/// template fence_post parent resolves, the side pair the
/// fence_side parent's TWO bar elements wears; the ONE #texture
/// ref the per-wood planks master the per-quad verdict reads —
/// the FACE_TILES row the same master the tile banks).
fn fence_models(
    kind: u16,
) -> &'static (Vec<DerivedQuad>, Vec<DerivedQuad>) {
    static MODELS: std::sync::OnceLock<
        std::collections::HashMap<u16, (Vec<DerivedQuad>, Vec<DerivedQuad>)>,
    > = std::sync::OnceLock::new();
    MODELS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE THIRTEEN KINDS the explicit seats the palette tail
        // banks (the constant rows the derive walk reads — the seat
        // ladder the enclosures' own walk keeps)
        for k in [
            crate::world::OAK_FENCE,
            crate::world::SPRUCE_FENCE,
            crate::world::BIRCH_FENCE,
            crate::world::JUNGLE_FENCE,
            crate::world::ACACIA_FENCE,
            crate::world::CHERRY_FENCE,
            crate::world::DARK_OAK_FENCE,
            crate::world::PALE_OAK_FENCE,
            crate::world::MANGROVE_FENCE,
            crate::world::BAMBOO_FENCE,
            crate::world::CRIMSON_FENCE,
            crate::world::WARPED_FENCE,
            crate::world::NETHER_BRICK_FENCE,
        ] {
            let name = crate::world::NAME_OF[k as usize];
            let post = format!("block/{name}_post");
            let side = format!("block/{name}_side");
            map.insert(
                k,
                (
                    derive_model_inherited(&post).unwrap_or_default(),
                    derive_model_inherited(&side).unwrap_or_default(),
                ),
            );
        }
        map
    })
    .get(&kind)
    .unwrap_or(&EMPTY_FENCE_MODELS)
}

static EMPTY_FENCE_MODELS: (Vec<DerivedQuad>, Vec<DerivedQuad>) = (Vec::new(), Vec::new());

/// THE WALL FAMILY'S OWN CACHED PARTS (the dev-56 round — the parsed
/// blockstate definition the selection rides, the fence lane's own
/// cache pattern): the blockstates/{base}_wall.json's MULTIPART the
/// generated BLOCKSTATES row banks (the up=true CONDITIONAL post
/// apply + the four side arms the north/east/south/west LOW/TALL
/// when-rows — the jar's own when-conditions the select_parts walk
/// the fire/bamboo/fence tests pin), the 32 kinds the contiguous
/// run the palette tail banks.
fn wall_parts(kind: u16) -> &'static [Part<'static>] {
    static PARTS: std::sync::OnceLock<std::collections::HashMap<u16, Vec<Part<'static>>>> =
        std::sync::OnceLock::new();
    PARTS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE THIRTY-TWO KINDS the explicit seats the palette tail
        // banks (the seat ladder the [`crate::world::WALL_KINDS`]
        // rows wear — the fence tables' own explicit-seat law keeps)
        for k in crate::world::WALL_KINDS {
            if let Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) =
                mc_registry::blockstate_lookup::parse(crate::world::NAME_OF[k as usize])
            {
                map.insert(k, parts.to_vec());
            }
        }
        map
    })
    .get(&kind)
    .map(|v| v.as_slice())
    .unwrap_or(&[])
}

/// THE WALL MODELS' OWN CACHED BAKES — the derive walk's quads per
/// kind, derived ONCE for the 32-kind set (the post pair the
/// template wall_post parent resolves, the side pair the wall_side
/// parent's ONE bar element wears, the tall pair the wall_side_tall
/// parent's full-height bar the y16..24 extension banks; the ONE
/// #texture ref the per-wall BASE master the per-quad verdict reads —
/// the FACE_TILES row the same master the tile banks).
fn wall_models(
    kind: u16,
) -> &'static (Vec<DerivedQuad>, Vec<DerivedQuad>, Vec<DerivedQuad>) {
    static MODELS: std::sync::OnceLock<
        std::collections::HashMap<u16, (Vec<DerivedQuad>, Vec<DerivedQuad>, Vec<DerivedQuad>)>,
    > = std::sync::OnceLock::new();
    MODELS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE THIRTY-TWO KINDS the explicit seats the palette tail
        // banks (the seat ladder the [`crate::world::WALL_KINDS`]
        // rows wear — the fence tables' own explicit-seat law keeps)
        for k in crate::world::WALL_KINDS {
            let name = crate::world::NAME_OF[k as usize];
            let post = format!("block/{name}_post");
            let side = format!("block/{name}_side");
            let tall = format!("block/{name}_side_tall");
            map.insert(
                k,
                (
                    derive_model_inherited(&post).unwrap_or_default(),
                    derive_model_inherited(&side).unwrap_or_default(),
                    derive_model_inherited(&tall).unwrap_or_default(),
                ),
            );
        }
        map
    })
    .get(&kind)
    .unwrap_or(&EMPTY_WALL_MODELS)
}

static EMPTY_WALL_MODELS: (Vec<DerivedQuad>, Vec<DerivedQuad>, Vec<DerivedQuad>) =
    (Vec::new(), Vec::new(), Vec::new());

/// THE PANE FAMILY'S OWN CACHED PARTS (the dev-58 round — the parsed
/// blockstate definition the selection rides, the fence lane's own
/// cache pattern): the blockstates/{name}.json's MULTIPART the
/// generated BLOCKSTATES row banks (the UNCONDITIONAL post apply +
/// the side/noside arms the panes wear, the post_ends/post/cap/side
/// arms the iron_bars and the eight copper rungs carry — the jar's
/// own when-conditions the select_parts walk the fire/bamboo/fence/
/// wall tests pin), the 26 kinds the pane + copper-bars ladders the
/// palette tail bank.
fn pane_parts(kind: u16) -> &'static [Part<'static>] {
    static PARTS: std::sync::OnceLock<std::collections::HashMap<u16, Vec<Part<'static>>>> =
        std::sync::OnceLock::new();
    PARTS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE TWENTY-SIX KINDS the explicit seats the palette tail
        // banks (the constant rows the range law reads — the seat
        // ladders the windows' own walk keeps: the dev-58 eighteen +
        // the dev-59 eight copper rungs)
        let kinds: Vec<u16> = crate::world::PANE_KINDS
            .iter()
            .chain(crate::world::COPPER_BARS_KINDS.iter())
            .copied()
            .collect();
        for k in kinds {
            if let Some(Ok(mc_registry::blockstate_lookup::Definition::Multipart(parts))) =
                mc_registry::blockstate_lookup::parse(crate::world::NAME_OF[k as usize])
            {
                map.insert(k, parts.to_vec());
            }
        }
        map
    })
    .get(&kind)
    .map(|v| v.as_slice())
    .unwrap_or(&[])
}

/// THE PANE MODELS' OWN CACHED BAKES — the derive walk's quads per
/// kind, derived ONCE for the twenty-six-kind set (the panes the
/// post/side/side_alt/noside/noside_alt five-model set the
/// template_glass_pane_* parents resolve, the iron_bars and the
/// eight copper rungs the post_ends/post/cap/cap_alt/side/side_alt
/// six-model sets the template_bars_* parents wear — the WAXED twins
/// the BASE models the blockstate references re-wear [the
/// texture-sharing law the vanilla waxed look keeps], the derived
/// quads' texture refs the resolve_ref walk the sprite stems answers
/// — the per-quad tile verdict the lane's own stem map reads). THE
/// MODEL IDS THE KIND'S OWN BLOCKSTATE the parts walk speaks — the
/// jar-driven walk the hardcoded name lists retires (the dev-59
/// round the copper rungs the same law serves).
fn pane_models(
    kind: u16,
) -> &'static std::collections::HashMap<&'static str, Vec<DerivedQuad>> {
    static MODELS: std::sync::OnceLock<std::collections::HashMap<u16, std::collections::HashMap<&'static str, Vec<DerivedQuad>>>> =
        std::sync::OnceLock::new();
    MODELS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE TWENTY-SIX KINDS the explicit seats the palette tail
        // banks (the constant rows the derive walk reads — the seat
        // ladders the windows' own walk keeps)
        let kinds: Vec<u16> = crate::world::PANE_KINDS
            .iter()
            .chain(crate::world::COPPER_BARS_KINDS.iter())
            .copied()
            .collect();
        for k in kinds {
            // THE MODEL IDS THE KIND'S OWN BLOCKSTATE the parts walk
            // speaks: every apply's model ref the multipart carries
            // (the strip_prefix walk the minecraft: namespace sheds,
            // the dedup the order the first-wear keeps — the copper
            // rungs' cap/cap_alt pair the same model twice the
            // when-rows reference, the derive walk the ONE bake
            // serves)
            let mut ids: Vec<String> = Vec::new();
            for part in pane_parts(k) {
                let apply = &part.apply;
                for a in apply.iter() {
                    let model = a
                        .model
                        .trim_start_matches("minecraft:")
                        .to_string();
                    if !ids.contains(&model) {
                        ids.push(model);
                    }
                }
            }
            let mut kind_models = std::collections::HashMap::new();
            for id in ids {
                let quads = derive_model_inherited(&id).unwrap_or_default();
                // the 'static key the leak law carries (the model id
                // strings the ONCE the process lifetime wears — the
                // fence cache's own Box::leak precedent the same law
                // banks)
                let leaked: &'static str = Box::leak(id.into_boxed_str());
                kind_models.insert(leaked, quads);
            }
            map.insert(k, kind_models);
        }
        map
    })
    .get(&kind)
    .unwrap_or_else(|| empty_pane_models())
}

fn empty_pane_models() -> &'static std::collections::HashMap<&'static str, Vec<DerivedQuad>> {
    static EMPTY: std::sync::OnceLock<std::collections::HashMap<&'static str, Vec<DerivedQuad>>> =
        std::sync::OnceLock::new();
    EMPTY.get_or_init(std::collections::HashMap::new)
}

/// THE LANTERN MODELS' OWN CACHED BAKES (the dev-61 round — the
/// kind's own blockstate the VARIANTS walk parses, the hanging entry
/// the placed byte selects, the derive walk the element quads bakes
/// ONCE per (kind, hanging)): the jar-driven walk the hardcoded model
/// lists retires — the blockstate row's OWN variants the entry list
/// carries (the hanging=false identity + the hanging=true pair the
/// probe's blockstate walk speaks, the waxed twins the BASE models
/// the blockstate references wear), the derived quads the
/// element_quads walk bakes (the template's body + top boxes the
/// shade:true table shades, the hook planes' ±45° y rotations the
/// Rodrigues fold answers, the zero-width plane geometry the
/// chain lane's own law serves). The unknown (kind, hanging) the
/// empty slice keeps (the honest not-our-domain).

/// THE CHEST MODEL'S OWN HAND-BUILT QUADS (the dev-64 round — the
/// BE-rendered family the blockstate's dummy model never carries, the
/// SHAPES law's own px vocabulary the boxes speak): the SINGLE the
/// [1,0,1]->[15,14,15] symmetric box every facing wears, the LEFT/
/// RIGHT halves the connected side the flush edge banks (the NORTH-
/// connected base [1,0,0]->[15,15,14] the z 0..14 span keeps, the
/// EAST the x 2..16, the SOUTH the z 2..16, the WEST the x 0..14 —
/// the [`world::chest_connected_direction`] walk the quarter
/// answers). The faces the FACE_TILES order returns (the face index
/// the lane's tile + shade tables read), the vertex order the
/// vanilla faceinfo corner picks keep (the winding the raster law
/// banks), the UVs the content alignment law speaks (the vertical
/// faces the v = y_hi - y fold — the top edge the tile's row 0 the
/// lid strip samples, the bottom edge the base trim rows 14/15; the
/// u the box's own span the 1..15 content band maps; the up/down the
/// centered 1..15 both axes keep). The returned pts the 0..1 block
/// units (the px /16 fold the derived quads wear), the uvs the
/// 0..16 px space (the /16 normalize the consumption keeps).
fn chest_model_quads(facing: u8, chest_type: u8) -> Vec<([[f32; 3]; 4], [[f32; 2]; 4], usize)> {
    // THE BOX'S OWN SPANS (the px vocabulary the SHAPES law keeps)
    let (x0, x1, z0, z1, y_hi) = if chest_type == crate::world::CHEST_TYPE_SINGLE {
        (1.0f32, 15.0f32, 1.0f32, 15.0f32, 14.0f32)
    } else {
        let conn = crate::world::chest_connected_direction(facing, chest_type);
        match conn {
            crate::world::CHEST_FACING_NORTH => (1.0, 15.0, 0.0, 14.0, 15.0),
            crate::world::CHEST_FACING_EAST => (2.0, 16.0, 1.0, 15.0, 15.0),
            crate::world::CHEST_FACING_SOUTH => (1.0, 15.0, 2.0, 16.0, 15.0),
            _ => (0.0, 14.0, 1.0, 15.0, 15.0),
        }
    };
    let y0 = 0.0f32;
    let px = |x: f32, y: f32, z: f32| [x / 16.0, y / 16.0, z / 16.0];
    // the vertical faces' own uv law (the u the face's own span the
    // 1..15 content band maps — the north/east the mirrored fold, the
    // south/west the direct fold; the v the top edge the row 0 the
    // lid strip samples, the bottom edge the y_hi row the base keeps)
    let vert_uv = |x: f32, y: f32, z: f32, face: usize| -> [f32; 2] {
        let u = match face {
            5 => x + 15.0 - x1,        // north: u = x + (15 - x1)
            0 => 16.0 - z + 1.0 - z0,  // east: u = (16 - z) + (1 - z0)
            4 => x - x0 + 1.0,         // south: u = x - x0 + 1
            _ => z - z0 + 1.0,         // west: u = z - z0 + 1
        };
        [u, y_hi - y]
    };
    let mut out: Vec<([[f32; 3]; 4], [[f32; 2]; 4], usize)> = Vec::with_capacity(6);
    let mut push = |pts: [[f32; 3]; 4], uvs: [[f32; 2]; 4], face: usize| {
        out.push((pts, uvs, face));
    };
    // DOWN (face 3): corners (x0,y0,z1) (x0,y0,z0) (x1,y0,z0) (x1,y0,z1)
    push(
        [
            px(x0, y0, z1),
            px(x0, y0, z0),
            px(x1, y0, z0),
            px(x1, y0, z1),
        ],
        [
            [x0 - x0 + 1.0, z1 - z0 + 1.0],
            [x0 - x0 + 1.0, z0 - z0 + 1.0],
            [x1 - x0 + 1.0, z0 - z0 + 1.0],
            [x1 - x0 + 1.0, z1 - z0 + 1.0],
        ],
        3,
    );
    // UP (face 2): corners (x0,y1,z0) (x0,y1,z1) (x1,y1,z1) (x1,y1,z0)
    push(
        [
            px(x0, y_hi, z0),
            px(x0, y_hi, z1),
            px(x1, y_hi, z1),
            px(x1, y_hi, z0),
        ],
        [
            [x0 - x0 + 1.0, z0 - z0 + 1.0],
            [x0 - x0 + 1.0, z1 - z0 + 1.0],
            [x1 - x0 + 1.0, z1 - z0 + 1.0],
            [x1 - x0 + 1.0, z0 - z0 + 1.0],
        ],
        2,
    );
    // NORTH (face 5): corners (x1,y1,z0) (x1,y0,z0) (x0,y0,z0) (x0,y1,z0)
    push(
        [
            px(x1, y_hi, z0),
            px(x1, y0, z0),
            px(x0, y0, z0),
            px(x0, y_hi, z0),
        ],
        [
            vert_uv(x1, y_hi, z0, 5),
            vert_uv(x1, y0, z0, 5),
            vert_uv(x0, y0, z0, 5),
            vert_uv(x0, y_hi, z0, 5),
        ],
        5,
    );
    // SOUTH (face 4): corners (x0,y1,z1) (x0,y0,z1) (x1,y0,z1) (x1,y1,z1)
    push(
        [
            px(x0, y_hi, z1),
            px(x0, y0, z1),
            px(x1, y0, z1),
            px(x1, y_hi, z1),
        ],
        [
            vert_uv(x0, y_hi, z1, 4),
            vert_uv(x0, y0, z1, 4),
            vert_uv(x1, y0, z1, 4),
            vert_uv(x1, y_hi, z1, 4),
        ],
        4,
    );
    // WEST (face 1): corners (x0,y1,z0) (x0,y0,z0) (x0,y0,z1) (x0,y1,z1)
    push(
        [
            px(x0, y_hi, z0),
            px(x0, y0, z0),
            px(x0, y0, z1),
            px(x0, y_hi, z1),
        ],
        [
            vert_uv(x0, y_hi, z0, 1),
            vert_uv(x0, y0, z0, 1),
            vert_uv(x0, y0, z1, 1),
            vert_uv(x0, y_hi, z1, 1),
        ],
        1,
    );
    // EAST (face 0): corners (x1,y1,z1) (x1,y0,z1) (x1,y0,z0) (x1,y1,z0)
    push(
        [
            px(x1, y_hi, z1),
            px(x1, y0, z1),
            px(x1, y0, z0),
            px(x1, y_hi, z0),
        ],
        [
            vert_uv(x1, y_hi, z1, 0),
            vert_uv(x1, y0, z1, 0),
            vert_uv(x1, y0, z0, 0),
            vert_uv(x1, y_hi, z0, 0),
        ],
        0,
    );
    out
}

fn lantern_model_quads(kind: u16, hanging: u8) -> &'static [mc_registry::element_quads::Quad] {
    static BAKES: std::sync::OnceLock<
        std::collections::HashMap<(u16, u8), Vec<mc_registry::element_quads::Quad>>,
    > = std::sync::OnceLock::new();
    BAKES.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE TEN KINDS the explicit seats the palette tail banks
        // (the constant rows the blockstate parse reads — the seat
        // ladder the hanging pair's own walk keeps)
        for k in crate::world::LANTERN_KINDS {
            let name = crate::world::NAME_OF[k as usize];
            let Some(Ok(mc_registry::blockstate_lookup::Definition::Variants(entries))) =
                mc_registry::blockstate_lookup::parse(name)
            else {
                continue;
            };
            for (key, applies) in entries {
                // THE HANGING KEY'S OWN SEAT (the "hanging=false"/
                // "hanging=true" entries the variant byte maps: 0 the
                // standing pair keeps, 1 the hanging pair wears)
                let hanging = match key {
                    "hanging=false" => crate::world::LANTERN_STANDING,
                    "hanging=true" => crate::world::LANTERN_HANGING,
                    _ => continue,
                };
                let Some(apply) = applies.first() else {
                    continue;
                };
                let model_id = apply.model.trim_start_matches("minecraft:");
                let quads = mc_registry::element_quads::derive_model_inherited(model_id)
                    .unwrap_or_default();
                map.insert((k, hanging), quads);
            }
        }
        map
    })
    .get(&(kind, hanging))
    .map(Vec::as_slice)
    .unwrap_or(&[])
}

/// THE CHAIN MODELS' OWN CACHED BAKES (the dev-60 round — the kind's
/// own blockstate the VARIANTS walk parses, the axis entry the placed
/// byte selects, the apply's x/y turns the fold walk answers): the
/// jar-driven walk the hardcoded model lists retires — the blockstate
/// row's OWN variants the entry list carries (the axis=y identity +
/// the axis=z x=90 + the axis=x x=90+y=90 the probe's walk speaks,
/// the waxed twins the BASE models the blockstate references wear),
/// the derived quads the element walk bakes ONCE per (kind, axis)
/// (the template_chain's ±45° y-axis element rotations the Rodrigues
/// fold answers). The unknown (kind, axis) the empty slice + the
/// identity turns keep (the vertical chain the unstamped default
/// wears).
fn chain_model_quads(kind: u16, axis: u8) -> (&'static [DerivedQuad], u32, u32) {
    static BAKES: std::sync::OnceLock<
        std::collections::HashMap<(u16, u8), (Vec<DerivedQuad>, u32, u32)>,
    > = std::sync::OnceLock::new();
    BAKES.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        // THE NINE KINDS the explicit seats the palette tail banks
        // (the constant rows the blockstate parse reads — the seat
        // ladder the hanging links' own walk keeps)
        for k in crate::world::CHAIN_KINDS {
            let name = crate::world::NAME_OF[k as usize];
            let Some(Ok(mc_registry::blockstate_lookup::Definition::Variants(entries))) =
                mc_registry::blockstate_lookup::parse(name)
            else {
                continue;
            };
            for (key, applies) in entries {
                // THE AXIS KEY'S OWN SEAT (the "axis=x"/"axis=y"/
                // "axis=z" entries the variant byte maps: 0 the y key
                // the vertical keeps, 1 the x key, 2 the z key)
                let axis = match key {
                    "axis=y" => crate::world::CHAIN_AXIS_Y,
                    "axis=x" => crate::world::CHAIN_AXIS_X,
                    "axis=z" => crate::world::CHAIN_AXIS_Z,
                    _ => continue,
                };
                let Some(apply) = applies.first() else {
                    continue;
                };
                let model_id = apply.model.trim_start_matches("minecraft:");
                let quads = derive_model_inherited(model_id).unwrap_or_default();
                map.insert((k, axis), (quads, apply.x, apply.y));
            }
        }
        map
    })
    .get(&(kind, axis))
    .map(|(quads, x, y)| (quads.as_slice(), *x, *y))
    .unwrap_or((&[], 0, 0))
}

/// THE STAIRS KEY'S OWN PARSE (the blockstate's "facing=X,half=Y,
/// shape=Z" triple the variant vocabulary unpacks: the facing the
/// north/east/south/west string the byte order keeps, the half the
/// bottom/top string the bit answers, the shape the five-name
/// vocabulary the StairsShape enum banks — the unknown key the None
/// the walk skips).
fn stairs_key_parts(key: &str) -> Option<(u8, bool, u8)> {
    let mut facing = None;
    let mut half = false;
    let mut shape = crate::world::STAIRS_SHAPE_STRAIGHT;
    for part in key.split(',') {
        let (k, v) = part.split_once('=')?;
        match k {
            "facing" => {
                facing = match v {
                    "north" => Some(crate::world::STAIRS_FACING_NORTH),
                    "east" => Some(crate::world::STAIRS_FACING_EAST),
                    "south" => Some(crate::world::STAIRS_FACING_SOUTH),
                    "west" => Some(crate::world::STAIRS_FACING_WEST),
                    _ => None,
                };
            }
            "half" => half = v == "top",
            "shape" => {
                shape = match v {
                    "straight" => crate::world::STAIRS_SHAPE_STRAIGHT,
                    "outer_left" => crate::world::STAIRS_SHAPE_OUTER_LEFT,
                    "outer_right" => crate::world::STAIRS_SHAPE_OUTER_RIGHT,
                    "inner_left" => crate::world::STAIRS_SHAPE_INNER_LEFT,
                    "inner_right" => crate::world::STAIRS_SHAPE_INNER_RIGHT,
                    _ => return None,
                };
            }
            _ => {}
        }
    }
    Some((facing?, half, shape))
}

#[inline]
fn stairs_state_index(facing: u8, half_top: bool, shape: u8) -> u8 {
    facing | ((half_top as u8) << 2) | (shape << 3)
}

/// THE STAIRS MODELS' OWN CACHED BAKES (the dev-62 round — the kind's
/// own blockstate the VARIANTS walk parses, the "facing/half/shape"
/// key the placed byte + the shape walk selects, the apply's x/y
/// turns the fold walk answers): the jar-driven walk the hardcoded
/// model lists retires — the blockstate's 40 keys the SAME rotation
/// table every kind carries (the probe's identity walk: 60 kinds
/// byte-identical, the waxed twins the BASE copper models the
/// references re-wear), the derived quads the element walk bakes ONCE
/// per model id (the 192-distinct-model dedup the Arc shares), the
/// (x, y) turns the blockstate apply carries (the x the 0/180 top
/// pair, the y the quarter-turns the facing speaks). The unknown
/// (kind, state) the empty slice + the identity turns keep (the
/// unstamped default the straight north-bottom pair wears).
fn stairs_model_quads(
    kind: u16,
    facing: u8,
    half_top: bool,
    shape: u8,
) -> (&'static [DerivedQuad], usize, usize) {
    static BAKES: std::sync::OnceLock<
        std::collections::HashMap<(u16, u8), (std::sync::Arc<Vec<DerivedQuad>>, usize, usize)>,
    > = std::sync::OnceLock::new();
    let idx = stairs_state_index(facing, half_top, shape);
    BAKES.get_or_init(|| {
        // THE MODEL DEDUP (the 192 distinct models the whole family
        // banks — the waxed twins the BASE ids re-wear — the Arc the
        // 2560 entries share)
        let mut models: std::collections::HashMap<String, std::sync::Arc<Vec<DerivedQuad>>> =
            std::collections::HashMap::new();
        let mut map = std::collections::HashMap::new();
        for k in crate::world::STAIRS_KINDS {
            let name = crate::world::NAME_OF[k as usize];
            let Some(Ok(mc_registry::blockstate_lookup::Definition::Variants(entries))) =
                mc_registry::blockstate_lookup::parse(name)
            else {
                continue;
            };
            for (key, applies) in entries {
                let Some((facing, half_top, shape)) = stairs_key_parts(&key) else {
                    continue;
                };
                let Some(apply) = applies.first() else {
                    continue;
                };
                let model_id = apply.model.trim_start_matches("minecraft:").to_string();
                let quads = models
                    .entry(model_id.clone())
                    .or_insert_with(|| {
                        std::sync::Arc::new(
                            derive_model_inherited(&model_id).unwrap_or_default(),
                        )
                    })
                    .clone();
                map.insert(
                    (k, stairs_state_index(facing, half_top, shape)),
                    (quads, apply.x as usize / 90, apply.y as usize / 90),
                );
            }
        }
        map
    })
    .get(&(kind, idx))
    .map(|(quads, x, y)| (quads.as_slice(), *x, *y))
    .unwrap_or((&[], 0, 0))
}

/// THE STAIRS STEMS' OWN TILE INDEX (the parent masters the per-quad
/// texture refs name — block/oak_planks the wooden rungs, block/stone
/// the stone walk, block/cut_copper the copper ladder, the
/// sandstone/tuff/cinnabar/sulfur/resin walk the split rows keep: the
/// TILE_NAMES census the once-built reverse index answers, the
/// unknown stem the None the fallback row serves).
fn stairs_stem_tile(stem: &str) -> Option<u16> {
    static TILES: std::sync::OnceLock<std::collections::HashMap<&'static str, u16>> =
        std::sync::OnceLock::new();
    let m = TILES.get_or_init(|| {
        atlas::TILE_NAMES
            .iter()
            .enumerate()
            .map(|(i, name)| (*name, i as u16))
            .collect()
    });
    m.get(stem).copied()
}

/// THE SLAB MODELS' OWN CACHED BAKES (the dev-63 round — the kind's
/// own blockstate the VARIANTS walk parses, the "type=bottom/top/
/// double" key the placed byte selects, the derived quads the element
/// walk bakes ONCE per model id (the Arc the dedup shares), the
/// blockstates the x=0 y=0 rows the NO-turn identity carries). The
/// unknown (kind, state) the empty slice keeps (the unstamped default
/// the bottom band wears).
fn slab_model_quads(kind: u16, slab_type: u8) -> &'static [DerivedQuad] {
    static BAKES: std::sync::OnceLock<
        std::collections::HashMap<(u16, u8), std::sync::Arc<Vec<DerivedQuad>>>,
    > = std::sync::OnceLock::new();
    BAKES.get_or_init(|| {
        // THE MODEL DEDUP (the bottom/top models the bare+copper pairs
        // share, the doubles the parent's full-cube model the waxed
        // twins re-wear — the Arc the entries share)
        let mut models: std::collections::HashMap<String, std::sync::Arc<Vec<DerivedQuad>>> =
            std::collections::HashMap::new();
        let mut map = std::collections::HashMap::new();
        for k in crate::world::SLAB_KINDS {
            let name = crate::world::NAME_OF[k as usize];
            let Some(Ok(mc_registry::blockstate_lookup::Definition::Variants(entries))) =
                mc_registry::blockstate_lookup::parse(name)
            else {
                continue;
            };
            for (key, applies) in entries {
                let want = match key {
                    "type=bottom" => crate::world::SLAB_TYPE_BOTTOM,
                    "type=top" => crate::world::SLAB_TYPE_TOP,
                    "type=double" => crate::world::SLAB_TYPE_DOUBLE,
                    _ => continue,
                };
                let Some(apply) = applies.first() else {
                    continue;
                };
                let model_id = apply.model.trim_start_matches("minecraft:").to_string();
                let quads = models
                    .entry(model_id.clone())
                    .or_insert_with(|| {
                        std::sync::Arc::new(
                            derive_model_inherited(&model_id).unwrap_or_default(),
                        )
                    })
                    .clone();
                map.insert((k, want), quads);
            }
        }
        map
    })
    .get(&(kind, slab_type))
    .map(|quads| quads.as_slice())
    .unwrap_or(&[])
}

/// THE SLAB STEMS' OWN TILE INDEX (the parent masters the per-quad
/// texture refs name — block/oak_planks the wooden rungs, block/stone
/// the stone walk, block/cut_copper the copper ladder, the
/// sandstone/quartz/blackstone walks the split rows keep: the
/// TILE_NAMES census the once-built reverse index answers, the
/// unknown stem the None the fallback row serves).
fn slab_stem_tile(stem: &str) -> Option<u16> {
    static TILES: std::sync::OnceLock<std::collections::HashMap<&'static str, u16>> =
        std::sync::OnceLock::new();
    let m = TILES.get_or_init(|| {
        atlas::TILE_NAMES
            .iter()
            .enumerate()
            .map(|(i, name)| (*name, i as u16))
            .collect()
    });
    m.get(stem).copied()
}

/// THE BAMBOO MODELS' OWN CACHED BAKES — the derive walk's quads per
/// model id, derived ONCE for the ten-model set (the four thin age0
/// stalks, the four thick age1 stalks, the small and the large leaves
/// crossed-plane pairs; the elements table the generated census banks,
/// the shade the stalk elements' default TRUE carries — the leaves
/// pairs' shade:false the fire lane's own law wears).
fn bamboo_model_quads() -> &'static std::collections::HashMap<&'static str, Vec<DerivedQuad>> {
    static QUADS: std::sync::OnceLock<std::collections::HashMap<&'static str, Vec<DerivedQuad>>> =
        std::sync::OnceLock::new();
    QUADS.get_or_init(|| {
        let models = [
            "block/bamboo1_age0",
            "block/bamboo2_age0",
            "block/bamboo3_age0",
            "block/bamboo4_age0",
            "block/bamboo1_age1",
            "block/bamboo2_age1",
            "block/bamboo3_age1",
            "block/bamboo4_age1",
            "block/bamboo_small_leaves",
            "block/bamboo_large_leaves",
        ];
        models
            .into_iter()
            .map(|m| (m, derive_model_inherited(m).unwrap_or_default()))
            .collect()
    })
}

/// Mesh the whole world in parallel.
pub fn mesh_world(world: &World, lut: Option<&TextureLut>) -> Vec<Quad> {
    let side = world.chunks_per_side;
    let jobs: Vec<(usize, usize)> = (0..side).flat_map(|cz| (0..side).map(move |cx| (cx, cz))).collect();
    let parts: Vec<Vec<Quad>> = jobs
        .par_iter()
        .map(|&(cx, cz)| mesh_chunk(world, cx, cz, lut))
        .collect();
    let total: usize = parts.iter().map(|p| p.len()).sum();
    let mut all = Vec::with_capacity(total);
    for p in parts {
        all.extend(p);
    }
    all
}

/// Sanity: world center column must be inside the world.
#[test]
fn mesh_produces_faces() {
    let w = World::new(2, 42);
    let q = mesh_world(&w, None);
    assert!(q.len() > 500, "suspiciously few quads: {}", q.len());
    // every quad point must be within world bounds (plus tree overhang margin)
    let n = w.size_blocks() as f32 + 3.0;
    for quad in &q {
        for p in &quad.pts {
            assert!(p[0] >= -1.0 && p[0] <= n);
        }
    }
    // smooth lighting: open-sky faces must stay at full brightness
    for quad in &q {
        for c in &quad.colors {
            assert!(c.iter().all(|v| (0.0..=1.0).contains(v)), "color out of range: {:?}", quad.colors);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE BAND'S OWN CRACK (the dev-88 law — the vanilla destroy
    /// overlay the rail's SHAPE_FLAT speaks): the six faces the 2-px
    /// band wraps — the top plane the band's own top carries, the
    /// bottom the cell floor keeps, the sides the 2-px strip wears,
    /// the full tile uv the every-face sample keeps, the full-cube
    /// law the contrast keeps.
    #[test]
    fn the_destroy_band_wears_the_two_px_span() {
        let quads = destroy_overlay_quads_band((3, 4, 5), 777, 2.0f32 / 16.0);
        assert_eq!(quads.len(), 6);
        let lo = 4.0 - 0.0015;
        let hi = 4.0 + 2.0f32 / 16.0 + 0.0015;
        for (face, q) in quads.iter().enumerate() {
            assert_eq!(q.tile, 777, "the tile passthrough the face {} keeps", face);
            for p in q.pts.iter() {
                assert!(
                    p[1] >= lo && p[1] <= hi,
                    "the band's own y span the face {} keeps",
                    face
                );
            }
            if face == 2 {
                for p in q.pts.iter() {
                    assert!((p[1] - hi).abs() < 1e-6, "the band top plane");
                }
            }
            if face == 3 {
                for p in q.pts.iter() {
                    assert!((p[1] - lo).abs() < 1e-6, "the cell floor plane");
                }
            }
        }
        // the sides the 2-px strip wears: both the low and the high
        // edge the span speaks
        let south = &quads[4];
        assert!(
            south.pts.iter().any(|p| (p[1] - lo).abs() < 1e-6)
                && south.pts.iter().any(|p| (p[1] - hi).abs() < 1e-6),
            "the side strip's own both edges"
        );
        // the full-cube law the contrast keeps: the top plane the cell
        // ceiling names
        let full = destroy_overlay_quads((3, 4, 5), 777);
        assert_eq!(full.len(), 6);
        for p in full[2].pts.iter() {
            assert!((p[1] - 5.0015).abs() < 1e-6, "the full cube's own ceiling");
        }
    }

    /// THE ASCENT'S OWN CRACK (the dev-94 round — the vanilla
    /// SHAPE_ASCENDING box the destroy overlay the ascent bytes wear):
    /// the 8-px top the caller's shape verdict serves — the top plane
    /// the box's own 8/16 carries, the bottom the cell floor keeps,
    /// the sides the 8-px strip wears.
    #[test]
    fn the_destroy_band_wears_the_ascent_span() {
        let quads = destroy_overlay_quads_band((3, 4, 5), 777, 8.0f32 / 16.0);
        assert_eq!(quads.len(), 6);
        let lo = 4.0 - 0.0015;
        let hi = 4.0 + 0.5 + 0.0015;
        for (face, q) in quads.iter().enumerate() {
            for p in q.pts.iter() {
                assert!(
                    p[1] >= lo && p[1] <= hi,
                    "the 8-px span the face {} keeps",
                    face
                );
            }
            if face == 2 {
                for p in q.pts.iter() {
                    assert!((p[1] - hi).abs() < 1e-6, "the ascent box's top plane");
                }
            }
            if face == 3 {
                for p in q.pts.iter() {
                    assert!((p[1] - lo).abs() < 1e-6, "the cell floor plane");
                }
            }
        }
        // the sides the 8-px strip wears: both edges the span speaks
        let south = &quads[4];
        assert!(
            south.pts.iter().any(|p| (p[1] - lo).abs() < 1e-6)
                && south.pts.iter().any(|p| (p[1] - hi).abs() < 1e-6),
            "the side strip's own both edges"
        );
    }


    /// 1-chunk world, all air.
    fn empty_world() -> World {
        World {
            darkness_scale: 0.0,
            chunks_per_side: 1,
            seed: 0,
            difficulty: crate::difficulty::Difficulty::Normal,
            difficulty_locked: false,
            inhabited: None,
            // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
            // the empty store the offline walk alone fills
            fire_ticks: Vec::new(),
            fire_age: std::collections::HashMap::new(),
            portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
            container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
            has_skylight: true,                            // the portal's own axis store: empty
            campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
            torch_ticks: Vec::new(),                       // the torch's own queue: empty
            detector_ticks: Vec::new(),                    // the detector's own queue: empty
            comparator_ticks: Vec::new(),
            repeater_ticks: Vec::new(),
            observer_ticks: Vec::new(),
            piston_ticks: Vec::new(),
            moving_states: Vec::new(),
            moving_ticks: Vec::new(),
            torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
            bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
            chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            light: None,
            day_tick: None,
            biomes: None,
            variants: None,
            // None = the bytecode-default black: every test scene keeps
            // the pre-ambient arithmetic bit for bit (the ambient lift is
            // pinned in its own tests, explicitly staged)
            ambient_light: None,
            block_light_tint: None,
            sky_light_factor: None,
            // None = the -1 default's white identity: the sky term's
            // tint stays the track verbatim, the pre-attribute shape
            sky_light_color: None,
            // None = the #999999 default's verdict under the factor 0.0:
            // the night-vision term stays the ambient base bit for bit
            night_vision_color: None,
            // the effect-absent floor: no effect store rides the test
            // scenes, the golden identity holds
            night_vision_factor: 0.0,
            rain_level: 0.0,  // the air-window scene: the dry world
            thunder_level: 0.0,
        }
    }

    /// An all-unknown light layer: same shape as `materialize` produces
    /// when the model never answers — the mesher must not change a pixel.
    fn unknown_light() -> LightGrid {
        LightGrid {
            sky: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            block: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]],
        }
    }

    /// The light layer value at a cell of the single test chunk.
    fn set_light(w: &mut World, sky: Option<u8>, block: Option<u8>, x: i32, y: i32, z: i32) {
        let idx = x as usize + z as usize * CHUNK_X + y as usize * CHUNK_X * CHUNK_Z;
        w.light.as_mut().unwrap().sky[0][idx] = sky.unwrap_or(LIGHT_UNKNOWN);
        w.light.as_mut().unwrap().block[0][idx] = block.unwrap_or(LIGHT_UNKNOWN);
    }

    /// THE MOVING PISTON'S OWN OFFSET WALK (the autopush-129 round —
    /// the mid-state cell the stash's pushed state renders: the
    /// quarters-0 arm the full-cell offset the SOURCE cell wears, the
    /// quarters-2 arm the half step answers, the EXTENDING bit the
    /// slide's own direction speaks [the extend the -delta arm, the
    /// retract pull the +delta arm], the stash-less mid-state the
    /// honest absence keeps)
    #[test]
    fn the_moving_piston_the_offset_the_stash_wears() {
        let mut w = World::new(3, 11);
        for x in 0..48 {
            for z in 0..48 {
                w.set(x, 9, z, STONE);
            }
        }
        // THE SEED'S OWN HILL THE SWEEP ANSWERS (the generated terrain
        // the band's own volume the buried verdict never wears — the
        // piston test's own box the family precedent keeps; the FULL
        // 12-cell span the world's own edge the border faces answer)
        for x in 0..48 {
            for y in 10..40 {
                for z in 0..48 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // THE MID-STATE CELL (the east extend mid-slide — the stone
        // stash the pushed state carries, the quarters 0 the event
        // frame's own offset wears)
        w.set(4, 10, 4, MOVING_PISTON);
        w.set_variant(4, 10, 4, moving_variant(5, true, 0));
        w.set_moving_state(4, 10, 4, STONE, 0, crate::world::MOVING_FLAG_LAND);
        let quads = mesh_chunk(&w, 0, 0, None);
        // THE FULL-CELL OFFSET (the slide the source cell wears — the
        // east facing the -x arm speaks: the pts the 3..4 x-band the
        // strict test keeps, the y band the ground tops the strict
        // > excludes)
        let in_band = |q: &Quad| {
            q.pts
                .iter()
                .all(|p| p[1] >= 10.0 - 1e-3 && p[1] <= 11.0 + 1e-3)
                && q.pts.iter().any(|p| p[1] > 10.5)
                && q.pts.iter().any(|p| (p[0] - 3.5).abs() < 0.6)
        };
        let mine: Vec<_> = quads.iter().filter(|q| in_band(q)).collect();
        assert!(
            !mine.is_empty(),
            "the mid-state's quads the source cell wears"
        );
        for q in &mine {
            for p in q.pts {
                assert!(
                    p[0] >= 3.0 - 1e-3 && p[0] <= 4.0 + 1e-3,
                    "the quarters-0 slide the full cell back wears"
                );
            }
        }
        // THE HALF STEP (the quarters 2 the 0.5 offset answers — the
        // pts the 3.5..4.5 band the mid-slide keeps)
        w.set_variant(4, 10, 4, moving_variant(5, true, 2));
        let quads = mesh_chunk(&w, 0, 0, None);
        let mine: Vec<_> = quads.iter().filter(|q| in_band(q)).collect();
        assert!(
            !mine.is_empty(),
            "the mid-slide's quads the half step wears"
        );
        for q in &mine {
            for p in q.pts {
                assert!(
                    p[0] >= 3.5 - 1e-3 && p[0] <= 4.5 + 1e-3,
                    "the quarters-2 slide the half cell back wears"
                );
            }
        }
        // THE RETRACT PULL'S OWN DIRECTION (the EXTENDING bit CLEAR
        // the +delta arm wears — the quarters 0 the slide the FRONT
        // cell keeps: the pts the 5..6 x-band the pull wears)
        let pull_band = |q: &Quad| {
            q.pts
                .iter()
                .all(|p| p[1] >= 10.0 - 1e-3 && p[1] <= 11.0 + 1e-3)
                && q.pts.iter().any(|p| p[1] > 10.5)
                && q.pts.iter().any(|p| (p[0] - 5.5).abs() < 0.6)
        };
        w.set_variant(4, 10, 4, moving_variant(5, false, 0));
        let quads = mesh_chunk(&w, 0, 0, None);
        let mine: Vec<_> = quads.iter().filter(|q| pull_band(q)).collect();
        assert!(!mine.is_empty(), "the pull's quads the front cell wears");
        for q in &mine {
            for p in q.pts {
                assert!(
                    p[0] >= 5.0 - 1e-3 && p[0] <= 6.0 + 1e-3,
                    "the pull's slide the full cell forward wears"
                );
            }
        }
        // THE STASH-LESS MID-STATE (the finalize's take the row
        // cleared — the honest absence the nothing renders keeps)
        assert!(
            w.take_moving_state(4, 10, 4).is_some(),
            "the take the row clears"
        );
        let quads = mesh_world(&w, None);
        assert!(
            quads.iter().all(|q| !in_band(q)),
            "the stash-less cell the honest silence keeps"
        );
    }

    /// THE HEAD'S OWN RETRACT CURVE (the autopush-130 round — the
    /// HEAD_RETRACT stash row the GROWING fraction renders: the
    /// quarters-0 arm the extended position the offset-zero wears,
    /// the quarters-2 arm the half sink the base the pts answer —
    /// the min-x corner the cell's own border the sink crosses)
    #[test]
    fn the_head_retract_the_offset_the_curve_wears() {
        let mut w = World::new(3, 11);
        for x in 0..16 {
            for z in 0..16 {
                w.set(x, 9, z, STONE);
            }
        }
        for x in 0..16 {
            for y in 10..24 {
                for z in 0..16 {
                    w.set(x, y, z, AIR);
                }
            }
        }
        // THE HEAD-RETRACT CELL (the east facing — the head bake the
        // 1.25 overhang the +x arm wears)
        w.set(4, 10, 4, MOVING_PISTON);
        w.set_variant(4, 10, 4, moving_variant(5, false, 0));
        w.set_moving_state(
            4,
            10,
            4,
            PISTON_HEAD,
            piston_head_variant(5, false),
            crate::world::MOVING_FLAG_HEAD_RETRACT,
        );
        let quads = mesh_chunk(&w, 0, 0, None);
        // THE QUARTERS-0 ARM (the extended position the offset zero
        // wears — the head's own pts the 4..5.25 band the min-x >= 4
        // keeps)
        let in_band = |q: &Quad| {
            q.pts
                .iter()
                .all(|p| p[1] >= 10.0 - 1e-3 && p[1] <= 11.0 + 1e-3)
                && q.pts.iter().any(|p| p[1] > 10.5)
                && q.pts.iter().any(|p| (p[0] - 4.5).abs() < 0.6)
        };
        let mine: Vec<_> = quads.iter().filter(|q| in_band(q)).collect();
        assert!(!mine.is_empty(), "the head's quads the extended pos wears");
        // THE PLATE'S OWN FRONT (the max-x the east facing's plate
        // front wears — the vanilla shaft the BACK the 0.25 overhang
        // the base cell's border the honest keep)
        let max_x = mine
            .iter()
            .flat_map(|q| q.pts.iter())
            .map(|p| p[0])
            .fold(f32::MIN, f32::max);
        assert!(
            (max_x - 5.0).abs() < 0.05,
            "the quarters-0 head the extended pos keeps: max_x {}",
            max_x
        );
        // THE QUARTERS-2 ARM (the half sink the base the pts wear —
        // the plate front the 4.5 the GROWING fraction speaks)
        w.set_variant(4, 10, 4, moving_variant(5, false, 2));
        let quads = mesh_chunk(&w, 0, 0, None);
        let mine: Vec<_> = quads.iter().filter(|q| in_band(q)).collect();
        assert!(!mine.is_empty(), "the half sink's quads the band wears");
        let max_x = mine
            .iter()
            .flat_map(|q| q.pts.iter())
            .map(|p| p[0])
            .fold(f32::MIN, f32::max);
        assert!(
            (max_x - 4.5).abs() < 0.05,
            "the half sink the growing fraction wears: max_x {}",
            max_x
        );
    }

    /// The top-face quad of the stone block at (8,8,8): shade(top)=1.0,
    /// open-sky AO=3 (1.0) — the multiplier is then light's alone.
    fn top_face(w: &World) -> crate::mesher::Quad {
        let q = mesh_chunk(w, 0, 0, None);
        q.iter()
            .find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0)
            .expect("top face quad")
            .clone()
    }

    fn vertex_multiplier(quad: &crate::mesher::Quad) -> [f32; 4] {
        // the textured path carries the pure multiplier (base = [1;3]);
        // the flat path divides the base color out
        let tile = atlas::FACE_TILES[STONE as usize][2];
        let base = if tile == atlas::TILE_NONE {
            default_color(STONE, 2)
        } else {
            [1.0; 3]
        };
        let mut out = [0f32; 4];
        for (i, c) in quad.colors.iter().enumerate() {
            out[i] = if base[0].abs() > 1e-9 { c[0] / base[0] } else { c[0] };
        }
        out
    }

    fn put(w: &mut World, x: i32, y: i32, z: i32, id: u16) {
        let lx = x.rem_euclid(CHUNK_X as i32) as usize;
        let lz = z.rem_euclid(CHUNK_Z as i32) as usize;
        w.chunks[0][lx + lz * CHUNK_X + y as usize * CHUNK_X * CHUNK_Z] = id;
    }

    /// The top-corner color of the water block at (8,8,8) — the textured
    /// path's vertex carrier (all water neighbors are air so the top face
    /// draws; the light layer is unknown/absent -> the legacy m = 1.0).
    fn water_top_corner(w: &World) -> [f32; 3] {
        mesh_chunk(w, 0, 0, None)
            .iter()
            .find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0)
            .expect("water top face quad")
            .colors[0]
    }

    #[test]
    fn pale_leaves_mesh_transparent_and_untinted() {
        // the twelfth-audit kind: pale oak leaves are non-occluding (the
        // stone face UNDER them still draws — the transparency verdict)
        // and consume NO tint slot: even a biome table carrying an
        // extreme dry-foliage opinion never folds into their carrier
        // (vanilla's BlockColors registers no source for the block).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, PALE_LEAVES);
        w.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            // grass slot carries an extreme color on every cell — if
            // pale leaves wrongly selected a slot, some face would fold
            tints: vec![[Some(0xFF0000), Some(0xFF0000), Some(0xFF0000), Some(0xFF0000)]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let quads = mesh_chunk(&w, 0, 0, None);
        // the stone top face at y 9 (under the pale leaves) DRAWS —
        // non-occluding, exactly the oak-leaves policy
        assert!(
            quads.iter().any(|quad| quad.pts[0][1] == 9.0
                && quad.pts.iter().all(|p| (p[1] - 9.0).abs() < f32::EPSILON)),
            "the stone face under pale leaves renders (non-occluding)"
        );
        // every pale-leaves face rides the UNTINTED carrier: no channel
        // is killed (a red fold would zero g/b on some corner)
        for quad in quads.iter().filter(|quad| quad.pts[0][1] == 10.0) {
            for c in quad.colors {
                assert!(c[0] > 0.0 && c[1] > 0.0 && c[2] > 0.0, "untinted: {c:?}");
            }
        }
    }

    #[test]
    fn leaf_litter_meshes_flat_ground_quads() {
        // the 13th-audit kind — the FIRST non-cube mesh path: exactly TWO
        // quads (up + down), both on the model's y=0.25/16 plane, full
        // footprint, `ambientocclusion: false` (all corners share the
        // face's single shade × light multiplier)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "up + down only, no cube faces exist");
        for q in &litter {
            assert_eq!(
                q.tile,
                atlas::FACE_TILES[LEAF_LITTER as usize][2],
                "the litter tile"
            );
            assert!(!q.translucent, "cutout, never translucent");
            for p in q.pts {
                assert!((p[1] - (9.0 + FLAT_Y)).abs() < 1e-6, "the flat plane");
            }
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&9.0), "x extent {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&9.0), "z extent {zs:?}");
            assert_eq!(q.colors[0], q.colors[1], "no AO: uniform corners");
            assert_eq!(q.colors[1], q.colors[2], "no AO: uniform corners");
            assert_eq!(q.colors[2], q.colors[3], "no AO: uniform corners");
        }
        // up quad (uv [0,0] on the first corner): shade 1.0, no light layer
        // -> legacy 1.0; down quad: the vanilla direction shade 0.5
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.colors[0], [1.0; 3], "up: shade 1.0, legacy light 1.0");
        // the down quad's first corner uv: face_uv(3, [0,0,0]) = [0, 1-0]
        let down = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 1.0])
            .expect("the down quad");
        assert_eq!(down.colors[0], [0.5; 3], "down: direction shade 0.5");
    }

    /// The cell index of (8, 9, 8) — the litter cell the flat-path tests use.
    fn litter_cell_idx() -> usize {
        8 + 8 * CHUNK_X + 9 * CHUNK_X * CHUNK_Z
    }

    #[test]
    fn leaf_litter_segment_one_meshes_the_northwest_quarter() {
        // THE 14TH AUDIT, end to end: the REAL state id (30339, north x
        // segment_amount 1) decodes to variant 1 and the mesher emits the
        // template element's exact geometry — one quarter rect, up + down
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30339);
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "one rect: up + down");
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&8.5), "x half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&8.5), "z half {zs:?}");
            assert!(!xs.contains(&9.0) && !zs.contains(&9.0), "a quarter, not full");
            assert_eq!(q.colors[0], q.colors[1], "no AO: uniform corners");
        }
        // the up quad's uv spans the texture's quarter too (u = x, v = z)
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.uvs[2], [0.5, 0.5], "uv spans the quarter");
        assert_eq!(up.colors[0], [1.0; 3], "up: shade 1.0, legacy light 1.0");
        // the down quad mirrors v WITHIN the rect: v = (z0 + z1)/16 - z
        let down = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.5])
            .expect("the down quad");
        assert_eq!(down.uvs[2], [0.5, 0.0], "v mirrored inside the rect");
        assert_eq!(down.colors[0], [0.5; 3], "down: direction shade 0.5");
    }

    #[test]
    fn leaf_litter_segments_two_and_three_stack_the_template_rects() {
        // the multipart's exact model sets: segment 2 -> {m2} (the west
        // strip), segment 3 -> {m2, m3} (strip + SE quarter, TWO rects)
        let mut w = empty_world();
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30340); // north, 2
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2, "the west strip: up + down");
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.0) && xs.contains(&8.5), "west half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&9.0), "full depth {zs:?}");
        }

        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30341); // north, 3
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 4, "strip + SE quarter: two rects, up + down");
        // the SE quarter rect exists: x [8.5, 9] x z [8.5, 9]
        assert!(litter.iter().any(|q| {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            xs.contains(&8.5) && xs.contains(&9.0) && zs.contains(&8.5) && zs.contains(&9.0)
        }));
        // and the west strip rides beside it
        assert!(litter.iter().any(|q| {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            xs.contains(&8.0) && xs.contains(&8.5) && zs.contains(&8.0) && zs.contains(&9.0)
        }));
    }

    #[test]
    fn leaf_litter_facing_east_rotates_the_footprint_with_glued_uv() {
        // facing east = the blockstate's y=90: the NW-quarter rect turns
        // about the cell center into the NE quarter; the uv stays GLUED
        // to the corners (the texture turns with the element — vanilla)
        let mut w = empty_world();
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30351); // east, 1
        w.variants = Some(variants);
        let quads = mesh_chunk(&w, 0, 0, None);
        let litter: Vec<_> = quads
            .iter()
            .filter(|q| q.pts[0][1] == 9.0 + FLAT_Y)
            .collect();
        assert_eq!(litter.len(), 2);
        for q in &litter {
            let xs: Vec<f32> = q.pts.iter().map(|p| p[0]).collect();
            let zs: Vec<f32> = q.pts.iter().map(|p| p[2]).collect();
            assert!(xs.contains(&8.5) && xs.contains(&9.0), "east half {xs:?}");
            assert!(zs.contains(&8.0) && zs.contains(&8.5), "north half {zs:?}");
            assert!(!xs.contains(&8.0), "the west edge vacated");
        }
        // the uv corners are the UNROTATED texture quarter, glued
        let up = litter
            .iter()
            .find(|q| q.uvs[0] == [0.0, 0.0])
            .expect("the up quad");
        assert_eq!(up.uvs[2], [0.5, 0.5]);
        // the first corner sits at the rotated position (x -> east edge)
        assert_eq!(up.pts[0][0], 9.0, "the (0,0) corner rotated to east");
        assert_eq!(up.pts[0][2], 8.0);
    }

    #[test]
    fn leaf_litter_segment_four_is_bit_identical_to_the_legacy_look() {
        // variant 0 (no opinion — the procedural scenes) and variant 13
        // (north, segment 4 = the full rect, y-rot 0) must emit THE SAME
        // bytes: the template uv arithmetic collapses to the old face_uv
        // exactly at the full rect
        let mut base = empty_world();
        put(&mut base, 8, 8, 8, DIRT);
        put(&mut base, 8, 9, 8, LEAF_LITTER);
        let legacy = mesh_chunk(&base, 0, 0, None);

        let mut w = base.clone();
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][litter_cell_idx()] = crate::world_view::leaf_litter_variant(30342); // north, 4
        w.variants = Some(variants);
        let shaped = mesh_chunk(&w, 0, 0, None);
        assert_eq!(legacy, shaped, "segment 4 = full rect = the old emission");
    }

    #[test]
    fn leaf_litter_never_occludes_or_casts_ao() {
        // the ground cover doesn't hide the dirt top face under it (the
        // is_transparent policy) and casts NO corner AO: the stone face
        // whose front plane touches the litter cell keeps full-open
        // corners — the shade-only 0.6 multiplier on every corner
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        put(&mut w, 7, 10, 8, STONE);
        let quads = mesh_chunk(&w, 0, 0, None);
        assert!(
            quads.iter().any(|q| {
                q.pts.iter().all(|p| {
                    (p[1] - 9.0).abs() < f32::EPSILON
                        && p[0] >= 8.0
                        && p[0] <= 9.0
                        && p[2] >= 8.0
                        && p[2] <= 9.0
                })
            }),
            "the dirt face under leaf litter renders (non-occluding)"
        );
        let face = quads
            .iter()
            .find(|q| {
                q.pts.iter().all(|p| {
                    (p[0] - 8.0).abs() < f32::EPSILON
                        && p[1] >= 10.0
                        && p[1] <= 11.0
                        && p[2] >= 8.0
                        && p[2] <= 9.0
                })
            })
            .expect("the stone +X face whose front plane holds the litter");
        for m in vertex_multiplier(face) {
            assert!(
                (m - 0.6).abs() < 1e-6,
                "litter casts no AO (full-open corners, shade 0.6): {m}"
            );
        }
    }

    #[test]
    fn leaf_litter_folds_the_dry_slot() {
        // the dry fold, end to end: the litter faces consume the DRY_SLOT —
        // a synced dry-foliage override rides the vertex carrier (the
        // override / baked-plains multiplier), and the baked plains sample
        // itself is a no-op (the atlas pixels already carry it)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, DIRT);
        put(&mut w, 8, 9, 8, LEAF_LITTER);
        w.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            // ONLY the dry slot speaks: the pale-garden override from the
            // 12th-audit probe; grass/foliage/water stay silent
            tints: vec![[None, None, None, Some(0xA0A69C)]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let quads = mesh_chunk(&w, 0, 0, None);
        let up = quads
            .iter()
            .find(|q| q.pts[0][1] == 9.0 + FLAT_Y && q.uvs[0] == [0.0, 0.0])
            .expect("the litter up quad");
        let want = tint::unpack(0xA0A69C);
        for c in up.colors {
            assert_eq!(c, want, "the dry override rides the carrier");
        }
        // the baked plains sample (TINTS carries it) folds to the no-op
        let baked = atlas::TINTS[LEAF_LITTER as usize][2];
        assert_eq!(baked, 0xA37546, "the generator's TINT_DRY constant");
        w.biomes.as_mut().unwrap().tints[0][3] = Some(baked);
        let quads = mesh_chunk(&w, 0, 0, None);
        let up = quads
            .iter()
            .find(|q| q.pts[0][1] == 9.0 + FLAT_Y && q.uvs[0] == [0.0, 0.0])
            .expect("the litter up quad");
        assert_eq!(up.colors[0], [1.0; 3], "the baked sample is a no-op");
    }

    #[test]
    fn biome_tint_folds_into_the_textured_carrier_only_when_it_differs() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, WATER);
        let reference = water_top_corner(&w);
        assert!(
            reference[0] > 0.0
                && (reference[0] - reference[1]).abs() < 1e-6
                && (reference[1] - reference[2]).abs() < 1e-6,
            "the untinted carrier is shade×AO×light on every channel: {:?}",
            reference
        );
        // plains: water == the baked default -> bit-identical to no layer
        let baked = atlas::TINTS[WATER as usize][0];
        let mut plains = empty_world();
        put(&mut plains, 8, 8, 8, WATER);
        plains.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            tints: vec![[None, None, Some(baked), None]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        assert_eq!(
            water_top_corner(&plains),
            reference,
            "a tint matching the baked default is a no-op, bit for bit"
        );
        // a real red-water biome: the tint rides the SAME carrier
        let mut red = empty_world();
        put(&mut red, 8, 8, 8, WATER);
        red.biomes = Some(BiomeLayer {
            ids: vec![vec![0; CHUNK_X * CHUNK_Y * CHUNK_Z]],
            tints: vec![[None, None, Some(0xFF0000), None]],
            climates: Vec::new(), // the tint law's own tests: no climate opinion
        });
        let tinted = water_top_corner(&red);
        assert_eq!(tinted[1], 0.0, "green killed by the tint");
        assert_eq!(tinted[2], 0.0, "blue killed by the tint");
        assert_eq!(tinted[0], reference[0], "red rides the SAME multiplier");
    }

    #[test]
    fn vertex_ao_truth_table() {
        assert_eq!(vertex_ao(true, true, false), 0, "two sides fully occlude");
        assert_eq!(vertex_ao(true, true, true), 0);
        assert_eq!(vertex_ao(false, false, false), 3, "open corner");
        assert_eq!(vertex_ao(true, false, false), 2);
        assert_eq!(vertex_ao(false, true, false), 2);
        assert_eq!(vertex_ao(false, false, true), 2);
        assert_eq!(vertex_ao(true, false, true), 1);
        assert_eq!(vertex_ao(false, true, true), 1);
    }

    #[test]
    fn light_brightness_matches_the_vanilla_ramp() {
        assert_eq!(light_brightness(15.0), 1.0, "open sky keeps the legacy look");
        assert_eq!(light_brightness(0.0), 0.0, "known-dark renders black");
        assert!(
            (light_brightness(8.0) - 0.222_222_22).abs() < 1e-4,
            "level 8 is about 22% — the curve's aggressive low end"
        );
        for l in 0..15u8 {
            assert!(
                light_brightness(l as f32) < light_brightness((l + 1) as f32),
                "monotonic at {l}"
            );
        }
    }

    #[test]
    fn light_layer_shades_known_cells_and_spares_unknown() {
        // stone at (8,8,8) in an all-air world; the top face cell is (8,9,8)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        let legacy = {
            let mut w2 = empty_world();
            put(&mut w2, 8, 8, 8, STONE);
            vertex_multiplier(&top_face(&w2))
        };
        // an ALL-UNKNOWN layer (materialize over a silent model) changes nothing
        w.light = Some(unknown_light());
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            legacy,
            "all-unknown light must be invisible"
        );
        // known full sky at the face cell — brightness 1.0, still the legacy look
        set_light(&mut w, Some(15), None, 8, 9, 8);
        assert_eq!(vertex_multiplier(&top_face(&w)), legacy, "full sky is the legacy look");
        // the engine says the cell is DARK (both layers known zero) -> black
        set_light(&mut w, Some(0), Some(0), 8, 9, 8);
        for m in vertex_multiplier(&top_face(&w)) {
            assert!(m.abs() < 1e-6, "known-dark cell is black: {m}");
        }
        // torch-lit: no sky opinion, block 14 -> the TINTED, FACTORED
        // block term under the clamp: BlockLightColor(14) ·
        // brightness(14) · BlockFactor — red and green saturate, blue
        // rides under (the warm verdict). v0.3.87: the term is no
        // longer the white brightness(14) — the #ffd88c mix landed.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b = light_brightness(14.0) * BLOCK_FACTOR;
        let c = block_light_color(14.0, None);
        let expect = [(b * c[0]).min(1.0), (b * c[1]).min(1.0), (b * c[2]).min(1.0)];
        assert!((expect[0] - 1.0).abs() < 1e-6, "red saturates the clamp");
        assert!(expect[2] < 1.0, "blue stays under the clamp");
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "block-only light: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn sky_factor_scales_only_the_sky_term() {
        // the stone block's top face under full sky: day is the legacy
        // look; the night factor drags the SKY term down the ramp while
        // the BLOCK term is untouched — and the two SUM, clamped at 1.0.
        // v0.3.86: the night sky term is additionally TINTED by the
        // timeline's visual/sky_light_color track (#7a7aff — the
        // moonlight blue), so the night pins are per-channel: the
        // vertex_multiplier reads the RED channel (r = g tinted, b not).
        let tint_rg = 122.0f32 / 255.0; // #7a7aff red/green, blue 1.0
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        // day: brightness(15) = 1.0 -> legacy
        assert_eq!(vertex_multiplier(&top_face(&w)), [1.0; 4], "day sky-15 is legacy");
        // night (the day timeline's midnight tick -> the 0.24 plateau +
        // the #7a7aff tint on the sky term)
        w.day_tick = Some(18_000);
        let night_sky = light_brightness(15.0) * 0.24;
        for m in vertex_multiplier(&top_face(&w)) {
            assert!(
                (m - night_sky * tint_rg).abs() < 1e-6,
                "night sky-15 scales AND tints the sky term: {m}"
            );
        }
        // the same night sky over a TORCH-lit cell (block 14): the
        // ADDITIVE combine sums the tinted sky term + the block term
        set_light(&mut w, Some(15), Some(14), 8, 9, 8);
        // v0.3.87: the block term carries BlockFactor 1.4 — the red
        // term alone is 1.0889, so the SUM saturates the clamp at 1.0
        // (a full-sky torch-lit face is fully lit, the vanilla
        // verdict); the warm tint shows on UNclamped cells instead
        // (see the_block_term_is_tinted_and_factored_not_white)
        let block_r =
            (light_brightness(14.0) * BLOCK_FACTOR * block_light_color(14.0, None)[0]).min(1.0);
        let night_torch_r = (light_brightness(15.0) * 0.24 * tint_rg + block_r).min(1.0);
        assert!(
            (night_torch_r - 1.0).abs() < 1e-6,
            "the torch sum saturates the clamp: {night_torch_r}"
        );
        for m in vertex_multiplier(&top_face(&w)) {
            assert!((m - night_torch_r).abs() < 1e-6, "night torch red saturates: {m}");
        }
        // the sum WITHOUT the clamp: day, sky 12 (brightness 0.5) over
        // block 10 (brightness 1/3 × BlockFactor 1.4 × tint-red 1.0) —
        // max(0.5, 1/3) was 0.5, the true shader sum is 0.5 + 7/15 =
        // 29/30 (the factor rides the block term since v0.3.87)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, STONE);
        w3.light = Some(unknown_light());
        set_light(&mut w3, Some(12), Some(10), 8, 9, 8);
        let expect = light_brightness(12.0)
            + light_brightness(10.0) * BLOCK_FACTOR * block_light_color(10.0, None)[0];
        assert!((expect - 29.0 / 30.0).abs() < 1e-6, "ramp sanity: {expect}");
        for m in vertex_multiplier(&top_face(&w3)) {
            assert!(
                (m - expect).abs() < 1e-6,
                "the combine is the additive sum, not the max: {m} vs {expect}"
            );
        }
        // an unknown sky layer is never scaled into an opinion: a corner
        // with only block known keeps the plain block term (no SKY
        // factor, no SKY tint — even at the night tick; the block term
        // carries its own factor 1.4 and its own tint since v0.3.87),
        // block 10 keeps every channel under the clamp
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        w2.light = Some(unknown_light());
        w2.day_tick = Some(18_000);
        set_light(&mut w2, None, Some(10), 8, 9, 8);
        let b = light_brightness(10.0) * BLOCK_FACTOR;
        let c = block_light_color(10.0, None);
        let expect = [b * c[0], b * c[1], b * c[2]];
        for corner in &top_face(&w2).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "unknown sky is not darkened, the block term is exact: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_night_sky_term_is_blue_not_grey() {
        // the full-channel witness: at the night tick the sky-lit top
        // face's multiplier is #7a7aff-tinted — red and green ride the
        // 122/255 tint, blue stays the plain factored ramp — while the
        // DAY tick of the same world is bit-identical to the scalar
        // (white) arithmetic.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        // the day product (shade * AO * ramp 1.0) — the legacy 1.0 the
        // previous test pins; derived here, not hard-coded
        w.day_tick = Some(6_000);
        let full = vertex_multiplier(&top_face(&w))[0];
        assert!((full - 1.0).abs() < 1e-6, "day product sanity: {full}");
        w.day_tick = Some(14_010); // the goldens' night tick
        let sky = light_brightness(15.0) * 0.24; // factor 0.24 * ramp 1.0
        let expect = [full * sky * (122.0f32 / 255.0), full * sky * (122.0 / 255.0), full * sky];
        for corner in &top_face(&w).colors {
            for (c, e) in corner.iter().zip(expect.iter()) {
                assert!((c - e).abs() < 1e-6, "night tint per channel: {c} vs {e}");
            }
        }
    }

    #[test]
    fn the_block_light_color_is_the_parabolic_tint_mix() {
        // the shader's parabola on the RAW level: 1.0 at BOTH ends
        // (level 0 and 15), exactly 0.0 at the mid — and the mix toward
        // white is 0.9 OF the parabola (lightmap.fsh verbatim).
        assert!(
            (parabolic_mix_factor(0.5) - 0.0).abs() < 1e-9,
            "mid level: the pure tint, no mix"
        );
        assert!(
            (parabolic_mix_factor(0.0) - 1.0).abs() < 1e-9,
            "level 0: the full mix (irrelevant there — brightness 0)"
        );
        assert!(
            (parabolic_mix_factor(1.0) - 1.0).abs() < 1e-9,
            "level 15: the full mix — white-hot torch"
        );
        // blocks 7 and 8 straddle the parabola's dip symmetrically —
        // the SAME mix, the nearest the integer levels get to the pure
        // #ffd88c (the dip's neighborhood, not the dip itself)
        let m7 = 0.9 * parabolic_mix_factor(7.0 / 15.0);
        let m8 = 0.9 * parabolic_mix_factor(8.0 / 15.0);
        assert!((m7 - m8).abs() < 1e-6, "the dip is level-symmetric: {m7} vs {m8}");
        assert!(m7 < 0.01, "near the pure tint: {m7}");
        // red is 1.0 at EVERY level (the tint's red is saturated — the
        // warmth lives entirely in green/blue)
        for b in [0.0f32, 3.0, 7.0, 8.0, 12.0, 15.0] {
            let c = block_light_color(b, None);
            assert!((c[0] - 1.0).abs() < 1e-9, "red constant at {b}: {}", c[0]);
        }
        // the extremes mix 0.9 of the way to white — derived from the
        // tint constant, both end levels share it (parabola 1.0 there)
        let expect_g = 216.0f32 / 255.0 + 0.9 * (1.0 - 216.0 / 255.0);
        let expect_b = 140.0f32 / 255.0 + 0.9 * (1.0 - 140.0 / 255.0);
        for b in [0.0f32, 15.0] {
            let c = block_light_color(b, None);
            assert!((c[1] - expect_g).abs() < 1e-6, "green at {b}: {} vs {expect_g}", c[1]);
            assert!((c[2] - expect_b).abs() < 1e-6, "blue at {b}: {} vs {expect_b}", c[2]);
        }
    }

    #[test]
    fn the_stated_block_light_tint_replaces_the_default_verbatim() {
        // the wire's stated attribute is the WHOLE tint (javap
        // LightmapRenderStateExtractor: the probe's value feeds
        // ARGB.vector3fFromRGB24 straight into the uniform — no
        // blending with the default). A stated cool tint (#302821,
        // the nether ambient's hue) shifts the mix target per channel;
        // the parabola itself is unchanged (still 0.9 on the raw
        // level, red saturated at the ends — the stated red is 0x30,
        // so red is NOT 1.0 here: the mix carries it toward white).
        let stated = Some([0x30 as f32 / 255.0, 0x28 as f32 / 255.0, 0x21 as f32 / 255.0]);
        let m15 = 0.9 * parabolic_mix_factor(15.0 / 15.0);
        let c = block_light_color(15.0, stated);
        let expect = [
            0x30 as f32 / 255.0 + m15 * (1.0 - 0x30 as f32 / 255.0),
            0x28 as f32 / 255.0 + m15 * (1.0 - 0x28 as f32 / 255.0),
            0x21 as f32 / 255.0 + m15 * (1.0 - 0x21 as f32 / 255.0),
        ];
        for (ch, e) in c.iter().zip(expect.iter()) {
            assert!((ch - e).abs() < 1e-6, "stated tint rides verbatim: {ch} vs {e}");
        }
        // the None call is the default's verdict (the corrected blue
        // 140 — NOT the retired 172 literal) — the two differ, the
        // stated path is a real consumer, not a shadowed argument
        let dflt = block_light_color(15.0, None);
        assert!(
            (dflt[2] - c[2]).abs() > 0.01,
            "stated vs default diverge: {} vs {}",
            dflt[2],
            c[2]
        );
        // the mesher witness: a stated tint on the world reaches the
        // block term through lightmap_combine — the flat path's own
        // plumbing (world.block_light_tint -> the combine's argument)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.block_light_tint = Some([0x30, 0x28, 0x21]);
        set_light(&mut w, None, Some(7), 8, 9, 8);
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let cs = block_light_color(7.0, stated);
        let expect_w = [b * cs[0], b * cs[1], b * cs[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_w.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the stated tint reaches the block term: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_stated_sky_light_color_folds_under_the_track() {
        // the sky-light color's consumer contract is the attribute-system
        // FOLD for the COLOR attribute (javap ColorModifier.MULTIPLY_RGB
        // -> ARGB.multiply(int, int): the -1 white default short-circuits
        // to the other operand, else per-channel imul / idiv 255 — the
        // probe's folded value IS an Integer the extractor unpacks): the
        // stated base (the end #ac60cd, the nether #7a7aff, a custom
        // server's wire) multiplies the BYTE-space day track per channel.
        // The integer floor bites: (128*122)/255 = 61 (61.24 floored),
        // blue (128*255)/255 = 128 exact.
        let stated = [0x80u8, 0x80, 0x80];
        let track = crate::timeline::sky_light_color_bytes(14_010); // the #7a7aff plateau
        assert_eq!(track, [122, 122, 255], "the night plateau's bytes");
        let folded = [
            ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
            ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
            ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
        ];
        assert_eq!(folded[0], 61.0 / 255.0, "the integer floor: 15616/255 = 61");
        assert_eq!(folded[1], 61.0 / 255.0);
        assert_eq!(folded[2], 128.0 / 255.0, "blue: (128*255)/255 = 128 exact");
        // the fold is NOT the float multiply — the arithmetic is the
        // bytecode's integer space, the fidelity round's verdict
        let float_mul = 128.0f32 / 255.0 * (122.0f32 / 255.0);
        assert!(
            (float_mul - folded[0]).abs() > 1e-4,
            "float {float_mul} vs integer-folded {}",
            folded[0]
        );
        // the mesher witness: the stated color reaches the sky term
        // through lightmap_combine — the night plateau's factor (0.24)
        // scales the brightness, the folded tint colors it
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.day_tick = Some(14_010);
        w.sky_light_color = Some(stated);
        set_light(&mut w, Some(15), None, 8, 9, 8);
        let s = light_brightness(15.0) * 0.24; // the factor fold at the plateau
        let expect_w = [s * folded[0], s * folded[1], s * folded[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_w.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the stated color reaches the sky term: {ch} vs {e}"
                );
            }
        }
        // the None world keeps the track verbatim (the -1 default's
        // white identity): the same scene unstated reads the track's
        // night blue directly — the exact expression the goldens pin
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        w.day_tick = Some(14_010);
        set_light(&mut w, Some(15), None, 8, 9, 8);
        let t = crate::timeline::sky_light_color(14_010);
        let expect_none = [s * t[0], s * t[1], s * t[2]];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect_none.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the None world is the track verbatim: {ch} vs {e}"
                );
            }
        }
    }

    #[test]
    fn the_block_term_is_tinted_and_factored_not_white() {
        // the full-channel mesher witness: a torch-lit top face (block
        // 7, no sky opinion) carries BlockLightColor(7) · brightness(7)
        // · BlockFactor per channel — red 7/15·1.4·1.0, green and blue
        // ride the near-pure-tint parabolic mix — ALL under the clamp.
        // The old white term would read brightness(7) flat on every
        // channel; the expectations derive from the audited constants.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(7), 8, 9, 8);
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let c = block_light_color(7.0, None);
        let expect = [b * c[0], b * c[1], b * c[2]];
        assert!(expect.iter().all(|e| *e < 1.0), "block 7 sits under the clamp");
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "torch tint per channel: {ch} vs {e}");
            }
        }
        // the clamp witness at block 14: red (1.0889) AND green (1.0350)
        // saturate, blue stays under (0.8540 tint × 1.0889 = 0.9300) —
        // the shader's single final clamp, the warm torch verdict
        set_light(&mut w, None, Some(14), 8, 9, 8);
        let b14 = light_brightness(14.0) * BLOCK_FACTOR;
        let c14 = block_light_color(14.0, None);
        let expect14 = [
            (b14 * c14[0]).min(1.0),
            (b14 * c14[1]).min(1.0),
            (b14 * c14[2]).min(1.0),
        ];
        assert!((expect14[0] - 1.0).abs() < 1e-6, "red saturates");
        assert!((expect14[1] - 1.0).abs() < 1e-6, "green saturates");
        assert!(expect14[2] < 1.0, "blue under: {}", expect14[2]);
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect14.iter()) {
                assert!((ch - e).abs() < 1e-6, "torch-14 per channel: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn the_stated_night_vision_color_rides_the_shader_max_term() {
        // the lightmap's LAST stated attribute: the shader's
        // `max(AmbientColor, NightVisionColor * NightVisionFactor)` base
        // (lightmap.fsh verbatim). The FACTOR is the player's EFFECT
        // state (NIGHT_VISION effect -> GameRenderer.nightVisionScale,
        // conduit power + water vision, else exactly 0.0 — javap
        // LightmapRenderStateExtractor.extract); this client carries no
        // effect system, so the factor sits at its bytecode-pinned
        // floor 0.0 and the term is the ambient base BIT FOR BIT for
        // any tint — max(A, C·0.0) = A for non-negative A. The fold's
        // arithmetic is pinned at the lifted factor too, the contract
        // the effect system's arrival will consume.
        let ambient = [0.1f32, 0.2, 0.3];
        // the floor: factor 0.0 — the identity, exact equality (no
        // tolerance: the goldens' untouched-by-construction witness)
        assert_eq!(
            night_vision_base(ambient, None, NIGHT_VISION_FACTOR),
            ambient,
            "factor 0.0: the default #999999 vanishes into the ambient base"
        );
        assert_eq!(
            night_vision_base(ambient, Some([0.3, 0.2, 0.1]), NIGHT_VISION_FACTOR),
            ambient,
            "factor 0.0: a stated tint vanishes into the ambient base too"
        );
        // the lifted contract, factor 1.0 — the nightVisionScale
        // plateau: None reads the BYTECODE default (#999999 gray =
        // 153/255 per channel) and the max lifts the dark channels
        let g = 153.0f32 / 255.0;
        assert_eq!(
            night_vision_base(ambient, None, 1.0),
            [ambient[0].max(g), ambient[1].max(g), ambient[2].max(g)],
            "factor 1.0: the gray default lifts the dark channels only"
        );
        // a stated color is the stated base: white maxes every channel
        // to 1.0; a DIM stated color below the ambient floor moves
        // NOTHING (the max is per-channel, never an average)
        assert_eq!(
            night_vision_base(ambient, Some([1.0; 3]), 1.0),
            [1.0; 3],
            "factor 1.0: the stated white maxes every channel"
        );
        assert_eq!(
            night_vision_base(ambient, Some([0.05, 0.0, 0.15]), 1.0),
            [ambient[0].max(0.05), ambient[1].max(0.0), ambient[2].max(0.15)],
            "the max is per-channel: a dim stated color cannot darken"
        );
        // the render-path witness: a lit cell renders EXACTLY the same
        // with a stated night-vision color as with None (the factor
        // 0.0 identity through the world snapshot — the goldens' own
        // shape), and a lifted factor WOULD lift the base (the same
        // fold the mesher calls, at the effect system's future state)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(4), Some(7), 8, 9, 8);
        let baseline = vertex_multiplier(&top_face(&w));
        w.night_vision_color = Some([0x18, 0x10, 0x28]);
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            baseline,
            "the stated attribute rides the snapshot; factor 0.0 keeps every pixel"
        );
        // the lifted base, derived through the SAME fold the paths call
        let amb = 0.0f32; // the black default: this scene's ambient
        let lifted = night_vision_base([amb; 3], Some([0x18 as f32 / 255.0, 0x10 as f32 / 255.0, 0x28 as f32 / 255.0]), 1.0);
        assert_eq!(
            lifted,
            [24.0f32 / 255.0, 16.0 / 255.0, 40.0 / 255.0],
            "the day the effect system lifts the factor, the stated color is the floor"
        );
    }

    #[test]
    fn the_night_vision_factor_lifts_the_lightmap_from_the_effect_state() {
        // the debt retired: the factor RIDES the snapshot — the
        // effect-absent floor 0.0 is the golden identity (every pixel
        // exactly the pre-round arithmetic), the effect's plateau 1.0
        // lifts the base through the SAME combine the paths call (the
        // nightVisionScale contract the const's doc carried).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(5), 8, 9, 8);
        let baseline = vertex_multiplier(&top_face(&w));
        // a stated night-vision color under the floor 0.0: the
        // identity, bit for bit (max(A, C·0.0) = A) — the goldens' own
        // shape, untouched by construction
        w.night_vision_color = Some([0x18, 0x10, 0x28]);
        assert_eq!(vertex_multiplier(&top_face(&w)), baseline);
        // the plateau 1.0 through the rendered path: the lifted base is
        // EXACTLY the shader's max(AmbientColor, NightVisionColor·1.0)
        // with the unchanged block term on top (lightmap.fsh verbatim)
        w.night_vision_factor = 1.0;
        let c = [0x18 as f32 / 255.0, 0x10 as f32 / 255.0, 0x28 as f32 / 255.0];
        let b = light_brightness(5.0) * BLOCK_FACTOR;
        let tint = block_light_color(5.0, None);
        let expect = [
            (c[0] + b * tint[0]).min(1.0),
            (c[1] + b * tint[1]).min(1.0),
            (c[2] + b * tint[2]).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "the lifted base per channel: {ch} vs {e}");
            }
        }
    }

    /// THE DARKNESS ARM THE SUBTRACT THE CLAMP KEEPS (javap
    /// LightmapRenderStateExtractor pc 304-344 + the 26.2 jar's
    /// `shaders/core/lightmap.fsh` — the `color = color -
    /// vec3(lightmapInfo.DarknessScale);` subtract the clamp's own
    /// floor speaks, the 0.0 no-effect floor the bit-for-bit identity
    /// keeps: the goldens' own shape, untouched by construction).
    #[test]
    fn the_darkness_arm_the_subtract_the_clamp_keeps() {
        // THE 0.0 FLOOR (the `x - 0.0` f32 identity + the bottom
        // clamp's no-op on the non-negative terms): every texel the
        // pre-darkness arithmetic keeps bit for bit
        let base = crate::mesher::lightmap_combine(
            Some(15.0),
            Some(5.0),
            1.0,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
            None,
            None,
            0.0,
            0.0,
        );
        let expect_raw = {
            let sky = light_brightness(15.0) * 1.0;
            let b = light_brightness(5.0) * BLOCK_FACTOR;
            let c = block_light_color(5.0, None);
            [0.0 + sky + b * c[0], 0.0 + sky + b * c[1], 0.0 + sky + b * c[2]]
        };
        let expect = [
            expect_raw[0].min(1.0),
            expect_raw[1].min(1.0),
            expect_raw[2].min(1.0),
        ];
        assert_eq!(base, expect, "the no-effect floor the identity wears");
        // THE SUBTRACT (the shader's own arm): the bright texel the
        // 0.45 scale dims — the exact float the subtract keeps
        let dimmed = crate::mesher::lightmap_combine(
            Some(15.0),
            Some(5.0),
            1.0,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
            None,
            None,
            0.0,
            0.45,
        );
        // THE SHADER'S OWN ORDER (the clamp rides AFTER the darkness
        // subtract — the unclamped sum the subtract sees, the
        // `clamp(color, 0.0, 1.0)` the tail wears): the bright texel
        // 1.0 + block-term 0.1556 = 1.1556 - 0.45 = 0.7056 — the
        // clamp-before-subtract shape the identity never spoke
        for ch in 0..3 {
            let raw = expect_raw[ch];
            let want = (raw - 0.45).clamp(0.0, 1.0);
            assert!(
                (dimmed[ch] - want).abs() < 1e-6,
                "the bright texel the subtract wears: {} vs {}",
                dimmed[ch],
                want
            );
        }
        // THE CLAMP'S OWN FLOOR (the dark texel the ambient base alone
        // carries — the subtraction's negative the 0.0 clamps: the
        // shader's `clamp(color, 0.0, 1.0)` BOTH sides)
        let dark = crate::mesher::lightmap_combine(
            Some(0.0),
            Some(0.0),
            0.26666668,
            [1.0, 1.0, 1.0],
            [0.039215688, 0.039215688, 0.039215688],
            None,
            None,
            0.0,
            0.45,
        );
        assert_eq!(dark, [0.0, 0.0, 0.0], "the dark texel the floor clamps");
        // THE WORLD FIELD'S OWN RIDE (the mesher's call sites the
        // World's own field consume — the mesh's vertex colors the
        // scale's dim wears, the no-effect floor the bake never
        // touches)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(5), 8, 9, 8);
        let floor = vertex_multiplier(&top_face(&w));
        w.darkness_scale = 0.45;
        let dimmed = vertex_multiplier(&top_face(&w));
        for (fc, dc) in floor.iter().zip(dimmed.iter()) {
            assert!(
                dc < fc || *fc <= 0.0,
                "the dimmed bake never brightens: {dc} vs {fc}"
            );
        }
        // the floor restored — the identity the zero keeps
        w.darkness_scale = 0.0;
        assert_eq!(vertex_multiplier(&top_face(&w)), floor);
    }

    #[test]
    fn the_ambient_base_lifts_the_combine_per_channel() {
        // the lightmap's AmbientColor BASE (v0.3.88): the shader's
        // `max(AmbientColor, nightVisionColor)` start — night vision 0,
        // so the dimension's stated attribute itself — sits UNDER the
        // sky/block sum. The overworld's #0a0a0a lifts a dark torch
        // cell by exactly 10/255 per channel over the black default
        // (derived from the None arithmetic the previous tests pin);
        // the nether's #302821 floor is per-CHANNEL, not uniform.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, None, Some(7), 8, 9, 8);
        // the black default: the pre-ambient arithmetic
        let b = light_brightness(7.0) * BLOCK_FACTOR;
        let c = block_light_color(7.0, None);
        let dark = [b * c[0], b * c[1], b * c[2]];
        // the overworld floor: the SAME sum, +10/255 per channel
        w.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        let lift = 10.0f32 / 255.0;
        let expect = [
            (dark[0] + lift).min(1.0),
            (dark[1] + lift).min(1.0),
            (dark[2] + lift).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "ambient lifts per channel: {ch} vs {e}");
            }
        }
        // the nether floor #302821: every channel a different step
        w.ambient_light = Some([0x30, 0x28, 0x21]);
        let expect = [
            (dark[0] + 48.0 / 255.0).min(1.0),
            (dark[1] + 40.0 / 255.0).min(1.0),
            (dark[2] + 33.0 / 255.0).min(1.0),
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!(
                    (ch - e).abs() < 1e-6,
                    "the nether floor is per-channel: {ch} vs {e}"
                );
            }
        }
        // a world with NO light grid keeps the legacy look even with a
        // stated floor: the early return wins (the procedural scenes'
        // shape — their light layer is None, the floor rides nothing)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        w2.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        assert_eq!(
            vertex_multiplier(&top_face(&w2)),
            [1.0; 4],
            "no light layer: the legacy look, ambient or not"
        );
    }

    #[test]
    fn the_ambient_base_never_moves_the_saturated_end() {
        // day sky-15 over the overworld floor STILL clamps to 1.0 — the
        // ambient sits under the sum, the final clamp's verdict is the
        // shader's; and the byte-carrying world stays None-identical on
        // the shapes the previous tests pin (the black default is the
        // pre-ambient arithmetic, the regression is theirs)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), Some(0), 8, 9, 8);
        w.ambient_light = Some([0x0a, 0x0a, 0x0a]);
        assert_eq!(
            vertex_multiplier(&top_face(&w)),
            [1.0; 4],
            "day sky-15 + ambient still clamps to the legacy full-bright"
        );
        // a dark night cell (sky-1, the 0.24 factor, the #7a7aff tint)
        // over the end's #3f473f floor: the floor dominates, the sky term
        // adds its sliver per channel — derived from the None-arithmetic
        // terms
        w.ambient_light = Some([0x3f, 0x47, 0x3f]);
        set_light(&mut w, Some(1), Some(0), 8, 9, 8);
        w.day_tick = Some(18_000);
        let tint_rg = 122.0f32 / 255.0;
        let sky = light_brightness(1.0) * 0.24;
        let expect = [
            63.0f32 / 255.0 + sky * tint_rg,
            71.0f32 / 255.0 + sky * tint_rg,
            63.0f32 / 255.0 + sky,
        ];
        for corner in &top_face(&w).colors {
            for (ch, e) in corner.iter().zip(expect.iter()) {
                assert!((ch - e).abs() < 1e-6, "end floor at night: {ch} vs {e}");
            }
        }
    }

    #[test]
    fn corner_light_averages_the_four_face_plane_cells() {
        // face cell (8,9,8) full sky; ONE known dark diagonal (7,9,7) —
        // exactly corner 0 samples it (x=0,z=0 -> d1=-x, d2=-z)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        w.light = Some(unknown_light());
        set_light(&mut w, Some(15), None, 8, 9, 8);
        set_light(&mut w, Some(0), None, 7, 9, 7);
        let m = vertex_multiplier(&top_face(&w));
        // corner 0 averages its known cells (15+0)/2 = 7.5 -> brightness 0.2
        assert!((light_brightness(7.5) - 0.2).abs() < 1e-6, "ramp sanity");
        assert!(
            (m[0] - light_brightness(7.5)).abs() < 1e-6,
            "the dark diagonal drags only its corner: {m:?}"
        );
        // the other corners know only the face cell
        assert_eq!(m[1], 1.0, "corner 1 untouched: {m:?}");
        assert_eq!(m[2], 1.0, "corner 2 untouched: {m:?}");
        assert_eq!(m[3], 1.0, "corner 3 untouched: {m:?}");
    }

    #[test]
    fn ao_open_face_is_bright() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        // top face has nothing above it anywhere -> every corner level 3
        assert_eq!(ao_levels(&w, 8, 8, 8, 2), [3, 3, 3, 3]);
        assert_eq!(ao_levels(&w, 8, 8, 8, 0), [3, 3, 3, 3]);
    }

    #[test]
    fn ao_wall_darkens_adjacent_ground() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS); // ground block
        put(&mut w, 9, 9, 8, STONE); // wall one step up, +X side
        // On the ground top face, corners adjacent to the wall (x=1 side)
        // pick up occlusion: their +X neighbor at y+1 is stone.
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert!(levels.iter().any(|&l| l < 3), "wall must darken some corner: {:?}", levels);
        // the far corners (x=0 side) stay fully open
        let far: Vec<u8> = FACE_CORNERS[2]
            .iter()
            .enumerate()
            .filter(|(_, c)| c[0] < 0.5)
            .map(|(i, _)| levels[i])
            .collect();
        assert!(far.iter().all(|&l| l == 3), "far corners must stay bright: {:?}", levels);
    }

    #[test]
    fn ao_enclosed_corner_is_darkest() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        // two walls meeting at the (1,1) corner above the ground block
        put(&mut w, 9, 9, 8, STONE);
        put(&mut w, 8, 9, 9, STONE);
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert!(levels.contains(&0), "double-sided corner must hit level 0: {:?}", levels);
    }

    #[test]
    fn ao_ignores_non_opaque_neighbors() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        put(&mut w, 9, 9, 8, WATER);
        put(&mut w, 9, 9, 9, LEAVES);
        assert_eq!(ao_levels(&w, 8, 8, 8, 2), [3, 3, 3, 3], "water/leaves must not occlude");
    }

    #[test]
    fn ao_corner_offset_maps_to_correct_neighbor() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        // single stone diagonal to exactly one corner of the top face:
        // base cell above the block is (8,9,8); corner 0 samples (7,9,7).
        put(&mut w, 7, 9, 7, STONE);
        let levels = ao_levels(&w, 8, 8, 8, 2);
        assert_eq!(levels, [2, 3, 3, 3], "only the diagonal corner darkens: {:?}", levels);
    }

    #[test]
    fn quad_diagonal_flips_to_brighter_pair() {
        // AO pattern whose brighter pair is (1,3): the fan diagonal must
        // rotate onto that pair; an already-optimal pattern stays untouched.
        let flipped = flip_order(&[1, 3, 1, 3]);
        assert_eq!(flipped, [1, 2, 3, 0]);
        assert!(3 + 3 >= 1 + 3, "new diagonal must carry the max AO sum");
        assert_eq!(flip_order(&[3, 1, 3, 1]), [0, 1, 2, 3], "optimal diagonal stays");
        assert_eq!(flip_order(&[2, 2, 2, 2]), [0, 1, 2, 3], "uniform AO stays");
    }

    #[test]
    fn mesh_colors_match_face_shade_and_ao() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        let q = mesh_chunk(&w, 0, 0, None);
        // top face of the lone block: shade 1.0, open sky -> full brightness
        let top = q.iter().find(|quad| quad.pts[0][1] == 9.0 && quad.pts[1][1] == 9.0).expect("top face quad");
        let tile = atlas::FACE_TILES[STONE as usize][2];
        if tile == atlas::TILE_NONE {
            let expected = default_color(STONE, 2);
            for c in &top.colors {
                for ch in 0..3 {
                    assert!((c[ch] - expected[ch]).abs() < 1e-6, "{:?} vs {:?}", c, expected);
                }
            }
        } else {
            // textured path: vertex colors carry the shade*AO multiplier only
            for c in &top.colors {
                assert!((c[0] - 1.0).abs() < 1e-6, "open-sky top multiplier must be 1.0: {:?}", c);
                assert_eq!(c[0], c[1]);
                assert_eq!(c[1], c[2]);
            }
        }
    }

    #[test]
    fn the_farmland_the_dry_top_the_master_wears() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::FARMLAND);
        let q = mesh_chunk(&w, 0, 0, None);
        let dry_top = atlas::FACE_TILES[crate::world::FARMLAND as usize][2];
        // the top quad the dry master wears (the moisture=0 variant the
        // model chain resolves)
        assert!(
            q.iter()
                .any(|quad| quad.pts[0][1] == 9.0 && quad.tile == dry_top),
            "the dry farmland the FACE_TILES top master wears"
        );
        // the wet verdict the no-water world never picks
        assert!(
            !q.iter()
                .any(|quad| quad.tile == atlas::TILE_FARMLAND_MOIST),
            "the dry world the wet tile never speaks"
        );
    }

    #[test]
    fn the_farmland_the_wet_top_the_moist_wears() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::FARMLAND);
        put(&mut w, 4, 8, 8, WATER); // the dx=-4 edge the box scans
        let q = mesh_chunk(&w, 0, 0, None);
        let dry_top = atlas::FACE_TILES[crate::world::FARMLAND as usize][2];
        // the top quad the wet master wears (the moisture=7 variant's own
        // tile the real chain resolves — the is_near_water verdict the
        // vanilla texture pick rides)
        assert!(
            q.iter().any(
                |quad| quad.pts[0][1] == 9.0 && quad.tile == atlas::TILE_FARMLAND_MOIST
            ),
            "the near-water farmland the farmland_moist master wears"
        );
        assert!(
            !q.iter().any(|quad| quad.tile == dry_top),
            "the wet cell the dry master never wears"
        );
        // THE BOUNDARY: the water one-past the box the dry arm keeps
        put(&mut w, 4, 8, 8, AIR);
        put(&mut w, 2, 8, 8, WATER);
        let q2 = mesh_chunk(&w, 0, 0, None);
        assert!(
            q2.iter()
                .any(|quad| quad.pts[0][1] == 9.0 && quad.tile == dry_top),
            "the far water the dry master keeps"
        );
        assert!(
            !q2.iter()
                .any(|quad| quad.tile == atlas::TILE_FARMLAND_MOIST),
            "the far water the wet tile never speaks"
        );
        // THE SIDES: the dirt the both models name (the wet verdict the
        // top face alone picks — the farmland_wet model the sides share)
        put(&mut w, 2, 8, 8, AIR);
        put(&mut w, 4, 8, 8, WATER);
        let q3 = mesh_chunk(&w, 0, 0, None);
        let dirt_tile = atlas::FACE_TILES[crate::world::FARMLAND as usize][0];
        assert!(
            q3.iter().any(|quad| quad.tile == dirt_tile),
            "the side quads the dirt master wears"
        );
        assert!(
            q3.iter().any(
                |quad| quad.pts[0][1] == 9.0 && quad.tile == atlas::TILE_FARMLAND_MOIST
            ),
            "the wet top the second fill wears"
        );
    }

    #[test]
    fn the_furnace_the_front_tile_the_facing_wears() {
        // THE FURNACE'S OWN FRONT-TILE SWAP (the autopush-601 round —
        // the orientable model's one textured direction the facing
        // byte rotates: the variant 1..=4 the north/east/south/west
        // order the blockstate's y-rotation keeps, the front tile the
        // facing direction's face wears, the vacated north slot the
        // side tile keeps; the byte's no-opinion answer the
        // north-facing default the blockstate's own y=0 row banks)
        let cell = |x: i32, y: i32, z: i32| {
            (x as usize)
                + (z as usize) * CHUNK_X
                + (y as usize) * CHUNK_X * CHUNK_Z
        };
        let front = atlas::FACE_TILES[crate::world::FURNACE as usize][5];
        let side = atlas::FACE_TILES[crate::world::FURNACE as usize][0];
        let top = atlas::FACE_TILES[crate::world::FURNACE as usize][2];

        // facing=east (the variant byte 2): the +X face the front
        // wears, the vacated north (-Z) the side keeps, the top the
        // top tile banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::FURNACE);
        let mut variants = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants[0][cell(8, 8, 8)] = 2;
        w.variants = Some(variants);
        let q = mesh_chunk(&w, 0, 0, None);
        assert!(
            q.iter().any(|quad| quad.pts[0][0] == 9.0 && quad.tile == front),
            "the east-facing front the +X face wears"
        );
        assert!(
            q.iter().any(|quad| quad.pts[0][2] == 8.0 && quad.tile == side),
            "the vacated north the side tile wears"
        );
        assert!(
            q.iter().any(|quad| quad.pts[0][1] == 9.0 && quad.tile == top),
            "the top the top tile keeps"
        );
        // exactly six quads: the full cube the generic path meshes
        let furnace_quads = q
            .iter()
            .filter(|quad| {
                quad.tile == front
                    || quad.tile == side
                    || quad.tile == top
                    || quad.tile == atlas::FACE_TILES[crate::world::FURNACE as usize][3]
            })
            .count();
        assert_eq!(furnace_quads, 6, "the lone cube's own six faces");

        // the no-opinion byte: the north face the front wears (the
        // default state the blockstate's y=0 row banks), the +X the
        // side keeps
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::FURNACE);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        assert!(
            q2.iter().any(|quad| quad.pts[0][2] == 8.0 && quad.tile == front),
            "the north-facing front the -Z face wears"
        );
        assert!(
            q2.iter().any(|quad| quad.pts[0][0] == 9.0 && quad.tile == side),
            "the east side the side tile keeps"
        );

        // THE LIT KIND: the front_on tile the same north default wears
        // (the FACE_TILES row's own north entry the lit kind banks)
        let front_on = atlas::FACE_TILES[crate::world::FURNACE_LIT as usize][5];
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::FURNACE_LIT);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        assert!(
            q3.iter().any(|quad| quad.pts[0][2] == 8.0 && quad.tile == front_on),
            "the lit furnace's front_on the north face wears"
        );
        assert!(
            !q3.iter().any(|quad| quad.tile == front),
            "the lit kind the unlit front never wears"
        );

        // facing=south (the variant byte 3): the +Z face the front wears
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::FURNACE);
        let mut variants4 = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
        variants4[0][cell(8, 8, 8)] = 3;
        w4.variants = Some(variants4);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        assert!(
            q4.iter().any(|quad| quad.pts[0][2] == 9.0 && quad.tile == front),
            "the south-facing front the +Z face wears"
        );
        assert!(
            q4.iter().any(|quad| quad.pts[0][2] == 8.0 && quad.tile == side),
            "the vacated north the side tile keeps"
        );
    }

    #[test]
    fn the_door_panel_the_facing_edges_wear() {
        // THE DOOR'S OWN ELEMENT LANE (the autopush-655 round — the
        // theta tables the blockstate's own y values fold: the closed
        // panel edge the OPPOSITE facing speaks [the SHAPES clinit's
        // boxZ(16.0, 13.0, 16.0) the facing=north panel at z 13..16
        // the model's own y-270 rotation the SAME edge the consistency
        // check answers], the open's swing the facing's LEFT the
        // left-hinge rows speak; the lower the #bottom master the
        // upper the #top banks)
        let bottom = atlas::TILE_OAK_DOOR_BOTTOM;
        let top = atlas::TILE_OAK_DOOR_TOP;
        let cell = |x: i32, y: i32, z: i32| {
            (x as usize)
                + (z as usize) * CHUNK_X
                + (y as usize) * CHUNK_X * CHUNK_Z
        };
        let variants = |v: u8| -> Vec<Vec<u8>> {
            let mut g = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
            g[0][cell(8, 8, 8)] = v;
            g
        };
        // THE CLOSED EAST DOOR (the variant byte 2): the base model's
        // own y=0 authoring — the west strip x 0..3 the panel wears,
        // five quads the lone panel speaks (down, N, S, W + the
        // uncullable east)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::OAK_DOOR);
        w.variants = Some(variants(2));
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(
            q.iter()
                .filter(|quad| quad.tile == bottom)
                .count(),
            5,
            "the lone closed panel's own five faces"
        );
        assert!(
            q.iter()
                .all(|quad| quad.tile != bottom || quad.pts.iter().all(|p| p[0] <= 8.1876)),
            "the closed east door the west strip wears"
        );
        // THE CLOSED NORTH DOOR (the variant byte 0 — the unpropertied
        // default): the y=270 rotation three CW steps speak — the
        // SOUTH edge z 13..16 the panel wears (the collision SHAPES
        // row the same edge the consistency check answers)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::OAK_DOOR);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        assert!(
            q2.iter()
                .all(|quad| quad.tile != bottom || quad.pts.iter().all(|p| p[2] >= 8.8124)),
            "the closed north door the south edge wears"
        );
        // THE OPEN EAST DOOR (the variant byte 2): the y=90 rotation
        // one CW step speaks — the LEFT-of-facing edge the swing
        // wears (east's left = north, z 0..3)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::OAK_DOOR_OPEN);
        w3.variants = Some(variants(2));
        let q3 = mesh_chunk(&w3, 0, 0, None);
        assert!(
            q3.iter()
                .all(|quad| quad.tile != bottom || quad.pts.iter().all(|p| p[2] <= 8.1876)),
            "the open east door the north edge wears"
        );
        // THE UPPER'S OWN MASTER (the #top ref the tile row banks —
        // the lower the #bottom the pair keeps)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::OAK_DOOR_UPPER);
        w4.variants = Some(variants(2));
        let q4 = mesh_chunk(&w4, 0, 0, None);
        assert!(
            q4.iter().any(|quad| quad.tile == top),
            "the upper half the top master wears"
        );
        assert!(
            !q4.iter().any(|quad| quad.tile == bottom),
            "the upper half the bottom master never wears"
        );
        // THE ROTATED CULLFACE'S OWN VERDICT (the direction the SAME
        // steps the geometry turns: the closed north panel's south
        // cullface the solid neighbor culls — the five faces the four
        // the stone keeps)
        let mut w5 = empty_world();
        put(&mut w5, 8, 8, 8, crate::world::OAK_DOOR);
        put(&mut w5, 8, 8, 9, crate::world::STONE);
        let q5 = mesh_chunk(&w5, 0, 0, None);
        assert_eq!(
            q5.iter().filter(|quad| quad.tile == bottom).count(),
            4,
            "the solid south neighbor the rotated cullface serves"
        );
    }

    #[test]
    fn the_door_copper_ladder_the_waxed_models_inherit() {
        // THE COPPER LADDER'S OWN MODEL WALK (the autopush-656 round —
        // the generated DOOR_MODEL_BY_ROW the blockstate's own rows
        // resolve: the weathering ladder the OWN per-state models
        // carries, the WAXED kinds the UNWAXED models the jar's own
        // rows serve — the waxed_copper_door.json the
        // block/copper_door_bottom_left model the facing=east row
        // speaks, the waxed model files NO jar keeps; the per-
        // oxidization textures the waxed kinds inherit, the mesher's
        // pair table the resolved stems the unwaxed rows answer)
        let cell = |x: i32, y: i32, z: i32| {
            (x as usize)
                + (z as usize) * CHUNK_X
                + (y as usize) * CHUNK_X * CHUNK_Z
        };
        let variants = |v: u8| -> Vec<Vec<u8>> {
            let mut g = vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]];
            g[0][cell(8, 8, 8)] = v;
            g
        };
        // THE OWN MODELS THE WEATHERING LADDER WEARS (the copper
        // ladder's own per-state masters the TILE consts bank)
        for (kind, bottom) in [
            (crate::world::COPPER_DOOR, atlas::TILE_COPPER_DOOR_BOTTOM),
            (
                crate::world::EXPOSED_COPPER_DOOR,
                atlas::TILE_EXPOSED_COPPER_DOOR_BOTTOM,
            ),
            (
                crate::world::WEATHERED_COPPER_DOOR,
                atlas::TILE_WEATHERED_COPPER_DOOR_BOTTOM,
            ),
            (
                crate::world::OXIDIZED_COPPER_DOOR,
                atlas::TILE_OXIDIZED_COPPER_DOOR_BOTTOM,
            ),
            (crate::world::IRON_DOOR, atlas::TILE_IRON_DOOR_BOTTOM),
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, kind);
            w.variants = Some(variants(0));
            let q = mesh_chunk(&w, 0, 0, None);
            assert!(
                q.iter().any(|quad| quad.tile == bottom),
                "the kind {kind} the own per-state master wears"
            );
        }
        // THE WAXED KINDS' OWN INHERITANCE (the unwaxed models the
        // blockstate rows serve — the waxed_copper cell the copper
        // master meshes, the waxed_oxidized the oxidized master keeps)
        for (kind, bottom) in [
            (crate::world::WAXED_COPPER_DOOR, atlas::TILE_COPPER_DOOR_BOTTOM),
            (
                crate::world::WAXED_OXIDIZED_COPPER_DOOR,
                atlas::TILE_OXIDIZED_COPPER_DOOR_BOTTOM,
            ),
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, kind);
            w.variants = Some(variants(0));
            let q = mesh_chunk(&w, 0, 0, None);
            assert!(
                q.iter().any(|quad| quad.tile == bottom),
                "the waxed kind {kind} the unwaxed master inherits"
            );
        }
        // THE IRON DOOR'S OWN FACE_TILES ROW (the generated table the
        // same resolution banks — the kind's own row the fallback
        // verdict serves)
        assert_eq!(
            atlas::FACE_TILES[crate::world::IRON_DOOR as usize][2],
            atlas::TILE_IRON_DOOR_BOTTOM,
            "the iron row the own master banks"
        );
        assert_eq!(
            atlas::FACE_TILES[crate::world::WAXED_COPPER_DOOR as usize][2],
            atlas::TILE_COPPER_DOOR_BOTTOM,
            "the waxed row the copper master banks"
        );
    }

    #[test]
    fn the_lever_rows_the_models_and_the_tile_bank_wear() {
        // THE LEVER'S OWN ATLAS LAWS (the dev-45 round — the two
        // palette rows the lever arm resolves, the tile the named
        // const banks, the jar's own REVERSED model rows the derive
        // walk wears): the rows the lever tile alone carries (all six
        // slots the ONE block/lever texture), the cobblestone sidecar
        // the base plate's #base ref answers, the models' per-quad
        // texture verdict the two names the branch reads, the stick's
        // own ±45° tilt the derive walk's rotated vertices wear.
        let lever_tile = atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "lever")
            .expect("the lever tile the atlas banks") as u16;
        let cobble_tile = atlas::TILE_COBBLESTONE;
        assert_eq!(
            atlas::TILE_NAMES[cobble_tile as usize],
            "cobblestone",
            "the sidecar's own name the const banks"
        );
        for kind in [crate::world::LEVER, crate::world::LEVER_ON] {
            for slot in 0..6 {
                assert_eq!(
                    atlas::FACE_TILES[kind as usize][slot],
                    lever_tile,
                    "the lever row's slot {slot} the one lever tile wears"
                );
            }
        }
        // THE MODELS' OWN WALK (the derived quads the two element
        // boxes carry — the base plate's six faces + the stick's five
        // the down face never defines): the texture verdict the two
        // names the branch reads, the tilt the rotated stick's own
        // vertices answer (the straight band the z 7..9/16 the
        // unrotated box keeps — the tilt's own corners the band
        // escapes).
        let off = lever_model_quads(false);
        let on = lever_model_quads(true);
        assert!(!off.is_empty() && !on.is_empty(), "the two models derive");
        for (powered, quads) in [(false, off), (true, on)] {
            let stems: std::collections::HashSet<&str> = quads
                .iter()
                .map(|q| {
                    q.texture
                        .trim_start_matches("minecraft:")
                        .trim_start_matches("block/")
                })
                .collect();
            assert!(
                stems.contains("lever") && stems.contains("cobblestone"),
                "the powered={powered} model the two textures wear ({stems:?})"
            );
            assert_eq!(
                quads.len(),
                11,
                "the powered={powered} model the six + five faces the jar banks"
            );
        }
        // THE TILT'S OWN SPREAD (the stick's own corners the straight
        // band escapes — the rotation's ±45° the y-rot never serves)
        let straight_band = |q: &mc_registry::element_quads::Quad| {
            q.vertices
                .iter()
                .all(|v| v[2] >= 0.4375 && v[2] <= 0.5625)
        };
        assert!(
            off.iter().any(|q| !straight_band(q)),
            "the off model's tilted stick the band escapes"
        );
        assert!(
            on.iter().any(|q| !straight_band(q)),
            "the on model's tilted stick the band escapes"
        );
        // THE TWO MODELS' OWN DIVERGENCE (the ±45° the sign flips —
        // the vertex sets the two models never share)
        let fp: Vec<[f32; 3]> = off
            .iter()
            .flat_map(|q| q.vertices.iter().copied())
            .collect();
        let tp: Vec<[f32; 3]> = on
            .iter()
            .flat_map(|q| q.vertices.iter().copied())
            .collect();
        assert_ne!(fp, tp, "the two tilts the vertex sets split");
    }


    #[test]
    fn the_button_models_the_plate_and_the_x_turn_wear() {
        // THE BUTTON'S OWN MODEL LAWS (the autopush-660 round — the
        // DIRECT authoring the resolution wears): the one-plate model
        // both states derive (the six faces the down cullface
        // carries), the pressed plate the thinner band the 1.02 top
        // wears, the wall button the plate the supporter's own edge
        // the x=90 turn presses, the rotated cullface the neighbor
        // culls (5 faces) / the air restores (6).
        let off = button_model_quads(crate::world::BUTTON, false);
        let on = button_model_quads(crate::world::BUTTON_ON, true);
        assert!(!off.is_empty() && !on.is_empty(), "the two models derive");
        assert_eq!(off.len(), 6, "the unpressed plate the six faces wear");
        assert_eq!(on.len(), 6, "the pressed plate the six faces wear");
        for (powered, quads) in [(false, off), (true, on)] {
            for q in quads {
                let stem = q
                    .texture
                    .trim_start_matches("minecraft:")
                    .trim_start_matches("block/");
                assert_eq!(
                    stem, "stone",
                    "the powered={powered} quad the one stone texture wears"
                );
            }
        }
        // THE THICKNESS'S OWN PAIR (the unpressed plate the 2/16 top
        // wears, the pressed the 1.02/16 top the thinner band keeps)
        let max_y = |qs: &[mc_registry::element_quads::Quad]| {
            qs.iter()
                .flat_map(|q| q.vertices.iter())
                .map(|v| v[1])
                .fold(f32::MIN, f32::max)
        };
        assert!(
            (max_y(off) - 0.125).abs() < 1e-3,
            "the unpressed plate the 2/16 top wears ({})",
            max_y(off)
        );
        assert!(
            (max_y(on) - 0.0638).abs() < 1e-3,
            "the pressed plate the 1.02/16 top wears ({})",
            max_y(on)
        );
        // THE WALL BUTTON'S OWN EDGE PRESS (the north-facing button the
        // dirt south the survival law banks — the plate the cell's
        // south edge the x=90 turn wears, the rotated cullface the
        // Down->South turn the neighbor culls; the dirt supporter the
        // own tile the filter's clean read keeps)
        let stone_tile = atlas::FACE_TILES[crate::world::BUTTON as usize][2];
        assert_eq!(atlas::TILE_NAMES[stone_tile as usize], "stone", "the button's own tile the row banks");
        let mut w = empty_world();
        let (x, y, z) = (5i32, 3, 5);
        w.set(x, y, z + 1, DIRT);
        w.set(x, y, z, BUTTON);
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 1),
        );
        let quads = mesh_world(&w, None);
        let cell: Vec<&Quad> = quads
            .iter()
            .filter(|q| {
                q.tile == stone_tile
                    && q.pts.iter().all(|p| {
                        p[0] >= x as f32 - 0.01
                            && p[0] <= x as f32 + 1.01
                            && p[1] >= y as f32 - 0.01
                            && p[1] <= y as f32 + 1.01
                            && p[2] >= z as f32 - 0.01
                            && p[2] <= z as f32 + 1.01
                    })
            })
            .collect();
        assert_eq!(cell.len(), 5, "the south-culled plate the five faces keep");
        for q in &cell {
            for p in q.pts.iter() {
                assert!(
                    p[2] >= z as f32 + 0.79,
                    "the plate the south edge the x-turn wears ({p:?})"
                );
                assert!(
                    p[1] >= y as f32 + 0.36 && p[1] <= y as f32 + 0.64,
                    "the plate the mid band the horizontal read keeps ({p:?})"
                );
            }
        }
        // THE AIR'S OWN RESTORE (the south cell the down-culled face
        // the six faces the verdict returns)
        w.set(x, y, z + 1, AIR);
        let quads = mesh_world(&w, None);
        let cell: Vec<&Quad> = quads
            .iter()
            .filter(|q| {
                q.tile == stone_tile
                    && q.pts.iter().all(|p| {
                        p[0] >= x as f32 - 0.01
                            && p[0] <= x as f32 + 1.01
                            && p[1] >= y as f32 - 0.01
                            && p[1] <= y as f32 + 1.01
                            && p[2] >= z as f32 - 0.01
                            && p[2] <= z as f32 + 1.01
                    })
            })
            .collect();
        assert_eq!(cell.len(), 6, "the air south the culled face restores");
    }
    #[test]
    fn the_lever_wall_ceiling_the_x_turn_lanes_wear() {
        // THE WALL/CEILING LANES' OWN TURNS (this round — the
        // blockstate's own rows the x/y fold serves: the WALL rows the
        // x=90 turn + the facing's y steps wear, the CEILING rows the
        // x=180 turn + the y-remap the (idx+2)%4 fold banks, the
        // cullface the same turns the flush verdict rotates — the
        // plate the supporter's own edge the geometry presses, the
        // cull the rotated direction the neighbor read answers).
        let lever_tile = atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "lever")
            .expect("the lever tile the atlas banks") as u16;
        let cobble_tile = atlas::TILE_COBBLESTONE;
        fn cell_quads<'a>(quads: &'a [Quad], x: i32, y: i32, z: i32) -> Vec<&'a Quad> {
            quads
                .iter()
                .filter(|q| {
                    q.pts.iter().all(|p| {
                        p[0] >= x as f32 - 0.01
                            && p[0] <= x as f32 + 1.01
                            && p[1] >= y as f32 - 0.01
                            && p[1] <= y as f32 + 1.01
                            && p[2] >= z as f32 - 0.01
                            && p[2] <= z as f32 + 1.01
                    })
                })
                .collect()
        }
        // THE WALL LEVER'S OWN SOUTH PRESS (the north-facing lever the
        // stone south the survival law banks — the plate the cell's
        // south edge the x=90 turn wears, the rotated cullface the
        // Down->South turn the stone neighbor culls — the six plate
        // faces the five the flush verdict keeps)
        let mut w = empty_world();
        let (x, y, z) = (5i32, 3, 5);
        w.set(x, y, z + 1, STONE);
        w.set(x, y, z, LEVER);
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 1),
        );
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let plates: Vec<&&Quad> = cell.iter().filter(|q| q.tile == cobble_tile).collect();
        let sticks: Vec<&&Quad> = cell.iter().filter(|q| q.tile == lever_tile).collect();
        assert_eq!(
            plates.len(),
            5,
            "the south-culled plate the five faces the flush verdict keeps"
        );
        for q in &plates {
            for p in q.pts.iter() {
                assert!(
                    p[2] >= z as f32 + 0.80,
                    "the plate the south edge the x-turn wears ({p:?})"
                );
                assert!(
                    p[1] >= y as f32 + 0.24 && p[1] <= y as f32 + 0.76,
                    "the plate the mid-y band the horizontal read keeps ({p:?})"
                );
            }
        }
        assert!(!sticks.is_empty(), "the stick the lever tile wears");
        for q in &sticks {
            for p in q.pts.iter() {
                assert!(
                    p[2] >= z as f32 + 0.44 && p[2] <= z as f32 + 0.99,
                    "the stick the wall's own reach the x-turn keeps ({p:?})"
                );
                assert!(
                    p[1] > y as f32 && p[1] < y as f32 + 1.0,
                    "the tilted stick the cell's own interior keeps ({p:?})"
                );
            }
        }
        assert!(
            sticks.iter().any(|q| q.pts.iter().any(|p| p[2] < z as f32 + 0.5)),
            "the stick the cell's own depth the protrude wears"
        );
        // THE CULL'S OWN ROTATED DIRECTION (the air south the down-
        // culled face the six faces the verdict restores — the rotated
        // cullface the SOUTH neighbor the x-turn banks)
        w.set(x, y, z + 1, AIR);
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let plates = cell.iter().filter(|q| q.tile == cobble_tile).count();
        assert_eq!(plates, 6, "the air south the culled face restores");
        // THE OPPOSITE FACING'S OWN NORTH PRESS (the south-facing
        // lever the stone north the supporter banks — the plate the
        // cell's north edge the two y steps wear, the rotated cullface
        // the Down->South->North turn the stone neighbor culls)
        w.set(x, y, z - 1, STONE);
        w.set(x, y, z, LEVER);
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_WALL, 3),
        );
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let plates: Vec<&&Quad> = cell.iter().filter(|q| q.tile == cobble_tile).collect();
        assert_eq!(
            plates.len(),
            5,
            "the north-culled plate the five faces the verdict keeps"
        );
        for q in &plates {
            for p in q.pts.iter() {
                assert!(
                    p[2] <= z as f32 + 0.20,
                    "the plate the north edge the y steps wear ({p:?})"
                );
            }
        }
        // THE CEILING'S OWN HANG (the south-facing lever the ceiling
        // row's y=0 the no-extra-turn keeps — the plate the cell's
        // top the x=180 wears, the rotated cullface the Down->Up turn
        // the above stone culls, the stick the hang the upper half
        // keeps)
        w.set(x, y, z - 1, AIR);
        w.set(x, y + 1, z, STONE);
        w.set(x, y, z, LEVER);
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 3),
        );
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let plates: Vec<&&Quad> = cell.iter().filter(|q| q.tile == cobble_tile).collect();
        let sticks: Vec<&&Quad> = cell.iter().filter(|q| q.tile == lever_tile).collect();
        assert_eq!(
            plates.len(),
            5,
            "the up-culled plate the five faces the ceiling verdict keeps"
        );
        for q in &plates {
            for p in q.pts.iter() {
                assert!(
                    p[1] >= y as f32 + 0.80,
                    "the plate the top edge the x=180 wears ({p:?})"
                );
            }
        }
        assert!(!sticks.is_empty(), "the ceiling stick the lever tile wears");
        for q in &sticks {
            for p in q.pts.iter() {
                assert!(
                    p[1] >= y as f32 + 0.44 && p[1] <= y as f32 + 0.99,
                    "the ceiling stick the upper half the hang keeps ({p:?})"
                );
            }
        }
        // THE Y-REMAP'S OWN 180 (the north-facing ceiling the
        // (idx+2)%4 fold the two steps wears — the stick's own vertex
        // set the south row's mirror the remap banks)
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 1),
        );
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let mut north_stick: Vec<[f32; 3]> = cell
            .iter()
            .filter(|q| q.tile == lever_tile)
            .flat_map(|q| q.pts.iter().copied())
            .collect();
        w.set_variant(
            x,
            y,
            z,
            crate::world::lever_variant(crate::world::LEVER_FACE_CEILING, 3),
        );
        let quads = mesh_world(&w, None);
        let cell = cell_quads(&quads, x, y, z);
        let mut south_stick: Vec<[f32; 3]> = cell
            .iter()
            .filter(|q| q.tile == lever_tile)
            .flat_map(|q| q.pts.iter().copied())
            .collect();
        let sort = |pts: &mut Vec<[f32; 3]>| pts.sort_by(|a, b| {
            a[1].total_cmp(&b[1])
                .then(a[0].total_cmp(&b[0]))
                .then(a[2].total_cmp(&b[2]))
        });
        sort(&mut north_stick);
        sort(&mut south_stick);
        assert_eq!(
            north_stick.len(),
            south_stick.len(),
            "the two rows the same stick geometry wear"
        );
        for (n, s) in north_stick.iter().zip(south_stick.iter()) {
            assert!(
                (n[1] - s[1]).abs() < 1e-4,
                "the y the turn never moves ({n:?} vs {s:?})"
            );
            assert!(
                (n[0] - s[0]).abs() < 1e-4,
                "the x the two steps the same 0.5-symmetric stick wears ({n:?} vs {s:?})"
            );
            assert!(
                ((n[2] - z as f32 - 0.5) + (s[2] - z as f32 - 0.5)).abs() < 1e-4,
                "the z the 180 remap the mirror wears ({n:?} vs {s:?})"
            );
        }
    }

    #[test]
    fn face_uv_orientation() {
        // side faces keep +Y at the texture top (v=0)
        for face in [0usize, 1, 4, 5] {
            assert_eq!(face_uv(face, [0.0, 1.0, 0.0])[1], 0.0, "face {face}: y=1 must be v=0");
            assert_eq!(face_uv(face, [0.0, 0.0, 0.0])[1], 1.0, "face {face}: y=0 must be v=1");
        }
        // east (+X): looking down -X, screen-right is -Z -> u = 1-z
        assert_eq!(face_uv(0, [1.0, 0.5, 0.0])[0], 1.0);
        assert_eq!(face_uv(0, [1.0, 0.5, 1.0])[0], 0.0);
        // west (-X): u = z
        assert_eq!(face_uv(1, [0.0, 0.5, 0.0])[0], 0.0);
        assert_eq!(face_uv(1, [0.0, 0.5, 1.0])[0], 1.0);
        // south (+Z): u = x; north (-Z): u = 1-x
        assert_eq!(face_uv(4, [0.0, 0.5, 1.0])[0], 0.0);
        assert_eq!(face_uv(5, [0.0, 0.5, 0.0])[0], 1.0);
        // top: u=x, v=z (north at texture top); bottom mirrors v
        assert_eq!(face_uv(2, [0.25, 1.0, 0.75]), [0.25, 0.75]);
        assert_eq!(face_uv(3, [0.25, 0.0, 0.75]), [0.25, 0.25]);
    }

    #[test]
    fn the_flower_cross_wears_the_bake_law() {
        // THE PLAINS' OWN CROSS (the autopush-411 round — the block/
        // cross bake the 45° rotY + rescale fold speaks): the flower
        // cell the 4 quads wear (the two diagonal planes × the two
        // windings), the corners the from/to inset keeps, the uv the
        // full sprite rides, the tile the flower's own atlas row banks.
        // THE DEV-27 OFFSET: the corners the audited triple rides —
        // the model-relative read the shift subtracts (the XZ row the
        // dandelion cell wears, the exact literals the audit embeds).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::DANDELION);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::DANDELION, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!((dx, dy, dz), (0.150_000_005_960_464_48f32, 0.0, -0.083_333_328_366_279_6f32));
        let flower: Vec<_> = q
            .iter()
            .filter(|quad| {
                // the vertical span (the stone's own top face at y=9 the
                // any-10 filter never rides) — the XZ row the Y the zero
                // keeps, the span the unshifted bounds wear
                quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 10.0)
            })
            .collect();
        assert_eq!(flower.len(), 4, "the two planes the two windings wear");
        for quad in &flower {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
                // THE DIAGONAL'S OWN PAIR (the plane A the (0.8, 0.8) ->
                // (15.2, 15.2) walk keeps, the plane B the mirrored run
                // answers — the rescale fold the 45° rotation undoes)
                assert!(
                    (fx - fz).abs() < 1e-6 || (fx + fz - 1.0).abs() < 1e-6,
                    "the cross plane the diagonal runs: {fx} vs {fz}"
                );
            }
            // the uv the full sprite (u spans 0..1, v spans 0..1)
            let u_span = quad.uvs.iter().map(|c| c[0]).fold(f32::MIN, f32::max)
                - quad.uvs.iter().map(|c| c[0]).fold(f32::MAX, f32::min);
            let v_span = quad.uvs.iter().map(|c| c[1]).fold(f32::MIN, f32::max)
                - quad.uvs.iter().map(|c| c[1]).fold(f32::MAX, f32::min);
            assert!((u_span - 1.0).abs() < 1e-6, "the u the full span keeps");
            assert!((v_span - 1.0).abs() < 1e-6, "the v the full span keeps");
            // the tile: the dandelion's own row (the atlas's 39th kind)
            assert_eq!(quad.tile, mc_registry::generated::atlas::FACE_TILES[39][2]);
        }
        // THE TWO WINDINGS' OWN PAIR (the reversed corner cycle the same
        // points wears — the raster culls one per view side)
        let p0: Vec<[f32; 3]> = flower[0].pts.to_vec();
        let reversed = flower
            .iter()
            .any(|other| other.pts.iter().rev().eq(p0.iter()));
        assert!(reversed, "the mirrored winding the pair keeps");
    }

    #[test]
    fn the_provider_flowers_wear_the_cross_tile_laws() {
        // THE PROVIDER'S OWN SEVEN KINDS (the autopush-414 round — the
        // noise_threshold_provider's own high_states/low_states rows the
        // atlas banks): every new flower cell the same 4 quads wear (the
        // two diagonal planes × the two windings, the corners the 0.8
        // inset keeps, the uv the full sprite rides), the tile the
        // kind's own atlas row banks (the blockstates/<flower>.json ->
        // block/<flower> -> block/cross chain the resolved sprite
        // speaks), the XZ offset the flower family's own render row
        // carries (the dev-27 law the model-relative shift keeps).
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        for kind in [
            crate::world::AZURE_BLUET,
            crate::world::OXEYE_DAISY,
            crate::world::CORNFLOWER,
            crate::world::ORANGE_TULIP,
            crate::world::RED_TULIP,
            crate::world::PINK_TULIP,
            crate::world::WHITE_TULIP,
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, STONE);
            put(&mut w, 8, 9, 8, kind);
            let q = mesh_chunk(&w, 0, 0, None);
            let (dx, dy, dz) = crate::world::plant_offset(kind, 8, 9, 8);
            let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
            assert_eq!(dy, 0.0, "the XZ row the y the zero keeps");
            let flower: Vec<_> = q
                .iter()
                .filter(|quad| {
                    quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                        && quad.pts.iter().any(|p| p[1] == 10.0)
                })
                .collect();
            assert_eq!(
                flower.len(),
                4,
                "the kind {kind} the two planes the two windings wear"
            );
            for quad in &flower {
                for p in &quad.pts {
                    let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                    assert!(
                        (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                        "the x the diagonal corner inset keeps: {fx}"
                    );
                    assert!(
                        (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                        "the z the diagonal corner inset keeps: {fz}"
                    );
                }
                // the tile: the kind's own atlas row (the resolved
                // cross sprite the provider's list order banks)
                assert_eq!(
                    quad.tile,
                    mc_registry::generated::atlas::FACE_TILES[kind as usize][2],
                    "the kind {kind} the own tile banks"
                );
            }
        }
        // THE SEVEN TILES' OWN DISTINCT ROWS: each kind its own sprite
        // index (the atlas rows the provider's one-kind-one-tile walk
        // banks — the shared-tile collapse the family never wears)
        let mut tiles = [
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::AZURE_BLUET as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::OXEYE_DAISY as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::CORNFLOWER as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::ORANGE_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::RED_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::PINK_TULIP as usize][2],
            mc_registry::generated::atlas::FACE_TILES
                [crate::world::WHITE_TULIP as usize][2],
        ];
        tiles.sort_unstable();
        for pair in tiles.windows(2) {
            assert_ne!(pair[0], pair[1], "the seven sprites the distinct rows keep");
        }
    }

    /// THE COMPARATOR'S OWN BAKES (the autopush-124 round — the four
    /// per-state derives the jar's own models answer: every state the
    /// non-empty slice keeps, the ON models the lit-tile quads carry,
    /// the subtract pair the bigger geometry the wire segments bank,
    /// the per-quad texture verdict the resolved names the derive walk
    /// serves).
    #[test]
    fn the_comparator_bakes_the_four_states_the_texture_verdicts_wear() {
        // THE FOUR STATES THE NON-EMPTY BAKES KEEP
        let off = comparator_model_quads(false, false);
        let on = comparator_model_quads(false, true);
        let sub = comparator_model_quads(true, false);
        let on_sub = comparator_model_quads(true, true);
        assert!(!off.is_empty(), "the base model the bake answers");
        assert!(!on.is_empty(), "the on model the bake answers");
        assert!(!sub.is_empty(), "the subtract model the bake answers");
        assert!(!on_sub.is_empty(), "the on-subtract model the bake answers");
        // THE ON MODELS THE WIRE GEOMETRY THE EXTRA QUADS BANK (the
        // 16-element chain the 4-element base the count pair speaks)
        assert!(
            on.len() > off.len(),
            "the lit model the wire segments the extra quads wear: {} vs {}",
            on.len(),
            off.len()
        );
        assert!(
            on_sub.len() > sub.len(),
            "the lit subtract the wire segments the extra quads wear"
        );
        // THE PER-QUAD TEXTURE VERDICT THE RESOLVED NAMES ANSWER (the
        // #top swap the comparator tiles wear, the #slab the smooth
        // stone keeps, the #unlit/#lit the torch kinds' tiles ride)
        let names = |quads: &[mc_registry::element_quads::Quad]| {
            let mut set = std::collections::BTreeSet::new();
            for q in quads {
                set.insert(q.texture);
            }
            set
        };
        let off_names = names(off);
        assert!(
            off_names.contains("block/comparator"),
            "the base top the comparator tile names"
        );
        assert!(
            off_names.contains("block/smooth_stone"),
            "the slab the smooth stone names"
        );
        let on_names = names(on);
        assert!(
            on_names.contains("block/comparator_on"),
            "the lit top the comparator_on tile names"
        );
        // THE UV TEXEL FOLD (the raw 0..16 the derive walk carries —
        // the mesh lane's own /16 fold the unit square speaks: the
        // raw texel rows the bounds the verdict reads)
        for q in off.iter().chain(on.iter()) {
            for uv in q.uvs.iter() {
                assert!(
                    (0.0..=16.0).contains(&uv[0]) && (0.0..=16.0).contains(&uv[1]),
                    "the raw uv the texel square keeps: {uv:?}"
                );
            }
        }
    }

    /// THE REPEATER'S OWN BAKES (the autopush-126 round — the sixteen
    /// per-state derives the jar's own models answer: every state the
    /// non-empty slice keeps, the ON models the lit-tile + halo quads
    /// carry, the locked rows the bedrock bar the freeze wears, the
    /// delay dial the slider z the four positions the geometry moves,
    /// the per-quad texture verdict the resolved names the derive
    /// walk serves).
    #[test]
    fn the_repeater_bakes_the_sixteen_states_the_texture_verdicts_wear() {
        // THE SIXTEEN STATES THE NON-EMPTY BAKES KEEP (the powered ×
        // dial × lock triple the blockstate rows' own variants answer)
        for powered in [false, true] {
            for delay in 1..=4u8 {
                for locked in [false, true] {
                    let quads = repeater_model_quads(powered, delay, locked);
                    assert!(
                        !quads.is_empty(),
                        "the ({powered},{delay},{locked}) state the bake answers"
                    );
                }
            }
        }
        // THE SLAB BASE THE GROUND CULL KEEPS (the down cullface the
        // below cell's opaque verdict culls — the DiodeBlock family's
        // own shape the lane wears)
        let off = repeater_model_quads(false, 1, false);
        assert!(
            off.iter()
                .any(|q| q.cullface == Some(mc_registry::element_quads::Direction::Down)),
            "the slab bottom the down cullface wears"
        );
        // THE ON MODELS THE HALO QUADS THE LIT GEOMETRY BANKS (the
        // twelve glow faces the star the count gap speaks)
        let on = repeater_model_quads(true, 1, false);
        assert!(
            on.len() > off.len(),
            "the lit model the halo star the extra quads wear: {} vs {}",
            on.len(),
            off.len()
        );
        // THE DELAY DIAL THE SLIDER Z THE GEOMETRY MOVES (the moving
        // torch the z = 4 + 2*delay lane the four positions wear —
        // the delay-1 the mid z, the delay-4 the far z the quad sets
        // differ)
        let d4 = repeater_model_quads(false, 4, false);
        let moved = off
            .iter()
            .zip(d4.iter())
            .any(|(a, b)| (a.vertices[0][2] - b.vertices[0][2]).abs() > 1e-6);
        assert!(moved, "the dial the slider z the geometry moves");
        // THE LOCKED ROW THE BEDROCK BAR WEARS (the #lock ref the
        // freeze bar the per-quad verdict answers, the moving torch
        // the bar the geometry replaces)
        let locked = repeater_model_quads(false, 1, true);
        let names = |quads: &[mc_registry::element_quads::Quad]| {
            let mut set = std::collections::BTreeSet::new();
            for q in quads {
                set.insert(q.texture);
            }
            set
        };
        let locked_names = names(locked);
        assert!(
            locked_names.contains("block/bedrock"),
            "the lock bar the bedrock tile names"
        );
        // THE PER-QUAD TEXTURE VERDICT THE RESOLVED NAMES ANSWER (the
        // #top swap the repeater tiles wear, the #slab the smooth
        // stone keeps, the #unlit/#lit the torch kinds' tiles ride)
        let off_names = names(off);
        assert!(
            off_names.contains("block/repeater"),
            "the base top the repeater tile names"
        );
        assert!(
            off_names.contains("block/smooth_stone"),
            "the slab the smooth stone names"
        );
        assert!(
            off_names.contains("block/redstone_torch_off"),
            "the dark torches the unlit tile names"
        );
        let on_names = names(on);
        assert!(
            on_names.contains("block/repeater_on"),
            "the lit top the repeater_on tile names"
        );
        assert!(
            on_names.contains("block/redstone_torch"),
            "the lit torches the lit tile names"
        );
    }

    /// THE OBSERVER'S OWN BAKE AND LANE (the autopush-127 round — the
    /// ONE full-cube bake the jar's own model rows wear, the resolved
    /// names the per-quad verdict reads, the all-six cullfaces the
    /// cube law keeps, the per-face tiles the facing byte picks, the
    /// powered back the verdict's #bottom seat swaps).
    #[test]
    fn the_observer_bakes_the_full_cube_the_tile_verdicts_wear() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE BAKE'S OWN SHAPE (the one full-cube chain the six faces
        // the jar's own model speaks)
        let quads = observer_model_quads();
        assert_eq!(quads.len(), 6, "the full cube the six faces banks");
        // THE RESOLVED NAMES THE PER-QUAD VERDICT READS (the #top/
        // #side/#front/#bottom refs the merged map answers)
        let names: std::collections::BTreeSet<&str> = quads.iter().map(|q| q.texture).collect();
        for want in [
            "block/observer_top",
            "block/observer_side",
            "block/observer_front",
            "block/observer_back",
        ] {
            assert!(names.contains(want), "the {want} ref the derive answers");
        }
        // THE ALL-SIX CULLFACES THE CUBE LAW KEEPS (the jar's own
        // model rows the cullface pairs speak)
        for cf in [
            mc_registry::element_quads::Direction::Down,
            mc_registry::element_quads::Direction::Up,
            mc_registry::element_quads::Direction::North,
            mc_registry::element_quads::Direction::South,
            mc_registry::element_quads::Direction::West,
            mc_registry::element_quads::Direction::East,
        ] {
            assert!(
                quads.iter().any(|q| q.cullface == Some(cf)),
                "the {cf:?} face the cullface law keeps"
            );
        }
        // THE MESHED CELL THE PER-FACE TILES THE FACING BYTE PICKS
        // (the facing east — the east face the eye tile wears, the
        // west face the back keeps, the up face the top tile, the
        // flanks the side tile, the down face the floor culls)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::OBSERVER);
        w.set_variant(8, 9, 8, crate::world::observer_variant(5, false));
        let q = mesh_chunk(&w, 0, 0, None);
        // THE CELL'S OWN SELECT (the quads the observer's own bounds
        // bank — the stone seat's top the y=9.0 plane the all-points
        // law refuses, the observer's faces the >9.0 arm answers)
        let cell: Vec<_> = q
            .iter()
            .filter(|quad| quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4))
            .collect();
        assert_eq!(
            cell.len(),
            5,
            "the six faces the culled bottom the five keeps"
        );
        // THE PLANE'S OWN PROBE (the quad the WHOLE face the plane
        // covers — the all-four-points law the axis probe keeps)
        let plane_tile = |axis: usize, value: f32| -> u16 {
            cell.iter()
                .find(|quad| quad.pts.iter().all(|p| (p[axis] - value).abs() < 1e-4))
                .map(|quad| quad.tile)
                .unwrap_or(u16::MAX)
        };
        assert_eq!(
            plane_tile(0, 9.0),
            gen_atlas::TILE_OBSERVER_FRONT,
            "the east face the eye tile the FACING arm wears"
        );
        assert_eq!(
            plane_tile(0, 8.0),
            gen_atlas::TILE_OBSERVER_BACK,
            "the west face the back tile the output arm wears"
        );
        assert_eq!(
            plane_tile(1, 10.0),
            gen_atlas::TILE_OBSERVER_TOP,
            "the up face the top tile the model's own rows wear"
        );
        // THE FLANKS THE SIDE TILE (the north face the z=8.0 plane
        // the side tile answers)
        assert_eq!(
            plane_tile(2, 8.0),
            gen_atlas::TILE_OBSERVER_SIDE,
            "the flank the side tile the model's own rows wear"
        );
        // THE POWERED BACK THE VERDICT SWAPS (the #bottom seat the
        // _on tile the lit pulse wears — the eye/top/side tiles the
        // same verdict keeps)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        put(&mut w2, 8, 9, 8, crate::world::OBSERVER);
        w2.set_variant(8, 9, 8, crate::world::observer_variant(5, true));
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let cell2: Vec<_> = q2
            .iter()
            .filter(|quad| quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4))
            .collect();
        let west: Vec<_> = cell2
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (7.99..=8.01).contains(&p[0])))
            .collect();
        assert_eq!(west.len(), 1, "the west face the one quad keeps");
        assert_eq!(
            west[0].tile,
            gen_atlas::TILE_OBSERVER_BACK_ON,
            "the powered back the _on tile the verdict swaps"
        );
        let east: Vec<_> = cell2
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (8.99..=9.01).contains(&p[0])))
            .collect();
        assert_eq!(
            east[0].tile,
            gen_atlas::TILE_OBSERVER_FRONT,
            "the powered eye the same front tile keeps"
        );
    }

    /// THE PISTON'S OWN BAKES AND FOLDS (the autopush-128 round — the
    /// three base chains + the head pair the blockstate rows name:
    /// the retracted full cube the six faces the template_piston
    /// parent serves, the extended body the inside face the
    /// piston_inner tile answers, the head the plate + shaft the
    /// ten quads the overhang the base cell's border wears, the
    /// sticky platform the kind's own tile swaps, the y-fold the
    /// facing byte picks).
    #[test]
    fn the_piston_bakes_the_states_the_tiles_the_folds_wear() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE BAKES' OWN SHAPES (the retracted full cube the six
        // faces, the extended body the six, the head the plate six +
        // the shaft four the ten wears)
        let normal = piston_model_quads(false, false);
        assert_eq!(normal.len(), 6, "the retracted full cube the six banks");
        let sticky = piston_model_quads(true, false);
        assert_eq!(sticky.len(), 6, "the sticky full cube the six keeps");
        let extended = piston_model_quads(false, true);
        assert_eq!(extended.len(), 6, "the extended body the six keeps");
        let head = piston_head_model_quads(false);
        assert_eq!(
            head.len(),
            10,
            "the plate six + the shaft four the ten wears"
        );
        // THE RESOLVED NAMES THE PER-QUAD VERDICT READS (the
        // #platform/#side/#bottom refs the merged map answers, the
        // #inside ref the extended body's open face carries, the
        // #unsticky ref the head's plate back wears)
        let names: std::collections::BTreeSet<&str> = normal.iter().map(|q| q.texture).collect();
        for want in [
            "block/piston_top",
            "block/piston_side",
            "block/piston_bottom",
        ] {
            assert!(names.contains(want), "the {want} ref the derive answers");
        }
        let sticky_names: std::collections::BTreeSet<&str> =
            sticky.iter().map(|q| q.texture).collect();
        assert!(
            sticky_names.contains("block/piston_top_sticky"),
            "the sticky platform the kind's own ref wears"
        );
        let ext_names: std::collections::BTreeSet<&str> =
            extended.iter().map(|q| q.texture).collect();
        assert!(
            ext_names.contains("block/piston_inner"),
            "the #inside ref the extended body carries"
        );
        // THE CULLFACES THE MODEL ROWS SPEAK (the full cube's six —
        // the extended body the open front the north cullface never
        // wears, the head plate's five the shaft's none keeps)
        for cf in [
            mc_registry::element_quads::Direction::Down,
            mc_registry::element_quads::Direction::Up,
            mc_registry::element_quads::Direction::North,
            mc_registry::element_quads::Direction::South,
            mc_registry::element_quads::Direction::West,
            mc_registry::element_quads::Direction::East,
        ] {
            assert!(
                normal.iter().any(|q| q.cullface == Some(cf)),
                "the {cf:?} face the cube cullface keeps"
            );
        }
        assert!(
            !extended
                .iter()
                .any(|q| q.cullface == Some(mc_registry::element_quads::Direction::North)),
            "the extended front the open face the cullface refuses"
        );
        // THE MESHED RETRACTED CELL THE FACING FOLD PICKS (the facing
        // east — the platform face the model north the y=90 turn the
        // EAST face wears, the bottom the model south the west keeps,
        // the up face the side tile, the down face the floor culls)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::PISTON);
        w.set_variant(8, 9, 8, crate::world::piston_variant(5, false));
        let q = mesh_chunk(&w, 0, 0, None);
        let cell: Vec<_> = q
            .iter()
            .filter(|quad| quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4))
            .collect();
        assert_eq!(
            cell.len(),
            5,
            "the six faces the culled bottom the five keeps"
        );
        let plane_tile = |axis: usize, value: f32| -> u16 {
            cell.iter()
                .find(|quad| quad.pts.iter().all(|p| (p[axis] - value).abs() < 1e-4))
                .map(|quad| quad.tile)
                .unwrap_or(u16::MAX)
        };
        assert_eq!(
            plane_tile(0, 9.0),
            gen_atlas::TILE_PISTON_TOP,
            "the east face the platform tile the y=90 fold wears"
        );
        assert_eq!(
            plane_tile(0, 8.0),
            gen_atlas::TILE_PISTON_BOTTOM,
            "the west face the bottom tile the model's south keeps"
        );
        assert_eq!(
            plane_tile(1, 10.0),
            gen_atlas::TILE_PISTON_SIDE,
            "the up face the side tile the model's own rows wear"
        );
        // THE STICKY'S OWN SWAP (the platform face the sticky tile
        // wears — the kind the model pick serves)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, STONE);
        put(&mut w3, 8, 9, 8, crate::world::STICKY_PISTON);
        w3.set_variant(8, 9, 8, crate::world::piston_variant(5, false));
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let cell3: Vec<_> = q3
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4)
                    && quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-4)
            })
            .collect();
        assert_eq!(cell3.len(), 1, "the east face the one quad keeps");
        assert_eq!(
            cell3[0].tile,
            gen_atlas::TILE_PISTON_TOP_STICKY,
            "the sticky platform the kind's own tile wears"
        );
        // THE EXTENDED BODY'S OWN INSIDE (the facing east — the open
        // front face the model north z=4 plane the y=90 fold the x=8.75
        // plane wears, the inside tile the body's open face answers)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, STONE);
        put(&mut w4, 8, 9, 8, crate::world::PISTON);
        w4.set_variant(8, 9, 8, crate::world::piston_variant(5, true));
        let q4 = mesh_chunk(&w4, 0, 0, None);
        let inside: Vec<_> = q4
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (p[0] - 8.75).abs() < 1e-4)
                    && quad.pts.iter().all(|p| p[1] > 9.0 - 1e-4)
            })
            .collect();
        assert_eq!(inside.len(), 1, "the body's open face the one quad keeps");
        assert_eq!(
            inside[0].tile,
            gen_atlas::TILE_PISTON_INNER,
            "the extended inside the piston_inner tile wears"
        );
        // THE HEAD'S OWN OVERHANG (the facing east — the plate the
        // east edge the platform tile wears, the shaft the WEST the
        // base cell's own border the overhang serves, the sticky head
        // the sticky platform swap keeps)
        let mut w5 = empty_world();
        put(&mut w5, 8, 8, 8, STONE);
        put(&mut w5, 8, 9, 8, crate::world::PISTON_HEAD);
        w5.set_variant(8, 9, 8, crate::world::piston_head_variant(5, false));
        let q5 = mesh_chunk(&w5, 0, 0, None);
        let head_cell: Vec<_> = q5
            .iter()
            .filter(|quad| quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4))
            .collect();
        let plate: Vec<_> = head_cell
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-4))
            .collect();
        assert_eq!(plate.len(), 1, "the plate face the one quad keeps");
        assert_eq!(
            plate[0].tile,
            gen_atlas::TILE_PISTON_TOP,
            "the plate face the platform tile the head's own rows wear"
        );
        let overhang: Vec<_> = head_cell
            .iter()
            .filter(|quad| quad.pts.iter().any(|p| p[0] < 8.0 - 1e-4))
            .collect();
        assert!(
            !overhang.is_empty(),
            "the shaft the west overhang the base cell's border serves"
        );
        w5.set_variant(8, 9, 8, crate::world::piston_head_variant(5, true));
        let q6 = mesh_chunk(&w5, 0, 0, None);
        let plate6: Vec<_> = q6
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] > 9.0 + 1e-4)
                    && quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-4)
            })
            .collect();
        assert_eq!(
            plate6[0].tile,
            gen_atlas::TILE_PISTON_TOP_STICKY,
            "the sticky head the platform swap wears"
        );
    }

    /// walk keeps, the down face the solid ground culls (the cullface
    /// law the below cell's opaque verdict answers), the vertex bounds
    /// the [7,0,7]-[9,10,9] box keeps, the uv windows the texel rows
    /// /16 fold wears, the tile the kind's own atlas row banks, the
    /// neighbors' faces the non-occluding verdict draws.
    #[test]
    fn the_torch_lane_wears_the_element_laws() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE GROUNDED TORCH: the stick on stone — five quads survive
        // (the down face the below stone culls), the bounds the box
        // keeps, the tile the torch row banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::TORCH);
        let q = mesh_chunk(&w, 0, 0, None);
        let stick: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (9.0..=9.7).contains(&p[1]))
                    && quad.pts.iter().all(|p| {
                        (8.0 + 7.0 / 16.0..=8.0 + 9.0 / 16.0).contains(&p[0])
                    })
            })
            .collect();
        assert_eq!(stick.len(), 5, "the six faces the culled bottom the five keeps");
        let torch_tile = gen_atlas::FACE_TILES[crate::world::TORCH as usize][2];
        assert_ne!(torch_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        for quad in &stick {
            assert_eq!(quad.tile, torch_tile, "the torch's own tile row");
            assert!(!quad.translucent, "the stick the opaque lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1] - 9.0, p[2] - 8.0);
                assert!(
                    (7.0 / 16.0..=9.0 / 16.0).contains(&fx)
                        && (7.0 / 16.0..=9.0 / 16.0).contains(&fz)
                        && (0.0..=10.0 / 16.0).contains(&fy),
                    "the [7,0,7]-[9,10,9] box the corners keep: ({fx},{fy},{fz})"
                );
            }
            for uv in &quad.uvs {
                assert!(
                    (0.0..=1.0).contains(&uv[0]) && (0.0..=1.0).contains(&uv[1]),
                    "the /16 fold the tile space keeps: {uv:?}"
                );
            }
        }
        // THE DOWN CULL'S OWN ABSENCE — the floating torch (the honest
        // canSurvive-fail cell the test meshes anyway) the SIX quads
        // keep: the below air the cullface verdict never fires
        let mut w2 = empty_world();
        put(&mut w2, 8, 9, 8, crate::world::TORCH);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let floating: Vec<_> = q2
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (9.0..=9.7).contains(&p[1])))
            .collect();
        assert_eq!(floating.len(), 6, "the below air the bottom face draws");
        // THE SUB-SPRITE'S OWN WINDOW (the texel rows the derive walk
        // keeps verbatim: the four sides [7,6,9,16] the /16 fold
        // wears — u 7/16..9/16, v 6/16..1.0; the up face [7,6,9,8] —
        // v 6/16..8/16): the side quads the narrower u band keeps
        let side_band: Vec<_> = stick
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| (p[1] - 9.0).abs() < 1e-6)
                    && quad.pts.iter().any(|p| (p[1] - 9.0 - 10.0 / 16.0).abs() < 1e-6)
            })
            .collect();
        assert_eq!(side_band.len(), 4, "the four sides the y extremes name");
        for quad in &side_band {
            for uv in &quad.uvs {
                assert!(
                    (7.0 / 16.0..=9.0 / 16.0).contains(&uv[0]),
                    "the side's u the [7,9] window keeps: {uv:?}"
                );
            }
        }
        // THE NEIGHBORS' OWN FACES — the torch cell the solid block
        // draws against (the non-occluding verdict the stick's
        // noCollision row keeps): the stone's west face at the
        // shared plane renders
        let mut w3 = empty_world();
        put(&mut w3, 8, 9, 8, crate::world::TORCH);
        put(&mut w3, 9, 9, 8, STONE);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        assert!(
            q3.iter().any(|quad| {
                quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6)
                    && quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
            }),
            "the stone's face the torch cell draws against"
        );
    }

    #[test]
    fn the_fence_lane_selects_the_post_and_the_connected_sides() {
        // THE FENCE LANE'S OWN SELECTION (the dev-55 round — the
        // blockstates {wood}_fence.json's multipart the
        // fence_parts walk parses, the same select_parts walk the
        // fire/bamboo tests pin): the all-false state the
        // UNCONDITIONAL post alone speaks, the connected state the
        // post + the side arms the when-rows answer, the y
        // quarters the four groups carry (north 0 / east 90 /
        // south 180 / west 270), the derive walk the post
        // [6,0,6]->[10,16,10] element the side's TWO bar elements
        // resolves.
        let lone: Vec<_> = select_parts(
            fence_parts(crate::world::OAK_FENCE),
            &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false")],
        );
        assert_eq!(lone.len(), 1, "the all-false state the post alone wears");
        assert!(lone[0].model.contains("oak_fence_post"), "the post the unconditional apply banks");
        assert_eq!(lone[0].y, 0, "the post the no-rotation row wears");
        let north: Vec<_> = select_parts(
            fence_parts(crate::world::OAK_FENCE),
            &[("north", "true"), ("east", "false"), ("south", "false"), ("west", "false")],
        );
        assert_eq!(north.len(), 2, "the north connection the post + the north arm wears");
        let sides: Vec<_> = north.iter().filter(|a| a.model.contains("oak_fence_side")).collect();
        assert_eq!(sides.len(), 1, "the ONE side arm the north when-row answers");
        assert_eq!(sides[0].y, 0, "the north arm the y=0 row wears");
        let all: Vec<_> = select_parts(
            fence_parts(crate::world::OAK_FENCE),
            &[("north", "true"), ("east", "true"), ("south", "true"), ("west", "true")],
        );
        assert_eq!(all.len(), 5, "the full connection the post + the four arms wears");
        // THE FOUR QUARTERS the side arms carry (the jar's own y
        // table: north 0 / east 90 / south 180 / west 270)
        for (y, dir) in [(0u32, "north"), (90, "east"), (180, "south"), (270, "west")] {
            let state = [
                ("north", if dir == "north" { "true" } else { "false" }),
                ("east", if dir == "east" { "true" } else { "false" }),
                ("south", if dir == "south" { "true" } else { "false" }),
                ("west", if dir == "west" { "true" } else { "false" }),
            ];
            let one: Vec<_> = select_parts(fence_parts(crate::world::OAK_FENCE), &state);
            let sides: Vec<_> = one.iter().filter(|a| a.model.contains("oak_fence_side")).collect();
            assert_eq!(sides.len(), 1, "the {dir} arm the when-row alone answers");
            assert_eq!(sides[0].y, y, "the {dir} arm the y={y} quarter wears");
        }
        // THE NETHER SEAM'S OWN COLUMN (the stone fence the SAME
        // multipart shape the nether_bricks master wears)
        let nether: Vec<_> = select_parts(
            fence_parts(crate::world::NETHER_BRICK_FENCE),
            &[("north", "true"), ("east", "false"), ("south", "false"), ("west", "false")],
        );
        assert_eq!(nether.len(), 2, "the nether brick fence the same post + arm shape wears");
        assert!(
            nether.iter().all(|a| a.model.contains("nether_brick_fence")),
            "the nether models the own column banks"
        );
        // THE DERIVE WALK'S OWN GEOMETRY (the fence_models pair the
        // template parents resolve — the post's ONE element, the
        // side's TWO bar elements)
        let (post, side) = fence_models(crate::world::OAK_FENCE);
        assert_eq!(post.len(), 6, "the post's six faces the ONE element wears");
        assert_eq!(side.len(), 10, "the side's TWO bar elements the ten faces wear");
        // the post's x/z band the [6,0,6]->[10,16,10] element speaks
        let xs: Vec<f32> = post.iter().flat_map(|q| q.vertices.iter().map(|v| v[0])).collect();
        assert!(
            xs.iter().all(|x| (x - 0.375).abs() < 1e-6 || (x - 0.625).abs() < 1e-6),
            "the post's 4/16 centered x band the element banks"
        );
    }

    #[test]
    fn the_wall_lane_selects_the_post_and_the_connected_sides() {
        // THE WALL LANE'S OWN SELECTION (the dev-56 round — the
        // blockstates {base}_wall.json's multipart the wall_parts
        // walk parses, the same select_parts walk the fire/bamboo/
        // fence tests pin): the lone state the up=true post alone
        // speaks, the connected state the post + the side arms the
        // when-rows answer, the LOW/TALL words the side vs side_tall
        // models the when-rows pick, the y quarters the four groups
        // carry (north 0 / east 90 / south 180 / west 270), the
        // derive walk the post [4,0,4]->[12,16,12] element the
        // side's ONE bar element resolves.
        let lone: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "none"),
                ("east", "none"),
                ("south", "none"),
                ("west", "none"),
                ("up", "true"),
            ],
        );
        assert_eq!(lone.len(), 1, "the lone state the post alone wears");
        assert!(
            lone[0].model.contains("cobblestone_wall_post"),
            "the post the up=true apply banks"
        );
        assert_eq!(lone[0].y, 0, "the post the no-rotation row wears");
        // the up=false state the NOTHING wears (the post the
        // conditional apply the verdict refuses)
        let flat: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "none"),
                ("east", "none"),
                ("south", "none"),
                ("west", "none"),
                ("up", "false"),
            ],
        );
        assert_eq!(flat.len(), 0, "the up=false state the empty fan wears");
        // THE LOW NORTH ARM the side model picks (the when-row the
        // "low" word answers)
        let north_low: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "low"),
                ("east", "none"),
                ("south", "none"),
                ("west", "none"),
                ("up", "false"),
            ],
        );
        assert_eq!(
            north_low.len(),
            1,
            "the north-low state the side arm alone wears"
        );
        assert!(
            north_low[0].model.contains("cobblestone_wall_side"),
            "the side arm the low when-row banks"
        );
        assert!(
            !north_low[0].model.contains("tall"),
            "the low word the tall model never picks"
        );
        assert_eq!(north_low[0].y, 0, "the north arm the y=0 row wears");
        // THE TALL NORTH ARM the side_tall model picks
        let north_tall: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "tall"),
                ("east", "none"),
                ("south", "none"),
                ("west", "none"),
                ("up", "false"),
            ],
        );
        assert_eq!(
            north_tall.len(),
            1,
            "the north-tall state the tall arm alone wears"
        );
        assert!(
            north_tall[0].model.contains("cobblestone_wall_side_tall"),
            "the tall arm the tall when-row banks"
        );
        // THE FULL CROSS the post + the four arms wears (the all-low
        // state the five applies the when-rows answer)
        let all: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "low"),
                ("east", "low"),
                ("south", "low"),
                ("west", "low"),
                ("up", "true"),
            ],
        );
        assert_eq!(
            all.len(),
            5,
            "the full connection the post + the four strips wears"
        );
        // THE FOUR QUARTERS the side arms carry (the jar's own y
        // table: north 0 / east 90 / south 180 / west 270)
        for (y, dir, word) in [
            (0u32, "north", "low"),
            (90, "east", "tall"),
            (180, "south", "low"),
            (270, "west", "tall"),
        ] {
            let state = [
                ("north", if dir == "north" { word } else { "none" }),
                ("east", if dir == "east" { word } else { "none" }),
                ("south", if dir == "south" { word } else { "none" }),
                ("west", if dir == "west" { word } else { "none" }),
                ("up", "false"),
            ];
            let one: Vec<_> = select_parts(wall_parts(crate::world::COBBLESTONE_WALL), &state);
            assert_eq!(one.len(), 1, "the {dir} arm the when-row alone answers");
            assert_eq!(one[0].y, y, "the {dir} arm the y={y} quarter wears");
        }
        // THE MIXED ARMS the low and tall pairs together wear (the
        // two-word vocabulary the when-rows independently answer)
        let mixed: Vec<_> = select_parts(
            wall_parts(crate::world::COBBLESTONE_WALL),
            &[
                ("north", "low"),
                ("east", "tall"),
                ("south", "low"),
                ("west", "tall"),
                ("up", "true"),
            ],
        );
        assert_eq!(
            mixed.len(),
            5,
            "the mixed state the post + the two low + the two tall wears"
        );
        assert_eq!(
            mixed.iter().filter(|a| a.model.contains("_side_tall")).count(),
            2,
            "the tall pair the east/west rows wear"
        );
        // THE MOSSY SIBLING'S OWN COLUMN (the second kind the SAME
        // multipart shape the mossy_cobblestone master wears)
        let mossy: Vec<_> = select_parts(
            wall_parts(crate::world::MOSSY_COBBLESTONE_WALL),
            &[
                ("north", "tall"),
                ("east", "none"),
                ("south", "none"),
                ("west", "none"),
                ("up", "true"),
            ],
        );
        assert_eq!(mossy.len(), 2, "the mossy wall the same post + arm shape wears");
        assert!(
            mossy.iter().all(|a| a.model.contains("mossy_cobblestone_wall")),
            "the mossy models the own column banks"
        );
        // THE DERIVE WALK'S OWN GEOMETRY (the wall_models triple the
        // template parents resolve — the post's ONE element SIX
        // faces, the side's ONE bar element FIVE faces [the south
        // face the element never defines — the cullface law the
        // connection edge keeps], the tall's same five)
        let (post, side, tall) = wall_models(crate::world::COBBLESTONE_WALL);
        assert_eq!(post.len(), 6, "the post's six faces the ONE element wears");
        assert_eq!(side.len(), 5, "the side's ONE bar element the five faces wear");
        assert_eq!(tall.len(), 5, "the tall's ONE bar element the five faces wear");
        // the post's x/z band the [4,0,4]->[12,16,12] element speaks
        let xs: Vec<f32> = post.iter().flat_map(|q| q.vertices.iter().map(|v| v[0])).collect();
        assert!(
            xs.iter().all(|x| (x - 0.25).abs() < 1e-6 || (x - 0.75).abs() < 1e-6),
            "the post's 8/16 centered x band the element banks"
        );
        // the tall bar the y 0..16 band the full-height element banks
        // (the side bar the y 0..14 the low element wears)
        let tall_ys: Vec<f32> = tall.iter().flat_map(|q| q.vertices.iter().map(|v| v[1])).collect();
        assert!(
            tall_ys.iter().all(|y| (*y - 0.0).abs() < 1e-6 || (*y - 1.0).abs() < 1e-6),
            "the tall bar the 0..16 y band the element banks"
        );
        let side_ys: Vec<f32> = side.iter().flat_map(|q| q.vertices.iter().map(|v| v[1])).collect();
        assert!(
            side_ys.iter().all(|y| (*y - 0.0).abs() < 1e-6 || (*y - 0.875).abs() < 1e-6),
            "the low bar the 0..14 y band the element banks"
        );
    }

    #[test]
    fn the_fire_lane_selects_the_real_multipart_parts() {
        // THE SELECTION'S OWN REGIMES (the real blockstate engine the
        // lane rides — parse("fire")/parse("soul_fire") over the jar's
        // own columns, the same select_parts walk the fence/wire tests
        // pin): the freestanding all-false state the FULL fan speaks
        // (the floor pair via the all-false condition, all four side
        // groups each via its OR's all-false arm, the y quarters the
        // four groups carry), the single-wall state the wall's own
        // group alone answers, the up state the up pair answers.
        let freestanding: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "false")]);
        assert_eq!(freestanding.len(), 18, "the floor pair + the four side groups the all-false arms wear");
        let floors = freestanding.iter().filter(|a| a.model.contains("fire_floor")).count();
        let sides = freestanding.iter().filter(|a| a.model.contains("fire_side")).count();
        let ups = freestanding.iter().filter(|a| a.model.contains("fire_up")).count();
        assert_eq!((floors, sides, ups), (2, 16, 0), "the floor pair, the 4x4 side stack, no up pair");
        // THE FOUR QUARTERS the side groups carry — north y=0, east
        // y=90, south y=180, west y=270 (the jar's own apply rows)
        for (y, quarter_count) in [(0u32, 4usize), (90, 4), (180, 4), (270, 4)] {
            assert_eq!(
                freestanding.iter().filter(|a| a.model.contains("fire_side") && a.y == y).count(),
                quarter_count,
                "the y={y} quarter the group's four side models wear"
            );
        }
        // THE EAST WALL ALONE — the floating fire's log-adjacent state:
        // the floor's all-false condition refuses, the other groups'
        // OR arms refuse, the east group's own arm answers at y=90
        let east: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "true"), ("south", "false"), ("west", "false"), ("up", "false")]);
        assert_eq!(east.len(), 4, "the east group's four side models alone");
        assert!(east.iter().all(|a| a.model.contains("fire_side") && a.y == 90));
        // THE UP PAIR ALONE — the burnable-above state: the up group's
        // four models at y=0, the floor and the sides all refuse
        let up: Vec<_> = select_parts(fire_parts(false), &[("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "true")]);
        assert_eq!(up.len(), 4, "the up group's four models alone");
        assert!(up.iter().all(|a| a.model.contains("fire_up") && a.y == 0));
        // THE SOUL COLUMN'S OWN UNCONDITIONAL FAN — no when anywhere:
        // ANY state the full 18-applies fan answers, the up pair never
        for state in [
            [("north", "false"), ("east", "false"), ("south", "false"), ("west", "false"), ("up", "false")],
            [("north", "true"), ("east", "true"), ("south", "true"), ("west", "true"), ("up", "true")],
        ] {
            let soul: Vec<_> = select_parts(fire_parts(true), &state);
            assert_eq!(soul.len(), 18, "the soul fan the no-when column always speaks");
            assert!(soul.iter().all(|a| !a.model.contains("up")), "the up pair the soul column never wears");
        }
    }

    #[test]
    fn the_bamboo_the_multipart_the_arms_select() {
        // THE SELECTION'S OWN ARMS (the real blockstate engine the lane
        // rides — parse("bamboo") over the jar's own multipart, the
        // same select_parts walk the fire test pins): the four parts
        // the blockstate's own array banks (the age0 fours, the age1
        // fours, the small-leaves arm, the large-leaves arm).
        assert_eq!(bamboo_parts().len(), 4, "the four parts the blockstate banks");
        // THE DEFAULT STATE'S OWN VERDICT (age 0, leaves NONE — the
        // kind-only vocabulary the placement law serves): the four
        // overlapping age0 stalks ALONE (the leaves arms the NONE
        // verdict never picks)
        let default_state: Vec<_> =
            select_parts(bamboo_parts(), &[("age", "0"), ("leaves", "none")]);
        assert_eq!(default_state.len(), 4, "the four age0 stalks the default state wears");
        assert!(
            default_state
                .iter()
                .all(|a| a.model.contains("bamboo") && a.model.ends_with("_age0")),
            "the age0 fours the default arm picks: {:?}",
            default_state.iter().map(|a| a.model).collect::<Vec<_>>()
        );
        assert!(
            default_state.iter().all(|a| a.y == 0),
            "the applies the rotation never carries (the bare models the jar speaks)"
        );
        // THE AGE1 ARM the thick fours picks (the growth the offline
        // client never runs — the arms the selection tests pin)
        let age1: Vec<_> = select_parts(bamboo_parts(), &[("age", "1"), ("leaves", "none")]);
        assert_eq!(age1.len(), 4, "the four age1 stalks the age arm picks");
        assert!(age1.iter().all(|a| a.model.ends_with("_age1")));
        // THE LEAVES ARMS the pairs pick (the small and the large the
        // blockstate's own when-conditions serve)
        let small: Vec<_> = select_parts(bamboo_parts(), &[("age", "0"), ("leaves", "small")]);
        assert_eq!(small.len(), 5, "the four stalks + the small pair's model");
        assert_eq!(
            small.iter().filter(|a| a.model.ends_with("bamboo_small_leaves")).count(),
            1,
            "the small-leaves arm the when verdict picks"
        );
        let large: Vec<_> = select_parts(bamboo_parts(), &[("age", "0"), ("leaves", "large")]);
        assert_eq!(large.len(), 5, "the four stalks + the large pair's model");
        assert_eq!(
            large.iter().filter(|a| a.model.ends_with("bamboo_large_leaves")).count(),
            1,
            "the large-leaves arm the when verdict picks"
        );
        // THE BAKES' OWN GEOMETRY (the elements table the census banks):
        // the age0 stalks the SAME [7,0,7]->[9,16,9] box (six faces
        // each), the age1 fours the [6.5,0,6.5]->[9.5,16,9.5] box, the
        // leaves pairs the crossed planes (the shade:false faces the
        // fire lane's own law keeps)
        let quads = bamboo_model_quads();
        for m in ["block/bamboo1_age0", "block/bamboo2_age0", "block/bamboo3_age0", "block/bamboo4_age0"] {
            assert_eq!(quads.get(m).map(|v| v.len()), Some(6), "{m}: the box's six faces");
        }
        for m in ["block/bamboo1_age1", "block/bamboo2_age1", "block/bamboo3_age1", "block/bamboo4_age1"] {
            assert_eq!(quads.get(m).map(|v| v.len()), Some(6), "{m}: the box's six faces");
        }
        for m in ["block/bamboo_small_leaves", "block/bamboo_large_leaves"] {
            let q = quads.get(m).expect("the leaves pair's bake");
            assert_eq!(q.len(), 4, "{m}: the two planes the two faces each");
            assert!(q.iter().all(|d| !d.shade), "{m}: the shade:false the pairs keep");
        }
        // THE AGE0 STALKS' OWN UV STRIPS (the four masters the same box
        // wears — the side faces' u bounds the strips speak: [0..2]/
        // [3..5]/[6..8]/[9..11] the bamboo_stalk master's columns)
        for (m, u0) in [
            ("block/bamboo1_age0", 0.0f32),
            ("block/bamboo2_age0", 3.0),
            ("block/bamboo3_age0", 6.0),
            ("block/bamboo4_age0", 9.0),
        ] {
            let q = quads.get(m).expect("the stalk bake");
            // the SIDE face the full-height v band carries (uv v 0..16
            // — the down/up faces' small bands the [4..6]/[0..2] rows
            // keep)
            let side = q
                .iter()
                .find(|d| d.uvs.iter().any(|uv| uv[1] >= 16.0 - 1e-3))
                .expect("the side face");
            let min_u = side.uvs.iter().map(|uv| uv[0]).fold(f32::MAX, f32::min);
            let max_u = side.uvs.iter().map(|uv| uv[0]).fold(f32::MIN, f32::max);
            assert!((min_u - u0).abs() < 1e-4, "{m}: the strip's min u {min_u} the {u0} speaks");
            assert!((max_u - (u0 + 2.0)).abs() < 1e-4, "{m}: the strip's max u {max_u}");
        }
    }

    #[test]
    fn the_bamboo_stalk_the_lane_the_reversed_fours_wear() {
        // THE LANE'S OWN E2E (the stone floor the stalk cell wears):
        // the 20 quads the 24-face fours keep (the four down cullfaces
        // the solid ground culled — the torch lane's own law the
        // neighbor reads keep), the reversed emission the bamboo4's
        // strip the first side speaks (the vanilla LEQUAL's own
        // last-pass verdict the LESS raster the reversed order
        // serves), the tile the kind's own atlas row banks, the shade
        // term the stalk elements' default TRUE carries, the XZ offset
        // the plant walk's row rides.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        let stalk_tile = atlas::FACE_TILES[crate::world::BAMBOO as usize][2];
        let (off_x, off_y, off_z) = crate::world::plant_offset(crate::world::BAMBOO, 8, 9, 8);
        let (off_x, off_y, off_z) = (off_x as f32, off_y as f32, off_z as f32);
        // THE CELL'S OWN QUADS: the stalk box [7,0,7]->[9,16,9] + the
        // offset — the four sides' bottoms at y=9.0, the four ups at
        // y=10.0, the downs the ground culled
        let stalk: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| {
                    p[0] >= 8.0 + 7.0 / 16.0 + off_x - 1e-4
                        && p[0] <= 8.0 + 9.0 / 16.0 + off_x + 1e-4
                        && p[2] >= 8.0 + 7.0 / 16.0 + off_z - 1e-4
                        && p[2] <= 8.0 + 9.0 / 16.0 + off_z + 1e-4
                        && p[1] >= 9.0 - 1e-4
                        && p[1] <= 10.0 + 1e-4
                })
            })
            .collect();
        assert_eq!(stalk.len(), 20, "the 24-face fours the four culled downs keep: {}", stalk.len());
        assert!(
            stalk.iter().all(|quad| quad.tile == stalk_tile),
            "every quad the kind's own atlas tile wears"
        );
        assert!(stalk.iter().all(|quad| !quad.translucent && !quad.per_texel_alpha));
        // THE UP FACES the air-above draw keeps (the four models' own
        // up quads — the [13,0,15,2] uv band the masters share), the
        // colors the shade-1.0 lm-1.0 fold keeps
        let ups: Vec<_> = stalk
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (p[1] - 10.0).abs() < 1e-4))
            .collect();
        assert_eq!(ups.len(), 4, "the four models' own up quads");
        for quad in &ups {
            for c in &quad.colors {
                for v in c {
                    assert!((v - 1.0).abs() < 1e-6, "the up face the 1.0 shade wears: {v}");
                }
            }
        }
        // THE SIDE FACES the reversed emission's own winner: the FIRST
        // side quad the bamboo4_age0's [9..11] strip wears (the
        // vanilla LEQUAL's framebuffer verdict the LESS raster the
        // reversed order serves), the 0.8 shade the N/S pair keeps
        let sides: Vec<_> = stalk
            .iter()
            .filter(|quad| {
                quad.pts.iter().filter(|p| (p[1] - 9.0).abs() < 1e-4).count() == 2
                    && quad.pts.iter().filter(|p| (p[1] - 10.0).abs() < 1e-4).count() == 2
            })
            .collect();
        assert_eq!(sides.len(), 16, "the four models' own four sides each");
        let first_side = sides[0];
        let min_u = first_side.uvs.iter().map(|uv| uv[0]).fold(f32::MAX, f32::min);
        let max_u = first_side.uvs.iter().map(|uv| uv[0]).fold(f32::MIN, f32::max);
        assert!(
            (min_u - 9.0 / 16.0).abs() < 1e-4 && (max_u - 11.0 / 16.0).abs() < 1e-4,
            "the first side the bamboo4 strip [9..11] wears: {min_u}..{max_u}"
        );
        // THE SHADE TERM the N/S 0.8 the direction table keeps (the
        // lm-1.0 no-grid fold the legacy look wears)
        for c in &first_side.colors {
            for v in c {
                assert!((v - 0.8).abs() < 1e-6, "the side face the 0.8 shade wears: {v}");
            }
        }
        // THE OFFSET the pts carry (the XZ row the seed-sliced fold
        // serves — the non-zero shift the stalk's own jitter wears)
        let straight = stalk
            .iter()
            .any(|quad| quad.pts.iter().any(|p| (p[0] - (8.0 + 7.0 / 16.0 + off_x)).abs() < 1e-5));
        assert!(straight, "the pts the offset shift carries (off={off_x},{off_y},{off_z})");
        assert!(off_y == 0.0, "the XZ row the y the zero keeps");
    }

    #[test]
    fn the_bamboo_the_growth_the_leaves_the_derived_wears() {
        // THE DERIVATION'S OWN HEIGHTS (the autopush-616 round): the
        // tick-grown steady state the neighbor reads answer — the
        // leaves arms the grown plants' segments ride, the h=1 lone
        // stalk the default state keeps (the lane test's own 20-quad
        // pin the same verdict wears), the age law the b>=3 thick box
        // speaks.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        // THE H=1 LONE STALK: b=0, k=0 -> age 0, leaves none — the
        // leaves tiles ABSENT (the default state the placement law
        // serves, the grown plants the 2-tall minimum wear)
        put(&mut w, 8, 9, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        assert!(
            q.iter().all(|quad| {
                quad.tile != atlas::TILE_BAMBOO_SMALL_LEAVES
                    && quad.tile != atlas::TILE_BAMBOO_LARGE_LEAVES
            }),
            "the lone stalk the leaves never renders"
        );
        // THE H=2 PAIR (the sapling-growth product): the base the NONE
        // keeps (k=1, h=2), the top the SMALL pair rides (k=0, h=2 —
        // the 4 quads the crossed planes speak)
        put(&mut w, 8, 10, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        let smalls = |y0: f32, y1: f32| -> usize {
            q.iter()
                .filter(|quad| {
                    quad.tile == atlas::TILE_BAMBOO_SMALL_LEAVES
                        && quad.pts.iter().all(|p| p[1] >= y0 - 1e-4 && p[1] <= y1 + 1e-4)
                })
                .count()
        };
        assert_eq!(
            smalls(10.0, 11.0),
            4,
            "the h=2 top the SMALL pair renders"
        );
        assert_eq!(smalls(9.0, 10.0), 0, "the h=2 base the NONE keeps");
        assert_eq!(
            q.iter()
                .filter(|quad| quad.tile == atlas::TILE_BAMBOO_LARGE_LEAVES)
                .count(),
            0,
            "the h=2 plant the LARGE pair never wears"
        );
        // THE H=3 PLANT: the top SMALL (k=0, h=3), the middle SMALL
        // (k=1, h=3), the base NONE (k=2, h=3 — the h>=4 band the base
        // refuses)
        put(&mut w, 8, 11, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        let smalls = |y0: f32, y1: f32| -> usize {
            q.iter()
                .filter(|quad| {
                    quad.tile == atlas::TILE_BAMBOO_SMALL_LEAVES
                        && quad.pts.iter().all(|p| p[1] >= y0 - 1e-4 && p[1] <= y1 + 1e-4)
                })
                .count()
        };
        assert_eq!(smalls(11.0, 12.0), 4, "the h=3 top the SMALL pair wears");
        assert_eq!(smalls(10.0, 11.0), 4, "the h=3 middle the SMALL pair wears");
        assert_eq!(smalls(9.0, 10.0), 0, "the h=3 base the NONE keeps");
        // THE H=4 PLANT: the top the LARGE pair (k=0, h=4 — the
        // below-SMALL verdict the large branch serves), the two below
        // SMALL (k=1/k=2, h=4), the base NONE (k=3); THE AGE LAW the
        // top's b=3 the thick box renders
        put(&mut w, 8, 12, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        let in_band = |tile: u16, y0: f32, y1: f32| -> Vec<_> {
            q.iter()
                .filter(|quad| {
                    quad.tile == tile
                        && quad.pts.iter().all(|p| p[1] >= y0 - 1e-4 && p[1] <= y1 + 1e-4)
                })
                .collect()
        };
        assert_eq!(
            in_band(atlas::TILE_BAMBOO_LARGE_LEAVES, 12.0, 13.0).len(),
            4,
            "the h=4 top the LARGE pair renders"
        );
        assert_eq!(
            in_band(atlas::TILE_BAMBOO_SMALL_LEAVES, 11.0, 12.0).len(),
            4,
            "the h=4 third the SMALL pair wears"
        );
        assert_eq!(
            in_band(atlas::TILE_BAMBOO_SMALL_LEAVES, 10.0, 11.0).len(),
            4,
            "the h=4 second the SMALL pair wears"
        );
        assert_eq!(
            in_band(atlas::TILE_BAMBOO_SMALL_LEAVES, 9.0, 10.0).len(),
            0,
            "the h=4 base the NONE keeps"
        );
        // THE THICK BOX the top's age1 fours render (the b=3 the
        // growBamboo flag serves — the [6.5,0,6.5]->[9.5,16,9.5] box
        // the x band speaks), the base's thin fours the [7,9] keep
        let top_off = crate::world::plant_offset(crate::world::BAMBOO, 8, 12, 8);
        let (top_ox, _, top_oz) = (top_off.0 as f32, top_off.1 as f32, top_off.2 as f32);
        let stalk_tile = atlas::FACE_TILES[crate::world::BAMBOO as usize][2];
        let thick: Vec<_> = in_band(stalk_tile, 12.0, 13.0);
        assert!(!thick.is_empty(), "the top's stalk quads render");
        let min_x = thick
            .iter()
            .flat_map(|quad| quad.pts.iter())
            .map(|p| p[0])
            .fold(f32::MAX, f32::min);
        let max_x = thick
            .iter()
            .flat_map(|quad| quad.pts.iter())
            .map(|p| p[0])
            .fold(f32::MIN, f32::max);
        assert!(
            (min_x - (8.0 + 6.5 / 16.0 + top_ox)).abs() < 1e-4
                && (max_x - (8.0 + 9.5 / 16.0 + top_ox)).abs() < 1e-4,
            "the h=4 top the thick age1 box wears: {min_x}..{max_x}"
        );
        let base_off = crate::world::plant_offset(crate::world::BAMBOO, 8, 9, 8);
        let (base_ox, _, base_oz) = (base_off.0 as f32, base_off.1 as f32, base_off.2 as f32);
        let thin: Vec<_> = in_band(stalk_tile, 9.0, 10.0);
        assert!(!thin.is_empty(), "the base's stalk quads render");
        let bmin_x = thin
            .iter()
            .flat_map(|quad| quad.pts.iter())
            .map(|p| p[0])
            .fold(f32::MAX, f32::min);
        let bmax_x = thin
            .iter()
            .flat_map(|quad| quad.pts.iter())
            .map(|p| p[0])
            .fold(f32::MIN, f32::max);
        assert!(
            (bmin_x - (8.0 + 7.0 / 16.0 + base_ox)).abs() < 1e-4
                && (bmax_x - (8.0 + 9.0 / 16.0 + base_ox)).abs() < 1e-4,
            "the h=4 base the thin age0 box keeps: {bmin_x}..{bmax_x}"
        );
        // THE H=6 PLANT (the steady state): the top LARGE, the two
        // below SMALL, the floor the NONE keeps (the k>=3 band the
        // y=9..12 cells wear)
        put(&mut w, 8, 13, 8, crate::world::BAMBOO);
        put(&mut w, 8, 14, 8, crate::world::BAMBOO);
        let q = mesh_chunk(&w, 0, 0, None);
        let smalls = |y0: f32, y1: f32| -> usize {
            q.iter()
                .filter(|quad| {
                    quad.tile == atlas::TILE_BAMBOO_SMALL_LEAVES
                        && quad.pts.iter().all(|p| p[1] >= y0 - 1e-4 && p[1] <= y1 + 1e-4)
                })
                .count()
        };
        let larges = |y0: f32, y1: f32| -> usize {
            q.iter()
                .filter(|quad| {
                    quad.tile == atlas::TILE_BAMBOO_LARGE_LEAVES
                        && quad.pts.iter().all(|p| p[1] >= y0 - 1e-4 && p[1] <= y1 + 1e-4)
                })
                .count()
        };
        assert_eq!(
            larges(14.0, 15.0),
            4,
            "the h=6 top the LARGE pair wears"
        );
        assert_eq!(
            smalls(13.0, 14.0),
            4,
            "the h=6 second the SMALL pair wears"
        );
        assert_eq!(
            smalls(12.0, 13.0),
            4,
            "the h=6 third the SMALL pair wears"
        );
        assert_eq!(
            smalls(9.0, 12.0),
            0,
            "the h=6 floor the NONE steady state keeps"
        );
        assert_eq!(
            larges(9.0, 14.0),
            0,
            "the LARGE pair the top alone wears"
        );
    }

    #[test]
    fn the_bamboo_sapling_the_cross_the_stage0_wears() {
        // THE SAPLING'S OWN CROSS (the tinted_cross parent the
        // flowers' own bake carries): the 4 quads the two diagonal
        // planes x the two windings wear, the tile the kind's own
        // atlas row banks (the stage0 master the generic walk
        // resolves), the XZ offset the plant walk's own row rides,
        // the pre-colored pixels the no-tint fold keeps.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::BAMBOO_SAPLING);
        let q = mesh_chunk(&w, 0, 0, None);
        let (dx, dy, dz) = crate::world::plant_offset(crate::world::BAMBOO_SAPLING, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!(dy, 0.0, "the XZ row the y the zero keeps");
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        let cross: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 10.0)
            })
            .collect();
        assert_eq!(cross.len(), 4, "the two planes the two windings wear: {}", cross.len());
        let sapling_tile = atlas::FACE_TILES[crate::world::BAMBOO_SAPLING as usize][2];
        for quad in &cross {
            assert_eq!(quad.tile, sapling_tile, "the kind's own atlas tile wears");
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
            }
        }
    }

    #[test]
    fn the_campfire_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CAMPFIRE FAMILY'S OWN TILES — the regen the log masters
        // bank (the palette arms' resolve walk the strip tables speak),
        // the fire quads' bases the FRAME0 consts answer
        let log_tile = gen_atlas::TILE_CAMPFIRE_LOG;
        let lit_tile = gen_atlas::TILE_CAMPFIRE_LOG_LIT;
        let soul_lit_tile = gen_atlas::TILE_SOUL_CAMPFIRE_LOG_LIT;
        let fire_tile = gen_atlas::CAMPFIRE_FIRE_FRAME0;
        assert_ne!(log_tile, gen_atlas::TILE_NONE, "the log tile the regen banks");
        assert_ne!(lit_tile, gen_atlas::TILE_NONE, "the lit-log tile the regen banks");
        assert_ne!(soul_lit_tile, gen_atlas::TILE_NONE, "the soul lit-log tile the regen banks");
        assert_ne!(fire_tile, gen_atlas::TILE_NONE, "the fire tile the strip tables bank");
        // THE UNLIT KIND'S OWN BAKE — the five-log geometry on stone:
        // the 28 element faces the walk derives, the THREE down
        // cullfaces the solid ground culled (log1/log3/log5's own
        // rows), the 25 drawn quads the log tile alone answers
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAMPFIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let unlit: Vec<_> = q.iter().filter(|quad| quad.tile == log_tile).collect();
        assert_eq!(
            unlit.len(),
            25,
            "the five-log boxes' 28 faces minus the three down culls"
        );
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the down
        // faces draw (the cullface verdict the neighbor read keeps):
        // the 28 faces the walk answers uncut
        put(&mut w, 12, 9, 8, crate::world::CAMPFIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == log_tile && quad.pts[0][0] >= 12.0)
            .collect();
        assert_eq!(floating.len(), 28, "the floating campfire the down culls never trigger");
        // THE LIT KIND'S OWN BAKE — the same five-log geometry the
        // lit_log faces swap PLUS the two 45-degree fire planes the
        // rotation walk bakes: the 16 log + 9 lit_log + 4 fire quads
        // the per-quad texture verdict splits (the 25 drawn log-box
        // quads the three down culls keep, the swap the texture refs
        // wear), the fire planes' y 1..17 span the cell above licks
        // into (the vanilla's own rise the mesh never clips)
        put(&mut w, 8, 9, 8, crate::world::CAMPFIRE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let lit_logs: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == log_tile && quad.pts[0][0] >= 6.0 && quad.pts[0][0] < 10.0)
            .collect();
        let lit_lits: Vec<_> = q.iter().filter(|quad| quad.tile == lit_tile).collect();
        let fires: Vec<_> = q.iter().filter(|quad| quad.tile == fire_tile).collect();
        assert_eq!(lit_logs.len(), 16, "the #log faces the lit template keeps");
        assert_eq!(lit_lits.len(), 9, "the #lit_log faces the lit template swaps");
        assert_eq!(fires.len(), 4, "the two fire planes' north/south + west/east pairs");
        for quad in &fires {
            assert!(
                quad.pts.iter().any(|p| p[1] > 10.0),
                "the fire plane the cell's top the vanilla lick wears: {:?}",
                quad.pts.iter().map(|p| p[1]).collect::<Vec<_>>()
            );
        }
        // THE SHADE TERM'S OWN SPLIT — the log boxes' `shade: true`
        // elements the direction table pays, the fire planes'
        // `shade: false` the lightmap alone multiplies (the no-grid
        // convention the unshaded 1.0 the fire corners carry)
        assert!(
            fires.iter().all(|quad| quad.colors.iter().all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the fire planes the 1.0 shade-false keeps"
        );
        // THE SOUL KIND'S OWN BAKE — the same geometry the soul
        // masters answer (the lit_log tile the soul strip's own row
        // keeps, the fire the soul strip's frame-0 base)
        put(&mut w, 4, 9, 8, crate::world::SOUL_CAMPFIRE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let soul_fires: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == gen_atlas::SOUL_CAMPFIRE_FIRE_FRAME0)
            .collect();
        let soul_lits: Vec<_> = q.iter().filter(|quad| quad.tile == soul_lit_tile).collect();
        assert_eq!(soul_fires.len(), 4, "the soul fire planes the same bake answers");
        assert_eq!(soul_lits.len(), 9, "the soul lit-log faces the same split wears");
    }

    #[test]
    fn the_candle_cake_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CANDLE CAKE PAIR'S OWN TILES — the regen the cake
        // masters bank (the appended walk the named consts answer),
        // the candle faces the palette's own candle tiles answer
        let bottom_tile = gen_atlas::TILE_CAKE_BOTTOM;
        let top_tile = gen_atlas::TILE_CAKE_TOP;
        let side_tile = gen_atlas::TILE_CAKE_SIDE;
        let candle_tile = gen_atlas::FACE_TILES[crate::world::CANDLE as usize][2];
        let lit_candle_tile = gen_atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2];
        assert_ne!(bottom_tile, gen_atlas::TILE_NONE, "the bottom tile the regen banks");
        assert_ne!(top_tile, gen_atlas::TILE_NONE, "the top tile the regen banks");
        assert_ne!(side_tile, gen_atlas::TILE_NONE, "the side tile the regen banks");
        assert_ne!(candle_tile, gen_atlas::TILE_NONE, "the candle tile the palette banks");
        assert_ne!(lit_candle_tile, gen_atlas::TILE_NONE, "the lit tile the palette banks");
        assert_ne!(lit_candle_tile, candle_tile, "the two candle states their own masters");
        // THE UNLIT KIND'S OWN BAKE — the slab + stick + wick geometry
        // on stone: the 16 element faces the walk derives, the TWO
        // down cullfaces the solid ground culled (the slab's and the
        // stick's own rows), the 14 drawn quads the texture verdict
        // splits — the bottom face the ground cull keeps 0, the top 1,
        // the four sides 4, the candle faces 9 (the stick's 5 drawn +
        // the two wick planes' 4 the rotation walk bakes)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CANDLE_CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let bottoms: Vec<_> = q.iter().filter(|quad| quad.tile == bottom_tile).collect();
        let tops: Vec<_> = q.iter().filter(|quad| quad.tile == top_tile).collect();
        let sides: Vec<_> = q.iter().filter(|quad| quad.tile == side_tile).collect();
        let candles: Vec<_> = q.iter().filter(|quad| quad.tile == candle_tile).collect();
        assert_eq!(bottoms.len(), 0, "the slab's down face the solid ground culls");
        assert_eq!(tops.len(), 1, "the slab's up face the top tile wears");
        assert_eq!(sides.len(), 4, "the slab's four side faces the side tile wears");
        assert_eq!(candles.len(), 9, "the stick's 5 drawn + the wick planes' 4 the candle tile answers");
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the two
        // down faces draw (the cullface verdict the neighbor read
        // keeps): the 16 faces the walk answers uncut, the bottom tile
        // the slab's own row serves
        put(&mut w, 12, 9, 8, crate::world::CANDLE_CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q
            .iter()
            .filter(|quad| {
                (quad.tile == bottom_tile
                    || quad.tile == top_tile
                    || quad.tile == side_tile
                    || quad.tile == candle_tile)
                    && quad.pts[0][0] >= 12.0
            })
            .collect();
        assert_eq!(floating.len(), 16, "the floating cake the down culls never trigger");
        assert_eq!(
            floating
                .iter()
                .filter(|quad| quad.tile == bottom_tile)
                .count(),
            1,
            "the slab's down face the air below draws"
        );
        // THE LIT KIND'S OWN BAKE — the same geometry the lit candle
        // faces swap (the #candle ref block/candle_lit the vanilla's
        // own texture override carries): the 9 drawn candle quads the
        // LIT tile answers, the cake faces the same masters keep
        put(&mut w, 8, 9, 8, crate::world::CANDLE_CAKE_LIT);
        let q = mesh_chunk(&w, 0, 0, None);
        let lit_candles: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == lit_candle_tile)
            .collect();
        assert_eq!(lit_candles.len(), 9, "the lit faces the same split the lit tile wears");
        // THE SHADE TERM'S OWN SPLIT — the template's shade:true
        // elements the direction table pays: the slab's up face the
        // 1.0 term keeps, the four sides the 0.8/0.6 horizontals wear
        // (the no-grid convention the lightmap alone multiplies)
        assert!(
            tops.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the up face the 1.0 shade term wears"
        );
        assert!(
            sides.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 0.8).abs() < 1e-5 || (c[0] - 0.6).abs() < 1e-5)),
            "the side faces the 0.8/0.6 shade terms wear"
        );
    }

    #[test]
    fn the_cake_lane_the_slice_models_bake() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CAKE LADDER'S OWN TILES — the named consts the regen
        // banks (the appended walk the four masters answer), the
        // bitten cut the TILE_CAKE_INNER const the west face reads
        let bottom_tile = gen_atlas::TILE_CAKE_BOTTOM;
        let top_tile = gen_atlas::TILE_CAKE_TOP;
        let side_tile = gen_atlas::TILE_CAKE_SIDE;
        let inner_tile = gen_atlas::TILE_CAKE_INNER;
        assert_ne!(bottom_tile, gen_atlas::TILE_NONE, "the bottom tile the regen banks");
        assert_ne!(top_tile, gen_atlas::TILE_NONE, "the top tile the regen banks");
        assert_ne!(side_tile, gen_atlas::TILE_NONE, "the side tile the regen banks");
        assert_ne!(inner_tile, gen_atlas::TILE_NONE, "the inner tile the regen banks");
        // THE FULL KIND'S OWN BAKE — the single box on stone: the SIX
        // element faces the walk derives, the down cullface the solid
        // ground culled (the 5 drawn quads the texture verdict splits
        // — the top 1, the four sides 4), the colors the shade:true
        // terms the direction table pays
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let cell = |quad: &&crate::mesher::Quad| quad.pts[0][0] >= 8.0 && quad.pts[0][0] < 9.0;
        let drawn: Vec<_> = q.iter().filter(|quad| {
            cell(quad) && (quad.tile == bottom_tile || quad.tile == top_tile
                || quad.tile == side_tile || quad.tile == inner_tile)
        }).collect();
        assert_eq!(drawn.len(), 5, "the six faces the down cull the ground serves");
        assert_eq!(drawn.iter().filter(|quad| quad.tile == top_tile).count(), 1,
            "the up face the top tile wears");
        assert_eq!(drawn.iter().filter(|quad| quad.tile == side_tile).count(), 4,
            "the four side faces the side tile wears");
        // THE FLOATING KIND'S OWN CULLS — the air-below cell the down
        // face draws (the cullface verdict the neighbor read keeps):
        // the 6 faces the walk answers uncut
        put(&mut w, 12, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let floating: Vec<_> = q.iter().filter(|quad| {
            quad.pts[0][0] >= 12.0 && quad.pts[0][0] < 13.0
                && (quad.tile == bottom_tile || quad.tile == top_tile || quad.tile == side_tile)
        }).collect();
        assert_eq!(floating.len(), 6, "the floating cake the down culls never trigger");
        assert_eq!(floating.iter().filter(|quad| quad.tile == bottom_tile).count(), 1,
            "the down face the air below draws");
        // THE BITTEN STAGES' OWN BAKES — every slice the SAME single
        // box the x-start walk narrows (the lambda$static$0 shape the
        // models exactly mirror): the west face the #inside ref the
        // TILE_CAKE_INNER const answers, the other five the cake
        // masters keep (the on-ground slice the down cull keeps — the
        // 5 drawn quads the west-inside split wears)
        for (slice, kind) in [
            ("block/cake_slice1", crate::world::CAKE_SLICE1),
            ("block/cake_slice2", crate::world::CAKE_SLICE2),
            ("block/cake_slice3", crate::world::CAKE_SLICE3),
            ("block/cake_slice4", crate::world::CAKE_SLICE4),
            ("block/cake_slice5", crate::world::CAKE_SLICE5),
            ("block/cake_slice6", crate::world::CAKE_SLICE6),
        ] {
            let mut w = empty_world();
            put(&mut w, 8, 8, 8, STONE);
            put(&mut w, 8, 9, 8, kind);
            let q = mesh_chunk(&w, 0, 0, None);
            let inners: Vec<_> = q
                .iter()
                .filter(|quad| quad.tile == inner_tile && cell(quad))
                .collect();
            assert_eq!(inners.len(), 1, "slice {slice}: the west face the inner tile wears");
            let tops: Vec<_> = q
                .iter()
                .filter(|quad| quad.tile == top_tile && cell(quad))
                .collect();
            assert_eq!(tops.len(), 1, "slice {slice}: the up face the top tile wears");
        }
        // THE SHADE TERM'S OWN SPLIT — the box's shade:true elements
        // the direction table pays: the up face the 1.0 term keeps,
        // the four sides the 0.8/0.6 horizontals wear (the no-grid
        // convention the lightmap alone multiplies)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CAKE);
        let q = mesh_chunk(&w, 0, 0, None);
        let tops: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == top_tile && cell(quad))
            .collect();
        assert!(
            tops.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 1.0).abs() < 1e-5)),
            "the up face the 1.0 shade term wears"
        );
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == side_tile && cell(quad))
            .collect();
        assert!(
            sides.iter().all(|quad| quad
                .colors
                .iter()
                .all(|c| (c[0] - 0.8).abs() < 1e-5 || (c[0] - 0.6).abs() < 1e-5)),
            "the side faces the 0.8/0.6 shade terms wear"
        );
    }

    #[test]
    fn the_web_the_cross_planes_and_wool_the_cube_wear() {
        use mc_registry::generated::atlas as gen_atlas;
        let web_tile = gen_atlas::FACE_TILES[crate::world::COBWEB as usize][2];
        let wool_tile = gen_atlas::FACE_TILES[crate::world::WHITE_WOOL as usize][0];
        assert_ne!(web_tile, gen_atlas::TILE_NONE, "the web tile the regen banks");
        assert_ne!(wool_tile, gen_atlas::TILE_NONE, "the wool tile the regen banks");
        // THE WEB'S OWN CROSS (the freestanding cell the two diagonal
        // planes the flowers' own path wears — the 4 quads the two
        // planes × two windings speak, the offset the NONE row zeroes
        // — the rigid diagonals the vanilla model renders)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::COBWEB);
        let q = mesh_chunk(&w, 0, 0, None);
        let webs: Vec<_> = q.iter().filter(|quad| quad.tile == web_tile).collect();
        assert_eq!(webs.len(), 4, "the web the two diagonals × two windings keeps");
        // THE RIGID PLANE (the offset zero — every point the cell's own
        // 8..9 x-band the no-jitter walk keeps)
        for quad in &webs {
            for pt in quad.pts {
                assert!(
                    (8.0..9.0).contains(&pt[0]),
                    "the web point the cell band keeps"
                );
            }
        }
        // THE WOOL'S OWN CUBE (the freestanding cell the six faces the
        // full-cube path wears — the 6 quads the backface-culled walk
        // keeps, one quad per face)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::WHITE_WOOL);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let wools: Vec<_> = q2.iter().filter(|quad| quad.tile == wool_tile).collect();
        assert_eq!(wools.len(), 6, "the wool the six faces the cube path keeps");
        // THE SOUL SAND'S OWN CUBE (the autopush-598 round — the
        // freestanding cell the six faces the same full-cube path
        // wears, the opaque verdict the is_transparent silence keeps —
        // the AO the opaque walk casts, the same-tile six the
        // cube_all's `all` ref banks)
        let sand_tile = gen_atlas::FACE_TILES[crate::world::SOUL_SAND as usize][0];
        assert_ne!(sand_tile, gen_atlas::TILE_NONE, "the sand tile the regen banks");
        assert_eq!(gen_atlas::tile_name(sand_tile), "soul_sand", "the sand tile the master names");
        for face in 0..6 {
            assert_eq!(
                gen_atlas::FACE_TILES[crate::world::SOUL_SAND as usize][face],
                sand_tile,
                "the sand face {face} the cube_all's same tile keeps"
            );
            assert_eq!(gen_atlas::TINTS[crate::world::SOUL_SAND as usize][face], 0, "the sand face {face} untinted");
        }
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::SOUL_SAND);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let sands: Vec<_> = q3.iter().filter(|quad| quad.tile == sand_tile).collect();
        assert_eq!(sands.len(), 6, "the sand the six faces the cube path keeps");
    }

    /// THE HONEY'S OWN TWO LAYERS (the autopush-599 round — javap the
    /// 26.2 jar: the TWO-ELEMENT model the element walk resolves): the
    /// freestanding cell the TWELVE quads the two cubes speak — the
    /// INSET quads the back-to-front order FIRST banks (the reversed
    /// derive walk the depth-write law serves), the per-quad tiles the
    /// three masters answer (#down the outer cube all six faces, the
    /// inset's #up/#side/#down skins the 14-texel crop wears), the
    /// translucent + per-texel flags the arm reads, the shared face
    /// the opaque neighbor culls (the outer cube's cullface row the
    /// stone verdict answers — the inset's own faces the no-cullframe
    /// rows keep).
    #[test]
    fn the_honey_the_two_layers_the_inset_first_wears() {
        use mc_registry::generated::atlas as gen_atlas;
        let bottom_tile = gen_atlas::TILE_HONEY_BLOCK_BOTTOM;
        let top_tile = gen_atlas::TILE_HONEY_BLOCK_TOP;
        let side_tile = gen_atlas::TILE_HONEY_BLOCK_SIDE;
        assert_eq!(gen_atlas::tile_name(bottom_tile), "honey_block_bottom");
        assert_eq!(gen_atlas::tile_name(top_tile), "honey_block_top");
        assert_eq!(gen_atlas::tile_name(side_tile), "honey_block_side");
        // THE FREESTANDING CELL (all neighbors air — no cull fires)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::HONEY_BLOCK);
        let q = mesh_chunk(&w, 0, 0, None);
        let honey: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.tile == bottom_tile || quad.tile == top_tile || quad.tile == side_tile
            })
            .collect();
        assert_eq!(honey.len(), 12, "the two cubes the twelve quads keep");
        // THE PER-TILE SPLIT (the outer cube's 6x #down + the inset's
        // 1x #down / 1x #up / 4x #side the model's texture map speaks)
        let bottoms = honey.iter().filter(|quad| quad.tile == bottom_tile).count();
        let tops = honey.iter().filter(|quad| quad.tile == top_tile).count();
        let sides = honey.iter().filter(|quad| quad.tile == side_tile).count();
        assert_eq!(bottoms, 7, "the outer six + the inset's own down the #down master wears");
        assert_eq!(tops, 1, "the inset's up the #up master wears");
        assert_eq!(sides, 4, "the inset's four sides the #side master wears");
        // THE INSET-FIRST ORDER (the reversed derive walk — the first
        // six quads the inset cube's own band keeps: the derive walk's
        // 0..1 NORMALIZED units the /16 fold carries — the 1/16..15/16
        // band the 1..15 texel from/to speaks, the tail six the outer
        // cube's full 0..1 band wears)
        for quad in &honey[..6] {
            for pt in quad.pts {
                let fx = pt[0] - 8.0;
                let fy = pt[1] - 8.0;
                let fz = pt[2] - 8.0;
                assert!(
                    (0.0625..=0.9375).contains(&fx)
                        && (0.0625..=0.9375).contains(&fy)
                        && (0.0625..=0.9375).contains(&fz),
                    "the inset quad the 1/16..15/16 band keeps: {pt:?}"
                );
            }
        }
        for quad in &honey[6..] {
            assert!(
                quad.pts.iter().any(|pt| (pt[0] - 8.0).abs() < 1e-6)
                    || quad.pts.iter().any(|pt| (pt[0] - 9.0).abs() < 1e-6),
                "the outer quad the full-cube plane wears"
            );
        }
        // THE 14-TEXEL CROP (the inset's uv [1,1,15,15] the /16 fold
        // normalizes — the first quad the inset's down face speaks)
        let first = &honey[0];
        for uv in first.uvs {
            assert!(
                (0.0625..=0.9375).contains(&uv[0]) && (0.0625..=0.9375).contains(&uv[1]),
                "the inset uv the 1..15 texel crop keeps: {uv:?}"
            );
        }
        // THE FLAGS (translucent + per-texel — the raster's arm reads)
        for quad in &honey {
            assert!(quad.translucent, "the honey quad the translucent pass rides");
            assert!(
                quad.per_texel_alpha,
                "the honey quad the texel's own alpha composites"
            );
        }
        // THE SHARED-FACE CULL (the opaque stone the +X neighbor — the
        // outer cube's cullface row the verdict answers, the inset's
        // own faces the no-cullframe rows keep: the eleven quads)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::HONEY_BLOCK);
        put(&mut w2, 9, 8, 8, crate::world::STONE);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let honey2: Vec<_> = q2
            .iter()
            .filter(|quad| {
                quad.tile == bottom_tile || quad.tile == top_tile || quad.tile == side_tile
            })
            .collect();
        assert_eq!(honey2.len(), 11, "the stone neighbor the shared outer face culls");
        assert_eq!(
            honey2.iter().filter(|quad| quad.tile == bottom_tile).count(),
            6,
            "the culled face the outer cube's own #down wears — the inset's down the count keeps"
        );
    }

    #[test]
    fn the_glass_lane_culls_the_same_kind_and_blends_the_stained_half() {
        // THE GLASS LANE'S OWN LAWS (the dev-57 round — javap
        // HalfTransparentBlock.skipRendering pc 0-17 the 26.2 jar the
        // probe scripts/jar_work/glass_685.py banks): the SAME block
        // the shared face never draws (the neighborState.is(this)
        // verdict the cull keeps), the DIFFERENT blocks the draw
        // wears, the stained half the translucent pass the per-texel
        // alpha blend serves (the models' force_translucent ref the
        // render-layer signal speaks), the plain glass the OPAQUE
        // pass the binary cutout wears (the textures' {0: 191,
        // 255: 65} alpha map the discard law answers)
        use mc_registry::generated::atlas as gen_atlas;
        let glass_tile = gen_atlas::FACE_TILES[crate::world::GLASS as usize][2];
        let white_tile = gen_atlas::FACE_TILES[crate::world::WHITE_STAINED_GLASS as usize][2];
        let red_tile = gen_atlas::FACE_TILES[crate::world::RED_STAINED_GLASS as usize][2];
        assert_eq!(gen_atlas::tile_name(glass_tile), "glass");
        assert_eq!(
            gen_atlas::tile_name(white_tile),
            "white_stained_glass"
        );
        // THE FREESTANDING CELL (all neighbors air — the six quads
        // the full cube keeps)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::GLASS);
        let q = mesh_chunk(&w, 0, 0, None);
        let glass: Vec<_> = q.iter().filter(|quad| quad.tile == glass_tile).collect();
        assert_eq!(glass.len(), 6, "the lone window the six faces wear");
        for quad in &glass {
            assert!(
                !quad.translucent,
                "the plain glass the OPAQUE pass rides (the cutout lane)"
            );
            assert!(
                !quad.per_texel_alpha,
                "the plain glass the 0.5 cutout gate wears"
            );
        }
        // THE STAINED CLOTH'S OWN FLAGS (the translucent pass the
        // per-texel alpha blend serves — the honey arm's own
        // machinery the ladder wears)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::WHITE_STAINED_GLASS);
        put(&mut w2, 10, 8, 8, crate::world::RED_STAINED_GLASS);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let white: Vec<_> = q2.iter().filter(|quad| quad.tile == white_tile).collect();
        let red: Vec<_> = q2.iter().filter(|quad| quad.tile == red_tile).collect();
        assert_eq!(white.len(), 6, "the lone white cloth the six faces wear");
        assert_eq!(red.len(), 6, "the lone red cloth the six faces wear");
        for quad in white.iter().chain(red.iter()) {
            assert!(
                quad.translucent,
                "the stained quad the translucent pass rides"
            );
            assert!(
                quad.per_texel_alpha,
                "the stained quad the texel's own alpha composites"
            );
        }
        // THE GLASS|GLASS PAIR THE SHARED FACE CULLS (the
        // skipRendering law — the ten quads the twelve-minus-two
        // wears, the coplanar z-fight the same plane never speaks)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::GLASS);
        put(&mut w3, 9, 8, 8, crate::world::GLASS);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let glass3: Vec<_> = q3.iter().filter(|quad| quad.tile == glass_tile).collect();
        assert_eq!(glass3.len(), 10, "the same-kind pair the shared faces cull");
        // THE GLASS|STAINED PAIR THE DRAW WEARS (the DIFFERENT blocks
        // the cull never fires — the twelve quads the full pair
        // keeps)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::GLASS);
        put(&mut w4, 9, 8, 8, crate::world::WHITE_STAINED_GLASS);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        let g4: Vec<_> = q4.iter().filter(|quad| quad.tile == glass_tile).collect();
        let wh4: Vec<_> = q4.iter().filter(|quad| quad.tile == white_tile).collect();
        assert_eq!(g4.len(), 6, "the glass half the full six wears");
        assert_eq!(wh4.len(), 6, "the stained half the full six wears");
        // THE OPAQUE CULL THE STONE PAIR KEEPS (the glass|stone pair
        // the glass's shared face culls — the stone the OPAQUE the
        // is_transparent verdict refuses — and the stone's own face
        // against the glass DRAWS the non-occluding law wears)
        let mut w5 = empty_world();
        put(&mut w5, 8, 8, 8, crate::world::GLASS);
        put(&mut w5, 9, 8, 8, crate::world::STONE);
        let q5 = mesh_chunk(&w5, 0, 0, None);
        let g5: Vec<_> = q5.iter().filter(|quad| quad.tile == glass_tile).collect();
        assert_eq!(
            g5.len(),
            5,
            "the stone neighbor the glass's shared face culls"
        );
        let stone_tile = gen_atlas::FACE_TILES[crate::world::STONE as usize][2];
        let stones: Vec<_> = q5
            .iter()
            .filter(|quad| quad.tile == stone_tile)
            .collect();
        assert!(
            stones.len() >= 1,
            "the stone's own faces the mesh keeps"
        );
        // the stone face AGAINST the glass draws (the +X face at
        // x=8.0 — the glass cell the non-occluding neighbor the draw
        // verdict answers): the plane the two cells' seam wears
        assert!(
            stones
                .iter()
                .any(|quad| quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6)),
            "the stone's -X face the glass cell draws against"
        );
    }

    #[test]
    fn the_pane_lane_selects_the_post_the_arms_and_the_strips_wear() {
        // THE PANE LANE'S OWN SELECTION (the dev-58 round — the
        // blockstates {name}.json's multipart the pane_parts walk
        // parses, the same select_parts walk the fence/wall tests
        // pin): the lone state the post + the four noside caps
        // speak, the connected state the post + the side arms the
        // when-rows answer, the same-kind cull the skipRendering
        // law banks, the per-quad tiles the #edge/#pane stems the
        // mapping answers, the stained flags the dyed cloths carry
        use mc_registry::generated::atlas as gen_atlas;
        let edge_tile = gen_atlas::FACE_TILES[crate::world::GLASS_PANE as usize][2];
        let glass_tile = gen_atlas::FACE_TILES[crate::world::GLASS as usize][2];
        let red_edge_tile = gen_atlas::FACE_TILES[crate::world::RED_STAINED_GLASS_PANE as usize][2];
        let red_glass_tile = gen_atlas::FACE_TILES[crate::world::RED_STAINED_GLASS as usize][2];
        let bars_tile = gen_atlas::FACE_TILES[crate::world::IRON_BARS as usize][2];
        assert_eq!(gen_atlas::tile_name(edge_tile), "glass_pane_top");
        assert_eq!(gen_atlas::tile_name(red_edge_tile), "red_stained_glass_pane_top");
        assert_eq!(gen_atlas::tile_name(bars_tile), "iron_bars");
        // THE LONE PANE (all neighbors air — the post caps + the
        // four noside caps the unconnected fan wears; the derived
        // quads the templates speak: the post's 2 caps + the four
        // noside ONE-face quads = the six quads)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::GLASS_PANE);
        let q = mesh_chunk(&w, 0, 0, None);
        let pane: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == edge_tile || quad.tile == glass_tile)
            .collect();
        assert_eq!(pane.len(), 6, "the lone pane the post + the four noside caps wear");
        // the plain pane the OPAQUE flags wear (the cutout lane)
        for quad in &pane {
            assert!(!quad.translucent, "the plain pane the OPAQUE pass rides");
            assert!(!quad.per_texel_alpha, "the plain pane the binary cutout wears");
        }
        // the tiles the #edge refs the kind's own row bank (the
        // post caps + the noside caps the SAME edge master wear —
        // the noside's #pane ref the template's particle-only walk
        // never emits [the noside element carries ONE north face
        // the #pane ref answers — hmm, the noside's north face IS
        // the #pane texture: the lone pane the noside quads the
        // GLASS master wear])
        let edges = pane.iter().filter(|quad| quad.tile == edge_tile).count();
        let panes = pane.iter().filter(|quad| quad.tile == glass_tile).count();
        assert_eq!(
            edges, 2,
            "the post's two caps the #edge master wears"
        );
        assert_eq!(
            panes, 4,
            "the four noside caps the #pane master wears (the glass block's own tile)"
        );
        // THE CONNECTED PAIR (the north neighbor — the side arm the
        // when-row adds, the north noside the unconnected arm
        // drops): the SAME-COLOR pair the seam END faces the
        // same-kind cull keeps — each pane: 2 post caps + 4 arm
        // quads (the END face the cull drops) + 3 noside caps = 9 —
        // the pair the 18 quads the cull wears
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::GLASS_PANE);
        put(&mut w2, 8, 8, 7, crate::world::GLASS_PANE);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let pane2: Vec<_> = q2
            .iter()
            .filter(|quad| quad.tile == edge_tile || quad.tile == glass_tile)
            .collect();
        assert_eq!(pane2.len(), 18, "the same-color pair the seam cull wears");
        // THE SEAM PLANE THE END FACES NEVER CARRY (the skipRendering
        // law — the z=8.0 plane the two cells' boundary the END
        // quads' shared plane the cull empties; the arm long faces'
        // corners touch the plane the ALL-corners walk never counts)
        let seam_quads = pane2
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (p[2] - 8.0).abs() < 1e-6))
            .count();
        assert_eq!(
            seam_quads, 0,
            "the connected seam the same-kind cull keeps (the skipRendering law wears)"
        );
        // THE STAINED CLOTH'S OWN FLAGS (the translucent pass the
        // per-texel blend serves) + THE DIFFERENT-COLOR PAIR THE
        // SEAM DRAWS (the skipRendering law the different blocks
        // never fires — the END faces the seam plane carries, ONE
        // per pane the #edge masters tile)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::WHITE_STAINED_GLASS_PANE);
        put(&mut w3, 9, 8, 8, crate::world::RED_STAINED_GLASS_PANE);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let white_edge_tile =
            gen_atlas::FACE_TILES[crate::world::WHITE_STAINED_GLASS_PANE as usize][2];
        let white_glass_tile =
            gen_atlas::FACE_TILES[crate::world::WHITE_STAINED_GLASS as usize][2];
        let stained: Vec<_> = q3
            .iter()
            .filter(|quad| {
                quad.tile == white_edge_tile
                    || quad.tile == white_glass_tile
                    || quad.tile == red_edge_tile
                    || quad.tile == red_glass_tile
            })
            .collect();
        // each pane: 2 post caps + 5 side-arm quads (the END face
        // the DIFFERENT-color neighbor the draw keeps) + 3 noside
        // caps = 10 — the pair the 20 quads the seam wears
        assert_eq!(stained.len(), 20, "the different-color pair the seam faces wear");
        for quad in &stained {
            assert!(
                quad.translucent,
                "the stained quad the translucent pass rides"
            );
            assert!(
                quad.per_texel_alpha,
                "the stained quad the texel's own alpha composites"
            );
        }
        // THE SEAM FACES THE PAIR CARRIES (the x=9.0 plane the two
        // END quads the DIFFERENT-color draw keeps — one per pane
        // the #edge masters tile)
        let seam_a = stained
            .iter()
            .filter(|quad| {
                quad.tile == white_edge_tile
                    && quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6)
            })
            .count();
        let seam_b = stained
            .iter()
            .filter(|quad| {
                quad.tile == red_edge_tile
                    && quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6)
            })
            .count();
        assert_eq!(seam_a, 1, "the white pane's east END the seam draws");
        assert_eq!(seam_b, 1, "the red pane's west END the seam draws");
        // THE IRON BARS (the zero-width planes the crossed-sheet
        // look renders — the lone bars the post_ends planes + the
        // post planes the all-false arm wears)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::IRON_BARS);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        let bars: Vec<_> = q4.iter().filter(|quad| quad.tile == bars_tile).collect();
        assert!(
            !bars.is_empty(),
            "the lone bars the crossed-plane quads wear"
        );
        for quad in &bars {
            assert!(
                !quad.translucent,
                "the bars the OPAQUE pass rides (the binary cutout)"
            );
            assert!(!quad.per_texel_alpha, "the bars the 0.5 cutout gate wears");
        }
        // the bars' planes the zero-width x folds speak (the post's
        // planes at x=8.5 and z=8.5 — the crossed sheet the
        // template_bars_post geometry wears)
        assert!(
            bars.iter()
                .any(|quad| quad.pts.iter().all(|p| (p[0] - 8.5).abs() < 1e-6)),
            "the x=8.5 plane the post's zero-width element wears"
        );
        assert!(
            bars.iter()
                .any(|quad| quad.pts.iter().all(|p| (p[2] - 8.5).abs() < 1e-6)),
            "the z=8.5 plane the post's zero-width element wears"
        );
    }

    #[test]
    fn the_copper_bars_lane_shares_the_bars_law_and_the_tag_pairs_cull_wear() {
        // THE COPPER BARS LANE'S OWN LAWS (the dev-59 round — javap
        // lambda$static$162 the plain IronBarsBlock instances the
        // collection walk fabricates, the SAME pane lane the eight
        // oxidization rungs the entry condition renders, the
        // template_bars_* geometry the crossed planes wear, the BARS
        // TAG the pair-cull arm the palette ARMS — the probe
        // scripts/jar_work/copper_687.py banks): the bare rungs the
        // OWN masters wear, the waxed twins the BASE rows re-wear,
        // the iron|copper pair the DIFFERENT blocks the tag law
        // culls, the copper|exposed pair the ladder neighbors the
        // same law culls
        use mc_registry::generated::atlas as gen_atlas;
        let copper_tile = gen_atlas::FACE_TILES[crate::world::COPPER_BARS as usize][2];
        let exposed_tile =
            gen_atlas::FACE_TILES[crate::world::EXPOSED_COPPER_BARS as usize][2];
        let iron_tile = gen_atlas::FACE_TILES[crate::world::IRON_BARS as usize][2];
        assert_eq!(gen_atlas::tile_name(copper_tile), "copper_bars");
        assert_eq!(gen_atlas::tile_name(exposed_tile), "exposed_copper_bars");
        assert_ne!(
            copper_tile, exposed_tile,
            "the bare rungs the DISTINCT masters wear"
        );
        assert_ne!(copper_tile, iron_tile, "the copper voice its own master wears");
        // THE WAXED TWIN THE BASE MASTER RE-WEARS (the texture-
        // sharing law the vanilla waxed look keeps)
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_COPPER_BARS as usize][2],
            copper_tile,
            "the waxed head the bare master re-wears"
        );
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_OXIDIZED_COPPER_BARS as usize][2],
            gen_atlas::FACE_TILES[crate::world::OXIDIZED_COPPER_BARS as usize][2],
            "the waxed oxidized twin the base row re-wears"
        );
        // THE LONE RUNG (the crossed planes the template_bars_post
        // geometry wears — the x=8.5/z=8.5 planes the iron seam's own
        // law speaks, the OPAQUE pass the binary cutout rides)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::COPPER_BARS);
        let q = mesh_chunk(&w, 0, 0, None);
        let lone: Vec<_> = q.iter().filter(|quad| quad.tile == copper_tile).collect();
        assert!(!lone.is_empty(), "the lone rung the crossed-plane quads wear");
        for quad in &lone {
            assert!(!quad.translucent, "the rung the OPAQUE pass rides");
            assert!(!quad.per_texel_alpha, "the rung the binary cutout wears");
        }
        assert!(
            lone.iter()
                .any(|quad| quad.pts.iter().all(|p| (p[0] - 8.5).abs() < 1e-6)),
            "the x=8.5 plane the post's zero-width element wears"
        );
        assert!(
            lone.iter()
                .any(|quad| quad.pts.iter().all(|p| (p[2] - 8.5).abs() < 1e-6)),
            "the z=8.5 plane the post's zero-width element wears"
        );
        // THE IRON|COPPER PAIR THE TAG ARM CULLS (the seam plane
        // x=9.0 the two cells' boundary — the skipRendering pc 8-47
        // the DIFFERENT blocks the BARS-tag pair arm the shared faces
        // culls: the crossing planes' boundary quads the seam plane
        // NEVER carries, the connected arms the when-rows still
        // render; the pair the SAME-KIND count the iron|iron pair the
        // reference wears — the tag law the different-kind pair the
        // same-kind look renders)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::IRON_BARS);
        put(&mut w2, 9, 8, 8, crate::world::COPPER_BARS);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let pair: Vec<_> = q2
            .iter()
            .filter(|quad| quad.tile == copper_tile || quad.tile == iron_tile)
            .collect();
        let mut w_ref = empty_world();
        put(&mut w_ref, 8, 8, 8, crate::world::IRON_BARS);
        put(&mut w_ref, 9, 8, 8, crate::world::IRON_BARS);
        let q_ref = mesh_chunk(&w_ref, 0, 0, None);
        let same: Vec<_> = q_ref.iter().filter(|quad| quad.tile == iron_tile).collect();
        assert_eq!(
            pair.len(),
            same.len(),
            "the iron|copper pair the same-kind count the tag law wears"
        );
        let seam_quads = pair
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6))
            .count();
        assert_eq!(
            seam_quads, 0,
            "the iron|copper seam the BARS-tag pair arm culls (the skipRendering law wears)"
        );
        // THE COPPER|EXPOSED PAIR THE LADDER LAW CULLS (the ladder
        // neighbors the DIFFERENT kinds the same tag arm the seam
        // culls — the connected lattice the ONE continuous band
        // renders, the count the same-kind reference wears)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::COPPER_BARS);
        put(&mut w3, 9, 8, 8, crate::world::EXPOSED_COPPER_BARS);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let ladder_pair: Vec<_> = q3
            .iter()
            .filter(|quad| quad.tile == copper_tile || quad.tile == exposed_tile)
            .collect();
        let mut w_ref2 = empty_world();
        put(&mut w_ref2, 8, 8, 8, crate::world::COPPER_BARS);
        put(&mut w_ref2, 9, 8, 8, crate::world::COPPER_BARS);
        let q_ref2 = mesh_chunk(&w_ref2, 0, 0, None);
        let same2: Vec<_> = q_ref2
            .iter()
            .filter(|quad| quad.tile == copper_tile)
            .collect();
        assert_eq!(
            ladder_pair.len(),
            same2.len(),
            "the copper|exposed pair the same-kind count wears"
        );
        let ladder_seam = ladder_pair
            .iter()
            .filter(|quad| quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-6))
            .count();
        assert_eq!(
            ladder_seam, 0,
            "the copper|exposed seam the tag arm culls (the ladder neighbors the one band render)"
        );
        // THE WAXED TWIN THE BASE GEOMETRY RENDERS (the blockstate
        // the BASE models the references wear — the waxed kind's own
        // quads the bare master's tile the same lane banks)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::WAXED_COPPER_BARS);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        let waxed: Vec<_> = q4.iter().filter(|quad| quad.tile == copper_tile).collect();
        assert_eq!(
            waxed.len(),
            lone.len(),
            "the waxed twin the BASE models the blockstate references render"
        );
    }

    #[test]
    fn the_chain_lane_renders_the_links_the_axis_turns_and_the_bare_masters_wear() {
        // THE CHAIN LANE'S OWN LAWS (the dev-60 round — javap
        // ChainBlock the 26.2 jar the probe scripts/jar_work/
        // chain_688.py banks): the blockstates the NINE the variants
        // carry (the axis=y identity + the axis=z x=90 + the axis=x
        // x=90+y=90), the template_chain the TWO zero-width planes
        // the ±45° y-axis rotations speak (the crossed diagonals the
        // x==z / x+z==16 laws wear), the fold order the geometry
        // demands (the x=90 the vertical shaft the horizontal rung
        // maps FIRST, the y-steps the z-rung the x-rung turns
        // SECOND), the waxed twins the BASE models the references
        // re-wear
        use mc_registry::generated::atlas as gen_atlas;
        let iron_tile = gen_atlas::FACE_TILES[crate::world::IRON_CHAIN as usize][2];
        let copper_tile = gen_atlas::FACE_TILES[crate::world::COPPER_CHAIN as usize][2];
        let exposed_tile =
            gen_atlas::FACE_TILES[crate::world::EXPOSED_COPPER_CHAIN as usize][2];
        assert_eq!(gen_atlas::tile_name(iron_tile), "iron_chain");
        assert_eq!(gen_atlas::tile_name(copper_tile), "copper_chain");
        assert_ne!(iron_tile, copper_tile, "the two voices the DISTINCT masters wear");
        assert_ne!(
            copper_tile, exposed_tile,
            "the bare rungs the DISTINCT masters wear"
        );
        // THE WAXED TWIN THE BASE MASTER RE-WEARS (the texture-
        // sharing law the vanilla waxed look keeps)
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_COPPER_CHAIN as usize][2],
            copper_tile,
            "the waxed head the bare master re-wears"
        );
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_OXIDIZED_COPPER_CHAIN as usize][2],
            gen_atlas::FACE_TILES[crate::world::OXIDIZED_COPPER_CHAIN as usize][2],
            "the waxed oxidized twin the base row re-wears"
        );
        // THE LONE VERTICAL CHAIN (the axis=y default the unstamped
        // cell wears — the TWO zero-width planes the ±45° rotations
        // render: two quads on the x==z diagonal, two on the
        // x+z==16 anti-diagonal, the y span the FULL 0..1 band the
        // vertical shaft keeps; shade:false the flat light wears)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::COPPER_CHAIN);
        let q = mesh_chunk(&w, 0, 0, None);
        let lone: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == copper_tile)
            .collect();
        assert_eq!(
            lone.len(),
            4,
            "the lone link the two crossed planes' faces wear (two quads each)"
        );
        for quad in &lone {
            assert!(!quad.translucent, "the link the OPAQUE pass rides");
            assert!(!quad.per_texel_alpha, "the link the binary cutout wears");
            let y_min = quad
                .pts
                .iter()
                .map(|p| p[1])
                .fold(f32::MAX, f32::min);
            let y_max = quad
                .pts
                .iter()
                .map(|p| p[1])
                .fold(f32::MIN, f32::max);
            assert!(
                (y_min - 8.0).abs() < 1e-5 && ((y_max - 9.0).abs() < 1e-5),
                "the y span the FULL 0..1 band the vertical shaft keeps, got {y_min}..{y_max}"
            );
        }
        let diagonal = lone
            .iter()
            .filter(|quad| {
                quad.pts
                    .iter()
                    .all(|p| (p[0] - p[2]).abs() < 1e-4)
            })
            .count();
        let anti_diagonal = lone
            .iter()
            .filter(|quad| {
                quad.pts
                    .iter()
                    .all(|p| (p[0] + p[2] - 17.0).abs() < 1e-4)
            })
            .count();
        assert_eq!(diagonal, 2, "the +45° plane the x==z diagonal wears");
        assert_eq!(anti_diagonal, 2, "the -45° plane the x+z==17 anti-diagonal wears");
        // THE AXIS=Z CHAIN THE X-FOLD RENDERS (the variant byte the
        // 2 the placed stamp banks — the x=90 turn the vertical
        // shaft the along-z rung maps: the y the centered 6.5..9.5
        // band, the z the FULL 0..1 span, the diagonal the x+y==1
        // plane wears)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::COPPER_CHAIN);
        w2.set_variant(8, 8, 8, crate::world::CHAIN_AXIS_Z);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        let rung_z: Vec<_> = q2
            .iter()
            .filter(|quad| quad.tile == copper_tile)
            .collect();
        assert_eq!(rung_z.len(), 4, "the horizontal rung the same crossed pair wears");
        for quad in &rung_z {
            for p in quad.pts.iter() {
                assert!(
                    p[1] - 8.0 >= 6.5 / 16.0 - 1e-5 && p[1] - 8.0 <= 9.5 / 16.0 + 1e-5,
                    "the y band the centered 6.5..9.5 px the x=90 fold keeps, got {}",
                    p[1]
                );
                assert!(
                    p[2] >= 8.0 - 1e-5 && p[2] <= 9.0 + 1e-5,
                    "the z span the FULL 0..1 band the rung keeps, got {}",
                    p[2]
                );
            }
            assert!(
                quad
                    .pts
                    .iter()
                    .all(|p| (p[0] + p[1] - 17.0).abs() < 1e-4)
                    || quad
                        .pts
                        .iter()
                        .all(|p| (p[0] - p[1]).abs() < 1e-4),
                "the diagonal the x±y plane the folded pair wears"
            );
        }
        // THE AXIS=X CHAIN THE Y-STEP TURNS (the x=90+y=90 pair the
        // along-x rung banks: the x the FULL 0..1 span, the y/z the
        // centered bands)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::COPPER_CHAIN);
        w3.set_variant(8, 8, 8, crate::world::CHAIN_AXIS_X);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        let rung_x: Vec<_> = q3
            .iter()
            .filter(|quad| quad.tile == copper_tile)
            .collect();
        assert_eq!(rung_x.len(), 4, "the x-rung the same crossed pair wears");
        for quad in &rung_x {
            let x_min = quad.pts.iter().map(|p| p[0]).fold(f32::MAX, f32::min);
            let x_max = quad.pts.iter().map(|p| p[0]).fold(f32::MIN, f32::max);
            assert!(
                (x_min - 8.0).abs() < 1e-4 && ((x_max - 9.0).abs() < 1e-4),
                "the x span the FULL 0..1 band the x-rung keeps, got {x_min}..{x_max}"
            );
            for p in quad.pts.iter() {
                assert!(
                    p[1] - 8.0 >= 6.5 / 16.0 - 1e-5 && p[1] - 8.0 <= 9.5 / 16.0 + 1e-5,
                    "the y band the centered band the x-rung keeps, got {}",
                    p[1]
                );
                assert!(
                    (p[2] - 8.0).abs() >= 6.5 / 16.0 - 1e-5
                        && (p[2] - 8.0).abs() <= 9.5 / 16.0 + 1e-5,
                    "the z band the centered band the x-rung keeps, got {}",
                    p[2]
                );
            }
        }
        // THE IRON LINK ITS OWN MASTER RENDERS (the metal link the
        // iron_chain master the tile walk banks) + THE WAXED TWIN THE
        // BASE GEOMETRY (the blockstate the BASE model references —
        // the same crossed-pair count the bare rung wears)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::IRON_CHAIN);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        let iron: Vec<_> = q4.iter().filter(|quad| quad.tile == iron_tile).collect();
        assert_eq!(iron.len(), 4, "the iron link the crossed pair wears");
        let mut w5 = empty_world();
        put(&mut w5, 8, 8, 8, crate::world::WAXED_COPPER_CHAIN);
        let q5 = mesh_chunk(&w5, 0, 0, None);
        let waxed: Vec<_> = q5.iter().filter(|quad| quad.tile == copper_tile).collect();
        assert_eq!(
            waxed.len(),
            lone.len(),
            "the waxed twin the BASE models the blockstate references render"
        );
    }

    #[test]
    fn the_lantern_lane_renders_the_boxes_the_hooks_and_the_hanging_pair_wear() {
        // THE LANTERN LANE'S OWN LAWS (the dev-61 round — javap
        // LanternBlock the 26.2 jar the probe scripts/jar_work/
        // lantern_689.py banks): the blockstates the TEN the hanging
        // pair carry (the hanging=false identity + the hanging=true
        // pair), the template pair the body [5,0,5]->[11,7,11] +
        // top [6,7,6]->[10,9,10] boxes the SHAPE mirrors + the two
        // hook planes the ±45° y-axis rotations speak (shade:false
        // every hook face keeps), the waxed twins the BASE models
        // the references re-wear, the DOWN CULLFACE the body's
        // bottom face carries the below cell the cull verdict reads
        use mc_registry::generated::atlas as gen_atlas;
        let lantern_tile = gen_atlas::FACE_TILES[crate::world::LANTERN as usize][2];
        let soul_tile = gen_atlas::FACE_TILES[crate::world::SOUL_LANTERN as usize][2];
        let copper_tile = gen_atlas::FACE_TILES[crate::world::COPPER_LANTERN as usize][2];
        assert_eq!(gen_atlas::tile_name(lantern_tile), "lantern");
        assert_eq!(gen_atlas::tile_name(soul_tile), "soul_lantern");
        assert_ne!(lantern_tile, soul_tile, "the two voices the DISTINCT masters wear");
        assert_ne!(
            copper_tile, lantern_tile,
            "the bare copper rung the DISTINCT master wears"
        );
        // THE WAXED TWIN THE BASE MASTER RE-WEARS (the texture-
        // sharing law the vanilla waxed look keeps)
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_COPPER_LANTERN as usize][2],
            copper_tile,
            "the waxed head the bare master re-wears"
        );
        // THE STANDING LANTERN (the hanging=false default the
        // unstamped cell wears — the body's five faces the DOWN
        // CULLFACE the solid ground culls, the top box's five faces
        // + the two hook planes' four quads the element walk bakes:
        // 5 + 5 + 4 = 14 quads on the solid floor)
        let mut w = empty_world();
        put(&mut w, 8, 7, 8, crate::world::STONE); // the support the body's down face culls against
        put(&mut w, 8, 8, 8, crate::world::LANTERN);
        let q = mesh_chunk(&w, 0, 0, None);
        let standing: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == lantern_tile)
            .collect();
        assert_eq!(
            standing.len(),
            14,
            "the standing lantern the body+top+hooks census wears, got {}",
            standing.len()
        );
        for quad in &standing {
            assert!(!quad.translucent, "the lantern the OPAQUE pass rides");
            assert!(!quad.per_texel_alpha, "the lantern the binary cutout wears");
        }
        // THE BODY + TOP BOXES' OWN SPANS (the SHAPE pair the
        // template mirrors: the body y 0..7/16, the top y 7..9/16 —
        // the x/z the body's 5..11/16 band the center hug keeps)
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;
        for quad in &standing {
            for p in quad.pts.iter() {
                y_min = y_min.min(p[1]);
                y_max = y_max.max(p[1]);
            }
        }
        assert!(
            ((y_min - 8.0).abs() < 1e-5) && ((y_max - (8.0 + 11.0 / 16.0)).abs() < 1e-5),
            "the standing pair the 0..11/16 y span the body+top+hooks union keeps, got {y_min}..{y_max}"
        );
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
        for quad in &standing {
            for p in quad.pts.iter() {
                x_min = x_min.min(p[0]);
                x_max = x_max.max(p[0]);
            }
        }
        assert!(
            ((x_min - (8.0 + 5.0 / 16.0)).abs() < 1e-5)
                && ((x_max - (8.0 + 11.0 / 16.0)).abs() < 1e-5),
            "the body band the 5..11/16 x span keeps, got {x_min}..{x_max}"
        );
        // THE FLOATING LANTERN THE DOWN FACE SHOWS (the cullface the
        // transparent-below verdict the sixth face draws: 15 quads
        // the census wears)
        let mut wf = empty_world();
        put(&mut wf, 8, 8, 8, crate::world::LANTERN);
        let qf = mesh_chunk(&wf, 0, 0, None);
        let floating: Vec<_> = qf
            .iter()
            .filter(|quad| quad.tile == lantern_tile)
            .collect();
        assert_eq!(
            floating.len(),
            15,
            "the floating lantern the down face the sixth face draws, got {}",
            floating.len()
        );
        // THE HANGING PAIR THE ONE-PIXEL LIFT WEARS (the hanging=true
        // variant the placed byte wears — the body y 1..8/16 band
        // + the top 8..10/16 [the template's own DOWN face the
        // body's neck the ceiling lamp renders] + the hooks the
        // 10..16 span the template pair serves: 6 + 6 + 2 + 2 = 16
        // quads the census keeps [the below air the BOTH down faces
        // draw], the y span the 1..16/16 band the ceiling lamp rides)
        let mut wh = empty_world();
        put(&mut wh, 8, 8, 8, crate::world::LANTERN);
        wh.set_variant(8, 8, 8, crate::world::LANTERN_HANGING);
        let qh = mesh_chunk(&wh, 0, 0, None);
        let hanging: Vec<_> = qh
            .iter()
            .filter(|quad| quad.tile == lantern_tile)
            .collect();
        assert_eq!(
            hanging.len(),
            16,
            "the hanging lantern the template's own census the lifted pair wears, got {}",
            hanging.len()
        );
        let mut hy_min = f32::MAX;
        let mut hy_max = f32::MIN;
        for quad in &hanging {
            for p in quad.pts.iter() {
                hy_min = hy_min.min(p[1]);
                hy_max = hy_max.max(p[1]);
            }
        }
        assert!(
            ((hy_min - (8.0 + 1.0 / 16.0)).abs() < 1e-5) && ((hy_max - 9.0).abs() < 1e-5),
            "the hanging pair the 1..16/16 y span the ceiling lamp keeps, got {hy_min}..{hy_max}"
        );
        // THE SOUL + COPPER KINDS THEIR OWN MASTERS RENDER (the iron
        // seam the soul flame the bare rung the distinct tiles wear)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::SOUL_LANTERN);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        assert_eq!(
            q2.iter().filter(|quad| quad.tile == soul_tile).count(),
            15,
            "the soul lantern its own master renders"
        );
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::COPPER_LANTERN);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        assert_eq!(
            q3.iter().filter(|quad| quad.tile == copper_tile).count(),
            15,
            "the copper lantern its own master renders"
        );
        // THE WAXED TWIN THE BASE GEOMETRY RENDERS (the BASE models
        // the blockstate references wear — the waxed head the same
        // count the bare rung keeps)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::WAXED_COPPER_LANTERN);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        assert_eq!(
            q4.iter().filter(|quad| quad.tile == copper_tile).count(),
            15,
            "the waxed twin the BASE models the blockstate references render"
        );
    }

    #[test]
    fn the_stairs_lane_renders_the_slab_the_steps_and_the_turns_wear() {
        // THE STAIRS LANE'S OWN LAWS (the dev-62 round — javap
        // StairBlock the 26.2 jar the probe scripts/jar_work/
        // stairs_690.py banks): the blockstates the 40-key
        // facing/half/shape map the SAME rotation table every kind
        // carries (the waxed twins the BASE copper models re-wear),
        // the template trio the slab + step boxes speak (the straight
        // the slab 6 + the step 5 quads the element walk bakes, the
        // inner the second arm the L completes, the outer the quarter
        // step the corner alone keeps), the SHAPE the mesh-time walk
        // the neighbors answer, the per-quad tile the parent masters
        // the stem census resolves
        use mc_registry::generated::atlas as gen_atlas;
        let oak_tile = gen_atlas::FACE_TILES[crate::world::OAK_STAIRS as usize][2];
        let stone_tile = gen_atlas::FACE_TILES[crate::world::STONE_STAIRS as usize][2];
        let copper_tile = gen_atlas::FACE_TILES[crate::world::CUT_COPPER_STAIRS as usize][2];
        assert_eq!(gen_atlas::tile_name(oak_tile), "oak_planks");
        assert_eq!(gen_atlas::tile_name(stone_tile), "stone");
        assert_eq!(gen_atlas::tile_name(copper_tile), "cut_copper");
        assert_ne!(oak_tile, stone_tile, "the two voices the DISTINCT masters wear");
        assert_ne!(copper_tile, oak_tile, "the copper rung the DISTINCT master wears");
        // THE WAXED TWIN THE BASE MASTER RE-WEARS (the texture-sharing
        // law the vanilla waxed look keeps)
        assert_eq!(
            gen_atlas::FACE_TILES[crate::world::WAXED_CUT_COPPER_STAIRS as usize][2],
            copper_tile,
            "the waxed twin the bare master re-wears"
        );
        // THE SANDSTONE WALK THE SPLIT ROW KEEPS (the side ref the
        // canonical palette row banks)
        assert_eq!(
            gen_atlas::tile_name(gen_atlas::FACE_TILES[crate::world::SANDSTONE_STAIRS as usize][2]),
            "sandstone",
            "the sandstone side the palette row banks"
        );
        // THE SINGLE STAIR (the bottom-east straight the unstamped
        // default wears — the slab's 6 + the step's 5 faces the 11
        // quads the element walk bakes)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::OAK_STAIRS);
        w.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, false),
        );
        let q = mesh_chunk(&w, 0, 0, None);
        // THE TEST CELL'S OWN QUADS (the (8,8,8) span filter the
        // terrain's own planks quads the count never banks — the
        // generated world the same tile the scenery wears)
        let in_cell = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| {
                p[0] >= 8.0 - 1e-4
                    && p[0] <= 9.0 + 1e-4
                    && p[1] >= 8.0 - 1e-4
                    && p[1] <= 9.0 + 1e-4
                    && p[2] >= 8.0 - 1e-4
                    && p[2] <= 9.0 + 1e-4
            })
        };
        let straight: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(
            straight.len(),
            11,
            "the straight template the 11 quads bakes, got {}",
            straight.len()
        );
        // THE STEP BAND'S OWN SPAN (the y >= 0.5 quads the EAST half
        // the x 0.5..1.0 band keeps, the slab band the FULL footprint
        // the x 0..1 span wears)
        for quad in &straight {
            let y_min = quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min);
            let y_max = quad.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
            let x_min = quad.pts.iter().map(|p| p[0]).fold(f32::MAX, f32::min);
            let x_max = quad.pts.iter().map(|p| p[0]).fold(f32::MIN, f32::max);
            if y_min >= 8.5 - 1e-5 && y_max > 8.5 + 1e-5 {
                assert!(
                    x_min >= 8.5 - 1e-5 && x_max <= 9.0 + 1e-5,
                    "the east half the step band keeps, got x {x_min}..{x_max}"
                );
            }
        }
        let slab_x_max = straight
            .iter()
            .filter(|quad| quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min) < 8.5 - 1e-5)
            .map(|quad| quad.pts.iter().map(|p| p[0]).fold(f32::MIN, f32::max))
            .fold(f32::MIN, f32::max);
        assert!(
            slab_x_max >= 9.0 - 1e-5,
            "the full footprint the slab band keeps, got {slab_x_max}"
        );
        // THE NORTH TURN (the y=270 the three CW quarters the step the
        // z 0..0.5 half banks — the facing byte the variant wears)
        let mut wn = empty_world();
        put(&mut wn, 8, 8, 8, crate::world::OAK_STAIRS);
        wn.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_NORTH, false),
        );
        let qn = mesh_chunk(&wn, 0, 0, None);
        let north: Vec<_> = qn
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(north.len(), 11, "the north turn the same 11 quads bakes");
        for quad in &north {
            let y_min = quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min);
            let y_max = quad.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
            if y_min >= 8.5 - 1e-5 && y_max > 8.5 + 1e-5 {
                let z_min = quad.pts.iter().map(|p| p[2]).fold(f32::MAX, f32::min);
                let z_max = quad.pts.iter().map(|p| p[2]).fold(f32::MIN, f32::max);
                assert!(
                    z_min >= 8.0 - 1e-5 && z_max <= 8.5 + 1e-5,
                    "the north half the step band keeps, got z {z_min}..{z_max}"
                );
            }
        }
        // THE TOP HALF (the x=180 the slab the upper band + the step
        // the lower band the east half keeps)
        let mut wt = empty_world();
        put(&mut wt, 8, 8, 8, crate::world::OAK_STAIRS);
        wt.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, true),
        );
        let qt = mesh_chunk(&wt, 0, 0, None);
        let top: Vec<_> = qt
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(top.len(), 11, "the top half the same 11 quads bakes");
        for quad in &top {
            let y_min = quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min);
            if y_min < 8.5 - 1e-5 {
                let x_min = quad.pts.iter().map(|p| p[0]).fold(f32::MAX, f32::min);
                assert!(
                    x_min >= 8.5 - 1e-5,
                    "the lower band the step the east half keeps, got x {x_min}"
                );
            }
        }
        // THE OUTER CORNER (the front neighbor the perpendicular
        // flight turns — the SE quarter the step the corner keeps: the
        // y >= 0.5 band the x >= 0.5 AND z >= 0.5 spans)
        let mut wo = empty_world();
        put(&mut wo, 8, 8, 8, crate::world::OAK_STAIRS);
        wo.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, false),
        );
        put(&mut wo, 9, 8, 8, crate::world::OAK_STAIRS);
        wo.set_variant(
            9,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_SOUTH, false),
        );
        let qo = mesh_chunk(&wo, 0, 0, None);
        // THE TEST CELL'S OWN QUADS (the east-PLANE quads the shared
        // face never banks — the all-pts-x=9.0 faces the my east pair
        // + the neighbor's west pair wear, the 11-2=9 the count
        // keeps)
        let outer: Vec<_> = qo
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .filter(|quad| !quad.pts.iter().all(|p| p[0] >= 9.0 - 1e-4))
            .collect();
        assert_eq!(
            outer.len(),
            9,
            "the outer template the 9 interior quads bakes, got {}",
            outer.len()
        );
        let mut outer_quarter = false;
        for quad in &outer {
            let y_min = quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min);
            let y_max = quad.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
            if y_min >= 8.5 - 1e-5 && y_max > 8.5 + 1e-5 {
                let x_min = quad.pts.iter().map(|p| p[0]).fold(f32::MAX, f32::min);
                let z_min = quad.pts.iter().map(|p| p[2]).fold(f32::MAX, f32::min);
                if x_min >= 8.5 - 1e-5 && z_min >= 8.5 - 1e-5 {
                    outer_quarter = true;
                } else {
                    assert!(
                        x_min >= 8.5 - 1e-5,
                        "the east half the outer's step band keeps, got x {x_min} z {z_min}"
                    );
                }
            }
        }
        assert!(
            outer_quarter,
            "the SE quarter the outer's step the corner keeps"
        );
        // THE INNER CORNER (the back neighbor the perpendicular flight
        // wraps — the two arms the L completes: the 16 quads the slab
        // + the two steps bakes)
        let mut wi = empty_world();
        put(&mut wi, 8, 8, 8, crate::world::OAK_STAIRS);
        wi.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, false),
        );
        put(&mut wi, 7, 8, 8, crate::world::OAK_STAIRS);
        wi.set_variant(
            7,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_SOUTH, false),
        );
        let qi = mesh_chunk(&wi, 0, 0, None);
        // THE TEST CELL'S OWN QUADS (the west-PLANE quads the shared
        // face never banks — the all-pts-x=8.0 faces the my west pair
        // + the neighbor's east pair wear, the 15-2=13 the count
        // keeps [the inner template the slab 6 + the east step 5 +
        // the SW quarter's 4 faces the no-east no-down pair speaks])
        let inner: Vec<_> = qi
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .filter(|quad| !quad.pts.iter().all(|p| p[0] <= 8.0 + 1e-4))
            .collect();
        assert_eq!(
            inner.len(),
            13,
            "the inner template the 13 interior quads bakes, got {}",
            inner.len()
        );
        // THE STONE WALK ITS OWN MASTER WEARS (the per-quad tile the
        // stem census resolves — the stone stairs the stone master
        // banks)
        let mut ws = empty_world();
        put(&mut ws, 8, 8, 8, crate::world::STONE_STAIRS);
        ws.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, false),
        );
        let qs = mesh_chunk(&ws, 0, 0, None);
        let stone_quads: Vec<_> = qs
            .iter()
            .filter(|quad| quad.tile == stone_tile && in_cell(quad))
            .collect();
        assert_eq!(
            stone_quads.len(),
            11,
            "the stone stairs its own master the 11 quads wear"
        );
        // THE WAXED TWIN THE BASE GEOMETRY WEARS (the texture-sharing
        // law the vanilla waxed look keeps — the copper quads the base
        // model banks)
        let mut ww = empty_world();
        put(&mut ww, 8, 8, 8, crate::world::WAXED_CUT_COPPER_STAIRS);
        ww.set_variant(
            8,
            8,
            8,
            crate::world::stairs_variant(crate::world::STAIRS_FACING_EAST, false),
        );
        let qw = mesh_chunk(&ww, 0, 0, None);
        let waxed: usize = qw
            .iter()
            .filter(|quad| quad.tile == copper_tile && in_cell(quad))
            .count();
        assert_eq!(
            waxed, 11,
            "the waxed twin the BASE geometry the copper master wears"
        );
        // THE TURNS TABLE'S OWN CROSS-CHECK (the jar's 40-key parse the
        // world's hand table the same turns every state keeps — the
        // x the 0/180 the half pair wears, the y the quarter turns the
        // facing speaks)
        for facing in 0..4u8 {
            for half_top in [false, true] {
                for shape in 0..5u8 {
                    let (_, mx, my) = stairs_model_quads(
                        crate::world::OAK_STAIRS,
                        facing,
                        half_top,
                        shape,
                    );
                    let (wx, wy) =
                        crate::world::stairs_state_turns(facing, half_top, shape);
                    assert_eq!(
                        (mx, my),
                        (wx, wy),
                        "the turns the state (facing {facing}, half {half_top}, shape {shape}) the parse + the table agree"
                    );
                }
            }
        }
    }

    #[test]
    fn the_slab_lane_renders_the_bands_the_cullfaces_and_the_double_wears() {
        // THE SLAB LANE'S OWN LAWS (the dev-63 round — javap
        // SlabBlock the 26.2 jar the probe scripts/jar_work/
        // slab_691.py banks): the blockstates the three-key type map
        // carry (type=bottom/top/double — the waxed twins the BASE
        // copper models re-wear), the template pair the band boxes
        // speak (block/slab the [0,0,0]->[16,8,16] bottom band, the
        // cullfaces the six-direction pair banks, block/slab_top the
        // mirrored pair wears), the DOUBLE the parent's full-cube
        // model the same lane renders, the per-quad tile the parent
        // masters the stem census resolves (the sandstone walk the
        // bottom/side/top split rows keep, the smooth stone the
        // slab_side special master banks)
        use mc_registry::generated::atlas as gen_atlas;
        let oak_tile = gen_atlas::FACE_TILES[crate::world::OAK_SLAB as usize][2];
        assert_eq!(gen_atlas::tile_name(oak_tile), "oak_planks");
        let in_cell = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| {
                p[0] >= 8.0 - 1e-4
                    && p[0] <= 9.0 + 1e-4
                    && p[1] >= 8.0 - 1e-4
                    && p[1] <= 9.0 + 1e-4
                    && p[2] >= 8.0 - 1e-4
                    && p[2] <= 9.0 + 1e-4
            })
        };
        let y_min_of = |quad: &crate::mesher::Quad| {
            quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min)
        };
        let y_max_of = |quad: &crate::mesher::Quad| {
            quad.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max)
        };
        // THE BOTTOM BAND (the unstamped default the registerDefault
        // pin wears — the SIX faces the band bakes: the down + the up
        // + the four sides the open cell draws)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::OAK_SLAB);
        let q = mesh_chunk(&w, 0, 0, None);
        let bottom: Vec<_> = q.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(
            bottom.len(),
            6,
            "the bottom band the 6 quads bakes, got {}",
            bottom.len()
        );
        for quad in &bottom {
            let y_min = y_min_of(quad);
            let y_max = y_max_of(quad);
            assert!(
                y_min >= 8.0 - 1e-5 && y_max <= 8.5 + 1e-5,
                "the 0..8 band the bottom slab keeps, got {y_min}..{y_max}"
            );
        }
        // THE DOWN FACE + THE UP FACE THE BAND'S OWN PLANES WEAR (the
        // one quad each the flat pair answers)
        let down_faces = bottom.iter().filter(|quad| {
            quad.pts.iter().all(|p| (p[1] - 8.0).abs() < 1e-5)
        }).count();
        let up_faces = bottom.iter().filter(|quad| {
            quad.pts.iter().all(|p| (p[1] - 8.5).abs() < 1e-5)
        }).count();
        assert_eq!(down_faces, 1, "the down face the band's floor banks");
        assert_eq!(up_faces, 1, "the up face the band's ceiling banks");
        // THE TOP BAND (the type=1 the mirrored pair the upper half
        // banks — the same 6 quads the y 0.5..1.0 span wears)
        let mut wt = empty_world();
        put(&mut wt, 8, 8, 8, crate::world::OAK_SLAB);
        wt.set_variant(8, 8, 8, crate::world::slab_variant(crate::world::SLAB_TYPE_TOP));
        let qt = mesh_chunk(&wt, 0, 0, None);
        let top: Vec<_> = qt.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(top.len(), 6, "the top band the same 6 quads bakes");
        for quad in &top {
            let y_min = y_min_of(quad);
            let y_max = y_max_of(quad);
            assert!(
                y_min >= 8.5 - 1e-5 && y_max <= 9.0 + 1e-5,
                "the 8..16 band the top slab keeps, got {y_min}..{y_max}"
            );
        }
        // THE DOUBLE (the type=2 the parent's full-cube model the same
        // lane renders — the SIX faces the 0..1 span the cube wears)
        let mut wd = empty_world();
        put(&mut wd, 8, 8, 8, crate::world::OAK_SLAB);
        wd.set_variant(8, 8, 8, crate::world::slab_variant(crate::world::SLAB_TYPE_DOUBLE));
        let qd = mesh_chunk(&wd, 0, 0, None);
        let dbl: Vec<_> = qd.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(dbl.len(), 6, "the double the full cube the 6 quads bakes");
        for quad in &dbl {
            let y_min = y_min_of(quad);
            let y_max = y_max_of(quad);
            assert!(
                y_min >= 8.0 - 1e-5 && y_max <= 9.0 + 1e-5,
                "the full span the double keeps, got {y_min}..{y_max}"
            );
        }
        // THE DOWN CULLFACE'S OWN VERDICT (the stone below the
        // solid-render neighbor the floor face culls — the 5 quads the
        // band keeps)
        let mut wb = empty_world();
        put(&mut wb, 8, 8, 8, crate::world::OAK_SLAB);
        put(&mut wb, 8, 7, 8, crate::world::STONE);
        let qb = mesh_chunk(&wb, 0, 0, None);
        let culled_bottom: Vec<_> = qb
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(
            culled_bottom.len(),
            5,
            "the stone below the down face culls, got {}",
            culled_bottom.len()
        );
        assert!(
            culled_bottom
                .iter()
                .all(|quad| !quad.pts.iter().all(|p| (p[1] - 8.0).abs() < 1e-5)),
            "the floor face the solid ground never draws"
        );
        // THE SIDE CULLFACE'S OWN VERDICT (the stone north the
        // north-facing side face culls — the four sides the open
        // three + the up + the down the count keeps)
        let mut wsn = empty_world();
        put(&mut wsn, 8, 8, 8, crate::world::OAK_SLAB);
        put(&mut wsn, 8, 8, 7, crate::world::STONE);
        let qsn = mesh_chunk(&wsn, 0, 0, None);
        let side_culled: Vec<_> = qsn
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(
            side_culled.len(),
            5,
            "the stone north the north face culls, got {}",
            side_culled.len()
        );
        // THE UP CULLFACE'S OWN VERDICT (the top slab the stone above
        // the ceiling face culls — the 5 quads the band keeps)
        let mut wu = empty_world();
        put(&mut wu, 8, 8, 8, crate::world::OAK_SLAB);
        wu.set_variant(8, 8, 8, crate::world::slab_variant(crate::world::SLAB_TYPE_TOP));
        put(&mut wu, 8, 9, 8, crate::world::STONE);
        let qu = mesh_chunk(&wu, 0, 0, None);
        let up_culled: Vec<_> = qu
            .iter()
            .filter(|quad| quad.tile == oak_tile && in_cell(quad))
            .collect();
        assert_eq!(
            up_culled.len(),
            5,
            "the stone above the up face culls, got {}",
            up_culled.len()
        );
        assert!(
            up_culled
                .iter()
                .all(|quad| !quad.pts.iter().all(|p| (p[1] - 9.0).abs() < 1e-5)),
            "the ceiling face the solid roof never draws"
        );
        // THE SANDSTONE WALK THE SPLIT ROWS KEEP (the #bottom/#side/
        // #top refs the three masters answer — the down face the
        // sandstone_bottom tile wears, the up the sandstone_top, the
        // four sides the sandstone side keeps)
        let sand_side = gen_atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "sandstone")
            .expect("the sandstone side master the census banks") as u16;
        let sand_bottom = gen_atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "sandstone_bottom")
            .expect("the sandstone bottom master the split walk banks") as u16;
        let sand_top = gen_atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "sandstone_top")
            .expect("the sandstone top master the split walk banks") as u16;
        let mut ws = empty_world();
        put(&mut ws, 8, 8, 8, crate::world::SANDSTONE_SLAB);
        let qs = mesh_chunk(&ws, 0, 0, None);
        let sand: Vec<_> = qs.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(sand.len(), 6, "the sandstone slab the 6 quads bakes");
        for quad in &sand {
            if quad.pts.iter().all(|p| (p[1] - 8.0).abs() < 1e-5) {
                assert_eq!(quad.tile, sand_bottom, "the down face the bottom split wears");
            } else if quad.pts.iter().all(|p| (p[1] - 8.5).abs() < 1e-5) {
                assert_eq!(quad.tile, sand_top, "the up face the top split wears");
            } else {
                assert_eq!(quad.tile, sand_side, "the side faces the side master wears");
            }
        }
        // THE SMOOTH STONE'S OWN SPECIAL MASTER (the #side ref the
        // slab_side png the ONLY own tile the family banks — the four
        // sides the special master wears, the up/down the smooth_stone
        // pair keeps)
        let smooth_side = gen_atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "smooth_stone_slab_side")
            .expect("the slab_side master the split walk banks") as u16;
        let smooth = gen_atlas::TILE_NAMES
            .iter()
            .position(|n| *n == "smooth_stone")
            .expect("the smooth_stone master the census banks") as u16;
        let mut wm = empty_world();
        put(&mut wm, 8, 8, 8, crate::world::SMOOTH_STONE_SLAB);
        let qm = mesh_chunk(&wm, 0, 0, None);
        let smooths: Vec<_> = qm.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(smooths.len(), 6, "the smooth stone slab the 6 quads bakes");
        for quad in &smooths {
            if quad.pts.iter().all(|p| (p[1] - 8.0).abs() < 1e-5)
                || quad.pts.iter().all(|p| (p[1] - 8.5).abs() < 1e-5)
            {
                assert_eq!(quad.tile, smooth, "the flat pair the smooth_stone master wears");
            } else {
                assert_eq!(
                    quad.tile, smooth_side,
                    "the side faces the OWN slab_side master wears"
                );
            }
        }
        // THE WAXED TWIN THE BASE GEOMETRY THE COPPER MASTER WEARS
        // (the blockstate's own bare-model refs the same quads bake)
        let copper_tile = gen_atlas::FACE_TILES[crate::world::CUT_COPPER_SLAB as usize][2];
        assert_eq!(gen_atlas::tile_name(copper_tile), "cut_copper");
        let mut ww = empty_world();
        put(&mut ww, 8, 8, 8, crate::world::WAXED_CUT_COPPER_SLAB);
        let qw = mesh_chunk(&ww, 0, 0, None);
        let waxed: Vec<_> = qw.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(
            waxed.len(),
            6,
            "the waxed twin the base geometry the 6 quads bakes"
        );
        assert!(
            waxed.iter().all(|quad| quad.tile == copper_tile),
            "the waxed twin the bare copper master re-wears"
        );
        // THE OAK DOUBLE THE SAME PARENT MASTER WEARS (the parent's
        // full-cube model the oak_planks tile the six faces keep)
        assert!(
            dbl.iter().all(|quad| quad.tile == oak_tile),
            "the double the parent master re-wears"
        );
    }

    #[test]
    fn the_chest_lane_renders_the_boxes_the_latch_and_the_doubles_wear() {
        // THE CHEST LANE'S OWN LAWS (the dev-64 round — the hand-built
        // boxes the SHAPES law's own px vocabulary wears): the SINGLE
        // the [1,0,1]->[15,14,15] box the 6 quads bake (the down + up
        // + the four sides the open cell draws), the front tile the
        // facing face wears, the sides the four vertical bands keep,
        // the top/bottom the centered pair banks, the DOUBLES the
        // connected side the flush edge wears (the 15/16 band the
        // flat seam keeps), the waxed twin the bare geometry re-wears
        use mc_registry::generated::atlas as gen_atlas;
        let tiles = gen_atlas::FACE_TILES[crate::world::CHEST as usize];
        let front_tile = tiles[5];
        let side_tile = tiles[0];
        let top_tile = tiles[2];
        let bottom_tile = tiles[3];
        assert_ne!(front_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        let in_cell = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| {
                p[0] >= 8.0 - 1e-4
                    && p[0] <= 9.0 + 1e-4
                    && p[1] >= 8.0 - 1e-4
                    && p[1] <= 9.0 + 1e-4
                    && p[2] >= 8.0 - 1e-4
                    && p[2] <= 9.0 + 1e-4
            })
        };
        let y_min_of = |quad: &crate::mesher::Quad| {
            quad.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min)
        };
        let y_max_of = |quad: &crate::mesher::Quad| {
            quad.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max)
        };
        let x_max_of = |quad: &crate::mesher::Quad| {
            quad.pts.iter().map(|p| p[0]).fold(f32::MIN, f32::max)
        };
        // THE SINGLE CHEST (the unstamped default the registerDefault
        // pin wears — the SIX quads the box bakes)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::CHEST);
        let q = mesh_chunk(&w, 0, 0, None);
        let cell: Vec<_> = q.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(
            cell.len(),
            6,
            "the single chest the 6 quads bakes, got {}",
            cell.len()
        );
        // THE 14/16 BAND (the y span the 0..14 px box keeps — the
        // world y 8.0..8.875 the cell's own base wears)
        for quad in &cell {
            let y_min = y_min_of(quad);
            let y_max = y_max_of(quad);
            assert!(
                y_min >= 8.0 - 1e-5 && y_max <= 8.0 + 14.0 / 16.0 + 1e-5,
                "the 0..14 band the single keeps, got {y_min}..{y_max}"
            );
            assert!(
                !quad.translucent,
                "the chest the opaque cutout lane wears"
            );
        }
        // THE TILE LAWS THE FACE ASSIGNMENT SPEAKS (the facing=north
        // default the front tile the -Z face wears, the sides the
        // four vertical bands keep, the top/bottom the centered pair)
        let north_face = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| (p[2] - (8.0 + 1.0 / 16.0)).abs() < 1e-5)
        };
        assert!(
            cell.iter()
                .filter(|quad| north_face(quad))
                .all(|quad| quad.tile == front_tile),
            "the facing face the front tile wears"
        );
        assert_eq!(
            cell.iter().filter(|quad| north_face(quad)).count(),
            1,
            "the one north face the box bakes"
        );
        assert!(
            cell.iter()
                .filter(|quad| quad.tile == side_tile)
                .count()
                >= 3,
            "the side faces the side tile keeps"
        );
        assert!(
            cell.iter().any(|quad| quad.tile == top_tile),
            "the top face the lid tile wears"
        );
        assert!(
            cell.iter().any(|quad| quad.tile == bottom_tile),
            "the bottom face the base tile wears"
        );
        // THE UV LAW (the v = y_hi - y fold — the top edge the row 0
        // the lid strip samples: the top-face quad's v the 0 the row
        // banks, the bottom edge the 14/16 the base keeps)
        let top_face_quad = cell
            .iter()
            .find(|quad| quad.tile == top_tile)
            .expect("the top face the lid tile wears");
        assert!(
            top_face_quad
                .uvs
                .iter()
                .all(|uv| uv[0] >= 1.0 / 16.0 - 1e-6 && uv[0] <= 15.0 / 16.0 + 1e-6),
            "the top face the centered 1..15 u band samples"
        );
        // THE FACING ROTATION (the east-facing chest the front tile
        // the +X face wears — the placement stamp's own byte the box
        // reads)
        let mut we = empty_world();
        put(&mut we, 8, 8, 8, crate::world::CHEST);
        we.set_variant(
            8,
            8,
            8,
            crate::world::chest_variant(crate::world::CHEST_FACING_EAST, crate::world::CHEST_TYPE_SINGLE),
        );
        let qe = mesh_chunk(&we, 0, 0, None);
        let celle: Vec<_> = qe.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(celle.len(), 6, "the east chest the same 6 quads bakes");
        let east_face = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| (p[0] - (8.0 + 15.0 / 16.0)).abs() < 1e-5)
        };
        assert!(
            celle
                .iter()
                .filter(|quad| east_face(quad))
                .all(|quad| quad.tile == front_tile),
            "the east face the front tile the facing rotation wears"
        );
        assert_eq!(
            celle.iter().filter(|quad| east_face(quad)).count(),
            1,
            "the one east face the rotated box bakes"
        );
        // THE DOUBLE (the LEFT type the connected EAST side the flush
        // edge wears — the x 2..16 arm the 15/16 band keeps, the 6
        // quads the same box count bakes)
        let mut wd = empty_world();
        put(&mut wd, 8, 8, 8, crate::world::CHEST);
        wd.set_variant(
            8,
            8,
            8,
            crate::world::chest_variant(crate::world::CHEST_FACING_NORTH, crate::world::CHEST_TYPE_LEFT),
        );
        let qd = mesh_chunk(&wd, 0, 0, None);
        let celld: Vec<_> = qd.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(celld.len(), 6, "the double the same 6 quads bakes");
        for quad in &celld {
            let y_max = y_max_of(quad);
            assert!(
                y_max <= 8.0 + 15.0 / 16.0 + 1e-5,
                "the 15/16 band the double keeps, got {y_max}"
            );
        }
        let cell_x_hi = celld
            .iter()
            .map(|quad| x_max_of(quad))
            .fold(f32::MIN, f32::max);
        assert!(
            (cell_x_hi - 9.0).abs() < 1e-5,
            "the flush east edge the LEFT double wears, got {cell_x_hi}"
        );
        // THE WAXED TWIN THE BARE GEOMETRY RE-WEARS (the same box the
        // bare copper tiles the waxed law keeps)
        let mut ww = empty_world();
        put(&mut ww, 8, 8, 8, crate::world::WAXED_COPPER_CHEST);
        let qw = mesh_chunk(&ww, 0, 0, None);
        let cellw: Vec<_> = qw.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(cellw.len(), 6, "the waxed twin the same box bakes");
        let waxed_tiles = gen_atlas::FACE_TILES[crate::world::WAXED_COPPER_CHEST as usize];
        assert!(
            cellw.iter().all(|quad| waxed_tiles.contains(&quad.tile)),
            "the waxed twin the bare copper tiles wears"
        );
    }

    #[test]
    fn the_barrel_lane_renders_the_lid_travel_and_the_open_speaks() {
        // THE BARREL LANE'S OWN LAWS (the dev-65 round — the FACING
        // x/y turns the blockstate's model law speaks): the full cube
        // the six quads every state wears, the lid tile the facing
        // face travels (up the UP face keeps, down the DOWN face
        // wears, north the north face banks — the blockstate's x=90/
        // x=180 turns the same rotation the model law speaks), the
        // OPEN bit the lid tile the barrel_top_open master swaps
        use mc_registry::generated::atlas as gen_atlas;
        let row = gen_atlas::FACE_TILES[crate::world::BARREL as usize];
        let top_tile = row[2];
        let bottom_tile = row[3];
        let side_tile = row[0];
        let open_tile = gen_atlas::TILE_BARREL_TOP_OPEN;
        assert_ne!(top_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        let in_cell = |quad: &crate::mesher::Quad| {
            quad.pts.iter().all(|p| {
                p[0] >= 8.0 - 1e-4
                    && p[0] <= 9.0 + 1e-4
                    && p[1] >= 8.0 - 1e-4
                    && p[1] <= 9.0 + 1e-4
                    && p[2] >= 8.0 - 1e-4
                    && p[2] <= 9.0 + 1e-4
            })
        };
        let face_of = |quad: &crate::mesher::Quad| -> Option<(f32, f32, f32)> {
            // the face's own constant axis (the plane the four
            // vertices share)
            let xs = quad.pts.iter().map(|p| p[0]).collect::<Vec<_>>();
            let ys = quad.pts.iter().map(|p| p[1]).collect::<Vec<_>>();
            let zs = quad.pts.iter().map(|p| p[2]).collect::<Vec<_>>();
            if xs.iter().all(|v| (*v - xs[0]).abs() < 1e-5) {
                Some((xs[0], -1.0, -1.0))
            } else if ys.iter().all(|v| (*v - ys[0]).abs() < 1e-5) {
                Some((-1.0, ys[0], -1.0))
            } else if zs.iter().all(|v| (*v - zs[0]).abs() < 1e-5) {
                Some((-1.0, -1.0, zs[0]))
            } else {
                None
            }
        };
        let tile_at = |quads: &[&crate::mesher::Quad], plane: (f32, f32, f32)| -> Option<u16> {
            quads
                .iter()
                .find(|quad| {
                    face_of(quad)
                        .map(|c| {
                            (c.0 >= 0.0 && (c.0 - plane.0).abs() < 1e-5)
                                || (c.1 >= 0.0 && (c.1 - plane.1).abs() < 1e-5)
                                || (c.2 >= 0.0 && (c.2 - plane.2).abs() < 1e-5)
                        })
                        .unwrap_or(false)
                })
                .map(|quad| quad.tile)
        };
        // THE UP LID (the unstamped default the registerDefault pin
        // wears — the top tile the UP face keeps, the bottom the
        // DOWN face wears)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::BARREL);
        let q = mesh_chunk(&w, 0, 0, None);
        let cell: Vec<_> = q.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(cell.len(), 6, "the barrel the 6 quads bakes, got {}", cell.len());
        assert_eq!(
            tile_at(&cell, (-1.0, 9.0, -1.0)),
            Some(top_tile),
            "the up lid the UP face keeps"
        );
        assert_eq!(
            tile_at(&cell, (-1.0, 8.0, -1.0)),
            Some(bottom_tile),
            "the bottom the DOWN face wears"
        );
        // THE NORTH LID (the facing=2 the x=90 turn the north face
        // the lid wears — the south face the bottom master carries)
        let mut wn = empty_world();
        put(&mut wn, 8, 8, 8, crate::world::BARREL);
        wn.set_variant(
            8,
            8,
            8,
            crate::world::barrel_variant(crate::world::BARREL_FACING_NORTH, false),
        );
        let qn = mesh_chunk(&wn, 0, 0, None);
        let celln: Vec<_> = qn.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(celln.len(), 6, "the north barrel the 6 quads bakes");
        assert_eq!(
            tile_at(&celln, (-1.0, -1.0, 8.0)),
            Some(top_tile),
            "the north lid the north face banks"
        );
        assert_eq!(
            tile_at(&celln, (-1.0, -1.0, 9.0)),
            Some(bottom_tile),
            "the south face the bottom master carries"
        );
        // THE OPEN LID (the OPEN bit the barrel_top_open master the
        // lid face swaps — the rest the closed law keeps)
        let mut wo = empty_world();
        put(&mut wo, 8, 8, 8, crate::world::BARREL);
        wo.set_variant(
            8,
            8,
            8,
            crate::world::barrel_variant(crate::world::BARREL_FACING_UP, true),
        );
        let qo = mesh_chunk(&wo, 0, 0, None);
        let cello: Vec<_> = qo.iter().filter(|quad| in_cell(quad)).collect();
        assert_eq!(cello.len(), 6, "the open barrel the 6 quads bakes");
        assert_eq!(
            tile_at(&cello, (-1.0, 9.0, -1.0)),
            Some(open_tile),
            "the open lid the barrel_top_open master swaps"
        );
        assert_eq!(
            tile_at(&cello, (-1.0, 8.0, -1.0)),
            Some(bottom_tile),
            "the bottom the open state keeps"
        );
        // THE SIDES THE OPEN STATE KEEPS (the four vertical faces
        // the side tile wears — the open arm the sides never swaps)
        assert!(
            cello.iter().filter(|quad| quad.tile == side_tile).count() >= 4,
            "the sides the open state keeps"
        );
    }

    #[test]
    fn the_fire_lane_renders_the_fan_the_blockstate_speaks() {
        use mc_registry::generated::atlas as gen_atlas;
        let fire_tile = gen_atlas::FACE_TILES[crate::world::FIRE as usize][2];
        let soul_tile = gen_atlas::FACE_TILES[crate::world::SOUL_FIRE as usize][2];
        assert_ne!(fire_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        assert_ne!(soul_tile, gen_atlas::TILE_NONE, "the soul tile the regen banks");
        // THE SECOND FRAME'S OWN BASE TILES — the *_1 families the
        // appended frame-00 tiles bake (the blockstate's own apply
        // pairs the BOTH-families fan speak)
        let fire1_tile = gen_atlas::FIRE_1_FRAMES[0];
        let soul1_tile = gen_atlas::SOUL_FIRE_1_FRAMES[0];
        assert_ne!(fire1_tile, fire_tile, "the two families' own tiles");
        assert_ne!(soul1_tile, soul_tile, "the soul families' own tiles");
        // THE FREESTANDING FAN — fire on stone (the below cell the
        // gate's sturdy arm answers): the 40 quads the floor pair (8)
        // plus the four side groups (32) wear, the TWO frame families'
        // own tiles on the quads (the blockstate's apply pairs the
        // both-families fan speaks — 20/20 the even split keeps), the
        // planes the cell's own footprint keeps and
        // the 22.4-texel rise the rotated tips' honest excursion wears
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::FIRE);
        let q = mesh_chunk(&w, 0, 0, None);
        let fan: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
            .collect();
        assert_eq!(fan.len(), 40, "the floor pair's 8 + the four groups' 32 the fan wears");
        assert_eq!(
            fan.iter().filter(|quad| quad.tile == fire_tile).count(),
            20,
            "the *_0 family's own half the even split keeps"
        );
        assert_eq!(
            fan.iter().filter(|quad| quad.tile == fire1_tile).count(),
            20,
            "the *_1 family's own half the even split keeps"
        );
        for quad in &fan {
            assert!(!quad.translucent, "the flame the opaque cutout lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1], p[2] - 8.0);
                assert!(
                    (-1e-6..=1.0 + 1e-6).contains(&fx) && (-1e-6..=1.0 + 1e-6).contains(&fz),
                    "the planes the cell's own footprint keeps: ({fx},{fz})"
                );
                // the vertical span the rescaled rotation wears: the
                // side walls' 22.4-texel rise (1.4), the rotated floor
                // planes' tips the ±22.5° + rescale fold lifts to
                // 1.42071 and dips to -0.02071 (the honest excursion
                // the vanilla fan's own bake carries)
                assert!(
                    (8.0 - 0.0208..=9.0 + 1.4208).contains(&fy),
                    "the fan's rise the rotated tips' excursion wears: {fy}"
                );
            }
            for uv in &quad.uvs {
                assert!(
                    (0.0..=1.0).contains(&uv[0]) && (0.0..=1.0).contains(&uv[1]),
                    "the /16 fold the tile space keeps: {uv:?}"
                );
            }
        }
        // THE GATE'S OWN SILENCE — the log beside the grounded fire the
        // props NEVER reach (the below stone's sturdy arm the default
        // state serves): the fan the same 40 quads keeps, no walls the
        // log's burnable odds could add
        let mut w_gate = empty_world();
        put(&mut w_gate, 8, 8, 8, STONE);
        put(&mut w_gate, 8, 9, 8, crate::world::FIRE);
        put(&mut w_gate, 9, 9, 8, crate::world::LOG);
        let q_gate = mesh_chunk(&w_gate, 0, 0, None);
        assert_eq!(
            q_gate
                .iter()
                .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
                .count(),
            40,
            "the grounded fire the gate's default state keeps freestanding"
        );
        // THE FLOATING FIRE'S OWN WALL — the below air the gate refuses,
        // the east log the canBurn read answers: the east group's eight
        // quads alone, the wall plane the y=90 quarter's (x,z) -> (1-z,
        // x) turn speaks (the template's z=0.01 the 0.999375 plane keeps)
        let mut w_east = empty_world();
        put(&mut w_east, 8, 9, 8, crate::world::FIRE);
        put(&mut w_east, 9, 9, 8, crate::world::LOG);
        let q_east = mesh_chunk(&w_east, 0, 0, None);
        let wall: Vec<_> = q_east
            .iter()
            .filter(|quad| quad.tile == fire_tile || quad.tile == fire1_tile)
            .collect();
        assert_eq!(wall.len(), 8, "the east group's four models x the two faces");
        assert_eq!(
            wall.iter().filter(|quad| quad.tile == fire_tile).count(),
            4,
            "the east group's *_0 half the even split keeps"
        );
        assert_eq!(
            wall.iter().filter(|quad| quad.tile == fire1_tile).count(),
            4,
            "the east group's *_1 half the even split keeps"
        );
        for quad in &wall {
            for p in &quad.pts {
                assert!(
                    (p[0] - 8.0 - (1.0 - 0.01 / 16.0)).abs() < 1e-5,
                    "the east plane the y=90 turn speaks: {}",
                    p[0] - 8.0
                );
                assert!(
                    (8.0..=9.0 + 1e-6).contains(&p[2]) && (9.0..=10.4 + 1e-4).contains(&p[1]),
                    "the wall's span the template element keeps: ({},{})",
                    p[1], p[2]
                );
            }
        }
        // THE NEIGHBORS' OWN FACES — the flame cell the ground's top
        // face draws against (the replaceable().noCollision() rows the
        // is_transparent verdict wears): the stone's top face at the
        // shared plane renders
        assert!(
            q.iter().any(|quad| {
                quad.tile == gen_atlas::FACE_TILES[STONE as usize][2]
                    && quad.pts.iter().all(|p| (p[1] - 9.0).abs() < 1e-6)
            }),
            "the stone's top face the flame cell draws against"
        );
        // THE SOUL FAN — the unconditional column's own 40 quads, the
        // soul tile the kind's own row banks
        let mut w_soul = empty_world();
        put(&mut w_soul, 8, 8, 8, STONE);
        put(&mut w_soul, 8, 9, 8, crate::world::SOUL_FIRE);
        let q_soul = mesh_chunk(&w_soul, 0, 0, None);
        let soul_fan: Vec<_> = q_soul
            .iter()
            .filter(|quad| quad.tile == soul_tile || quad.tile == soul1_tile)
            .collect();
        assert_eq!(soul_fan.len(), 40, "the soul fan the no-when column always wears");
        assert_eq!(
            soul_fan.iter().filter(|quad| quad.tile == soul_tile).count(),
            20,
            "the soul *_0 family's own half the even split keeps"
        );
        assert_eq!(
            soul_fan.iter().filter(|quad| quad.tile == soul1_tile).count(),
            20,
            "the soul *_1 family's own half the even split keeps"
        );
        assert!(
            q_soul.iter()
                .all(|quad| quad.tile != fire_tile && quad.tile != fire1_tile),
            "the soul cell the plain flame's tiles never wears"
        );
    }

    #[test]
    fn the_candle_lane_the_template_bakes() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE CANDLE PAIR'S OWN TILES — the regen the two masters bank
        // (the blockstate's one-candle rows the `#all` refs resolve),
        // the two states' own tiles the kind rows keep
        let unlit_tile = gen_atlas::FACE_TILES[crate::world::CANDLE as usize][2];
        let lit_tile = gen_atlas::FACE_TILES[crate::world::CANDLE_LIT as usize][2];
        assert_ne!(unlit_tile, gen_atlas::TILE_NONE, "the tile the regen banks");
        assert_ne!(lit_tile, gen_atlas::TILE_NONE, "the lit tile the regen banks");
        assert_ne!(unlit_tile, lit_tile, "the two states' own masters");
        // THE STICK'S OWN BAKE — the unlit candle on stone: the 9 quads
        // the box's six faces (the down face the cullface verdict
        // culled) plus the two wick planes' four wear, the template's
        // own texel geometry the cell keeps (the box [7,0,7]->[9,6,9],
        // the wick planes the ±45° turns bake — the vertices never
        // leave the cell, the rise the 7-texel wick tip wears)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::CANDLE);
        let q = mesh_chunk(&w, 0, 0, None);
        let stick: Vec<_> = q
            .iter()
            .filter(|quad| quad.tile == unlit_tile)
            .collect();
        assert_eq!(stick.len(), 9, "the box's five faces + the wick's four");
        for quad in &stick {
            assert!(!quad.translucent, "the candle the opaque cutout lane wears");
            for p in &quad.pts {
                let (fx, fy, fz) = (p[0] - 8.0, p[1], p[2] - 8.0);
                assert!(
                    (-1e-6..=1.0 + 1e-6).contains(&fx) && (-1e-6..=1.0 + 1e-6).contains(&fz),
                    "the stick the cell's own footprint keeps: ({fx},{fz})"
                );
                assert!(
                    (9.0 - 1e-6..=9.0 + 7.0 / 16.0 + 1e-6).contains(&fy),
                    "the rise the box's 6 texels + the wick's 7th keeps: {fy}"
                );
            }
        }
        // THE SHADE TERM'S OWN READ — the elements' `shade: true` the
        // vanilla tesselation's direction table answers: the up face's
        // corners the 1.0 keeps, the horizontals the 0.8 wears (the
        // lightmap the no-grid convention the unshaded 1.0 carries —
        // the colors the shade alone splits)
        assert!(
            stick.iter().any(|quad| {
                quad.pts.iter().all(|p| (p[1] - 9.375).abs() < 1e-5)
                    && quad.colors.iter().all(|c| (c[0] - 1.0).abs() < 1e-5)
            }),
            "the box top's up face the 1.0 shade keeps"
        );
        assert!(
            stick.iter().any(|quad| {
                quad.colors.iter().all(|c| (c[0] - 0.8).abs() < 1e-5)
            }),
            "the horizontal faces the 0.8 shade wears"
        );
        assert!(
            stick.iter().all(|quad| {
                quad.colors.iter().all(|c| (0.5..=1.0).contains(&c[0]))
            }),
            "the shade table the cube path's constants keeps"
        );
        // THE AIR BELOW'S OWN TEN — the floating candle the down face
        // the cull keeps: the full 10 quads the template's three
        // elements wear
        let mut w_float = empty_world();
        put(&mut w_float, 8, 9, 8, crate::world::CANDLE);
        let q_float = mesh_chunk(&w_float, 0, 0, None);
        assert_eq!(
            q_float
                .iter()
                .filter(|quad| quad.tile == unlit_tile)
                .count(),
            10,
            "the down face the air-below cell never culls"
        );
        // THE LIT KIND'S OWN TILES — the same geometry the lit master
        // wears: 9 quads on the stone, the lit tile the quads carry
        let mut w_lit = empty_world();
        put(&mut w_lit, 8, 8, 8, STONE);
        put(&mut w_lit, 8, 9, 8, crate::world::CANDLE_LIT);
        let q_lit = mesh_chunk(&w_lit, 0, 0, None);
        let lit: Vec<_> = q_lit.iter().filter(|quad| quad.tile == lit_tile).collect();
        assert_eq!(lit.len(), 9, "the lit candle the same template bakes");
        assert!(
            q_lit.iter().all(|quad| quad.tile != unlit_tile),
            "the lit cell the unlit master never wears"
        );
    }

    #[test]
    fn the_fire_anim_remap_the_playlist_walks() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE PLAYLIST'S OWN WALK — the vanilla TextureAtlasSprite
        // animator's per-tick step the frametime 1 cadence speaks: tick
        // t the playlist position t % 32 reads, the frames the strip's
        // own rows bank. fire_0's mcmeta [16..31, 0..15] the playlist
        // the jar banks, fire_1's empty animation the sequential rule
        // serves — the two families the OUT-OF-PHASE flicker speaks.
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            gen_atlas::FIRE_0_FRAMES[16],
            "tick 0 the playlist's own head frame 16 wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 1),
            gen_atlas::FIRE_0_FRAMES[17],
            "tick 1 the walk's own next row"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 15),
            gen_atlas::FIRE_0_FRAMES[31],
            "tick 15 the playlist's first leg ends"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 16),
            gen_atlas::FIRE_0_FRAMES[0],
            "tick 16 the second leg the frame 0 row opens"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 31),
            gen_atlas::FIRE_0_FRAMES[15],
            "tick 31 the playlist's own tail"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 32),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the 32-tick cycle the playlist's own period wears (frametime 1)"
        );
        // THE SECOND FAMILY'S OWN STRIP — the sequential walk the empty
        // animation serves, the two families the phase split speaks
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 0),
            gen_atlas::FIRE_1_FRAMES[0],
            "tick 0 the strip's own first frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 1),
            gen_atlas::FIRE_1_FRAMES[1],
            "tick 1 the sequential walk's next row"
        );
        assert_ne!(
            gen_atlas::anim_tile(gen_atlas::FIRE_1_FRAME0, 0),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the families' phase split the distinct frames keep"
        );
        // THE SOUL HALF'S OWN STRIPS — the same playlists the kind's
        // own frame arrays answer, never the plain flame's rows
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_0_FRAME0, 0),
            gen_atlas::SOUL_FIRE_0_FRAMES[16],
            "the soul *_0 strip the playlist's own head wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_1_FRAME0, 0),
            gen_atlas::SOUL_FIRE_1_FRAMES[0],
            "the soul *_1 strip the sequential head wears"
        );
        assert_ne!(
            gen_atlas::anim_tile(gen_atlas::SOUL_FIRE_0_FRAME0, 0),
            gen_atlas::anim_tile(gen_atlas::FIRE_0_FRAME0, 0),
            "the soul's frames the plain flame's rows never share"
        );
        // THE IDENTITY — every non-fire tile the remap never touches
        // (the block palette, the destroy stages, the missing-material
        // sentinel the flat path keeps)
        let stone = gen_atlas::FACE_TILES[STONE as usize][2];
        assert_eq!(gen_atlas::anim_tile(stone, 7), stone, "the stone tile the identity wears");
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::DESTROY_TILES[3], 19),
            gen_atlas::DESTROY_TILES[3],
            "the destroy overlay's crack frames the identity wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_NONE, 5),
            gen_atlas::TILE_NONE,
            "the flat path's sentinel the identity wears"
        );
        // THE APPENDED FRAMES' OWN BODY — every frame tile the non-empty
        // 16x16 texel body carries, the frame-00 rows the block tile's
        // own pixels speak (the strip's first frame the palette's crop
        // already banked — the two index spaces the same pixels wear)
        for f in gen_atlas::FIRE_0_FRAMES.iter().copied()
            .chain(gen_atlas::FIRE_1_FRAMES.iter().copied())
            .chain(gen_atlas::SOUL_FIRE_0_FRAMES.iter().copied())
            .chain(gen_atlas::SOUL_FIRE_1_FRAMES.iter().copied())
        {
            assert_eq!(
                gen_atlas::tile_rgba(f).len(),
                16 * 16 * 4,
                "the frame tile's own body: tile {f}"
            );
        }
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::FIRE_0_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::FIRE_0_FRAMES[0]),
            "the block tile's pixels the strip's first frame carries"
        );
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::SOUL_FIRE_0_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::SOUL_FIRE_0_FRAMES[0]),
            "the soul block tile's pixels the strip's first frame carries"
        );
    }

    #[test]
    fn the_lit_log_anim_the_interpolate_blends() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE INTERPOLATE LANE'S OWN WALK (the 26.2 jar's mcmeta: the
        // lit-log strips the {"interpolate": true, "frametime": 20}
        // 4-frame cycle carry): the blend bank the full 80-tick cycle
        // tiles (the frametime-1 cadence the per-tick resolution
        // keeps), the tick-0 head the BASE tile the alpha-0 identity
        // wears (the golden safety the byte-identical head rides), the
        // wrap the LAST frame's span the FIRST frame blends toward
        // (javap SpriteContents$AnimationState.drawToAtlas pc 88-115 —
        // the (frame + 1) % count modulo law)
        assert_eq!(
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMETIME,
            1,
            "the per-tick blend cadence"
        );
        assert_eq!(
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES.len(),
            80,
            "the 4-frame x 20-tick cycle"
        );
        assert_eq!(gen_atlas::SOUL_CAMPFIRE_LOG_LIT_FRAMES.len(), 80);
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 0),
            gen_atlas::TILE_CAMPFIRE_LOG_LIT,
            "tick 0 the base tile the alpha-0 identity wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 1),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[1],
            "tick 1 the first blend tile the walk answers"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 20),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[20],
            "tick 20 the second span's own head (the pure frame 1)"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 79),
            gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[79],
            "tick 79 the wrap span's tail"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 80),
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG_LIT, 0),
            "the 80-tick cycle the playlist's own period wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_SOUL_CAMPFIRE_LOG_LIT, 3),
            gen_atlas::SOUL_CAMPFIRE_LOG_LIT_FRAMES[3],
            "the soul strip the same walk answers"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::TILE_CAMPFIRE_LOG, 7),
            gen_atlas::TILE_CAMPFIRE_LOG,
            "the unlit log the identity wears"
        );
        // THE PIXEL LAW'S OWN PROBE (the thousandths quantization the
        // f2i truncation banks — javap pc 39-76; for the frametime 20
        // the quantization the NO-OP law keeps — the empirical walk
        // the 3/20 -> 150 truncation speaks, the f32 grid rounds UP):
        // the tick-3 blend tile the exact 150/1000 alpha's round mix
        // wears (the 149/1000 neighbor the strip refuses), the tick-10
        // tile the exact 500/1000 half, the tick-79 wrap the 950/1000
        // mix (the LAST frame the FIRST blends toward)
        let base = gen_atlas::tile_rgba(gen_atlas::TILE_CAMPFIRE_LOG_LIT);
        let f1 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[20]);
        let t3 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[3]);
        let t10 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[10]);
        assert!(
            base.iter().zip(f1.iter()).any(|(a, b)| a != b),
            "the frame 1 the base's own pixels depart somewhere"
        );
        let law = |alpha: f64| -> Vec<u8> {
            base.iter()
                .zip(f1.iter())
                .map(|(&a, &b)| (a as f64 + (b as f64 - a as f64) * alpha).round() as u8)
                .collect()
        };
        assert_eq!(t3, law(0.15), "tick 3 every texel the 150/1000 alpha's mix wears");
        assert_ne!(
            t3,
            law(0.149),
            "the 149/1000 neighbor the exact quantization refuses"
        );
        assert_eq!(t10, law(0.5), "tick 10 the exact half's mix wears");
        let f3 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[60]);
        let t79 = gen_atlas::tile_rgba(gen_atlas::CAMPFIRE_LOG_LIT_FRAMES[79]);
        let law_wrap = |alpha: f64| -> Vec<u8> {
            f3.iter()
                .zip(base.iter())
                .map(|(&a, &b)| (a as f64 + (b as f64 - a as f64) * alpha).round() as u8)
                .collect()
        };
        assert_eq!(
            t79,
            law_wrap(0.95),
            "tick 79 the wrap's 950/1000 mix wears"
        );
        assert_ne!(
            t79,
            law_wrap(0.949),
            "the wrap's own quantization the 949/1000 law refuses"
        );
    }

    #[test]
    fn the_fluid_anim_the_strips_walk() {
        use mc_registry::generated::atlas as gen_atlas;
        // THE FLUIDS' OWN MCMETA (the 26.2 jar): water_still frametime 2
        // the EMPTY frames array the sequential rule serves, lava_still
        // frametime 2 the EXPLICIT ping-pong [0..19, 18..1] speaks (the
        // 38 display steps over the strip's 20 frames) — the per-strip
        // frametime the single fire constant never served.
        assert_eq!(gen_atlas::WATER_FRAMETIME, 2, "the water mcmeta's own key");
        assert_eq!(gen_atlas::LAVA_FRAMETIME, 2, "the lava mcmeta's own key");
        // THE WATER'S OWN WALK — the frametime 2 dwell the even ticks
        // answer: tick 0 the first frame, tick 1 the SAME frame (the
        // dwell the divisor speaks), tick 2 the walk's next row; the
        // 64-tick cycle the 32 frames x the frametime wears
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 0),
            gen_atlas::WATER_FRAMES[0],
            "tick 0 the strip's own first frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 1),
            gen_atlas::WATER_FRAMES[0],
            "tick 1 the frametime dwell the same frame keeps"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 2),
            gen_atlas::WATER_FRAMES[1],
            "tick 2 the sequential walk's next row"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 64),
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 0),
            "the 64-tick cycle the 32-frame strip's period wears"
        );
        // THE LAVA'S OWN PING-PONG — the explicit playlist the ascent
        // and the descent both walk: tick 38 (pos 19) the strip's last
        // frame, tick 40 (pos 20) the descent's 18 row, the 76-tick
        // cycle the 38 steps x the frametime wears
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 38),
            gen_atlas::LAVA_FRAMES[19],
            "tick 38 the ascent's own tail frame"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 40),
            gen_atlas::LAVA_FRAMES[18],
            "tick 40 the ping-pong's descent the 18 row wears"
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 76),
            gen_atlas::anim_tile(gen_atlas::LAVA_FRAME0, 0),
            "the 76-tick cycle the 38-step playlist's period wears"
        );
        // THE TICK-0 PIXEL IDENTITY — the appended frame-00 tiles the
        // palette's own rows the same bytes carry (the goldens' own
        // proof: the scenes' water at tick 0 the still the palette's
        // crop painted, the renders byte-identical)
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::WATER_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::WATER_FRAMES[0]),
            "the water block tile's pixels the strip's first frame carries"
        );
        assert_eq!(
            gen_atlas::tile_rgba(gen_atlas::LAVA_FRAME0),
            gen_atlas::tile_rgba(gen_atlas::LAVA_FRAMES[0]),
            "the lava block tile's pixels the strip's first frame carries"
        );
        // THE FRAME BODIES — every appended frame tile the full 16x16
        // body carries (the water strip's 32, the lava strip's 20)
        for f in gen_atlas::WATER_FRAMES.iter().copied() {
            assert_eq!(gen_atlas::tile_rgba(f).len(), 16 * 16 * 4, "water frame {f}");
        }
        for f in gen_atlas::LAVA_FRAMES.iter().copied() {
            assert_eq!(gen_atlas::tile_rgba(f).len(), 16 * 16 * 4, "lava frame {f}");
        }
    }

    #[test]
    fn the_tuft_cross_wears_the_tint_laws() {
        // THE GRASS TUFT'S OWN CROSS (the autopush-412 round — the
        // tinted_cross parent the flowers' own block/cross bake
        // repeats): the tuft cell the same 4 quads wear (the two
        // diagonal planes × the two windings, the corners the 0.8
        // inset keeps, the uv the full sprite rides), the tile the
        // tuft's own atlas row banks, THE TINT the no-op rule keeps —
        // the offline world's no-biome lane rides the identity (the
        // baked plains pixels speak), the synced plains verdict the
        // baked constant equals (the fold never fires), the off-plains
        // synced biome the fold answers (the vertex colors multiply).
        // THE DEV-27 OFFSET (the XYZ row): the tuft the vertical sink
        // wears — the (v-1.0)*maxV band the dsub speaks, the span the
        // shifted bounds keep, the model-relative read the shift
        // subtracts.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::SHORT_GRASS);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::SHORT_GRASS, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!((dx, dy, dz), (0.150_000_005_960_464_48f32, -0.200_000_002_980_232_24f32, -0.083_333_328_366_279_6f32));
        // the shifted span the same corner walk computes (the fy+dy
        // innermost addition the mesher speaks — the f32 the f64 law
        // casts)
        let bottom = 9.0f32 + (0.0 + dy);
        let top = 9.0f32 + (1.0 + dy);
        let tuft: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (bottom..=top).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == top)
            })
            .collect();
        assert_eq!(tuft.len(), 4, "the two planes the two windings wear");
        for quad in &tuft {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
                assert!(
                    (fx - fz).abs() < 1e-6 || (fx + fz - 1.0).abs() < 1e-6,
                    "the cross plane the diagonal runs: {fx} vs {fz}"
                );
            }
            // the tile: the short_grass's own row (the atlas's 41st kind)
            assert_eq!(
                quad.tile,
                mc_registry::generated::atlas::FACE_TILES[41][2]
            );
        }
        // THE BUSH'S OWN ROW (the patch_bush kind the same cross bake
        // wears — the 42nd kind the tile row banks)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, STONE);
        put(&mut w2, 8, 9, 8, crate::world::BUSH);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        assert!(
            q2.iter().any(|quad| quad.tile
                == mc_registry::generated::atlas::FACE_TILES[42][2]),
            "the bush's own tile the cross quads bank"
        );
        // THE TINT FOLD'S OWN VERDICT (the audited slot walk): the tuft
        // kinds ride the grass source on every face — the synced
        // off-plains biome the fold answers, the no-biome world the
        // identity keeps (the folded quad's colors = the baked pixels'
        // own light walk, byte-identical to the no-fold verdict)
        let folded = crate::tint::unpack(0xBFB755);
        assert!(folded != [1.0; 3], "the desert verdict the fold carries");
        assert_eq!(
            crate::tint::slot_for(crate::world::SHORT_GRASS, 2),
            Some(crate::tint::GRASS_SLOT),
            "the tuft's grass source the slot walk answers"
        );
    }

    #[test]
    fn the_double_plant_cross_wears_the_half_tile_laws() {
        // THE DOUBLE PLANT'S OWN CROSS (the autopush-413 round — the
        // tinted_cross parent the tuft's own block/cross bake repeats,
        // PER HALF): the tall_grass cell the same 4 quads wear (the two
        // diagonal planes × the two windings, the corners the 0.8 inset
        // keeps), the LOWER kind the tall_grass_bottom tile banks, the
        // UPPER kind the tall_grass_top tile banks (the blockstate's
        // half-keyed variants the atlas's own rows speak — the two
        // sprites the pair wears), THE TINT the no-op rule keeps — the
        // offline world's no-biome lane rides the identity (the baked
        // plains pixels speak), the grass source the slot walk answers
        // on every face (the audited doubleTallGrass register row).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 9, 8, crate::world::TALL_GRASS);
        put(&mut w, 8, 10, 8, crate::world::TALL_GRASS_UPPER);
        let q = mesh_chunk(&w, 0, 0, None);
        let a = 0.8 / 16.0;
        let b = 15.2 / 16.0;
        // THE DEV-27 OFFSET: the XZ row the pair RIGIDLY rides — the
        // seed the Y-zero law hashes, the two cells the same triple
        // wear, the shift the model-relative reads subtract
        let (dx, dy, dz) =
            crate::world::plant_offset(crate::world::TALL_GRASS, 8, 9, 8);
        let (dx, dy, dz) = (dx as f32, dy as f32, dz as f32);
        assert_eq!(dy, 0.0, "the XZ row the zero Y keeps");
        assert_eq!(
            (dx, dz),
            {
                let (ux, _uy, uz) =
                    crate::world::plant_offset(crate::world::TALL_GRASS_UPPER, 8, 10, 8);
                (ux as f32, uz as f32)
            },
            "the pair the rigid offset rides"
        );
        let lower: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (9.0..=10.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 10.0)
            })
            .collect();
        let upper: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().all(|p| (10.0..=11.0).contains(&p[1]))
                    && quad.pts.iter().any(|p| p[1] == 11.0)
            })
            .collect();
        assert_eq!(lower.len(), 4, "the two planes the two windings wear");
        assert_eq!(upper.len(), 4, "the pair's upper the same bake wears");
        let lower_tile = mc_registry::generated::atlas::FACE_TILES
            [crate::world::TALL_GRASS as usize][2];
        let upper_tile = mc_registry::generated::atlas::FACE_TILES
            [crate::world::TALL_GRASS_UPPER as usize][2];
        assert_ne!(
            lower_tile, upper_tile,
            "the half-keyed sprites the two tiles bank"
        );
        for quad in &lower {
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the x the diagonal corner inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the z the diagonal corner inset keeps: {fz}"
                );
            }
            assert_eq!(quad.tile, lower_tile, "the lower half its own tile");
        }
        for quad in &upper {
            // THE UPPER'S OWN RIDE: the same triple the rigid pair
            // wears — the shifted corners the same lattice keeps
            for p in &quad.pts {
                let (fx, fz) = (p[0] - 8.0 - dx, p[2] - 8.0 - dz);
                assert!(
                    (fx - a).abs() < 1e-6 || (fx - b).abs() < 1e-6,
                    "the upper's x the same inset keeps: {fx}"
                );
                assert!(
                    (fz - a).abs() < 1e-6 || (fz - b).abs() < 1e-6,
                    "the upper's z the same inset keeps: {fz}"
                );
            }
            assert_eq!(quad.tile, upper_tile, "the upper half its own tile");
        }
        // THE GRASS SOURCE'S OWN SLOTS (the audited register walk): the
        // doubleTallGrass() source the TALL_GRASS/LARGE_FERN pair
        // covers — the GRASS_SLOT on EVERY face both halves speak
        for kind in [
            crate::world::TALL_GRASS,
            crate::world::TALL_GRASS_UPPER,
            crate::world::LARGE_FERN,
            crate::world::LARGE_FERN_UPPER,
        ] {
            for face in 0..6 {
                assert_eq!(
                    crate::tint::slot_for(kind, face),
                    Some(crate::tint::GRASS_SLOT),
                    "the double plant's grass source the slot walk answers ({kind}, {face})"
                );
            }
        }
        // THE BAKE'S OWN PIXELS (the generated tiles the plains look
        // wears): the four masters the TINT_GRASS bake banked — the
        // grayscale greens the tuft family's own tint speaks
        assert_eq!(
            mc_registry::generated::atlas::TINTS[crate::world::TALL_GRASS as usize][2],
            0x91bd59,
            "the plains constant the TINTS row keeps"
        );
    }

    #[test]
    fn water_quads_translucent_others_not() {
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, STONE);
        put(&mut w, 8, 5, 8, WATER);
        let q = mesh_chunk(&w, 0, 0, None);
        for quad in &q {
            // every face of the water block (sides/top/bottom) is translucent;
            // every face of the stone block (y 8..9) is opaque
            let in_water_span = quad.pts.iter().all(|p| (5.0..=6.0).contains(&p[1]));
            assert_eq!(quad.translucent, in_water_span);
        }
    }

    #[test]
    fn the_fluid_the_side_faces_the_flow_master_wears() {
        // THE VANILLA SIDE-FACE LAW (javap FluidRenderer pc 1458-1466,
        // the 26.2 jar): the horizontal sides the HORIZONTAL loop walks
        // the flowing sprite ALWAYS wear — the calm source's own
        // flow==(0,0) verdict the still pick the top face keeps
        // (pc 671-691) and the bottom face's own stillMaterial walk
        // (pc 1040-1048) keeps. The side quads the flow strip's frame-0
        // tile bake — the generator's quadrant bank the top-left 16x16
        // region the side UVs pc 1515-1567 sample (getU(0.0)..getU(0.5)
        // x getV((1-h)*0.5)..getV(0.5), the full-height cell the exact
        // quadrant answers), the tile the side quad's whole UV rect
        // maps. The draw-time remap the flow strips' own frametime the
        // anim_tile arms speak (the sibling walk test pins).
        use mc_registry::generated::atlas as gen_atlas;
        let mut w = empty_world();
        put(&mut w, 8, 5, 8, WATER);
        let q = mesh_chunk(&w, 0, 0, None);
        // the top face: the still master keeps (the calm pool's verdict —
        // the palette's own water_still row the FACE_TILES bank)
        let top = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 6.0))
            .expect("water top face");
        assert_eq!(top.tile, gen_atlas::FACE_TILES[WATER as usize][2]);
        assert_eq!(top.tile, gen_atlas::WATER_FRAME0, "the still strip's own base");
        // the bottom face: the still master keeps (the vanilla
        // stillMaterial walk the down face wears)
        let bottom = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 5.0))
            .expect("water bottom face");
        assert_eq!(bottom.tile, gen_atlas::FACE_TILES[WATER as usize][3]);
        // the four sides: the FLOW master's frame-0 tile — never the
        // still row the pre-529 world wore
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] == 5.0) && quad.pts.iter().any(|p| p[1] == 6.0)
            })
            .collect();
        assert_eq!(
            sides.len(),
            4,
            "the four horizontal faces the air neighbors draw"
        );
        for side in &sides {
            assert_eq!(side.tile, gen_atlas::WATER_FLOW_FRAME0);
            assert!(side.translucent, "the water side the translucent pass rides");
        }
        // the generated bank's own contract: the base the appended
        // frame-00 quadrant names (the strip the quadrant rows follow)
        assert_eq!(
            gen_atlas::TILE_NAMES[gen_atlas::WATER_FLOW_FRAME0 as usize],
            "water_flow_frame00"
        );
        // the flow master's pixels the still row never shares (the side
        // faces' look the flow strip's own streaks speak)
        assert_ne!(
            gen_atlas::TILES[gen_atlas::WATER_FLOW_FRAME0 as usize],
            gen_atlas::TILES[gen_atlas::FACE_TILES[WATER as usize][2] as usize],
        );
    }

    #[test]
    fn the_lava_the_sides_the_flow_the_still_top_keeps() {
        // the SECOND fluid member (the lava lane): the same side-face law
        // the water arm speaks — the sides the lava_flow master's frame-0
        // quadrant wear, the top/bottom the lava_still keeps, the quads
        // OPAQUE (the translucent pass the water alone rides)
        use mc_registry::generated::atlas as gen_atlas;
        let mut w = empty_world();
        put(&mut w, 8, 5, 8, LAVA);
        let q = mesh_chunk(&w, 0, 0, None);
        let sides: Vec<_> = q
            .iter()
            .filter(|quad| {
                quad.pts.iter().any(|p| p[1] == 5.0) && quad.pts.iter().any(|p| p[1] == 6.0)
            })
            .collect();
        assert_eq!(sides.len(), 4);
        for side in &sides {
            assert_eq!(side.tile, gen_atlas::LAVA_FLOW_FRAME0);
            assert!(!side.translucent, "the lava stays opaque");
        }
        assert_eq!(
            gen_atlas::TILE_NAMES[gen_atlas::LAVA_FLOW_FRAME0 as usize],
            "lava_flow_frame00"
        );
        let top = q
            .iter()
            .find(|quad| quad.pts.iter().all(|p| p[1] == 6.0))
            .expect("lava top face");
        assert_eq!(top.tile, gen_atlas::FACE_TILES[LAVA as usize][2]);
        assert_eq!(top.tile, gen_atlas::LAVA_FRAME0, "the still strip's own base");
    }

    #[test]
    fn the_flow_anim_the_water_walks_the_lava_lags() {
        // THE MCMETA'S OWN TRUTH the anim_tile arms serve: water_flow {}
        // the frametime-1 sequential 32 (the 32-tick cycle), lava_flow
        // {"frametime": 3} the sequential 16 (the 48-tick cycle), NO
        // interpolate key the static frame walk keeps. The tick-0 head
        // the frame-0 verdict (the fresh atlas's first frame the
        // playlist's own head row speaks).
        use mc_registry::generated::atlas as gen_atlas;
        assert_eq!(gen_atlas::WATER_FLOW_FRAMETIME, 1);
        assert_eq!(gen_atlas::WATER_FLOW_FRAMES.len(), 32);
        assert_eq!(gen_atlas::WATER_FLOW_PLAYLIST.len(), 32);
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 0),
            gen_atlas::WATER_FLOW_FRAMES[0]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 1),
            gen_atlas::WATER_FLOW_FRAMES[1]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 31),
            gen_atlas::WATER_FLOW_FRAMES[31]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FLOW_FRAME0, 32),
            gen_atlas::WATER_FLOW_FRAMES[0],
            "the 32-tick cycle wraps"
        );
        // every frame tile a DISTINCT index (the strip the full row bank
        // walks — the appended frames no dedup shares)
        let mut set = gen_atlas::WATER_FLOW_FRAMES.to_vec();
        set.sort_unstable();
        set.dedup();
        assert_eq!(set.len(), 32);
        assert_eq!(gen_atlas::LAVA_FLOW_FRAMETIME, 3);
        assert_eq!(gen_atlas::LAVA_FLOW_FRAMES.len(), 16);
        assert_eq!(gen_atlas::LAVA_FLOW_PLAYLIST.len(), 16);
        // the frametime-3 lag: tick 2 still the head frame, tick 3 the
        // second frame, the 48-tick cycle wraps
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 2),
            gen_atlas::LAVA_FLOW_FRAMES[0]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 3),
            gen_atlas::LAVA_FLOW_FRAMES[1]
        );
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::LAVA_FLOW_FRAME0, 48),
            gen_atlas::LAVA_FLOW_FRAMES[0]
        );
        // the still masters their own strips walk (the bases the
        // FACE_TILES rows bank — water_still the frametime-2 walk the
        // tick-10/pos-5 row speaks), the plain tiles the identity keeps
        assert_eq!(
            gen_atlas::anim_tile(gen_atlas::WATER_FRAME0, 10),
            gen_atlas::WATER_FRAMES[5]
        );
        assert_eq!(gen_atlas::anim_tile(0, 99), 0, "the stone tile the identity wears");
    }

    #[test]
    fn uv_order_follows_vertex_reorder() {
        // after flip_order rotation, uv[i] must equal face_uv of the SAME corner
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, GRASS);
        put(&mut w, 9, 9, 8, STONE); // force asymmetric AO -> flip
        put(&mut w, 8, 9, 9, STONE);
        let q = mesh_chunk(&w, 0, 0, None);
        let top = q.iter().find(|quad| quad.pts[0][1] == 9.0).expect("top face");
        let ao = ao_levels(&w, 8, 8, 8, 2);
        let order = flip_order(&ao);
        for (i, &src) in order.iter().enumerate() {
            assert_eq!(top.uvs[i], face_uv(2, FACE_CORNERS[2][src]), "uv must track corner {src}");
        }
    }

    #[test]
    fn the_tinted_glass_the_same_kind_cull_and_the_translucent_lane_wear() {
        // THE TINTED GLASS'S OWN MESH LAWS (the dev-81 round — javap
        // HalfTransparentBlock.skipRendering pc 0-17 the 26.2 jar the
        // TintedGlassBlock super inherits, the probe
        // scripts/tinted_glass_probe_81.py the NO override banks): the
        // tinted|tinted pair the shared face never draws, the
        // tinted|glass pair the DIFFERENT blocks the draw wears, the
        // translucent lane the per-texel alpha blend the partial-alpha
        // texture serves, the neighbors' faces the non-occluding cube
        // draws against.
        // SOLO: five outer faces + the bottom face the air below keeps
        // — six quads the one cube speaks.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, TINTED_GLASS);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 6, "the solo cube the six faces wears");
        for quad in &q {
            assert!(
                quad.translucent && quad.per_texel_alpha,
                "the translucent lane the per-texel blend every face wears"
            );
            assert_eq!(
                quad.tile,
                atlas::FACE_TILES[TINTED_GLASS as usize][2],
                "the ONE master every face banks"
            );
        }
        // THE SAME-KIND PAIR: two tinted cubes side by side — the
        // shared plane the SAME block never draws (the skipRendering
        // verdict true the skip banks), the 10 quads the pair speaks.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, TINTED_GLASS);
        put(&mut w, 9, 8, 8, TINTED_GLASS);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(
            q.len(),
            10,
            "the same-kind pair the shared faces never draws"
        );
        // THE MIXED PAIR: tinted + PLAIN glass side by side — the
        // shared plane the DIFFERENT blocks the draw wears (the
        // neighborState.is(this) verdict false the super's own default
        // answers), the 12 quads the pair speaks, the flags the two
        // lanes keep (the tinted per-texel, the plain cutout).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, TINTED_GLASS);
        put(&mut w, 9, 8, 8, GLASS);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(
            q.len(),
            12,
            "the mixed pair the shared faces the draw wears"
        );
        let tinted_x_face = q
            .iter()
            .find(|quad| {
                quad.tile == atlas::FACE_TILES[TINTED_GLASS as usize][0]
                    && quad.pts.iter().all(|p| (p[0] - 9.0).abs() < 1e-5)
            })
            .expect("the tinted +X face at the shared plane");
        assert!(
            tinted_x_face.translucent && tinted_x_face.per_texel_alpha,
            "the tinted lane the shared face wears"
        );
        // THE NEIGHBOR'S OWN FACE: the stone face against the tinted
        // cube draws (the noOcclusion verdict the transparent family
        // keeps).
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, TINTED_GLASS);
        put(&mut w, 8, 9, 8, STONE);
        let q = mesh_chunk(&w, 0, 0, None);
        let stone_bottom = q
            .iter()
            .find(|quad| {
                quad.tile == atlas::FACE_TILES[STONE as usize][3]
                    && quad.pts.iter().all(|p| (p[1] - 9.0).abs() < 1e-5)
            })
            .expect("the stone's bottom face against the tinted cell");
        assert!(
            !stone_bottom.translucent,
            "the stone's own opaque lane the bottom face keeps"
        );
    }

    #[test]
    fn the_floor_head_the_mini_cube_the_segment_laws_wear() {
        // THE MOB HEADS' OWN MESH LAWS (the dev-82 round — javap
        // SkullModel.createHeadModel + createGroundTransformation +
        // ModelPart$Cube/<Polygon> the 26.2 jar the probe
        // scripts/skull_probe_82.py banks): the 8x8x8 box the cell
        // center wears (x/z 4..12 px, y 0..8), the six model faces
        // the ENTITY_HEAD_FACES tiles bank, the ROTATION segment's
        // 22.5-degree quarter the face assignment rotates.
        use mc_registry::generated::atlas;
        // THE SOLO SKULL (the fresh-kind default the ROTATION=0 pin
        // keeps): six quads, the box the [8.25, 8.75] x [8, 8.5]
        // spans wear, the tiles the row's six slots speak.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, SKELETON_SKULL);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 6, "the solo head the six model faces wears");
        let row = atlas::ENTITY_HEAD_FACES[SKELETON_SKULL as usize];
        for quad in &q {
            assert!(
                !quad.translucent && !quad.per_texel_alpha,
                "the entity cutout lane the head quads keep"
            );
            for p in &quad.pts {
                assert!(
                    (8.25..=8.75).contains(&p[0]) && (8.0..=8.5).contains(&p[1])
                        && (8.25..=8.75).contains(&p[2]),
                    "the 8-px box the cell-center spans keep, got {p:?}"
                );
            }
            assert!(
                row.contains(&quad.tile),
                "the tile the row's own slots bank"
            );
        }
        // THE FACE'S OWN SEAT (the ROTATION=0 law: the NORTH face at
        // z = 8.25 the FRONT tile wears — the skin face the -Z plane
        // banks)
        let north_face = q
            .iter()
            .find(|quad| {
                quad.tile == row[3]
                    && quad.pts.iter().all(|p| (p[2] - 8.25).abs() < 1e-5)
            })
            .expect("the rotation-0 north face");
        assert!(
            north_face.pts.iter().all(|p| (8.25..=8.75).contains(&p[0])),
            "the face the 4..12 px x band keeps"
        );
        // THE SEGMENT'S OWN QUARTER (the ROTATION=4 stamp — the
        // -90-degree quarter the EAST plane the face rotates to: the
        // face tile the x = 8.75 plane wears)
        w.set_variant(8, 8, 8, 4);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 6, "the rotated head the same six faces wears");
        let east_face = q
            .iter()
            .find(|quad| {
                quad.tile == row[3]
                    && quad.pts.iter().all(|p| (p[0] - 8.75).abs() < 1e-5)
            })
            .expect("the rotation-4 east face");
        assert!(
            east_face.pts.iter().all(|p| (8.25..=8.75).contains(&p[2])),
            "the rotated face the z band keeps"
        );
        // THE EMISSIVE CELL'S OWN LIGHT (the uniform entity law —
        // every quad's four corners the SAME color the
        // entityCutoutNoCull lane speaks, NO face shade the BE pass
        // never samples)
        for quad in &q {
            assert_eq!(quad.colors[0], quad.colors[1]);
            assert_eq!(quad.colors[1], quad.colors[2]);
            assert_eq!(quad.colors[2], quad.colors[3]);
        }
    }

    #[test]
    fn the_rails_own_flat_lane_the_two_quads_wear() {
        // THE RAILS' OWN MESH LAWS (the dev-86 round — the rail_flat
        // model the probe scripts/rail_probe_86.py banks): the
        // zero-thickness plane the y=1/16 seat wears, the full
        // footprint the two faces (up + down) keep, the tile the
        // kind's own row banks, the E-W quarter the transposed uv
        // fold wears, the down face the vanilla's own v-flip keeps.
        use mc_registry::generated::atlas;
        // THE N-S DEFAULT (the variant byte 0 the registerDefaultState
        // pin speaks)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        let tile = atlas::FACE_TILES[crate::world::RAIL as usize][2];
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the flat plane the up+down pair wears");
        for quad in &q {
            assert!(
                !quad.translucent && !quad.per_texel_alpha,
                "the cutout lane the rail quads keep"
            );
            assert_eq!(quad.tile, tile, "the tile the kind's own row banks");
            for p in &quad.pts {
                assert!(
                    (8.0..=9.0).contains(&p[0]) && (8.0..=9.0).contains(&p[2]),
                    "the full footprint the plane spans, got {p:?}"
                );
                assert!(
                    (p[1] - 8.0625).abs() < 1e-5,
                    "the y=1/16 seat the flat plane wears, got {p:?}"
                );
            }
        }
        // THE N-S UV FOLD (the u the x-axis, the v the z-axis tracks —
        // the up face the straight map, the down the mirrored v the
        // model's [0,16,16,0] window wears): exactly one quad each
        let ns_up = q.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[0] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[2] - 8.0)).abs() < 1e-5
                })
        });
        let ns_down = q.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[0] - 8.0)).abs() < 1e-5
                        && (uv[1] - (9.0 - p[2])).abs() < 1e-5
                })
        });
        assert_eq!(ns_up.count(), 1, "the up face the straight N-S map wears");
        assert_eq!(ns_down.count(), 1, "the down face the v-flip wears");
        // THE E-W QUARTER (the variant byte 1 the placement's X-axis
        // arm stamps): the same plane the same seat the transposed
        // uv fold wears (the u the z-axis, the v the x-axis tracks)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        w.set_variant(8, 8, 8, 1);
        let q_ew = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_ew.len(), 2, "the quarter the same two quads wears");
        for quad in &q_ew {
            assert_eq!(quad.tile, tile, "the tile the same row banks");
        }
        let ew_up = q_ew.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[0] - 8.0)).abs() < 1e-5
                })
        });
        let ew_down = q_ew.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                        && (uv[1] - (9.0 - p[0])).abs() < 1e-5
                })
        });
        assert_eq!(ew_up.count(), 1, "the up face the transposed map wears");
        assert_eq!(ew_down.count(), 1, "the down face the flipped transpose wears");
    }

    #[test]
    fn the_powered_rails_own_on_lane_the_tile_swap_wears() {
        // THE POWERED RAIL'S OWN MESH LAW (the dev-96 round — the
        // vanilla blockstates/powered_rail.json's powered=true rows
        // the model swap the bit reads): the powered bit the
        // powered_rail_on seat the swap banks, the off master the
        // kind's own row keeps, the powered quarter the transposed
        // uv fold the same lane serves, the shape nibble the mask
        // the bit never touches.
        use mc_registry::generated::atlas;
        // THE OFF MASTER (the variant byte 0 the powered=false row
        // speaks): the kind's own tile the lane banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::POWERED_RAIL);
        let off_tile = atlas::FACE_TILES[crate::world::POWERED_RAIL as usize][2];
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the flat plane the up+down pair wears");
        for quad in &q {
            assert_eq!(quad.tile, off_tile, "the off rail the kind's own row keeps");
        }
        // THE ON SWAP (the POWERED bit the powered=true row speaks):
        // the _on seat the same two quads wear
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q_on = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_on.len(), 2, "the swap the same two quads wears");
        for quad in &q_on {
            assert_eq!(
                quad.tile,
                atlas::TILE_POWERED_RAIL_ON,
                "the powered bit the _on seat the swap banks"
            );
        }
        // THE POWERED QUARTER (the bit + the E-W byte the powered
        // east_west row speaks): the _on seat the transposed uv fold
        // the same lane wears (the u the z-axis tracks — the shape
        // nibble the mask reads)
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT | 1);
        let q_ew = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_ew.len(), 2, "the powered quarter the same two quads wears");
        for quad in &q_ew {
            assert_eq!(
                quad.tile,
                atlas::TILE_POWERED_RAIL_ON,
                "the powered quarter the _on seat keeps"
            );
        }
        let ew_up = q_ew.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[0] - 8.0)).abs() < 1e-5
                })
        });
        assert_eq!(ew_up.count(), 1, "the powered quarter the transposed map wears");
        // THE PLAIN RAIL THE BIT NEVER WEARS: the RAIL kind the same
        // bit the off row keeps (the swap the powered kind alone
        // takes)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::RAIL);
        w2.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        for quad in &q2 {
            assert_ne!(
                quad.tile,
                atlas::TILE_POWERED_RAIL_ON,
                "the plain rail the bit the swap never wears"
            );
        }
    }

    #[test]
    fn the_activator_rails_own_on_lane_the_tile_swap_wears() {
        // THE ACTIVATOR RAIL'S OWN MESH LAW (the dev-97 round — the
        // vanilla blockstates/activator_rail.json's powered=true rows
        // the same bit the seat reads): the powered bit the
        // activator_rail_on seat the swap banks, the off master the
        // kind's own row keeps, the powered kind the dev-96 seat
        // keeps, the detector kind the dev-98 seat its own lane
        // banks, the powered quarter the transposed uv fold the same
        // lane serves, the shape nibble the mask the bit never
        // touches.
        use mc_registry::generated::atlas;
        assert_ne!(
            atlas::TILE_ACTIVATOR_RAIL_ON,
            atlas::TILE_POWERED_RAIL_ON,
            "the twin seats the pool's own distinct rows bank"
        );
        // THE OFF MASTER (the variant byte 0 the powered=false row
        // speaks): the kind's own tile the lane banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::ACTIVATOR_RAIL);
        let off_tile = atlas::FACE_TILES[crate::world::ACTIVATOR_RAIL as usize][2];
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the flat plane the up+down pair wears");
        for quad in &q {
            assert_eq!(quad.tile, off_tile, "the off activator the kind's own row keeps");
        }
        // THE ON SWAP (the POWERED bit the powered=true row speaks):
        // the activator's own _on seat the same two quads wear
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q_on = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_on.len(), 2, "the swap the same two quads wears");
        for quad in &q_on {
            assert_eq!(
                quad.tile,
                atlas::TILE_ACTIVATOR_RAIL_ON,
                "the activator's bit its own _on seat banks"
            );
        }
        // THE POWERED QUARTER (the bit + the E-W byte the powered
        // east_west row speaks): the activator's own _on seat the
        // transposed uv fold the same lane wears
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT | 1);
        let q_ew = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_ew.len(), 2, "the activator quarter the same two quads wears");
        for quad in &q_ew {
            assert_eq!(
                quad.tile,
                atlas::TILE_ACTIVATOR_RAIL_ON,
                "the activator quarter its own _on seat keeps"
            );
        }
        let ew_up = q_ew.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[0] - 8.0)).abs() < 1e-5
                })
        });
        assert_eq!(ew_up.count(), 1, "the activator quarter the transposed map wears");
        // THE TWIN KINDS THE OWN SEATS KEEP: the powered kind the
        // same bit the dev-96 seat banks (the activator's seat the
        // powered kind never wears)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::POWERED_RAIL);
        w3.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        for quad in &q3 {
            assert_eq!(
                quad.tile,
                atlas::TILE_POWERED_RAIL_ON,
                "the powered kind its own dev-96 seat keeps"
            );
        }
        // THE DETECTOR KIND ITS OWN SEAT WEARS (the dev-98 round —
        // the vanilla blockstates/detector_rail.json's powered=true
        // rows the same bit the detector's own _on seat the swap
        // banks; the WORLD law the cart detection its own lane
        // serves, the mesher the banked byte alone renders)
        let mut w4 = empty_world();
        put(&mut w4, 8, 8, 8, crate::world::DETECTOR_RAIL);
        w4.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q4 = mesh_chunk(&w4, 0, 0, None);
        for quad in &q4 {
            assert_eq!(
                quad.tile,
                atlas::TILE_DETECTOR_RAIL_ON,
                "the detector's bit its own dev-98 seat banks"
            );
        }
    }

    #[test]
    fn the_detector_rails_own_on_lane_the_tile_swap_wears() {
        // THE DETECTOR RAIL'S OWN MESH LAW (the dev-98 round — the
        // vanilla blockstates/detector_rail.json's powered=true rows
        // the same bit the lane reads): the powered bit the
        // detector_rail_on seat the swap banks, the off master the
        // kind's own row keeps, the twin kinds the dev-96/97 seats
        // keep, the powered quarter the transposed uv fold the same
        // lane serves, the plain rail the bit the swap never wears.
        use mc_registry::generated::atlas;
        assert_ne!(
            atlas::TILE_DETECTOR_RAIL_ON,
            atlas::TILE_POWERED_RAIL_ON,
            "the twin seats the pool's own distinct rows bank"
        );
        assert_ne!(
            atlas::TILE_DETECTOR_RAIL_ON,
            atlas::TILE_ACTIVATOR_RAIL_ON,
            "the detector's seat the activator's row never shares"
        );
        // THE OFF MASTER (the variant byte 0 the powered=false row
        // speaks): the kind's own tile the lane banks
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::DETECTOR_RAIL);
        let off_tile = atlas::FACE_TILES[crate::world::DETECTOR_RAIL as usize][2];
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the flat plane the up+down pair wears");
        for quad in &q {
            assert_eq!(quad.tile, off_tile, "the off detector the kind's own row keeps");
        }
        // THE ON SWAP (the POWERED bit the powered=true row speaks):
        // the detector's own _on seat the same two quads wear
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q_on = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_on.len(), 2, "the swap the same two quads wears");
        for quad in &q_on {
            assert_eq!(
                quad.tile,
                atlas::TILE_DETECTOR_RAIL_ON,
                "the detector's bit its own _on seat banks"
            );
        }
        // THE POWERED QUARTER (the bit + the E-W byte the powered
        // east_west row speaks): the detector's own _on seat the
        // transposed uv fold the same lane wears
        w.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT | 1);
        let q_ew = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_ew.len(), 2, "the detector quarter the same two quads wears");
        for quad in &q_ew {
            assert_eq!(
                quad.tile,
                atlas::TILE_DETECTOR_RAIL_ON,
                "the detector quarter its own _on seat keeps"
            );
        }
        let ew_up = q_ew.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[0] - 8.0)).abs() < 1e-5
                })
        });
        assert_eq!(ew_up.count(), 1, "the detector quarter the transposed map wears");
        // THE TWIN KINDS THE OWN SEATS KEEP: the powered kind the
        // dev-96 seat, the activator kind the dev-97 seat (the
        // detector's seat the twins never wear)
        let mut w3 = empty_world();
        put(&mut w3, 8, 8, 8, crate::world::POWERED_RAIL);
        w3.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q3 = mesh_chunk(&w3, 0, 0, None);
        for quad in &q3 {
            assert_eq!(
                quad.tile,
                atlas::TILE_POWERED_RAIL_ON,
                "the powered kind its own dev-96 seat keeps"
            );
        }
        let mut w5 = empty_world();
        put(&mut w5, 8, 8, 8, crate::world::ACTIVATOR_RAIL);
        w5.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q5 = mesh_chunk(&w5, 0, 0, None);
        for quad in &q5 {
            assert_eq!(
                quad.tile,
                atlas::TILE_ACTIVATOR_RAIL_ON,
                "the activator kind its own dev-97 seat keeps"
            );
        }
        // THE PLAIN RAIL THE BIT NEVER WEARS: the RAIL kind the
        // same bit the off row keeps (the swap the powered kinds
        // alone take)
        let mut w2 = empty_world();
        put(&mut w2, 8, 8, 8, crate::world::RAIL);
        w2.set_variant(8, 8, 8, crate::world::RAIL_POWERED_BIT);
        let q2 = mesh_chunk(&w2, 0, 0, None);
        for quad in &q2 {
            assert_ne!(
                quad.tile,
                atlas::TILE_DETECTOR_RAIL_ON,
                "the plain rail the bit the swap never wears"
            );
        }
    }

    #[test]
    fn the_rails_own_corner_lane_the_rotation_folds_wear() {
        // THE CORNER SEAT (the dev-92 round — the vanilla
        // blockstates/rail.json's corner rows the model rotation the
        // byte resolves): the curve bytes the rail_corner tile wear
        // (the pool tail seat), the SE byte the identity fold keeps,
        // the NE byte the y=270 arm samples (u=16-z, v=x), the same
        // y=1/16 plane the same two quads wear
        use mc_registry::generated::atlas;
        let corner_tile = atlas::TILE_RAIL_CORNER;
        assert_ne!(
            corner_tile,
            atlas::TILE_NONE,
            "the corner seat the pool tail banks"
        );
        // THE SE QUARTER (the byte 6): the identity uv (u=x, v=z)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        w.set_variant(8, 8, 8, 6);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the corner the same two quads wears");
        for quad in &q {
            assert_eq!(quad.tile, corner_tile, "the corner tile the byte 6 wears");
        }
        let se_up = q.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (p[0] - 8.0)).abs() < 1e-5
                        && (uv[1] - (p[2] - 8.0)).abs() < 1e-5
                })
        });
        assert_eq!(se_up.count(), 1, "the SE byte the identity fold wears");
        // THE NE QUARTER (the byte 9): the y=270 arm (u=16-z, v=x)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        w.set_variant(8, 8, 8, 9);
        let q_ne = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q_ne.len(), 2, "the quarter the same two quads wears");
        for quad in &q_ne {
            assert_eq!(quad.tile, corner_tile, "the corner tile the byte 9 wears");
        }
        let ne_up = q_ne.iter().filter(|quad| {
            quad.pts
                .iter()
                .zip(quad.uvs.iter())
                .all(|(p, uv)| {
                    (uv[0] - (9.0 - p[2])).abs() < 1e-5
                        && (uv[1] - (p[0] - 8.0)).abs() < 1e-5
                })
        });
        assert_eq!(ne_up.count(), 1, "the NE byte the y=270 fold wears");
    }

    #[test]
    fn the_rails_own_ascent_lane_the_sloped_plane_wears() {
        // THE ASCENT'S OWN MESH LAWS (the dev-93 round — the
        // raised_ne/sw templates the 45-degree rotation the rescale
        // keeps): the sloped plane the flat seat at the low edge
        // wears, the y=17/16 seam the high edge reaches [the upper
        // cell's own rail seat the one plane connects], the tile the
        // kind's own row banks (the raised models the plain rail
        // texture reuse), the E-W bytes the y=90 uv fold wears, the
        // N-S bytes the identity fold keeps
        use mc_registry::generated::atlas;
        let tile = atlas::FACE_TILES[crate::world::RAIL as usize][2];
        // THE EAST ASCENT (the byte 2): the plane rises toward +x
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        w.set_variant(8, 8, 8, 2);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the slope the up+down pair wears");
        for quad in &q {
            assert_eq!(
                quad.tile, tile,
                "the raised models the plain rail texture reuse"
            );
        }
        // the up quad: the y the x-correlated slope wears (the west
        // edge the 1/16 seat, the east edge the 17/16 seam)
        let up = q
            .iter()
            .find(|quad| {
                quad.pts
                    .iter()
                    .zip(quad.uvs.iter())
                    .all(|(p, uv)| {
                        (uv[0] - (p[2] - 8.0)).abs() < 1e-5
                            && (uv[1] - (9.0 - p[0])).abs() < 1e-5
                    })
            })
            .expect("the up face the y=90 fold wears");
        for p in &up.pts {
            assert!(
                (p[1] - (8.0 + 1.0 / 16.0 + (p[0] - 8.0))).abs() < 1e-5,
                "the slope the x-correlated y wears, got {p:?}"
            );
        }
        let y_hi = up.pts.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
        let y_lo = up.pts.iter().map(|p| p[1]).fold(f32::MAX, f32::min);
        assert!(
            (y_hi - (8.0 + 17.0 / 16.0)).abs() < 1e-4,
            "the high edge the 17/16 seam reaches, got {y_hi}"
        );
        assert!(
            (y_lo - (8.0 + 1.0 / 16.0)).abs() < 1e-4,
            "the low edge the flat seat keeps, got {y_lo}"
        );
        // THE NORTH ASCENT (the byte 4): the identity uv fold, the
        // slope the z-correlated y wears (the north edge the 17/16
        // seam, the south edge the flat seat)
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, crate::world::RAIL);
        w.set_variant(8, 8, 8, 4);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 2, "the slope the same pair wears");
        let up4 = q
            .iter()
            .find(|quad| {
                quad.pts
                    .iter()
                    .zip(quad.uvs.iter())
                    .all(|(p, uv)| {
                        (uv[0] - (p[0] - 8.0)).abs() < 1e-5
                            && (uv[1] - (p[2] - 8.0)).abs() < 1e-5
                    })
            })
            .expect("the up face the identity fold wears");
        for p in &up4.pts {
            assert!(
                (p[1] - (8.0 + 17.0 / 16.0 - (p[2] - 8.0))).abs() < 1e-5,
                "the slope the z-correlated y wears, got {p:?}"
            );
        }
    }

    #[test]
    fn the_wall_head_the_wall_transform_the_quads_wear() {
        // THE WALL HEADS' OWN MESH LAWS (the dev-83 round — javap
        // SkullBlockRenderer.createWallTransformation the 26.2 jar the
        // probe scripts/skull_probe_83.py banks): the same 8x8x8 box
        // the wall compose wears — T(0.5 - stepX/4, 0.25, 0.5 -
        // stepZ/4) · R_y(-toYRot(opposite(facing))) · scale(-1,-1,1)
        // — the y 4..12 px band the hugging side the facing speaks,
        // the north the z 8..16 seat keeps, the east the x 0..8 arm,
        // the six model faces the FLOOR TWINS' own tiles bank.
        use mc_registry::generated::atlas;
        // THE NORTH FACING (the variant byte 1 the torch vocabulary
        // banks): the box the x/z [8.25, 8.75] x [8.5, 9.0] spans
        // wear [x 4..12 px, z 8..16 px], the y [8.25, 8.75] the wall
        // band keeps, the six quads the floor twin's tiles bank.
        let mut w = empty_world();
        put(&mut w, 8, 8, 8, SKELETON_WALL_SKULL);
        w.set_variant(8, 8, 8, crate::world::WALL_TORCH_FACING_NORTH);
        let row = atlas::ENTITY_HEAD_FACES[SKELETON_WALL_SKULL as usize];
        let floor_row = atlas::ENTITY_HEAD_FACES[SKELETON_SKULL as usize];
        assert_eq!(row, floor_row, "the wall twin the floor tiles re-wears");
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 6, "the wall head the six model faces wears");
        for quad in &q {
            assert!(
                !quad.translucent && !quad.per_texel_alpha,
                "the entity cutout lane the wall quads keep"
            );
            for p in &quad.pts {
                assert!(
                    (8.25..=8.75).contains(&p[0]) && (8.25..=8.75).contains(&p[1])
                        && (8.5..=9.0).contains(&p[2]),
                    "the north-facing box the hugging spans keep, got {p:?}"
                );
            }
            assert!(row.contains(&quad.tile), "the tile the row's own slots bank");
        }
        // THE FACE'S OWN SEAT (the north facing the rotation-0 law
        // keeps: the FRONT tile the z = 8.5 plane the -Z side banks)
        let north_face = q
            .iter()
            .find(|quad| quad.tile == row[3] && quad.pts.iter().all(|p| (p[2] - 8.5).abs() < 1e-5))
            .expect("the north-facing front plane");
        assert!(
            north_face.pts.iter().all(|p| (8.25..=8.75).contains(&p[0])),
            "the face the 4..12 px x band keeps"
        );
        // THE EAST FACING'S OWN QUARTER (the variant byte 2 — the
        // -90-degree quarter the +X plane the face rotates to: the
        // box the x [8.0, 8.5] arm the hugging side wears, the FRONT
        // tile the x = 8.5 plane banks)
        w.set_variant(8, 8, 8, crate::world::WALL_TORCH_FACING_EAST);
        let q = mesh_chunk(&w, 0, 0, None);
        assert_eq!(q.len(), 6, "the east head the same six faces wears");
        for quad in &q {
            for p in &quad.pts {
                assert!(
                    (8.0..=8.5).contains(&p[0]) && (8.25..=8.75).contains(&p[1])
                        && (8.25..=8.75).contains(&p[2]),
                    "the east-facing box the west-side spans keep, got {p:?}"
                );
            }
        }
        let east_face = q
            .iter()
            .find(|quad| quad.tile == row[3] && quad.pts.iter().all(|p| (p[0] - 8.5).abs() < 1e-5))
            .expect("the east-facing front plane");
        assert!(
            east_face.pts.iter().all(|p| (8.25..=8.75).contains(&p[2])),
            "the rotated face the z band keeps"
        );
        // THE EMISSIVE CELL'S OWN LIGHT (the uniform entity law — the
        // wall lane the same cell light every face wears)
        for quad in &q {
            assert_eq!(quad.colors[0], quad.colors[1]);
            assert_eq!(quad.colors[1], quad.colors[2]);
            assert_eq!(quad.colors[2], quad.colors[3]);
        }
    }
}

