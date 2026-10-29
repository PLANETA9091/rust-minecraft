#!/usr/bin/env python3
"""Resolve ldc constants + list methods for a 26.2 jar class.
Usage: python3 cp_resolve.py <class-path-inside-jar> [method ...]
"""
import importlib.util, struct, sys, zipfile

spec = importlib.util.spec_from_file_location('rd', 'scripts/raw_disasm.py')
rd = importlib.util.module_from_spec(spec); spec.loader.exec_module(rd)

JAR = '.mc2rs/classpath/client.jar'

def cp_table(data):
    """Return cp as list of (tag, value) with raw offset math mirroring parse_class."""
    r = rd.R(data)
    r.raw(4); r.raw(2) + r.raw(2)
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
        elif tag == 7: cp[i] = ('Class', rd.utf8(cp, r.u2()))
        elif tag == 8: cp[i] = ('Str', rd.utf8(cp, r.u2()))
        elif tag in (9, 10, 11): cp[i] = ('Ref', rd.member_str(cp, i))
        elif tag == 12: cp[i] = ('Name', rd.utf8(cp, r.u2()), rd.utf8(cp, r.u2()))
        elif tag == 15: cp[i] = ('MH', r.u1(), r.u2())
        elif tag == 16: cp[i] = ('MT', r.u2())
        elif tag == 17: cp[i] = ('Dyn', r.u2(), r.u2())
        elif tag == 18: cp[i] = ('Indy', r.u2(), r.u2())
        elif tag == 19: cp[i] = ('Module', r.u2())
        elif tag == 20: cp[i] = ('Pkg', r.u2())
        else: raise ValueError(f'tag {tag}')
        i += 1
    return cp

def main():
    cls = sys.argv[1]
    want = set(sys.argv[2:])
    z = zipfile.ZipFile(JAR)
    data = z.read(f'{cls}.class')
    cp = cp_table(data)
    for name, desc, code, cpr in rd.parse_class(data, want):
        print(f'=== {name} {desc} ===')
        if code is None:
            print('  (abstract/native)')
            continue
        for line in rd.disasm(code, cp):
            # resolve ldc #N = value with the rich table
            if line.lstrip().startswith(('ldc', 'ldc_w')):
                try:
                    idx = int(line.split('#')[1].split()[0])
                    e = cp[idx]
                    if e and e[0] in ('Float', 'Int', 'Str', 'Long', 'Double'):
                        line = f'{line}  [{e[0]} {e[1]!r}]'
                except Exception:
                    pass
            print(line)
        print()

if __name__ == '__main__':
    main()
