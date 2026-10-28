#!/usr/bin/env python3
"""autopush-313 — THE FMT REGION WALK (the 312 record's own law the
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
    # THE BED'S OWN RESPAWN RETURN PURE FNS (the world.rs block the
    # round wrote — the dangerous/generic/invalid reads, the clockwise
    # ring, the yaw quadrant, the side pick, the offset tables, the
    # floor walk, the bbox pair, the dismount, the offset scan, the
    # bunk lane, the top walk, the static walk)
    ("crates/mc-client/src/world.rs",
     "// ===========================================================================\n// THE BED'S OWN RESPAWN RETURN WALK (the autopush-313 round",
     "/// THE END GATEWAY'S OWN KIND SEAT",
     "top"),
    # THE BED'S 9-PX COLLISION ROW (the collision_shape_at row — the
    # fn body's own 8-indent statements)
    ("crates/mc-client/src/world.rs",
     "    // THE BED'S OWN 9-PX ROW (the autopush-313 round — javap BedBlock",
     "    // THE TRAPDOOR'S OWN VARIANT-AWARE ROW (the autopush-668 round —",
     "stmt4"),
    # THE RESPAWN WALK UNIT (the world.rs test)
    ("crates/mc-client/src/world.rs",
     "    /// THE BED'S OWN RESPAWN RETURN (the autopush-313 round — javap",
     "    #[test]\n    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    # THE RESPAWN RETURN (the screens.rs respawn walk)
    ("crates/mc-client/src/screens.rs",
     "        // THE BED'S OWN RESPAWN RETURN (the autopush-313 round — javap",
     "        if let (Some(b), Some((x, y, z, yaw, pitch))) = (self.body.as_mut(), anchor) {",
     "stmt8"),
    # THE THREE PART_11 REAL WALKS (the round's own tail)
    ("crates/mc-client/src/screens/tests/part_11.rs",
     "/// THE BED'S OWN RESPAWN RETURN (the autopush-313 round — javap 26.2",
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
    apply = "--apply" in sys.argv
    bad = 0
    for path, start, end, wrapper in FRAGMENTS:
        text = open(path).read()
        i = text.find(start)
        if i < 0:
            print(f"FRAG MISSING start: {start[:60]!r} in {path}")
            bad += 1
            continue
        j = text.find(end, i + len(start)) if end != "@@EOF@@" else len(text)
        if end != "@@EOF@@" and j < 0:
            print(f"FRAG MISSING end: {end[:60]!r} in {path}")
            bad += 1
            continue
        frag = text[i:j]
        wrapped = wrap(wrapper, frag)
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as tf:
            tf.write(wrapped)
            name = tf.name
        try:
            r = rustfmt_file(name, check=True)
            if r.returncode == 0:
                print(f"[CLEAN] {path} :: {start[:48]!r}...")
            elif apply:
                subprocess.run([RUSTFMT, name], check=True)
                fixed = open(name).read()
                # unwrap: strip the wrapper's own lines, restore the indent
                if wrapper == "top" or wrapper == "top-tail":
                    new_frag = fixed
                elif wrapper == "mod":
                    body = fixed[len("mod t {\n"):-len("}\n")]
                    new_frag = body
                elif wrapper == "stmt16":
                    body = fixed[len("fn f() {\n"):-len("}\n")]
                    new_frag = reindent(dedent(body, 1), 16)
                elif wrapper == "stmt12":
                    body = fixed[len("fn f() {\n"):-len("}\n")]
                    new_frag = reindent(dedent(body, 1), 12)
                elif wrapper == "stmt8":
                    body = fixed[len("fn f() {\n"):-len("}\n")]
                    new_frag = reindent(dedent(body, 1), 8)
                elif wrapper == "stmt4":
                    body = fixed[len("fn f() {\n"):-len("}\n")]
                    new_frag = reindent(dedent(body, 1), 4)
                else:
                    sys.exit(f"apply unhandled wrapper {wrapper}")
                if not new_frag.endswith("\n"):
                    new_frag += "\n"
                open(path, "w").write(text[:i] + new_frag + text[j:])
                print(f"[APPLIED] {path} :: {start[:48]!r}...")
            else:
                bad += 1
                print(r.stdout[:2000])
                print(f"[DIFF] {path} :: {start[:48]!r}...")
        finally:
            os.unlink(name)
    if apply:
        print("FMT REGION APPLY: pass complete (rerun without --apply to verify)")
        return 0
    if bad:
        print(f"FMT REGION: {bad} fragment(s) DIFF")
        return 1
    print("FMT REGION: all fragments CLEAN")
    return 0


if __name__ == "__main__":
    sys.exit(main())
