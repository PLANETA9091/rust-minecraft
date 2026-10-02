#!/usr/bin/env python3
"""pins_484.py — the BLOCK case-2 oracle (poseBlockingArm, the f32
chains the jar's own bytecode keeps). Every op rounds: the client's
fold must reproduce the EXACT bits these chains speak.

poseBlockingArm(arm, right):
  arm.xRot = f32(f32(f32(arm.xRot * 0.5) - TILT) + clamp(head.xRot, PITCH_MIN, PITCH_MAX))
  arm.yRot = f32(f32(side * DEG2RAD) + clamp(head.yRot, -YAW_CLAMP, YAW_CLAMP))
  side = right ? -30.0 : 30.0   (CP #421 / #422, the iconst gate the
                                 ifeq answers — DEGREES the fmul the
                                 DEG2RAD factor converts)
TILT       = CP #418 = 0x3F71463B (54 deg)
PITCH_MIN  = CP #419 = 0xBFB2B8C2 (-80 deg)
PITCH_MAX  = CP #420 = 0x3EDF66F3 (25 deg)
YAW_CLAMP  = CP #417/#423 = 0x3F060A92 / 0xBF060A92 (30 deg)
DEG2RAD    = CP #242 = 0x3C8EFA35
the head reads hx = f32(30 * DEG2RAD_F32), the client's to_radians
law (f32 PI / 180 first, one f32 mul) — the same factor the CP keeps.
"""
import struct

def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]

def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]

def add(a, b):
    return f32(f32(a) + f32(b))

def mul(a, b):
    return f32(f32(a) * f32(b))

def clamp(v, lo, hi):
    v = f32(v)
    if v < f32(lo):
        return f32(lo)
    if v > f32(hi):
        return f32(hi)
    return v

TILT      = f32(0.9424778819084167)    # CP #418, 0x3F71463B
PITCH_MIN = f32(-1.3962633609771729)   # CP #419, 0xBFB2B8C2
PITCH_MAX = f32(0.4363323152065277)    # CP #420, 0x3EDF66F3
YAW_CLAMP = f32(0.5235987901687622)    # CP #417, 0x3F060A92
DEG2RAD   = f32(0.01745329238474369)   # CP #242, 0x3C8EFA35
ITEM_TILT = f32(0.314_159_274_101_257_3)  # the ITEM case-1 constant

print(f"TILT      = {TILT!r}  0x{bits(TILT):08X}")
print(f"PITCH_MIN = {PITCH_MIN!r}  0x{bits(PITCH_MIN):08X}")
print(f"PITCH_MAX = {PITCH_MAX!r}  0x{bits(PITCH_MAX):08X}")
print(f"YAW_CLAMP = {YAW_CLAMP!r}  0x{bits(YAW_CLAMP):08X}")
print(f"DEG2RAD   = {DEG2RAD!r}  0x{bits(DEG2RAD):08X}")
print(f"ITEM_TILT = {ITEM_TILT!r}  0x{bits(ITEM_TILT):08X}")

hx30  = mul(30.0, DEG2RAD)    # head pitch +30 deg
hy20  = mul(20.0, DEG2RAD)    # head yaw offset +20 deg
hxm90 = mul(-90.0, DEG2RAD)   # head pitch -90 deg
hym40 = mul(-40.0, DEG2RAD)   # head yaw offset -40 deg
print(f"hx30  = {hx30!r}  0x{bits(hx30):08X}")
print(f"hy20  = {hy20!r}  0x{bits(hy20):08X}")
print(f"hxm90 = {hxm90!r}  0x{bits(hxm90):08X}")
print(f"hym40 = {hym40!r}  0x{bits(hym40):08X}")

def side_deg(side):
    return mul(side, DEG2RAD)  # the ldc/fmul one rounding

NEG30 = side_deg(-30.0)
POS30 = side_deg(30.0)
print(f"-30*DEG2RAD = {NEG30!r}  0x{bits(NEG30):08X}")
print(f"+30*DEG2RAD = {POS30!r}  0x{bits(POS30):08X}")
print()

# SCENE R — lane A, right shield, head pitch +30 (clamp MAX lane),
# yaw off +20 (no clamp), standing idle arm (start 0.0)
x_r = add(add(mul(0.0, 0.5), -TILT), clamp(hx30, PITCH_MIN, PITCH_MAX))
y_r = add(NEG30, clamp(hy20, -YAW_CLAMP, YAW_CLAMP))
print(f"SCENE R  right.xRot = {x_r!r}  0x{bits(x_r):08X}")
print(f"SCENE R  right.yRot = {y_r!r}  0x{bits(y_r):08X}")

# SCENE C — the clamp MIN lanes: head pitch -90, yaw off -40
x_c = add(add(mul(0.0, 0.5), -TILT), clamp(hxm90, PITCH_MIN, PITCH_MAX))
y_c = add(NEG30, clamp(hym40, -YAW_CLAMP, YAW_CLAMP))
print(f"SCENE C  right.xRot = {x_c!r}  0x{bits(x_c):08X}")
print(f"SCENE C  right.yRot = {y_c!r}  0x{bits(y_c):08X}")

# SCENE CROUCH — the xRot start 0.4 the crouch's own += rides, the
# x0.5 chain the non-zero start reads
x_cr = add(add(mul(0.4, 0.5), -TILT), clamp(hx30, PITCH_MIN, PITCH_MAX))
print(f"SCENE CR right.xRot = {x_cr!r}  0x{bits(x_cr):08X}")

# SCENE L — lanes B+C, left shield (side +30), same head as R
x_l = add(add(mul(0.0, 0.5), -TILT), clamp(hx30, PITCH_MIN, PITCH_MAX))
y_l = add(POS30, clamp(hy20, -YAW_CLAMP, YAW_CLAMP))
print(f"SCENE L  left.xRot  = {x_l!r}  0x{bits(x_l):08X}")
print(f"SCENE L  left.yRot  = {y_l!r}  0x{bits(y_l):08X}")

# GATE SCENES — the ITEM fold the second posing keeps (BLOCK the
# affectsOffhandPose false the gate opens)
item_x = add(mul(0.0, 0.5), -ITEM_TILT)
print(f"ITEM fold x (start 0.0) = {item_x!r}  0x{bits(item_x):08X}")

# the walk's own plain arm bits the scene asserts differ (age 0 idle)
print(f"0.4 f32 bits = 0x{bits(0.4):08X}")
