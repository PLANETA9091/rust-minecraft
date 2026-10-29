#!/usr/bin/env python3
"""the autopush-333 round's enchanting fn battery part 2 (the render +
the qc drag lanes + the throw/click walk — the brewing 332 shapes the
same lanes wear)."""
import sys

P = "crates/mc-client/src/screens.rs"
s = open(P).read()


def rep(old, new, want=1, tag=""):
    global s
    n = s.count(old)
    if n == 0 and new in s:
        print(f"  [{tag}] already inserted")
        return
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    s = s.replace(old, new)
    print(f"  [{tag}] inserted")


# ---- C. render_enchanting (before the beacon render doc) ----
rep(
    "    /// THE BEACON'S OWN ROUND (the autopush-322 round — the vanilla",
    '''    /// THE ENCHANTING TABLE'S OWN RENDER (the vanilla EnchantmentScreen
    /// over the frozen world — the brewing's own shape the
    /// texture-blitted base keeps): world underneath, dimmed, the REAL
    /// `gui/container/enchanting_table.png` background blitted 176x166
    /// at the centered panel [the extractBackground walk the inherited
    /// imageWidth/imageHeight the 176x166 standard canvas keeps — the
    /// PNG's own opaque bbox (0,0)-(175,165) the decode pins, the book
    /// the texture bakes, the extractBook's animated glyphs the
    /// offline canvas never draws], the centered title the init walk's
    /// own formula speaks, the labels the extractLabels walk draws,
    /// the pair + the mirror icons the believed rows render, the slot
    /// HIT BOXES the 16x16 inner cells the +1 inset wears — the
    /// buttons' own render-then-hit pattern, zero pixel surface of its
    /// own.
    fn render_enchanting(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        g.fill(0, 0, w, h, [0.0, 0.0, 0.0, 0.35]);
        let panel = inventory_panel_origin(w, h);
        self.enchant_panel = panel;
        // THE HIT BOXES (the 26.2 ctor's own literal seats the
        // bytecode probe banks — scripts/enchant_menu_probe_333.py the
        // constants stream speaks: the item (15, 47), the lapis
        // (35, 47), the standard rows 2..38 the
        // addStandardInventorySlots(8, 84) walk keeps — the 16x16
        // inner cells the +1 inset wears)
        self.enchant_slot_rects = [
            (panel.0 + 15 + 1, panel.1 + 47 + 1, 0i16),
            (panel.0 + 35 + 1, panel.1 + 47 + 1, 1i16),
        ]
        .into_iter()
        .chain((0..36).map(|i| {
            let (sx, sy) = chest_mirror_slot(panel, i);
            (sx + 1, sy + 1, (2 + i) as i16)
        }))
        .collect();
        g.sprite_file_region(
            a,
            "gui/container/enchanting_table.png",
            0,
            0,
            176,
            166,
            panel.0,
            panel.1,
        );
        // THE CENTERED TITLE (the init walk's own formula the
        // DispenserScreen's own override speaks — the kind's own lang
        // row the opened kind banks)
        let label = [0.251, 0.251, 0.251];
        let title_text = tr("container.enchant").unwrap_or("").to_string();
        if !title_text.is_empty() {
            let tx = panel.0 + (176 - g.text_width(&title_text)) / 2;
            g.text(a, tx, panel.1 + 6, &title_text, label, false);
        }
        if let Some(t) = tr("container.inventory") {
            g.text(a, panel.0 + 8, panel.1 + 166 - 94, t, label, false);
        }
        // THE HIGHLIGHT'S OWN BACK LAYER (the autopush-333 round — the
        // extractSlotHighlightBack walk the slots BEFORE serves, the
        // same shared AbstractContainerScreen law the family rounds
        // bank): the hovered seat's own 24x24 sprite the (slot.x - 4,
        // slot.y - 4) seat blits, the isHighlightable gate the ONLY
        // wall speaks — NO carried wall, NO hasItem gate, the white
        // alpha-96 the back layer keeps. The mouse_pos gate the
        // headless scenes track no mouse — the pixel goldens never see
        // this pass. THE SHARED SEAT the enchanting's own fn answers,
        // the texture-baked canvas the chest's explicit-chrome reorder
        // never needs (the enchanting_table.png blit the bg IS — the
        // brewing's own law the texture-baked family keeps).
        let highlight_origin = self
            .mouse_pos
            .and_then(|(mx, my)| self.enchant_highlight_origin(mx, my));
        if let Some((ox, oy)) = highlight_origin {
            g.sprite_at_tinted(
                a,
                "container/slot_highlight_back",
                ox - 4,
                oy - 4,
                24,
                24,
                [1.0, 1.0, 1.0, 1.0],
            );
        }
        // THE SLOT ICONS (the brewing's own draw — the believed rows
        // the pair + the mirror keep: the transient pair, the standard
        // rows the real inventory speaks)
        for &(sx, sy, slot) in &self.enchant_slot_rects {
            let slot = slot as usize;
            let believed = if slot < 2 {
                self.enchant_slots.get(slot)
            } else {
                self.enchant_mirror.get(slot - 2)
            };
            let Some(&(id, count)) = believed else {
                continue;
            };
            if count <= 0 {
                continue; // the wire's own emptiness sentinel
            }
            draw_item_icon(g, id, sx, sy);
            if count > 1 {
                let s = count.to_string();
                let tw = g.text_width(&s);
                g.text(a, sx + 16 - tw, sy + 9, &s, WHITE, true);
            }
        }
        // THE HELD STACK follows the cursor (the vanilla carried
        // render — the offline sim's local cursor the chest's own gate
        // keeps, the headless scenes track no mouse)
        if self.enchant_carried.1 > 0 {
            if let Some((mx, my)) = self.mouse_pos {
                draw_item_icon(g, self.enchant_carried.0, mx - 8, my - 8);
                if self.enchant_carried.1 > 1 {
                    let s = self.enchant_carried.1.to_string();
                    let tw = g.text_width(&s);
                    g.text(a, mx - 8 + 17 - tw, my - 8 + 9, &s, WHITE, true);
                }
            }
        }
        self.render_bundle_tooltip(g, a);
        // THE HIGHLIGHT'S OWN FRONT LAYER (the extractSlotHighlightFront
        // walk the slots AFTER serves — the white alpha-32 the front
        // layer keeps, the same no-gate law the back layer wears, the
        // icon the translucent tint the tooltip the topmost layer
        // never under-runs)
        if let Some((ox, oy)) = highlight_origin {
            g.sprite_at_tinted(
                a,
                "container/slot_highlight_front",
                ox - 4,
                oy - 4,
                24,
                24,
                [1.0, 1.0, 1.0, 1.0],
            );
        }
    }

    /// THE BEACON'S OWN ROUND (the autopush-322 round — the vanilla''',
    1,
    "render fn",
)

# ---- D. the qc arm + paint fns (before the table classification doc) ----
rep(
    "    /// THE TABLE'S OWN SEAT CLASSIFICATION (the hovered seat the",
    '''    /// THE ENCHANTING DRAG'S OWN ARM (the same mouseClicked pcs 316-378
    /// base walk the family rounds bank — the enchanting screen the
    /// SAME AbstractContainerScreen base class wears): the
    /// carried-NON-EMPTY press the isQuickCrafting start speaks — NO
    /// slot click, NO packet. The hovered seat AND the outside press
    /// the arm answers; the press BETWEEN the slots (k == -1) the
    /// whole arm skips. The OFFLINE sim the drag the local lanes ride
    /// (the wire lane the brewing's own law never wears).
    fn qc_arm_enchanting(&mut self, mx: i32, my: i32, button: usize) {
        if self.qc_armed {
            return; // pc 168-172 — the in-flight drag the press never re-arms
        }
        if self.enchant_slot_rects.is_empty() {
            return; // no render spoke yet — no geometry to hit
        }
        let panel = self.enchant_panel;
        let on_seat = self.enchant_hover_seat(mx, my).is_some();
        let outside = panel != (0, 0)
            && (mx < panel.0 || my < panel.1 || mx >= panel.0 + 176 || my >= panel.1 + 166);
        if !on_seat && !outside {
            return; // pc 162-165 — the k == -1 press the walk skips
        }
        self.qc_armed = true;
        self.qc_button = button as i32;
        self.qc_type = button as i32;
        self.qc_slots.clear();
        self.qc_remainder = 0;
    }

    /// THE ENCHANTING DRAG'S OWN COLLECT (mouseDragged pcs 23-58 — the
    /// hovered seat the shouldAddSlotToQuickCraft gates answer): the
    /// same walk the brewing's paint wears — the carried non-empty,
    /// the count's own law, the seat's canItemQuickReplace, the plain
    /// slots' mayPlace the base Slot's iconst_1 pair keeps.
    fn qc_paint_enchanting(&mut self, mx: i32, my: i32) {
        let Some((_, _, slot)) = self.enchant_hover_seat(mx, my) else {
            return; // pc 25 — the no-seat frame the paint skips
        };
        let carried = self.enchant_carried;
        if carried.1 <= 0 {
            return; // shouldAdd pc 7-11 — the empty cursor never paints
        }
        if !(carried.1 as usize > self.qc_slots.len() || self.qc_type == 2) {
            return; // shouldAdd pc 14-37 — the count's own gate
        }
        let menu = slot as usize;
        let (seat_id, _) = self.qc_seat_stack(menu);
        if seat_id != 0 && seat_id != carried.0 {
            return; // canItemQuickReplace — the different item the seat refuses
        }
        if self.qc_slots.contains(&menu) {
            return; // pc 45-50 — the Set.add's own false the recalc never feeds
        }
        self.qc_slots.push(menu);
        self.qc_recalc_remainder(); // pc 53-54 — the new seat the recalc speaks
    }

    /// THE TABLE'S OWN SEAT CLASSIFICATION (the hovered seat the''',
    1,
    "qc arm/paint fns",
)

# ---- E. the distribute fn (before the ghost recalc doc) ----
rep(
    "    /// THE GHOST'S OWN RECALC (recalculateQuickCraftRemaining",
    '''    /// THE ENCHANTING DRAG'S OWN DISTRIBUTION (the same pcs 299-322 walk
    /// the brewing's offline lane wears — the painted set the
    /// one-each law serves): the offline door the wire lane never
    /// wears (the brewing's own law the enchanting door keeps).
    fn qc_distribute_enchanting_offline(&mut self) {
        let carried = self.enchant_carried;
        if carried.1 <= 0 {
            return; // pcs 307-319 — the empty cursor the walk aborts
        }
        let painted = self.qc_slots.len();
        let mut k = carried.1;
        let seats = self.qc_slots.clone(); // the disjoint borrow the write keeps
        for &menu in &seats {
            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            if self.qc_type != 2 && carried.1 < painted as i32 {
                continue; // pc 394-416 — the count's own gate
            }
            let cap = crate::world_view::item_max_stack(carried.0);
            let place_count = if self.qc_type == 1 {
                1
            } else {
                carried.1 / painted as i32
            };
            let place = std::cmp::min(place_count + seat_count, cap);
            k -= place - seat_count; // pc 495-503 — the net the carried loses
            let new = (carried.0, place); // pc 505-514
            if menu < 2 {
                if menu < self.enchant_slots.len() {
                    self.enchant_slots[menu] = new;
                }
            } else {
                // THE WRITE-THROUGH (the projection the inventory IS —
                // the mirror row the inventory cell answers: the main
                // rows 0..27 the items 9..36 keep, the hotbar 27..36
                // the items 0..9 speak)
                let row = menu - 2;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
        }
        self.enchant_carried = if k > 0 { (carried.0, k) } else { (0, 0) };
        // THE MIRROR'S OWN SYNC (the fresh rows EVERY mutation re-syncs
        // — the open's own law the write-through's tail wears)
        self.enchant_mirror_sync_from_inventory();
    }

    /// THE GHOST'S OWN RECALC (recalculateQuickCraftRemaining''',
    1,
    "distribute fn",
)

# ---- F. the throw + click fns (before the dispenser throw doc) ----
rep(
    "    /// THE DISPENSER'S OWN OUTSIDE THROW (the furnace's own walk the",
    '''    /// THE ENCHANTING TABLE'S OWN OUTSIDE THROW (the brewing's own walk
    /// the enchanting canvas wears — the doClick -999 PICKUP the
    /// carried's drop serves).
    fn throw_enchanting_carried_outside(&mut self, button: i8) {
        let (id, count) = self.enchant_carried;
        if count <= 0 {
            return; // the doClick THROW's own isEmpty guard — silence
        }
        let dropped = if button == 0 { count } else { 1 };
        self.drop_item_entity_aimed(id, dropped);
        self.enchant_carried = if button == 0 { (0, 0) } else { (id, count - 1) };
    }

    /// THE ENCHANTING TABLE'S OWN CLICK LANE (the offline sim's own walk
    /// — the vanilla doClick's local walk the believed stack the
    /// pickup/split prediction mutates, the carried the local cursor
    /// keeps, NO echo: the fresh stand-in has no window and no
    /// session). THE STANDARD ROWS the player's own store wear (the
    /// menu 2..28 the main grid rides, the menu 29..37 the hotbar —
    /// the write-through projection the inventory IS).
    fn click_enchanting_slot(&mut self, mx: i32, my: i32, button: i8) {
        if self.enchant_slot_rects.is_empty() {
            return; // no render spoke yet — no geometry to hit
        }
        let Some((_, _, slot)) = self.enchant_hover_seat(mx, my) else {
            // THE DROP-OUTSIDE LANE (the brewing's own shape — the
            // laid-out panel the strict rect compare speaks)
            if self.enchant_panel != (0, 0)
                && (mx < self.enchant_panel.0
                    || my < self.enchant_panel.1
                    || mx >= self.enchant_panel.0 + 176
                    || my >= self.enchant_panel.1 + 166)
            {
                self.throw_enchanting_carried_outside(button);
            }
            return;
        };
        let menu = slot as usize;
        // THE QUICK_MOVE LANE (the shift-click — the vanilla doClick's
        // own precedence the chest's walk keeps): the combined
        // 38-slot snapshot the REAL inventory wears (the pair + the
        // main 27 + the hotbar 9), the multi-slot deltas the pair AND
        // the items mutate (the menu's own numbering the split
        // speaks — the menu 2..28 the items 9..35 answer, the menu
        // 29..37 the items 0..8 keep).
        if self.shift_down {
            let mut snapshot: Vec<(i32, i32)> = Vec::with_capacity(38);
            snapshot.extend_from_slice(&self.enchant_slots);
            snapshot.extend_from_slice(&self.inventory.items[9..36]);
            snapshot.extend_from_slice(&self.inventory.items[0..9]);
            let changed = crate::world_view::predict_quick_move_grid(&snapshot, menu, 2);
            for (i, pair) in &changed {
                if *i < 2 {
                    self.enchant_slots[*i] = *pair;
                } else if *i < 29 {
                    self.inventory.items[*i + 7] = *pair;
                } else {
                    self.inventory.items[*i - 29] = *pair;
                }
            }
            self.enchant_mirror_sync_from_inventory();
            return;
        }
        // the believed slot value: the pair's 0..2, the REAL
        // inventory's rows the menu numbering speaks (2..28 the main
        // grid, 29..37 the hotbar)
        let believed = if menu < 2 {
            self.enchant_slots[menu]
        } else if menu < 29 {
            self.inventory.items[menu + 7]
        } else {
            self.inventory.items[menu - 29]
        };
        let (new_slot, new_carried) = if button == 0 {
            crate::world_view::predict_pickup(believed, self.enchant_carried)
        } else {
            crate::world_view::predict_pickup_right(believed, self.enchant_carried)
        };
        if menu < 2 {
            self.enchant_slots[menu] = new_slot;
        } else if menu < 29 {
            self.inventory.items[menu + 7] = new_slot;
        } else {
            self.inventory.items[menu - 29] = new_slot;
        }
        self.enchant_carried = new_carried;
        self.enchant_mirror_sync_from_inventory();
    }

    /// THE DISPENSER'S OWN OUTSIDE THROW (the furnace's own walk the''',
    1,
    "throw/click fns",
)

open(P, "w").write(s)
print("battery C-F done")
