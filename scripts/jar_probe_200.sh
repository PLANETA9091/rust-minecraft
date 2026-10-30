#!/bin/bash
# THE JAR LAW probe for dev-200 (the potion-kind's own attribute rows round):
# the MobEffect.createModifiers walk + the MobEffects clinit registrations
# + the ItemAttributeModifiers DecimalFormat + the lang keys the 26.2 jar serves.
set -u
JAR=.mc2rs/classpath/client.jar
OUT=scripts/jar_work
mkdir -p "$OUT"
JAVAP=/home/z/my-project/jdk-25.0.5+8/bin/javap
[ -x "$JAVAP" ] || JAVAP=javap

"$JAVAP" -c -p -cp "$JAR" net.minecraft.world.effect.MobEffect > "$OUT"/mob_effect_200.txt 2>&1
echo "mob_effect rc=$?"
"$JAVAP" -c -p -cp "$JAR" net.minecraft.world.effect.MobEffects > "$OUT"/mob_effects_200.txt 2>&1
echo "mob_effects rc=$?"
"$JAVAP" -c -p -cp "$JAR" net.minecraft.world.item.component.ItemAttributeModifiers > "$OUT"/item_attr_mods_200.txt 2>&1
echo "item_attr_mods rc=$?"
# the nested AttributeTemplate / AttributeModifiers classes the walk may ride
"$JAVAP" -c -p -cp "$JAR" 'net.minecraft.world.effect.MobEffect$AttributeTemplate' > "$OUT"/mob_effect_template_200.txt 2>&1
echo "mob_effect_template rc=$?"
"$JAVAP" -c -p -cp "$JAR" 'net.minecraft.world.effect.MobEffect$AttributeModifiers' > "$OUT"/mob_effect_attrmods_200.txt 2>&1
echo "mob_effect_attrmods rc=$?"
# the lang keys the attribute rows wear
unzip -p "$JAR" assets/minecraft/lang/en_us.json > "$OUT"/en_us_200.json 2>/dev/null
echo "en_us rc=$?"
grep -o '"potion.whenDrank"[^,]*' "$OUT"/en_us_200.json | head -2
grep -o '"attribute.modifier.plus.[0-9]"[^,]*' "$OUT"/en_us_200.json | head -4
grep -o '"attribute.modifier.take.[0-9]"[^,]*' "$OUT"/en_us_200.json | head -4
