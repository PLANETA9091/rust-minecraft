import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints ONE line per BLOCK (the
 * true registry order), the explosion lane's own data the client's blast
 * arc consumes:
 *
 *   block_name|explosionResistance
 *
 * The number rides the block's own public query — `getExplosionResistance()`
 * (the ExplosionDamageCalculator.getBlockExplosionResistance fold's own
 * read: the max(block, fluid) the Optional arm serves, the isAir &&
 * fluid-empty cells the Optional.empty path keeps). Generated, never
 * remembered.
 */
public class DumpBlockResistance {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int n = 0;
        for (Block block : BuiltInRegistries.BLOCK) {
            var key = BuiltInRegistries.BLOCK.getKey(block);
            out.println(key + "|" + block.getExplosionResistance());
            n++;
        }
        out.close();
        System.err.println("dumped " + n + " blocks");
    }
}
