#!/usr/bin/env python3
"""Search helper: reliable text search when grep display mangles output."""
import sys

pattern = sys.argv[1]
paths = sys.argv[2:]
for p in paths:
    with open(p, encoding="utf-8", errors="replace") as f:
        for ln, line in enumerate(f, 1):
            if pattern in line:
                print(f"{p}:{ln}: {line.rstrip()[:220]}")
