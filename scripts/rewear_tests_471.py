#!/usr/bin/env python3
"""autopush-469 — THE FIVE SIBLING TESTS' OWN RE-BANK (the whole-value
folds the 470 census refuses the range law the 26.2 bytes speak
re-wears — the five tests the range asserts re-bank, the debug trio
the 468 law already wears the block the round splices).

Usage: python3 scripts/rewear_tests_471.py
"""
import sys

PART11 = "crates/mc-client/src/screens/tests/part_11.rs"
BLOCK = "scripts/rewear_block_471.rs"

START_MARK = "#[test]\nfn the_anvil_field_the_clipboard_walk_wears() {"
END_MARK = '/// THE DEBUG SEARCH BOX\'S OWN CLIPBOARD TRIO (the 470 census\'s'

text = open(PART11, encoding="utf-8").read()
start = text.find(START_MARK)
if start < 0:
    sys.exit("GUARD: the anvil test's own start the block never wears")
end = text.find(END_MARK, start)
if end < 0:
    sys.exit("GUARD: the debug test's own doc the block never wears")
new_block = open(BLOCK, encoding="utf-8").read()
if not new_block.endswith("\n"):
    new_block += "\n"
# the splice keeps the debug doc comment the EOF side wears
replaced = text[:start] + new_block + "\n" + text[end:]
open(PART11, "w", encoding="utf-8").write(replaced)
print(f"the five sibling tests re-banked: {start}..{end} the block the splice wears")
print(f"old block {end - start} bytes -> new block {len(new_block)} bytes")
