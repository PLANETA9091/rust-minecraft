import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.Registry;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.RegistryDataLoader;
import net.minecraft.server.packs.PackLocationInfo;
import net.minecraft.server.packs.PackType;
import net.minecraft.server.packs.VanillaPackResources;
import net.minecraft.server.packs.VanillaPackResourcesBuilder;
import net.minecraft.server.packs.repository.PackSource;
import net.minecraft.server.packs.resources.MultiPackResourceManager;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;
import java.util.List;
import java.util.Optional;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process — INCLUDING the data-driven
 * synchronized registries (dimension_type, worldgen/biome, timeline,
 * world_clock, ...) which `RegistryDataLoader` loads from the jar's own
 * vanilla data pack — and prints ONE line per registry entry, in true
 * registration order:
 *
 *   registry_name|entry_id|entry_name
 *
 * This is the bytecode-true WIRE-ORDER evidence for the registries a
 * vanilla server syncs: the server assigns synced-registry ids in this
 * same built-in order, so a client claiming its known packs must carry
 * exactly this order. No game assets or code are read into the repository,
 * only the name/order structure the dump emits.
 */
public class DumpRegistries {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();

        // the vanilla data pack: the client jar's own data/ directory
        VanillaPackResources pack = new VanillaPackResourcesBuilder()
            .pushJarResources()
            .exposeNamespace("minecraft")
            .build(new PackLocationInfo("vanilla", Component.literal("vanilla"),
                PackSource.BUILT_IN, Optional.empty()));
        MultiPackResourceManager manager =
            new MultiPackResourceManager(PackType.SERVER_DATA, List.of(pack));

        // the static layer's lookups feed the data registries' references —
        // TAG-AWARE, the same two-step the vanilla WorldLoader runs: block
        // tags (infiniburn etc.) resolve through the pending-tags lookups
        RegistryAccess.Frozen staticAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
        java.util.List<net.minecraft.core.HolderLookup.RegistryLookup<?>> lookups = new java.util.ArrayList<>();
        java.util.Map<Object, net.minecraft.core.HolderLookup.RegistryLookup<?>> tagged =
            new java.util.HashMap<>();
        for (Object pt : net.minecraft.tags.TagLoader.loadTagsForExistingRegistries(manager, staticAccess)) {
            net.minecraft.core.Registry.PendingTags<?> p = (net.minecraft.core.Registry.PendingTags<?>) pt;
            tagged.put(p.key().identifier(), p.lookup());
        }
        for (Object e : staticAccess.registries().toList()) {
            RegistryAccess.RegistryEntry<?> entry = (RegistryAccess.RegistryEntry<?>) e;
            var l = tagged.get(entry.key().identifier());
            lookups.add(l != null ? l : (net.minecraft.core.HolderLookup.RegistryLookup<?>) entry.value());
        }

        // the SYNCHRONIZED list is exactly what a vanilla server streams
        // during configuration — the registries whose wire order matters
        RegistryAccess.Frozen loaded = RegistryDataLoader
            .load(manager, lookups, RegistryDataLoader.SYNCHRONIZED_REGISTRIES, Runnable::run)
            .join();

        // raw types: per-registry captures (Registry<?>) would fight
        // getKey/byId — the erased calls are exactly what we want
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int entries = 0;
        int registries = 0;
        for (Object entryObj : loaded.registries().toList()) {
            RegistryAccess.RegistryEntry<?> entry = (RegistryAccess.RegistryEntry<?>) entryObj;
            Registry reg = (Registry) entry.value();
            String regKey = entry.key().identifier().toString();
            int n = reg.size();
            for (int id = 0; id < n; id++) {
                Object e = reg.byId(id);
                out.println(regKey + "|" + id + "|" + reg.getKey(e));
                entries++;
            }
            registries++;
        }
        out.close();
        System.err.println("dumped " + entries + " entries across " + registries
            + " synchronized registries");
    }
}
