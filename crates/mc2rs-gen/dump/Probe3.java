import java.lang.reflect.*;

public class Probe3 {
    public static void main(String[] a) throws Exception {
        for (String cn : new String[]{
                "net.minecraft.client.model.geom.ModelPart",
                "net.minecraft.client.model.geom.builders.MaterialDefinition",
                "net.minecraft.client.model.geom.ModelPart$Cube",
        }) {
            Class<?> c = Class.forName(cn);
            System.out.println("## " + cn);
            for (Field f : c.getDeclaredFields()) System.out.println("  f " + f);
            for (Method m : c.getDeclaredMethods()) {
                if (m.getName().startsWith("lambda") || (m.getModifiers() & Modifier.STATIC) != 0)
                    continue;
                if (m.getParameterCount() == 0) System.out.println("  m " + m);
            }
        }
    }
}
