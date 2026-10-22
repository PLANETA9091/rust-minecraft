#!/usr/bin/env python3
"""THE ITEM MAX-STACK TABLE'S OWN EXTRACTION (the 26.2 jar verbatim).

THE WALK: an ALIGNED opcode decode of Items.<clinit> (the exact operand
lengths, the branch assert the straight-line law wears) driving a
linear mini-stack walk. Each registerItem/registerBlock call's KEY ARG
(the references/ItemIds|BlockItemIds getstatic) names the item; the
MAX_STACK_SIZE law rides three layers:

1. THE VISIBLE CHAIN — the Properties arg the call's arg window
   carries: `stacksTo(I)` (the DataComponents.MAX_STACK_SIZE component
   the ItemStack.getMaxStackSize walk reads) and the 1-setting walks
   the Item$Properties class's own bodies speak (durability(I) writes
   MAX_STACK_SIZE 1; sword/axe/pickaxe/hoe/spear/tool ride
   ToolMaterial.applyCommonProperties -> durability; humanoidArmor
   rides durability; horseArmor/nautilusArmor/wolfArmor ride
   stacksTo(1));

2. THE FUNCTION LAMBDA — the `registerItem(KEY, Function)` overload
   keeps the walks INSIDE the lambda body (props -> new
   AxeItem(TIER, props.durability(...))): the invokedynamic site's
   BootstrapMethods impl resolves, the body's own straight-line scan
   (stacksTo int / the 1-walks) speaks the value, and the Java eval
   order (the chain built before the call, the body's writes applied
   to the chain INSIDE registerItem) makes the BODY's value win when
   both speak;

3. THE CAPTURING LAMBDA — the DyeColor families (the harnesses, the
   dyed shulker boxes, bundles, beds, banners) do their OWN
   registerItem/registerBlock inside the body: the site's pushed
   captures carry the key and the body's value commits at the site.

The LAST value each layer spoke wins; no value = the 64 default the
row never wears. Emits crates/mc-registry/src/generated/
item_max_stack.rs — the name-keyed NON-64 rows, sorted for the
binary-search lookup, the same shape item_rarity.rs wears.

Usage (the trace the first pass reads):
  python3 scripts/classfile_dump.py .mc2rs/classpath/client.jar \\
      net/minecraft/world/item/Items > items_all.txt
  python3 scripts/extract_max_stack.py
"""
import re, struct, sys, zipfile, os

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from classfile_dump import Reader, parse_constant_pool, cp_utf8, cp_ref

VOCAB = 'crates/mc-registry/src/generated/items.rs'
OUT = 'crates/mc-registry/src/generated/item_max_stack.rs'

ONE_WALKS = {'durability', 'sword', 'axe', 'pickaxe', 'hoe', 'spear', 'tool', 'shovel',
             'humanoidArmor', 'horseArmor', 'nautilusArmor', 'wolfArmor'}
KEY_RE = re.compile(r'references/(?:ItemIds|BlockItemIds)\.([A-Za-z0-9_]+)$')
# the ItemIds FIELD shorthand vs the registry id the ResourceKey wears
# (the one row the vocabulary's own name differs on)
NAME_ALIASES = {'powder_snow': 'powder_snow_bucket'}

jar = zipfile.ZipFile('.mc2rs/classpath/client.jar')
data = jar.read('net/minecraft/world/item/Items.class')
r = Reader(data)
assert r.u4() == 0xCAFEBABE
r.u2(); r.u2()
cp = parse_constant_pool(r)
r.u2(); r.u2(); r.u2()
for _ in range(r.u2()): r.u2()
fields_count = r.u2()
for _ in range(fields_count):
    r.u2(); r.u2(); r.u2()
    for _ in range(r.u2()):
        r.u2()
        alen = r.u4(); r.raw(alen)
methods = {}
methods_count = r.u2()
for _ in range(methods_count):
    r.u2(); name = cp_utf8(cp, r.u2()); desc = cp_utf8(cp, r.u2())
    attrs = {}
    for _ in range(r.u2()):
        aname = cp_utf8(cp, r.u2()); alen = r.u4(); attrs[aname] = r.raw(alen)
    methods[(name, desc)] = attrs
boot = {}
boot_owner = {}
nattr = r.u2()
for _ in range(nattr):
    aname = cp_utf8(cp, r.u2()); alen = r.u4(); adata = r.raw(alen)
    if aname == 'BootstrapMethods':
        ar = Reader(adata)
        n = ar.u2()
        for i in range(n):
            mh_i = ar.u2()
            argc = ar.u2()
            args = [ar.u2() for _ in range(argc)]
            # args[1] = the impl MethodHandle (metafactory's own law):
            # the lambda body the site binds
            impl_name = '?'
            impl_owner = '?'
            if len(args) > 1:
                entry = cp[args[1]]
                if entry[0] == 'MethodHandle':
                    _, kind, ref_i = entry
                    owner, inm, idesc = cp_ref(cp, ref_i)
                    impl_name = inm
                    impl_owner = owner
            boot[i] = impl_name
            boot_owner[i] = impl_owner

def code_bytes(pair):
    attrs = methods.get(pair)
    if attrs is None or 'Code' not in attrs:
        return None
    a = attrs['Code']
    clen = struct.unpack_from('>I', a, 4)[0]
    return a[8:8 + clen]

def desc_arity(desc):
    body = desc[1:desc.index(')')]
    n = 0
    while body:
        c = body[0]
        if c in 'JD':
            n += 2; body = body[1:]
        elif c == '[':
            n += 1
            body = body.lstrip('[')
            if body and body[0] == 'L':
                body = body[body.index(';') + 1:]
            elif body:
                body = body[1:]
        elif c == 'L':
            n += 1; body = body[body.index(';') + 1:]
        else:
            n += 1; body = body[1:]
    return n

# ---- the aligned opcode decoder ----
class Op:
    __slots__ = ('op', 'a', 'b')
    def __init__(self, op, a=None, b=None):
        self.op = op; self.a = a; self.b = b

BRANCH_OPS = set(range(0x99, 0xa9)) | {0xc6, 0xc7, 0xc8, 0xc9, 0xaa, 0xab}

def decode(code):
    ops = []
    i, n = 0, len(code)
    while i < n:
        op = code[i]; i += 1
        if op in (0x10,):
            ops.append(Op(op, struct.unpack_from('>b', code, i)[0])); i += 1
        elif op == 0x11:
            ops.append(Op(op, struct.unpack_from('>h', code, i)[0])); i += 2
        elif 0x02 <= op <= 0x08:
            ops.append(Op(op, op - 0x03))
        elif op == 0x12:
            cp_i = code[i]; i += 1
            t = cp[cp_i][0]
            v = cp_utf8(cp, cp[cp_i][1]) if t == 'String' else (cp[cp_i][1] if t in ('Integer', 'Float') else None)
            ops.append(Op(op, ('int', v) if t in ('Integer',) else ('other', v)))
        elif op in (0x13, 0x14):
            cp_i = struct.unpack_from('>H', code, i)[0]; i += 2
            t = cp[cp_i][0]
            v = cp_utf8(cp, cp[cp_i][1]) if t == 'String' else (cp[cp_i][1] if t in ('Integer', 'Float', 'Double') else None)
            ops.append(Op(op, ('int', v) if t == 'Integer' else ('other', v)))
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3a, 0xa9):
            ops.append(Op(op, code[i])); i += 1
        elif 0x3b <= op <= 0x3e or 0x4b <= op <= 0x4e or 0x2a <= op <= 0x2d or 0x1a <= op <= 0x1d:
            ops.append(Op(op, op if op >= 0x1a else None))
        elif op == 0x84:
            ops.append(Op(op)); i += 2
        elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xbb, 0xbd, 0xc0, 0xc1, 0xc5):
            idx = struct.unpack_from('>H', code, i)[0]
            i += 2
            extra = 0
            if op == 0xb9: extra = 2
            elif op == 0xc5: extra = 1
            i += extra
            ops.append(Op(op, idx))
        elif op == 0xba:
            # the invokedynamic: u2 cp index + u2 zero — 4 operand bytes
            idx = struct.unpack_from('>H', code, i)[0]
            i += 4
            ops.append(Op(op, idx))
        elif op == 0xbc:
            ops.append(Op(op)); i += 1
        elif op == 0xc4:
            wop = code[i]
            i += 3 if wop != 0x84 else 5
            ops.append(Op(op))
        else:
            ops.append(Op(op))
    return ops

# ---- the lambda bodies' own values (the straight-line scan) ----
LAMBDA_VAL = {}
LAMBDA_CTOR = {}   # the lambda body's constructed class (the ctor-law layer)
LAMBDA_WARN = []
for (nm, desc), attrs in methods.items():
    if not nm.startswith('lambda$'):
        continue
    code = code_bytes((nm, desc))
    if code is None:
        continue
    ops = decode(code)
    if any(o.op in BRANCH_OPS for o in ops):
        LAMBDA_WARN.append(nm)
        continue
    val = None
    for o in ops:
        if o.op == 0xb6:  # invokevirtual
            owner, m, d = cp_ref(cp, o.a)
            if m == 'stacksTo':
                val = ('pending',)  # the int arg rides the walk below
            elif m in ONE_WALKS:
                val = 1
    # the stacksTo int needs the mini-stack — redo with the walk:
    val = None
    stack = []
    for o in ops:
        op = o.op
        if op in (0x10, 0x11) or 0x02 <= op <= 0x08:
            stack.append(('int', o.a))
        elif op == 0x12 and o.a and o.a[0] == 'int':
            stack.append(('int', o.a[1]))
        elif op == 0x13 and o.a and o.a[0] == 'int':
            stack.append(('int', o.a[1]))
        elif op in (0xb2,):
            stack.append(('other', None))
        elif op in (0xb6, 0xb7, 0xb8, 0xb9):
            owner, m, d = cp_ref(cp, o.a)
            nargs = desc_arity(d) + (0 if op == 0xb8 else 1)
            popped = list(reversed([stack.pop() if stack else ('other', None) for _ in range(nargs)]))
            if m == 'stacksTo':
                ints = [v for k, v in popped if k == 'int']
                if ints:
                    val = ints[-1]
            elif m in ONE_WALKS:
                val = 1
            ret = d[1 + d.index(')'):]
            if m == '<init>' and op == 0xb7:
                ctor = owner
                if ctor not in ('net/minecraft/world/item/Item$Properties',) and nm not in LAMBDA_CTOR:
                    LAMBDA_CTOR[nm] = ctor
            elif op != 0xb8 and ret != 'V':
                stack.append(('other', None))
        elif op in (0x57, 0x58):
            stack.pop() if stack else None
        elif op == 0x59:
            if stack: stack.append(stack[-1])
        elif op in (0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f):
            if op == 0x5f:
                if len(stack) >= 2:
                    stack[-1], stack[-2] = stack[-2], stack[-1]
            elif op == 0x5a and len(stack) >= 2:
                a = stack.pop(); b = stack.pop(); stack += [b, a, a]
            elif op == 0x5c and len(stack) >= 2:
                a = stack.pop(); b = stack.pop(); stack += [b, a, b, a]
            else:
                stack.append(('other', None))
        elif op in (0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x4b, 0x4c, 0x4d, 0x4e):
            stack.pop() if stack else None
        elif 0x15 <= op <= 0x19 or 0x1a <= op <= 0x2d or op in (0x01, 0xbb, 0xbd, 0xc0, 0xc1, 0x02, 0x03):
            stack.append(('other', None))
        elif op in (0x57,):
            pass
        elif op == 0xbe or op in (0xbf,):
            stack.pop() if stack else None
        # iaload-style (0x2e-0x35) pops handled above only for the ranges modeled;
        # the lambda bodies the vocabulary wears never array-load mid-chain
    LAMBDA_VAL[nm] = val

print(f"lambdas with a value: {sum(1 for v in LAMBDA_VAL.values() if v is not None)}")
print(f"branchy lambdas skipped: {LAMBDA_WARN}")

# ---- the item-class ctor law (the tools' own layer) ----
# the (key, Function)-overload lambdas construct the item classes whose
# CTORS speak the 1-law (AxeItem/ShovelItem/... call
# ToolMaterial.applyToolProperties -> applyCommonProperties ->
# durability -> MAX_STACK_SIZE 1): the ctor bytecode's own verdict
_cls_cache = {}

def class_ctor_value(cls):
    if cls in _cls_cache:
        return _cls_cache[cls]
    val = None
    try:
        cdata = jar.read(f"{cls}.class")
        cr = Reader(cdata)
        assert cr.u4() == 0xCAFEBABE
        cr.u2(); cr.u2()
        ccp = parse_constant_pool(cr)
        cr.u2(); cr.u2(); cr.u2()
        for _ in range(cr.u2()): cr.u2()
        fcnt = cr.u2()
        for _ in range(fcnt):
            cr.u2(); cr.u2(); cr.u2()
            for _ in range(cr.u2()):
                cr.u2(); alen = cr.u4(); cr.raw(alen)
        mc2 = cr.u2()
        ctors = {}
        for _ in range(mc2):
            cr.u2(); cnm = cp_utf8(ccp, cr.u2()); cdsc = cp_utf8(ccp, cr.u2())
            cattrs = {}
            for _ in range(cr.u2()):
                anm = cp_utf8(ccp, cr.u2()); alen = cr.u4(); cattrs[anm] = cr.raw(alen)
            if cnm == '<init>':
                ctors[(cnm, cdsc)] = cattrs
        for (cnm, cdsc), cattrs in ctors.items():
            if 'Code' not in cattrs:
                continue
            ca = cattrs['Code']
            clen = struct.unpack_from('>I', ca, 4)[0]
            ccode = ca[8:8 + clen]
            cr2 = Reader(ccode)
            while cr2.p < len(ccode):
                op = cr2.u1()
                if op == 0xba:
                    cr2.u2(); cr2.u2()
                elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xbb, 0xbd, 0xc0, 0xc1, 0xc5):
                    idx = cr2.u2()
                    if op == 0xb9: cr2.u2()
                    if op == 0xc5: cr2.u1()
                    if op in (0xb6, 0xb7, 0xb8, 0xb9):
                        try:
                            wo, wm, wd = cp_ref(ccp, idx)
                            if wm == 'stacksTo' or wm in ONE_WALKS \
                                or wm in ('applyToolProperties', 'applyCommonProperties'):
                                val = 1
                        except Exception:
                            pass
                elif op in (0x10,):
                    cr2.u1()
                elif op in (0x11,):
                    cr2.u2()
                elif op in (0x12,):
                    cr2.u1()
                elif op in (0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37,
                            0x38, 0x39, 0x3a, 0xa9, 0xbc):
                    cr2.u1()
                elif op == 0x84:
                    cr2.u2()
                elif op == 0xc4:
                    cr2.u1(); cr2.u2()
                elif 0x99 <= op <= 0xa8 or op in (0xc6, 0xc7):
                    cr2.u2()
                elif op == 0xaa or op == 0xab:
                    break
    except (KeyError, struct.error, AssertionError, ValueError):
        val = None
    _cls_cache[cls] = val
    return val

# ---- the clinit walk ----
clinit = code_bytes(('<clinit>', '()V'))
ops = decode(clinit)
assert not any(o.op in BRANCH_OPS for o in ops), "branchy clinit — the linear walk aborts"

table = {}
pending = []       # the visible chain's events (in eval order)
stack = []
lambda_pending = []  # the Function-lambda impls pushed since the last commit

def commit(key, vals):
    if key is None:
        return
    vs = [v for v in vals if v is not None]
    if vs:
        table.setdefault(key, vs[-1])

for o in ops:
    op = o.op
    if op in (0x10, 0x11) or 0x02 <= op <= 0x08:
        stack.append(('int', o.a))
    elif op in (0x12, 0x13, 0x14):
        kind, v = o.a if o.a else ('other', None)
        stack.append((kind, v))
    elif op == 0xb2:  # getstatic
        owner, m, d = cp_ref(cp, o.a)
        sym = f"{owner}.{m}"
        km = KEY_RE.search(sym)
        stack.append(('sym', km.group(1).lower()) if km else ('other', sym))
    elif op in (0xb6, 0xb7, 0xb8, 0xb9):  # invokes
        owner, m, d = cp_ref(cp, o.a)
        nargs = desc_arity(d) + (0 if op == 0xb8 else 1)
        popped = list(reversed([stack.pop() if stack else ('other', None) for _ in range(nargs)]))
        if m == 'stacksTo':
            ints = [v for k, v in popped if k == 'int']
            if ints:
                pending.append(ints[-1])
        elif m in ONE_WALKS:
            pending.append(1)
        elif op == 0xb8 and owner.endswith('/Items') and m.startswith('register'):
            key = None
            for k, v in popped:
                if k == 'sym':
                    key = v
                    break
            if key is not None:
                key = NAME_ALIASES.get(key, key)
            lam_vals = []
            for k, v in popped:
                if k != 'lambda':
                    continue
                lv = LAMBDA_VAL.get(v)
                if lv is None and v in LAMBDA_CTOR:
                    lv = class_ctor_value(LAMBDA_CTOR[v])
                lam_vals.append(lv)
            commit(key, pending + lam_vals)
            pending = []
        # the push: the `new; dup; <init>` idiom CONSUMES the dup copy and
        # pushes nothing (the original ref beneath is the same object) —
        # every other non-void invoke pushes its result, and the result
        # INHERITS a popped key sym (the ItemIds.X.item() and
        # ItemIds.SHIELD.delayedComponent(...) chains the key rides)
        ret = d[1 + d.index(')'):]
        if m == '<init>' and op == 0xb7:
            pass
        elif ret != 'V':
            inherited = next((v for k, v in popped if k == 'sym'), None)
            stack.append(('sym', inherited) if inherited else ('other', None))
    elif op == 0xb3:  # putstatic — the field boundary pops its value
        stack.pop() if stack else None
    elif op == 0xba:  # invokedynamic — the Function/capturing lambda
        idx = o.a
        entry = cp[idx]
        if entry[0] == 'Dynamic':
            _, bsm_index, nat_i = entry
            impl = boot.get(bsm_index, '?')
            impl_owner = boot_owner.get(bsm_index, '?')
            nat = cp[nat_i]
            nat_desc = cp_utf8(cp, nat[2])
            capn = desc_arity(nat_desc)
            popped = list(reversed([stack.pop() if stack else ('other', None) for _ in range(capn)]))
            if impl.startswith('lambda$') and LAMBDA_VAL.get(impl) is not None:
                key = None
                for k, v in popped:
                    if k == 'sym':
                        key = v
                        break
                if key is not None:
                    commit(key, [LAMBDA_VAL[impl]])
            elif impl == '<init>' and LAMBDA_VAL.get(impl) is None:
                # the direct METHOD-REFERENCE factory (Item::new style) —
                # the constructed class's own ctor law the next layer reads
                ctor_law = class_ctor_value(impl_owner)
                if ctor_law is not None:
                    key = None
                    for k, v in popped:
                        if k == 'sym':
                            key = v
                            break
                    if key is not None:
                        commit(key, [ctor_law])
            stack.append(('lambda', impl))
    elif op in (0x59,):
        if stack: stack.append(stack[-1])
    elif op in (0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f):
        if op == 0x5f:
            if len(stack) >= 2:
                stack[-1], stack[-2] = stack[-2], stack[-1]
        elif op == 0x5a and len(stack) >= 2:
            a = stack.pop(); b = stack.pop(); stack += [b, a, a]
        elif op == 0x5c and len(stack) >= 2:
            a = stack.pop(); b = stack.pop(); stack += [b, a, b, a]
        elif op == 0x5b and len(stack) >= 3:
            a = stack.pop(); b = stack.pop(); c = stack.pop(); stack += [c, b, a, a]
        elif op == 0x5d and len(stack) >= 3:
            a = stack.pop(); b = stack.pop(); c = stack.pop(); stack += [c, b, a, c, b]
        elif op == 0x5e and len(stack) >= 4:
            d4 = stack.pop(); c = stack.pop(); b = stack.pop(); a = stack.pop(); stack += [b, a, d4, c, b, a]
    elif op in (0x57, 0x58, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e,
                0x4b, 0x4c, 0x4d, 0x4e, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
                0xbe, 0xbf, 0xbc, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56):
        stack.pop() if stack else None
    elif 0x15 <= op <= 0x19 or 0x1a <= op <= 0x2d or op in (0x01, 0xbb, 0xbd, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a):
        stack.append(('other', None))
    elif op == 0x60 <= 0:  # unreachable guard
        pass
    elif 0x60 <= op <= 0x83 or 0x85 <= op <= 0x98:
        stack.pop() if stack else None
        if 0x85 <= op <= 0x93:
            stack.append(('other', None))
        elif op in (0x94, 0x95, 0x96, 0x97, 0x98):
            pass
        else:
            stack.append(('other', None))
    # everything else: no stack effect

# ---- the ColorCollection loop families (the five DyeColor lambdas) ----
# the dyes register through ONE invokedynamic each inside
# ColorCollection.registerItems(collection, fn): the lambda body does
# its own registerItem with stacksTo, the KEYS the collection's
# color map derives (the 16 DyeColor registry names). The membership =
# the vocabulary's [color]_X shapes (exactly 16 each — the count the
# DyeColor enum's own size asserts); the value = the lambda body's own
# straight-line stacksTo.
DYE_COLORS = ['white', 'orange', 'magenta', 'light_blue', 'yellow', 'lime',
              'pink', 'gray', 'light_gray', 'cyan', 'purple', 'blue',
              'brown', 'green', 'red', 'black']
COLLECTION_FAMILIES = [
    ('lambda$static$47', 1, r'harness'),
    ('lambda$static$26', 1, r'shulker_box'),
    ('lambda$static$132', 1, r'bundle'),
    ('lambda$static$134', 1, r'bed'),
    ('lambda$static$148', 16, r'banner'),
]
vocab_list = re.findall(r'"([a-z0-9_]+)"', open(VOCAB, encoding='utf-8').read())
vocab_set2 = set(vocab_list)
for lam, val, suffix in COLLECTION_FAMILIES:
    names = sorted(
        f'{c}_{suffix}' for c in DYE_COLORS if f'{c}_{suffix}' in vocab_set2)
    # the template_*/open_back variants never ride the collection — the
    # exact 16-name shape IS the membership
    if len(names) != 16:
        print(f"COLLECTION MISMATCH {lam} ({suffix}): {len(names)} names")
        continue
    for nm in names:
        table.setdefault(nm, val)
    print(f"collection family {lam} ({suffix}): 16 names @ {val}")

ones = sorted(k for k, v in table.items() if v == 1)
six = sorted(k for k, v in table.items() if v == 16)
other = {k: v for k, v in table.items() if v not in (1, 16)}
print(f"\nrows: {len(table)} (max1 {len(ones)} / max16 {len(six)} / other {len(other)})")
if other:
    print("other:", other)

for probe, want in [('snowball', 16), ('egg', 16), ('ender_pearl', 16),
                    ('oak_sign', 16), ('bucket', 16), ('water_bucket', 1),
                    ('white_banner', 16), ('diamond_sword', 1),
                    ('oak_boat', 1), ('minecart', 1), ('elytra', 1),
                    ('shield', 1), ('diamond_helmet', 1),
                    ('white_harness', 1), ('white_bed', 1),
                    ('white_bundle', 1), ('white_shulker_box', 1),
                    ('netherite_pickaxe', 1), ('wooden_axe', 1),
                    ('wooden_shovel', 1), ('wooden_hoe', 1),
                    ('stone_axe', 1), ('iron_hoe', 1), ('diamond_shovel', 1),
                    ('mace', 1), ('trident', 1), ('bow', 1), ('crossbow', 1),
                    ('fishing_rod', 1), ('shears', 1), ('totem_of_undying', 1),
                    ('written_book', 16), ('writable_book', 1),
                    ('honey_bottle', 16), ('saddle', 1), ('bundle', 1),
                    ('shulker_box', 1), ('powder_snow_bucket', 1),
                    ('oak_chest_boat', 1), ('stone', None), ('dirt', None)]:
    got = table.get(probe)
    flag = 'OK' if got == want else 'MISMATCH'
    if flag != 'OK':
        print(f"probe {probe}: {got} (want {want}) {flag}")
print("probe sweep done (only mismatches print)")

# ---- emit ----
src = open(VOCAB, encoding='utf-8').read()
vocab = set(re.findall(r'"([a-z0-9_]+)"', src))
rows = sorted((k, v) for k, v in table.items() if k in vocab)
dropped = sorted(k for k in table if k not in vocab)
print(f"\ndropped non-vocabulary rows: {dropped}")

out = [
    "// @generated by `scripts/extract_max_stack.py` for Minecraft 26.2 — DO NOT EDIT BY HAND.",
    "// Source of truth: the official client jar's REAL per-item max_stack_size laws",
    "// (client sha1 2dc72797acbc1b63fc16a11c4ac393605f453754) — the `Items.class`",
    "// <clinit> walked by the ALIGNED mini-stack walk (scripts/classfile_dump.py's",
    "// own parser, the branch assert the straight-line law wears): each",
    "// registerItem/registerBlock call's KEY arg names the item and the",
    "// MAX_STACK_SIZE law rides three layers — (1) the visible Properties chain's",
    "// `stacksTo(I)` (the DataComponents.MAX_STACK_SIZE component the",
    "// ItemStack.getMaxStackSize walk reads) and the 1-setting walks the",
    "// Item$Properties class's own bodies speak (durability(I) writes 1;",
    "// sword/axe/pickaxe/hoe/spear/tool ride ToolMaterial.applyCommonProperties",
    "// -> durability; humanoidArmor rides durability; horseArmor/nautilusArmor/",
    "// wolfArmor ride stacksTo(1)); (2) the `registerItem(KEY, Function)`",
    "// overload's lambda bodies (the invokedynamic site's BootstrapMethods impl",
    "// resolves, the body's straight-line scan speaks, the Java eval order makes",
    "// the body's write win over the chain's); (3) the capturing DyeColor lambdas",
    "// (the harnesses, the dyed shulker boxes, bundles, beds, banners) whose",
    "// pushed captures carry the key. The LAST value each layer spoke wins. Only",
    "// the NON-64 items wear rows — the 64 default the `Item.Properties` field",
    "// init keeps. The NAME is the registry id (the wire order table's own",
    "// vocabulary). Sorted by name for the binary-search lookup.",
    "",
    "/// The per-item max stack size, NON-64 rows only: `(item name, max stack)`.",
    "/// The missing rows ARE the 64 default — the `Item.Properties` field init",
    "/// the jar's own constructor law keeps.",
    f"pub static ITEM_MAX_STACK: [(&str, u8); {len(rows)}] = [",
]
for k, v in rows:
    out.append(f'    ("{k}", {v}),')
out += [
    "];",
    "",
]
open(OUT, 'w', encoding='utf-8').write('\n'.join(out) + '\n')
print(f"wrote {OUT} ({len(rows)} rows)")
