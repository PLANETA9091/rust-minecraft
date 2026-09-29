#!/usr/bin/env python3
"""Jar-wide constant-pool scan: which classes reference the chat-draft walk
(saveAsDraft / discardDraft / latestDraft / preserveCurrentChatScreen /
restoreChatScreen / isDraftRestorable). Prints class -> refs.

Usage: python3 scripts/draft_ref_scan_49.py <jar>
"""
import struct, sys, zipfile

TARGETS = (
    "saveAsDraft", "discardDraft", "latestDraft", "preserveCurrentChatScreen",
    "restoreChatScreen", "isDraftRestorable", "Draft", "keepDraft",
)


def utf8_at(data, idx, cp):
    tag, off, ln = cp[idx]
    if tag != 1:
        return None
    return data[off:off + ln].decode("utf-8", "replace")


def scan(data, name, hits):
    # minimal cp walk: collect ALL utf8 constants + methodref nameAndTypes
    if data[0:4] != b"\xca\xfe\xba\xbe":
        return
    u2 = lambda p: struct.unpack_from(">H", data, p)[0]
    p = 8
    count = u2(p); p += 2
    cp = {}
    i = 1
    while i < count:
        tag = data[p]; p += 1
        if tag == 1:
            ln = u2(p); p += 2
            cp[i] = (tag, p, ln); p += ln
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, p, 0); p += 2
        elif tag in (15,):
            cp[i] = (tag, p, 0); p += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            cp[i] = (tag, p, 0); p += 4
        elif tag in (5, 6):
            cp[i] = (tag, p, 0); p += 8; i += 1
        else:
            cp[i] = (tag, p, 0); p += 4  # best effort
        i += 1
    strs = set()
    for i, (tag, off, ln) in cp.items():
        if tag == 1:
            s = data[off:off + ln].decode("utf-8", "replace")
            strs.add(s)
    refed = [t for t in TARGETS if t in strs]
    if refed:
        hits.append((name, refed))


def main():
    hits = []
    with zipfile.ZipFile(sys.argv[1]) as z:
        for n in z.namelist():
            if n.endswith(".class"):
                scan(z.read(n), n[:-6], hits)
    for name, refed in hits:
        print(name, "->", ",".join(refed))
    print("TOTAL CLASSES WITH DRAFT REFS:", len(hits))


if __name__ == "__main__":
    main()
