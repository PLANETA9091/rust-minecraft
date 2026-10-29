#!/usr/bin/env python3
"""autopush-323 — list every AbstractContainerMenu method name (the
quickcraft lane's own names the needles keep missing)."""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'
CLS = sys.argv[1] if len(sys.argv) > 1 else 'net/minecraft/world/inventory/AbstractContainerMenu'

zf = zipfile.ZipFile(JAR)
data = zf.read(f'{CLS}.class')
cp, n = parse_cp(data)
i = n + 6
nif = struct.unpack_from('>H', data, i)[0]
i += 2 + 2 * nif
nfields = struct.unpack_from('>H', data, i)[0]
i += 2
for _ in range(nfields):
    fname = utf8(cp, struct.unpack_from('>H', data, i + 2)[0])
    fdesc = utf8(cp, struct.unpack_from('>H', data, i + 4)[0])
    print(f"  field {fname}: {fdesc}")
    i += 6
    attrs = struct.unpack_from('>H', data, i)[0]
    i += 2
    for _ in range(attrs):
        i += 2
        alen = struct.unpack_from('>I', data, i)[0]
        i += 4 + alen
nmethods = struct.unpack_from('>H', data, i)[0]
i += 2
for _ in range(nmethods):
    name = utf8(cp, struct.unpack_from('>H', data, i + 2)[0])
    desc = utf8(cp, struct.unpack_from('>H', data, i + 4)[0])
    print(f"  method {name}: {desc}")
    i += 6
    attrs = struct.unpack_from('>H', data, i)[0]
    i += 2
    for _ in range(attrs):
        i += 2
        alen = struct.unpack_from('>I', data, i)[0]
        i += 4 + alen
