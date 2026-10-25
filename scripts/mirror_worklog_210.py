import os

ENTRY = """---
Task ID: autopush-210
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 11:33:12 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251133): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD cf1b533 (autopush-209 mirror), clean tree, origin synced — the live log the compression lag the re-check corrects (autopush-208 v0.3.958 F3+T reload resource packs + dev-171 v0.3.959 debug crash hold + autopush-209 v0.3.960 F3+N cycle spectator already banked, the worklog's last autopush entry 209, this round 210).
- THE ROUND'S OWN PICK: the ADVANCED TOOLTIPS' OWN RENDER (the autopush-209 RESUME PLAN's first candidate) — the survival inventory's hovered slot the tooltip the F3+H flag gates.
- PROBE: scripts/jar_work/advtooltip_210.py (chained off controls_185.py helpers) -> the FOUR-class chain the walk banks: AbstractContainerScreen.extractTooltip (pc 0-17 the `hoveredSlot == null || !hoveredSlot.hasItem()` return, pc 27-46 the CARRIED WALL — the `!menu.getCarried().isEmpty() && showTooltipWithItemInHand(stack)` walk the image-less stacks the Optional.empty().orElse(false) FALSE answer keeps), Screen.getTooltipFromItem (pc 13-31 the `options.advancedItemTooltips ? TooltipFlag.Default.ADVANCED : NORMAL` ternary — the F3+H chord's own field the flag IS; the static the Methodref-packed-int resolve the subclass citation's superclass home finds), ItemStack.getTooltipLines (pc 61-69 the getStyledHoverName FIRST row, pc 88-93 the addDetailsToTooltip delegate) + addDetailsToTooltip (pc 533-540 the `tooltipFlag.isAdvanced()` gate ifeq->678, pc 543-601 the DURABILITY row the `isDamaged() && display.shows(DAMAGE)` gate keeps — the `item.durability = Durability: %s / %s` en_us row the WHITE withStyle-less arm wears, pc 602-632 the ID row the `literal(BuiltInRegistries.ITEM.getKey(item).toString()).withStyle(DARK_GRAY)` law — the Item.toString pc walk the `wrapAsHolder().getRegisteredName()` answers [the `minecraft:<name>` word the registry id wears], pc 633-673 the COMPONENTS row the `components.size() > 0` gate — the `item.components = %s component(s)` row), DefaultTooltipPositioner (the base the `new Vector2i(mouseX, mouseY).add(12, -12)` pc 4-14 keeps, the right overflow the `max(x - 24 - tooltipWidth, 4)` flip the pc 11-26 arm speaks, the bottom the `tooltipHeight + 3` gap the pc 29-51 arm holds).
- IMPL: screens.rs item_tooltip_rows (the static — the name row the item_display_name stand-in keeps, the ID row the advanced flag banks, the durability + components rows the model's own walls keep silent [the isDamaged + component-map gates the (id, count) data never serve]), item_tooltip_origin (the static — the positioner's own law the three arms wear), item_tooltip_hover (the gate — the inventory_hover seat walk + the round's own stack belief [the hotbar row the HUD belief, the main rows the window-0 projection the live lane speaks, the offline store the local truth] + the hasItem gate + the carried wall), render_item_tooltip (the pass — the mouse_pos None the headless law keeps, the panel the renderTooltipBackground law paints [the 3px margins, the 9px row pitch], the rows the g.text rides); the call the render_inventory_slots tail keeps (the carried stack and the bundle tooltip the neighbors the layer order wears). The goldens never see this pass (the mouse-driven lane the headless scenes never wear).
- TEST: part_4.rs the trio (the rows the flag wears — the name row the normal keeps, the `minecraft:dirt` DARK_GRAY the advanced banks, the durability + components walls the tail holds; the origin the +12/-12 base the 38-room flip the max-4 clamp the bottom +3 gap the four seats speak — the flip math the first draft's (4,188) slip the (38,188) answer corrects, the max-4 wall the narrow-screen seat banks; the gate the filled seat the hasItem wall the carried wall the chord-flip ID row the walks wear) — part_4 168 passed (165 +3).
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the mouse-driven pass the headless scenes never see — the fixed scenes the tooltip never wears).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the part_4 10419 clip_crosshair block + the screens.rs 39868 spectator-error-word block BOTH the HEAD's own drift the stash-verify keeps — the drift the pre-edit check the same blocks answers, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.960 -> 0.3.961 (bump_version.py the six crates + lock wear).
- Commit cc513fb (scripts/commit_msg_961.txt) pushed origin main: cf1b533..cc513fb.

Stage Summary:
- v0.3.961: the ADVANCED TOOLTIPS' OWN RENDER the FUNCTIONAL lane wears — the survival inventory's hovered slot the name tooltip the normal flag serves, the `minecraft:<name>` DARK_GRAY ID row the F3+H flag banks, the carried + empty walls the vanilla walk keeps, the positioner the DefaultTooltipPositioner law wears — the client the real Minecraft's advanced-tooltips behavior the F3+H seam speaks.
- The mc-client bank the part_4 window 168 (165 + the tooltip trio), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the keyDebugDumpDynamicTextures lane (the pc 802 the silent lane), the keyDebugDebugOptions lane (the pc 651 the debug-options screen walk), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk), the ADVANCED TOOLTIP'S CREATIVE-SIDE SEAT (the creative picker's palette hover the same rows the flag wears).
"""

for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "a", encoding="utf-8") as f:
        f.write(ENTRY)
    print(f"appended -> {path}")

# verify
for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    data = open(path, encoding="utf-8").read()
    i = data.rindex("---\nTask ID: autopush-210")
    print(f"{path}: verified tail -> {data[i:i+40]!r}")
