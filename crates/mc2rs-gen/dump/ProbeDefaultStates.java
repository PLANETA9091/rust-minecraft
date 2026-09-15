import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.Property;

import java.util.Collection;

/**
 * mc2rs probe — the DEFAULT state's property map for the client palette's
 * own blocks: the loot lane's `block_state_property` conditions resolve
 * against the block's default state (the vanilla LootItemBlockStatePropertyCondition
 * reads the ORIGIN state; the survival break speaks the default — our
 * break lane removes the cell's own state, the default stands in for the
 * procedural world's uniform states). Clean-room runtime query. One line
 * per block:
 *
 *   block_name|prop=value,prop=value...
 */
public class ProbeDefaultStates {
    // the client's 26-kind world palette (crates/mc-client/src/world.rs NAME_OF)
    static final String[] PALETTE = {
        "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water",
        "sand", "granite", "diorite", "andesite", "deepslate",
        "cobbled_deepslate", "gravel", "bedrock", "coal_ore", "copper_ore",
        "iron_ore", "gold_ore", "redstone_ore", "lapis_ore", "diamond_ore",
        "emerald_ore", "netherrack", "pale_oak_leaves", "leaf_litter"
    };

    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        for (String name : PALETTE) {
            Block block = BuiltInRegistries.BLOCK.getValue(net.minecraft.resources.Identifier.parse("minecraft:" + name));
            BlockState st = block.defaultBlockState();
            StringBuilder sb = new StringBuilder(name + "|");
            Collection<Property<?>> props = st.getProperties();
            boolean first = true;
            for (Property<?> p : props) {
                if (!first) sb.append(',');
                sb.append(p.getName()).append('=').append(st.getValue(p));
                first = false;
            }
            System.out.println(sb);
        }
    }
}
