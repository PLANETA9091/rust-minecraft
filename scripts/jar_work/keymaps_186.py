#!/usr/bin/env python3
"""THE KEYMAPPINGS' OWN STORE PROBE (the autopush-186 round): the
Options.save walk the key_ rows the file ladder banks — the save()'s
own body (the keyMappings loop the write order keeps), the
KeyMapping class (the name/category/defaultKey fields the ctor wears),
the KeyMapping.Category enum (the translation keys the rows speak),
the KeyBindsList$KeyEntry (the row's own walk — the widgets the row
hosts), and the lang rows the movement keys answer.

Usage: python3 scripts/jar_work/keymaps_186.py <client.jar>
"""
import json
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPTS = "net.minecraft.client.Options"
KM = "net.minecraft.client.KeyMapping"
CAT = "net.minecraft.client.KeyMapping$Category"
KEYENTRY = "net.minecraft.client.gui.screens.options.controls.KeyBindsList$KeyEntry"

print("########## A. Options.save (the key_ rows the ladder wears)")
dump_method(z, OPTS, "save()V", max_pc=600)

print()
print("########## B. KeyMapping fields + methods")
list_methods(z, KM)
class_utf8_words(z, KM, ["key_", "keyboard", "mouse", "category", "name",
                         "down", "conflict", "anonymous"])

print()
print("########## C. KeyMapping ctor + setKey + getDefaultKey + isDown")
dump_method(z, KM, "<init>")
dump_method(z, KM, "setKey")
dump_method(z, KM, "getDefaultKey")
dump_method(z, KM, "isDown()")

print()
print("########## D. Category enum (the translation keys)")
try:
    list_methods(z, CAT)
    class_utf8_words(z, CAT, ["key.categories", "movement", "gameplay"])
except Exception as e:
    print("!!", e)

print()
print("########## E. KeyEntry (the row's own walk)")
list_methods(z, KEYENTRY)
dump_method(z, KEYENTRY, "<init>", max_pc=300)
dump_method(z, KEYENTRY, "reset")

print()
print("########## F. Options clinit — the keyMappings array's ctor walk (the first rows)")
dump_method(z, OPTS, "new KeyMapping", max_pc=10)
class_utf8_words(z, OPTS, ["key_key.forward", "key_key.jump", "key_key.sprint",
                           "key_key.sneak", "key_key.inventory", "key_key.chat",
                           "key_key.drop", "key_key.hotbar", "key_key.togglePerspective"])

print()
print("########## G. lang rows (the movement keys + the keyboard names)")
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if k in ("key.forward", "key.back", "key.left", "key.right", "key.jump",
             "key.sneak", "key.sprint", "key.inventory", "key.drop", "key.chat",
             "key.hotbar.1", "key.hotbar.9", "key.swapOffhand",
             "key.keyboard.w", "key.keyboard.s", "key.keyboard.a", "key.keyboard.d",
             "key.keyboard.space", "key.keyboard.left.shift", "key.keyboard.left.control",
             "key.keyboard.unknown", "controls.reset", "key.categories.movement"):
        print(f"  {k} = {lang[k]!r}")
