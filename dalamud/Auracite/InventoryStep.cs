using FFXIVClientStructs.FFXIV.Client.Game;

namespace Auracite;

public class InventoryStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (Plugin.ObjectTable.LocalPlayer != null)
        {
            unsafe
            {
                var manager = InventoryManager.Instance();
                Plugin.package.inventory1 = ProcessContainer(manager->GetInventoryContainer(InventoryType.Inventory1));
                Plugin.package.inventory2 = ProcessContainer(manager->GetInventoryContainer(InventoryType.Inventory2));
                Plugin.package.inventory3 = ProcessContainer(manager->GetInventoryContainer(InventoryType.Inventory3));
                Plugin.package.inventory4 = ProcessContainer(manager->GetInventoryContainer(InventoryType.Inventory4));

                Plugin.package.equipped_items = ProcessContainer(manager->GetInventoryContainer(InventoryType.EquippedItems));

                Plugin.package.currency = ProcessContainer(manager->GetInventoryContainer(InventoryType.Currency));

                Plugin.package.armory_off_hand = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryOffHand));
                Plugin.package.armory_head = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryHead));
                Plugin.package.armory_body = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryBody));
                Plugin.package.armory_hands = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryHands));
                Plugin.package.armory_waist = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryWaist));
                Plugin.package.armory_legs = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryLegs));
                Plugin.package.armory_ear = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryEar));
                Plugin.package.armory_neck = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryNeck));
                Plugin.package.armory_wrist = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryWrist));
                Plugin.package.armory_rings = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryRings));
                Plugin.package.armory_soul_crystal = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmorySoulCrystal));
                Plugin.package.armory_main_hand = ProcessContainer(manager->GetInventoryContainer(InventoryType.ArmoryMainHand));
            }
        }
        Completed?.Invoke();
    }

    private unsafe Auracite.Plugin.InventoryContainer ProcessContainer(FFXIVClientStructs.FFXIV.Client.Game.InventoryContainer *container) {
        var serializedContainer = new Auracite.Plugin.InventoryContainer();
        serializedContainer.items = new System.Collections.Generic.List<Plugin.InventoryItem>(); // TODO: lol

        for (int i = 0; i < container->Size; i++) {
            var item = container->GetInventorySlot(i);
            if (item != null) {
                if (item->GetQuantity() == 0) {
                    continue;
                }

                var serializedItem = new Auracite.Plugin.InventoryItem();
                serializedItem.condition = item->GetCondition();
                serializedItem.id = item->GetBaseItemId();
                serializedItem.quantity = item->GetQuantity();
                serializedItem.slot = item->GetSlot();
                serializedItem.glamour_id = item->GetGlamourId();

                serializedContainer.items.Add(serializedItem);
            }
        }

        return serializedContainer;
    }

    public string StepName()
    {
        return "Currency";
    }

    public string StepDescription()
    {
        return "No user action required.";
    }

    public void Dispose()
    {
    }
}
