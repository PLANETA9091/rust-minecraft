#!/usr/bin/env python3
"""autopush-383 — THE DRAG'S OWN PLACE VERDICT splice: the seven
kind-worded offline distribute folds' own per-seat component gate +
the empty-seat word bank (the anvil/beacon field-less walls the plain
verdict keeps). Each splice asserts the exact anchor count first."""
import sys

PATH = "crates/mc-client/src/screens.rs"
src = open(PATH, encoding="utf-8").read()


def splice(old, new, label, count=1):
    global src
    n = src.count(old)
    assert n == count, f"{label}: anchor count {n} != {count}"
    src = src.replace(old, new)
    print(f"[ok] {label}")


GATE_TAIL = "the place refuses\n"

# ---------- 1. CHEST OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // the mayPlace verdict: the chest menu's slots ALL plain"""
new = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // container rows the bank's normalized read speaks, the
            // store rows the shared bank's gated word (the paint's
            // own reads the fold re-wears)
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                if menu < self.chest_slots.len() {
                    self.chest_row_potion_kind(menu)
                } else {
                    let rel = menu - self.chest_slots.len();
                    let row = if rel < 27 { 9 + rel } else { rel - 27 };
                    self.items_potion_kinds[row]
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.chest_carried_potion_kind)
            {
                continue; // the component half — the same-id kind pair refuses
            }
            // the mayPlace verdict: the chest menu's slots ALL plain"""
splice(old, new, "chest gate")

# ---------- 1b. CHEST OFFLINE: the word bank ----------
old = """                let row = menu - n;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
        }
        self.chest_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
new = """                let row = menu - n;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
            // THE PLACE'S OWN WORD BANK (the autopush-383 round — the
            // drag's own word ride the chest rows re-wear: the empty
            // row the placed stack's own word banks [the click lane's
            // own place arm the same bank wears], the occupied row the
            // merge the original's word keeps)
            if seat_count == 0 {
                if menu < n {
                    if menu < self.chest_row_potion_kinds.len() {
                        self.chest_row_potion_kinds[menu] = self.chest_carried_potion_kind;
                    }
                } else {
                    let rel = menu - n;
                    let idx = if rel < 27 { 9 + rel } else { rel - 27 };
                    if idx < self.items_potion_kinds.len() {
                        self.items_potion_kinds[idx] = self.chest_carried_potion_kind;
                    }
                }
            }
        }
        self.chest_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
splice(old, new, "chest word bank")

# ---------- 2. DISPENSER OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // the mayPlace verdict: the dispenser menu's slots ALL
            // plain (the base Slot's iconst_1 pair the chest swap
            // walk banks) — no refusal the walk wears"""
new = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // grid nine the kindless store the 378 wall keeps, the
            // mirror rows the shared bank's gated word (the paint's
            // own mapped read the fold re-wears)
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                if menu < 9 {
                    crate::brewing_sim::POTION_KIND_NONE
                } else {
                    let mapped = if menu < 36 { menu } else { menu - 36 };
                    self.items_potion_kinds[mapped]
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.dispenser_carried_potion_kind)
            {
                continue; // the component half — the same-id kind pair refuses
            }
            // the mayPlace verdict: the dispenser menu's slots ALL
            // plain (the base Slot's iconst_1 pair the chest swap
            // walk banks) — no refusal the walk wears"""
splice(old, new, "dispenser gate")

# ---------- 2b. DISPENSER OFFLINE: the word bank ----------
old = """                let row = menu - 9;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
        }
        self.dispenser_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
new = """                let row = menu - 9;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
            // THE PLACE'S OWN WORD BANK (the autopush-383 round — the
            // empty mirror row the placed stack's own word banks, the
            // occupied row the merge the original's word keeps, the
            // grid nine the kindless wall the bank never wears)
            if seat_count == 0 && menu >= 9 {
                let row = menu - 9;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.items_potion_kinds.len() {
                    self.items_potion_kinds[idx] = self.dispenser_carried_potion_kind;
                }
            }
        }
        self.dispenser_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
splice(old, new, "dispenser word bank")

# ---------- 3. BREWING OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
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
            if menu < 5 {"""
new = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // stand seats the five-word bank the 377 ride reads, the
            // mirror rows the shared bank's gated word (the paint's
            // own row mapping the fold re-wears)
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                if menu < 5 {
                    self.brewing_kinds[menu]
                } else if menu < 32 {
                    self.items_potion_kinds[menu + 4]
                } else {
                    self.items_potion_kinds[menu - 32]
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.brewing_carried_kind)
            {
                continue; // the component half — the same-id kind pair refuses
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
            if menu < 5 {"""
splice(old, new, "brewing gate")

# ---------- 3b. BREWING OFFLINE: the word bank ----------
old = """                let row = menu - 5;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
        }
        self.brewing_carried = if k > 0 { (carried.0, k) } else { (0, 0) };"""
new = """                let row = menu - 5;
                let idx = if row < 27 { 9 + row } else { row - 27 };
                if idx < self.inventory.items.len() {
                    self.inventory.items[idx] = new;
                }
            }
            // THE PLACE'S OWN WORD BANK (the autopush-383 round — the
            // empty seat the placed stack's own word banks [the stand
            // seats the 377 bank wears, the mirror rows the shared
            // bank speaks], the occupied seat the merge the original's
            // word keeps)
            if seat_count == 0 {
                if menu < 5 {
                    if menu < self.brewing_kinds.len() {
                        self.brewing_kinds[menu] = self.brewing_carried_kind;
                    }
                } else {
                    let row = menu - 5;
                    let idx = if row < 27 { 9 + row } else { row - 27 };
                    if idx < self.items_potion_kinds.len() {
                        self.items_potion_kinds[idx] = self.brewing_carried_kind;
                    }
                }
            }
        }
        self.brewing_carried = if k > 0 { (carried.0, k) } else { (0, 0) };"""
splice(old, new, "brewing word bank")

# ---------- 4. ENCHANTING OFFLINE: the gate (the 379 wall — no bank) ----------
old = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
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
                }"""
new = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // pair seats the kindless store the 379 wall keeps (the
            // bank the carried word alone wears), the kinded carried
            // the wordless seat the mismatch refuses
            if seat_id == carried.0
                && qc_collect_kind_refuses(
                    crate::brewing_sim::POTION_KIND_NONE,
                    self.enchant_carried_potion_kind,
                )
            {
                continue; // the component half — the kinded carried refuses
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
                }"""
splice(old, new, "enchanting gate")

# ---------- 5. TABLE OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            if !quickcraft_table_seat_may_place(menu) {
                continue; // pc 384-391 — the result's own mayPlace verdict
            }"""
new = """            let (seat_id, seat_count) = self.qc_seat_stack(menu);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // grid cells the 376 bank answers, the store rows the
            // shared bank's word (the paint's own reads the fold
            // re-wears)
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                match menu {
                    1..=9 => self.table_grid_potion_kinds[menu - 1],
                    10..=36 => self.items_potion_kinds[menu - 1],
                    37..=45 => self.items_potion_kinds[menu - 37],
                    _ => crate::brewing_sim::POTION_KIND_NONE,
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.inv_carried_potion_kind)
            {
                continue; // the component half — the same-id kind pair refuses
            }
            if !quickcraft_table_seat_may_place(menu) {
                continue; // pc 384-391 — the result's own mayPlace verdict
            }"""
splice(old, new, "table gate")

# ---------- 5b. TABLE OFFLINE: the store word bank ----------
old = """                self.table_grid[menu - 1] = new;
            } else if menu < 37 {
                self.inventory.items[menu - 1] = new;
            } else {
                self.inventory.items[menu - 37] = new;
            }
        }"""
new = """                self.table_grid[menu - 1] = new;
            } else if menu < 37 {
                self.inventory.items[menu - 1] = new;
            } else {
                self.inventory.items[menu - 37] = new;
            }
            // THE STORE'S OWN PLACE BANK (the autopush-383 round — the
            // drag's own word ride the store rows re-wear: the empty
            // row the placed stack's own word banks, the occupied row
            // the merge the original's word keeps)
            if seat_count == 0 && menu >= 10 {
                let row = if menu < 37 { menu - 1 } else { menu - 37 };
                self.items_potion_kinds[row] = self.inv_carried_potion_kind;
            }
        }"""
splice(old, new, "table store word bank")

# ---------- 6. FURNACE OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.furnace_slots.get(menu).copied().unwrap_or((0, 0));
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            if !quickcraft_furnace_seat_may_place(menu, carried.0, kind) {"""
new = """            let (seat_id, seat_count) = self.furnace_slots.get(menu).copied().unwrap_or((0, 0));
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // trio the kindless store the dev-208 wall keeps, the
            // store rows the menu map's gated word (the paint's own
            // reads the fold re-wears)
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                match Self::furnace_menu_to_inventory(menu) {
                    Some(row) => self.items_potion_kinds[row],
                    None => crate::brewing_sim::POTION_KIND_NONE,
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.furnace_carried_potion_kind)
            {
                continue; // the component half — the same-id kind pair refuses
            }
            if !quickcraft_furnace_seat_may_place(menu, carried.0, kind) {"""
splice(old, new, "furnace gate")

# ---------- 6b. FURNACE OFFLINE: the store word bank ----------
old = """            if menu >= 3 {
                // THE WRITE-THROUGH (the projection the inventory IS —
                // the menu slot the inventory cell answers)
                let idx = if menu < 30 { 9 + menu - 3 } else { menu - 30 };
                self.inventory.items[idx] = new;
            }
        }
        self.furnace_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
new = """            if menu >= 3 {
                // THE WRITE-THROUGH (the projection the inventory IS —
                // the menu slot the inventory cell answers)
                let idx = if menu < 30 { 9 + menu - 3 } else { menu - 30 };
                self.inventory.items[idx] = new;
            }
            // THE PLACE'S OWN WORD BANK (the autopush-383 round — the
            // empty store row the placed stack's own word banks, the
            // occupied row the merge the original's word keeps, the
            // trio the kindless wall the bank never wears)
            if seat_count == 0 {
                if let Some(row) = Self::furnace_menu_to_inventory(menu) {
                    if row < self.items_potion_kinds.len() {
                        self.items_potion_kinds[row] = self.furnace_carried_potion_kind;
                    }
                }
            }
        }
        self.furnace_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps"""
splice(old, new, "furnace store word bank")

# ---------- 7. GENERIC OFFLINE: the gate ----------
old = """            let (seat_id, seat_count) = self.inventory_hover_stack(hover);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            if !self.inventory_seat_may_place(hover, carried.0) {"""
new = """            let (seat_id, seat_count) = self.inventory_hover_stack(hover);
            if seat_id != 0 && seat_id != carried.0 {
                continue; // pc 373-381 — canItemQuickReplace
            }
            // THE COMPONENT HALF (the autopush-383 round — the same-id
            // kind pair the seat's own word the place refuses): the
            // store rows the bank's own word reads, the offline craft
            // cells the grid bank answers, the armor/result seats the
            // kindless law speaks
            let seat_kind = if crate::world::brewing_is_potion_kind(seat_id) {
                match hover {
                    InventoryHover::Craft(cell) => self.craft_grid_potion_kinds[cell],
                    InventoryHover::Store(i) => self.items_potion_kinds[i],
                    InventoryHover::Offhand => self.items_potion_kinds[OFFHAND_KIND_SEAT],
                    _ => crate::brewing_sim::POTION_KIND_NONE,
                }
            } else {
                crate::brewing_sim::POTION_KIND_NONE
            };
            if seat_id == carried.0
                && qc_collect_kind_refuses(seat_kind, self.inv_carried_potion_kind)
            {
                continue; // the component half — the same-id kind pair refuses
            }
            if !self.inventory_seat_may_place(hover, carried.0) {"""
splice(old, new, "generic gate")

# ---------- 7b. GENERIC OFFLINE: the store/offhand word bank ----------
old = """                if seat_count == 0 {
                    self.craft_grid_potion_kinds[cell] = self.inv_carried_potion_kind;
                }
            }
        }
        self.inv_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps (the empty the (0,0) law speaks)"""
new = """                if seat_count == 0 {
                    self.craft_grid_potion_kinds[cell] = self.inv_carried_potion_kind;
                }
            }
            // THE STORE'S OWN PLACE BANK (the autopush-383 round — the
            // drag's own word ride the store rows re-wear: the empty
            // row the placed stack's own word banks, the occupied row
            // the merge the original's word keeps, the craft cells the
            // dev-206 ride the bank already wears)
            if seat_count == 0 {
                match hover {
                    InventoryHover::Store(i) => {
                        self.items_potion_kinds[i] = self.inv_carried_potion_kind
                    }
                    InventoryHover::Offhand => {
                        self.items_potion_kinds[OFFHAND_KIND_SEAT] = self.inv_carried_potion_kind
                    }
                    _ => {}
                }
            }
        }
        self.inv_carried = if k > 0 { (carried.0, k) } else { (0, 0) }; // pc 520-530 — the remainder the cursor keeps (the empty the (0,0) law speaks)"""
splice(old, new, "generic store/offhand word bank")

open(PATH, "w", encoding="utf-8").write(src)
print("[done] screens.rs spliced — 11 regions")
sys.exit(0)
