#!/usr/bin/env python3
"""fmt fix for the dev-199 round's dirty regions (the 335 harness law —
rustfmt the extracted unit, paste the formatted body back: the region
law the probe's verdict banks). Regions: screens.rs [bottle_fill_lane,
click_brewing_slot] + part_11 [brew_loop, quickmove, disk tests]."""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fix_199.rs")


def reformat(body, wrapper):
    """rustfmt the body inside the wrapper, return inner lines."""
    # THE BODY JOINS THE WRAPPER (the 335 law — the constant-list bug
    # the blank-splice destruction taught: the wrapper the HEAD+TAIL
    # pair wears, the body the middle keeps)
    wrap = wrapper if callable(wrapper) else (
        lambda b: [wrapper[0]] + b + list(wrapper[1:])
    )
    text = "\n".join(wrap(body))
    open(SCRATCH, "w").write(text + "\n")
    r = subprocess.run([RUSTFMT, "--edition", "2021", SCRATCH],
                       capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"rustfmt failed: {r.stderr[:2000]}")
    out = open(SCRATCH).read().split("\n")
    while out and out[-1].strip() == "":
        out.pop()
    inner = out[1:-1]
    os.unlink(SCRATCH)
    return inner


def fix_file(path, units):
    lines = open(path).read().split("\n")
    # process in reverse order so the earlier splices never shift the
    # later markers
    spans = []
    for marker, wrapper, dedent in units:
        f = next(i for i, l in enumerate(lines) if marker in l)
        start = f
        while start > 0 and (lines[start - 1].lstrip().startswith("///")
                             or lines[start - 1].strip().startswith("#[test]")):
            start -= 1
        indent = len(lines[f]) - len(lines[f].lstrip())
        end = f
        depth = 0
        seen_open = False
        while end < len(lines):
            ln = lines[end]
            if not seen_open and "{" in ln:
                seen_open = True
            depth += ln.count("{") - ln.count("}")
            if seen_open and depth == 0 and ln.rstrip().endswith("}"):
                break
            end += 1
        spans.append((start, end + 1, wrapper, dedent))
    for start, end, wrapper, dedent in sorted(spans, reverse=True):
        body = lines[start:end]
        while body and body[-1].strip() == "":
            body.pop()
        # THE SANITY GATES (the blank-splice destruction the guards
        # kill: the span the real unit wears, the output the same
        # magnitude keeps — any violation ABORTS, nothing writes)
        if len(body) < 15:
            raise SystemExit(f"GUARD: span too small at lines {start}..{end}: {len(body)}")
        inner = reformat(body, wrapper)
        if len(inner) < 15 or abs(len(inner) - len(body)) > max(30, len(body) // 2):
            raise SystemExit(
                f"GUARD: reformat size drift at {start}..{end}: {len(body)} -> {len(inner)}"
            )
        joined = "\n".join(inner)
        for probe in ("fn ", "assert"):
            if probe in "\n".join(body) and probe not in joined:
                raise SystemExit(f"GUARD: content lost at {start}..{end} ('{probe}' gone)")
        if dedent:
            inner = [ln[4:] if ln.startswith("    ") else ln for ln in inner]
        lines[start:end] = inner
    open(path, "w").write("\n".join(lines))
    print(f"fixed {path}: {len(units)} regions")


fix_file("crates/mc-client/src/screens.rs", [
    ("fn on_right_press_bottle_fill_lane(&mut self", ["impl App {", "}"], False),
    ("fn click_brewing_slot(&mut self", ["impl App {", "}"], False),
])
fix_file("crates/mc-client/src/screens/tests/part_11.rs", [
    ("fn the_offline_bottle_the_fill_the_brew_loop_wears",
     ["mod fix199 {", "    use super::*;", "}"], True),
    ("fn the_offline_brewing_the_quickmove_the_kind_hands_over",
     ["mod fix199 {", "    use super::*;", "}"], True),
    ("fn the_player_rows_the_potion_kinds_the_disk_wears",
     ["mod fix199 {", "    use super::*;", "}"], True),
])
