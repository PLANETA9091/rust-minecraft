import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.IdMapper;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.Property;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints ONE line per global block
 * state id, in true registry order:
 *
 *   id|block_name|prop=val,prop=val,...
 *
 * This is the bytecode-true global state id table the client's id mapper
 * consumes — generated, never remembered. No game assets or code are read
 * into the repository, only the name/structure mapping the dump emits.
 */
public class DumpBlockStates {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        IdMapper<BlockState> reg = Block.BLOCK_STATE_REGISTRY;
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int n = reg.size();
        for (int id = 0; id < n; id++) {
            BlockState st = reg.byId(id);
            var key = BuiltInRegistries.BLOCK.getKey(st.getBlock());
            StringBuilder sb = new StringBuilder();
            for (Property<?> p : st.getProperties()) {
                if (sb.length() > 0) sb.append(',');
                sb.append(p.getName()).append('=').append(st.getValue(p));
            }
            out.println(id + "|" + key + "|" + sb);
        }
        out.close();
        System.err.println("dumped " + n + " block states");
    }
}
