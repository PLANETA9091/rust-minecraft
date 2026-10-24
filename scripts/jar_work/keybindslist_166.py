#!/usr/bin/env python3
"""THE KEYBINDS LIST'S OWN WALK (dev-166): the ctor the
`Arrays.sort` the compareTo law answers — the within-category order
the orderI/i18n the list wears, or the array order the rows keep.

Usage: python3 scripts/jar_work/keybindslist_166.py <client.jar>
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CLS = "net.minecraft.client.gui.screens.options.controls.KeyBindsList"
d = dict(methods_of(z, CLS))
for sig in ("<init>(Lnet/minecraft/client/gui/screens/options/controls/KeyBindsScreen;Lnet/minecraft/client/Minecraft;)V", "refreshEntries()V"):
    cp, body = d[sig]
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8 : 8 + clen]
    print(f"########## {sig}")
    for t in walk_code(cp, code):
        print(f"  {t}")
    print()
