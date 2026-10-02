#!/usr/bin/env python3
"""THE F32 CHAIN ORACLE for the third-person spear use lanes (autopush-480).

Mirrors the 26.2 jar's own arithmetic per-op rounded to f32 — the
thirdPersonHandUse base raise/clamps/sway folds and the
thirdPersonUseItem translate/rotateAround chain. Every printed value
lands bit-pinned in the Rust tests (struct round-trip per operation,
never a f64 shortcut the jar never wore).
"""
import struct
import math

FMAX = 3.4028234663852886e38


def f32(x: float) -> float:
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x: float) -> str:
    return f"0x{struct.unpack('>I', struct.pack('>f', x))[0]:08x}"


def add(a, b):
    return f32(f32(a) + f32(b))


def sub(a, b):
    return f32(f32(a) - f32(b))


def mul(a, b):
    return f32(f32(a) * f32(b))


def clamp(v, lo, hi):
    v = f32(v)
    if v < lo:
        return f32(lo)
    if v > hi:
        return f32(hi)
    return v


# ---------- the constants ----------
DEG = f32(0.01745329238474369)          # the CP float the jar folds
TO_DEG = f32(57.2957763671875)          # the f32 180/pi
NEG_HALF_PI = f32(-1.5707963705062866)
EIGHT = f32(0.800000011920929)
FLY_EXTRA = f32(0.9599310755729675)
SIDE_NUDGE = f32(-0.10000000149011612)
FWD = f32(0.3799999952316284)           # forwardMovement, CP #275

print("DEG      =", DEG, bits(DEG))
print("TO_DEG   =", TO_DEG, bits(TO_DEG))
print("FWD      =", FWD, bits(FWD))


def hand_use_base(x0, y0, z0, right, fall_flying, swim):
    """pcs 11-108 — the base raise + the degree clamps (no kinetic)."""
    side = 1.0 if right else -1.0
    x = add(x0, NEG_HALF_PI)
    x = add(x, EIGHT)
    if fall_flying or swim > 0.0:
        x = sub(x, FLY_EXTRA)
    y = add(y0, mul(SIDE_NUDGE, f32(side)))
    # the clamps (the zRot never clamped)
    x = mul(DEG, clamp(mul(x, TO_DEG), -120.0, 30.0))
    y = mul(DEG, clamp(mul(y, TO_DEG), -60.0, 60.0))
    return x, y, z0


def hand_use_sway(x, y, z, right, p):
    """pcs 181-310 — the kinetic sway over the clamped pose."""
    side = 1.0 if right else -1.0
    neg_side = f32(-side)
    y = add(y, mul(mul(mul(mul(neg_side, p["fast"]), DEG), p["int"]), 1.0))
    z = add(z, mul(mul(mul(mul(neg_side, p["slow"]), DEG), p["int"]), 0.5))
    s = mul(f32(-40.0), p["rs"])
    s = add(s, mul(f32(30.0), p["rm"]))
    s = add(s, mul(f32(-20.0), p["re"]))
    s = add(s, mul(f32(20.0), p["lo"]))
    s = add(s, mul(f32(10.0), p["rb"]))
    s = add(s, mul(f32(0.6), mul(p["slow"], p["int"])))
    x = add(x, mul(DEG, s))
    return x, y, z


# ---------- A: the walk-zero arm's rest raise (right / left) ----------
for right in (True, False):
    x, y, z = hand_use_base(0.0, 0.0, 0.0, right, False, 0.0)
    tag = "RIGHT" if right else "LEFT "
    print(f"A[{tag}] rest  x={x!r} {bits(x)}  y={y!r} {bits(y)}  z=0.0")

# ---------- B: the clamps bite ----------
x, y, _ = hand_use_base(2.0, 1.5, 0.0, True, False, 0.0)
print(f"B[ bite] x0=2.0/y0=1.5  x={x!r} {bits(x)}  y={y!r} {bits(y)}")

# ---------- C: the fall-flying extra ----------
x, y, _ = hand_use_base(0.0, 0.0, 0.0, True, True, 0.0)
print(f"C[ fly]  x={x!r} {bits(x)}  y={y!r} {bits(y)}")
x, y, _ = hand_use_base(0.0, 0.0, 0.0, True, False, 0.5)
print(f"C[swim]  x={x!r} {bits(x)}  y={y!r} {bits(y)}")

# ---------- E: the kinetic sway (explicit params) ----------
P = {
    "fast": f32(-0.5), "slow": f32(1.25), "int": f32(0.75),
    "rs": f32(0.25), "rm": f32(0.5), "re": f32(0.75), "lo": f32(0.0),
    "rb": f32(0.375),
}
x0, y0, z0 = hand_use_base(0.0, 0.0, 0.0, True, False, 0.0)
x, y, z = hand_use_sway(x0, y0, z0, True, P)
print(f"E[sway]  x={x!r} {bits(x)}  y={y!r} {bits(y)}  z={z!r} {bits(z)}")
x0, y0, z0 = hand_use_base(0.0, 0.0, 0.0, False, False, 0.0)
x, y, z = hand_use_sway(x0, y0, z0, False, P)
print(f"E[swayL] x={x!r} {bits(x)}  y={y!r} {bits(y)}  z={z!r} {bits(z)}")


# ---------- F: the use matrix (thirdPersonUseItem) ----------
def progress(v, lo, hi):
    return clamp(f32((f32(v) - f32(lo)) / f32(f32(hi) - f32(lo))), 0.0, 1.0)


def in_quad(x):
    return mul(x, x)


def in_out_expo(x):
    x = f32(x)
    if x < 0.5:
        if x == 0.0:
            return 0.0
        return f32((2.0 ** (20.0 * x - 10.0)) / 2.0)
    if x == 1.0:
        return 1.0
    return f32((2.0 - 2.0 ** (-20.0 * x + 10.0)) / 2.0)


def out_back(x):
    t = sub(x, 1.0)
    return add(1.0, add(mul(f32(2.70158), mul(mul(t, t), t)), mul(f32(1.70158), mul(t, t))))


def hit_feedback(t):
    def out_quart(x):
        tt = sub(1.0, x)
        return sub(1.0, mul(mul(tt, tt), mul(tt, tt)))

    def in_out_sine(x):
        # Ease.inOutSine — the (1 - cos(pi·x))/2 cos form the jar's own
        # body keeps (the Mth table the client's ease rides — the
        # fb(5.0) = 0.32470223 the client's own pin speaks)
        return f32((1.0 - math.cos(math.pi * f32(x))) / 2.0)

    return mul(f32(0.4), sub(out_quart(progress(t, 1.0, 3.0)), in_out_sine(progress(t, 3.0, 10.0))))


def mat_f32_mul(a, b):
    out = []
    for col in range(4):
        for row in range(4):
            s = 0.0
            for k in range(4):
                s = f32(s + f32(a[k * 4 + row] * b[col * 4 + k]))
            out.append(s)
    return out


def rot_x(deg):
    r = math.radians(deg)
    s, c = f32(math.sin(r)), f32(math.cos(r))
    return [1, 0, 0, 0, 0, c, s, 0, 0, -s, c, 0, 0, 0, 0, 1]


def rot_y(deg):
    r = math.radians(deg)
    s, c = f32(math.sin(r)), f32(math.cos(r))
    return [c, 0, -s, 0, 0, 1, 0, 0, s, 0, c, 0, 0, 0, 0, 1]


def translation(x, y, z):
    return [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, f32(x), f32(y), f32(z), 1]


def rot_around(r, ox, oy, oz):
    t1 = translation(ox, oy, oz)
    t2 = translation(-ox, -oy, -oz)
    return mat_f32_mul(mat_f32_mul(t1, r), t2)


def use_matrix(p, attack_time, feedback, side):
    f6 = in_quad(progress(attack_time, 0.05, 0.2))
    f7 = in_out_expo(progress(attack_time, 0.4, 1.0))
    f10 = sub(1.0, out_back(sub(1.0, p["raise"])))
    f12 = f32(feedback)
    ty = f32(f32(-f12) * 0.4)              # the F64 product the f32 store narrows
    tz = f32(add(mul(f32(-FWD), sub(f10, p["rb"])), f12))
    rot_x_deg = sub(mul(f32(70.0), sub(p["raise"], p["rb"])), mul(f32(40.0), sub(f6, f7)))
    rot_y_deg = mul(f32(side), mul(f32(90.0), add(add(sub(p["raise"], p["sway"]), mul(f32(3.0), f7)), f6)))
    m = translation(0.0, ty, tz)
    m = mat_f32_mul(m, rot_around(rot_x(-rot_x_deg), 0.0, -0.03125, 0.125))
    m = mat_f32_mul(m, rot_around(rot_y(rot_y_deg), 0.0, 0.0, 0.125))
    return m


REST = {"raise": 0.0, "rb": 0.0, "sway": 0.0, "rs": 0.0, "rm": 0.0, "re": 0.0, "lo": 0.0}
m = use_matrix(REST, 0.0, 0.0, 1.0)
ident = all(m[i] == [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1][i] for i in range(16))
print("F[rest]  identity:", ident)

FULL = {"raise": 1.0, "rb": 0.0, "sway": 0.0, "rs": 0.0, "rm": 0.0, "re": 0.0, "lo": 0.0}
f10 = sub(1.0, out_back(sub(1.0, 1.0)))
tz = f32(add(mul(f32(-FWD), sub(f10, 0.0)), 0.0))
ty = f32(0.0 * 0.4)
print(f"F[full]  f10={f10!r}  ty={ty!r} {bits(ty)}  tz={tz!r} {bits(tz)}")
m = use_matrix(FULL, 0.0, 0.0, 1.0)
print("F[full]  m =", "[" + ", ".join(bits(v) for v in m) + "]")
print("F[full]  m =", [f"{v:.9g}" for v in m])
m = use_matrix(FULL, 0.5, f32(0.32470223), -1.0)
fb = f32(0.32470223)  # the Mth TABLE's verdict the client's own pin speaks
print(f"F[midL]  fb(5)={fb!r} {bits(fb)}")
print("F[midL]  m =", [f"{v:.9g}" for v in m])
