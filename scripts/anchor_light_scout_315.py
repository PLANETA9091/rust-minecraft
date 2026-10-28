#!/usr/bin/env python3
"""autopush-315 — THE ANCHOR'S OWN CHARGE-LADDER MESH scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - assets/minecraft/blockstates/respawn_anchor.json the charges=0..4
    variant rows (the model ladder the blockstate speaks),
  - assets/minecraft/models/block/respawn_anchor_{0..4}.json the
    cube_bottom_top trio the #side/#top/#bottom refs answer (the
    charge-0 top_off the uncharged look the charged top the glow
    wears),
  - the respawn_anchor_top.png.mcmeta the animated master the atlas
    bake answers,
  - the Blocks clinit's RESPAWN_ANCHOR registration the lightLevel
    indy the bootstrap lambda resolves (the glow ladder's own law the
    lambda body speaks — the bipush/imul/ireturn rows the constant
    answers).

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'
BLOCKS = 'net/minecraft/world/level/block/Blocks'


def dump_assets(zf):
    print("=== the blockstate + the model ladder (the jar's own JSON) ===")
    for path in [
        'assets/minecraft/blockstates/respawn_anchor.json',
        'assets/minecraft/models/block/respawn_anchor_0.json',
        'assets/minecraft/models/block/respawn_anchor_1.json',
        'assets/minecraft/models/block/respawn_anchor_2.json',
        'assets/minecraft/models/block/respawn_anchor_3.json',
        'assets/minecraft/models/block/respawn_anchor_4.json',
        'assets/minecraft/textures/block/respawn_anchor_top.png.mcmeta',
        'assets/minecraft/textures/block/respawn_anchor_top_off.png.mcmeta',
    ]:
        try:
            body = zf.read(path).decode('utf-8', 'replace')
        except KeyError:
            print(f"!! {path} MISSING")
            continue
        print(f"--- {path}")
        print(body)


def bsm_lambda_name(cp, bsm_idx):
    """The bootstrap entry's lambda method name (the impl MethodHandle's
    method ref the name answers)."""
    # walk the BootstrapMethods class attribute
    pass  # resolved in main() against the raw attribute bytes


def main():
    zf = zipfile.ZipFile(JAR)
    dump_assets(zf)

    data = zf.read(BLOCKS + '.class')

    # --- the BootstrapMethods table (index -> impl handle method name)
    bsm_table = {}
    # find the attribute via the class-attribute walk (indy_dump's walk)
    cp, p = parse_cp(data)
    p += 6  # access/this/super
    (nif,) = struct.unpack_from('>H', data, p)
    p += 2 + 2 * nif
    (nfields,) = struct.unpack_from('>H', data, p)
    p += 2
    for _ in range(nfields):
        p += 6
        (na,) = struct.unpack_from('>H', data, p)
        p += 2
        for _ in range(na):
            p += 2
            (alen,) = struct.unpack_from('>I', data, p)
            p += 4 + alen
    (nmethods,) = struct.unpack_from('>H', data, p)
    p += 2
    for _ in range(nmethods):
        p += 6
        (na,) = struct.unpack_from('>H', data, p)
        p += 2
        for _ in range(na):
            p += 2
            (alen,) = struct.unpack_from('>I', data, p)
            p += 4 + alen
    (ncls,) = struct.unpack_from('>H', data, p)
    p += 2
    for _ in range(ncls):
        name_idx = struct.unpack_from('>H', data, p)[0]
        p += 2
        (alen,) = struct.unpack_from('>I', data, p)
        p += 4
        if utf8(cp, name_idx) == 'BootstrapMethods':
            j = p
            (nb,) = struct.unpack_from('>H', data, j)
            j += 2
            for b in range(nb):
                mh_idx, nargs = struct.unpack_from('>HH', data, j)
                j += 4
                impl_name = None
                args = []
                for _a in range(nargs):
                    aidx = struct.unpack_from('>H', data, j)[0]
                    j += 2
                    args.append(aidx)
                # the impl lambda = the bootstrap entry's SECOND arg (the
                # LambdaMetafactory signature: MethodType, impl MethodHandle
                # kind-6 ref_static, instantiated MethodType the trio wears)
                if len(args) > 1:
                    amh = cp.get(args[1])
                    if amh and amh[0] == 15:
                        aref = cp.get(amh[2])
                        if aref and aref[0] == 'method':
                            acls_i, anat_i = aref[1]
                            ant = cp.get(anat_i)
                            if ant and ant[0] == 'name_type':
                                impl_name = utf8(cp, ant[1][0])
                if impl_name:
                    bsm_table[b] = impl_name

    # --- the clinit's anchor window
    print("=== the Blocks clinit's RESPAWN_ANCHOR registration walk ===")
    indy_windows = []
    for name, desc, code, rcp in parse_class(data, {'<clinit>'}):
        lines = disasm(code, rcp)
        anchors = [i for i, l in enumerate(lines)
                   if 'putstatic' in l and 'RESPAWN_ANCHOR' in l]
        if not anchors:
            print('!! no putstatic RESPAWN_ANCHOR in clinit')
            return
        for ai in anchors:
            lo = max(0, ai - 46)
            for l in lines[lo:ai + 4]:
                print(l)
            # the indys in the window resolve to lambdas
            for l in lines[lo:ai + 4]:
                if 'invokedynamic' in l:
                    tok = l.split('invokedynamic')[1].split()[0].lstrip('#')
                    indy_windows.append(int(tok))

    # --- the indy -> bootstrap -> lambda walk
    print("=== the window's indys the bootstrap lambdas resolve ===")
    for idx in indy_windows:
        e = cp.get(idx)
        if not (isinstance(e, tuple) and e and e[0] == 18):
            print(f"indy #{idx}: NOT an InvokeDynamic entry")
            continue
        bsm_idx, nt_idx = e[1]
        nt = cp.get(nt_idx)
        name = utf8(cp, nt[1]) if nt and nt[0] == 'name_type' else f'#{nt_idx}'
        lam = bsm_table.get(bsm_idx, f'?bsm{bsm_idx}')
        print(f"indy #{idx}: bootstrap {bsm_idx} name={name} -> lambda {lam}")

        # --- the lambda's own body (the law the constant speaks)
        for mname, mdesc, mcode, mcp in parse_class(data, {lam}):
            print(f"--- {lam} {mdesc}")
            for ln in disasm(mcode, mcp):
                print(ln)


if __name__ == '__main__':
    main()
