#!/usr/bin/env python3
"""jar_probe_potions_339.py — the autopush-339 round's own jar probe.

The POTION-KIND'S OWN REGISTRY the evidence banks:
  1. Potions.<clinit> — the registration order IS the potion registry
     id order (the holder identity the Mix.from comparison speaks).
  2. PotionBrewing$Builder — the addMix/addStartMix/addContainerRecipe
     semantics (the startMix lane the water-from law answers).
  3. PotionBrewing.clinit — the ALLOWED_CONTAINERS the container lane
     keeps + the build order.
"""
import subprocess

JDK = "/home/z/my-project/jdk-25.0.5+8/bin/javap"
JAR = ".mc2rs/classpath/client.jar"
OUT = "scripts/jar_work"

def dump(name, out):
    cmd = [JDK, "-cp", JAR, "-private", "-constants", "-c", name]
    r = subprocess.run(cmd, capture_output=True, text=True)
    with open(f"{OUT}/{out}", "w") as f:
        f.write(r.stdout)
    print(out, "rc=", r.returncode, "lines=", len(r.stdout.splitlines()))

dump("net.minecraft.world.item.alchemy.Potions", "potions_clinit_339.txt")
dump("net.minecraft.world.item.alchemy.PotionBrewing$Builder", "builder_339.txt")
