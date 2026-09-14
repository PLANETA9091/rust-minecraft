import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints ONE line per BLOCK (the
 * true registry order), the dig lane's own data the client's destroy arc
 * consumes:
 *
 *   block_name|destroySpeed|requiresCorrectToolForDrops
 *
 * The two numbers ride the block's DEFAULT state through its own public
 * queries — `getDestroySpeed(null, null)` (the bytecode reads the state's
 * destroySpeed field verbatim, the BlockGetter/BlockPos arguments unused)
 * and `requiresCorrectToolForDrops()`. The destroy progress law the client
 * speaks (javap BlockBehaviour.getDestroyProgress): the per-tick fraction =
 * playerDigSpeed / destroySpeed / (correctTool ? 30 : 100), with
 * destroySpeed -1.0f (bedrock) pinning the progress at zero and 0.0f
 * (leaf_litter) dividing to infinity — the vanilla insta-break.
 * Generated, never remembered.
 */
public class DumpBlockDig {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int n = 0;
        for (Block block : BuiltInRegistries.BLOCK) {
            var key = BuiltInRegistries.BLOCK.getKey(block);
            BlockState st = block.defaultBlockState();
            float speed = st.getDestroySpeed(null, null);
            boolean requiresTool = st.requiresCorrectToolForDrops();
            out.println(key + "|" + speed + "|" + requiresTool);
            n++;
        }
        out.close();
        System.err.println("dumped " + n + " blocks");
    }
}
