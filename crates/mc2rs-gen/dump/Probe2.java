import java.lang.reflect.*;

public class Probe2 {
    public static void main(String[] a) throws Exception {
        Class.forName("net.minecraft.SharedConstants").getMethod("tryDetectVersion").invoke(null);
        Class<?> b = Class.forName("net.minecraft.server.Bootstrap");
        b.getMethod("bootStrap").invoke(null);
        Class<?> layers = Class.forName("net.minecraft.client.model.geom.LayerDefinitions");
        Method cr = null;
        for (Method m : layers.getDeclaredMethods()) if (m.getName().equals("createRoots")) cr = m;
        cr.setAccessible(true);
        java.util.Map<?, ?> roots = (java.util.Map<?, ?>) cr.invoke(null);
        // hunt the copper golem statue + shield/trident-ish layers
        for (Object k : roots.keySet()) {
            String ks = k.toString();
            if (ks.contains("statue") || ks.contains("golem") || ks.contains("shield")
                    || ks.contains("trident") || ks.contains("pot") || ks.contains("skull")
                    || ks.contains("dragon") || ks.contains("conduit")) {
                System.out.println("KEY " + ks);
            }
        }
        // shield/trident classes: ctor surface
        for (String cn : new String[]{
                "net.minecraft.client.model.ShieldModel",
                "net.minecraft.client.model.TridentModel",
                "net.minecraft.client.model.dragon.DragonHeadModel",
        }) {
            try {
                Class<?> c = Class.forName(cn);
                System.out.println("CLASS " + cn);
                for (Constructor<?> k : c.getDeclaredConstructors()) System.out.println("  " + k);
                for (Field f : c.getDeclaredFields()) System.out.println("  f " + f);
            } catch (Throwable t) { System.out.println("ABSENT " + cn); }
        }
        // a Cube's toString + fields
        Class<?> clb = Class.forName("net.minecraft.client.model.geom.builders.CubeListBuilder");
        Object builder = clb.getMethod("create").invoke(null);
        clb.getMethod("texOffs", int.class, int.class).invoke(builder, 0, 16);
        clb.getMethod("addBox", float.class, float.class, float.class,
                float.class, float.class, float.class)
                .invoke(builder, -4f, -4f, -4f, 8f, 8f, 8f);
        Object cubes = clb.getMethod("getCubes").invoke(builder);
        Object box = ((java.util.List<?>) cubes).get(0);
        System.out.println("BOX class " + box.getClass().getName() + " toString " + box);
        for (Field f : box.getClass().getDeclaredFields()) System.out.println("  f " + f);
    }
}
