using System;
using Dalamud.Interface.Windowing;
using Dalamud.Bindings.ImGui;

namespace Auracite;

public class StepWindow : Window, IDisposable
{
    public StepWindow() : base("Auracite", ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.NoSavedSettings)
    {
        this.ShowCloseButton = false;
    }

    public void Dispose()
    {

    }

    public override void Draw()
    {
        if (Plugin.CurrentStep != null)
        {
            ImGui.Text($"Step: {Plugin.CurrentStep.StepName()}");
            ImGui.Separator();
            ImGui.Text(Plugin.CurrentStep.StepDescription());

            ImGui.TextDisabled("This step requires manual user action.");

            if (ImGui.Button("Retry"))
            {
                Plugin.CurrentStep.Run();
            }
        }
        else
        {
            ImGui.Text("Auracite is not running.");
        }
    }
}
