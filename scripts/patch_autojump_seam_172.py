#!/usr/bin/env python3
"""THE AUTOJUMP ROW'S OWN SEAM PATCH (the dev-172 round — the dev-216
14-site precedent wears): the full-equality assert strings the save law
speaks gain the `autoJump:false` row the pauseOnLostFocus/toggleCrouch
seam keeps. The synthetic INPUT writes (the fs::write feeds the decode
arms read) the missing-row law answers — the skip keeps them verbatim.
"""
import re

FILES = [
    "crates/mc-client/src/screens/tests/part_4.rs",
    "crates/mc-client/src/options_file.rs",
]

# the seam inside a full-equality assert line: the showSubtitles row the
# head keeps + the pauseOnLostFocus/toggleCrouch seam the insert wears
PAT = re.compile(r"(showSubtitles[^\n]*?pauseOnLostFocus:(true|false)\\n)toggleCrouch:")

for path in FILES:
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    out = []
    patched = 0
    for ln in lines:
        if "showSubtitles" in ln and "pauseOnLostFocus:" in ln and "\\ntoggleCrouch:" in ln:
            new = PAT.sub(r"\1autoJump:false\\ntoggleCrouch:", ln)
            if new != ln:
                patched += 1
            ln = new
        out.append(ln)
    with open(path, "w", encoding="utf-8") as f:
        f.writelines(out)
    print(f"{path}: {patched} site(s) patched")
