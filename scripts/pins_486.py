#!/usr/bin/env python3
"""pins_486.py — the CROSSBOW_CHARGE (case 5) and CROSSBOW_HOLD
(case 6) oracle (AnimationUtils.animateCrossbowCharge/Hold, the f32
chains the jar's own bytecode keeps). Every op rounds: the client's
fold must reproduce the EXACT bits these chains speak.

animateCrossbowCharge(rightArm, leftArm, maxDuration, ticks, right):
  charge = right ? rightArm : leftArm; other = right ? leftArm : rightArm
  charge.yRot = right ? #25(-0.8) : #26(0.8)
  charge.xRot = #27(-0.9707963466644287)
  other.xRot  = charge.xRot            (the float copy, pc 48-55)
  f  = clamp(ticks, 0.0, maxDuration)  (Mth.clamp)
  f1 = f32(f / maxDuration)
  other.yRot = f32(f32(lerp(f1, #32(0.4), #33(0.85))) * (right ? 1.0 : -1.0))
  other.xRot = f32(#27 + f32(f1 * f32(#19 - #27)))   (lerp(f1, #27, #19))
animateCrossbowHold(rightArm, leftArm, head, right):
  charge = right ? rightArm : leftArm; other = right ? leftArm : rightArm
  charge.yRot = f32((right ? #7(-0.3) : #8(0.3)) + head.yRot)
  other.yRot  = f32((right ? #15(0.6) : #16(-0.6)) + head.yRot)
  charge.xRot = f32(f32(#19(-pi/2) + head.xRot) + #23(0.1))
  other.xRot  = f32(#24(-1.5) + head.xRot)
Mth.lerp(delta, start, end) = f32(start + f32(delta * f32(end - start)))
the head reads hx = f32(deg * TO_RAD_F32), the client's to_radians law.
"""
import math
import struct


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]


def sub(a, b):
    return f32(f32(a) - f32(b))


def mul(a, b):
    return f32(f32(a) * f32(b))


def add(a, b):
    return f32(f32(a) + f32(b))


def clamp(v, lo, hi):
    v = f32(v)
    if v < f32(lo):
        return f32(lo)
    if v > f32(hi):
        return f32(hi)
    return v


def lerp(delta, start, end):
    return add(start, mul(delta, sub(end, start)))


CHARGE_Y_R = f32(-0.8)                    # CP #25
CHARGE_Y_L = f32(0.8)                     # CP #26
CHARGE_PITCH = f32(-0.9707963466644287)   # CP #27
LERP_START = f32(0.4)                     # CP #32
LERP_END = f32(0.85)                      # CP #33
HOLD_PITCH = f32(-1.5707963705062866)     # CP #19
HOLD_Y_R = f32(-0.3)                      # CP #7
HOLD_Y_L = f32(0.3)                       # CP #8
OTHER_Y_R = f32(0.6)                      # CP #15
OTHER_Y_L = f32(-0.6)                     # CP #16
PITCH_ADD = f32(0.1)                      # CP #23
OTHER_PITCH = f32(-1.5)                   # CP #24
TO_RAD = f32(math.pi / 180.0)
MAX = f32(25.0)

for name, v in [("CHARGE_Y_R", CHARGE_Y_R), ("CHARGE_Y_L", CHARGE_Y_L),
                ("CHARGE_PITCH", CHARGE_PITCH), ("LERP_START", LERP_START),
                ("LERP_END", LERP_END), ("HOLD_PITCH", HOLD_PITCH),
                ("HOLD_Y_R", HOLD_Y_R), ("HOLD_Y_L", HOLD_Y_L),
                ("OTHER_Y_R", OTHER_Y_R), ("OTHER_Y_L", OTHER_Y_L),
                ("PITCH_ADD", PITCH_ADD), ("OTHER_PITCH", OTHER_PITCH)]:
    print(f"{name} = {v!r}  0x{bits(v):08X}")


def hx_of(deg):
    return mul(f32(deg), TO_RAD)


def charge(ticks, right):
    f1 = f32(clamp(ticks, 0.0, MAX) / MAX)
    cy = CHARGE_Y_R if right else CHARGE_Y_L
    cx = CHARGE_PITCH
    oy = mul(lerp(f1, LERP_START, LERP_END), f32(1.0) if right else f32(-1.0))
    ox = add(CHARGE_PITCH, mul(f1, sub(HOLD_PITCH, CHARGE_PITCH)))
    return cx, cy, ox, oy


def hold(deg_pitch, deg_yaw, right):
    hx = hx_of(deg_pitch)
    hy = hx_of(deg_yaw)
    cy = add(HOLD_Y_R if right else HOLD_Y_L, hy)
    oy = add(OTHER_Y_R if right else OTHER_Y_L, hy)
    cx = add(add(HOLD_PITCH, hx), PITCH_ADD)
    ox = add(OTHER_PITCH, hx)
    return cx, cy, ox, oy


print("\n== CHARGE scene R (ticks 1.0/25.0 — the right arm the CHARGE arm) ==")
cx, cy, ox, oy = charge(1.0, True)
print(f"charge(right).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(right).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(left).yRot   = {oy!r}  0x{bits(oy):08X}")
print(f"other(left).xRot   = {ox!r}  0x{bits(ox):08X}")

print("\n== CHARGE scene F (ticks 30 -> clamp 25, f1 = 1.0) ==")
cx, cy, ox, oy = charge(30.0, True)
print(f"charge(right).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(right).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(left).yRot   = {oy!r}  0x{bits(oy):08X}")
print(f"other(left).xRot   = {ox!r}  0x{bits(ox):08X}")

print("\n== CHARGE scene L (ticks 12.0/25.0 — the LEFT arm the CHARGE arm, the off-hand lane) ==")
cx, cy, ox, oy = charge(12.0, False)
print(f"charge(left).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(left).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(right).yRot = {oy!r}  0x{bits(oy):08X}")
print(f"other(right).xRot = {ox!r}  0x{bits(ox):08X}")

print("\n== HOLD scene R (pitch 0, yaw 20 — the right arm the HOLD arm) ==")
cx, cy, ox, oy = hold(0.0, 20.0, True)
print(f"charge(right).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(right).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(left).xRot   = {ox!r}  0x{bits(ox):08X}")
print(f"other(left).yRot   = {oy!r}  0x{bits(oy):08X}")

print("\n== HOLD scene L (pitch 0, yaw 20 — the LEFT arm the HOLD arm) ==")
cx, cy, ox, oy = hold(0.0, 20.0, False)
print(f"charge(left).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(left).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(right).xRot = {ox!r}  0x{bits(ox):08X}")
print(f"other(right).yRot = {oy!r}  0x{bits(oy):08X}")

print("\n== HOLD scene C (pitch -90, yaw 0 — the head's own pitch the sums read) ==")
cx, cy, ox, oy = hold(-90.0, 0.0, True)
print(f"charge(right).xRot = {cx!r}  0x{bits(cx):08X}")
print(f"charge(right).yRot = {cy!r}  0x{bits(cy):08X}")
print(f"other(left).xRot   = {ox!r}  0x{bits(ox):08X}")
print(f"other(left).yRot   = {oy!r}  0x{bits(oy):08X}")
