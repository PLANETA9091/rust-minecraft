#!/usr/bin/env python3
"""THE MTH-TABLE ORACLE for the kinetic-delay re-pin (dev-61).

The jar's own SIN table (autopush-478's law): SIN[i] = (float)
Math.sin(i / 10430.378350470453) — the DIVISION the bytecode speaks —
indexed by `(long)(d·C) & 65535` (the d2l truncation toward zero, the
mask the negative angles lift). Validates the OLD sway pins (t=60/50,
the iron delay-10 windows) then speaks the NEW ones (t=62/52, the
delay-12 windows the ctor evidence orders).
"""
import math
import struct

C = 10430.378350470453  # the f64 the ldc2_w banks (0x40c45f306dc9c883)
CONV = struct.unpack(">f", struct.pack(">f", 0.0174532924))[0]  # the client's f32 literal


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return f"0x{struct.unpack('>I', struct.pack('>f', x))[0]:08x}"


print("building the 65536-entry table (f32-rounded fills)...")
SIN = [f32(math.sin(i / C)) for i in range(65536)]


def mth_sin(d):
    idx = int(d * C) & 65535  # the d2l truncation toward zero, the mask lifts
    return SIN[idx]


def lane(t, mult):
    """The client's own f32 chain: (t·mult)·CONV, widened, the table."""
    x = f32(f32(f32(t) * f32(mult)) * CONV)
    return mth_sin(x)


def out_circ(x):
    # Ease.outCirc: sqrt(1 - (x-1)^2) — the f32 body
    t = f32(x - 1.0)
    return f32(math.sqrt(f32(1.0 - f32(t * t))))


# ---------- THE OLD PINS' VALIDATION (delay 10: ds=60, the sway at 60/50) ----------
pump60 = f32(2.0 * out_circ(1.0))
print(f"OLD t=60 pump={pump60!r} (pin 2.0)")
s60 = f32(lane(60.0, 19.0) * pump60)
f60 = f32(lane(60.0, 30.0) * pump60)
print(f"OLD t=60 slow={s60!r} {bits(s60)} (pin 1.7319869)")
print(f"OLD t=60 fast={f60!r} {bits(f60)} (pin -0.00019174760)")
pump50 = f32(2.0 * out_circ(0.5))
print(f"OLD t=50 pump={pump50!r} (pin 1.7320508)")
s50 = f32(lane(50.0, 19.0) * pump50)
f50 = f32(lane(50.0, 30.0) * pump50)
print(f"OLD t=50 slow={s50!r} {bits(s50)} (pin -1.3268041)")
print(f"OLD t=50 fast={f50!r} {bits(f50)} (pin 1.4999446)")

# ---------- THE NEW PINS (delay 12: ds=62, the sway at 62/52) ----------
pump62 = f32(2.0 * out_circ(1.0))
s62 = f32(lane(62.0, 19.0) * pump62)
f62 = f32(lane(62.0, 30.0) * pump62)
print(f"NEW t=62 pump={pump62!r} slow={s62!r} {bits(s62)}  fast={f62!r} {bits(f62)}")
pump52 = f32(2.0 * out_circ(0.5))
s52 = f32(lane(52.0, 19.0) * pump52)
f52 = f32(lane(52.0, 30.0) * pump52)
print(f"NEW t=52 pump={pump52!r} slow={s52!r} {bits(s52)}  fast={f52!r} {bits(f52)}")
