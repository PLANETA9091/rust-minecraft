import java.lang.reflect.*;
public class Probe6 {
    public static void main(String[] a) throws Exception {
        Class.forName("net.minecraft.SharedConstants").getMethod("tryDetectVersion").invoke(null);
        Class<?> b = Class.forName("net.minecraft.server.Bootstrap");
        b.getMethod("bootStrap").invoke(null);
        // conduit SHELL_TEXTURE + golem statue texture + skull skins
        Class<?> cr = Class.forName("net.minecraft.client.renderer.blockentity.ConduitRenderer");
        for (Field f : cr.getDeclaredFields()) {
            f.setAccessible(true);
            if (Modifier.isStatic(f.getModifiers())) System.out.println("CONDUIT " + f.getName() + " = " + f.get(null));
        }
        Class<?> go = Class.forName("net.minecraft.client.renderer.special.CopperGolemStatueSpecialRenderer");
        for (Field f : go.getDeclaredFields()) {
            f.setAccessible(true);
            if (Modifier.isStatic(f.getModifiers())) System.out.println("GOLEM " + f.getName() + " = " + f.get(null));
        }
        Class<?> sk = Class.forName("net.minecraft.client.renderer.blockentity.SkullBlockRenderer");
        for (Field f : sk.getDeclaredFields()) {
            f.setAccessible(true);
            if (Modifier.isStatic(f.getModifiers()) && !f.getType().getName().endsWith("Map"))
                System.out.println("SKULL " + f.getName() + " = " + f.get(null));
        }
        // the skull MAP (type -> render type) values
        for (Field f : sk.getDeclaredFields()) {
            f.setAccessible(true);
            if (Modifier.isStatic(f.getModifiers()) && f.getType().getName().endsWith("Map")) {
                Object m = f.get(null);
                System.out.println("SKULL MAP " + f.getName());
                if (m instanceof java.util.Map) {
                    for (Object e : ((java.util.Map<?, ?>) m).entrySet())
                        System.out.println("   " + ((java.util.Map.Entry<?, ?>) e).getKey() + " -> "
                            + ((java.util.Map.Entry<?, ?>) e).getValue());
                }
            }
        }
    }
}
