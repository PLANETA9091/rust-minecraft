import java.lang.reflect.*;

public class Probe5 {
    public static void main(String[] a) throws Exception {
        Class.forName("net.minecraft.SharedConstants").getMethod("tryDetectVersion").invoke(null);
        Class<?> b = Class.forName("net.minecraft.server.Bootstrap");
        b.getMethod("bootStrap").invoke(null);
        // static field VALUES on the renderer classes + Sheets + TridentModel
        String[] classes = {
            "net.minecraft.client.model.object.projectile.TridentModel",
            "net.minecraft.client.renderer.Sheets",
            "net.minecraft.client.renderer.special.ChestSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.SkullSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.CopperGolemStatueSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.ShulkerBoxSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.BannerSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.ConduitSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.DecoratedPotSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.ShieldSpecialRenderer$Unbaked",
            "net.minecraft.client.renderer.special.TridentSpecialRenderer$Unbaked",
        };
        for (String cn : classes) {
            Class<?> c;
            try { c = Class.forName(cn); }
            catch (Throwable t) { System.out.println("== ABSENT " + cn); continue; }
            System.out.println("== " + cn);
            for (Field f : c.getDeclaredFields()) {
                f.setAccessible(true);
                if (!Modifier.isStatic(f.getModifiers())) continue;
                String ft = f.getType().getName();
                if (ft.endsWith("ResourceLocation") || ft.endsWith("Identifier")
                        || ft.endsWith("SpriteId") || ft.endsWith("ModelLayerLocation")
                        || ft.endsWith("String") || ft.endsWith("Material")
                        || ft.endsWith("ModelLayerLocation")) {
                    try { System.out.println("  " + f.getName() + " = " + f.get(null)); }
                    catch (Throwable t) { System.out.println("  " + f.getName() + " <?>"); }
                }
            }
        }
    }
}
