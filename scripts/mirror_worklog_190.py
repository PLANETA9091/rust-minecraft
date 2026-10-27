import os

SRC = "/home/z/my-project/worklog.md"
DST = "/home/z/my-project/rust-minecraft/logs/worklog.md"

data = open(SRC, encoding="utf-8").read()
i = data.rindex("---\nTask ID: dev-190")
entry = data[i:]
assert "Task ID: dev-190" in entry and "0646b778" in entry
with open(DST, "a", encoding="utf-8") as f:
    f.write(entry)
print(f"appended {len(entry)} chars -> {DST}")

d2 = open(DST, encoding="utf-8").read()
j = d2.rindex("---\nTask ID: dev-190")
assert d2[j:] == entry, "mirror tail mismatch"
print(f"verified {DST} rindex={j} len={len(d2)}")
