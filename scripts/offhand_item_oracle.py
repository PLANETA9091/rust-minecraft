#!/usr/bin/env python3
"""The offhand lane's oracle walk — the exact composition the 26.2
bytecode speaks, mirrored op for op, and the stone's box it lands.

THE CHAIN (ItemInHandLayer.submitArmWithItem, both arms):
  v' = fold( M_arm * L * P * v )
  M_arm — the translateToHand walk (root identity + the arm part's own
          translateAndRotate: T(pivot/16) * Rz * Ry * Rx)
  L     — the layer's law: rotX(-90) * rotY(180) *
          T(±1/16, 2/16, -10/16)  (the X sign the arm==LEFT branch
          keeps — the loc 60-151 walk; Y/Z untouched)
  P     — the display stanza pose (build_pose): T(±tx, ty, tz) *
          rotationXYZ(rx, ±ry, ±rz) * S * T(-0.5)  (the left hand's
          tx/ry/rz negation the ItemDisplayContext.leftHand flag
          speaks; the -0.5 centering BOTH hands apply — the
          ItemRenderer's own unconditional translate)
  fold  — the entity fold (setupRotations): S(-1,-1,1) over the
          -1.501 line, then rotY(180 - yaw). At yaw 180 alive:
          [-x, -y + 1.501, z].

HONESTY GATE: the RIGHT-hand chain must reproduce the pinned main-hand
stone box (the_held_item_rides_the_third_person_chain's asserts)
before the LEFT-hand numbers mean anything.

The stone rides the block/block stanza (the parent chain's own):
thirdperson {rotation: [75, 45, 0], translation: [0, 2.5, 0],
scale: [0.375 x3]} — both hands share the JSON (symmetric), the
left-hand APPLICATION does the mirroring.
"""
import math

# JOML f32 rounding — the floats walk f32-wide (the registry's own law)
def f32(x):
    return struct_f32(x)

import struct
def struct_f32(x):
    return struct.unpack('f', struct.pack('f', x))[0]

# column-major 4x4: m[c][r]
def mat_identity():
    return [[1.0 if c == r else 0.0 for r in range(4)] for c in range(4)]

def mat_mul(a, b):
    out = [[0.0] * 4 for _ in range(4)]
    for c in range(4):
        for r in range(4):
            s = 0.0
            for k in range(4):
                s += a[k][r] * b[c][k]
            out[c][r] = f32(s)
    return out

def mat_translate(x, y, z):
    m = mat_identity()
    m[3][0], m[3][1], m[3][2] = f32(x), f32(y), f32(z)
    return m

def mat_scale(x, y, z):
    m = mat_identity()
    m[0][0], m[1][1], m[2][2] = f32(x), f32(y), f32(z)
    return m

def rot_x(a):
    # the math.rs Mat4 law (col-major m[c][r]): out_y = c*y - s*z,
    # out_z = s*y + c*z
    m = mat_identity()
    c, s = f32(math.cos(a)), f32(math.sin(a))
    m[1][1], m[1][2], m[2][1], m[2][2] = c, s, f32(-s), c
    return m

def rot_y(a):
    m = mat_identity()
    c, s = f32(math.cos(a)), f32(math.sin(a))
    m[0][0], m[0][2], m[2][0], m[2][2] = c, f32(-s), s, c
    return m

def rot_z(a):
    m = mat_identity()
    c, s = f32(math.cos(a)), f32(math.sin(a))
    m[0][0], m[0][1], m[1][0], m[1][1] = c, s, f32(-s), c
    return m

def quat_mul(a, b):
    aw, ax, ay, az = a
    bw, bx, by, bz = b
    return [
        f32(aw * bw - ax * bx - ay * by - az * bz),
        f32(aw * bx + ax * bw + ay * bz - az * by),
        f32(aw * by - ax * bz + ay * bw + az * bx),
        f32(aw * bz + ax * by - ay * bx + az * bw),
    ]

def rotation_xyz(rx, ry, rz):
    """JOML Quaternionf.rotationXYZ: q = (qx*qy)*qz — the nine-term
    matrix the registry's rotation_matrix speaks, col-major."""
    sx, cx = f32(math.sin(rx * 0.5)), f32(math.cos(rx * 0.5))
    sy, cy = f32(math.sin(ry * 0.5)), f32(math.cos(ry * 0.5))
    sz, cz = f32(math.sin(rz * 0.5)), f32(math.cos(rz * 0.5))
    qx = [cx, sx, 0.0, 0.0]
    qy = [cy, 0.0, sy, 0.0]
    qz = [cz, 0.0, 0.0, sz]
    q = quat_mul(quat_mul(qx, qy), qz)
    w, x, y, z = q
    ww, xx, yy, zz = f32(w * w), f32(x * x), f32(y * y), f32(z * z)
    zw = f32(z * w) * 2
    xy = f32(x * y) * 2
    xz = f32(x * z) * 2
    yw = f32(y * w) * 2
    yz = f32(y * z) * 2
    xw = f32(x * w) * 2
    m = mat_identity()
    m[0][0] = f32((ww + xx - zz) - yy)
    m[0][1] = f32(xy + zw)
    m[0][2] = f32(xz - yw)
    m[1][0] = f32(-zw + xy)
    m[1][1] = f32(((yy - zz) + ww) - xx)
    m[1][2] = f32(yz + xw)
    m[2][0] = f32(yw + xz)
    m[2][1] = f32(yz - xw)
    m[2][2] = f32(((zz - yy) - xx) + ww)
    return m

def apply(m, v):
    """The plain nested fma the apply_vertex walk speaks."""
    x, y, z = v
    return [
        f32(m[0][0] * x + (m[1][0] * y + (m[2][0] * z + m[3][0]))),
        f32(m[0][1] * x + (m[1][1] * y + (m[2][1] * z + m[3][1]))),
        f32(m[0][2] * x + (m[1][2] * y + (m[2][2] * z + m[3][2]))),
    ]

DEG = math.pi / 180.0

def arm_matrix(pivot, z_rot, y_rot, x_rot):
    """translateAndRotate: T(pivot/16) * Rz * Ry * Rx."""
    m = mat_translate(pivot[0] / 16.0, pivot[1] / 16.0, pivot[2] / 16.0)
    m = mat_mul(m, rot_z(z_rot))
    m = mat_mul(m, rot_y(y_rot))
    m = mat_mul(m, rot_x(x_rot))
    return m

def layer_matrix(x_off):
    return mat_mul(rot_x(-math.pi / 2),
                   mat_mul(rot_y(math.pi),
                           mat_translate(x_off, 2.0 / 16.0, -10.0 / 16.0)))

def stanza_pose(rotation, translation, scale, left):
    """build_pose: T(±tx, ty, tz) * rotationXYZ(rx, ±ry, ±rz) * S * T(-0.5)."""
    tx = -translation[0] if left else translation[0]
    ry = -rotation[1] if left else rotation[1]
    rz = -rotation[2] if left else rotation[2]
    m = mat_translate(tx, translation[1], translation[2])
    m = mat_mul(m, rotation_xyz(rotation[0] * DEG, ry * DEG, rz * DEG))
    m = mat_mul(m, mat_scale(scale[0], scale[1], scale[2]))
    # the -0.5 centering folded (the nested fma's own fold)
    out = [[m[c][r] for r in range(4)] for c in range(4)]
    for r in range(3):
        out[3][r] = f32(m[2][r] * -0.5 + (m[1][r] * -0.5 + (m[0][r] * -0.5 + m[3][r])))
    return out

def fold_point(p, yaw_deg=180.0, death_time=0.0, crouch_drop=0.0, feet=(0.0, 0.0, 0.0)):
    a = (180.0 - yaw_deg) * DEG
    sa, ca = f32(math.sin(a)), f32(math.cos(a))
    flip = 0.0
    if death_time > 0.0:
        f5 = max((death_time - 1.0) / 20.0 * 1.6, 0.0)
        flip = min(math.sqrt(f5), 1.0) * (math.pi / 2)
    sf, cf = f32(math.sin(flip)), f32(math.cos(flip))
    mx, my, mz = -p[0], -p[1], p[2]
    fx = f32(mx * cf - my * sf)
    fy = f32(mx * sf + my * cf)
    px = f32(fx * ca + mz * sa)
    pz = f32(-fx * sa + mz * ca)
    return [f32(feet[0] + px), f32(feet[1] + (fy + 1.501) + crouch_drop), f32(feet[2] + pz)]

# THE STONE'S BAKE: the unit cube's eight corners (the block/block
# stanza's own space — the bake spans [0,1]^3)
CORNERS = [(x, y, z) for x in (0.0, 1.0) for y in (0.0, 1.0) for z in (0.0, 1.0)]

# the block/block thirdperson stanza (both hands share the JSON);
# the translation rides the deserializer's own /16 (vanilla's
# ItemTransform$Deserializer mul 0.0625 — the 2.5 JSON speaks 2.5/16)
ROTATION = (75.0, 45.0, 0.0)
TRANSLATION = (0.0, 2.5 / 16.0, 0.0)
SCALE = (0.375, 0.375, 0.375)

def box(chain):
    pts = [fold_point(apply(chain, c)) for c in CORNERS]
    lo = [min(p[k] for p in pts) for k in range(3)]
    hi = [max(p[k] for p in pts) for k in range(3)]
    return lo, hi

# the REST pose's arms (the bob's Z-component rides the cos term
# = +0.1 the 0.1 base keeps — the right arm's z-rot +0.2, the left's
# -0.2; the bob's X-component is the SIN term = 0.0 both arms keep)
M_RIGHT = arm_matrix([-5.0, 2.0, 0.0], 0.2, 0.0, 0.0)
M_LEFT = arm_matrix([5.0, 2.0, 0.0], -0.2, 0.0, 0.0)

def chain_for(left):
    m_arm = M_LEFT if left else M_RIGHT
    x_off = (-1.0 if left else 1.0) / 16.0
    pose = stanza_pose(ROTATION, TRANSLATION, SCALE, left)
    return mat_mul(mat_mul(m_arm, layer_matrix(x_off)), pose)

if __name__ == "__main__":
    rlo, rhi = box(chain_for(False))
    print("RIGHT (pinned: lo 0.20206 0.53111 -0.58591 / hi 0.79378 1.02064 0.02341)")
    print("  lo %.5f %.5f %.5f" % tuple(rlo))
    print("  hi %.5f %.5f %.5f" % tuple(rhi))
    llo, lhi = box(chain_for(True))
    print("LEFT (the offhand's own box)")
    print("  lo %.5f %.5f %.5f" % tuple(llo))
    print("  hi %.5f %.5f %.5f" % tuple(lhi))
