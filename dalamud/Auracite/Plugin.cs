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
        [typeof(AppearanceStep), typeof(InventoryStep), typeof(MiscStep), typeof(PlaytimeStep), typeof(AdventurerPlateStep), typeof(EndStep)];

    private int _stepIndex;

    private readonly StepWindow StepWindow;

    [SuppressMessage("ReSharper", "InconsistentNaming")]
    public class InventoryItem
    {
        public int slot;
        public uint quantity;
        public int condition;
        public uint id;
        public uint glamour_id;
    }

    [SuppressMessage("ReSharper", "InconsistentNaming")]
    public class InventoryContainer
    {
        public List<InventoryItem> items;
    }

    [SuppressMessage("ReSharper", "InconsistentNaming")]
    public class Package
    {
        public string? playtime;
        public bool is_battle_mentor;
        public bool is_trade_mentor;
        public bool is_novice;
        public bool is_returner;
        public short player_commendations;
        
        // Appearance
        public int race;
        public int gender;
        public int model_type;
        public int height;
        public int tribe;
        public int face_type;
        public int hair_style;
        public bool has_highlights;
        public int skin_color;
        public int eye_color;
        public int hair_color;
        public int hair_color2;
        public int face_features;
        public int face_features_color;
        public int eyebrows;
        public int eye_color2;
        public int eye_shape;
        public int nose_shape;
        public int jaw_shape;
        public int lip_style;
        public int lip_color;
        public int race_feature_size;
        public int race_feature_type;
        public int bust_size;
        public int facepaint;
        public int facepaint_color;
        public string? portrait;
        public string? plate_title;
        public bool? plate_title_is_prefix;
        public string? plate_class_job;
        public int plate_class_job_level;
        public string? search_comment;
        public string? base_plate;
        public string? pattern_overlay;
        public string? backing;
        public string? top_border;
        public string? bottom_border;
        public string? portrait_frame;
        public string? plate_frame;
        public string? accent;

        // inventory
        public InventoryContainer inventory1;
        public InventoryContainer inventory2;
        public InventoryContainer inventory3;
        public InventoryContainer inventory4;

        public InventoryContainer equipped_items;

        public InventoryContainer currency;

        public InventoryContainer armory_off_hand;
        public InventoryContainer armory_head;
        public InventoryContainer armory_body;
        public InventoryContainer armory_hands;
        public InventoryContainer armory_waist;
        public InventoryContainer armory_legs;
        public InventoryContainer armory_ear;
        public InventoryContainer armory_neck;
        public InventoryContainer armory_wrist;
        public InventoryContainer armory_rings;
        public InventoryContainer armory_soul_crystal;
        public InventoryContainer armory_main_hand;

        public int voice;
        public List<byte> unlock_flags;
        public List<byte> unlock_aetherytes;
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

    [PluginService] internal static IObjectTable ObjectTable { get; private set; } = null!;

    [PluginService] internal static IDalamudPluginInterface PluginInterface { get; private set; } = null!;

    [PluginService] internal static IChatGui ChatGui { get; private set; } = null!;

    [PluginService] internal static ICommandManager CommandManager { get; private set;  } = null!;
    
    [PluginService] internal static IDataManager DataManager { get; private set;  } = null!;

    public void Dispose()
    {
        CurrentStep?.Dispose();
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
            CurrentStep?.Dispose();
            CurrentStep = null;
            StepWindow.IsOpen = false;
            return;
        }
        CurrentStep = (IStep)Activator.CreateInstance(_steps[_stepIndex])!;
        CurrentStep.Completed += NextStep;
        CurrentStep.Run();
    }
}
