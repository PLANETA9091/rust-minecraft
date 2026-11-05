#!/usr/bin/env python3
"""stateid_469 — the state-id correction's own max law the jar's own
bytes speak (the javap-less walk the JRE-only sandbox wears): the
AbstractContainerMenu's own re-key walk the SetSlot correction serves —
the setItem the `stateId = Math.max(stateId, stateId + 1)` law wears
[the getfield + iadd + if_icmpge/invokestatic Math.max fold the stale
word the believed revision never regresses], the initializeContents
the EXACT set the wholesale snapshot re-bases [the putfield the direct
word keeps, the Math.max the bulk lane never wears], the broadcastChanges
the counter the tick climb wears. Usage: python3
scripts/jar_work_parse_stateid_469.py > scripts/jar_work/stateid_469.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/inventory/AbstractContainerMenu.class",
]
WANTED = {
    "setItem",
    "initializeContents",
    "getStateId",
    "setStateId",
    "incrementStateId",
    "broadcastChanges",
}


def main():
    buf = io.StringIO()
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                return 1
            print(f"== {cls}")
            with contextlib.redirect_stdout(buf):
                dump_class(data, lambda n, d: any(k in n for k in WANTED))
    text = buf.getvalue()
    print(text, end="")

    # the per-method blocks the fold slices read — the header the
    # slice anchors, the next METHOD header the block closes
    def method_block(sig):
        start = text.find(f"METHOD {sig}")
        if start < 0:
            return ""
        nxt = text.find("\n  METHOD ", start + 1)
        return text[start : nxt if nxt >= 0 else len(text)]

    set_item = method_block("setItem (IILnet/minecraft/world/item/ItemStack;)V")
    init_contents = method_block(
        "initializeContents (ILjava/util/List;Lnet/minecraft/world/item/ItemStack;)V"
    )
    broadcast = method_block("broadcastChanges ()V")

    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak; the 26.2 law the EXACT SET wears:
    # the SetSlot correction the packet's own word the believed
    # revision banks, the wholesale snapshot the same exact word
    # re-bases — the Math.max fold the older builds wore the 26.2
    # bytes never speak, the port's exact-set lanes the census keeps)
    verdicts = [
        ("the setItem walk the census wears (the II ItemStack shape)",
         bool(set_item)),
        ("the setItem the slot write serves (getSlot + Slot.set)",
         "AbstractContainerMenu.getSlot:(I)" in set_item
         and "Slot.set:(Lnet/minecraft/world/item/ItemStack;)V" in set_item),
        ("the setItem the EXACT set banks (putfield stateId)",
         "putfield net/minecraft/world/inventory/AbstractContainerMenu.stateId:I" in set_item),
        ("the setItem the Math.max fold never wears (the 26.2 bytes)",
         "Math.max" not in set_item),
        ("the initializeContents walk the census wears",
         bool(init_contents)),
        ("the bulk the slot walk the List loop wears",
         "java/util/List.size:()I" in init_contents
         and "java/util/List.get:(I)" in init_contents
         and "Slot.set:(Lnet/minecraft/world/item/ItemStack;)V" in init_contents),
        ("the bulk the exact set banks (putfield, no Math.max)",
         "putfield net/minecraft/world/inventory/AbstractContainerMenu.stateId:I" in init_contents
         and "Math.max" not in init_contents),
        ("the broadcastChanges walk the census wears (code=127)",
         "METHOD broadcastChanges ()V" in text),
        ("the broadcastChanges the stateId walk never touches (the counter the packets alone move)",
         bool(broadcast)
         and "AbstractContainerMenu.stateId" not in broadcast),
        ("the getStateId getter the census wears",
         "METHOD getStateId ()I" in text),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
