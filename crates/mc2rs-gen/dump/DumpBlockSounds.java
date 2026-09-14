import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.SoundType;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.sounds.SoundEvent;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.PrintWriter;

/**
 * mc2rs runtime dumper — OUR source (clean-room tooling, no Mojang code is
 * copied into the repository). Run against the official client jar + the
 * official library classpath (sha1-verified by the generator); it boots the
 * real vanilla registries in-process and prints ONE line per BLOCK (the
 * true registry order), the sound arc's own data the client's interaction
 * lanes consume:
 *
 *   block_name|volume|pitch|break|step|place|hit|fall
 *
 * The five sound events ride the block's DEFAULT state through its own
 * SoundType (`st.getSoundType()`) — the same object the vanilla sound calls
 * read (javap MultiPlayerGameMode.continueDestroyBlock: the dig beat plays
 * `getHitSound()` at `(getVolume()+1.0f)/8.0f` volume and `getPitch()*0.5f`
 * pitch; javap LevelEventHandler.levelEvent case 2001: the break plays
 * `getBreakSound()` at `(getVolume()+1.0f)/2.0f` and `getPitch()*0.8f`;
 * javap BlockItem: the place plays `getPlaceSound()` at the same
 * `(v+1)/2` / `p*0.8` shape; javap Entity.playStepSound: the footsteps play
 * `getStepSound()` at `getVolume()*0.15f` and `getPitch()`; javap
 * LivingEntity.playBlockFallSound: the hurt fall plays `getFallSound()` at
 * `getVolume()*0.5f` and `getPitch()`). The event ids are the SOUND_EVENT
 * registry keys ("block.stone.hit" — the same ids sounds.json speaks, so
 * the generated sound-event table resolves them to real files). The
 * volume/pitch pair rides the getters (the per-material base numbers).
 * Generated, never remembered.
 */
public class DumpBlockSounds {
    private static String id(SoundEvent e) {
        var key = BuiltInRegistries.SOUND_EVENT.getKey(e);
        return key == null ? "none" : key.getPath();
    }

    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        PrintWriter out = new PrintWriter(new BufferedWriter(new FileWriter(args[0])));
        int n = 0;
        for (Block block : BuiltInRegistries.BLOCK) {
            var key = BuiltInRegistries.BLOCK.getKey(block);
            BlockState st = block.defaultBlockState();
            SoundType snd = st.getSoundType();
            out.println(key + "|" + snd.getVolume() + "|" + snd.getPitch()
                + "|" + id(snd.getBreakSound())
                + "|" + id(snd.getStepSound())
                + "|" + id(snd.getPlaceSound())
                + "|" + id(snd.getHitSound())
                + "|" + id(snd.getFallSound()));
            n++;
        }
        out.close();
        System.err.println("dumped " + n + " blocks");
    }
}
