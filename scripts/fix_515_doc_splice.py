#!/usr/bin/env python3
"""autopush-515 repair — the 515 insert cut the wire_furnace_swap doc
mid-comment: the merchant block landed between the furnace doc's head
and its tail. This moves the merchant block before the furnace doc
head and rejoins the furnace doc."""
import sys

PATH = "crates/mc-client/src/screens.rs"
text = open(PATH, encoding="utf-8").read()

a = text.find("/// THE MERCHANT BELIEF'S OWN INVENTORY ADD")
b = text.find("/// + the ORIGINAL hovered stack the", a)
c = text.find("#[allow(clippy::too_many_arguments)]\nfn wire_furnace_swap(", b)
h = text.rfind("/// THE SWAP TABLE OVER THE FURNACE'S OWN BELIEF", 0, a)

for name, idx in (("a", a), ("b", b), ("c", c), ("h", h)):
    if idx < 0:
        sys.exit(f"MARKER NOT FOUND: {name}={idx}")
if not (h < a < b < c):
    sys.exit(f"ORDER WRONG: h={h} a={a} b={b} c={c}")

merchant_block = text[a:b]
furnace_head = text[h:a]
furnace_tail = text[b:c]
rest = text[c:]

new = text[:h] + merchant_block + furnace_head + furnace_tail + rest
open(PATH, "w", encoding="utf-8").write(new)
print(f"moved {len(merchant_block)} chars; head {len(furnace_head)}, tail {len(furnace_tail)}")
