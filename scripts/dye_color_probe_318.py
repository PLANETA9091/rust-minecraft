#!/usr/bin/env python3
"""autopush-318 probe — the 16 DyeColor rows' textureDiffuseColor the
clinit's raw CP ints resolve (the beacon glass ladder the sections
walk colors). Exit 0 iff the 16 rows print.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'

zf = zipfile.ZipFile(JAR)
data = zf.read('net/minecraft/world/item/DyeColor.class')
cp, _ = parse_cp(data)

# resolve cp int/float/string constants
def cp_const(idx):
    tag = cp[idx][0] if isinstance(cp[idx], tuple) else None
    return cp[idx]

# first: the fields + the ctor's field stores
for mname, mdesc, mcode, mcp in parse_class(data, None):
    if mname == '<init>':
        print("--- ctor")
        for ln in disasm(mcode, mcp):
            print(ln)
        break
print()
for mname, mdesc, mcode, mcp in parse_class(data, None):
    if mname == 'getTextureDiffuseColor':
        print("--- getTextureDiffuseColor")
        for ln in disasm(mcode, mcp):
            print(ln)
