#!/usr/bin/env python3
"""QA probe for the dev-79 creative golden (the picker's own canvas pin).

Renders the staged creative scene and verifies the dev-30 screen's own
pixel work on the 195x136 panel:
  1. THE PANEL'S OWN CANVAS — the (139,139,139) family the blitted
     background wears in the centered seat,
  2. THE PALETTE'S OWN FILL — the 45-slot grid the building-blocks tab
     speaks (the item mass the display list wears, the empty tail the
     slot-bg count keeps),
  3. THE TAB STRIP'S TWO ROWS — the colored icons the 14 tabs carry
     (the TOP row the building-blocks..search seats speak, the BOTTOM
     row the tools..inventory tail wears),
  4. THE HOTBAR ROW — the demo loadout the store paints (the count
     digits the white shadow text wears, the item colors the six
     stacks speak),
  5. THE SCROLLER — the 12x15 handle's strip the right edge keeps (the
     dark rails + the light track the widget's own colors wear),
  6. THE BYTE DETERMINISM — the scene renders twice, the PNGs agree.
"""
import subprocess
import sys

from PIL import Image

ROOT = "/home/z/my-project/rust-minecraft"
CLIENT = f"{ROOT}/target/release/mc-client"
WORK = f"{ROOT}/target/golden_verify"

FLAGS = ["--screen", "creative", "--ui-frame", "30", "--seed", "20260214",
         "--width", "320", "--height", "180"]

def render(out):
    cmd = [CLIENT, "--headless", *FLAGS, "--out", out]
    subprocess.run(cmd, cwd=ROOT, check=True, capture_output=True)

render(f"{WORK}/qa_creative_a.png")
render(f"{WORK}/qa_creative_b.png")

a = open(f"{WORK}/qa_creative_a.png", "rb").read()
b = open(f"{WORK}/qa_creative_b.png", "rb").read()
fails = []
if a != b:
    fails.append("the scene's own double render disagrees (determinism)")

img = Image.open(f"{WORK}/qa_creative_a.png").convert("RGB")
px = img.load()
W, H = img.size
if (W, H) != (320, 180):
    fails.append(f"the frame's own size ({W}x{H})")

def is_panel(p):
    r, g, b = p
    return abs(r - g) <= 3 and abs(g - b) <= 3 and 130 <= r <= 150

def is_slotbg(p):
    r, g, b = p
    return abs(r - g) <= 2 and abs(g - b) <= 2 and 190 <= r <= 205

def is_colored(p):
    r, g, b = p
    return max(r, g, b) - min(r, g, b) >= 30

def is_white(p):
    r, g, b = p
    return r >= 250 and g >= 250 and b >= 250

def is_strip_dark(p):
    r, g, b = p
    return abs(r - 85) <= 6 and abs(g - 85) <= 6 and abs(b - 85) <= 6

def is_track_light(p):
    r, g, b = p
    return abs(r - 198) <= 3 and abs(g - 198) <= 3 and abs(b - 198) <= 3

# 1. THE PANEL'S OWN CANVAS (the centered seat the (320-195)/2,
#    (180-136)/2 walk keeps: x 62..257, y 22..158)
panel = sum(1 for y in range(20, 158) for x in range(62, 258) if is_panel(px[x, y]))
print(f"panel px = {panel}")
if panel < 8000:
    fails.append(f"the panel's own canvas mass ({panel} < 8000)")

# 2. THE PALETTE'S OWN FILL (the 45-slot grid the building-blocks tab
#    speaks: gui (9 + col*18, 18 + row*18) the ItemPickerMenu ctor
#    wears — screen x 71..242, y 40..126)
pal_items = sum(1 for y in range(40, 126) for x in range(71, 242)
                if not is_panel(px[x, y]) and not is_slotbg(px[x, y]))
pal_empty = sum(1 for y in range(40, 126) for x in range(71, 242)
                if is_slotbg(px[x, y]))
print(f"palette item px = {pal_items}, slot-bg px = {pal_empty}")
if pal_items < 7000:
    fails.append(f"the palette's own item mass ({pal_items} < 7000)")
if pal_empty > 800:
    fails.append(f"the palette's empty tail speaks ({pal_empty} > 800)")

# 3. THE TAB STRIP'S TWO ROWS (the 27px pitch the 14 tabs keep — the
#    TOP row's overhang y 0..20, the BOTTOM row's y 156..180)
top_col = sum(1 for y in range(0, 20) for x in range(62, 258)
              if is_colored(px[x, y]))
bot_col = sum(1 for y in range(156, 180) for x in range(62, 258)
              if is_colored(px[x, y]))
print(f"top tabs colored = {top_col}, bottom tabs colored = {bot_col}")
if top_col < 700:
    fails.append(f"the TOP tab row's icons ({top_col} < 700)")
if bot_col < 400:
    fails.append(f"the BOTTOM tab row's icons ({bot_col} < 400)")

# 4. THE HOTBAR ROW (the addInventoryHotbarSlots(9, 112) walk the
#    store paints: the count digits the shadow text wears, the six
#    demo stacks the colors speak)
hot_white = sum(1 for y in range(134, 153) for x in range(62, 258)
                if is_white(px[x, y]))
hot_col = sum(1 for y in range(128, 156) for x in range(62, 258)
              if is_colored(px[x, y]))
print(f"hotbar white = {hot_white}, colored = {hot_col}")
if hot_white < 150:
    fails.append(f"the hotbar row's count digits ({hot_white} < 150)")
if hot_col < 400:
    fails.append(f"the hotbar row's item colors ({hot_col} < 400)")

# 5. THE SCROLLER (the 12x15 handle the (175, 18 + 77*scroll) seat
#    wears — the strip's dark rails + the light track the right edge
#    keeps: screen x 237..258, y 30..130)
strip_dark = sum(1 for y in range(30, 130) for x in range(237, 258)
                 if is_strip_dark(px[x, y]))
track = sum(1 for y in range(30, 130) for x in range(237, 258)
            if is_track_light(px[x, y]))
print(f"scroller dark = {strip_dark}, track = {track}")
if strip_dark < 150:
    fails.append(f"the scroller's own rails ({strip_dark} < 150)")
if track < 300:
    fails.append(f"the scroller's own track ({track} < 300)")

if fails:
    print("QA FAIL:")
    for f in fails:
        print(" -", f)
    sys.exit(1)
print("QA PASS: the panel's canvas, the palette's fill, the two tab rows, "
      "the hotbar row, the scroller all speak")
