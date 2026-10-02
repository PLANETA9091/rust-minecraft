#!/usr/bin/env python3
"""pins_483.py — the BOW_AND_ARROW case-3 oracle (the f32 chains the
jar's own bytecode keeps). Every op rounds: the client's folds must
reproduce the EXACT bits these chains speak.

poseRightArm case 3 (right arm the BOW verdict):
  right.yRot = f32(-0.1 + hy)
  left.yRot  = f32(f32(0.1 + hy) + 0.4)
  right.xRot = f32(-1.5707963705062866 + hx)
  left.xRot  = f32(-1.5707963705062866 + hx)
poseLeftArm case 3 (left arm the BOW verdict):
  right.yRot = f32(f32(-0.1 + hy) - 0.4)
  left.yRot  = f32(0.1 + hy)
  right.xRot = f32(-1.5707963705062866 + hx)
  left.xRot  = f32(-1.5707963705062866 + hx)
hx = f32(30.0 * DEG2RAD_F32), hy = f32(20.0 * DEG2RAD_F32) — the
client's f32::to_radians (f32 PI / 180 first, one f32 mul).
"""
import struct

def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]

def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]

def add(a, b):
    return f32(f32(a) + f32(b))

def sub(a, b):
    return f32(f32(a) - f32(b))

NEG_0_1 = f32(-0.1)
POS_0_1 = f32(0.1)
POS_0_4 = f32(0.4)
BOW_PITCH = f32(-1.5707963705062866)  # CP #392, 0xBFC90FDB
DEG2RAD = f32(f32(3.14159274101257324) / f32(180.0))  # the f32 to_radians factor

hx = f32(30.0 * DEG2RAD)
hy = f32(20.0 * DEG2RAD)

print(f"DEG2RAD  = {DEG2RAD!r}  bits 0x{bits(DEG2RAD):08X}")
print(f"hx (30d) = {hx!r}  bits 0x{bits(hx):08X}")
print(f"hy (20d) = {hy!r}  bits 0x{bits(hy):08X}")
print()

# poseRightArm case 3
r_y_R = add(NEG_0_1, hy)
l_y_R = add(add(POS_0_1, hy), POS_0_4)
x_R = add(BOW_PITCH, hx)
print("RIGHT BODY (poseRightArm case 3, lane A):")
print(f"  right.yRot = {r_y_R!r}  bits 0x{bits(r_y_R):08X}")
print(f"  left.yRot  = {l_y_R!r}  bits 0x{bits(l_y_R):08X}")
print(f"  right.xRot = left.xRot = {x_R!r}  bits 0x{bits(x_R):08X}")

# poseLeftArm case 3
r_y_L = sub(add(NEG_0_1, hy), POS_0_4)
l_y_L = add(POS_0_1, hy)
print("LEFT BODY (poseLeftArm case 3, lanes B+C):")
print(f"  right.yRot = {r_y_L!r}  bits 0x{bits(r_y_L):08X}")
print(f"  left.yRot  = {l_y_L!r}  bits 0x{bits(l_y_L):08X}")
print(f"  right.xRot = left.xRot = {x_R!r}  bits 0x{bits(x_R):08X}")

print()
print("Rust literals:")
print(f"  r_y_R = f32::from_bits(0x{bits(r_y_R):08X})  // {r_y_R!r}")
print(f"  l_y_R = f32::from_bits(0x{bits(l_y_R):08X})  // {l_y_R!r}")
print(f"  r_y_L = f32::from_bits(0x{bits(r_y_L):08X})  // {r_y_L!r}")
print(f"  l_y_L = f32::from_bits(0x{bits(l_y_L):08X})  // {l_y_L!r}")
print(f"  x     = f32::from_bits(0x{bits(x_R):08X})  // {x_R!r}")
