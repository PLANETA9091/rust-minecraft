#!/usr/bin/env python3
"""the autopush-333 round's screens.rs wiring + the enchanting fn
battery (the brewing 332 shapes the same lanes wear — the transient
pair the store-less close the clearContainer law speaks)."""
import sys

P = "crates/mc-client/src/screens.rs"
s = open(P).read()


def rep(old, new, want=1, tag=""):
    global s
    n = s.count(old)
    if n == 0 and new in s:
        print(f"  [{tag}] already wired")
        return
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    s = s.replace(old, new)
    print(f"  [{tag}] wired")


# ---- 1. the close dispatches (both the same two-line pair wear) ----
rep(
    "            ScreenKind::Brewing => self.close_brewing(ScreenKind::World),\n"
    "            ScreenKind::Furnace => self.close_furnace(ScreenKind::World),",
    "            ScreenKind::Brewing => self.close_brewing(ScreenKind::World),\n"
    "            ScreenKind::Enchanting => self.close_enchanting(ScreenKind::World),\n"
    "            ScreenKind::Furnace => self.close_furnace(ScreenKind::World),",
    2,
    "close dispatches",
)

# ---- 2. the second close dispatch (the back-parameter walk) ----
rep(
    "            ScreenKind::Brewing => self.close_brewing(back),\n",
    "            ScreenKind::Brewing => self.close_brewing(back),\n"
    "            ScreenKind::Enchanting => self.close_enchanting(back),\n",
    1,
    "second close dispatch",
)

# ---- 3. the Escape arm ----
rep(
    "        } else if self.screen == ScreenKind::Brewing {\n"
    "            // THE FIVE-SEAT'S OWN CLOSE (the vanilla Escape/E on the\n"
    "            // brewing stand's screen — the chest's own close walk the\n"
    "            // carried return, the store flush and the fill ride speak)\n"
    "            self.close_brewing(ScreenKind::World);\n"
    "        } else if self.screen == ScreenKind::Furnace {",
    "        } else if self.screen == ScreenKind::Brewing {\n"
    "            // THE FIVE-SEAT'S OWN CLOSE (the vanilla Escape/E on the\n"
    "            // brewing stand's screen — the chest's own close walk the\n"
    "            // carried return, the store flush and the fill ride speak)\n"
    "            self.close_brewing(ScreenKind::World);\n"
    "        } else if self.screen == ScreenKind::Enchanting {\n"
    "            // THE TRANSIENT PAIR'S OWN CLOSE (the vanilla Escape/E\n"
    "            // on the enchanting table's screen — the close walk the\n"
    "            // carried + the pair home return, the NO store the table\n"
    "            // never keeps)\n"
    "            self.close_enchanting(ScreenKind::World);\n"
    "        } else if self.screen == ScreenKind::Furnace {",
    1,
    "escape arm",
)

# ---- 4. the carried lane ----
rep(
    "            return self.brewing_carried;\n"
    "        }\n"
    "        if self.screen == ScreenKind::Chest {",
    "            return self.brewing_carried;\n"
    "        }\n"
    "        if self.screen == ScreenKind::Enchanting {\n"
    "            // THE TRANSIENT PAIR'S OWN LANE CURSOR (the offline sim\n"
    "            // the SAME field's own stack holds — the wire lane the\n"
    "            // enchanting door never wears)\n"
    "            return self.enchant_carried;\n"
    "        }\n"
    "        if self.screen == ScreenKind::Chest {",
    1,
    "carried lane",
)

# ---- 5. the qc_seat_stack block ----
rep(
    "                self.brewing_mirror[menu - 5]\n"
    "            };\n"
    "        }\n"
    "        if self.screen == ScreenKind::Chest {",
    "                self.brewing_mirror[menu - 5]\n"
    "            };\n"
    "        }\n"
    "        if self.screen == ScreenKind::Enchanting {\n"
    "            // THE TRANSIENT PAIR'S OWN NUMBERING (the pair the table\n"
    "            // keeps, the standard rows 2..38 the mirror holds — the\n"
    "            // paint seat read the SAME split wears)\n"
    "            return if menu < 2 {\n"
    "                self.enchant_slots[menu]\n"
    "            } else {\n"
    "                self.enchant_mirror[menu - 2]\n"
    "            };\n"
    "        }\n"
    "        if self.screen == ScreenKind::Chest {",
    1,
    "qc seat stack",
)

# ---- 6. the qc distribute dispatch ----
rep(
    "                self.qc_distribute_brewing_offline();\n"
    "            } else if self.screen == ScreenKind::Crafting {",
    "                self.qc_distribute_brewing_offline();\n"
    "            } else if self.screen == ScreenKind::Enchanting {\n"
    "                // THE TRANSIENT PAIR'S OWN LANES (the offline sim the\n"
    "                // local write-through answer — the dispenser's own\n"
    "                // shape the offline door wears, the wire lane the\n"
    "                // hopper's own law never wears)\n"
    "                self.qc_distribute_enchanting_offline();\n"
    "            } else if self.screen == ScreenKind::Crafting {",
    1,
    "qc distribute dispatch",
)

# ---- 7. the left-press gate ----
rep(
    "                self.click_brewing_slot(mx, my, 0);\n"
    "            }\n"
    "            return;\n"
    "        }\n",
    "                self.click_brewing_slot(mx, my, 0);\n"
    "            }\n"
    "            return;\n"
    "        }\n"
    "        if self.screen == ScreenKind::Enchanting {\n"
    "            // THE TRANSIENT PAIR'S OWN PRESS (the same mouseClicked\n"
    "            // walk the brewing's gate shape wears — the carried press\n"
    "            // the drag arms, the carried-empty press the PICKUP\n"
    "            // dispatch keeps)\n"
    "            let held = self.enchant_carried.1 > 0;\n"
    "            if held {\n"
    "                self.qc_arm_enchanting(mx, my, 0);\n"
    "            } else {\n"
    "                self.click_enchanting_slot(mx, my, 0);\n"
    "            }\n"
    "            return;\n"
    "        }\n",
    1,
    "left press gate",
)

# ---- 8. the right-press gate ----
rep(
    "                self.click_brewing_slot(mx, my, 1);\n"
    "            }\n"
    "            return false;\n"
    "        }\n",
    "                self.click_brewing_slot(mx, my, 1);\n"
    "            }\n"
    "            return false;\n"
    "        }\n"
    "        if self.screen == ScreenKind::Enchanting {\n"
    "            // THE IN-FLIGHT DRAG'S OWN GATE (mouseClicked pc 168-172\n"
    "            // — the armed drag's frames the other button's press\n"
    "            // keep silent); the right press the TYPE-1 DRAG ARMS (the\n"
    "            // one-each distribution the release serves); the\n"
    "            // carried-empty press the SPLIT-PICKUP dispatch keeps.\n"
    "            if self.qc_armed {\n"
    "                return false;\n"
    "            }\n"
    "            let held = self.enchant_carried.1 > 0;\n"
    "            if held {\n"
    "                self.qc_arm_enchanting(mx, my, 1);\n"
    "            } else {\n"
    "                self.click_enchanting_slot(mx, my, 1);\n"
    "            }\n"
    "            return false;\n"
    "        }\n",
    1,
    "right press gate",
)

# ---- 9. the qc paint dispatch ----
rep(
    "                self.qc_paint_brewing(mx, my);\n"
    "            } else if self.screen == ScreenKind::Crafting {",
    "                self.qc_paint_brewing(mx, my);\n"
    "            } else if self.screen == ScreenKind::Enchanting {\n"
    "                self.qc_paint_enchanting(mx, my);\n"
    "            } else if self.screen == ScreenKind::Crafting {",
    1,
    "qc paint dispatch",
)

# ---- 10. the hovered_container arm ----
rep(
    "                    *self.brewing_mirror.get(s - 5).unwrap_or(&(0, 0))\n"
    "                };\n"
    "                Some((id, count, slot as i32))\n"
    "            }\n",
    "                    *self.brewing_mirror.get(s - 5).unwrap_or(&(0, 0))\n"
    "                };\n"
    "                Some((id, count, slot as i32))\n"
    "            }\n"
    "            ScreenKind::Enchanting => {\n"
    "                // THE SHARED SEAT (the autopush-333 round — the bundle\n"
    "                // lane the same walk rides)\n"
    "                let (_, _, slot) = self.enchant_hover_seat(mx, my)?;\n"
    "                let s = slot as usize;\n"
    "                let (id, count) = if s < 2 {\n"
    "                    *self.enchant_slots.get(s).unwrap_or(&(0, 0))\n"
    "                } else {\n"
    "                    *self.enchant_mirror.get(s - 2).unwrap_or(&(0, 0))\n"
    "                };\n"
    "                Some((id, count, slot as i32))\n"
    "            }\n",
    1,
    "hovered container arm",
)

# ---- 11. the use-walk door ----
rep(
    "            self.open_brewing_for(hit.pos);\n"
    "            return true;\n"
    "        }\n",
    "            self.open_brewing_for(hit.pos);\n"
    "            return true;\n"
    "        }\n"
    "        // THE ENCHANTING TABLE'S OWN GUI DOOR (javap 26.2\n"
    "        // EnchantmentTableBlock.useWithoutItem — the openMenu walk\n"
    "        // the table's own MenuProvider answers: the right-press on\n"
    "        // the placed enchanting table the OFFLINE world answers —\n"
    "        // the transient-pair screen the fresh seats open, the NO\n"
    "        // store the vanilla menu the table's own BE never keeps).\n"
    "        // The sneaking player the arm SKIPS (the crafting-table's\n"
    "        // own gate the vanilla use walk wears). The WIRE session the\n"
    "        // local screen never opens (the server's own OpenScreen word\n"
    "        // owns the container — the hopper's own law the door keeps).\n"
    "        if !self.shift_down\n"
    "            && !self.world_is_network\n"
    "            && self.active_world().is_some_and(|w| {\n"
    "                crate::world::is_enchanting_table(w.get(hit.pos.0, hit.pos.1, hit.pos.2))\n"
    "            })\n"
    "        {\n"
    "            self.open_enchanting_for(hit.pos);\n"
    "            return true;\n"
    "        }\n",
    1,
    "use-walk door",
)

# ---- 12. the render dispatch arm ----
rep(
    "                self.render_brewing(&mut g, a, gw, gh);\n"
    "            }\n"
    "            ScreenKind::Beacon => {",
    "                self.render_brewing(&mut g, a, gw, gh);\n"
    "            }\n"
    "            ScreenKind::Enchanting => {\n"
    "                // THE ENCHANTING TABLE'S OWN ROUND (the vanilla\n"
    "                // EnchantmentScreen over the frozen world — the\n"
    "                // brewing's own shape the texture-blitted base keeps):\n"
    "                // world underneath, dimmed, the REAL background\n"
    "                // texture, the pair + the mirror icons, the carried\n"
    "                // follow — no mannequin (the EnchantmentScreen the\n"
    "                // vanilla PiP never renders, the extractBook's\n"
    "                // animated glyphs the offline canvas never draws).\n"
    "                self.render_world_3d(fb, a);\n"
    "                let mut g = Gui::new(fb, Gui::auto_scale(fb.w, fb.h)).with_scramble(self.frame);\n"
    "                let (gw, gh) = (g.w(), g.h());\n"
    "                self.render_enchanting(&mut g, a, gw, gh);\n"
    "            }\n"
    "            ScreenKind::Beacon => {",
    1,
    "render dispatch arm",
)

open(P, "w").write(s)
print("wiring done")
