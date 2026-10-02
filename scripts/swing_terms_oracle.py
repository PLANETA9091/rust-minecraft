#!/usr/bin/env python3
"""The swing-terms oracle, v2 — the 65536-entry SIN table law.

The autopush-478 round swapped both swing lanes to the jar's own
Mth.sin(D)F/Mth.cos(D)F lookup (SIN[i] = (float) Math.sin(i / 10430.378350470453),
index (long)(d*C) & 65535, the cos the +16384.0 offset inside the single d2l).
The f64-sin chain this file's predecessor emulated is retired; the folds
below reproduce `swing_terms` / `item_swing_terms` bit-for-bit under the
table so the sweep test's pinned bit patterns regenerate.
"""
import math
import struct

C = 10430.378350470453  # the CP's ldc2_w #17, f64 bits 0x40c45f306dc9c883


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]


SIN = [f32(math.sin(i / C)) for i in range(65536)]


def mth_sin(d):
    # d arrives as an f64 (the jar's own signature); int() truncates
    # toward zero like the JVM's d2l, the mask lifts the negatives
    return SIN[int(d * C) & 0xFFFF]


def mth_sqrt_f(x_f32):
    return f32(math.sqrt(x_f32))


PI_F32 = f32(math.pi)          # std::f32::consts::PI
TWO_PI_F32 = f32(6.2831855)    # the bytecode's 2pi f32 literal


def swing_terms(s):
    """[f10, f11, f12, f13, f14] — the bare arm's five."""
    sq = mth_sqrt_f(s)
    f10 = f32(f32(-0.3) * mth_sin(f32(sq * PI_F32)))
    f11 = f32(f32(0.4) * mth_sin(f32(sq * TWO_PI_F32)))
    f12 = f32(f32(-0.4) * mth_sin(f32(s * PI_F32)))
    f13 = mth_sin(f32(f32(s * s) * PI_F32))
    f14 = mth_sin(f32(sq * PI_F32))
    return [f10, f11, f12, f13, f14]


def item_swing_terms(s):
    """[f5s, f6s, f7s, f5, f6] — the held item's five."""
    sq = mth_sqrt_f(s)
    f5s = f32(f32(-0.4) * mth_sin(f32(sq * PI_F32)))
    f6s = f32(f32(0.2) * mth_sin(f32(sq * TWO_PI_F32)))
    f7s = f32(f32(-0.2) * mth_sin(f32(s * PI_F32)))
    f5 = mth_sin(f32(f32(s * s) * PI_F32))
    f6 = mth_sin(f32(sq * PI_F32))
    return [f5s, f6s, f7s, f5, f6]


PROGRESSES = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0]


def main():
    print("const ARM: [(u32, [u32; 5]); 7] = [")
    for p in PROGRESSES:
        pb = bits(f32(p))
        row = ", ".join(f"0x{bits(v):08x}" for v in swing_terms(f32(p)))
        print(f"    (0x{pb:08x}, [{row}]),")
    print("];")
    print()
    print("const ITEM: [(u32, [u32; 5]); 7] = [")
    for p in PROGRESSES:
        pb = bits(f32(p))
        row = ", ".join(f"0x{bits(v):08x}" for v in item_swing_terms(f32(p)))
        print(f"    (0x{pb:08x}, [{row}]),")
    print("];")
    print()
    # the humans' cross-checks the old doc spoke
    print("p=1 arm terms:", swing_terms(1.0))
    print("p=0.25 rotY (f14):", swing_terms(0.25)[4])
    print("p=0.5 f12/f13:", swing_terms(0.5)[2], swing_terms(0.5)[3])
    print("p=0.75 f6 peak:", item_swing_terms(0.75)[4])


if __name__ == "__main__":
    main()
