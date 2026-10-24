#!/usr/bin/env python3
"""THE TEXT BACKGROUND OPACITY'S OWN WALK v2 (autopush-177): the
CORRECT opcode-length disasm the 0x42/0x44/0x46 desync the v1 wore —
scan_jar finds every class the textBackgroundOpacity utf8 carries,
the disasm the consumer methods' fills the (F)I/(I)I getBackgroundColor
pair serves (ChatComponent.extractRenderState the v1 crash the v2
answers), the getBackgroundColor pair + the ChatScreen band the v1
bank re-wears.

Usage: python3 scripts/jar_work/textbg_262v2.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"

core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

scan_jar = core["scan_jar"]
method_bodies = core["method_bodies"]
disasm = core["disasm"]

z = zipfile.ZipFile(JAR)


def dump_class_methods(cls, needles, label):
    print(f"==== {label}: {cls}")
    rb = method_bodies(z, cls)
    if rb is None:
        print("   !! class not found")
        return
    cp, bodies = rb
    for (nm, desc), body in sorted(bodies.items()):
        if body is None:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = disasm(cp, code)
            text = "\n".join(l for _, l in lines)
        except Exception as e:
            print(f"   -- {nm}{desc}: disasm failed ({e!r})")
            continue
        if any(n in text for n in needles):
            print(f"   == {nm}{desc}")
            for _, l in lines:
                print("   ", l)
            print()


# 1. every class the option's utf8 rides (the consumers' own map)
print("==== 1) classes the textBackgroundOpacity utf8 carries")
consumers = []
for cls, status, refs in scan_jar(z, "textBackgroundOpacity"):
    print(f"-- {cls} [{status}]")
    for i, kind, owner, nm in refs[:6]:
        print(f"   cp#{i} {kind} owner={owner} name={nm}")
    consumers.append(cls)

# 2. the ChatComponent consumer's own walk (the v1 crash the v2 answers)
dump_class_methods(
    "net.minecraft.client.gui.components.ChatComponent",
    ["textBackgroundOpacity", "getBackgroundColor", "chatOpacity"],
    "2) ChatComponent consumer walk",
)

# 3. the Options pair re-wear (the (F)I the v1 mangled, the (I)I the bank keeps)
dump_class_methods(
    "net.minecraft.client.Options",
    ["getBackgroundColor", "backgroundForChatOnly"],
    "3) Options.getBackgroundColor pair",
)

# 4. the ChatScreen band re-wear
dump_class_methods(
    "net.minecraft.client.gui.screens.ChatScreen",
    ["getBackgroundColor"],
    "4) ChatScreen band",
)
