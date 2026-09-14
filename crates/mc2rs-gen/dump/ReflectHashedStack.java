/**
 * Clean-room wire-format dumper for the HashedStack prediction lane's
 * NON-EMPTY shapes: the container-click round (`ReflectContainerClick.java`)
 * pinned only the EMPTY hashed patch map (`00 00`) — a non-empty map is
 * real wire the client must be able to speak. This dumper boots the REAL
 * vanilla registries and serializes HashedPatchMaps keyed by REAL
 * DataComponentTypes (the codec resolves the keys through the
 * data_component_type registry — the synthetic-int-key probe was REFUSED
 * with "Can't find id for '19' in map Registry[minecraft:data_component_type]")
 * with known hash values (deliberate negatives and a large value — the
 * width probes that pinned the hash lane as a fixed big-endian int32,
 * NOT a VarInt) through the official jar's own STREAM_CODEC — printing
 * the exact wire bytes.
 *
 * The REAL hash generator (the stack-to-HashedStack factory) is a
 * client-internal lane — the same decision ReflectContainerClick
 * documented: the WIRE under audit is the codec's, so the ActualItem
 * variants are constructed directly. (The direct ItemStack factory probe
 * additionally NPEs in this boot — "Components not bound yet" — the
 * lazy component binding the plain bootstrap does not reach.)
 *
 * Everything rides pure reflection so this file compiles with NO
 * classpath at all (source-file mode); the client jar + its libraries
 * join at RUNTIME only.
 *
 * Run (source-file mode):
 *   java -cp <client.jar:libs/*> ReflectHashedStack.java <out-file>
 */
public class ReflectHashedStack {
    public static void main(String[] args) throws Exception {
        java.io.PrintStream out = new java.io.PrintStream(new java.io.FileOutputStream(args[0]));
        // the same boot sequence DumpPacketIds proves works in-process
        Class.forName("net.minecraft.SharedConstants")
                .getMethod("tryDetectVersion").invoke(null);
        Class<?> bootstrap = Class.forName("net.minecraft.server.Bootstrap");
        bootstrap.getMethod("bootStrap").invoke(null);
        bootstrap.getMethod("validate").invoke(null);
        out.println("boot ok");

        Class<?> hstack = Class.forName("net.minecraft.network.HashedStack");
        Class<?> actualItem = Class.forName("net.minecraft.network.HashedStack$ActualItem");
        Class<?> patchMap = Class.forName("net.minecraft.network.HashedPatchMap");
        Object stackCodec = MethodAcc.staticField(hstack, "STREAM_CODEC");

        // -- the record shapes (the field ORDER and types, straight from
        // the running classes) -----------------------------------------
        MethodAcc.recordShape(out, actualItem, "HashedStack.ActualItem");
        MethodAcc.recordShape(out, patchMap, "HashedPatchMap");

        // -- the registry access (the item-holder lanes resolve ids
        // through the buf's RegistryAccess, so null would NPE the codec)
        Class<?> rfb = Class.forName("net.minecraft.network.RegistryFriendlyByteBuf");
        java.lang.reflect.Constructor<?> rfbCtor = rfb.getDeclaredConstructor(
                Class.forName("io.netty.buffer.ByteBuf"),
                Class.forName("net.minecraft.core.RegistryAccess"));
        Object registryAccess = Class.forName("net.minecraft.core.RegistryAccess")
                .getMethod("fromRegistryOfRegistries", Class.forName("net.minecraft.core.Registry"))
                .invoke(null, Class.forName("net.minecraft.core.registries.BuiltInRegistries")
                        .getField("REGISTRY").get(null));
        Object buf = MethodAcc.newBuf(rfbCtor, registryAccess);

        Object diamondItem = Class.forName("net.minecraft.world.item.Items").getField("DIAMOND").get(null);
        Object diamondHolder = MethodAcc.itemHolder(diamondItem);
        java.lang.reflect.Constructor<?> actualCtor = actualItem.getDeclaredConstructor(
                Class.forName("net.minecraft.core.Holder"), int.class,
                patchMap);
        java.lang.reflect.Constructor<?> patchCtor = patchMap.getDeclaredConstructor(
                java.util.Map.class, java.util.Set.class);

        // -- the component types (the added map's REAL keys — the codec
        // resolves them through the data_component_type registry)
        Class<?> dcClass = Class.forName("net.minecraft.core.component.DataComponents");
        Object damageType = dcClass.getField("DAMAGE").get(null);
        Object customNameType = dcClass.getField("CUSTOM_NAME").get(null);
        MethodAcc.typeId(out, damageType, "DAMAGE");
        MethodAcc.typeId(out, customNameType, "CUSTOM_NAME");
        MethodAcc.dataComponentFields(out);

        // -- the synthetic patch maps: the wire SHAPE under real keys ---
        // the deliberate hash values double as WIDTH probes: a negative
        // hash and a large one separate VarInt from the fixed big-endian
        // int32 the codec actually writes.
        Object emptyPatch = patchCtor.newInstance(java.util.Map.of(), java.util.Set.of());
        Object addedOnly = patchCtor.newInstance(
                java.util.Map.of(damageType, 12345), java.util.Set.of());
        Object removedOnly = patchCtor.newInstance(
                java.util.Map.of(), java.util.Set.of(customNameType));
        java.util.Map<Object, Object> twoAdded = new java.util.LinkedHashMap<>();
        twoAdded.put(damageType, 12345);       // positive small
        twoAdded.put(customNameType, -99);     // negative (the width probe)
        Object addedAndRemoved = patchCtor.newInstance(
                twoAdded, java.util.Set.of(customNameType));
        Object bigValue = patchCtor.newInstance(
                java.util.Map.of(damageType, 123456789), java.util.Set.of());
        MethodAcc.writeHex(out, stackCodec, buf,
                actualCtor.newInstance(diamondHolder, 64, emptyPatch),
                "synthetic baseline diamond x64 empty patch");
        MethodAcc.writeHex(out, stackCodec, buf,
                actualCtor.newInstance(diamondHolder, 64, addedOnly),
                "synthetic patch added{DAMAGE:12345} on diamond x64");
        MethodAcc.writeHex(out, stackCodec, buf,
                actualCtor.newInstance(diamondHolder, 64, removedOnly),
                "synthetic patch removed{CUSTOM_NAME} on diamond x64");
        MethodAcc.writeHex(out, stackCodec, buf,
                actualCtor.newInstance(diamondHolder, 64, addedAndRemoved),
                "synthetic patch added{DAMAGE:12345,CUSTOM_NAME:-99} removed{CUSTOM_NAME} on diamond x64");
        MethodAcc.writeHex(out, stackCodec, buf,
                actualCtor.newInstance(diamondHolder, 64, bigValue),
                "synthetic patch added{DAMAGE:123456789} on diamond x64");

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
        static Object itemHolder(Object item) throws Exception {
            try {
                return item.getClass().getMethod("builtInRegistryHolder").invoke(item);
            } catch (NoSuchMethodException ignored) {
                // the mapping's name differs — the fallback wraps via the registry
            }
            Object registries = Class.forName("net.minecraft.core.registries.BuiltInRegistries")
                    .getField("ITEM").get(null);
            return registries.getClass().getMethod("wrapAsHolder", Object.class)
                    .invoke(registries, item);
        }

        /** A static field by name (declared access — the codecs may be private). */
        static Object staticField(Class<?> owner, String name) throws Exception {
            java.lang.reflect.Field f = owner.getDeclaredField(name);
            f.setAccessible(true);
            return f.get(null);
        }

        /** The record's component names + types (the field order, in class). */
        static void recordShape(java.io.PrintStream out, Class<?> record, String label) {
            StringBuilder sb = new StringBuilder();
            try {
                for (java.lang.reflect.RecordComponent rc : record.getRecordComponents()) {
                    if (sb.length() > 0) sb.append(", ");
                    sb.append(rc.getType().getSimpleName()).append(' ').append(rc.getName());
                }
                out.println(label + " record components: " + sb);
            } catch (Exception e) {
                // not a record — the declared fields speak instead
                sb = new StringBuilder();
                for (java.lang.reflect.Field f : record.getDeclaredFields()) {
                    if (sb.length() > 0) sb.append(", ");
                    sb.append(f.getType().getSimpleName()).append(' ').append(f.getName());
                }
                out.println(label + " fields: " + sb);
            }
        }

        /** A component type's registry id (the wire's own key lane). */
        static void typeId(java.io.PrintStream out, Object type, String label) {
            try {
                Object registries = Class.forName("net.minecraft.core.registries.BuiltInRegistries")
                        .getField("DATA_COMPONENT_TYPE").get(null);
                Object id = registries.getClass().getMethod("getId", Object.class)
                        .invoke(registries, type);
                out.println("data component type id (" + label + "): " + id);
            } catch (Exception e) {
                out.println("data component type id (" + label + "): FAILED " + e);
            }
        }

        /** The DataComponents lanes that exist in this boot (informative). */
        static void dataComponentFields(java.io.PrintStream out) throws Exception {
            StringBuilder sb = new StringBuilder();
            for (java.lang.reflect.Field f : Class.forName("net.minecraft.core.component.DataComponents")
                    .getDeclaredFields()) {
                if (!java.lang.reflect.Modifier.isStatic(f.getModifiers())) continue;
                if (sb.length() > 0) sb.append(' ');
                sb.append(f.getName());
            }
            out.println("DataComponents fields: " + sb);
        }

        /** encode, hex-dump, and reset the shared scratch buffer. */
        static void writeHex(java.io.PrintStream out, Object codec,
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
