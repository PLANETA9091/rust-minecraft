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
 * real vanilla registries in-process and prints the MOB_EFFECT built-in
 * registry in true registration order, ONE line per entry:
 *
 *   id|name|beneficial|color
 *
 * The extra columns carry the HUD's own laws: `beneficial` is
 * MobEffect.isBeneficial() (the category's != HARMFUL verdict — the HUD's
 * effect-icons row split: the beneficial icons ride the top line at y=1,
 * the harmful ones the second line 26px below), `color` is
 * MobEffect.getColor() (the int tint the MobEffectInstance.compareTo's
 * natural order uses as its final tie-break — the HUD sorts the icons in
 * REVERSE natural order, so the color is what separates same-length
 * same-ambient effects).
 *
 * The mob-effect registry is NOT a synchronized registry (DumpRegistries
 * covers the configuration-synced set): the client resolves it LOCALLY —
 * the `holderRegistry(Registries.MOB_EFFECT)` serializers ride the built-in
 * registration order, so the wire ids a vanilla server sends in the
 * effect packets are THIS boot's 0-based ids. The legacy 1-based wiki ids
 * are not the wire. No game assets or code are read into the repository,
 * only the name/order structure the dump emits.
 */
public class DumpMobEffects {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        PrintWriter out = args.length > 0
            ? new PrintWriter(new BufferedWriter(new FileWriter(args[0])))
            : new PrintWriter(System.out, true);
        var reg = BuiltInRegistries.MOB_EFFECT;
        int entries = 0;
        for (var effect : reg) {
            int id = reg.getId(effect);
            out.println(id + "|" + reg.getKey(effect) + "|" + effect.isBeneficial() + "|"
                + effect.getColor());
            entries++;
        }
        out.close();
        if (args.length > 0) {
            System.err.println("dumped " + entries + " mob effects");
        } else {
            System.exit(0);
        }
    }
}
