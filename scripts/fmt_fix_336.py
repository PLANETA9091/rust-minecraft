#!/usr/bin/env python3
"""fmt fix: rustfmt-format the two dirty 336 regions (B the gui text
fns + D the render label lane) and paste back (the 335 walk's own
lesson the wrapper-aware strip keeps — the out[1:-1] the WRAPPED
regions alone serves)."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fix_336.rs")

GUI = "crates/mc-client/src/gui.rs"
SCREENS = "crates/mc-client/src/screens.rs"


def load_lines(path):
    return open(path).read().split("\n")


def find(lines, marker, start=0):
    for i in range(start, len(lines)):
        if marker in lines[i]:
            return i
    raise SystemExit(f"marker not found: {marker}")


def trim(body):
    while body and body[-1].strip() == "":
        body.pop()
    return body


def dedent(body, n):
    out = []
    for ln in body:
        out.append(ln[n:] if ln.startswith(" " * n) else ln)
    return out


def reformat_wrapped(body, wrapper):
    text = "\n".join(wrapper(body))
    open(SCRATCH, "w").write(text + "\n")
    r = subprocess.run([RUSTFMT, "--edition", "2021", SCRATCH],
                       capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:2000]}")
    out = open(SCRATCH).read().split("\n")
    while out and out[-1].strip() == "":
        out.pop()
    os.unlink(SCRATCH)
    return out[1:-1]  # the wrapper-aware strip


gui = load_lines(GUI)
scr = load_lines(SCREENS)

# B: the sga fns (the impl item the 4-space level keeps)
b0 = find(gui, "THE SGA TEXT'S OWN WIDTH")
b1 = find(gui, "/// Draw text with `", b0)
new_b = reformat_wrapped(trim(gui[b0:b1]), lambda b: ["impl Fix336 {"] + b + ["}"])

# D: the render label lane (the 8-space statements the fn wrapper wears)
d0 = find(scr, "LABEL'S OWN SEED (the dev-336")
d1 = find(scr, "THE CENTERED TITLE", d0)
new_d = reformat_wrapped(dedent(trim(scr[d0:d1]), 4),
                         lambda b: ["fn fix336() {"] + b + ["}"])

# paste back (bottom-up so the indices hold)
scr[d0:d1] = [ln if ln.startswith(" " * 4) or not ln.strip() else ln for ln in new_d]
gui[b0:b1] = new_b

open(GUI, "w").write("\n".join(gui))
open(SCREENS, "w").write("\n".join(scr))
print(f"B: {b1 - b0} -> {len(new_b)} lines; D: {d1 - d0} -> {len(new_d)} lines")
