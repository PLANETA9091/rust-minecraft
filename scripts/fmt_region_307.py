#!/usr/bin/env python3
"""autopush-307 — THE FMT REGION WALK (the 305 record's own law the
round wears: the parseable fragments the standalone rustfmt the CLEAN
rc=0 verdict banks — the whole-file rustfmt the 1761-diff noise the
crate's own static the HEAD walk confirms).

The trailing blanks the extraction's own boundary the strip folds —
the in-region shape the rustfmt verdict reads. The dedent fragment
the format-and-compare the wrapper's own indent the artifact answers.

Exit 0 iff every fragment fmt-checks clean.
"""
import subprocess
import sys
import tempfile
import os

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")

# (path, start_marker inclusive, end_marker EXCLUSIVE, wrapper)
FRAGMENTS = [
    ("crates/mc-client/src/world.rs",
     "/// THE HOPPER'S OWN COOLDOWN LAW",
     "/// THE FAMILY'S OWN VARIANT ENCODE",
     "top"),
    ("crates/mc-client/src/world.rs",
     "    /// THE HOPPER'S OWN SCHEDULE (the autopush-307 round",
     "    /// THE MOVING PISTON'S OWN STASH WALK",
     "impl"),
    ("crates/mc-client/src/world.rs",
     "    fn the_hopper_transfer_the_flat_order_laws_wear() {",
     "    fn the_shulker_cell_the_lids_tick_the_same_ramp_wears() {",
     "mod"),
    ("crates/mc-client/src/screens.rs",
     "    /// THE HOPPER'S OWN TRANSFER BEAT (the autopush-307 round",
     "    /// THE DISPENSER VOICE'S OWN FOLD",
     "impl"),
    ("crates/mc-client/src/screens.rs",
     "            if crate::world::is_hopper(placed) {",
     "            // THE PLACEMENT'S OWN CHAIN",
     "dedent-fn"),
]


def rustfmt_file(name, check):
    cmd = [RUSTFMT] + (["--check"] if check else []) + [name]
    return subprocess.run(cmd, capture_output=True, text=True)


def dedent12(text):
    return "".join(
        ln[12:] if ln.startswith(" " * 12) else ln.lstrip(" ")
        for ln in text.splitlines(keepends=True)
    )


def main():
    ok = True
    for path, start, end, kind in FRAGMENTS:
        ls = open(path).read().splitlines(keepends=True)
        si = ei = None
        for i, ln in enumerate(ls):
            if si is None and start in ln:
                si = i
            elif si is not None and end in ln:
                ei = i
                break
        if si is None or ei is None:
            print(f"FAIL {path} :: markers not found: {start[:50]}")
            ok = False
            continue
        body = "".join(ls[si:ei]).rstrip()
        if kind == "mod":
            if body.endswith("#[test]"):
                body = body[:-len("#[test]")].rstrip()
        body += "\n"
        if kind == "dedent-fn":
            body = dedent12(body)
            wrapper = "fn probe() {\n%s}\n"
        elif kind == "top":
            wrapper = "%s"
        elif kind == "mod":
            wrapper = "mod t {\n%s}\n"
        else:
            wrapper = "struct P;\nimpl P {\n%s}\n"
        wrapped = wrapper % body
        with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as f:
            f.write(wrapped)
            name = f.name
        try:
            if kind == "dedent-fn":
                rustfmt_file(name, False)
                formatted = open(name).read()
                # the wrapper's own 4-space body indent the unwrap folds
                inner = formatted.split("\n", 1)[1].rsplit("}", 1)[0]
                inner = "".join(
                    ln[4:] if ln.startswith("    ") else ln
                    for ln in inner.splitlines(keepends=True)
                )
                if inner == body:
                    print(f"PASS {path} :: {start[:58]} -> CLEAN(dedent)")
                else:
                    ok = False
                    print(f"FAIL {path} :: {start[:58]} (dedent mismatch)")
            else:
                r = rustfmt_file(name, True)
                if r.returncode == 0:
                    print(f"PASS {path} :: {start[:58]} -> CLEAN({kind})")
                else:
                    ok = False
                    print(f"FAIL {path} :: {start[:58]}")
                    print((r.stdout + r.stderr)[:600])
        finally:
            os.unlink(name)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
