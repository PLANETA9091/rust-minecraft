#!/usr/bin/env python3
"""lead_use_486 — the lead item's own mob-tie the jar's own bytes speak
(the JRE-only walk the sandbox wears): the LeadItem.interactLivingEntity
the tie walk the LEAD stack the right-press serves, the Leashable.leashTo
+ canBeLeashed + setLeashedTo rows the interface default banks, the tie
voice the LeadItem's own clinit/sounds row keeps. Usage: python3
scripts/jar_work_parse_lead_use_486.py > scripts/jar_work/lead_use_486.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
import jar_work_parse_sprites_461 as dumper  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
FULL = [
    "net/minecraft/world/item/LeadItem.class",
    "net/minecraft/world/entity/Leashable.class",
    "net/minecraft/sounds/SoundEvents.class",
]
WANTED = {
    "interactLivingEntity",
    "leashTo",
    "canBeLeashed",
    "setLeashedTo",
    "getLeashData",
    "LEAD_TIED",
    "LEAD_BREAK",
    "LEAD_UNTIED",
    "createLeashData",
}


def main():
    parts = []
    with zipfile.ZipFile(JAR) as z:
        for cls in FULL:
            try:
                data = z.read(cls)
            except KeyError:
                parts.append(f"== {cls}: ABSENT")
                continue
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                dumper.dump_class(data, lambda n, d: True)
            parts.append(f"== {cls}\n{buf.getvalue()}")

    out = "\n\n".join(parts)
    print(out, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
