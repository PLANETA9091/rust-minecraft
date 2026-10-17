#!/usr/bin/env python3
"""THE NOTE BLOCK'S OWN SECOND PROBE (dev-54): the above-block
instrument mapping law the BlockBehaviour$BlockStateBase.instrument()
walk speaks, the setInstrument tail (the harp-keep law), the
isTunable/hasCustomSound verdicts, the full enum row list.
Usage: python3 scripts/jar_work/note_block_682b.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
sys.path.insert(0, __file__.rsplit("/", 1)[0])
# reuse walkers by import-less copy: exec the first probe's defs up to main
src = open(__file__.rsplit("/", 1)[0] + "/note_block_682.py").read()
head = src[:src.index("z = zipfile.ZipFile(JAR)")]
ns = {"__name__": "probe"}
exec(head, ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]; walk_code = ns["walk_code"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]

z = zipfile.ZipFile(JAR)

print("=== BlockBehaviour$BlockStateBase.instrument() — the ABOVE-block mapping law")
for cls in ("net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase",
            "net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase$Cache"):
    ms = methods_of(z, cls)
    if not ms:
        print(f"!! {cls} not found")
        continue
    for sig, (cp, body) in ms.items():
        if "instrument" in sig.split("(")[0] and body:
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8:8 + clen]
            print(f"--- {cls}.{sig}")
            for line in walk_code(cp, code)[:300]:
                print(line)

print()
print("=== NoteBlockInstrument isTunable / hasCustomSound / getCustomSoundId")
ms = methods_of(z, "net.minecraft.world.level.block.state.properties.NoteBlockInstrument")
for sig, (cp, body) in ms.items():
    head_ = sig.split("(")[0]
    if head_ in ("isTunable", "hasCustomSound", "getCustomSoundId", "byBlockBelow") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- NoteBlockInstrument.{sig}")
        for line in walk_code(cp, code)[:80]:
            print(line)

print()
print("=== ALL enum rows (putstatic order)")
for sig, (cp, body) in ms.items():
    if sig.startswith("<clinit>") and body:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        cur = None
        snd = None
        for line in lines:
            if "putstatic" in line and "NoteBlockInstrument." in line:
                nm = line.split("NoteBlockInstrument.")[1].split(":")[0]
                if nm not in ("Type",):
                    if cur:
                        print(f"{cur:18s} {snd or '?'}")
                    cur = nm
                    snd = None
            if "SoundEvents.NOTE_BLOCK" in line or "SoundEvents.PARROT_IMITATE" in line:
                snd = line.split("SoundEvents.")[1].split(":")[0]
        if cur:
            print(f"{cur:18s} {snd or '?'}")

print()
print("=== NoteBlock.setInstrument FULL (the harp-keep law)")
dump_method(z, "net.minecraft.world.level.block.NoteBlock", "setInstrument", max_pc=60)

print()
print("=== getCustomSoundId (NoteBlock)")
dump_method(z, "net.minecraft.world.level.block.NoteBlock", "getCustomSoundId", max_pc=80)
