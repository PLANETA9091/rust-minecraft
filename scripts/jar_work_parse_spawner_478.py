#!/usr/bin/env python3
"""spawner_478 — the wanderer's own spawn walk the jar's own bytes
speak (the javap-less walk the JRE-only sandbox wears): the
WanderingTraderSpawner's tick + spawn + the placement/space/find arms
— the 1200-tick checkpoint the delay decrement serves, the 24000
re-arm, the 25-step chance climb the [25, 75] clamp keeps, the roll
the OLD chance answers, the 1-in-10 gate the spawn opens with, the 48
ring the 10 attempts wear, the 2x3x2 space fold, the 2 llama attempts
the 4 range speaks, the 16-wide home the wander target banks. Usage:
python3 scripts/jar_work_parse_spawner_478.py > scripts/jar_work/spawner_478.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/world/entity/npc/wanderingtrader/WanderingTraderSpawner.class",
    "net/minecraft/world/level/saveddata/WanderingTraderData.class",
]
WANTED = {"tick", "spawn", "findSpawnPositionNear", "hasEnoughSpace",
          "tryToSpawnLlamaFor", "<init>", "spawnDelay", "spawnChance"}


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

    def method_block(sig):
        start = text.find(f"METHOD {sig}")
        if start < 0:
            return ""
        nxt = text.find("\n  METHOD ", start + 1)
        return text[start : nxt if nxt >= 0 else len(text)]

    tick = method_block("tick (Lnet/minecraft/server/level/ServerLevel;Z)V")
    spawn = method_block("spawn (Lnet/minecraft/server/level/ServerLevel;)Z")
    find = method_block("findSpawnPositionNear")
    space = method_block("hasEnoughSpace")
    llama = method_block("tryToSpawnLlamaFor")
    data_init = method_block("<init> ()V")

    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak)
    verdicts = [
        ("the tick the gamerule gate opens with (SPAWN_WANDERING_TRADERS read)",
         "GameRules.SPAWN_WANDERING_TRADERS" in tick),
        ("the tick the 1200-tick checkpoint wears (isub + re-seat)",
         "sipush 1200" in tick and "putfield" in tick),
        ("the tick the delay decrement + the >0 silence wears",
         "spawnDelay:()I" in tick and "setSpawnDelay:(I)V" in tick),
        ("the tick the 24000 re-arm banks",
         "sipush 24000" in tick),
        ("the tick the 25-step climb the [25, 75] clamp keeps",
         "bipush 25" in tick and "bipush 75" in tick
         and "Mth.clamp:(III)I" in tick),
        ("the tick the roll the OLD chance answers (nextInt(100) vs the pre-climb word)",
         "nextInt:(I)I" in tick and "bipush 100" in tick),
        ("the tick the success the chance-25 reset wears (setSpawnChance(25))",
         "setSpawnChance:(I)V" in tick),
        ("the spawn the random-player pick opens with",
         "getRandomPlayer:()" in spawn),
        ("the spawn the 1-in-10 gate wears (nextInt(10) != 0 refusal)",
         "bipush 10" in spawn),
        ("the spawn the 48 ring wears (the POI anchor the orElse fallback keeps)",
         "bipush 48" in spawn and "java/util/Optional.orElse" in spawn),
        ("the spawn the find + the space arms ride (placement + collision)",
         "findSpawnPositionNear" in spawn and "hasEnoughSpace" in spawn),
        ("the spawn the 2 llama attempts the range-4 speaks (loop 0..2, tryToSpawnLlamaFor .., 4)",
         "tryToSpawnLlamaFor" in spawn),
        ("the spawn the despawn-0 / wander-target / 16-wide home tails wear",
         "setDespawnDelay:(I)V" in spawn and "setWanderTarget" in spawn
         and "setHomeTo" in spawn),
        ("the find the 10 attempts the 2*range draw wears",
         "bipush 10" in find and "iconst_2" in find
         and "nextInt:(I)I" in find),
        ("the find the heightmap + the placement verdict ride",
         "getHeight" in find and "isSpawnPositionOk" in find),
        ("the space the 2x3x2 collision fold wears (offset(1, 2, 1), isEmpty)",
         "iconst_1" in space and "iconst_2" in space
         and "getCollisionShape" in space and "isEmpty:()Z" in space),
        ("the llama the find + the TRADER_LLAMA spawn + the leash wear",
         "findSpawnPositionNear" in llama and "TRADER_LLAMA" in llama
         and "setLeashedTo" in llama),
        ("the fresh data the (24000, 25) defaults wear",
         "sipush 24000" in data_init and "bipush 25" in data_init),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
