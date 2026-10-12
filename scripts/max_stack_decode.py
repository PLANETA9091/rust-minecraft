#!/usr/bin/env python3
"""Walk Items.<clinit>: pair each putstatic field with the stacksTo/
durability constants its own construction block spoke. The 26.2 jar's
own max_stack_size law."""
import re

lines = open('items_clinit.txt', encoding='utf-8', errors='replace').read().splitlines()
events = []  # (lineno, kind, val)
for i, ln in enumerate(lines):
    m = re.search(r'putstatic\s+#\d+\s+// Field net/minecraft/world/item/Items\.(\w+):', ln)
    if m:
        events.append((i, 'field', m.group(1)))
        continue
    if 'stacksTo' in ln:
        # the constant rides the 1-3 lines above (bipush/sipush/iconst)
        val = None
        for j in range(i - 1, max(i - 4, -1), -1):
            m2 = re.search(r'\b(?:bipush|sipush)\s+(\d+)', lines[j])
            if m2:
                val = int(m2.group(1)); break
            m2 = re.search(r'\biconst_(\d)', lines[j])
            if m2:
                val = int(m2.group(1)); break
        events.append((i, 'stacksTo', val))
    elif 'durability' in ln and 'invokevirtual' in ln and 'Properties.durability' in ln:
        events.append((i, 'durability', 1))  # the walk's own 1 (MAX_STACK_SIZE iconst_1)
    elif 'horseArmor' in ln or 'nautilusArmor' in ln:
        events.append((i, 'durability', 1))

table = {}
cur = None
pending = []
for i, kind, val in events:
    if kind == 'field':
        if pending:
            vals = [v for _, v in pending if v is not None]
            if vals:
                table[val] = vals[-1]  # the LAST stacksTo the block spoke
        pending = []
    else:
        pending.append((kind, val))

ones = sorted(k.lower() for k, v in table.items() if v == 1)
six = sorted(k.lower() for k, v in table.items() if v == 16)
other = {k.lower(): v for k, v in table.items() if v not in (1, 16)}
print(f"total fields with explicit stacks: {len(table)}")
print(f"\n== stacksTo(1) [{len(ones)}] ==")
print(' '.join(ones))
print(f"\n== stacksTo(16) [{len(six)}] ==")
print(' '.join(six))
print(f"\n== other ==")
print(other)
