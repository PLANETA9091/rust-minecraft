#!/usr/bin/env python3
"""leash_481 — the leash rope's own render the jar's own bytes speak
(the JRE-only walk the sandbox wears): the LeashFeatureRenderer's
segment loop the sag fold the color words the rope geometry banks, the
EntityRenderState$LeashState fields the extraction carries, the
Entity/LivingEntity/Mob getRopeHoldPosition + getLeashOffset rows the
attach points speak. Usage: python3 scripts/jar_work_parse_leash_481.py
> scripts/jar_work/leash_481.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
import jar_work_parse_sprites_461 as dumper  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
FULL = [
    "net/minecraft/client/renderer/feature/LeashFeatureRenderer.class",
    "net/minecraft/client/renderer/entity/state/EntityRenderState$LeashState.class",
    "net/minecraft/client/renderer/entity/state/EntityRenderState.class",
    "net/minecraft/client/renderer/entity/EntityRenderer.class",
]
WANTED = {
    "getRopeHoldPosition",
    "getLeashOffset",
    "getLeashHoldRiderOffset",
    "getEyeHeight",
    "getPosition",
}
CLASSES = [
    "net/minecraft/world/entity/Entity.class",
    "net/minecraft/world/entity/LivingEntity.class",
    "net/minecraft/world/entity/Mob.class",
    "net/minecraft/world/entity/Leashable.class",
    "net/minecraft/world/entity/animal/horse/Llama.class",
    "net/minecraft/world/entity/animal/equine/Llama.class",
]


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

        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                parts.append(f"== {cls}: ABSENT")
                continue
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                dumper.dump_class(data, lambda n, d: any(k in n for k in WANTED))
            body = buf.getvalue()
            if body.strip():
                parts.append(f"== {cls}\n{body}")

    out = "\n\n".join(parts)
    print(out, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
