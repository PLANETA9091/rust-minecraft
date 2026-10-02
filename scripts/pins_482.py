#!/usr/bin/env python3
"""THE F32 CHAIN ORACLE for the third-person STAB attack-item lane (autopush-481).

Mirrors the 26.2 jar's own arithmetic per-op rounded to f32 —
SpearAnimations.thirdPersonAttackItem's two attack clocks, the XN
rotateAround and the forward translate. Every printed matrix lands
cell-pinned in the Rust tests (the client's own Mat4 column-major
multiply the per-op f32 rounding keeps — the same convention the
pins_480 oracle speaks).
"""
import struct
import math

FWD = 0.3799999952316284  # forwardMovement, the shared ldc #275


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
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


def progress(v, lo, hi):
    # THE PER-OP F32 CHAIN the jar's own bytecode speaks: fsub (rounds),
    # fsub (rounds), fdiv (rounds) — the numerator's own f32 store the
    # real subtraction keeps, never a f64 shortcut the JVM never wore
    # (the unrounded numerator the older oracle draft carried — the
    # 1-ulp asymmetry the t=1.0 pin EXPOSED: f32(1.0)-f32(0.4) rounds
    # to 0.6000000238418579 both sides, the ratio the EXACT 1.0 the
    # clamp never moves — the unrounded f64 0.5999999940395355 the
    # wrong draft divided spoke 0.99999994 instead)
    num = f32(f32(v) - f32(lo))
    den = f32(f32(hi) - f32(lo))
    return clamp(f32(num / den), 0.0, 1.0)


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


def translation(x, y, z):
    return [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, f32(x), f32(y), f32(z), 1]


def rot_around(r, ox, oy, oz):
    t1 = translation(ox, oy, oz)
    t2 = translation(-ox, -oy, -oz)
    return mat_f32_mul(mat_f32_mul(t1, r), t2)


def attack_item_matrix(attack_time, forward):
    """pcs 47-109 — the two clocks, the XN rotateAround, the translate."""
    f6 = in_quad(progress(attack_time, 0.05, 0.2))
    f7 = in_out_expo(progress(attack_time, 0.4, 1.0))
    rot_x_deg = mul(f32(70.0), sub(f6, f7))
    ty = mul(f32(forward), sub(f6, f7))
    m = rot_around(rot_x(-rot_x_deg), 0.0, -0.125, 0.125)
    m = mat_f32_mul(m, translation(0.0, ty, 0.0))
    return m, f6, f7, rot_x_deg, ty


def show(tag, attack_time, forward):
    m, f6, f7, deg, ty = attack_item_matrix(attack_time, forward)
    ident = all(
        m[i] == [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1][i]
        for i in range(16)
    )
    print(
        f"{tag} t={attack_time} fwd={forward}: f6={f6!r} f7={f7!r} "
        f"deg={deg!r} {bits(deg)} ty={ty!r} {bits(ty)} identity={ident}"
    )
    print(f"  m = [{', '.join(f'{v:.9g}' for v in m)}]")
    print(f"  bits = [{', '.join(bits(v) for v in m)}]")


# THE CLOCK WALK — the phases the two windows speak:
# t=0.0   rest: f6=f7=0           -> identity (the gate the jar keeps)
# t=0.05  raise's own first tick: progress=0 -> f6=0, f7=0 -> identity
# t=0.2   the raise's full: f6=1, f7=0 -> full 70°, ty=FWD
# t=0.3   the thrust's hold: f6=1, f7=0 -> full 70°, ty=FWD
# t=0.5   the unwind's middle: f6=1, f7=inOutExpo(1/6)
# t=1.0   the return: f6=f7=1     -> identity (the arc re-collides)
show("A[rest ]", 0.0, FWD)
show("B[raise-first]", 0.05, FWD)
show("C[hold ]", 0.3, FWD)
show("D[mid  ]", 0.5, FWD)
show("E[full ]", 0.2, FWD)
show("F[ret  ]", 1.0, FWD)
# THE NON-KINETIC MAIN HAND — forward 0.0 the component-less stack
# wears: the rotation rides, the translate never moves
show("G[hold-nokinet]", 0.3, 0.0)
