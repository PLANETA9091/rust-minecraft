#!/usr/bin/env python3
"""autopush-316 — scout 8: the BeaconRenderer.submitBeaconBeam indys the
bootstrap lambdas resolve (the glow/solid + alpha + beaconBeam-flag
pairing the beam prism law wears).

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'


def bootstrap_table(data, cp):
    """bsm index -> lambda impl method name."""
    _, q = parse_cp(data)
    q += 6
    (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
    (nfields,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nfields):
        q += 6
        (na,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(na):
            q += 2
            (alen,) = struct.unpack_from('>I', data, q)
            q += 4 + alen
    (nmethods,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nmethods):
        q += 6
        (na,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(na):
            q += 2
            (alen,) = struct.unpack_from('>I', data, q)
            q += 4 + alen
    (ncls,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(ncls):
        name_idx = struct.unpack_from('>H', data, q)[0]
        q += 2
        (alen,) = struct.unpack_from('>I', data, q)
        q += 4
        if utf8(cp, name_idx) == 'BootstrapMethods':
            j = q
            (nb,) = struct.unpack_from('>H', data, j)
            j += 2
            table = {}
            for b in range(nb):
                mh_idx, nargs = struct.unpack_from('>HH', data, j)
                j += 4
                args = []
                for _a in range(nargs):
                    args.append(struct.unpack_from('>H', data, j)[0])
                    j += 2
                impl = None
                if len(args) > 1:
                    amh = cp.get(args[1])
                    if amh and amh[0] == 15:
                        aref = cp.get(amh[2])
                        if aref and aref[0] == 'method':
                            ant = cp.get(aref[1][1])
                            if ant and ant[0] == 'name_type':
                                impl = utf8(cp, ant[1][0])
                table[b] = impl
            return table
    return {}


def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read('net/minecraft/client/renderer/blockentity/BeaconRenderer.class')
    cp, _ = parse_cp(data)
    bsm = bootstrap_table(data, cp)

    for mname, mdesc, mcode, mcp in parse_class(data, {'submitBeaconBeam'}):
        if 'IIIFF' not in mdesc:
            continue
        print(f"--- {mname} {mdesc}")
        lines = disasm(mcode, mcp)
        for i, ln in enumerate(lines):
            if 'invokedynamic' in ln or 'iconst' in ln or 'submitCustomGeometry' in ln:
                print(ln)
                if 'invokedynamic' in ln:
                    tok = ln.split('invokedynamic')[1].split()[0].lstrip('#')
                    idx = int(tok)
                    e = cp.get(idx)
                    if isinstance(e, tuple) and e and e[0] == 18:
                        bsm_idx, nt_idx = e[1]
                        nt = cp.get(nt_idx)
                        name = utf8(cp, nt[1]) if nt and nt[0] == 'name_type' else '?'
                        print(f"    -> bootstrap {bsm_idx} name={name} lambda={bsm.get(bsm_idx)}")


if __name__ == '__main__':
    main()
