import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints ONE line per registry
 * entry, in true registration order:
 *
 *   registry_name|entry_id|entry_name
 *
 * The root registry-of-registries is iterated whole, so EVERY built-in
 * registry rides (blocks, items, biomes, dimension types, timelines, ...).
 * This is the bytecode-true WIRE-ORDER evidence: a vanilla server assigns
 * synced-registry ids in this same built-in order, so a client claiming
 * its known packs must carry exactly this order. No game assets or code
 * are read into the repository, only the name/order structure the dump
 * emits.
 */
public class DumpRegistries {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        // raw types: the root is a Registry OF registries, and per-member
        // captures (Registry<?>) would fight getKey/byId — the erased calls
        // are exactly what we want for a dumper
        Registry root = BuiltInRegistries.REGISTRY;
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int entries = 0;
        int registries = 0;
        for (Object member : root) {
            Registry reg = (Registry) member;
            Object regKey = root.getKey(member);
            int n = reg.size();
            for (int id = 0; id < n; id++) {
                Object entry = reg.byId(id);
                out.println(regKey + "|" + id + "|" + reg.getKey(entry));
                entries++;
            }
            registries++;
        }
        out.close();
        System.err.println("dumped " + entries + " entries across " + registries + " registries");
    }
}
