#!/usr/bin/env python3
"""autopush-304 — THE SHULKER'S OWN FACING LANE scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - ShulkerBoxBlock.getStateForPlacement the FACING stamp walk,
  - ShulkerBoxRenderer.submit/render the pose rotation the facing
    direction wears (the modelTransform's own rotation law),
  - the FACING property the ShulkerBoxBlock's own state definition.

Exit 0 iff the walks parse.
"""
import sys, zipfile, struct

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm, R

JAR = '.mc2rs/classpath/client.jar'

def find_class(zf, name):
    for cand in (f'net/minecraft/{name}', f'net/minecraft/client/{name}',
                 f'net/minecraft/world/level/block/{name}'):
        try:
            return zf.read(cand + '.class')
        except KeyError:
            continue
    return None

def walk(zf, path):
    try:
        return zf.read(path)
    except KeyError:
        return None

def disasm_walk(zf, name, data):
    """Parse + disasm the methods the facing walk needs."""
    want_names = {'getStateForPlacement', 'render', 'submit', 'extractRenderState',
                  '<clinit>', 'createBlockStateDefinition', 'rotate', 'mirror',
                  'getRenderLocation', 'modelTransform', 'setupAnim'}
    for mname, desc, code, cp in parse_class(data, want_names):
        print(f"== {name} :: {mname} {desc} ==")
        if code is None:
            print("  (abstract)")
        else:
            print(disasm(code, cp))
        print()
    return 0

def main():
    zf = zipfile.ZipFile(JAR)
    names = zf.namelist()
    hits = [n for n in names if 'ShulkerBox' in n or 'Shulker' in n]
    print("== Shulker classes ==")
    for n in sorted(hits)[:30]:
        print(" ", n)
    # ShulkerBoxBlock — the server jar may hold it; try both jars
    for jar in (JAR, JAR.replace('client.jar', 'server.jar')):
        try:
            zf2 = zipfile.ZipFile(jar)
        except Exception:
            continue
        for n in zf2.namelist():
            if n.endswith('ShulkerBoxBlock.class') or n.endswith('ShulkerBoxRenderer.class'):
                data = zf2.read(n)
                # want = the method names the walk answers
                out_methods = disasm_walk(zf2, n, data)
        zf2.close()
    return 0

if __name__ == '__main__':
    sys.exit(main())
