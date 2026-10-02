#!/usr/bin/env python3
"""jar-wide constant pool scan — lists the classes whose constant pool
references a given symbol (class name / utf8 needle). The cp_grep tool the
container reset lost, restored from the full_disasm parser bank.

usage: python3 scripts/cp_grep.py <jar> <needle> [--class-only]
"""
import sys
import zipfile

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp, utf8  # noqa: E402


def class_refs(jar_path: str, needle: str) -> list[str]:
    """The classes whose CP strings mention the needle."""
    hits = []
    with zipfile.ZipFile(jar_path) as jar:
        for name in jar.namelist():
            if not name.endswith(".class"):
                continue
            data = jar.read(name)
            cp, _ = parse_cp(data)
            pool = set()
            for idx in range(1, len(cp) + 1):
                e = cp.get(idx)
                if not e:
                    continue
                tag, val = e[0], e[1] if len(e) > 1 else None
                if tag == "utf8":
                    pool.add(val)
                elif tag == 7:  # class → utf8 idx
                    pool.add(utf8(cp, val))
            if any(needle in s for s in pool):
                hits.append(name)
    return hits


def main() -> None:
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)
    jar_path, needle = sys.argv[1], sys.argv[2]
    hits = class_refs(jar_path, needle)
    for h in hits:
        print(h)
    print(f"--- {len(hits)} classes reference {needle!r}", file=sys.stderr)


if __name__ == "__main__":
    main()
