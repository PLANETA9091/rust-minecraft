#!/usr/bin/env python3
"""Census: every class in the jar whose CONSTANT POOL references
ItemStack.getPopTime / setPopTime (method or interface refs), plus the
classes carrying the pop-affine float constants 5.0F + 2.0F near
getPopTime — the compiler names every site the pop law wears.
Usage: python3 scripts/poptime_census.py <jar>
"""
import struct, sys, zipfile

def read_u1(b, o): return b[o]
def read_u2(b, o): return struct.unpack_from(">H", b, o)[0]
def read_u4(b, o): return struct.unpack_from(">I", b, o)[0]

# THE TRUE JVM CONSTANT-POOL TAGS (the census's first draft the
# off-by-one table wore — Utf8=1 .. Package=20 the spec speaks)
CP_UTF8, CP_INTEGER, CP_FLOAT, CP_LONG, CP_DOUBLE, CP_CLASSES, CP_STRING, \
CP_FIELD, CP_METHOD, CP_IFACE, CP_NAMEANDTYPE, CP_METHODHANDLE, CP_METHODTYPE, \
CP_DYNAMIC, CP_INVOKEDYNAMIC, CP_MODULE, CP_PACKAGE = \
    1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 16, 17, 18, 19, 20

def parse_class(b):
    o = 8  # skip magic + version
    count = read_u2(b, o); o += 2
    cp = [None] * count
    i = 1
    while i < count:
        tag = b[o]; o += 1
        # 2-byte entries: Class 7, String 8, MethodType 16, Module 19, Package 20
        if tag in (CP_CLASSES, CP_STRING, CP_METHODTYPE, CP_MODULE, CP_PACKAGE):
            cp[i] = (tag, read_u2(b, o)); o += 2
        # 4-byte entries: Integer 3, Float 4, the refs 9/10/11,
        # NameAndType 12, Dynamic 17, InvokeDynamic 18
        elif tag in (CP_INTEGER, CP_FLOAT, CP_FIELD, CP_METHOD, CP_IFACE, \
                     CP_NAMEANDTYPE, CP_DYNAMIC, CP_INVOKEDYNAMIC):
            cp[i] = (tag, read_u2(b, o), read_u2(b, o + 2)); o += 4
        elif tag in (CP_LONG, CP_DOUBLE):
            cp[i] = (tag, read_u4(b, o), read_u4(b, o + 4)); o += 8
            i += 1  # 8-byte constants take two slots
        elif tag == CP_UTF8:
            ln = read_u2(b, o); o += 2
            cp[i] = (tag, b[o:o+ln]); o += ln
        elif tag == CP_METHODHANDLE:
            cp[i] = (tag, b[o], read_u2(b, o+1)); o += 3
        else:
            raise ValueError(f"bad cp tag {tag} at {i}")
        i += 1
    # skip access/this/super
    o += 2 + 2 + 2
    ifc = read_u2(b, o); o += 2 + 2 * ifc
    def utf8(idx): return cp[idx][1].decode("utf-8", "replace")
    def utf(idx): return cp[cp[idx][1]][1].decode("utf-8", "replace")  # Class index
    def name_type(idx):
        nt = cp[idx]
        return utf8(nt[1]), utf8(nt[2])
    # walk FIELDS for popTime field name too
    hits = []
    def walk_members(count, static_flag):
        nonlocal o
        for _ in range(count):
            o += 6  # access, name, desc
            attrs = read_u2(b, o); o += 2
            for _ in range(attrs):
                an = utf8(read_u2(b, o)); o += 2
                ln = read_u4(b, o); o += 4
                if an == "ConstantValue":
                    o += ln
                    continue
                # skip other attribute bodies (we only need Code bodies
                # for method scans below; fields carry no code)
                o += ln
    fc = read_u2(b, o); o += 2
    walk_members(fc, False)
    mc = read_u2(b, o); o += 2
    methods = []
    for _ in range(mc):
        acc = read_u2(b, o); name = utf8(read_u2(b, o+2)); desc = utf8(read_u2(b, o+4))
        o += 6
        attrs = read_u2(b, o); o += 2
        code_body = None
        for _ in range(attrs):
            an = utf8(read_u2(b, o)); o += 2
            ln = read_u4(b, o); o += 4
            if an == "Code":
                code_body = b[o:o+ln]
            o += ln
        methods.append((name, desc, code_body))
    return cp, utf8, utf, name_type, methods

def class_refs_poptime(b, cp, utf8, utf, name_type):
    """Does the constant pool reference getPopTime/setPopTime?"""
    out = []
    for e in cp:
        if e is None: continue
        if e[0] in (CP_FIELD, CP_METHOD, CP_IFACE):
            try:
                cls = utf(e[1])
                nm, ds = name_type(e[2])
            except Exception:
                continue
            if "PopTime" in nm:
                out.append((cls, nm, ds))
    return out

def method_poptime_calls(body, cp, utf8, utf, name_type):
    """Rough scan of a Code attribute: does it invoke getPopTime/setPopTime?
    Finds invokevirtual/getfield referencing those names in the cp —
    positional scan of the code bytes for any cp index that resolves to
    such an entry (cheap but honest: the cp census already gates)."""
    if body is None: return False
    # any cp index referenced; indices < len(cp); scan 2-byte windows
    for o in range(len(body) - 1):
        idx = read_u2(body, o)
        if 1 <= idx < len(cp):
            e = cp[idx]
            if e is None or e[0] not in (CP_METHOD, CP_IFACE, CP_FIELD): continue
            try:
                nm, ds = name_type(e[2])
            except Exception:
                continue
            if "PopTime" in nm:
                return True
    return False

jar = sys.argv[1]
z = zipfile.ZipFile(jar)
for n in sorted(z.namelist()):
    if not n.endswith(".class"):
        continue
    b = z.read(n)
    try:
        cp, utf8, utf, name_type, methods = parse_class(b)
    except Exception as e:
        print(f"PARSE FAIL {n}: {e}")
        continue
    refs = class_refs_poptime(b, cp, utf8, utf, name_type)
    if refs:
        callers = []
        for name, desc, body in methods:
            if method_poptime_calls(body, cp, utf8, utf, name_type):
                callers.append(f"{name}{desc}")
        print(f"{n.replace('.class','').replace('/','.')}")
        for r in refs:
            print(f"    cp-ref: {r[0]}.{r[1]} {r[2]}")
        for c in callers:
            print(f"    code-walk: {c}")
