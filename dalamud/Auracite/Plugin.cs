using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using Dalamud.Game.Command;
using Dalamud.Interface.Windowing;
using Dalamud.IoC;
using Dalamud.Plugin;
using Dalamud.Plugin.Services;

namespace Auracite;

public sealed class Plugin : IDalamudPlugin
{
    public static IStep? CurrentStep;
    private readonly WindowSystem WindowSystem = new("Auracite");

    private readonly List<Type> _steps =
        [typeof(AppearanceStep), typeof(InventoryStep), typeof(MiscStep), typeof(PlaytimeStep), typeof(AdventurerPlateStep), typeof(TitleStep), typeof(EndStep)];

    private int _stepIndex;

    private readonly StepWindow StepWindow;

    public static CharacterJson? package;

    public Plugin()
    {
        CommandManager.AddHandler("/auracite", new CommandInfo(OnAuraciteCommand)
        {
            HelpMessage = "Start the archival process."
        });

        StepWindow = new StepWindow(this);
        WindowSystem.AddWindow(StepWindow);

        PluginInterface.UiBuilder.Draw += WindowSystem.Draw;
        Framework.Update += CheckCurrentStep;
    }

    [PluginService] internal static IClientState ClientState { get; private set; } = null!;

    [PluginService] internal static IObjectTable ObjectTable { get; private set; } = null!;

    [PluginService] internal static IDalamudPluginInterface PluginInterface { get; private set; } = null!;

    [PluginService] internal static IChatGui ChatGui { get; private set; } = null!;

    [PluginService] internal static ICommandManager CommandManager { get; private set;  } = null!;
    
    [PluginService] internal static IDataManager DataManager { get; private set;  } = null!;

    [PluginService] internal static IFramework Framework { get; private set;  } = null!;

    public void Dispose()
    {
        CurrentStep?.Dispose();
        WindowSystem.RemoveAllWindows();
    }

    private void OnAuraciteCommand(string command, string arguments)
    {
        if (CurrentStep == null)
        {
            _stepIndex = -1;
            package = new CharacterJson();
            NextStep();
            StepWindow.IsOpen = true;
        }
    }

    private void NextStep()
    {
        _stepIndex++; 
        if (_stepIndex >= _steps.Count)
        {
            CurrentStep?.Dispose();
            CurrentStep = null;
            StepWindow.IsOpen = false;
            return;
        }
        CurrentStep = (IStep)Activator.CreateInstance(_steps[_stepIndex])!;
        CurrentStep.Completed += NextStep;
        CurrentStep.Run();
    }

    public void Stop()
    {
        CurrentStep = null;
        StepWindow.IsOpen = false;
        package = null;
    }

    private void CheckCurrentStep(IFramework framework)
    {
        if (CurrentStep != null && CurrentStep.NeedsUpdateEveryFrame()) {
            CurrentStep.Run();
        }
    }
}
