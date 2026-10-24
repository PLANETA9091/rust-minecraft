#!/usr/bin/env python3
"""THE CONTROLS PROBE'S OWN PART B (the autopush-185 round): the
ControlsScreen.addOptions body + the two button lambdas, the
KeyBindsScreen's full walk (the ctor/init/removed/the list/the reset),
the KeyBindsList's entry walk, and the OptionsSubScreen base's own
removed/onClose law the lane inherits.

Usage: python3 scripts/jar_work/controls_185b.py <client.jar>
"""
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
import importlib.util

spec = importlib.util.spec_from_file_location("c185", "scripts/jar_work/controls_185.py")
# reuse helpers without re-running the module body: copy the needed fns
import struct

MNEMONIC = __import__("controls_185").MNEMONIC if False else None
# simpler: exec the helper defs from controls_185 up to the zipfile use
src = open("scripts/jar_work/controls_185.py").read()
helper_src = src.split("z = zipfile.ZipFile(JAR)")[0]
ns = {}
exec(helper_src, ns)

parse_cp = ns["parse_cp"]; utf = ns["utf"]; walk_code = ns["walk_code"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CTRL = "net.minecraft.client.gui.screens.options.controls.ControlsScreen"
KEYS = "net.minecraft.client.gui.screens.options.controls.KeyBindsScreen"
LIST = "net.minecraft.client.gui.screens.options.controls.KeyBindsList"
LIST_CAT = "net.minecraft.client.gui.screens.options.controls.KeyBindsList$CategoryEntry"
LIST_KEY = "net.minecraft.client.gui.screens.options.controls.KeyBindsList$KeyEntry"
SUB = "net.minecraft.client.gui.screens.options.OptionsSubScreen"

print("########## A. ControlsScreen.addOptions + the two lambdas")
dump_method(z, CTRL, "addOptions")
dump_method(z, CTRL, "lambda$addOptions$0")
dump_method(z, CTRL, "lambda$addOptions$1")

print()
print("########## B. ControlsScreen options(Lnet/minecraft/client/Options;) static")
dump_method(z, CTRL, "options(Lnet")

print()
print("########## C. OptionsSubScreen base (the removed/onClose law the lane inherits)")
list_methods(z, SUB)
dump_method(z, SUB, "removed")
dump_method(z, SUB, "onClose")
dump_method(z, SUB, "init")
dump_method(z, SUB, "renderBackground")

print()
print("########## D. KeyBindsScreen methods + utf8 words")
list_methods(z, KEYS)
class_utf8_words(z, KEYS, ["controls", "key", "reset", "done", "keyboard",
                           "mouse", "category", "conflict", "none", "press"])

print()
print("########## E. KeyBindsScreen ctor + init + removed + keyPressed")
dump_method(z, KEYS, "<init>")
dump_method(z, KEYS, "init")
dump_method(z, KEYS, "removed")
dump_method(z, KEYS, "onClose")
dump_method(z, KEYS, "keyPressed")
dump_method(z, KEYS, "rebind")
