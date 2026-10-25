#!/usr/bin/env python3
"""THE OPTIONS-TAIL PROBE'S OWN WALK (the autopush-218 round): the
processOptions visit ladder's own seats the operatorItemsTab row wears
(the FieldAccess.process call pc the save row's order keeps), the
ControlsScreen OPTIONS array's tail the (4,5) pair + the index-6 the
addSmall law speaks, the sprintWindow caption lambda's own words the
indy bootstrap answers, the operatorItemsTab's caption lane the
createBoolean generic pair keeps.

Usage: python3 scripts/jar_work/opttail_218.py <client.jar>
"""
import sys
import zipfile

# the parse helpers the earlier probes share (controls_185.py's own head)
src = open("scripts/jar_work/controls_185.py").read()
helper_src = src.split("z = zipfile.ZipFile(JAR)")[0]
ns = {}
exec(helper_src, ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPT = "net.minecraft.client.Options"
CS = "net.minecraft.client.gui.screens.options.controls.ControlsScreen"

print("########## A. Options.processOptions (the visit ladder's own pcs)")
dump_method(z, OPT, "processOptions")

print("########## B. the sprintWindow caption lambda words")
# the Options <init> constant pool: the indy refs the lambda names speak
data = z.read("net/minecraft/client/Options.class")
cp, _ = parse_cp(data, 0)
# find the methodref/c string rows near the sprintWindow indy
words = class_utf8_words(z, "net.minecraft.client.Options")
hits = [w for w in words if "sprint" in w.lower() or "operator" in w.lower()]
for w in hits:
    print("  utf8:", w)

print("########## C. Options method census (the accessors the rows wear)")
for m in list_methods(z, OPT):
    if "print" in m or "Sprint" in m or "perator" in m:
        print("  ", m)
