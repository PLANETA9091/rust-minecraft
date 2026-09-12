import io.netty.buffer.ByteBuf;
import net.minecraft.network.ProtocolInfo;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.PacketType;
import net.minecraft.network.protocol.SimpleUnboundProtocol;
import net.minecraft.network.protocol.UnboundProtocol;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.network.protocol.handshake.HandshakeProtocols;
import net.minecraft.network.protocol.login.LoginProtocols;
import net.minecraft.network.protocol.status.StatusProtocols;

import java.io.PrintWriter;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.function.Function;

/**
 * Clean-room packet id dumper: boots the REAL vanilla registries, binds
 * the official jar's own clientbound + serverbound protocols for EVERY
 * phase (play, login, configuration, status, handshake) and writes
 * "<flow> id identifier" lines from each built dispatch codec's live id
 * map. No ids are remembered anywhere — the running protocol is the
 * source of truth for the wire.
 *
 * Run (source-file mode): java -cp <client.jar:libs/*> DumpPacketIds.java <out-file>
 */
public class DumpPacketIds {
    public static void main(String[] args) throws Exception {
        // the same boot sequence DumpBlockStates proves works in-process
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();
        net.minecraft.server.Bootstrap.validate();

        Function<ByteBuf, RegistryFriendlyByteBuf> decorator =
                buf -> new RegistryFriendlyByteBuf(buf, null);
        Method codec = ProtocolInfo.class.getMethod("codec");
        Method simpleBind = SimpleUnboundProtocol.class.getMethod("bind", Function.class);

        try (PrintWriter out = new PrintWriter(args[0], "UTF-8")) {
            // play clientbound is a plain (context-free) protocol
            Object cbInfo = simpleBind.invoke(GameProtocols.CLIENTBOUND_TEMPLATE, decorator);
            dumpFlow(cbInfo, "cb", out, codec);
            // play serverbound binds through the context protocol with an
            // empty vanilla Context (hasInfiniteMaterials=false default path)
            Method sbBind = UnboundProtocol.class.getMethod("bind", Function.class, Object.class);
            Object sbInfo = sbBind.invoke(GameProtocols.SERVERBOUND_TEMPLATE, decorator,
                    (GameProtocols.Context) new GameProtocols.Context() {
                        @Override
                        public boolean hasInfiniteMaterials() {
                            return false;
                        }
                    });
            dumpFlow(sbInfo, "sb", out, codec);
            // the four pre-play phases are all context-free protocols —
            // the same SimpleUnboundProtocol bind as play clientbound
            dumpFlow(simpleBind.invoke(LoginProtocols.CLIENTBOUND_TEMPLATE, decorator), "login-cb", out, codec);
            dumpFlow(simpleBind.invoke(LoginProtocols.SERVERBOUND_TEMPLATE, decorator), "login-sb", out, codec);
            dumpFlow(simpleBind.invoke(ConfigurationProtocols.CLIENTBOUND_TEMPLATE, decorator), "config-cb", out, codec);
            dumpFlow(simpleBind.invoke(ConfigurationProtocols.SERVERBOUND_TEMPLATE, decorator), "config-sb", out, codec);
            dumpFlow(simpleBind.invoke(StatusProtocols.CLIENTBOUND_TEMPLATE, decorator), "status-cb", out, codec);
            dumpFlow(simpleBind.invoke(StatusProtocols.SERVERBOUND_TEMPLATE, decorator), "status-sb", out, codec);
            dumpFlow(simpleBind.invoke(HandshakeProtocols.SERVERBOUND_TEMPLATE, decorator), "handshake-sb", out, codec);
        }
    }

    /** One flow's dispatch codec -> "flow id minecraft:name" lines. */
    static void dumpFlow(Object dispatch, String flow, PrintWriter out, Method codec) throws Exception {
        for (Map.Entry<Object, Integer> e : sortedIds(dispatch, flow)) {
            out.println(flow + " " + e.getValue() + " " + identifier(e.getKey()));
        }
    }

    /** (type, id) pairs of one flow's dispatch codec, sorted by id. */
    static List<Map.Entry<Object, Integer>> sortedIds(Object dispatch, String flow) throws Exception {
        Map<Object, Integer> map = findIdMap(dispatch, 0);
        if (map == null) throw new IllegalStateException(flow + " id map not found");
        List<Map.Entry<Object, Integer>> entries = new ArrayList<>(map.entrySet());
        entries.sort(Comparator.comparingInt(Map.Entry::getValue));
        return entries;
    }

    /** The packet type's identifier ("minecraft:respawn") via the record accessor. */
    static String identifier(Object type) throws Exception {
        Method id = PacketType.class.getMethod("id");
        return id.invoke(type).toString();
    }

    /** Depth-limited object-graph search for the fastutil id map. */
    @SuppressWarnings("unchecked")
    static Map<Object, Integer> findIdMap(Object o, int depth) throws Exception {
        if (o == null || depth > 3) return null;
        if (o instanceof it.unimi.dsi.fastutil.objects.Object2IntMap) {
            return (Map<Object, Integer>) (Object) o;
        }
        if (o instanceof Map || o instanceof java.util.Collection || o instanceof Number
                || o instanceof String || o.getClass().isPrimitive()) {
            return null;
        }
        Class<?> c = o.getClass();
        while (c != null) {
            for (Field f : c.getDeclaredFields()) {
                if (java.lang.reflect.Modifier.isStatic(f.getModifiers())) continue;
                f.setAccessible(true);
                Object v;
                try { v = f.get(o); } catch (Exception e) { continue; }
                Map<Object, Integer> hit = findIdMap(v, depth + 1);
                if (hit != null) return hit;
            }
            c = c.getSuperclass();
        }
        return null;
    }
}
