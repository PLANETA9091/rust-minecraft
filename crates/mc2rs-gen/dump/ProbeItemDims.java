import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.EntityDimensions;

/**
 * mc2rs probe — the ITEM entity's own dimensions the drop lane's laws
 * speak (the popResource spawn lift = getHeight()/2, the physics box =
 * the width/height pair). Clean-room runtime query: boots the real
 * registries and reads the EntityType's own getters. One line:
 *
 *   width|height
 */
public class ProbeItemDims {
    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        Bootstrap.validate();
        EntityDimensions d = EntityTypes.ITEM.getDimensions();
        System.out.println(d.width() + "|" + d.height());
    }
}
