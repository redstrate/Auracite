using Dalamud.Interface.Windowing;
using Dalamud.Bindings.ImGui;
using System.Diagnostics;

namespace Auracite;

public class StepWindow : Window
{
    private Plugin plugin;

    public StepWindow(Plugin plugin) : base("Auracite", ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.NoSavedSettings)
    {
        this.plugin = plugin;
        this.ShowCloseButton = false;
    }

    public override void Draw()
    {
        if (Plugin.CurrentStep != null)
        {
            if (Plugin.CurrentStep.IsEnd())
            {
                ImGui.TextWrapped("Archive created! Please download it below and keep it in a safe place.");
                ImGui.TextWrapped("The plugin can be disabled once you're done using it.");

                if (ImGui.Button("Download"))
                {
                    Process.Start(new ProcessStartInfo { FileName = "http://localhost:42073/download", UseShellExecute = true });
                }
                ImGui.SameLine();
                if (ImGui.Button("Close"))
                {
                    plugin.Stop();
                }
            }
            else
            {
                ImGui.TextWrapped($"Step: {Plugin.CurrentStep.StepName()}");
                ImGui.Separator();
                ImGui.TextWrapped(Plugin.CurrentStep.StepDescription());

                ImGui.TextDisabled("This step requires manual user action.");

                if (ImGui.Button("Try Again"))
                {
                    Plugin.CurrentStep.Run();
                }
            }
        }
        else
        {
            ImGui.TextWrapped("Auracite is not running.");
        }
    }
}
