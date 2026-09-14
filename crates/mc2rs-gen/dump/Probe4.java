import java.lang.reflect.*;
public class Probe4 {
    public static void main(String[] a) throws Exception {
        for (String cn : new String[]{
                "net.minecraft.client.model.geom.ModelPart$Polygon",
                "net.minecraft.client.model.geom.ModelPart$Vertex",
                "net.minecraft.client.model.geom.ModelPart$Cube",
        }) {
            Class<?> c = Class.forName(cn);
            System.out.println("## " + cn);
            for (Field f : c.getDeclaredFields()) System.out.println("  f " + f);
        }
    }
}
