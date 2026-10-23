#!/usr/bin/env python3
"""THE FOV EFFECT SCALE'S OWN PROBE (the autopush-175 round): the
`fovEffectScale` option the 26.2 jar's own law keeps —
1) `Options.<clinit>`/ctor the fovEffectScale OptionInstance the
   options_init_262.txt bank already speaks (the UnitDouble.xmap slider
   the doubleRange(0,1) codec the default 1.0 the dconst_1 answers);
2) `AbstractClientPlayer.getFieldOfViewModifier` the TAIL the consumer
   law keeps — the lerp the option's value speaks (the exact final
   instructions the walk prints);
3) the row's own screen seat — the AccessibilityScreen the tooltip
   constant ACCESSIBILITY_TOOLTIP_FOV_EFFECT names, the createRow the
   options.screenLayout() walk banks.
javap absent — the JRE's own bytecode the python walk reads (the same
law the glass_685 helpers keep).

Usage: python3 scripts/jar_work/fov_scale_262.py <client.jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"

# glass_685.py runs its own probe at module level — exec only the
# helper-function region (the source before its trailing ZipFile open).
_src = open("scripts/jar_work/glass_685.py").read()
_cut = _src.rindex("z = zipfile.ZipFile(JAR)")
glass_ns: dict = {}
exec(compile(_src[:_cut], "glass_685_helpers", "exec"), glass_ns)

methods_of = glass_ns["methods_of"]
walk_code = glass_ns["walk_code"]
utf = glass_ns["utf"]


def dump_method(z, cls, needle):
    ms = methods_of(z, cls)
    if ms is None:
        print(f"!! class not found: {cls}")
        return None
    hits = {sig: (cp, body) for sig, (cp, body) in ms.items() if needle in sig}
    if not hits:
        print(f"-- no '{needle}' in {cls}; methods:")
        for sig in sorted(ms):
            print("   ", sig)
        return None
    for sig, (cp, body) in hits.items():
        print(f"== {cls} :: {sig}")
        if body is None:
            print("   (no Code attribute)")
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        for line in walk_code(cp, code):
            print("   ", line)
    return True


def main():
    z = zipfile.ZipFile(JAR)
    # 1) the consumer's own tail
    dump_method(z, "net.minecraft.client.player.AbstractClientPlayer", "FieldOfView")
    # fallback: any fov-ish name
    ms = methods_of(z, "net.minecraft.client.player.AbstractClientPlayer")
    if ms:
        print("== AbstractClientPlayer method names (fov/zoom-ish):")
        for sig in sorted(ms):
            if "fov" in sig.lower() or "zoom" in sig.lower() or "1087" in sig:
                print("   *", sig)
    # 2) the row's own seat: the AccessibilityScreen walk
    for cls in (
        "net.minecraft.client.gui.screens.options.AccessibilityScreen",
        "net.minecraft.client.gui.screens.options.OptionsSubScreen",
    ):
        dump_method(z, cls, "init")


if __name__ == "__main__":
    main()
