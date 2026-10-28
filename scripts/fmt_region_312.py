#!/usr/bin/env python3
"""autopush-312 — THE FMT REGION WALK (the 311 record's own law the
round wears: the parseable fragments the standalone rustfmt the CLEAN
rc=0 verdict banks — the whole-file rustfmt the pre-existing static
noise the HEAD walk confirms).

Exit 0 iff every fragment fmt-checks clean.
"""
import subprocess
import sys
import tempfile
import os

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# (path, start_marker inclusive, end_marker EXCLUSIVE, wrapper)
FRAGMENTS = [
    # THE BED'S OWN SLEEP PURE FNS (the world.rs block the round wrote)
    ("crates/mc-client/src/world.rs",
     "/// THE BED'S OWN USE HEAD CELL (javap BedBlock.useWithoutItem pcs 11-56",
     "/// THE END GATEWAY'S OWN KIND SEAT",
     "top"),
    # THE BED SLEEP TEST (the world.rs unit)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_bed_sleep_walk_the_verdicts_the_folds_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE SLEEPING POSE (the body.rs consts + the height/eye arms)
    ("crates/mc-client/src/body.rs",
     "/// THE SLEEPING POSE'S OWN SIZE (javap the 26.2 jar the Pose.SLEEPING",
     "/// The eye walk's own per-tick blend",
     "top"),
    ("crates/mc-client/src/body.rs",
     "    /// The pose's height (the POSES table's own numbers).",
     "    /// THE CAMERA'S OWN EYE (javap the render's",
     "impl"),
    # THE SLEEP FIELDS (the screens.rs App fields)
    ("crates/mc-client/src/screens.rs",
     "    /// THE BED'S OWN SLEEP BANK (the autopush-312 round — javap 26.2",
     "}",
     "struct"),
    # THE SLEEP DOOR (the right-press arm)
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN SLEEP DOOR (the autopush-312 round — javap 26.2",
     "        // THE CHEST FAMILY'S OWN DOOR",
     "stmt8"),
    # THE SLEEP PRESS + WAKE + TICK (the three fns)
    ("crates/mc-client/src/screens.rs",
     "    /// THE BED'S OWN SLEEP PRESS (the autopush-312 round — javap 26.2",
     "    /// THE BLOCK-USE LANE — the vanilla startUseItem's block branch:",
     "impl"),
    # THE FREEZE ARM (the body tick chain)
    ("crates/mc-client/src/screens.rs",
     "                // THE SLEEP'S OWN FREEZE (the autopush-312 round — javap",
     "                } else if self.net_ride.is_some() {",
     "stmt16"),
    # THE ESC / PRESS / ATTACK GATES
    ("crates/mc-client/src/screens.rs",
     "            // THE SLEEP SCREEN'S OWN ESC (the autopush-312 round — javap",
     "            // THE PAUSE ARM'S OWN RELEASE",
     "stmt12"),
    ("crates/mc-client/src/screens.rs",
     "    pub fn on_press(&mut self, mx: i32, my: i32) {\n        // THE SLEEP SCREEN'S OWN MODAL GATE",
     "        // THE SENSITIVITY SLIDER'S OWN ARM",
     "stmt4"),
    ("crates/mc-client/src/screens.rs",
     "        // THE SLEEP SCREEN'S OWN MODAL GATE (the autopush-312 round —\n        // the InBedChatScreen the sleep walk wears",
     "        // THE SELECTED-KEY GATE'S OWN RIGHT SEAT",
     "stmt8"),
    # THE RENDER (the sleep screen layer + the fn)
    ("crates/mc-client/src/screens.rs",
     "                    // THE SLEEP SCREEN'S OWN LAYER (the autopush-312 round",
     "                    // THE CHAT SCREEN'S OWN LAYER",
     "stmt16"),
    ("crates/mc-client/src/screens.rs",
     "    /// THE SLEEP SCREEN'S OWN RENDER (the autopush-312 round — javap",
     "    /// Vanilla pause menu (PauseScreen): the 26.2 createPauseMenu",
     "impl"),
    # THE CLICK + RESPAWN + DISK rows
    ("crates/mc-client/src/screens.rs",
     "                // THE SLEEP SCREEN'S OWN LEAVE (the autopush-312 round —",
     "                // the DeathScreen's own pair:",
     "stmt16"),
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN RESPAWN RETURN (the autopush-312 round — javap",
     "        if let (Some(b), Some((x, y, z, yaw, pitch))) = (self.body.as_mut(), anchor) {",
     "stmt8"),
    ("crates/mc-client/src/screens.rs",
     "/// THE BED SPAWN'S OWN BANK (the autopush-312 round — the banked",
     "/// THE BREWING BANK'S OWN PARSE",
     "top"),
    ("crates/mc-client/src/screens.rs",
     "            // THE BED SPAWN'S OWN ROW (the autopush-312 round — the",
     "        ]);",
     "stmt12"),
    ("crates/mc-client/src/screens.rs",
     "                // THE BED SPAWN'S OWN PARSE (the autopush-312 round —",
     "                // THE FRAME BANK'S OWN PARSE",
     "stmt16"),
    # THE FOUR PART_11 REAL WALKS
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE BED'S OWN DAY PRESS (the autopush-312 round — javap 26.2",
     "@@EOF@@",
     "top-tail"),
]


def rustfmt_file(name, check):
    cmd = [RUSTFMT] + (["--check"] if check else []) + [name]
    return subprocess.run(cmd, capture_output=True, text=True)


def reindent(text, pads):
    pre = "    " * pads
    return "\n".join((pre + ln) if ln.strip() else ln for ln in text.split("\n"))


def dedent(text, base):
    out = []
    for ln in text.split("\n"):
        out.append(ln[base:] if ln.startswith(" " * base) else ln.lstrip())
    while out and out[-1].strip() == "":
        out.pop()
    return "\n".join(out)


def wrap(kind, frag):
    if kind in ("top", "top-tail"):
        return frag
    if kind == "impl":
        return "struct S;\nimpl S {\n" + frag + "\n}\n"
    if kind == "struct":
        return "struct S {\n" + frag + "\n}\n"
    if kind == "mod":
        return "mod t {\n" + frag + "}\n"
    if kind == "stmt16":
        return "fn f() {\n" + reindent(dedent(frag, 16), 1) + "\n}\n"
    if kind == "stmt12":
        return "fn f() {\n" + reindent(dedent(frag, 12), 1) + "\n}\n"
    if kind == "stmt8":
        return "fn f() {\n" + reindent(dedent(frag, 8), 1) + "\n}\n"
    if kind == "stmt4":
        return "fn f() {\n" + reindent(dedent(frag, 4), 1) + "\n}\n"
    sys.exit(f"unknown wrapper {kind}")


def main():
    bad = 0
    for path, start, end, wrapper in FRAGMENTS:
        text = open(path).read()
        i = text.find(start)
        if i < 0:
            print(f"MISS  {path} :: start not found: {start[:50]!r}")
            bad += 1
            continue
        j = text.find(end, i + len(start)) if end != "@@EOF@@" else len(text)
        if end != "@@EOF@@" and j < 0:
            print(f"MISS  {path} :: end not found: {end[:50]!r}")
            bad += 1
            continue
        frag = text[i:j].rstrip() + "\n"
        frag = wrap(wrapper, frag)
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as f:
            f.write(frag)
            name = f.name
        r = rustfmt_file(name, check=True)
        label = f"{path.rsplit('/', 1)[-1]} :: {start[:44]!r}"
        if r.returncode == 0:
            print(f"CLEAN {label}")
        else:
            print(f"DIRTY {label}\n{r.stdout[:1200]}{r.stderr[:600]}")
            bad += 1
        os.unlink(name)
    print(f"[fmt-region] {'CLEAN' if bad == 0 else f'{bad} DIRTY'}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
