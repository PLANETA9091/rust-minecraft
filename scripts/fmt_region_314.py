#!/usr/bin/env python3
"""autopush-314 — THE FMT REGION WALK (the 313 record's own law the
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
    # THE ANCHOR'S OWN PURE FNS (the world.rs block the round wrote —
    # the kind seat, the charge folds, the verdict arms, the offset
    # tables, the stand-up scan, the static walk, the dampening scan)
    ("crates/mc-client/src/world.rs",
     "/// THE BED'S OWN KIND SEAT (the KIND_NAMES tail's row the vanilla\n/// registration order keeps — the kind id 798",
     "/// THE END GATEWAY'S OWN KIND SEAT",
     "top"),
    # THE ANCHOR'S OWN PRESS (the screens.rs lane — the charge arm +
    # the verdict walk the press serves)
    ("crates/mc-client/src/screens.rs",
     "    /// THE ANCHOR'S OWN PRESS (javap 26.2 RespawnAnchorBlock the live",
     "    fn bed_sleep_press(&mut self, pos: (i32, i32, i32)) -> bool {",
     "impl"),
    # THE ANCHOR'S OWN DOOR (the block lane's arm)
    ("crates/mc-client/src/screens.rs",
     "        // THE ANCHOR'S OWN DOOR (the autopush-314 round — javap 26.2",
     "        // THE CHEST FAMILY'S OWN DOOR (the dev-64 round",
     "stmt8"),
    # THE ANCHOR'S OWN BOOMS (the tick lane's application walk)
    ("crates/mc-client/src/screens.rs",
     "                        // THE ANCHOR'S OWN BOOMS (the autopush-314 round —",
     "                        for (sx, sy, sz) in tnt_smokes {",
     "stmt24"),
    # THE RESPAWN ANCHOR ARM (the respawn lane's composition — the
    # whole let-statement the balance keeps)
    ("crates/mc-client/src/screens.rs",
     "        let anchor = match (self.offline_world.as_ref(), self.respawn_point) {",
     "        if let (Some(b), Some((x, y, z, yaw, pitch))) = (self.body.as_mut(), anchor) {",
     "stmt8"),
    # THE ANCHOR'S OWN BOOM QUEUE FIELD (the struct row)
    ("crates/mc-client/src/screens.rs",
     "    /// THE ANCHOR'S OWN BOOM QUEUE (the autopush-314 round — javap",
     "    /// THE CART RIDE'S OWN ROTATION ANCHOR",
     "struct"),
    # THE ANCHOR UNIT (the world.rs test)
    ("crates/mc-client/src/world.rs",
     "    #[test]\n    fn the_anchor_the_charge_the_verdict_the_scan_the_static_wear() {",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE TWO PART_11 REAL WALKS (the round's own tail)
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "// THE ANCHOR'S OWN CHARGE-EXPLODE WALK (the autopush-314 round — javap",
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
    # THE FRAGMENT-BOUND TRIM (the tail's blank lines the wrapper's own
    # closing brace would double — the rustfmt the stripped frag the
    # CLEAN verdict banks)
    frag = frag.rstrip("\n \t")
    if kind in ("top", "top-tail"):
        return frag + "\n"
    if kind == "impl":
        return "struct S;\nimpl S {\n" + frag + "\n}\n"
    if kind == "struct":
        return "struct S {\n" + frag + "\n}\n"
    if kind == "mod":
        return "mod t {\n" + frag + "\n}\n"
    if kind == "stmt8":
        return "fn f() {\n" + reindent(dedent(frag, 8), 1) + "\n}\n"
    if kind == "stmt12":
        return "fn f() {\n" + reindent(dedent(frag, 12), 1) + "\n}\n"
    if kind == "stmt16":
        return "fn f() {\n" + reindent(dedent(frag, 16), 1) + "\n}\n"
    if kind == "stmt24":
        return "fn f() {\n" + reindent(dedent(frag, 24), 1) + "\n}\n"
    sys.exit(f"unknown wrapper {kind}")


def main() -> int:
    ok = True
    for path, start, end, kind in FRAGMENTS:
        text = open(path).read()
        i = text.find(start)
        if i < 0:
            print(f"FAIL {path}: start marker missing")
            ok = False
            continue
        j = text.find(end, i) if end != "@@EOF@@" else len(text)
        if j < 0:
            print(f"FAIL {path}: end marker missing")
            ok = False
            continue
        frag = text[i:j]
        wrapped = wrap(kind, frag)
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as f:
            f.write(wrapped)
            tmp = f.name
        r = rustfmt_file(tmp, True)
        os.unlink(tmp)
        tag = f"{path.split('/')[-1]}[{start.strip()[:44]}...]"
        if r.returncode == 0:
            print(f"CLEAN {tag}")
        else:
            ok = False
            print(f"DIRTY {tag}")
            print(r.stdout[-800:] or r.stderr[-800:])
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
