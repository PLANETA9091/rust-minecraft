#!/usr/bin/env python3
"""llama_480 — the trader llama's own kind the jar's own bytes speak
(the javap-less walk the JRE-only sandbox wears): the LlamaModel.
createBodyModel's mesh the float-exact dumper ride needs, the Llama's
createAttributes + the TraderLlama's own goals the attribute rows the
port banks, the EntityTypes clinit's TRADER_LLAMA registration region
the sized/eyeHeight words speak. The dumper's CONSTANT_Integer/Float
half-word read the round fixes first (jar_work_parse_sprites_461.py —
the 4-byte payload the >HH pair never wore, the full u32 the >I unpack
banks). Usage: python3 scripts/jar_work_parse_llama_480.py >
scripts/jar_work/llama_480.txt"""
import contextlib
import io
import struct
import sys
import zipfile

sys.path.insert(0, "scripts")
import jar_work_parse_sprites_461 as dumper  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/model/animal/llama/LlamaModel.class",
    "net/minecraft/client/model/animal/llama/BabyLlamaModel.class",
    "net/minecraft/world/entity/animal/equine/Llama.class",
    "net/minecraft/world/entity/animal/equine/TraderLlama.class",
]
WANTED = {
    "createBodyModel", "setupAnim", "createAttributes", "registerGoals",
    "isFood", "getEatAnimation", "<init>", "<clinit>", "makeFlameParticle",
    "getPickResult", "onClimbable", "getPassengersRidingOffset",
}
REGIONS = [
    # (class, marker-substring, before, after)
    ("net/minecraft/world/entity/EntityTypes.class", "TRADER_LLAMA:Lnet/minecraft/world/entity/EntityType;", 48, 4),
    ("net/minecraft/world/entity/EntityTypes.class", "LLAMA:Lnet/minecraft/world/entity/EntityType;", 24, 4),
]


def disasm_to_lines(data, wanted):
    pool = dumper.read_const_pool(data)
    lines = []
    cp_count = struct.unpack_from(">H", data, 8)[0]
    off = 10
    i = 1
    while i < cp_count:
        tag = data[off]
        if tag == dumper.CONSTANT_Utf8:
            off += 3 + struct.unpack_from(">H", data, off + 1)[0]
        elif tag in (dumper.CONSTANT_Long, dumper.CONSTANT_Double):
            off += 9
            i += 1
        elif tag in (dumper.CONSTANT_Field, dumper.CONSTANT_Method,
                     dumper.CONSTANT_Iface, dumper.CONSTANT_NameAndType,
                     dumper.CONSTANT_Integer, dumper.CONSTANT_Float,
                     dumper.CONSTANT_Dynamic, dumper.CONSTANT_InvokeDynamic):
            off += 5
        elif tag == dumper.CONSTANT_MethodHandle:
            off += 4
        else:
            off += 3
        i += 1
    off += 6
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc
    fs = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(fs):
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(ac):
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6 + alen
    ms = struct.unpack_from(">H", data, off)[0]
    off += 2
    for _ in range(ms):
        mflags, mname, mdesc = struct.unpack_from(">HHH", data, off)
        off += 6
        ac = struct.unpack_from(">H", data, off)[0]
        off += 2
        mname_s = dumper.utf8(pool, mname)
        mdesc_s = dumper.utf8(pool, mdesc)
        for _ in range(ac):
            aname_i = struct.unpack_from(">H", data, off)[0]
            alen = struct.unpack_from(">I", data, off + 2)[0]
            off += 6
            aname = dumper.utf8(pool, aname_i)
            if aname == "Code":
                clen2 = struct.unpack_from(">I", data, off + 4)[0]
                body = data[off + 8: off + 8 + clen2]
                if wanted(mname_s, mdesc_s):
                    lines.append(f"  METHOD {mname_s} {mdesc_s}  (code={clen2})")
                    lines.extend(dumper.disasm(body, pool, indent="    "))
            off += alen
    return lines


def main():
    parts = []
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                parts.append(f"== {cls}: ABSENT")
                continue
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                dumper.dump_class(data, lambda n, d: any(k in n for k in WANTED))
            parts.append(f"== {cls}\n{buf.getvalue()}")

        # THE REGISTRATION REGIONS (the EntityTypes clinit the marker
        # windows speak — the builder chain the putstatic's own back
        # walk serves)
        data = z.read("net/minecraft/world/entity/EntityTypes.class")
        lines = disasm_to_lines(
            data, lambda n, d: n == "<clinit>"
        )
        for cls_name, marker, before, after in REGIONS:
            idx = next((k for k, ln in enumerate(lines) if marker in ln), None)
            if idx is None:
                parts.append(f"== {cls_name} {marker}: MARKER ABSENT")
                continue
            lo = max(0, idx - before)
            hi = min(len(lines), idx + 1 + after)
            parts.append(
                f"== {cls_name} <clinit> region [{marker}] lines {lo}..{hi}\n"
                + "\n".join(lines[lo:hi])
            )
        # THE REGISTRY NAME VERDICT (the clinit's own
        # `register("trader_llama")` string the pool keeps — the count
        # the admission word reads)
        names = [ln for ln in lines if '"trader_llama"' in ln or '"llama"' in ln]
        parts.append("== name strings in clinit\n" + "\n".join(names))
    out = "\n\n".join(parts)
    print(out, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
