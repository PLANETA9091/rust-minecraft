#!/usr/bin/env python3
"""brewing_walk_338.py — the autopush-338 round's own jar probe (the 26.2 client.jar).

The BREWING STAND'S OWN CHEMISTRY WALK the evidence banks:
  1. BrewingStandBlockEntity — the serverTick walk (the fuel/blend/finish arms),
     the doBrew chemistry, the isBrewable gate, the canPlaceItem place lanes,
     the fields (brewTime/totalBrewTime/fuel) the data slots speak.
  2. BrewingStandMenu — the ContainerData lane (the 2 slots) + the accessors.
  3. BrewingStandScreen.renderBg — the flame + bubbles + arrow draws.
  4. PotionBrewing — the mix tables + isIngredient + the mix() walk.

javap the JDK-25 lane serves (the 338 round the wiped sandbox re-fetches —
/home/z/my-project/jdk-25.0.5+8/bin/javap).
"""
import re
import subprocess
import sys

JDK = "/home/z/my-project/jdk-25.0.5+8/bin/javap"
JAR = ".mc2rs/classpath/client.jar"
OUT = "scripts/jar_work"

CP = JAR
COMMON = ["-private", "-constants"]


def dump(name, out, extra=None):
    cmd = [JDK, "-cp", CP] + COMMON + (extra or []) + [name]
    r = subprocess.run(cmd, capture_output=True, text=True)
    with open(f"{OUT}/{out}", "w") as f:
        f.write(r.stdout)
        if r.stderr:
            f.write("\n--STDERR--\n" + r.stderr)
    print(out, "rc=", r.returncode, "len=", len(r.stdout))


def method(name, method_sig, out):
    cmd = [JDK, "-cp", CP, "-private", "-c", "-p", name]
    r = subprocess.run(cmd, capture_output=True, text=True)
    src = r.stdout
    # slice: match the method-name-only header (the -c header carries the
    # full generic sig — the name + word-boundary law the slice keeps)
    pat = re.compile(
        r"^  (?:public|private|protected|static|final)[^\n]*\b"
        + re.escape(method_sig)
        + r"\b[^\n]*;$",
        re.M,
    )
    m = pat.search(src)
    if not m:
        with open(f"{OUT}/{out}", "w") as f:
            f.write(src)
        print(out, "MARKER-NOT-FOUND len=", len(src), "rc=", r.returncode)
        return
    i = m.start()
    j = src.find("\n  ", m.end())
    # the next member header (two-space indent) ends the body
    nxt = None
    for mm in re.finditer(r"^  \S.*;$", src[m.end():], re.M):
        nxt = m.end() + mm.start()
        break
    if nxt and nxt > i:
        j = nxt
    body = src[i: j if j > 0 else len(src)]
    with open(f"{OUT}/{out}", "w") as f:
        f.write(body)
    print(out, "rc=", r.returncode, "bodylen=", len(body))


BE = "net.minecraft.world.level.block.entity.BrewingStandBlockEntity"
MENU = "net.minecraft.world.inventory.BrewingStandMenu"
SCREEN = "net.minecraft.client.gui.screens.inventory.BrewingStandScreen"
PB = "net.minecraft.world.item.alchemy.PotionBrewing"
PBS = "net.minecraft.world.item.alchemy.PotionBrewing$Mix"

dump(BE, "brewbe_338.txt")
method(BE, "serverTick", "brew_tick_338.txt")
method(BE, "isBrewable", "brew_isbrewable_338.txt")
method(BE, "doBrew", "brew_dobrew_338.txt")
method(BE, "canPlaceItem", "brew_canplace_338.txt")
method(BE, "canTakeItemThroughFace", "brew_cantake_338.txt")
method(BE, "loadAdditional", "brew_load_338.txt")
method(BE, "saveAdditional", "brew_save_338.txt")
method(BE, "getUpdatePacket", "brew_update_338.txt")
method(BE, "getUpdateTag", "brew_updatetag_338.txt")
method(BE, "clientTick", "brew_clienttick_338.txt")

dump(MENU, "brewmenu_338.txt")
method(MENU, "BrewingStandMenu", "brew_menu_ctor_338.txt")
method(MENU, "getBrewingTicks", "brew_menu_ticks_338.txt")
method(MENU, "getFuel", "brew_menu_fuel_338.txt")

dump(SCREEN, "brewscreen_338.txt")
method(SCREEN, "extractBackground", "brew_screenbg_338.txt")

dump(PB, "potionbrewing_338.txt")
method(PB, "isIngredient", "pb_isingredient_338.txt")
method(PB, "isContainerItem", "pb_iscontainer_338.txt")
method(PB, "isContainerIngredient", "pb_iscontainering_338.txt")
method(PB, "mix", "pb_mix_338.txt")
method(PB, "hasMix", "pb_hasmix_338.txt")
dump(PBS, "potionmix_338.txt")
method(PBS, "isInput", "pmix_isinput_338.txt")
method(PBS, "isIngredient", "pmix_isingredient_338.txt")

print("done")
