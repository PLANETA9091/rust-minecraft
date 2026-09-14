/**
 * Clean-room wire-format dumper for the serverbound Container Click /
 * Container Close packets: boots the REAL vanilla registries, builds
 * REAL packet instances with known field values (the client's own hash
 * generator for the HashedStack prediction lanes) and serializes them
 * through the official jar's own STREAM_CODEC — printing the exact wire
 * bytes. No format knowledge is remembered anywhere: the running codec
 * is the source of truth for the wire.
 *
 * Everything rides pure reflection so this file compiles with NO
 * classpath at all (source-file mode); the client jar + its libraries
 * join at RUNTIME only.
 *
 * Run (source-file mode):
 *   java -cp <client.jar:libs/*> ReflectContainerClick.java <out-file>
 */
public class ReflectContainerClick {
    public static void main(String[] args) throws Exception {
        java.io.PrintStream out = new java.io.PrintStream(new java.io.FileOutputStream(args[0]));
        // the same boot sequence DumpPacketIds proves works in-process
        Class.forName("net.minecraft.SharedConstants")
                .getMethod("tryDetectVersion").invoke(null);
        Class<?> bootstrap = Class.forName("net.minecraft.server.Bootstrap");
        bootstrap.getMethod("bootStrap").invoke(null);
        bootstrap.getMethod("validate").invoke(null);
        out.println("boot ok");

        Class<?> packet = Class.forName("net.minecraft.network.protocol.game.ServerboundContainerClickPacket");
        Class<?> closePacket = Class.forName("net.minecraft.network.protocol.game.ServerboundContainerClosePacket");
        Class<?> input = Class.forName("net.minecraft.world.inventory.ContainerInput");
        Class<?> hstack = Class.forName("net.minecraft.network.HashedStack");
        Class<?> stack = Class.forName("net.minecraft.world.item.ItemStack");
        Class<?> streamCodec = Class.forName("net.minecraft.network.codec.StreamCodec");

        // -- enums -------------------------------------------------------
        Object[] inputs = input.getEnumConstants();
        MethodAcc.name(out, inputs);
        out.println("containerInput values: " + inputs.length);

        // -- codec handles (declared access: the codecs may be private) ---
        Object codec = MethodAcc.staticField(packet, "STREAM_CODEC");
        Object closeCodec = MethodAcc.staticField(closePacket, "STREAM_CODEC");
        Object slotsCodec = MethodAcc.staticField(packet, "SLOTS_STREAM_CODEC");
        Object stackCodec = MethodAcc.staticField(hstack, "STREAM_CODEC");
        MethodAcc.write(out, streamCodec, "codec methods");

        // -- a serialization buffer (the DumpPacketIds precedent, with the
        // REAL registry access — the item-holder lanes resolve ids through
        // the buf's RegistryAccess, so null would NPE the codec)
        Class<?> rfb = Class.forName("net.minecraft.network.RegistryFriendlyByteBuf");
        java.lang.reflect.Constructor<?> rfbCtor = rfb.getDeclaredConstructor(
                Class.forName("io.netty.buffer.ByteBuf"),
                Class.forName("net.minecraft.core.RegistryAccess"));
        Object registryAccess = Class.forName("net.minecraft.core.RegistryAccess")
                .getMethod("fromRegistryOfRegistries", Class.forName("net.minecraft.core.Registry"))
                .invoke(null, Class.forName("net.minecraft.core.registries.BuiltInRegistries")
                        .getField("REGISTRY").get(null));
        out.println("registry access: " + registryAccess.getClass().getName());
        Object buf = MethodAcc.newBuf(rfbCtor, registryAccess);

        // -- HashedStack lanes -------------------------------------------
        // The real hash generator is a client-internal (ClientPacketListener)
        // detail; the WIRE under audit is the codec's, so the ActualItem
        // variant is constructed directly — the official STREAM_CODEC still
        // serializes it (the same class the client's hash lane rides).
        Object emptyHashed = hstack.getField("EMPTY").get(null);
        out.println("HashedStack.EMPTY: " + emptyHashed);
        Class<?> actualItem = Class.forName("net.minecraft.network.HashedStack$ActualItem");
        Object diamondItem = Class.forName("net.minecraft.world.item.Items").getField("DIAMOND").get(null);
        Object appleItem = Class.forName("net.minecraft.world.item.Items").getField("APPLE").get(null);
        Object holder = MethodAcc.itemHolder(out, diamondItem);
        Object appleHolder = MethodAcc.itemHolder(out, appleItem);
        Object emptyPatch = Class.forName("net.minecraft.network.HashedPatchMap")
                .getDeclaredConstructor(java.util.Map.class, java.util.Set.class)
                .newInstance(java.util.Map.of(), java.util.Set.of());
        java.lang.reflect.Constructor<?> actualCtor = actualItem.getDeclaredConstructor(
                Class.forName("net.minecraft.core.Holder"), int.class,
                Class.forName("net.minecraft.network.HashedPatchMap"));
        Object hashedDiamonds = actualCtor.newInstance(holder, 3, emptyPatch);
        Object hashedApples = actualCtor.newInstance(appleHolder, 1, emptyPatch);
        out.println("HashedStack actual items built: " + hashedDiamonds + " / " + hashedApples);

        // -- the changed-slots map ----------------------------------------
        Class<?> mapClass = Class.forName("it.unimi.dsi.fastutil.ints.Int2ObjectOpenHashMap");
        Object emptyMap = mapClass.getDeclaredConstructor().newInstance();
        Object oneMap = mapClass.getDeclaredConstructor().newInstance();
        mapClass.getMethod("put", int.class, Object.class).invoke(oneMap, 7, hashedDiamonds);

        // -- serialize the isolated lanes first ----------------------------
        Object laneBuf = MethodAcc.newBuf(rfbCtor, registryAccess);
        MethodAcc.writeHex(out, streamCodec, stackCodec, laneBuf, emptyHashed, "HashedStack.EMPTY");
        MethodAcc.writeHex(out, streamCodec, stackCodec, laneBuf, hashedDiamonds, "HashedStack diamond x3");
        MethodAcc.writeHex(out, streamCodec, slotsCodec, laneBuf, emptyMap, "changed-slots empty map");
        MethodAcc.writeHex(out, streamCodec, slotsCodec, laneBuf, oneMap, "changed-slots {7: diamond x3}");

        // -- the packets ----------------------------------------------------
        java.lang.reflect.Constructor<?> ctor = packet.getDeclaredConstructor(
                int.class, int.class, short.class, byte.class,
                input, Class.forName("it.unimi.dsi.fastutil.ints.Int2ObjectMap"), hstack);
        Object pickupEmpty = ctor.newInstance(2, 7, (short) 13, (byte) 0,
                input.getField("PICKUP").get(null), emptyMap, emptyHashed);
        MethodAcc.writeHex(out, streamCodec, codec, laneBuf, pickupEmpty,
                "click container=2 state=7 slot=13 button=0 PICKUP empty/empty");
        Object pickupRight = ctor.newInstance(2, 7, (short) 13, (byte) 1,
                input.getField("PICKUP").get(null), oneMap, hashedApples);
        MethodAcc.writeHex(out, streamCodec, codec, laneBuf, pickupRight,
                "click container=2 state=7 slot=13 button=1 PICKUP {7:diamond x3}/apple x1");
        Object quickMove = ctor.newInstance(2, 7, (short) 5, (byte) 0,
                input.getField("QUICK_MOVE").get(null), emptyMap, emptyHashed);
        MethodAcc.writeHex(out, streamCodec, codec, laneBuf, quickMove,
                "click container=2 state=7 slot=5 button=0 QUICK_MOVE empty/empty");

        // -- the close packet ------------------------------------------------
        java.lang.reflect.Constructor<?> closeCtor = closePacket.getDeclaredConstructor(int.class);
        Object close = closeCtor.newInstance(2);
        MethodAcc.writeHex(out, streamCodec, closeCodec, laneBuf, close, "close container=2");

        out.println("done");
        out.close();
    }

    /** Tiny reflection helpers (the dumper's only plumbing). */
    static final class MethodAcc {
        /** A fresh scratch buffer over the real registry access. */
        static Object newBuf(java.lang.reflect.Constructor<?> rfbCtor, Object registryAccess)
                throws Exception {
            return rfbCtor.newInstance(
                    Class.forName("io.netty.buffer.Unpooled").getMethod("buffer").invoke(null),
                    registryAccess);
        }

        /** An Item's built-in registry Holder (the ActualItem ctor's first arg). */
        static Object itemHolder(java.io.PrintStream out, Object item) throws Exception {
            for (String m : new String[] {"builtInRegistryHolder"}) {
                try {
                    return item.getClass().getMethod(m).invoke(item);
                } catch (NoSuchMethodException ignored) {
                    // the mapping's name differs — the fallback tries the registry
                }
            }
            Object registries = Class.forName("net.minecraft.core.registries.BuiltInRegistries")
                    .getField("ITEM").get(null);
            return registries.getClass().getMethod("wrapAsHolder", Object.class).invoke(registries, item);
        }

        /** A static field by name (declared access — the codecs may be private). */
        static Object staticField(Class<?> owner, String name) throws Exception {
            java.lang.reflect.Field f = owner.getDeclaredField(name);
            f.setAccessible(true);
            return f.get(null);
        }

        static void name(java.io.PrintStream out, Object[] enums) {
            for (Object e : enums) {
                try {
                    out.println("  ContainerInput." + e.getClass().getMethod("name").invoke(e));
                } catch (Exception ex) {
                    out.println("  <enum name failed>");
                }
            }
        }

        static void write(java.io.PrintStream out, Class<?> codecClass, String label) {
            for (java.lang.reflect.Method m : codecClass.getDeclaredMethods()) {
                out.println("  " + label + ": " + m.getName() + " " + m.getParameterCount());
            }
        }

        /** encode, hex-dump, and reset the shared scratch buffer. */
        static void writeHex(java.io.PrintStream out, Class<?> codecClass, Object codec,
                Object buf, Object value, String label) throws Exception {
            java.lang.reflect.Method encode = null;
            for (java.lang.reflect.Method m : codec.getClass().getMethods()) {
                Class<?>[] ps = m.getParameterTypes();
                if (ps.length == 2 && m.getReturnType() == void.class
                        && !java.lang.reflect.Modifier.isStatic(m.getModifiers())
                        && ps[0].isInstance(buf) && ps[1].isInstance(value)) {
                    encode = m;
                    if (m.getName().equals("encode")) break; // the mapping's own name wins
                }
            }
            if (encode == null) {
                throw new NoSuchMethodException("no 2-arg void instance method on " + codec.getClass());
            }
            encode.setAccessible(true);
            try {
                encode.invoke(codec, buf, value);
            } catch (java.lang.reflect.InvocationTargetException e) {
                Throwable c = e.getCause();
                out.println("  encode failed for " + label + ": " + c);
                throw e;
            }
            int n = (Integer) buf.getClass().getMethod("writerIndex").invoke(buf);
            StringBuilder hex = new StringBuilder();
            for (int i = 0; i < n; i++) {
                int b = (Byte) buf.getClass().getMethod("getByte", int.class).invoke(buf, i) & 0xFF;
                hex.append(String.format("%02x", b));
                if (i % 2 == 1) hex.append(' ');
            }
            out.println(label + " [" + n + " B]: " + hex);
            buf.getClass().getMethod("clear").invoke(buf);
        }
    }
}
