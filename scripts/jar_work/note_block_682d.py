#!/usr/bin/env python3
"""THE NOTE BLOCK'S OWN FOURTH PROBE (dev-54): the below-block ->
instrument mapping the Blocks clinit's own .instrument(...) property
rows speak — every register(name, ...props().instrument(X)) pair the
whole clinit walk banks. Usage: python3 scripts/jar_work/note_block_682d.py <jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
src = open(__file__.rsplit("/", 1)[0] + "/note_block_682.py").read()
head = src[:src.index("z = zipfile.ZipFile(JAR)")]
ns = {"__name__": "probe"}
exec(head, ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]; walk_code = ns["walk_code"]
methods_of = ns["methods_of"]

z = zipfile.ZipFile(JAR)

ms = methods_of(z, "net.minecraft.world.level.block.Blocks")
for sig, (cp, body) in ms.items():
    if not sig.startswith("<clinit>") or not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = walk_code(cp, code)
    last_name = None
    last_inst_arg = None
    pending = None
    rows = []
    for i, line in enumerate(lines):
        parts = line.split(": ", 1)
        op = parts[1].split(" ")[0] if len(parts) > 1 else ""
        if "BlockItemIds." in line and "getstatic" in line:
            # modern clinit registers via BlockItemId enums — the
            # getstatic row the registration's own name wears
            last_name = line.split("BlockItemIds.")[1].split(":")[0].strip()
        if "NoteBlockInstrument." in line and ("getstatic" in line or "ldc" in line):
            pending = line.split("NoteBlockInstrument.")[1].split(":")[0].strip()
        if "Properties.instrument:(Lnet" in line or "BlockBehaviour$Properties.instrument:" in line:
            if pending and last_name:
                rows.append((last_name, pending))
            pending = None
    print(f"total .instrument() property rows: {len(rows)}")
    for nm, inst in rows:
        print(f"{nm:28s} -> {inst}")
