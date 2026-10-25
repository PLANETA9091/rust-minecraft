#!/usr/bin/env python3
"""THE FULL-FILE LAWS' OWN ROW INSERTION (dev-168): the
advancedItemTooltips row the ladder's pauseOnLostFocus BEFORE seat
wears — the byte-exact asserts the row the soundCategory AFTER the
insertion answers (the full-file strings the 10 asserts the same
speak, the ends_with tails the key rows the row never touches).
"""
import re

FILES = [
    "crates/mc-client/src/options_file.rs",
    "crates/mc-client/src/screens/tests/part_4.rs",
]

# the literal escape: the test sources keep "\n" as the two-char escape
PAT = re.compile(r'(soundCategory_master:[0-9.]+)\\npauseOnLostFocus')

for path in FILES:
    src = open(path, encoding="utf-8").read()
    n = len(PAT.findall(src))
    out = PAT.sub(r'\1\\nadvancedItemTooltips:false\\npauseOnLostFocus', src)
    open(path, "w", encoding="utf-8").write(out)
    print(f"{path}: {n} full-file asserts updated")
