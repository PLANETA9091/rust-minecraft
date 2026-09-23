#!/usr/bin/env python3
"""Scan the client jar for `sipush <N>` instruction sites (the jar's own
scan the LevelEvent rounds speak). Walks every method's Code attribute
linearly (opcode widths + switch alignment padding), so operands read
INSTRUCTION positions, not payload noise.

Usage: python3 scripts/sipush_scan.py <jar> <value> [<value> ...]
Prints: class::method  (pc)  for each site.
"""
import struct, sys, zipfile

def widths():
    w = [None] * 202
    no_op = list(range(0x00, 0x10)) + list(range(0x1a, 0x36)) + list(range(0x3b, 0x4f)) \
        + list(range(0x4f, 0x57)) + list(range(0x57, 0x84)) + list(range(0x85, 0x94)) \
        + list(range(0x94, 0x99)) + [0xaa, 0xab, 0xac, 0xb0, 0xb1, 0xbe, 0xbf, 0xc2, 0xc3]
    for op in no_op:
        w[op] = 0
    w[0x10] = 1; w[0x11] = 2; w[0x12] = 1
    for op in (0x13, 0x14): w[op] = 2
    for op in (0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3a, 0xa9): w[op] = 1
    w[0x84] = 2
    for op in list(range(0x99, 0xa9)) + [0xc6, 0xc7]: w[op] = 2
    for op in list(range(0xb2, 0xb9)): w[op] = 2   # field/method access: cp index u2
    w[0xb9] = 4  # invokeinterface: cp u2 + count + 0
    w[0xba] = 4  # invokedynamic: cp u2 + 0u2
    w[0xbb] = 2; w[0xbc] = 1; w[0xbd] = 2
    w[0xc0] = 2; w[0xc1] = 2
    w[0xc5] = 3
    w[0xc8] = 4; w[0xc9] = 4
    return w

W = widths()

def walk_code(code):
    pc, n = 0, len(code)
    while pc < n:
        op = code[pc]
        if op == 0xc4:
            nop = code[pc + 1]
            if nop == 0x84:
                yield (pc, op, code[pc + 2:pc + 6]); pc += 6
            else:
                yield (pc, op, code[pc + 2:pc + 4]); pc += 4
            continue
        if op in (0xaa, 0xab):
            p = pc + 1
            p += (4 - (p % 4)) % 4
            if op == 0xaa:
                lo, hi = struct.unpack('>ii', code[p:p + 8])
                p += 8 + 4 * (hi - lo + 2)
            else:
                nkeys = struct.unpack('>i', code[p:p + 4])[0]
                p += 4 + 8 * nkeys + 4
            yield (pc, op, b'')
            pc = p
            continue
        if W[op] is None:
            raise ValueError(f"unknown opcode {op:#x} at {pc}")
        nb = W[op]
        yield (pc, op, code[pc + 1:pc + 1 + nb])
        pc += 1 + nb

def cp_utf8s(data):
    p = 8
    cpc = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    out = {}
    i = 1
    while i < cpc:
        tag = data[p]; p += 1
        if tag == 1:
            ln = struct.unpack('>H', data[p:p + 2])[0]
            out[i] = data[p + 2:p + 2 + ln].decode('utf-8', 'replace')
            p += 2 + ln
        elif tag in (7, 8, 16, 19, 20):
            p += 2
        elif tag == 15:
            p += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            p += 4
        elif tag in (5, 6):
            p += 8; i += 1
        else:
            raise ValueError(f"cp tag {tag}")
        i += 1
    return out

def method_codes(data, u8):
    """Yield (method_name, code_body_bytes)."""
    p = 8
    p += 2  # cpc already past — no: re-parse cp by calling cp_utf8s separately; here skip cp:
    # re-walk cp to find fields/methods start:
    cpc = struct.unpack('>H', data[8:10])[0]
    p = 10
    i = 1
    while i < cpc:
        tag = data[p]; p += 1
        if tag == 1:
            p += 2 + struct.unpack('>H', data[p:p + 2])[0]
        elif tag in (7, 8, 16, 19, 20):
            p += 2
        elif tag == 15:
            p += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            p += 4
        elif tag in (5, 6):
            p += 8; i += 1
        else:
            raise ValueError(tag)
        i += 1
    p += 6
    ic = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    for _ in range(ic):
        p += 2
        icn = struct.unpack('>H', data[p:p + 2])[0]; p += 2
        p += 2 * icn
    fc = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    for _ in range(fc):
        p += 6
        an = struct.unpack('>H', data[p:p + 2])[0]; p += 2
        for _ in range(an):
            p += 2; aln = struct.unpack('>I', data[p:p + 4])[0]; p += 4 + aln
    mc = struct.unpack('>H', data[p:p + 2])[0]; p += 2
    for _ in range(mc):
        acc, name_i, desc_i = struct.unpack('>HHH', data[p:p + 6]); p += 6
        ac = struct.unpack('>H', data[p:p + 2])[0]; p += 2
        for _ in range(ac):
            an_i = struct.unpack('>H', data[p:p + 2])[0]
            aln = struct.unpack('>I', data[p + 2:p + 6])[0]
            body = data[p + 6:p + 6 + aln]
            p += 6 + aln
            if u8.get(an_i) == 'Code' and aln >= 8:
                clen = struct.unpack('>I', body[4:8])[0]
                yield (u8.get(name_i, '?'), body[8:8 + clen])

def main():
    jar, vals = sys.argv[1], set(int(v) for v in sys.argv[2:])
    z = zipfile.ZipFile(jar)
    for n in z.namelist():
        if not n.endswith('.class'):
            continue
        data = z.read(n)
        try:
            u8 = cp_utf8s(data)
            for name, body in method_codes(data, u8):
                for pc, op, operand in walk_code(body):
                    if op == 0x11:
                        v = struct.unpack('>h', operand)[0]
                        if v in vals:
                            print(f"{n[:-6]}::{name}  pc={pc}  sipush {v}")
        except Exception as e:
            print(f"# skip {n}: {e}", file=sys.stderr)

if __name__ == '__main__':
    main()
