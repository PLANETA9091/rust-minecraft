#!/usr/bin/env python3
"""the enchanting table's own book tiles pin (the autopush-335 round —
the 319/333 idempotent precedent the pool-tail append keeps): the
jar's entity/enchantment/enchanting_table_book.png the four 16x16
crops serve (LID_LEFT (0,0), LID_RIGHT (16,0), PAGES_LEFT (0,3),
PAGES_RIGHT (12,3) — the bakes scripts/book_bake_335.py the rects
windows answer), TILE_1070..1073_RGBA the pool tail appends
(1070 -> 1074 the every-existing-index law keeps — the goldens the
same pixels render), TILE_NAMES the four tails wear, the named
consts the tests read. NO kind seat (texture-only mint — no
FACE_TILES/TINTS/DOOR/HEAD/CHEST rows change, N_KINDS stays 806).
The re-run the all-SKIP verdict keeps (the tree the pin never
moves)."""
import re
import sys

ATLAS = "crates/mc-registry/src/generated/atlas.rs"
BAKE = "scripts/jar_work/book_bake_335.txt"

# --- the bake's tile arrays ---
tiles = {}
cur = None
for line in open(BAKE):
    m = re.match(r"=== TILE_(\d+)_RGBA", line)
    if m:
        cur = int(m.group(1))
        tiles[cur] = []
    elif cur is not None and re.match(r"^ *[\d,]+,$", line):
        tiles[cur].append(line.strip())
    elif cur is not None and line.startswith("==="):
        cur = None
assert sorted(tiles) == [1070, 1071, 1072, 1073], sorted(tiles)
for t, rows in tiles.items():
    assert len(rows) == 16, (t, len(rows))
    for r in rows:
        assert len(r.rstrip(",").split(",")) == 64, (t, r[:40])

src = open(ATLAS).read()
skips = []

# --- 1. the four tile masters ---
for t in (1070, 1071, 1072, 1073):
    decl = f"static TILE_{t}_RGBA: [u8; 1024] = ["
    if decl in src:
        skips.append(f"TILE_{t}_RGBA pinned")
        continue
    body = "\n".join(tiles[t])
    anchor = f"static TILE_{t-1}_RGBA: [u8; 1024] = ["
    i = src.index(anchor)
    j = src.index("];", i) + 3
    block = decl + "\n" + body + "\n];\n"
    src = src[:j] + block + src[j:]
    print(f"TILE_{t}_RGBA appended")

# --- 2. the TILES array tail ---
if "&TILE_1073_RGBA,\n];" in src:
    skips.append("TILES tail pinned")
else:
    src = src.replace(
        "    &TILE_1069_RGBA,\n];",
        "    &TILE_1069_RGBA,\n"
        "    &TILE_1070_RGBA,\n"
        "    &TILE_1071_RGBA,\n"
        "    &TILE_1072_RGBA,\n"
        "    &TILE_1073_RGBA,\n];",
        1,
    )
    src = src.replace("pub static TILES: [&[u8]; 1070] = [", "pub static TILES: [&[u8]; 1074] = [", 1)
    print("TILES tail extended 1070 -> 1074")

# --- 3. the TILE_NAMES tail ---
NAMES = ["enchanting_table_book_lid_left", "enchanting_table_book_lid_right",
         "enchanting_table_book_pages_left", "enchanting_table_book_pages_right"]
if '"enchanting_table_book_pages_right",' in src:
    skips.append("TILE_NAMES tail pinned")
else:
    src = src.replace(
        '    "enchanting_table_side",\n];',
        '    "enchanting_table_side",\n'
        '    "enchanting_table_book_lid_left",\n'
        '    "enchanting_table_book_lid_right",\n'
        '    "enchanting_table_book_pages_left",\n'
        '    "enchanting_table_book_pages_right",\n];',
        1,
    )
    src = src.replace("pub static TILE_NAMES: [&str; 1070] = [", "pub static TILE_NAMES: [&str; 1074] = [", 1)
    print("TILE_NAMES tail extended 1070 -> 1074")

# --- 4. the named consts ---
if "pub const TILE_ENCHANTING_TABLE_BOOK_PAGES_RIGHT: u16 = 1073;" in src:
    skips.append("named consts pinned")
else:
    src = src.replace(
        "pub const TILE_ENCHANTING_TABLE_SIDE: u16 = 1069;\n",
        "pub const TILE_ENCHANTING_TABLE_SIDE: u16 = 1069;\n"
        "/// THE ENCHANTING TABLE'S OWN BOOK TILES (the pool tail's four\n"
        "/// the mesher's book-float lane the bakes' crops serve — the\n"
        "/// jar's entity/enchantment/enchanting_table_book.png the 64x32\n"
        "/// master wears, the four 16x16 windows the face rects keep).\n"
        "pub const TILE_ENCHANTING_TABLE_BOOK_LID_LEFT: u16 = 1070;\n"
        "pub const TILE_ENCHANTING_TABLE_BOOK_LID_RIGHT: u16 = 1071;\n"
        "pub const TILE_ENCHANTING_TABLE_BOOK_PAGES_LEFT: u16 = 1072;\n"
        "pub const TILE_ENCHANTING_TABLE_BOOK_PAGES_RIGHT: u16 = 1073;\n",
        1,
    )
    print("named consts appended")

open(ATLAS, "w").write(src)
print("SKIPS:", skips if skips else "none")
print("PIN COMPLETE")
