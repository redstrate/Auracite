using FFXIVClientStructs.FFXIV.Client.Game;

namespace Auracite;

public class CurrencyStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (Plugin.ClientState.LocalPlayer != null)
        {
            unsafe
            {
                Plugin.package.gil = InventoryManager.Instance()->GetGil();
            }
        }
        Completed?.Invoke();
    }

    public string StepName()
    {
        return "Currency";
    }

    public string StepDescription()
    {
        return "No user action required.";
    }
}