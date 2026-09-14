/**
 * Clean-room wire-format dumper for the serverbound Set Carried Item
 * packet: boots the REAL vanilla registries, builds REAL packet
 * instances for the hotbar selection domain's ends and middle, and
 * serializes them through the official jar's own STREAM_CODEC —
 * printing the exact wire bytes. No format knowledge is remembered
 * anywhere: the running codec is the source of truth for the wire
 * (the ReflectContainerClick arc's own pattern, one lane).
 *
 * Everything rides pure reflection so this file compiles with NO
 * classpath at all (source-file mode); the client jar + its libraries
 * join at RUNTIME only.
 *
 * Run (source-file mode):
 *   java -cp <client.jar:libs/*> ReflectSetCarriedItem.java <out-file>
 */
public class ReflectSetCarriedItem {
    public static void main(String[] args) throws Exception {
        java.io.PrintStream out = new java.io.PrintStream(new java.io.FileOutputStream(args[0]));
        // the same boot sequence DumpPacketIds proves works in-process
        Class.forName("net.minecraft.SharedConstants")
                .getMethod("tryDetectVersion").invoke(null);
        Class<?> bootstrap = Class.forName("net.minecraft.server.Bootstrap");
        bootstrap.getMethod("bootStrap").invoke(null);
        bootstrap.getMethod("validate").invoke(null);
        out.println("boot ok");

        Class<?> packet = Class.forName(
                "net.minecraft.network.protocol.game.ServerboundSetCarriedItemPacket");
        Class<?> streamCodec = Class.forName("net.minecraft.network.codec.StreamCodec");
        Object codec = MethodAcc.staticField(packet, "STREAM_CODEC");

        // the ctor's parameter list IS evidence: the packet's own fields
        java.lang.reflect.Constructor<?> ctor = packet.getDeclaredConstructor(int.class);
        out.println("ctor: " + ctor);

        // a scratch buffer over the real registry access (this packet
        // has no registry lanes, but the RegistryFriendlyByteBuf IS a
        // ByteBuf — the same buf the click dumper rides works)
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

        // the selection domain's ends and middle (Inventory.selected 0..8)
        for (int slot : new int[] {0, 3, 8}) {
            Object pkt = ctor.newInstance(slot);
            MethodAcc.writeHex(out, streamCodec, codec, buf, pkt,
                    "set carried item slot=" + slot);
        }

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

        /** A static field by name (declared access — the codecs may be private). */
        static Object staticField(Class<?> owner, String name) throws Exception {
            java.lang.reflect.Field f = owner.getDeclaredField(name);
            f.setAccessible(true);
            return f.get(null);
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
