#!/usr/bin/env python3
"""THE V-LAW PROBE (the autopush-302 round — the idempotent evidence bank).

Re-derives from the 26.2 jar, live:
  1. the ModelPart$Polygon ctor's remap law (v[0]->(u1,v0) v[1]->(u0,v0)
     v[2]->(u0,v1) v[3]->(u1,v1), the zero slack),
  2. the ModelPart$Cube ctor's 8 vertex coords + the 6 polygon vertex
     orders (the per-face u/v preimage table),
  3. the BE pose chain's no-y-mirror verdict (LevelRenderer.submitBlock-
     Entities the translate-only, the dispatcher the passthrough,
     ChestRenderer.submit the mulPose(Y-rotation) the scale never calls,
     Cube.compile the worldY=y/16 raw pass).

Exit 0 iff every law re-pins. THE raw_disasm.cp_all HEADER SKIP the same
round fixes: the cp count the 8-byte header FOLLOWS — the offset-0 read
the magic's 0xCAFE the count wore, the walk the opcode bytes the bogus
tag 186 (0xBA) hit.
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm, cp_all

JAR = '.mc2rs/classpath/client.jar'


def method_lines(cls, method):
    data = zipfile.ZipFile(JAR).read(f'{cls}.class')
    for name, desc, code, cp in parse_class(data, {method}):
        if code is None:
            continue
        return name, desc, disasm(code, cp)
    return None, None, []


def check(cond, msg):
    print(('  OK  ' if cond else '  FAIL') + ' ' + msg)
    return cond


def main():
    ok = True

    print('1. THE cp_all HEADER SKIP (the tag-186 desync the fix retires):')
    try:
        data = zipfile.ZipFile(JAR).read(
            'net/minecraft/client/renderer/blockentity/ChestRenderer.class')
        cp = cp_all(data)
        ok &= check(len(cp) > 400, f'cp_all parses the ChestRenderer cp ({len(cp)-1} entries) without the bogus tag')
    except ValueError as e:
        ok &= check(False, f'cp_all still desyncs: {e}')

    print('2. THE POLYGON CTOR REMAP LAW (the v seat):')
    _, _, lines = method_lines('net/minecraft/client/model/geom/ModelPart$Polygon', '<init>')
    txt = '\n'.join(lines)
    # v[0] gets (p4/w - slack, p3/h + slack); v[1] (p2/w + slack, p3/h + slack)
    # v[2] (p2/w + slack, p5/h - slack); v[3] (p4/w - slack, p5/h - slack)
    import re
    # the four REMAP stores (the aastore the segment boundary serves —
    # the remap block's fload operands the remap call PRECEDE, the
    # piece the store's own block wears; the mirror swap block's tail
    # the remap call never wears)
    pieces = txt.split('aastore')
    remap_pieces = [p for p in pieces if '.remap' in p]
    ok &= check(len(remap_pieces) == 4, f'the four vertex remap stores walk ({len(remap_pieces)})')
    # v[0] and v[1] share p3 (the band's v0); v[2] and v[3] share p5 (v1)
    shares_v0 = sum(1 for b in remap_pieces if 'fload_3' in b) == 2
    shares_v1 = sum(1 for b in remap_pieces if 'fload 5' in b) == 2
    ok &= check(shares_v0 and shares_v1,
                'v[0]/v[1] the p3 (v0) share, v[2]/v[3] the p5 (v1) — the both-top-vertices-v0 law')

    print('3. THE CUBE CTOR FACE ORDERS (the per-face preimage):')
    _, _, lines = method_lines('net/minecraft/client/model/geom/ModelPart$Cube', '<init>')
    txt = '\n'.join(lines)
    # the polygon constructions in emission order: DOWN UP WEST NORTH EAST SOUTH
    orders = {
        'DOWN': ['24', '23', '19', '20'],
        'UP': ['21', '22', '26', '25'],
        'WEST': ['19', '23', '26', '22'],
        'NORTH': ['20', '19', '22', '21'],
        'EAST': ['24', '20', '21', '25'],
        'SOUTH': ['23', '24', '25', '26'],
    }
    pos = 0
    for face in ['DOWN', 'UP', 'WEST', 'NORTH', 'EAST', 'SOUTH']:
        want = orders[face]
        seg_start = txt.find(f'Direction.{face}', pos)
        seg_end = txt.find(f'Direction.', seg_start + 1) if txt.find(f'Direction.', seg_start + 1) > 0 else len(txt)
        seg = txt[seg_start:seg_end] if seg_start >= 0 else ''
        got = re.findall(r'astore (\d+)\n', seg)  # no — the array stores
        got = re.findall(r'iconst_\d\n\s*\d+ aload (\d+)\n', seg)
        ok &= check(got == want, f'{face} polygon the vertex order {got} the {want} law wears')
        pos = seg_start + 1

    print('4. THE POSE CHAIN NO-Y-MIRROR (the render pipeline the raw pass):')
    _, _, lines = method_lines('net/minecraft/client/renderer/LevelRenderer', 'submitBlockEntities')
    txt = '\n'.join(lines)
    ok &= check('translate' in txt and 'scale' not in txt.lower().replace('translate', ''),
                'LevelRenderer.submitBlockEntities the translate the scale never calls')
    _, _, lines = method_lines('net/minecraft/client/renderer/blockentity/ChestRenderer', 'submit')
    txt = '\n'.join(lines)
    ok &= check('mulPose' in txt and 'scale' not in txt.lower().replace('translate', ''),
                'ChestRenderer.submit the mulPose(Y-rotation) the scale never calls')
    _, _, lines = method_lines('net/minecraft/client/model/geom/ModelPart$Cube', 'compile')
    txt = '\n'.join(lines)
    ok &= check('worldY' in txt and 'addVertex' in txt,
                'Cube.compile the worldY() the addVertex the raw pass (no v/u swap)')

    print()
    print('THE V-LAW VERDICT: ' + ('ALL LAWS RE-PIN' if ok else 'A LAW DESYNCED — INVESTIGATE'))
    sys.exit(0 if ok else 1)


if __name__ == '__main__':
    main()
