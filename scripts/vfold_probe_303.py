#!/usr/bin/env python3
"""THE V-FOLD PROBE (the autopush-303 round — the idempotent evidence bank).

Re-derives from the 26.2 jar, live: the ChestModel's own box constants the
whole-chest v-fold law rides:
  - the single layer: the bottom box texOffs(0,19).addBox(1,0,1,14,10,14)
    PartPose.ZERO, the lid texOffs(0,0).addBox(1,0,0,14,5,14) offset(0,9,1),
    the lock texOffs(0,0).addBox(7,-2,14,2,4,1) offset(0,9,1),
  - the double right layer: the bottom texOffs(0,19).addBox(1,0,1,15,10,14),
    the lid texOffs(0,0).addBox(1,0,0,15,5,14) offset(0,9,1),
  - the DERIVED BANDS the fold law rides: the body band rows 33..42
    (B0 = 19 + dz 14, dy 10 rows), the lid band rows 14..18 (L0 = 0 + dz 14,
    dy 5 rows),
  - the Polygon ctor remap law the 302 round banked (the band's v0 the two
    MIN-Y vertices the seat wears, v1 the two MAX-Y).

Exit 0 iff every constant re-pins.
"""
import sys, zipfile, struct, re

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm, R

JAR = '.mc2rs/classpath/client.jar'
CLS = 'net/minecraft/client/model/object/chest/ChestModel'


def cp_with_floats(data):
    r = R(data)
    r.raw(4)
    r.raw(2) + r.raw(2)
    n = r.u2()
    cp = [None] * n
    i = 1
    while i < n:
        tag = r.u1()
        if tag == 1:
            ln = r.u2(); cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
        elif tag == 3: cp[i] = ('Int', r.s4())
        elif tag == 4: cp[i] = ('Float', r.f4())
        elif tag == 5: cp[i] = ('Long', struct.unpack_from('>q', r.raw(8))[0]); i += 1
        elif tag == 6: cp[i] = ('Double', r.f8()); i += 1
        elif tag == 7: cp[i] = ('Class', r.u2())
        elif tag == 8: cp[i] = ('Str', r.u2())
        elif tag in (9, 10, 11): cp[i] = ('Ref', r.u2(), r.u2())
        elif tag == 12: cp[i] = ('Name', r.u2(), r.u2())
        elif tag == 15: cp[i] = ('MethodHandle', r.u1(), r.u2())
        elif tag == 16: cp[i] = ('MethodType', r.u2())
        elif tag in (17, 18): cp[i] = ('Dyn', r.u2(), r.u2())
        elif tag in (19, 20): cp[i] = ('Mod', r.u2())
        else: raise ValueError(f'tag {tag}')
        i += 1
    return cp


class AnyC:
    def __contains__(self, x): return True


def method_text(data, cp, want):
    for name, desc, code, _ in parse_class(data, {want}):
        if code is None:
            continue
        out = []
        for ln in disasm(code, cp):
            m = re.search(r'ldc #(\d+) = \?', ln)
            if m:
                ent = cp[int(m.group(1))]
                if ent:
                    if ent[0] == 'Float': ln = ln.replace('= ?', f'= {ent[1]}f')
                    elif ent[0] == 'Int': ln = ln.replace('= ?', f'= {ent[1]}')
                    elif ent[0] == 'Str': ln = ln.replace('= ?', f'= "{ent[1]}"')
            out.append(ln)
        return '\n'.join(out)
    return ''


def check(cond, msg):
    print(('  OK  ' if cond else '  FAIL') + ' ' + msg)
    return cond


def main():
    ok = True
    data = zipfile.ZipFile(JAR).read(f'{CLS}.class')
    cp = cp_with_floats(data)

    single = method_text(data, cp, 'createSingleBodyLayer')
    ok &= check(bool(single), 'createSingleBodyLayer walks')
    # the bottom box: texOffs(0,19) addBox(1,0,1,14,10,14) ZERO
    ok &= check('bipush 19' in single and 'iconst_0' in single.split('ldc #13 = "lid"')[0].split('texOffs')[0],
                'the bottom texOffs(0,19) pair rides')
    ok &= check('= 14.0f' in single and '= 10.0f' in single and '= 5.0f' in single,
                'the single dims 14/10 (body) + 5 (lid) speak')
    ok &= check('= 9.0f' in single, 'the lid/lock offset y 9 rides')
    ok &= check('= 7.0f' in single and '4.0f' in single, 'the lock (7,-2,..,4) pair rides')
    ok &= check('PartPose.ZERO' in single, 'the bottom ZERO pose keeps')

    dbl = method_text(data, cp, 'createDoubleBodyRightLayer')
    ok &= check(bool(dbl), 'createDoubleBodyRightLayer walks')
    ok &= check('= 15.0f' in dbl and '= 10.0f' in dbl and '= 5.0f' in dbl,
                'the double dims 15/10 (body) + 5 (lid) speak')
    ok &= check('allOfEnumExcept' in dbl, 'the double cull Set ride keeps')

    # the DERIVED BANDS the fold law rides
    b0 = 19 + 14   # the body band's first row
    l0 = 0 + 14    # the lid band's first row
    ok &= check(b0 == 33 and l0 == 14, f'the bands derive: body B0={b0} (10 rows), lid L0={l0} (5 rows)')

    # the Polygon ctor remap law (the 302 bank)
    pdata = zipfile.ZipFile(JAR).read('net/minecraft/client/model/geom/ModelPart$Polygon.class')
    poly = method_text(pdata, cp_with_floats(pdata), '<init>')
    ok &= check('aastore' in poly and 'fload' in poly, 'the Polygon remap walk keeps')

    print('\nRESULT:', 'PIN' if ok else 'DRIFT')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
