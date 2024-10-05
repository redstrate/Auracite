using System;
using System.Collections.Generic;
using System.Net.Http;
using Dalamud.Game.Command;
using Dalamud.Interface.Windowing;
using Dalamud.IoC;
using Dalamud.Plugin;
using Dalamud.Plugin.Services;
using Newtonsoft.Json;

namespace Auracite;

public sealed class Plugin : IDalamudPlugin
{
    public static IStep? CurrentStep;
    private readonly WindowSystem WindowSystem = new("Auracite");

    private readonly List<Type> _steps =
        [typeof(AppearanceStep), typeof(CurrencyStep), typeof(PlaytimeStep)];

    private int _stepIndex;

    private readonly StepWindow StepWindow;

    public class Package
    {
        public string playtime;
        public int height;
        public int bust_size;
        public uint gil;
    }

    public static Package? package;

    public Plugin()
    {
        CommandManager.AddHandler("/auracite", new CommandInfo(OnAuraciteCommand)
        {
            HelpMessage = "Start the server."
        });

        StepWindow = new StepWindow();
        WindowSystem.AddWindow(StepWindow);

        PluginInterface.UiBuilder.Draw += WindowSystem.Draw;
    }

    [PluginService] internal static IClientState ClientState { get; private set; } = null!;

    [PluginService] internal static IDalamudPluginInterface PluginInterface { get; private set; } = null!;

    [PluginService] internal static IChatGui ChatGui { get; private set; } = null!;

    [PluginService] internal static ICommandManager CommandManager { get; private set;  } = null!;

    public void Dispose()
    {
        WindowSystem.RemoveAllWindows();
    }

    private void OnAuraciteCommand(string command, string arguments)
    {
        if (arguments == "begin" && CurrentStep == null)
        {
            _stepIndex = -1;
            package = new Package();
            NextStep();
            StepWindow.IsOpen = true;
        }
    }

    private void NextStep()
    {
        _stepIndex++; 
        if (_stepIndex >= _steps.Count)
        {
            CurrentStep = null;
            StepWindow.IsOpen = false;
            SendPackage();
            return;
        }
        CurrentStep = (IStep)Activator.CreateInstance(_steps[_stepIndex])!;
        CurrentStep.Completed += NextStep;
        CurrentStep.Run();
    }

    private void SendPackage()
    {
        var client = new HttpClient();
        client.PostAsync("http://127.0.0.1:8000/package", new StringContent(JsonConvert.SerializeObject(package)));
        package = null;
    }
}