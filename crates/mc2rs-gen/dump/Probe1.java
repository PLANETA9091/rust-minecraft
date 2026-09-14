import java.lang.reflect.*;

public class Probe1 {
    public static void main(String[] a) throws Exception {
        Class.forName("net.minecraft.SharedConstants").getMethod("tryDetectVersion").invoke(null);
        Class<?> b = Class.forName("net.minecraft.server.Bootstrap");
        b.getMethod("bootStrap").invoke(null);
        Class<?> layers = Class.forName("net.minecraft.client.model.geom.LayerDefinitions");
        System.out.println("## LayerDefinitions methods:");
        for (Method m : layers.getDeclaredMethods()) {
            if (!m.getName().startsWith("lambda") && !m.getName().startsWith("access$"))
                System.out.println("  " + m);
        }
        // walk the mesh builder classes from the createRoots return type
        Method cr = null;
        for (Method m : layers.getDeclaredMethods())
            if (m.getName().equals("createRoots")) cr = m;
        System.out.println("## createRoots = " + cr);
        if (cr != null) {
            cr.setAccessible(true);
            Object roots = cr.invoke(null);
            System.out.println("## roots class " + roots.getClass() + " size "
                    + ((java.util.Map<?, ?>) roots).size());
            // print a couple of keys
            int i = 0;
            for (Object k : ((java.util.Map<?, ?>) roots).keySet()) {
                if (i++ > 6) break;
                System.out.println("  key " + k + " (" + k.getClass().getName() + ")");
            }
        }
        // discover the geometry classes from a chest layer's value
        for (Method m : layers.getDeclaredMethods()) {
            if (m.getName().equals("createRoots")) {
                m.setAccessible(true);
                java.util.Map<?, ?> roots = (java.util.Map<?, ?>) m.invoke(null);
                for (Object k : roots.keySet()) {
                    String ks = k.toString();
                    if (ks.contains("chest") || ks.contains("banner") || ks.contains("shulker")
                            || ks.contains("skull") || ks.contains("head") || ks.contains("conduit")
                            || ks.contains("decorated") || ks.contains("golem_statue")) {
                        System.out.println("## WANT " + ks);
                    }
                }
                break;
            }
        }
        // the builder classes actually used
        for (String cn : new String[]{
                "net.minecraft.client.model.geom.builders.MeshDefinition",
                "net.minecraft.client.model.geom.builders.PartDefinition",
                "net.minecraft.client.model.geom.builders.PartPose",
                "net.minecraft.client.model.geom.builders.CubeListBuilder",
                "net.minecraft.client.model.geom.builders.LayerDefinition",
                "net.minecraft.client.model.geom.builders.CubeDeformation",
                "net.minecraft.client.model.geom.ModelPart",
        }) {
            try {
                Class<?> c = Class.forName(cn);
                System.out.println("## OK " + cn);
                for (Method m : c.getDeclaredMethods())
                    if (!m.getName().startsWith("lambda"))
                        System.out.println("    m " + m);
                for (Field f : c.getDeclaredFields()) System.out.println("    f " + f);
            } catch (Throwable t) {
                System.out.println("## ABSENT " + cn + " (" + t + ")");
            }
        }
    }
}
