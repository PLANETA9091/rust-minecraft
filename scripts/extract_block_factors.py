#!/usr/bin/env python3
"""Extract the block speed/jump factors (javap Blocks clinit — the
BlockBehaviour$Properties.speedFactor/jumpFactor registration rows the
26.2 jar keeps) into a generated Rust table:
crates/mc-registry/src/generated/block_factors.rs.

The 26.2 clinit speaks exactly TWO factor rows: soul_sand
speedFactor(0.4f) (pcs 10667-10670) and honey_block speedFactor(0.4f) +
jumpFactor(0.5f) (pcs 26877-26885). The makeStuckInBlock family the
older versions carried is RETIRED: SoulSandBlock carries NO
entityInside override in 26.2 (its SHAPE = Block.column(16.0, 0.0,
14.0) the collision's Column row keeps) and HoneyBlock.entityInside
only the slide-down achievements speak (the HalfTransparentBlock
super the walk delegates). Entity.getBlockSpeedFactor /
Entity.getBlockJumpFactor read the property rows; the unmatched block
the 1.0 Properties default rides.

Usage: extract_block_factors.py [clinit_dump.txt]
       (default: javap the client jar at /tmp/mc-verify/client.jar
        via /tmp/mc-verify/jdk/bin/javap — re-downloadable per the
        RESUME block in worklog.md)
"""
import re
import subprocess
import sys
from pathlib import Path

JAR = Path("/tmp/mc-verify/client.jar")
JAVAP = Path("/tmp/mc-verify/jdk/bin/javap")
OUT = Path(__file__).resolve().parent.parent / "crates/mc-registry/src/generated/block_factors.rs"

GETSTATIC_ITEM = re.compile(r"getstatic\s+#\d+\s+// Field net/minecraft/references/BlockItemIds\.(\w+):")
LDC_FLOAT = re.compile(r"ldc(?:_w)?\s+#\d+\s+// float (-?[0-9.e+-]+)f$")
INVOKE = re.compile(r"invokevirtual\s+#\d+\s+// Method net/minecraft/world/level/block/state/BlockBehaviour\$Properties\.(\w+):")
PUTSTATIC = re.compile(r"putstatic\s+#\d+\s+// Field (\w+):Lnet/minecraft/world/level/block/Block;")


def dump_lines() -> list[str]:
    if len(sys.argv) > 1:
        return Path(sys.argv[1]).read_text().splitlines()
    out = subprocess.run(
        [str(JAVAP), "-p", "-c", "-cp", str(JAR), "net.minecraft.world.level.block.Blocks"],
        capture_output=True, text=True, check=True,
    )
    return out.stdout.splitlines()


def main() -> None:
    speeds: dict[str, float] = {}
    jumps: dict[str, float] = {}
    current: str | None = None
    last_float: float | None = None
    for line in dump_lines():
        if m := GETSTATIC_ITEM.search(line):
            current = m.group(1).lower()
            last_float = None
            continue
        if m := LDC_FLOAT.search(line):
            last_float = float(m.group(1))
            continue
        if m := INVOKE.search(line):
            if m.group(1) == "speedFactor" and current and last_float is not None:
                speeds[current] = last_float
            elif m.group(1) == "jumpFactor" and current and last_float is not None:
                jumps[current] = last_float
            last_float = None
            continue
        if PUTSTATIC.search(line):
            current = None
            last_float = None

    assert speeds == {"soul_sand": 0.4, "honey_block": 0.4}, f"speed rows drifted: {speeds}"
    assert jumps == {"honey_block": 0.5}, f"jump rows drifted: {jumps}"

    s_names = sorted(speeds)
    j_names = sorted(jumps)
    parts: list[str] = []
    parts.append("""// GENERATED from the client.jar's Blocks clinit (javap the
// BlockBehaviour$Properties speedFactor/jumpFactor registration rows
// the 26.2 jar keeps) by scripts/extract_block_factors.py — the
// Entity.getBlockSpeedFactor / Entity.getBlockJumpFactor walks' data
// rows: the 26.2 clinit speaks exactly TWO rows (soul_sand
// speedFactor 0.4f pcs 10667-10670; honey_block speedFactor 0.4f +
// jumpFactor 0.5f pcs 26877-26885 — the makeStuckInBlock family the
// older versions carried RETIRED: the SoulSandBlock the entityInside
// override never carries, the HoneyBlock's own the slide-down
// achievements the HalfTransparentBlock super speaks), the unmatched
// block the 1.0 Properties default rides.""")
    parts.append("")
    parts.append(f"pub const SPEED_FACTOR_COUNT: usize = {len(s_names)};")
    parts.append("")
    parts.append("pub const SPEED_FACTOR_NAMES: [&str; SPEED_FACTOR_COUNT] = [")
    parts.extend(f'    "{n}",' for n in s_names)
    parts.append("];")
    parts.append("")
    parts.append("pub const SPEED_FACTOR_VALUES: [f32; SPEED_FACTOR_COUNT] = [")
    parts.append("    " + ", ".join(str(speeds[n]) for n in s_names) + ",")
    parts.append("];")
    parts.append("")
    parts.append(f"pub const JUMP_FACTOR_COUNT: usize = {len(j_names)};")
    parts.append("")
    parts.append("pub const JUMP_FACTOR_NAMES: [&str; JUMP_FACTOR_COUNT] = [")
    parts.extend(f'    "{n}",' for n in j_names)
    parts.append("];")
    parts.append("")
    parts.append("pub const JUMP_FACTOR_VALUES: [f32; JUMP_FACTOR_COUNT] = [")
    parts.append("    " + ", ".join(str(jumps[n]) for n in j_names) + ",")
    parts.append("];")
    parts.append("")
    parts.append("""/// The block's own speed factor (the 1.0 default the unmatched
/// name rides — the Properties default the clinit's silent rows
/// keep); the getBlockSpeedFactor walk's layered read consumes (the
/// feet cell first, the WATER pass-through arm, the below-cell
/// fallback the default-1.0 feet wears).
pub fn speed_factor(name: &str) -> f32 {
    SPEED_FACTOR_NAMES
        .binary_search(&name)
        .ok()
        .map(|i| SPEED_FACTOR_VALUES[i])
        .unwrap_or(1.0)
}

/// The block's own jump factor (the same 1.0 default — the
/// getBlockJumpFactor walk's layered read consumes: the feet cell,
/// the below-cell fallback, NO water arm the jump walk carries).
pub fn jump_factor(name: &str) -> f32 {
    JUMP_FACTOR_NAMES
        .binary_search(&name)
        .ok()
        .map(|i| JUMP_FACTOR_VALUES[i])
        .unwrap_or(1.0)
}""")
    OUT.write_text("\n".join(parts) + "\n")
    print(f"wrote {OUT}: speed {speeds}, jump {jumps}")


if __name__ == "__main__":
    main()
