#!/usr/bin/env python3
"""THE CAPTION KEY + LAMBDA + CONSUMER FIXER (autopush-179): the CP entry
924 the caption key speaks, the Options lambda$new the CaptionBasedToString
law keeps, the scan_jar walk the backgroundForChatOnly refs the consumer
class answers.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)

# ---- 1. resolve CP entries by index in Options (caption key + proc key) ----
d = z.read("net/minecraft/client/Options.class")
cp, _p = core["parse_cp"](d)
def utf8_of_string_entry(idx):
    e = cp[idx] if idx < len(cp) else None
    if e and e[0] == 8:  # CONSTANT_String -> e[1] is the utf8 index
        s_idx = e[1]
        u = cp[s_idx]
        if u and u[0] == 1:
            return u[1]
        return s_idx
    return e
for idx in (924, 474):
    print("CP#%d:" % idx, repr(utf8_of_string_entry(idx)))
print()

# ---- 2. the Options lambda$new methods (the caption indy's impl) ----
cp2, bs2 = core["method_bodies"](z, "net.minecraft.client.Options")
if bs2:
    for (nm, desc), b in sorted(bs2.items()):
        if b is None or "lambda" not in nm:
            continue
        txt = "\n".join(l for _, l in dis(cp2, b))
        if "visibility" in txt or "Chat" in txt or "booleanValue" in txt or "commonComponents" in txt.lower() or "optionStatus" in txt.lower():
            print("== Options ::", nm, desc)
            for _, l in dis(cp2, b):
                print("   ", l)
            print()
print()

# ---- 3. the whole-jar scan: who references backgroundForChatOnly ----
hits = core["scan_jar"](z, "backgroundForChatOnly")
print("SCAN HITS:", hits)
print()

# ---- 4. the lang rows the caption key + the bool values wear ----
lang = z.read("assets/minecraft/lang/en_us.json").decode("utf-8", "replace")
import json
table = json.loads(lang)
for k, v in table.items():
    if "background" in k.lower() or "for_chat" in k.lower() or "forChat" in k:
        print("LANG:", k, "=", repr(v))
# the caption key resolved above may be one of these; print near rows too
print("LANG options.chat.visibility.full =", repr(table.get("options.chat.visibility.full")))
print("LANG options.chat.visibility =", repr(table.get("options.chat.visibility")))
