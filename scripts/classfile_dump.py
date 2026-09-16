#!/usr/bin/env python3
"""Generic JVM classfile reader + symbolic bytecode trace.

Usage: python3 scripts/classfile_dump.py <jar> <net/minecraft/.../Class> [method ...]

Walks the constant pool and traces the named methods' bytecode, folding
literal pushes (bipush/sipush/ldc/iconst/getstatic Math.PI) and arithmetic
so builder-pattern call sites (texOffs/addBox/addOrReplaceChild/PartPose.*/...)
print with their CONCRETE arguments. The javap replacement the headless JRE owes.
"""
import struct, sys, zipfile

class Reader:
    def __init__(self, data):
        self.d = data; self.p = 0
    def u1(self):
        v = self.d[self.p]; self.p += 1; return v
    def u2(self):
        v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v
    def u4(self):
        v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v
    def s1(self):
        v = struct.unpack_from('>b', self.d, self.p)[0]; self.p += 1; return v
    def s2(self):
        v = struct.unpack_from('>h', self.d, self.p)[0]; self.p += 2; return v
    def s4(self):
        v = struct.unpack_from('>i', self.d, self.p)[0]; self.p += 4; return v
    def s8(self):
        v = struct.unpack_from('>q', self.d, self.p)[0]; self.p += 8; return v
    def f4(self):
        v = struct.unpack_from('>f', self.d, self.p)[0]; self.p += 4; return v
    def f8(self):
        v = struct.unpack_from('>d', self.d, self.p)[0]; self.p += 8; return v
    def raw(self, n):
        v = self.d[self.p:self.p+n]; self.p += n; return v

def fmt(v):
    if isinstance(v, float):
        if v == int(v) and abs(v) < 1e15:
            return f"{int(v)}.0"
        return repr(v)
    return str(v)

def parse_constant_pool(r):
    n = r.u2()
    cp = [None] * n  # 1-indexed
    i = 1
    while i < n:
        tag = r.u1()
        if tag == 1:
            ln = r.u2(); cp[i] = ('Utf8', r.raw(ln).decode('utf-8', 'replace'))
        elif tag == 3: cp[i] = ('Integer', r.s4())
        elif tag == 4: cp[i] = ('Float', r.f4())
        elif tag == 5: cp[i] = ('Long', r.s8()); i += 1
        elif tag == 6: cp[i] = ('Double', r.f8()); i += 1
        elif tag == 7: cp[i] = ('Class', r.u2())
        elif tag == 8: cp[i] = ('String', r.u2())
        elif tag in (9, 10, 11): cp[i] = ('Ref', tag, r.u2(), r.u2())
        elif tag == 12: cp[i] = ('NameAndType', r.u2(), r.u2())
        elif tag == 15: cp[i] = ('MethodHandle', r.u1(), r.u2())
        elif tag == 16: cp[i] = ('MethodType', r.u2())
        elif tag in (17, 18): cp[i] = ('Dynamic', r.u2(), r.u2())
        elif tag in (19, 20): cp[i] = ('ModPkg', r.u2())
        else: raise ValueError(f"bad cp tag {tag} at {i}")
        i += 1
    return cp

def cp_utf8(cp, i): return cp[i][1]
def cp_class_name(cp, i): return cp_utf8(cp, cp[i][1])
def cp_nat(cp, i): return cp_utf8(cp, cp[i][1]), cp_utf8(cp, cp[i][2])
def cp_ref(cp, i):
    _, tag, ci, nati = cp[i]
    return cp_class_name(cp, ci), *cp_nat(cp, nati)

def parse_class(data):
    r = Reader(data)
    assert r.u4() == 0xCAFEBABE
    r.u2(); r.u2()
    cp = parse_constant_pool(r)
    r.u2()  # access
    r.u2()  # this
    r.u2()  # super
    for _ in range(r.u2()): r.u2()
    methods = {}
    for section in ('fields', 'methods'):
        cnt = r.u2()
        for _ in range(cnt):
            r.u2(); name = cp_utf8(cp, r.u2()); desc = cp_utf8(cp, r.u2())
            attrs = {}
            for _ in range(r.u2()):
                aname = cp_utf8(cp, r.u2()); alen = r.u4(); adata = r.raw(alen)
                attrs[aname] = adata
            if section == 'methods':
                methods[(name, desc)] = attrs
    return cp, methods

def count_args(desc):
    """descriptor arg slot count (longs/doubles = 2)"""
    i = 1; n = 0
    while desc[i] != ')':
        c = desc[i]
        if c in 'JD': n += 2; i += 1
        elif c == '[':
            n += 1; i += 1
            while desc[i] == '[': i += 1
            if desc[i] == 'L':
                i = desc.index(';', i) + 1
            else:
                i += 1
        elif c == 'L':
            n += 1; i = desc.index(';', i) + 1
        else:
            n += 1; i += 1
    return n

def parse_desc(desc):
    i = desc.index(')') + 1
    return desc[i:]

def read_code(attrs):
    if 'Code' not in attrs: return None
    a = attrs['Code']
    max_stack = struct.unpack_from('>H', a, 0)[0]
    code_len = struct.unpack_from('>I', a, 4)[0]
    return max_stack, a[8:8+code_len]

def trace_code(cp, code):
    """symbolic stack-machine walk, prints folded call sites"""
    r = Reader(code)
    out = []
    stack = []
    locals_ = {}

    def pop():
        return stack.pop() if stack else ('?', 0)
    def popn(k):
        if k == 0: return []
        return [pop() for _ in range(k)][::-1]

    def short(sym):
        v = sym[0]
        if v == 'lit': return fmt(sym[1])
        if v == 'new': return f"new {sym[1]}"
        if v == 'local':
            lv = locals_.get(sym[1], ('?',0))
            return f"loc{sym[1]}={short(lv)}" if sym[1] in locals_ else f"loc{sym[1]}"
        if v == 'expr': return sym[1]
        if v == 'obj': return sym[1]
        if v == 'str': return repr(sym[1])
        return str(sym[0])

    while r.p < len(code):
        pc = r.p
        op = r.u1()
        if op in (0x00,): pass
        elif op == 0x01: stack.append(('obj', 'null'))
        elif 0x02 <= op <= 0x08: stack.append(('lit', op - 0x03))
        elif 0x09 <= op <= 0x0a: stack.append(('lit', float(op - 0x09)))
        elif 0x0b <= op <= 0x0d: stack.append(('lit', float(op - 0x0b)))
        elif 0x0e <= op <= 0x0f: stack.append(('lit', float(op - 0x0e)))
        elif op == 0x10: stack.append(('lit', r.s1()))
        elif op == 0x11: stack.append(('lit', r.s2()))
        elif op == 0x12:
            i = r.u1()
            t = cp[i][0]
            stack.append(('str', cp_utf8(cp, cp[i][1])) if t == 'String'
                         else ('lit', cp[i][1]) if t in ('Integer', 'Float')
                         else ('cp', i))
        elif op in (0x13, 0x14):
            i = r.u2()
            t = cp[i][0]
            stack.append(('str', cp_utf8(cp, cp[i][1])) if t == 'String'
                         else ('lit', cp[i][1]) if t in ('Integer', 'Float', 'Double')
                         else ('cp', i))
        elif 0x15 <= op <= 0x19: stack.append(('local', r.u1()))
        elif 0x1a <= op <= 0x1d: stack.append(('local', op - 0x1a))
        elif 0x1e <= op <= 0x1f: stack.append(('local', op - 0x1e))
        elif 0x20 <= op <= 0x23: stack.append(('local', op - 0x20))
        elif 0x24 <= op <= 0x27: stack.append(('local', op - 0x24))
        elif 0x2a <= op <= 0x2d: stack.append(('local', op - 0x2a))
        elif 0x2e <= op <= 0x35:
            popn(2); stack.append(('obj', 'arr'))
        elif 0x36 <= op <= 0x3a: locals_[r.u1()] = pop()
        elif 0x3b <= op <= 0x3e: locals_[op - 0x3b] = pop()
        elif 0x3f <= op <= 0x40: locals_[op - 0x3f] = pop()
        elif 0x41 <= op <= 0x44: locals_[op - 0x41] = pop()
        elif 0x45 <= op <= 0x46: locals_[op - 0x45] = pop()
        elif 0x47 <= op <= 0x4a: locals_[op - 0x47] = pop()
        elif op == 0x57: pop()
        elif op == 0x58: popn(2)
        elif op == 0x59:
            stack.append(stack[-1])
        elif op in (0x5a, 0x5b, 0x5c, 0x5d, 0x5e):
            # approximate: dup with offset — track raw list
            if op == 0x5a:
                a = pop(); b = pop(); stack += [a, b, a]
            elif op == 0x5b:
                a = pop(); b = pop(); c = pop(); stack += [a, b, c, a]
            elif op == 0x5c:
                a = pop(); b = pop(); stack += [b, a, b, a]
            elif op == 0x5d:
                a = pop(); b = pop(); c = pop(); stack += [a, b, c, a, b]
            else:
                a = pop(); b = pop(); c = pop(); stack += [b, a, b, c, b]
        elif op == 0x5f:
            a = pop(); b = pop(); stack += [a, b]
        elif op in (0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x77, 0x7b, 0x7f, 0x7e, 0x80, 0x81, 0x83, 0x82, 0x7d, 0x79,
                    0x62, 0x66, 0x6a, 0x6e):
            b = pop(); a = pop()
            va = a[1] if a[0] == 'lit' else None
            vb = b[1] if b[0] == 'lit' else None
            name = {0x60:'iadd',0x64:'isub',0x68:'imul',0x6c:'idiv',0x70:'irem',
                    0x74:'ladd',0x77:'lmul',0x7b:'lshl',0x7f:'land',0x7e:'iand',
                    0x80:'ior',0x81:'ixor',0x83:'lor',0x82:'lxor',0x7d:'lushr',0x79:'lshr',
                    0x62:'fadd',0x66:'fsub',0x6a:'fmul',0x6e:'fdiv'}[op]
            if op in (0x62, 0x66, 0x6a, 0x6e):
                # float arith: fold when both sides literal, else keep the symbolic expr
                if va is not None and vb is not None:
                    try:
                        res = {0x62: va+vb, 0x66: va-vb, 0x6a: va*vb, 0x6e: va/vb if vb else None}.get(op)
                        stack.append(('lit', res) if res is not None else ('arith', name, short(a), short(b)))
                    except Exception:
                        stack.append(('arith', name, short(a), short(b)))
                else:
                    stack.append(('expr', f"({short(a)} {('+' if op==0x62 else '-' if op==0x66 else '*' if op==0x6a else '/')} {short(b)})"))
            elif op in (0x7b, 0x79, 0x7d):
                if va is not None: stack.append(('lit', va))
                else: stack.append(('arith', name, short(a), short(b)))
            else:
                if va is not None and vb is not None:
                    try:
                        res = {0x60: va+vb, 0x64: va-vb, 0x68: va*vb,
                               0x6c: va//vb if vb else None, 0x70: va%vb if vb else None}.get(op)
                        stack.append(('lit', res) if res is not None else ('arith', name, short(a), short(b)))
                    except Exception:
                        stack.append(('arith', name, short(a), short(b)))
                else:
                    stack.append(('arith', name, short(a), short(b)))
        elif op == 0x84:  # iinc
            idx = r.u1(); k = r.s1()
            cur = locals_.get(idx, ('lit', 0))
            if cur[0] == 'lit': locals_[idx] = ('lit', cur[1] + k)
        elif 0x85 <= op <= 0x93:
            a = pop()
            nm = {0x85:'i2l',0x86:'i2f',0x87:'i2d',0x88:'l2i',0x89:'l2f',0x8a:'l2d',
                  0x8b:'f2i',0x8c:'f2l',0x8d:'f2d',0x8e:'d2i',0x8f:'d2l',0x90:'d2f',
                  0x91:'i2b',0x92:'i2c',0x93:'i2s'}[op]
            if a[0] == 'lit':
                v = a[1]
                stack.append(('lit', v))  # value-preserving for our reads
            else:
                stack.append(('arith', nm, short(a), ''))
        elif 0x94 <= op <= 0x98: popn(2); stack.append(('lit', 0))
        elif 0x99 <= op <= 0xa8:
            r.u2()
            k = {0x99:1,0x9a:1,0x9b:2,0x9c:2,0x9d:2,0x9e:2,0x9f:2,0xa0:2,0xa1:2,
                 0xa2:2,0xa3:2,0xa4:2,0xa5:2,0xa6:2,0xa7:0,0xa8:0}[op]
            popn(k)
            if op in (0xa7, 0xa8): r.u2() if False else None
        elif op == 0xa7: r.u2()
        elif op == 0xa8: r.u2()
        elif op in (0xa9,): r.u1()
        elif op == 0xaa:  # tableswitch
            pad = (4 - (r.p % 4)) % 4
            if pad: r.raw(pad)
            df = r.s4(); lo = r.s4(); hi = r.s4()
            for _ in range(hi - lo + 1): r.s4()
            pop()
        elif op == 0xab:
            pad = (4 - (r.p % 4)) % 4
            if pad: r.raw(pad)
            df = r.s4(); npairs = r.s4()
            for _ in range(npairs): r.s4(); r.s4()
            pop()
        elif 0xac <= op <= 0xb1:
            k = {0xac:1,0xad:2,0xae:1,0xaf:2,0xb0:1,0xb1:0}[op]
            v = pop() if k else None
            out.append(f"  {pc:4d} return {short(v)}" if k else f"  {pc:4d} return")
        elif op == 0xb2:
            i = r.u2()
            owner, nm, desc = cp_ref(cp, i)
            val = 3.141592653589793 if nm == 'PI' else None
            stack.append(('lit', val) if val is not None else ('obj', f"{owner}.{nm}"))
            if val is not None:
                out.append(f"  {pc:4d} getstatic {owner}.{nm}  == {val}")
        elif op == 0xb3: r.u2(); popn(1)
        elif op == 0xb4:
            i = r.u2(); owner, nm, desc = cp_ref(cp, i)
            a = pop()
            stack.append(('expr', f"{short(a)}.{nm}"))
            out.append(f"  {pc:4d} getfield {owner}.{nm}  ({short(a)})")
        elif op == 0xb5:
            i = r.u2(); owner, nm, desc = cp_ref(cp, i)
            v = pop()
            out.append(f"  {pc:4d} putfield {owner}.{nm} = {short(v)}")
        elif op in (0xb6, 0xb7, 0xb8, 0xb9):
            i = r.u2()
            if op == 0xb9: r.u2()
            owner, nm, desc = cp_ref(cp, i)
            nargs = count_args(desc)
            args = popn(nargs)
            recv = pop() if op != 0xb8 else None
            ret = parse_desc(desc)
            argstr = ', '.join(short(a) for a in args)
            label = {0xb6:'invokevirtual',0xb7:'invokespecial',0xb8:'invokestatic',0xb9:'invokeinterface'}[op]
            owner_short = owner.rsplit('/', 1)[-1]
            if ret != 'V':
                # builder chains: composite expr carries the receiver + args forward
                if recv is not None:
                    stack.append(('expr', f"{short(recv)}.{nm}({argstr})"))
                else:
                    stack.append(('expr', f"{owner_short}.{nm}({argstr})"))
            elif recv is not None and nm == '<init>' and recv[0] == 'new':
                stack.append(('expr', f"{recv[1]}({argstr})"))
            out.append(f"  {pc:4d} {label} {owner}.{nm}{desc}  ({argstr})")
        elif op == 0xba: r.u2(); r.u2()
        elif op == 0xbb:
            i = r.u2(); stack.append(('new', cp_class_name(cp, i)))
        elif op == 0xbc: r.u1(); pop()
        elif op == 0xbd:
            i = r.u2(); pop(); stack.append(('obj', 'arr'))
        elif op == 0xbe: pop()
        elif op == 0xbf: pop()
        elif op == 0xc0: r.u2()
        elif op == 0xc1: r.u2(); pop()
        elif op in (0xc2, 0xc3): pop()
        elif op == 0xc4:  # wide
            wop = r.u1()
            idx = r.u2()
            if wop == 0x84: r.s2(); locals_[idx] = pop()
            elif 0x15 <= wop <= 0x19: stack.append(('local', idx))
            elif 0x36 <= wop <= 0x3a: locals_[idx] = pop()
            elif 0xa9 == wop: pass
        elif op == 0xc6 or op == 0xc7: r.u2(); pop()
        else:
            # anything we don't model precisely — note and continue
            out.append(f"  {pc:4d} ?? opcode 0x{op:02x}")
        del pc
    return out

def main():
    jar_path, cls = sys.argv[1], sys.argv[2]
    want = set(sys.argv[3:])
    jar = zipfile.ZipFile(jar_path)
    data = jar.read(f"{cls}.class")
    cp, methods = parse_class(data)
    print(f"class {cls}")
    for (name, desc), attrs in sorted(methods.items()):
        if want and name not in want: continue
        code = read_code(attrs)
        if code is None: continue
        print(f"\n=== {name}{desc} ===")
        for line in trace_code(cp, code[1]):
            print(line)

if __name__ == '__main__':
    main()
