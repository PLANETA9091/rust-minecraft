#!/usr/bin/env python3
"""Raw opcode disassembly of one method — the classfile_dump.py fold cannot lose this.
Usage: python3 scripts/raw_disasm.py <jar> <net/minecraft/Class> <method>"""
import struct, sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from classfile_dump import parse_class, read_code, cp_utf8, cp_ref, Reader

jar = zipfile.ZipFile(sys.argv[1])
data = jar.read(f"{sys.argv[2]}.class")
cp, methods = parse_class(data)
want = sys.argv[3]

MNEMONIC = {
    0x00:'nop',0x01:'aconst_null',0x02:'iconst_m1',0x03:'iconst_0',0x04:'iconst_1',0x05:'iconst_2',
    0x06:'iconst_3',0x07:'iconst_4',0x08:'iconst_5',0x09:'lconst_0',0x0a:'lconst_1',0x0b:'fconst_0',
    0x0c:'fconst_1',0x0d:'fconst_2',0x0e:'dconst_0',0x0f:'dconst_1',0x10:'bipush',0x11:'sipush',
    0x12:'ldc',0x13:'ldc_w',0x14:'ldc2_w',0x15:'iload',0x16:'lload',0x17:'fload',0x18:'dload',
    0x19:'aload',0x1a:'iload_0',0x1b:'iload_1',0x1c:'iload_2',0x1d:'iload_3',0x1e:'lload_0',
    0x1f:'lload_1',0x20:'lload_2',0x21:'lload_3',0x22:'fload_0',0x23:'fload_1',0x24:'fload_2',
    0x25:'fload_3',0x26:'dload_0',0x27:'dload_1',0x28:'dload_2',0x29:'dload_3',0x2a:'aload_0',
    0x2b:'aload_1',0x2c:'aload_2',0x2d:'aload_3',0x2e:'iaload',0x2f:'laload',0x30:'faload',
    0x31:'daload',0x32:'aaload',0x33:'baload',0x34:'caload',0x35:'saload',0x36:'istore',
    0x37:'lstore',0x38:'fstore',0x39:'dstore',0x3a:'astore',0x3b:'istore_0',0x3c:'istore_1',
    0x3d:'istore_2',0x3e:'istore_3',0x3f:'lstore_0',0x40:'lstore_1',0x41:'lstore_2',0x42:'lstore_3',
    0x43:'fstore_0',0x44:'fstore_1',0x45:'fstore_2',0x46:'fstore_3',0x47:'dstore_0',0x48:'dstore_1',
    0x49:'dstore_2',0x4a:'dstore_3',0x4b:'astore_0',0x4c:'astore_1',0x4d:'astore_2',0x4e:'astore_3',
    0x4f:'iastore',0x50:'lastore',0x51:'fastore',0x52:'dastore',0x53:'aastore',0x54:'bastore',
    0x55:'castore',0x56:'sastore',0x57:'pop',0x58:'pop2',0x59:'dup',0x5a:'dup_x1',0x5b:'dup_x2',
    0x5c:'dup2',0x5d:'dup2_x1',0x5e:'dup2_x2',0x5f:'swap',0x60:'iadd',0x61:'ladd',0x62:'fadd',
    0x63:'dadd',0x64:'isub',0x65:'lsub',0x66:'fsub',0x67:'dsub',0x68:'imul',0x69:'lmul',
    0x6a:'fmul',0x6b:'dmul',0x6c:'idiv',0x6d:'ldiv',0x6e:'fdiv',0x6f:'ddiv',0x70:'irem',
    0x71:'lrem',0x72:'frem',0x73:'drem',0x74:'ineg',0x75:'lneg',0x76:'fneg',0x77:'dneg',
    0x78:'ishl',0x79:'lshl',0x7a:'ishr',0x7b:'lshr',0x7c:'iushr',0x7d:'lushr',0x7e:'iand',
    0x7f:'land',0x80:'ior',0x81:'lor',0x82:'ixor',0x83:'lxor',0x84:'iinc',0x85:'i2l',0x86:'i2f',
    0x87:'i2d',0x88:'l2i',0x89:'l2f',0x8a:'l2d',0x8b:'f2i',0x8c:'f2l',0x8d:'f2d',0x8e:'d2i',
    0x8f:'d2l',0x90:'d2f',0x91:'i2b',0x92:'i2c',0x93:'i2s',0x94:'lcmp',0x95:'fcmpl',0x96:'fcmpg',
    0x97:'dcmpl',0x98:'dcmpg',0x99:'ifeq',0x9a:'ifne',0x9b:'iflt',0x9c:'ifge',0x9d:'ifgt',
    0x9e:'ifle',0x9f:'if_icmpeq',0xa0:'if_icmpne',0xa1:'if_icmplt',0xa2:'if_icmpge',0xa3:'if_icmpgt',
    0xa4:'if_icmple',0xa5:'if_acmpeq',0xa6:'if_acmpne',0xa7:'goto',0xa8:'jsr',0xa9:'ret',
    0xaa:'tableswitch',0xab:'lookupswitch',0xac:'ireturn',0xad:'lreturn',0xae:'freturn',
    0xaf:'dreturn',0xb0:'areturn',0xb1:'return',0xb2:'getstatic',0xb3:'putstatic',0xb4:'getfield',
    0xb5:'putfield',0xb6:'invokevirtual',0xb7:'invokespecial',0xb8:'invokestatic',
    0xb9:'invokeinterface',0xba:'invokedynamic',0xbb:'new',0xbc:'newarray',0xbd:'anewarray',
    0xbe:'arraylength',0xbf:'athrow',0xc0:'checkcast',0xc1:'instanceof',0xc2:'monitorenter',
    0xc3:'monitorexit',0xc4:'wide',0xc5:'multianewarray',0xc6:'ifnull',0xc7:'ifnonnull',
    0xc8:'goto_w',0xc9:'jsr_w',
}

for (name, desc), attrs in methods.items():
    if name != want:
        continue
    code = read_code(attrs)
    if code is None:
        continue
    print(f"=== {name}{desc} ===")
    r = Reader(code[1])
    while r.p < len(code[1]):
        pc = r.p
        op = r.u1()
        m = MNEMONIC.get(op, f"0x{op:02x}")
        extra = ""
        if op in (0x10,):
            extra = str(r.s1())
        elif op in (0x11,):
            extra = str(r.s2())
        elif op in (0x12,):
            i = r.u1()
            t = cp[i]
            extra = f"#{i} " + (repr(t[1]) if t[0] in ('Utf8','String','Integer','Float') else t[0])
        elif op in (0x13, 0x14):
            i = r.u2()
            t = cp[i]
            extra = f"#{i} " + (repr(t[1]) if t[0] in ('String','Integer','Float','Double') else t[0])
        elif 0x15 <= op <= 0x19 or 0x36 <= op <= 0x3a or (op == 0x84):
            if op == 0x84:
                idx = r.u1(); k = r.s1(); extra = f"loc{idx} += {k}"
            else:
                extra = f"loc{r.u1()}"
        elif (0x99 <= op <= 0xa8) or op in (0xc6, 0xc7, 0xc8):
            extra = f"-> {pc + r.s2()}"
        elif op in (0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8):
            i = r.u2()
            ref = cp_ref(cp, i)
            extra = f"{ref[0]}.{ref[1]}{ref[2]}"
        elif op == 0xb9:
            i = r.u2(); r.u2()
            ref = cp_ref(cp, i)
            extra = f"{ref[0]}.{ref[1]}{ref[2]}"
        elif op in (0xbb, 0xbd, 0xc0, 0xc1):
            extra = cp_utf8(cp, cp[r.u2()][1])
        elif op == 0xbc:
            r.u1()
        elif op == 0xba:
            r.u2(); r.u2()
        elif op == 0xc4:
            wop = r.u1(); idx = r.u2()
            extra = f"{MNEMONIC.get(wop, hex(wop))} loc{idx}" + (f" += {r.s2()}" if wop == 0x84 else "")
        elif op == 0xc5:
            i = r.u2(); r.u1()
        elif op in (0xaa, 0xab):
            pad = (4 - (r.p % 4)) % 4
            if pad: r.raw(pad)
            if op == 0xaa:
                df = r.s4(); lo = r.s4(); hi = r.s4()
                extra = f"default={pc+df} lo={lo} hi={hi} " + ", ".join(f"{lo+i}->{pc+r.s4()}" for i in range(hi-lo+1))
            else:
                df = r.s4(); n = r.s4()
                extra = f"default={pc+df} " + ", ".join(f"{r.s4()}->{pc+r.s4()}" for _ in range(n))
        print(f"  {pc:4d} {m} {extra}")
