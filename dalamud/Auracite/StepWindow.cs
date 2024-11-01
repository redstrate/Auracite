using System;
using Dalamud.Interface.Windowing;
using ImGuiNET;

namespace Auracite;

public class StepWindow()
    : Window("Step Window"), IDisposable
{
    public void Dispose()
    {
    }

    public override void Draw()
    {
        if (Plugin.CurrentStep != null)
        {
            ImGui.Text(Plugin.CurrentStep.StepName());
            ImGui.Text(Plugin.CurrentStep.StepDescription());

            ImGui.TextDisabled("Step requires manual user action.");

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