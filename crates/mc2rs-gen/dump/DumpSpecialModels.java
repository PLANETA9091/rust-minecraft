import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;

/**
 * Clean-room geometry dumper for the special item renderers (the 63
 * block-entity / entity-model items the item-model trees refuse):
 * shield, trident, chest, shulker_box, banner, heads, conduit,
 * decorated_pot, copper_golem_statue. In 26.2 EVERY one of these is a
 * LayerDefinitions root (shield#main and trident#main included) — the
 * running client's own static mesh registry is the source of truth;
 * nothing is remembered here.
 *
 * For each wanted ModelLayerLocation the dump prints:
 *   - the resolved ModelPart tree (LayerDefinition.bakeRoot()): every
 *     part's pose (x/y/z, xRot/yRot/zRot radians) and visibility,
 *   - every cube's constants (origin, dimensions, grow, mirror) AND
 *     its RESOLVED polygons: per face, the four vertices (positions +
 *     per-vertex UVs) — the client's own bake, float hex bits exact.
 *   - DyeColor: every constant's numeric fields (the banner and shulker
 *     tints' source colors), so the tint law is data, not lore.
 *   - the item special renderer's Material/ResourceLocation constants
 *     (the texture NAMES the pixels ride).
 *
 * Everything rides pure reflection so this file compiles with NO
 * classpath at all (source-file mode); the client jar + libraries
 * join at RUNTIME only.
 *
 * Run:
 *   java -cp <client.jar:libs/*> DumpSpecialModels.java <out-file>
 */
public class DumpSpecialModels {
    static java.io.PrintStream out;

    public static void main(String[] args) throws Exception {
        out = new java.io.PrintStream(new java.io.FileOutputStream(args[0]));
        Class.forName("net.minecraft.SharedConstants")
                .getMethod("tryDetectVersion").invoke(null);
        Class<?> bootstrap = Class.forName("net.minecraft.server.Bootstrap");
        bootstrap.getMethod("bootStrap").invoke(null);
        out.println("boot ok");

        laneLayers();
        laneDyeColors();
        laneTextureConstants();
        out.println("done");
    }

    static Object fieldOf(Object o, String name) throws Exception {
        for (Class<?> c = o.getClass(); c != null; c = c.getSuperclass()) {
            try {
                Field f = c.getDeclaredField(name);
                f.setAccessible(true);
                return f.get(o);
            } catch (NoSuchFieldException e) { /* climb */ }
        }
        throw new NoSuchFieldException(name);
    }

    static String fx(float v) {
        return Float.toHexString(v) + "/" + v;
    }

    static String vec(Object v) throws Exception {
        if (v == null) return "null";
        Class<?> c = v.getClass();
        Method x = null, y = null, z = null;
        for (Method m : c.getMethods()) {
            if (m.getParameterCount() != 0) continue;
            if (m.getName().equals("x")) x = m;
            if (m.getName().equals("y")) y = m;
            if (m.getName().equals("z")) z = m;
        }
        return "(" + fx((Float) x.invoke(v)) + " " + fx((Float) y.invoke(v))
                + " " + fx((Float) z.invoke(v)) + ")";
    }

    static String uvpair(Object v) throws Exception {
        if (v == null) return "null";
        Object u = fieldOf(v, "u");
        Object vv = fieldOf(v, "v");
        return "(" + u + " " + vv + ")";
    }

    static void laneLayers() throws Exception {
        Class<?> layers = Class.forName("net.minecraft.client.model.geom.LayerDefinitions");
        Method cr = null;
        for (Method m : layers.getDeclaredMethods())
            if (m.getName().equals("createRoots")) cr = m;
        cr.setAccessible(true);
        java.util.Map<?, ?> roots = (java.util.Map<?, ?>) cr.invoke(null);

        String[] want = {
            "minecraft:shield#main", "minecraft:trident#main",
            "minecraft:chest#main", "minecraft:shulker_box#main",
            "minecraft:standing_banner#main", "minecraft:standing_banner#flag",
            "minecraft:creeper_head#main", "minecraft:skeleton_skull#main",
            "minecraft:wither_skeleton_skull#main", "minecraft:zombie_head#main",
            "minecraft:player_head#main", "minecraft:piglin_head#main",
            "minecraft:dragon_skull#main",
            "minecraft:conduit#eye", "minecraft:conduit#wind",
            "minecraft:conduit#shell", "minecraft:conduit#cage",
            "minecraft:decorated_pot_base#main", "minecraft:decorated_pot_sides#main",
            "minecraft:copper_golem#main", "minecraft:copper_golem_running#main",
            "minecraft:copper_golem_sitting#main", "minecraft:copper_golem_star#main",
        };
        Class<?> layerDef = Class.forName("net.minecraft.client.model.geom.builders.LayerDefinition");
        Method bakeRoot = layerDef.getMethod("bakeRoot");

        for (String key : want) {
            Object ld = null;
            for (java.util.Map.Entry<?, ?> e : roots.entrySet()) {
                if (String.valueOf(e.getKey()).equals(key)) { ld = e.getValue(); break; }
            }
            if (ld == null) { out.println("=== LAYER " + key + " ABSENT"); continue; }
            out.println("=== LAYER " + key);
            // the tex dims ride the LayerDefinition's MaterialDefinition
            Object material = fieldOf(ld, "material");
            try {
                Object texW = fieldOf(material, "xTexSize");
                Object texH = fieldOf(material, "yTexSize");
                out.println("TEX " + texW + " " + texH);
            } catch (Exception e) { out.println("TEX ? ?"); }
            Object root = bakeRoot.invoke(ld);
            dumpPart(root, "  ", "?");
        }
    }

    static void dumpPart(Object part, String ind, String name) throws Exception {
        out.println(ind + "PART " + name);
        StringBuilder sb = new StringBuilder(ind + "  POSE");
        for (String fn : new String[]{"x", "y", "z", "xRot", "yRot", "zRot",
                "xScale", "yScale", "zScale"}) {
            try {
                Field f = part.getClass().getDeclaredField(fn);
                f.setAccessible(true);
                sb.append(" " + fn + "=" + fx(f.getFloat(part)));
            } catch (Exception e) { /* absent */ }
        }
        try {
            Field f = part.getClass().getDeclaredField("visible");
            f.setAccessible(true);
            sb.append(" visible=" + f.get(part));
        } catch (Exception e) { /* absent */ }
        out.println(sb);

        Object cubes = fieldOf(part, "cubes");
        if (cubes instanceof java.util.List) {
            for (Object cube : (java.util.List<?>) cubes) dumpCube(cube, ind + "  ");
        }
        Object children = fieldOf(part, "children");
        if (children instanceof java.util.Map) {
            for (Object e : ((java.util.Map<?, ?>) children).entrySet()) {
                dumpPart(((java.util.Map.Entry<?, ?>) e).getValue(),
                        ind + "  ", String.valueOf(((java.util.Map.Entry<?, ?>) e).getKey()));
            }
        }
    }

    static void dumpCube(Object cube, String ind) throws Exception {
        StringBuilder sb = new StringBuilder(ind + "CUBE");
        for (String fn : new String[]{"minX", "minY", "minZ", "maxX", "maxY", "maxZ"}) {
            try {
                Field f = cube.getClass().getDeclaredField(fn);
                f.setAccessible(true);
                sb.append(" " + fn + "=" + fx(f.getFloat(cube)));
            } catch (Exception e) { sb.append(" " + fn + "=?"); }
        }
        try {
            Object grow = fieldOf(cube, "grow");
            sb.append(" grow=(" + fieldOf(grow, "growX") + " " + fieldOf(grow, "growY")
                    + " " + fieldOf(grow, "growZ") + ")");
        } catch (Exception e) { sb.append(" grow=?"); }
        try { sb.append(" mirror=" + fieldOf(cube, "mirror")); }
        catch (Exception e) { /* absent */ }
        out.println(sb);
        // the resolved polygons — the 26.2 Vertex carries x/y/z + u/v itself
        Class<?> polyClass = Class.forName("net.minecraft.client.model.geom.ModelPart$Polygon");
        Object polys = fieldOf(cube, "polygons");
        for (Object poly : (Object[]) polys) {
            StringBuilder ps = new StringBuilder(ind + "  FACE verts=");
            Object verts = fieldOf(poly, "vertices");
            for (Object vert : (Object[]) verts) {
                ps.append("[xyz=" + fx((Float) fieldOf(vert, "x"))
                        + " " + fx((Float) fieldOf(vert, "y"))
                        + " " + fx((Float) fieldOf(vert, "z"))
                        + " uv=" + fx((Float) fieldOf(vert, "u"))
                        + " " + fx((Float) fieldOf(vert, "v")) + "]");
            }
            out.println(ps);
        }
    }

    static void laneDyeColors() throws Exception {
        Class<?> c = Class.forName("net.minecraft.world.item.DyeColor");
        out.println("=== DYECOLOR");
        Method[] ms = c.getMethods();
        for (Object e : c.getEnumConstants()) {
            StringBuilder sb = new StringBuilder("  " + e);
            for (Field f : c.getDeclaredFields()) {
                f.setAccessible(true);
                if (Modifier.isStatic(f.getModifiers())) continue;
                String ft = f.getType().getName();
                try {
                    Object v = f.get(e);
                    if (v instanceof Integer || v instanceof Float || v instanceof int[]
                            || v instanceof float[] || (v != null
                                && (v.getClass().getName().endsWith("Vec3")
                                    || v.getClass().getName().endsWith("Vector3f")))) {
                        if (v instanceof Vec3Like)
                            sb.append(" " + f.getName() + "=vec");
                        else
                            sb.append(" " + f.getName() + "=" + describe(v));
                    }
                } catch (Exception x) { /* ignore */ }
            }
            // the accessor methods too (the mapped names may be computed)
            for (Method m : ms) {
                if (m.getParameterCount() != 0) continue;
                String mn = m.getName();
                if (mn.equals("getTextureDiffuseColor") || mn.equals("getTextureDiffuseColors")
                        || mn.equals("getTextColor") || mn.equals("getMapColor")
                        || mn.equals("getSignColor") || mn.equals("getFireworkColor")
                        || mn.equals("getName")) {
                    try { sb.append(" " + mn + "=" + describe(m.invoke(e))); }
                    catch (Exception x) { /* ignore */ }
                }
            }
            out.println(sb);
        }
    }

    static String describe(Object v) {
        if (v instanceof int[]) {
            StringBuilder sb = new StringBuilder("[");
            for (int x : (int[]) v) sb.append(x + ",");
            return sb + "]";
        }
        if (v instanceof float[]) {
            StringBuilder sb = new StringBuilder("[");
            for (float x : (float[]) v) sb.append(x + ",");
            return sb + "]";
        }
        if (v instanceof Float) return fx((Float) v);
        return String.valueOf(v);
    }

    interface Vec3Like {}

    static void laneTextureConstants() throws Exception {
        String[] classes = {
            "net.minecraft.client.renderer.blockentity.BannerRenderer",
            "net.minecraft.client.renderer.blockentity.ChestRenderer",
            "net.minecraft.client.renderer.blockentity.ConduitRenderer",
            "net.minecraft.client.renderer.blockentity.DecoratedPotRenderer",
            "net.minecraft.client.renderer.blockentity.SkullBlockRenderer",
            "net.minecraft.client.renderer.BlockEntityWithoutLevelRenderer",
            "net.minecraft.client.renderer.special.ShieldSpecialRenderer",
            "net.minecraft.client.renderer.special.TridentSpecialRenderer",
            "net.minecraft.client.renderer.special.SkullSpecialRenderer",
            "net.minecraft.client.renderer.special.BedSpecialRenderer",
        };
        for (String cn : classes) {
            Class<?> c;
            try { c = Class.forName(cn); }
            catch (Exception e) { out.println("=== TEXCLASS " + cn + " ABSENT"); continue; }
            out.println("=== TEXCLASS " + cn);
            for (Class<?> cc = c; cc != null; cc = cc.getSuperclass()) {
                for (Field f : cc.getDeclaredFields()) {
                    f.setAccessible(true);
                    String ft = f.getType().getName();
                    if (Modifier.isStatic(f.getModifiers())
                            && (ft.endsWith("ResourceLocation") || ft.endsWith("Material"))) {
                        try { out.println("  " + f.getName() + " = " + f.get(null)); }
                        catch (Exception e) { /* never for statics */ }
                    }
                }
            }
        }
        // the texture constants the shield/trident ITEM specials ride —
        // hunt the special-renderer registry classes
        for (String cn : new String[]{
                "net.minecraft.client.renderer.special.SpecialBlockModelRenderer",
                "net.minecraft.client.renderer.special.SpecialModelRenderers",
                "net.minecraft.client.renderer.special.SpecialModelTypes",
        }) {
            Class<?> c;
            try { c = Class.forName(cn); }
            catch (Exception e) { out.println("=== TEXCLASS " + cn + " ABSENT"); continue; }
            out.println("=== TEXCLASS " + cn);
            for (Field f : c.getDeclaredFields()) {
                f.setAccessible(true);
                if (Modifier.isStatic(f.getModifiers())) {
                    out.println("  " + f.getName() + " : " + f.getType().getName());
                }
            }
        }
    }
}
