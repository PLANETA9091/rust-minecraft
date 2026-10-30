#!/bin/bash
# THE JAR LAW probe for autopush-340 (the potion-kind's own tooltip + tint round):
# the PotionContents getName/getColor walks + the Potion color blend the 26.2 jar serves.
set -u
JAR=.mc2rs/classpath/client.jar
OUT=scripts/jar_work
mkdir -p "$OUT"
JAVAP=/home/z/my-project/jdk-25.0.5+8/bin/javap
[ -x "$JAVAP" ] || JAVAP=javap

"$JAVAP" -c -p -cp "$JAR" net.minecraft.world.item.alchemy.PotionContents > "$OUT"/potion_contents_340.txt 2>&1
echo "potion_contents rc=$?"
"$JAVAP" -c -p -cp "$JAR" net.minecraft.world.item.alchemy.Potion > "$OUT"/potion_class_340.txt 2>&1
echo "potion_class rc=$?"
"$JAVAP" -c -p -cp "$JAR" net.minecraft.core.component.DataComponents > "$OUT"/data_components_340.txt 2>&1
echo "data_components rc=$?"
grep -n "POTION_CONTENTS\|POTION" "$OUT"/data_components_340.txt | head -8
