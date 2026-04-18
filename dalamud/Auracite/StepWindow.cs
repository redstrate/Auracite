using System;
using Dalamud.Interface.Windowing;
using Dalamud.Bindings.ImGui;
using System.Diagnostics;

namespace Auracite;

public class StepWindow : Window, IDisposable
{
    private Plugin plugin;

    public StepWindow(Plugin plugin) : base("Auracite", ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.NoSavedSettings)
    {
        this.plugin = plugin;
        this.ShowCloseButton = false;
    }

    public void Dispose()
    {

    }

    public override void Draw()
    {
        if (Plugin.CurrentStep != null)
        {
            if (Plugin.CurrentStep.IsEnd()) {
                ImGui.Text("Archive created! Please download it below and keep it in a safe place.");
                ImGui.Text("The plugin can be disabled once you're done using it.");

                if (ImGui.Button("Download"))
                {
                    Process.Start(new ProcessStartInfo { FileName = "http://localhost:42072/download", UseShellExecute = true });
                }
                ImGui.SameLine();
                if (ImGui.Button("Close"))
                {
                    plugin.Stop();
                }
            } else {
                ImGui.Text($"Step: {Plugin.CurrentStep.StepName()}");
                ImGui.Separator();
                ImGui.Text(Plugin.CurrentStep.StepDescription());

                ImGui.TextDisabled("This step requires manual user action.");
            }
        }
        else
        {
            ImGui.Text("Auracite is not running.");
        }
    }
}
