#!/usr/bin/env python3
"""pins_485.py — the SPYGLASS (case 7) and TOOT_HORN (case 8) oracle
(poseRightArm/poseLeftArm pc 326-386 / 389-430, the f32 chains the
jar's own bytecode keeps). Every op rounds: the client's fold must
reproduce the EXACT bits these chains speak.

SPYGLASS (the pure assignment — no walk read the value chain keeps):
  arm.xRot = clamp(f32(f32(head.xRot - #408) - (crouch ? #409 : 0.0)), #410, #411)
  right.yRot = f32(head.yRot - #409)      (the fsub the right body keeps)
  left.yRot  = f32(head.yRot + #409)      (the fadd the left body keeps)
TOOT_HORN:
  arm.xRot = f32(clamp(head.xRot, #187, #415) - #416)
  right.yRot = f32(head.yRot - #417)      (the fsub)
  left.yRot  = f32(head.yRot + #417)      (the fadd)
#408 = 1.9198622 (0x3FF5BE0B)   #409 = 0.2617994 (0x3E860A92)
#410 = -2.4 (0xC019999A)        #411 = 3.3 (0x40533333)
#187 = -1.2 (0xBF99999A)        #415 = 1.2 (0x3F99999A)
#416 = 1.4835298 (0x3FBDE44E)   #417 = 0.5235988 (0x3F060A92)
the head reads hx = f32(deg * TO_RAD_F32), the client's to_radians
law (f32 PI / 180 first, one f32 mul) — the same factor the CP keeps.
"""
import math
import struct


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]


def add(a, b):
    return f32(f32(a) + f32(b))


def sub(a, b):
    return f32(f32(a) - f32(b))


def mul(a, b):
    return f32(f32(a) * f32(b))


def clamp(v, lo, hi):
    v = f32(v)
    if v < f32(lo):
        return f32(lo)
    if v > f32(hi):
        return f32(hi)
    return v


PITCH_OFF = f32(1.919862151145935)     # CP #408
CROUCH_OFF = f32(0.2617993950843811)   # CP #409
CLAMP_MIN = f32(-2.4000000953674316)   # CP #410
CLAMP_MAX = f32(3.299999952316284)     # CP #411
HORN_MIN = f32(-1.2000000476837158)    # CP #187
HORN_MAX = f32(1.2000000476837158)     # CP #415
HORN_PITCH = f32(1.483529806137085)    # CP #416
HORN_YAW = f32(0.5235987901687622)     # CP #417
TO_RAD = f32(math.pi / 180.0)

print(f"PITCH_OFF  = {PITCH_OFF!r}  0x{bits(PITCH_OFF):08X}")
print(f"CROUCH_OFF = {CROUCH_OFF!r}  0x{bits(CROUCH_OFF):08X}")
print(f"CLAMP_MIN  = {CLAMP_MIN!r}  0x{bits(CLAMP_MIN):08X}")
print(f"CLAMP_MAX  = {CLAMP_MAX!r}  0x{bits(CLAMP_MAX):08X}")
print(f"HORN_MIN   = {HORN_MIN!r}  0x{bits(HORN_MIN):08X}")
print(f"HORN_MAX   = {HORN_MAX!r}  0x{bits(HORN_MAX):08X}")
print(f"HORN_PITCH = {HORN_PITCH!r}  0x{bits(HORN_PITCH):08X}")
print(f"HORN_YAW   = {HORN_YAW!r}  0x{bits(HORN_YAW):08X}")
print(f"TO_RAD     = {TO_RAD!r}  0x{bits(TO_RAD):08X}")


def hx_of(deg):
    return mul(f32(deg), TO_RAD)


def spyglass(deg_pitch, deg_yaw, crouch, right):
    hx = hx_of(deg_pitch)
    hy = hx_of(deg_yaw)
    x = clamp(sub(sub(hx, PITCH_OFF), CROUCH_OFF if crouch else f32(0.0)),
              CLAMP_MIN, CLAMP_MAX)
    y = sub(hy, CROUCH_OFF) if right else add(hy, CROUCH_OFF)
    return x, y


def horn(deg_pitch, deg_yaw, right):
    hx = hx_of(deg_pitch)
    hy = hx_of(deg_yaw)
    x = sub(clamp(hx, HORN_MIN, HORN_MAX), HORN_PITCH)
    y = sub(hy, HORN_YAW) if right else add(hy, HORN_YAW)
    return x, y


print("\n== SPYGLASS scene R (pitch 30, yaw 20, standing) ==")
x, y = spyglass(30.0, 20.0, False, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")
x, y = spyglass(30.0, 20.0, False, False)
print(f"left.xRot  = {x!r}  0x{bits(x):08X}")
print(f"left.yRot  = {y!r}  0x{bits(y):08X}")

print("\n== SPYGLASS scene C (pitch -90, yaw 0, crouching — clamp MIN + the crouch 15 the wipe keeps) ==")
x, y = spyglass(-90.0, 0.0, True, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")

print("\n== SPYGLASS scene Z (pitch 310 — clamp MAX lane) ==")
x, y = spyglass(310.0, 0.0, False, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")

print("\n== HORN scene R (pitch 0, yaw 20) ==")
x, y = horn(0.0, 20.0, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")
x, y = horn(0.0, 20.0, False)
print(f"left.xRot  = {x!r}  0x{bits(x):08X}")
print(f"left.yRot  = {y!r}  0x{bits(y):08X}")

print("\n== HORN scene C (pitch +90 — clamp MAX lane) ==")
x, y = horn(90.0, 0.0, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")

print("\n== HORN scene C2 (pitch -90 — clamp MIN lane) ==")
x, y = horn(-90.0, 0.0, True)
print(f"right.xRot = {x!r}  0x{bits(x):08X}")
print(f"right.yRot = {y!r}  0x{bits(y):08X}")

print("\n== GATES scene (right spyglass first-pose, left ITEM the gate-open folds — affectsOffhand FALSE the second posing keeps) ==")
item_tilt = f32(0.314_159_274_101_257_3)
left_ix = sub(f32(0.0), item_tilt)
print(f"left ITEM xRot (walk-zero start) = {left_ix!r}  0x{bits(left_ix):08X}")

print("\n== HORN gates scene (left horn first-pose, right ITEM the gate-open folds) ==")
print(f"right ITEM xRot = {left_ix!r}  0x{bits(left_ix):08X}")
