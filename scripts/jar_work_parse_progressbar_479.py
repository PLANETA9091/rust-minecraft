#!/usr/bin/env python3
"""progressbar_479 — the trade window's own progress bar the jar's own
bytes speak (the javap-less walk the JRE-only sandbox wears): the
MerchantScreen.extractProgressBar's gates + the three sprite blits +
the VillagerData's NEXT_LEVEL_XP_THRESHOLDS table + the
showProgressBar overrides the trader's silence wears. Usage: python3
scripts/jar_work_parse_progressbar_479.py > scripts/jar_work/progressbar_479.txt"""
import contextlib
import io
import sys
import zipfile

sys.path.insert(0, "scripts")
from jar_work_parse_sprites_461 import dump_class  # noqa: E402

JAR = ".mc2rs/classpath/client.jar"
CLASSES = [
    "net/minecraft/client/gui/screens/inventory/MerchantScreen.class",
    "net/minecraft/world/entity/npc/villager/VillagerData.class",
    "net/minecraft/world/entity/npc/wanderingtrader/WanderingTrader.class",
]
WANTED = {"extractProgressBar", "extractContents", "<clinit>", "getMinXpPerLevel", "getMaxXpPerLevel",
          "canLevelUp", "showProgressBar"}


def main():
    parts = []
    with zipfile.ZipFile(JAR) as z:
        for cls in CLASSES:
            try:
                data = z.read(cls)
            except KeyError:
                print(f"== {cls}: ABSENT")
                return 1
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                dump_class(data, lambda n, d: any(k in n for k in WANTED))
            block = f"== {cls}\n{buf.getvalue()}"
            parts.append(block)
            print(block, end="")
    text = "\n".join(parts)

    def class_block(cls_name):
        start = text.find(f"== {cls_name}")
        nxt = text.find("\n== ", start + 1)
        return text[start:nxt if nxt > 0 else len(text)]

    screen = class_block("net/minecraft/client/gui/screens/inventory/MerchantScreen.class")
    vdata = class_block("net/minecraft/world/entity/npc/villager/VillagerData.class")
    trader = class_block("net/minecraft/world/entity/npc/wanderingtrader/WanderingTrader.class")

    def method_block(src, sig):
        start = src.find(f"METHOD {sig}")
        if start < 0:
            return ""
        nxt = src.find("\n  METHOD ", start + 1)
        return src[start:nxt if nxt >= 0 else len(src)]

    pb = method_block(screen, "extractProgressBar")
    clinit = method_block(screen, "<clinit>")
    contents = method_block(screen, "extractContents")
    vclinit = method_block(vdata, "<clinit>")
    min_xp = method_block(vdata, "getMinXpPerLevel")
    max_xp = method_block(vdata, "getMaxXpPerLevel")
    level_up = method_block(vdata, "canLevelUp")
    trader_show = method_block(trader, "showProgressBar")

    # THE ROUND'S OWN CENSUS (the verdicts the worklog banks — each the
    # dump's own bytes must speak)
    verdicts = [
        ("the bar the level >= 5 silence opens with (iconst_5 if_icmplt gate)",
         "iconst_5" in pb and "if_icmplt" in pb),
        ("the background blit the (136, 16) seat + the 102x5 size wears",
         "sipush 136" in pb and "bipush 16" in pb and "bipush 102" in pb
         and "iconst_5" in pb and "EXPERIENCE_BAR_BACKGROUND_SPRITE" in pb),
        ("the xp gates the min table + the canLevelUp pair speak",
         "getMinXpPerLevel" in pb and "canLevelUp" in pb),
        ("the fill the 102.0 scale the floor + the min cap wear",
         "Mth.floor" in pb and "java/lang/Math.min:(II)I" in pb),
        ("the current blit the windowed (0, 0) UV + the fill width wears",
         "EXPERIENCE_BAR_CURRENT_SPRITE" in pb and "iconst_0" in pb),
        ("the future xp the getFutureTraderXp word reads",
         "getFutureTraderXp" in pb),
        ("the result blit the fill-forward UV + the +fill seat wears",
         "EXPERIENCE_BAR_RESULT_SPRITE" in pb and "iload 10" in pb
         and "iadd" in pb),
        ("the caller the showProgressBar gate + the shopItem offer pass",
         "showProgressBar:()Z" in contents and "shopItem:I" in contents
         and "extractProgressBar" in contents),
        ("the xp table the [0, 10, 70, 150, 250] thresholds wear",
         "bipush 10" in vclinit and "bipush 70" in vclinit
         and "sipush 150" in vclinit and "sipush 250" in vclinit),
        ("the min/max reads the level-1 and level-index pairs speak",
         "iconst_1" in min_xp and "isub" in min_xp
         and "iaload" in max_xp),
        ("the canLevelUp the [1, 5) band wears",
         "iconst_1" in level_up and "iconst_5" in level_up),
        ("the trader the showProgressBar FALSE override wears (the bar the trader never shows)",
         "iconst_0" in trader_show),
    ]
    rc = 0
    for label, ok in verdicts:
        print(f"VERDICT [{'ok' if ok else 'MISSING'}] {label}")
        if not ok:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
