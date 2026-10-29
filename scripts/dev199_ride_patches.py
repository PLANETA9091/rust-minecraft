#!/usr/bin/env python3
"""dev-199 THE WATER-BOTTLE'S OWN FILL ROUND — the ride patches.

P1: click_brewing_slot quick-move kind hand-over (both directions)
P2: click_brewing_slot main ride extends to the inventory rows
P3: on_drop_key emptied row clears the bank seat
P4: SavedPlayer.potion_kinds field
P5: player_row_to_nbt "PotionKinds" list
P6: player_state_from_nbt parse + construction
P7: boot restore of the bank
P8: player_state_now construction
P9: close_brewing carried home ride the kind seat banks
"""
import sys

PATH = 'crates/mc-client/src/screens.rs'

with open(PATH) as f:
    src = f.read()

count = 0


def patch(old, new, tag):
    global src, count
    n = src.count(old)
    if n != 1:
        print(f"FAIL {tag}: {n} matches")
        sys.exit(1)
    src = src.replace(old, new)
    count += 1
    print(f"ok {tag}")


NONE = 'crate::brewing_sim::POTION_KIND_NONE'

# ---------------------------------------------------------------- P1
p1_old = '''            // THE QUICK-MOVE'S OWN KIND WALK (the inventory wall — the
            // five's changed rows the NONE resets wear, the kind the
            // (id, count) pair the inventory never carries)
            for i in 0..5 {
                if self.brewing_slots[i] != snapshot[i] {
                    self.brewing_kinds[i] = crate::brewing_sim::POTION_KIND_NONE;
                }
            }
            self.brewing_mirror_sync_from_inventory();
            return;
        }
'''
p1_new = '''            // THE QUICK-MOVE'S OWN KIND WALK (the dev-199 round — the
            // kind the whole move the hand-over wears BOTH directions:
            // the kinded source the cleared seat hands over [the five's
            // reset the seat's own law keeps, the row's bank the same
            // verdict answers], the gained dest the seat lays — the
            // potion the max-stack-1 law the merge never speaks, the
            // whole stack the one ride serves; the kindless moves the
            // NONE resets keep — the 339 wall the same verdict wears).
            // THE READ the pre-move bank the snapshot pairs guard — the
            // write-back the real stores already spoke, the normalization
            // the potion-family id test wears.
            let mut riding_kind: Option<i32> = None;
            for (i, _) in &changed {
                let old = snapshot[*i];
                let old_kind = if *i < 5 {
                    self.brewing_kinds[*i]
                } else if *i < 32 {
                    self.items_potion_kinds[*i + 4]
                } else {
                    self.items_potion_kinds[*i - 32]
                };
                if old_kind != crate::brewing_sim::POTION_KIND_NONE
                    && crate::world::brewing_is_potion_kind(old.0)
                    && old.1 > 0
                {
                    riding_kind = Some(old_kind);
                }
            }
            for i in 0..5 {
                if self.brewing_slots[i] != snapshot[i] {
                    self.brewing_kinds[i] = crate::brewing_sim::POTION_KIND_NONE;
                }
            }
            for (i, _) in &changed {
                if *i >= 5 {
                    let row = if *i < 32 { *i + 4 } else { *i - 32 };
                    self.items_potion_kinds[row] = crate::brewing_sim::POTION_KIND_NONE;
                }
            }
            if let Some(kind) = riding_kind {
                for (i, pair) in &changed {
                    if *i < 5 {
                        // THE GAINED SEAT (the empty pre-move the pair
                        // fills — the kind the ride banks)
                        if self.brewing_kinds[*i] == crate::brewing_sim::POTION_KIND_NONE
                            && crate::world::brewing_is_potion_kind(self.brewing_slots[*i].0)
                            && self.brewing_slots[*i].1 > 0
                            && pair.1 > 0
                        {
                            self.brewing_kinds[*i] = kind;
                            break;
                        }
                    } else {
                        let row = if *i < 32 { *i + 4 } else { *i - 32 };
                        if self.items_potion_kinds[row]
                            == crate::brewing_sim::POTION_KIND_NONE
                            && crate::world::brewing_is_potion_kind(
                                self.inventory.items[row].0,
                            )
                            && self.inventory.items[row].1 > 0
                            && pair.1 > 0
                        {
                            self.items_potion_kinds[row] = kind;
                            break;
                        }
                    }
                }
            }
            self.brewing_mirror_sync_from_inventory();
            return;
        }
'''
patch(p1_old, p1_new, 'P1 quick-move hand-over')

# ---------------------------------------------------------------- P2
p2_old = '''        // THE KIND'S OWN RIDE (the 339 round — the kind the whole-stack
        // moves keep: the pickup the seat's kind the carried banks, the
        // place the carried kind the seat lays, the swap the pairs
        // exchange, the merge the emptied carried the NONE wears, the
        // split the kindless remainder the seat's kind keeps; the
        // potion the max-stack-1 law the partial kinded lanes never
        // speaks).
        let old_seat_kind = if menu < 5 {
            self.brewing_kinds[menu]
        } else {
            crate::brewing_sim::POTION_KIND_NONE
        };
        let old_carried_kind = self.brewing_carried_kind;
        let mut new_seat_kind = old_seat_kind;
        let mut new_carried_kind = old_carried_kind;
        if menu < 5 {
            let seat_gone = believed.1 > 0 && new_slot.1 <= 0;
            let seat_new = believed.1 <= 0 && new_slot.1 > 0;
            let carried_gone = self.brewing_carried.1 > 0 && new_carried.1 <= 0;
            let carried_new = self.brewing_carried.1 <= 0 && new_carried.1 > 0;
            if seat_gone && carried_new {
                // the PICKUP — the kind rides to the carried
                new_carried_kind = old_seat_kind;
                new_seat_kind = crate::brewing_sim::POTION_KIND_NONE;
            } else if carried_gone && seat_new {
                // the PLACE — the kind rides to the seat
                new_seat_kind = old_carried_kind;
                new_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
            } else if seat_gone && seat_new {
                // the SWAP — the pairs exchange
                std::mem::swap(&mut new_seat_kind, &mut new_carried_kind);
            } else if carried_gone {
                // the MERGE — the seat keeps its own kind, the emptied
                // carried the NONE wears
                new_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
            }
            self.brewing_kinds[menu] = new_seat_kind;
            self.brewing_carried_kind = new_carried_kind;
        }
'''
p2_new = '''        // THE KIND'S OWN RIDE (the 339 round — the kind the whole-stack
        // moves keep: the pickup the seat's kind the carried banks, the
        // place the carried kind the seat lays, the swap the pairs
        // exchange, the merge the emptied carried the NONE wears, the
        // split the kindless remainder the seat's kind keeps; the
        // potion the max-stack-1 law the partial kinded lanes never
        // speaks. THE DEV-199 ROUND the ROWS the same ride wears —
        // the seat the five keeps, the row the bank answers: the read
        // the normalization speaks [the stale seat the non-potion rows
        // never], the pickup the carried banks the row clears, the
        // place the row lays, the swap the pairs exchange, the merge
        // the emptied carried the NONE wears — the ride-home lane the
        // fill loop's own half wears).
        let row = if menu < 5 {
            0
        } else if menu < 32 {
            menu + 4
        } else {
            menu - 32
        };
        let old_seat_kind = if menu < 5 {
            self.brewing_kinds[menu]
        } else {
            self.row_potion_kind(row)
        };
        let old_carried_kind = self.brewing_carried_kind;
        let mut new_seat_kind = old_seat_kind;
        let mut new_carried_kind = old_carried_kind;
        {
            let seat_gone = believed.1 > 0 && new_slot.1 <= 0;
            let seat_new = believed.1 <= 0 && new_slot.1 > 0;
            let carried_gone = self.brewing_carried.1 > 0 && new_carried.1 <= 0;
            let carried_new = self.brewing_carried.1 <= 0 && new_carried.1 > 0;
            if seat_gone && carried_new {
                // the PICKUP — the kind rides to the carried
                new_carried_kind = old_seat_kind;
                new_seat_kind = crate::brewing_sim::POTION_KIND_NONE;
            } else if carried_gone && seat_new {
                // the PLACE — the kind rides to the seat
                new_seat_kind = old_carried_kind;
                new_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
            } else if seat_gone && seat_new {
                // the SWAP — the pairs exchange
                std::mem::swap(&mut new_seat_kind, &mut new_carried_kind);
            } else if carried_gone {
                // the MERGE — the seat keeps its own kind, the emptied
                // carried the NONE wears
                new_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
            }
            if menu < 5 {
                self.brewing_kinds[menu] = new_seat_kind;
            } else {
                self.items_potion_kinds[row] = new_seat_kind;
            }
            self.brewing_carried_kind = new_carried_kind;
        }
'''
patch(p2_old, p2_new, 'P2 row ride')

# ---------------------------------------------------------------- P3
p3_old = '''        let dropped = if whole_stack { count } else { 1 };
        self.drop_item_entity_aimed(id, dropped);
        // removeFromSelected's store walk: the whole stack gone or
        // exactly one off (the removeItem(selected, 1) law)
        self.inventory.items[sel] = if whole_stack { (0, 0) } else { (id, count - 1) };
        self.hotbar_slots = self.inventory.hotbar();
        self.swing.start(); // the handleKeybinds success arm's MAIN_HAND swing
'''
p3_new = '''        let dropped = if whole_stack { count } else { 1 };
        self.drop_item_entity_aimed(id, dropped);
        // removeFromSelected's store walk: the whole stack gone or
        // exactly one off (the removeItem(selected, 1) law)
        self.inventory.items[sel] = if whole_stack { (0, 0) } else { (id, count - 1) };
        // THE DROPPED KIND'S OWN SEAT (the dev-199 round — the emptied
        // row the bank clears; the dropped entity the (id, count) pair
        // the kind never carries [the same honest note the thrown
        // carried speaks], the partial drop the potion the max-1 law
        // never speaks)
        if self.inventory.items[sel].1 <= 0 {
            self.items_potion_kinds[sel] = crate::brewing_sim::POTION_KIND_NONE;
        }
        self.hotbar_slots = self.inventory.hotbar();
        self.swing.start(); // the handleKeybinds success arm's MAIN_HAND swing
'''
patch(p3_old, p3_new, 'P3 drop seat clear')

# ---------------------------------------------------------------- P4
p4_old = '''    /// The gameplay inventory's own 41 stacks (the hotbar 0..8, the
    /// main 9..35, the armor, the offhand).
    pub inventory: crate::drops::Inventory,
    /// The active hotbar slot (the HUD selection the reload re-wears).
    pub selected_hotbar: i32,
'''
p4_new = '''    /// The gameplay inventory's own 41 stacks (the hotbar 0..8, the
    /// main 9..35, the armor, the offhand).
    pub inventory: crate::drops::Inventory,
    /// THE ROWS' OWN POTION-KIND BANK (the dev-199 round — the
    /// "PotionKinds" list the tag stores flat, the 36 rows the same
    /// numbering the Items list wears; the elder file the missing row
    /// the NONE parse keeps).
    pub potion_kinds: [i32; 36],
    /// The active hotbar slot (the HUD selection the reload re-wears).
    pub selected_hotbar: i32,
'''
patch(p4_old, p4_new, 'P4 SavedPlayer field')

# ---------------------------------------------------------------- P5
p5_old = '''        ("Items".to_string(), stacks(&sp.inventory.items)),
        ("Armor".to_string(), stacks(&sp.inventory.armor)),
        ("Offhand".to_string(), stacks(&sp.inventory.offhand)),
'''
p5_new = '''        ("Items".to_string(), stacks(&sp.inventory.items)),
        (
            "PotionKinds".to_string(),
            mc_protocol::nbt::Nbt::List(
                sp.potion_kinds
                    .iter()
                    .map(|k| mc_protocol::nbt::Nbt::Int(*k))
                    .collect(),
            ),
        ),
        ("Armor".to_string(), stacks(&sp.inventory.armor)),
        ("Offhand".to_string(), stacks(&sp.inventory.offhand)),
'''
patch(p5_old, p5_new, 'P5 NBT save')

# ---------------------------------------------------------------- P6
# the parse: find the construction site and the match arm insertion
p6a_old = '''    Some(SavedPlayer {
        x: p[0],
        y: p[1],
        z: p[2],
        yaw: r[0],
        pitch: r[1],
        inventory: crate::drops::Inventory {
            items: items?.try_into().ok()?,
            armor: armor?.try_into().ok()?,
            offhand: offhand?.try_into().ok()?,
        },
        selected_hotbar: sel?,
'''
p6a_new = '''    Some(SavedPlayer {
        x: p[0],
        y: p[1],
        z: p[2],
        yaw: r[0],
        pitch: r[1],
        inventory: crate::drops::Inventory {
            items: items?.try_into().ok()?,
            armor: armor?.try_into().ok()?,
            offhand: offhand?.try_into().ok()?,
        },
        potion_kinds,
        selected_hotbar: sel?,
'''
patch(p6a_old, p6a_new, 'P6a parse construction')

# the parse locals: the match loop head — insert the bank local after
# the xp_total local (positional splice — the whole-src edit, no truncation)
fn_head = 'pub fn player_state_from_nbt(root: &mc_protocol::nbt::Nbt) -> Option<SavedPlayer> {'
idx = src.index(fn_head)
anchor = '    let mut xp_total = 0i32;'
apos = src.index(anchor, idx)
insert_at = apos + len(anchor)
bank_local = '''
    // THE ROWS' OWN POTION-KIND BANK (the dev-199 round — the
    // "PotionKinds" list the elder file the missing row the NONE
    // parse keeps, the short list the pad fills, the over-long the
    // 36-cap truncates)
    let mut potion_kinds = [crate::brewing_sim::POTION_KIND_NONE; 36];'''
src = src[:insert_at] + bank_local + src[insert_at:]
count += 1
print('ok P6b parse local')

p6c_old = '''            "Items" => items = stacks_row_from_nbt(v, 36),
'''
p6c_new = '''            "Items" => items = stacks_row_from_nbt(v, 36),
            "PotionKinds" => {
                if let mc_protocol::nbt::Nbt::List(rows) = v {
                    for (i, row) in rows.iter().enumerate().take(36) {
                        if let mc_protocol::nbt::Nbt::Int(k) = row {
                            potion_kinds[i] = *k;
                        }
                    }
                }
            }
'''
patch(p6c_old, p6c_new, 'P6c parse arm')

# ---------------------------------------------------------------- P7
p7_old = '''                                if let Some(sp) = player_state_from_nbt(&root) {
                                    self.inventory = sp.inventory;
                                    self.selected_hotbar = sp.selected_hotbar.clamp(0, 8) as usize;
'''
p7_new = '''                                if let Some(sp) = player_state_from_nbt(&root) {
                                    self.inventory = sp.inventory;
                                    // THE KINDS' OWN RESTORE (the dev-199
                                    // round — the bank the same tag row the
                                    // restart resumes, the elder file the
                                    // NONE parse keeps)
                                    self.items_potion_kinds = sp.potion_kinds;
                                    self.selected_hotbar = sp.selected_hotbar.clamp(0, 8) as usize;
'''
patch(p7_old, p7_new, 'P7 boot restore')

# ---------------------------------------------------------------- P8
p8_old = '''            return Some(SavedPlayer {
                x: b.x,
                y: b.y,
                z: b.z,
                yaw: b.yaw,
                pitch: b.pitch,
                inventory: self.inventory,
                selected_hotbar: self.selected_hotbar as i32,
'''
p8_new = '''            return Some(SavedPlayer {
                x: b.x,
                y: b.y,
                z: b.z,
                yaw: b.yaw,
                pitch: b.pitch,
                inventory: self.inventory,
                potion_kinds: self.items_potion_kinds,
                selected_hotbar: self.selected_hotbar as i32,
'''
patch(p8_old, p8_new, 'P8 snapshot construction')

# ---------------------------------------------------------------- P9
p9_old = '''        if self.brewing_carried.1 > 0 {
            let (id, count) = self.brewing_carried;
            let mut rest = count;
            for it in self.inventory.items.iter_mut() {
                if rest == 0 {
                    break;
                }
                if it.1 > 0 && it.0 == id && it.1 < crate::world_view::item_max_stack(id) {
                    let take = (crate::world_view::item_max_stack(id) - it.1).min(rest);
                    it.1 += take;
                    rest -= take;
                }
            }
            for it in self.inventory.items.iter_mut() {
                if rest == 0 {
                    break;
                }
                if it.1 <= 0 {
                    *it = (id, rest);
                    rest = 0;
                }
            }
            if rest > 0 {
                self.spawn_drop_at_player(id, rest);
            }
            self.brewing_carried = (0, 0);
            // THE CARRIED KIND'S OWN DROP (the inventory wall — the
            // (id, count) pair the kind never carries, the home ride
            // the NONE wears)
            self.brewing_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
        }
'''
p9_new = '''        if self.brewing_carried.1 > 0 {
            let (id, count) = self.brewing_carried;
            let carried_kind = self.brewing_carried_kind;
            let mut rest = count;
            for it in self.inventory.items.iter_mut() {
                if rest == 0 {
                    break;
                }
                if it.1 > 0 && it.0 == id && it.1 < crate::world_view::item_max_stack(id) {
                    let take = (crate::world_view::item_max_stack(id) - it.1).min(rest);
                    it.1 += take;
                    rest -= take;
                }
            }
            for (idx, it) in self.inventory.items.iter_mut().enumerate() {
                if rest == 0 {
                    break;
                }
                if it.1 <= 0 {
                    *it = (id, rest);
                    rest = 0;
                    // THE KIND'S OWN RIDE HOME (the dev-199 round — the
                    // kinded carried the receiving row the seat banks;
                    // the non-potion carried the NONE seat the same
                    // value wears)
                    self.items_potion_kinds[idx] = carried_kind;
                }
            }
            if rest > 0 {
                self.spawn_drop_at_player(id, rest);
            }
            self.brewing_carried = (0, 0);
            // THE CARRIED KIND'S OWN DROP (the ride home the seat the
            // rows bank; the dropped remainder the entity the kind
            // never carries [the same honest note the thrown carried
            // speaks] — the carried word the NONE wears)
            self.brewing_carried_kind = crate::brewing_sim::POTION_KIND_NONE;
        }
'''
patch(p9_old, p9_new, 'P9 close home ride')

with open(PATH, 'w') as f:
    f.write(src)
print(f"all {count} patches applied")
