#!/usr/bin/env python3
"""THE PART_4 FILE-LAW RE-WEAR (the autopush-190 round): the spectator
pair rows the byte-exact file laws the END appends — the full-file
asserts the `...loadToolbarActivator:key.keyboard.x\n",` tail the pair
rows wear, the END-seat `ends_with` law the spectator hotbar row
answers."""
import io

P = "crates/mc-client/src/screens/tests/part_4.rs"
t = io.open(P, encoding="utf-8").read()

OLD_TAIL = 'key_key.saveToolbarActivator:key.keyboard.c\\nkey_key.loadToolbarActivator:key.keyboard.x\\n",'
NEW_TAIL = 'key_key.saveToolbarActivator:key.keyboard.c\\nkey_key.loadToolbarActivator:key.keyboard.x\\nkey_key.spectatorOutlines:key.keyboard.unknown\\nkey_key.spectatorHotbar:key.mouse.middle\\n",'
n = t.count(OLD_TAIL)
t = t.replace(OLD_TAIL, NEW_TAIL)
print(f"full-file tails re-worn: {n}")

OLD_END = '.ends_with("key_key.loadToolbarActivator:key.keyboard.x")'
NEW_END = '.ends_with("key_key.spectatorHotbar:key.mouse.middle")'
m = t.count(OLD_END)
t = t.replace(OLD_END, NEW_END)
print(f"ends_with END-seat laws re-worn: {m}")

io.open(P, "w", encoding="utf-8").write(t)
print("done")
