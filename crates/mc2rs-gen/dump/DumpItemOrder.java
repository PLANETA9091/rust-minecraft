import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.registries.BuiltInRegistries;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints the ITEM built-in registry
 * in true registration order, ONE line per entry:
 *
 *   id|name
 *
 * The item registry is NOT a synchronized registry (DumpRegistries covers
 * the configuration-synced set): the client resolves it LOCALLY — the
 * holderRegistry(Registries.ITEM) serializers (the container lanes' item
 * stacks, the equipment lane) ride the built-in registration order, so the
 * wire ids a vanilla server sends are THIS boot's 0-based ids. The assets
 * table (the item model JSONs' alphabetical order) is a TEXTURE vocabulary,
 * NOT the wire — the chest screen's live icons resolve through THIS table.
 * No game assets or code are read into the repository, only the name/order
 * structure the dump emits.
 */
public class DumpItemOrder {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        PrintWriter out = args.length > 0
            ? new PrintWriter(new BufferedWriter(new FileWriter(args[0])))
            : new PrintWriter(System.out, true);
        var reg = BuiltInRegistries.ITEM;
        int entries = 0;
        for (var holder : reg) {
            int id = reg.getId(holder);
            out.println(id + "|" + reg.getKey(holder));
            entries++;
        }
        out.close();
        if (args.length > 0) {
            System.err.println("dumped " + entries + " items");
        } else {
            System.exit(0);
        }
    }
}
