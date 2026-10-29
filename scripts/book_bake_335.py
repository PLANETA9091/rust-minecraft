#!/usr/bin/env python3
"""the enchanting table's own book float bake (the autopush-335 round —
the 26.2 jar's BookModel.createBodyLayer + BookModel.setupAnim +
EnchantTableRenderer.submit + ModelPart$Cube/<Polygon> the probes
scripts/book_float_probe_335.py + book_tex_probe_335.py bank): the
REST pose the fresh-BE zeros speak (time 0, open 0, flip 0, rot 0 —
the bookAnimationTick's no-viewer decay the rest pins), the closed
book the static block lane renders:

  anchor  = (0.5, 0.75 + 0.1 + sin(0)*0.01, 0.5) = (0.5, 0.85, 0.5)
  world_p = anchor + R_z(+80deg) * p_model / 16
  leftLid   yRot = pi + openness = pi   offset (0, 0, -1)
  rightLid  yRot = -openness = 0        offset (0, 0, +1)
  pages     yRot = openness = 0         ZERO pose (sin(open)=0 x)
  flipPages yRot = 0 (interior — the right_pages slab the z band
            contains, the occlusion the bake's own skip keeps)
  seam      0.01-px sliver the bake skips (sub-pixel the raster
            never resolves)

the face rects the ModelPart$Cube law speaks (the DOWN face the
FIRST strip (u0+dz, v0) wears, the UP face the SECOND strip the
v-swap carries, the sides the west/north/east/south row keeps), the
corner UVs the Polygon ctor assigns (k0 -> (u2,v1), k1 -> (u1,v1),
k2 -> (u1,v2), k3 -> (u2,v2)), the FOUR 16x16 tiles the crops serve
(LID_LEFT = tex (0,0)-(16,16), LID_RIGHT = (16,0)-(32,16),
PAGES_LEFT = (0,3)-(16,19) [the page rects v 10..19 the v 3..19
window answers], PAGES_RIGHT = (12,3)-(28,19) — the u 12..24 span
the pages-right wears the single crop answers)."""
import struct
import zlib

# --- the book texture decode (the palette PNG the jar serves) ---
data = open(
    "assets-cache/jar/assets/minecraft/textures/entity/enchantment/enchanting_table_book.png",
    "rb",
).read()
pos = 8
w = h = 0
idat = b""
plte = b""
trns = b""
while pos < len(data):
    ln, typ = struct.unpack(">I4s", data[pos : pos + 8])
    if typ == b"IHDR":
        w, h, bd, ctype = struct.unpack(">IIBB", data[pos + 8 : pos + 18])
    elif typ == b"PLTE":
        plte = data[pos + 8 : pos + 8 + ln]
    elif typ == b"tRNS":
        trns = data[pos + 8 : pos + 8 + ln]
    elif typ == b"IDAT":
        idat += data[pos + 8 : pos + 8 + ln]
    pos += 12 + ln
assert (w, h) == (64, 32) and ctype == 3, (w, h, ctype)
raw = zlib.decompress(idat)
out = bytearray()
prev = bytearray(w)
o = 0
for y in range(h):
    f = raw[o]
    o += 1
    line = bytearray(raw[o : o + w])
    o += w
    if f == 1:
        for i in range(1, w):
            line[i] = (line[i] + line[i - 1]) & 255
    elif f == 2:
        for i in range(w):
            line[i] = (line[i] + prev[i]) & 255
    elif f == 3:
        for i in range(w):
            line[i] = (line[i] + ((line[i - 1] if i else 0) + prev[i]) // 2) & 255
    elif f == 4:
        pa = line[i - 1] if i else 0

        def paeth(a, b, c):
            p = a + b - c
            pp = (abs(p - a), abs(p - b), abs(p - c))
            return a if pp[0] <= pp[1] and pp[0] <= pp[2] else (b if pp[1] <= pp[2] else c)

        for i in range(w):
            line[i] = (line[i] + paeth(line[i - 1] if i else 0, prev[i], prev[i - 1] if i else 0)) & 255
    out += line
    prev = line
rgba = bytearray(64 * 32 * 4)
for i in range(64 * 32):
    pi = out[i]
    rgba[i * 4 : i * 4 + 3] = plte[pi * 3 : pi * 3 + 3]
    rgba[i * 4 + 3] = trns[pi] if pi < len(trns) else 255

# --- the four 16x16 crops ---


def crop(cx0, cy0):
    tile = bytearray(16 * 16 * 4)
    for y in range(16):
        for x in range(16):
            s = ((cy0 + y) * 64 + (cx0 + x)) * 4
            d = (y * 16 + x) * 4
            tile[d : d + 4] = rgba[s : s + 4]
    return tile


tile_lid_l = crop(0, 0)
tile_lid_r = crop(16, 0)
tile_page_l = crop(0, 3)
tile_page_r = crop(12, 3)


def fmt_bytes(tile):
    rows = []
    for y in range(16):
        row = tile[y * 64 : (y + 1) * 64]
        rows.append("    " + ",".join(str(b) for b in row) + ",")
    return "\n".join(rows)


# --- the model constants (the jar law) ---
C80 = 0.17364817766693033  # cos 80 deg
S80 = 0.984807753012208    # sin 80 deg
ANCHOR = (0.5, 0.85, 0.5)  # 0.75 + 0.1 lift, bob 0 at rest

# --- the face corner orders (ModelPart$Cube law) ---
def face_corners(x0, y0, z0, x1, y1, z1, f):
    if f == "N":
        return [(x1, y0, z0), (x0, y0, z0), (x0, y1, z0), (x1, y1, z0)]
    if f == "S":
        return [(x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1)]
    if f == "W":
        return [(x0, y0, z0), (x0, y0, z1), (x0, y1, z1), (x0, y1, z0)]
    if f == "E":
        return [(x1, y0, z1), (x1, y0, z0), (x1, y1, z0), (x1, y1, z1)]
    if f == "D":
        return [(x1, y0, z1), (x0, y0, z1), (x0, y0, z0), (x1, y0, z0)]
    if f == "U":
        return [(x1, y1, z0), (x0, y1, z0), (x0, y1, z1), (x1, y1, z1)]


def face_rect(u0, v0, dx, dy, dz, f):
    # (u1, v1, u2, v2) per the Cube law
    if f == "N":
        return (u0 + dz, v0 + dz, u0 + dz + dx, v0 + dz + dy)
    if f == "S":
        return (u0 + 2 * dz + dx, v0 + dz, u0 + 2 * dz + 2 * dx, v0 + dz + dy)
    if f == "W":
        return (u0, v0 + dz, u0 + dz, v0 + dz + dy)
    if f == "E":
        return (u0 + dz + dx, v0 + dz, u0 + 2 * dz + dx, v0 + dz + dy)
    if f == "D":
        return (u0 + dz, v0, u0 + dz + dx, v0 + dz)
    if f == "U":
        return (u0 + dz + dx, v0 + dz, u0 + 2 * dz + dx, v0)  # v swapped


def uv_corner(rect, k):
    u1, v1, u2, v2 = rect
    # Polygon law: k0 -> (u2,v1), k1 -> (u1,v1), k2 -> (u1,v2), k3 -> (u2,v2)
    return [(u2, v1), (u1, v1), (u1, v2), (u2, v2)][k]


def rot_y(p, ang_cos, ang_sin):
    # rotation about Y: x' = x*c + z*s ; z' = -x*s + z*c  (right-handed)
    x, y, z = p
    return (x * ang_cos + z * ang_sin, y, -x * ang_sin + z * ang_cos)


def world_transform(p, lid):
    # part transform then world transform
    if lid == "left":
        p = (-p[0], p[1], -p[2])       # R_y(pi)
        p = (p[0], p[1], p[2] - 1.0)   # offset (0,0,-1)
    elif lid == "right":
        p = (p[0], p[1], p[2] + 1.0)   # offset (0,0,+1)
    # pages: ZERO pose
    x, y, z = p
    x /= 16.0
    y /= 16.0
    z /= 16.0
    wx = x * C80 - y * S80
    wy = x * S80 + y * C80
    return (ANCHOR[0] + wx, ANCHOR[1] + wy, ANCHOR[2] + z)


# --- the parts (createBodyLayer law; the slivers + flip pages skip) ---
PARTS = [
    # (lid, box(x0,y0,z0,dx,dy,dz), texOffs(u0,v0), faces)
    ("left", (-6.0, -5.0, -0.005, 6.0, 10.0, 0.005), (0, 0), ("N", "S")),
    ("right", (0.0, -5.0, -0.005, 6.0, 10.0, 0.005), (16, 0), ("N", "S")),
    ("pages", (0.0, -4.0, -0.99, 5.0, 8.0, 1.0), (0, 10), ("N", "S", "W", "E", "D", "U")),
    ("pages", (0.0, -4.0, -0.01, 5.0, 8.0, 1.0), (12, 10), ("N", "S", "W", "E", "D", "U")),
]

quads = []
for lid, (x0, y0, z0, dx, dy, dz), (u0, v0), faces in PARTS:
    x1, y1, z1 = x0 + dx, y0 + dy, z0 + dz
    for f in faces:
        rect = face_rect(u0, v0, dx, dy, dz, f)
        corners = face_corners(x0, y0, z0, x1, y1, z1, f)
        pts = [world_transform(c, lid) for c in corners]
        uvs = [uv_corner(rect, k) for k in range(4)]
        # tile: by PART — the four crops the part's own rect contains
        # (the leftLid the (0,0) crop, the rightLid the (16,0) crop, the
        # leftPages the (0,3) crop [v 10..19 -> local 7..16], the
        # rightPages the (12,3) crop — the straddle the u_center law
        # never risks, the v overflow the two-window split answers)
        if lid == "left":
            tile, cx0, cy0 = 1070, 0, 0
        elif lid == "right":
            tile, cx0, cy0 = 1071, 16, 0
        elif u0 == 0:
            tile, cx0, cy0 = 1072, 0, 3
        else:
            tile, cx0, cy0 = 1073, 12, 3
        local_uvs = [((u - cx0) / 16.0, (v - cy0) / 16.0) for u, v in uvs]
        quads.append((tile, pts, local_uvs, f, lid))

print("quads:", len(quads))
from collections import Counter

print("tiles:", Counter(q[0] for q in quads))

# --- the world-space span verdict ---
xs = [p[0] for _, pts, _, _, _ in quads for p in pts]
ys = [p[1] for _, pts, _, _, _ in quads for p in pts]
zs = [p[2] for _, pts, _, _, _ in quads for p in pts]
print("x span %.4f..%.4f (px %.2f..%.2f)" % (min(xs), max(xs), min(xs) * 16, max(xs) * 16))
print("y span %.4f..%.4f (px %.2f..%.2f)" % (min(ys), max(ys), min(ys) * 16, max(ys) * 16))
print("z span %.4f..%.4f (px %.2f..%.2f)" % (min(zs), max(zs), min(zs) * 16, max(zs) * 16))
assert min(ys) > 0.75, "the book must clear the pedestal ceiling (12 px)"
assert max(ys) > 1.0, "the book must poke above the block top (16 px)"
assert min(zs) >= 0.0 and max(zs) <= 1.0, "the book must stay in the cell column"
assert min(xs) >= 0.0 and max(xs) <= 1.0, "the book must stay in the cell column"

# --- the rust emission ---
def f32(v):
    return float(struct.unpack(">f", struct.pack(">f", v))[0])


def fmt(v):
    v = f32(v)
    s = "%.6f" % v
    s = s.rstrip("0").rstrip(".")
    return s if s not in ("", "-") else "0.0" if s == "-0" else (s + ".0" if "." not in s else s)


print()
print("=== TILE_1070_RGBA (lid left, crop 0,0) ===")
print(fmt_bytes(tile_lid_l))
print("=== TILE_1071_RGBA (lid right, crop 16,0) ===")
print(fmt_bytes(tile_lid_r))
print("=== TILE_1072_RGBA (pages left, crop 0,3) ===")
print(fmt_bytes(tile_page_l))
print("=== TILE_1073_RGBA (pages right, crop 12,3) ===")
print(fmt_bytes(tile_page_r))
print()
print("=== BOOK QUADS (tile, pts, uvs) ===")
lines = []
for tile, pts, uvs, f, lid in quads:
    ptxt = ", ".join(
        "[%s, %s, %s]" % (fmt(p[0]), fmt(p[1]), fmt(p[2])) for p in pts
    )
    utxt = ", ".join("[%s, %s]" % (fmt(u), fmt(v)) for u, v in uvs)
    lines.append("    BookQuad { tile: %s, pts: [%s], uvs: [%s] }, // %s %s" % (
        {1070: "TILE_ENCHANTING_TABLE_BOOK_LID_LEFT",
         1071: "TILE_ENCHANTING_TABLE_BOOK_LID_RIGHT",
         1072: "TILE_ENCHANTING_TABLE_BOOK_PAGES_LEFT",
         1073: "TILE_ENCHANTING_TABLE_BOOK_PAGES_RIGHT"}[tile], ptxt, utxt, lid, f))
print("\n".join(lines))
