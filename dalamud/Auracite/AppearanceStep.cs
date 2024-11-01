using Dalamud.Game.ClientState.Objects.Enums;

namespace Auracite;

public class AppearanceStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (Plugin.ClientState.LocalPlayer != null)
        {
            Plugin.package.height = Plugin.ClientState.LocalPlayer.Customize[(int)CustomizeIndex.Height];
            Plugin.package.bust_size = Plugin.ClientState.LocalPlayer.Customize[(int)CustomizeIndex.BustSize];
        }
        Completed?.Invoke();
    }

    public string StepName()
    {
        return "Appearance";
    }

    public string StepDescription()
    {
        return "No user action required.";
    }

    public void Dispose()
    {
    }
}