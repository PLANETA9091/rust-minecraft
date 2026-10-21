#!/usr/bin/env python3
"""Extract the explosion lane's DECAY rows (the loot JSONs the 26.2
client jar's data/ carries) into the generated Rust table:
crates/mc-registry/src/generated/block_explosion_decay.rs — the blocks
whose loot tables wear the minecraft:explosion_decay function (the
ApplyExplosionDecay survival roll the DESTROY_WITH_DECAY interaction
arms: nextFloat() <= 1/radius per count unit — the radius the
EXPLOSION_RADIUS loot param the onExplosionHit walk passes when the
block's own dropFromExplosion answers true; the default Block class's
own the iconst_1 answer keeps).

The blast RESISTANCE table rides the runtime dumper's own lane
(`cargo run -p mc2rs-gen -- dump-block-resistance --version 26.2` —
the bootstrapped registry's own query); the decay rows the loot JSONs
the same jar speaks, the clean-room parse the gen_block_drops.py
precedent keeps.

Usage: extract_block_explosion_decay.py
"""
import re
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
JAR = ROOT / ".mc2rs/classpath/client.jar"
OUT_DECAY = ROOT / "crates/mc-registry/src/generated/block_explosion_decay.rs"


def main() -> None:
    decay: list[str] = []
    with zipfile.ZipFile(JAR) as jar:
        for name in jar.namelist():
            if not name.startswith("data/minecraft/loot_table/blocks/"):
                continue
            if not name.endswith(".json"):
                continue
            if "minecraft:explosion_decay" in jar.read(name).decode("utf-8", "replace"):
                decay.append(name.rsplit("/", 1)[1][:-5])
    assert "iron_ore" in decay, "the ore rows the decay wears drifted"
    assert "stone" not in decay, "the cobblestone arm the decay never wore"
    decay.sort()

    parts = [
        "// GENERATED from the client.jar's data/minecraft/loot_table/blocks/"
        "\n// JSONs by scripts/extract_block_explosion_decay.py — the blocks"
        "\n// whose loot tables wear the minecraft:explosion_decay function"
        "\n// (the ApplyExplosionDecay survival roll the DESTROY_WITH_DECAY"
        "\n// interaction arms: nextFloat() <= 1/radius per count unit — the"
        "\n// radius the EXPLOSION_RADIUS loot param the onExplosionHit walk"
        "\n// passes when the block's own dropFromExplosion answers true)."
        "\n",
        f"pub const DECAY_COUNT: usize = {len(decay)};",
        "",
        "pub const DECAY_NAMES: [&str; DECAY_COUNT] = [",
    ]
    for name in decay:
        parts.append(f'    "{name}",')
    parts.append("];")
    parts.append("")
    parts.append("""/// The block's own decay verdict (the false default the
/// function-less tables keep — the drops the explosion wears FULLY).
pub fn has_explosion_decay(name: &str) -> bool {
    DECAY_NAMES.binary_search(name).is_ok()
}""")
    OUT_DECAY.write_text("\n".join(parts) + "\n")

    print(f"decay: {len(decay)} rows")


if __name__ == "__main__":
    main()
