import os

SRC = "/home/z/my-project/worklog.md"
DST = "/home/z/my-project/rust-minecraft/logs/worklog.md"

data = open(SRC, encoding="utf-8").read()
i = data.rindex("---\nTask ID: dev-180")
entry = data[i:]
assert "Task ID: dev-180" in entry and "25d2a3ba" in entry
with open(DST, "a", encoding="utf-8") as f:
    f.write(entry)
print(f"appended {len(entry)} chars -> {DST}")

d2 = open(DST, encoding="utf-8").read()
j = d2.rindex("---\nTask ID: dev-180")
assert d2[j:] == entry, "mirror tail mismatch"
print(f"verified {DST} rindex={j} len={len(d2)}")
