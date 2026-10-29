#!/usr/bin/env python3
"""the autopush-333 round's enchanting fn battery (the brewing 332
shapes the same lanes wear — the transient pair the store-less close
the clearContainer law speaks, the 176x166 standard canvas the
inherited imageWidth/imageHeight keeps)."""
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


# ---- A. the door fns (before the beacon open doc) ----
rep(
    "    /// THE BEACON'S OWN OFFLINE OPEN (the autopush-322 round — javap",
    '''    /// THE ENCHANTING TABLE'S OWN DOOR (javap 26.2 EnchantmentTableBlock
    /// the openMenu walk the EnchantmentMenu's MenuProvider answer —
    /// the right-press on the placed enchanting table the OFFLINE
    /// world answers — the window the table's own 176x166 canvas
    /// opens, the TRANSIENT pair the SimpleContainer(2) the fresh
    /// seats lay [the vanilla menu the table's own BE never keeps —
    /// NO per-cell store the chest's map law never wears, the close
    /// the pair the ride-home walk returns]).
    pub fn open_enchanting_for(&mut self, pos: (i32, i32, i32)) {
        self.enter_world();
        // THE STALE CELL'S OWN RELEASE FIRST (the chest open walk's
        // own law the brewing lane wears)
        self.barrel_lid_released();
        // the OFFLINE stand-in: the live lane's flag stays down (the
        // enchanting door the wire session never wears)
        self.chest_is_live = false;
        self.chest_mirror.clear();
        // THE TRANSIENT PAIR (the EnchantmentMenu's own SimpleContainer(2)
        // the fresh open lays — the item 0 the lapis 1 the menu
        // numbering banks)
        self.enchant_slots = vec![(0, 0), (0, 0)];
        self.enchant_carried = (0, 0);
        // the player's own rows the REAL inventory projects (the
        // menu's addStandardInventorySlots(8, 84) the mirror rows the
        // same projection keeps)
        self.enchant_mirror_sync_from_inventory();
        // THE FAMILY'S OWN VOICE (the counter law the chest walk keeps
        // — the enchanting kind the vanilla's own silence arm the
        // emit call returns)
        self.container_voice_emit(true);
        self.open(ScreenKind::Enchanting);
    }

    /// THE ENCHANTING TABLE'S OWN MIRROR SYNC (the chest's own law — the
    /// projection the inventory IS: the main rows the items 9..35
    /// speak, the hotbar row the items 0..8, the fresh rows EVERY
    /// mutation re-syncs).
    pub fn enchant_mirror_sync_from_inventory(&mut self) {
        if self.enchant_mirror.len() != 36 {
            self.enchant_mirror.resize(36, (0, 0));
        }
        for i in 0..27 {
            self.enchant_mirror[i] = self.inventory.items[9 + i];
        }
        for i in 0..9 {
            self.enchant_mirror[27 + i] = self.inventory.items[i];
        }
        self.hotbar_slots = self.inventory.hotbar();
    }

    /// THE ENCHANTING TABLE'S OWN CLOSE (the vanilla Escape/E on the
    /// enchanting table's screen — the chest's own close walk the
    /// carried return speaks, the EnchantmentMenu.removed()'s
    /// clearContainer the PAIR home returns: the same-item merges, the
    /// first-empty pass, the full store the remainder drops — the
    /// close_brewing shape, the NO store flush the table's BE never
    /// keeps), the screen back to the world.
    pub fn close_enchanting(&mut self, back: ScreenKind) {
        // THE CARRIED'S OWN HOME RIDE (the vanilla removed()'s
        // placeItemBackInInventory walk — the close_brewing shape)
        if self.enchant_carried.1 > 0 {
            let (id, count) = self.enchant_carried;
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
            self.enchant_carried = (0, 0);
        }
        // THE PAIR'S OWN HOME RIDE (the removed()'s clearContainer walk —
        // the transient seats the close NEVER banks: each held stack
        // the same merge/first-empty/drop lane returns)
        for seat in 0..2usize {
            let (id, count) = self.enchant_slots[seat];
            if count <= 0 {
                continue;
            }
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
            self.enchant_slots[seat] = (0, 0);
        }
        self.container_removed_rode = true; // the removed law rode
        self.open(back);
    }

    /// THE BEACON'S OWN OFFLINE OPEN (the autopush-322 round — javap''',
    1,
    "door fns",
)

# ---- B. the hover/highlight fns (before the F4 doc) ----
rep(
    "    /// THE F4 SWITCHER'S OWN RENDER (javap 26.2 GameModeSwitcherScreen",
    '''    /// THE ENCHANTING TABLE'S OWN HOVER SEAT (the autopush-333 round —
    /// the same shared law the furnace + chest + hopper + dispenser +
    /// brewing walks bank): the hovered seat the rects the last render
    /// laid answer — the strict 16x16 predicate the inner origins the
    /// +1 inset the Slot.x keeps, the table's pair the mirror rows the
    /// menu numbering banks. The find the FIRST match the
    /// non-overlapping cells keep.
    fn enchant_hover_seat(&self, mx: i32, my: i32) -> Option<(i32, i32, i16)> {
        self.enchant_slot_rects
            .iter()
            .find(|&&(x, y, _)| mx >= x && mx < x + 16 && my >= y && my < y + 16)
            .copied()
    }

    /// THE ENCHANTING TABLE'S OWN HIGHLIGHT SEAT (the autopush-333 round
    /// — the same shared AbstractContainerScreen law the beacon +
    /// inventory + table + furnace + chest + hopper + dispenser +
    /// brewing rounds bank, the EnchantmentScreen the plain extension
    /// the subclass absence banks): the hovered seat's INNER origin the
    /// enchant_hover_seat walk's own verdict answers — the ONE hover
    /// walk the clicks and the drops and the drags share. The NO-GATE
    /// law the 325 round banks: NO carried wall, NO hasItem gate —
    /// the empty cell the highlight answers. The (ox, oy) the paint
    /// serves — None the gates speak.
    fn enchant_highlight_origin(&self, mx: i32, my: i32) -> Option<(i32, i32)> {
        if self.screen != ScreenKind::Enchanting {
            return None; // the enchanting's own screen the walk serves
        }
        let (sx, sy, _) = self.enchant_hover_seat(mx, my)?;
        Some((sx, sy)) // the rects the +1 inset the render banks
    }

    /// THE F4 SWITCHER'S OWN RENDER (javap 26.2 GameModeSwitcherScreen''',
    1,
    "hover fns",
)

open(P, "w").write(s)
print("battery A+B done")
