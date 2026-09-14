#!/usr/bin/env python3
"""Line-anchored workspace version bump (the js-sys lesson applied).

Bumps the six workspace crates' version in Cargo.toml and the six
matching Cargo.lock entries — BY LINE ANCHOR, never a global sed.
Verifies before writing: exactly the six workspace crates moved, and
the ecosystem crates riding the same band (js-sys, web-sys) are
checked BY NAME to still ride their own version.

Usage: python3 scripts/bump_version.py <new-version>
"""
import re
import sys

NEW = sys.argv[1] if len(sys.argv) > 1 else None
if not NEW or not re.fullmatch(r"\d+\.\d+\.\d+", NEW):
    sys.exit("usage: bump_version.py <major.minor.patch>")

WORKSPACE_CRATES = [
    "mc2rs-gen",
    "mc-protocol",
    "mc-registry",
    "mc-assets",
    "mc-client",
    "mc2rs-pixel",
]
ECOSYSTEM = {"js-sys": "0.3.105", "web-sys": "0.3.105"}

toml_path = "Cargo.toml"
lock_path = "Cargo.lock"

# --- Cargo.toml: the workspace crate's own version line (line-anchored
# to the [package] section's version key) ---
toml = open(toml_path).read().splitlines(keepends=True)
toml_hits = [i for i, l in enumerate(toml) if l.rstrip("\n") == 'version = "0.3.126"']
if len(toml_hits) != 1:
    sys.exit(f"FATAL: expected exactly 1 workspace version line in Cargo.toml, found {len(toml_hits)}")
toml[toml_hits[0]] = f'version = "{NEW}"\n'

# --- Cargo.lock: bump only the version line INSIDE each workspace
# crate's block (name line precedes it); refuse if any crate is missing
# or the ecosystem crates moved ---
lock = open(lock_path).read().splitlines(keepends=True)
name_re = re.compile(r'^name = "(.+)"$')
ver_re = re.compile(r'^version = "(\d+\.\d+\.\d+)"$')
bumped = {}
for i, line in enumerate(lock):
    m = name_re.match(line.rstrip("\n"))
    if m and m.group(1) in WORKSPACE_CRATES:
        # the next version line within this block
        for j in range(i + 1, min(i + 4, len(lock))):
            vm = ver_re.match(lock[j].rstrip("\n"))
            if vm:
                bumped[m.group(1)] = (j, vm.group(1))
                break
missing = [c for c in WORKSPACE_CRATES if c not in bumped]
if missing:
    sys.exit(f"FATAL: workspace crates not found in Cargo.lock: {missing}")
wrong = {c: old for c, (_, old) in bumped.items() if old != "0.3.126"}
if wrong:
    sys.exit(f"FATAL: expected old version 0.3.126, found {wrong}")
if len(set(j for j, _ in bumped.values())) != len(bumped):
    sys.exit("FATAL: two workspace crates resolved to the same lock line")
for (j, _old) in bumped.values():
    lock[j] = f'version = "{NEW}"\n'

# --- ecosystem check BY NAME (after the edit, before writing) ---
eco = {}
for i, line in enumerate(lock):
    m = name_re.match(line.rstrip("\n"))
    if m and m.group(1) in ECOSYSTEM:
        for j in range(i + 1, min(i + 4, len(lock))):
            vm = ver_re.match(lock[j].rstrip("\n"))
            if vm:
                eco[m.group(1)] = vm.group(1)
                break
if eco != ECOSYSTEM:
    sys.exit(f"FATAL: ecosystem crates moved: expected {ECOSYSTEM}, found {eco}")

open(toml_path, "w").write("".join(toml))
open(lock_path, "w").write("".join(lock))
print(f"bumped {len(bumped)} lock entries + Cargo.toml to {NEW}")
print(f"ecosystem verified by name: {eco}")
