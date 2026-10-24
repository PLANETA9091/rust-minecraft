#!/usr/bin/env python3
"""THE CONTROLS PROBE'S OWN PART C (the autopush-185 round): the
KeyBindsScreen's own contents/footer/click walk, the KeyBindsList's
rows, the OptionsSubScreen's footer law, and the lang table's own rows
(controls.title / controls.keybinds / controls.keybinds.title /
controls.resetAll / options.mouse_settings / key.categories.*) the
client's captions wear.

Usage: python3 scripts/jar_work/controls_185c.py <client.jar>
"""
import json
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
helper_src = src.split("z = zipfile.ZipFile(JAR)")[0]
ns = {}
exec(helper_src, ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

KEYS = "net.minecraft.client.gui.screens.options.controls.KeyBindsScreen"
LIST = "net.minecraft.client.gui.screens.options.controls.KeyBindsList"
LIST_KEY = "net.minecraft.client.gui.screens.options.controls.KeyBindsList$KeyEntry"
LIST_CAT = "net.minecraft.client.gui.screens.options.controls.KeyBindsList$CategoryEntry"
SUB = "net.minecraft.client.gui.screens.options.OptionsSubScreen"

print("########## A. KeyBindsScreen addContents/addOptions/addFooter + lambdas")
dump_method(z, KEYS, "addContents")
dump_method(z, KEYS, "addOptions")
dump_method(z, KEYS, "addFooter")
dump_method(z, KEYS, "lambda$addFooter$0")
dump_method(z, KEYS, "lambda$addFooter$1")
dump_method(z, KEYS, "mouseClicked")
dump_method(z, KEYS, "repositionElements")

print()
print("########## B. OptionsSubScreen addFooter + resetOption (the base law)")
dump_method(z, SUB, "addFooter")
dump_method(z, SUB, "resetOption")

print()
print("########## C. KeyBindsList methods + utf8 words")
list_methods(z, LIST)
class_utf8_words(z, LIST, ["key", "category", "none", "> <", "conflict"])

print()
print("########## D. KeyBindsList ctor (the rows the list builds)")
dump_method(z, LIST, "<init>", max_pc=200)

print()
print("########## E. KeyEntry walk (the row's own click/key)")
list_methods(z, LIST_KEY)
dump_method(z, LIST_KEY, "<init>", max_pc=200)

print()
print("########## F. lang rows (en_us.json)")
try:
    lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
except KeyError:
    # nested path fallback
    cands = [n for n in z.namelist() if n.endswith("en_us.json")]
    print("candidates:", cands[:5])
    lang = json.loads(z.read(cands[0]))
needles = ["controls.title", "controls.keybinds", "controls.reset",
           "options.mouse_settings", "options.controls", "key.categories",
           "key.keyboard", "key.mouse", "options.narrator", "gui.done",
           "key.forward", "key.left", "key.back", "key.right", "key.jump",
           "key.sneak", "key.sprint", "key.inventory", "key.drop",
           "key.chat", "key.use", "key.attack", "key.pickItem"]
for k in sorted(lang):
    if any(k.startswith(n) or k == n for n in needles):
        print(f"  {k} = {lang[k]!r}")
