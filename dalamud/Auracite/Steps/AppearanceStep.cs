using Dalamud.Game.ClientState.Objects.Enums;

namespace Auracite;

public class AppearanceStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (Plugin.ObjectTable.LocalPlayer != null)
        {
            Plugin.package.appearance.model_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.ModelType];
            Plugin.package.appearance.height = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Height];
            Plugin.package.appearance.face_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceType];
            Plugin.package.appearance.hair_style = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairStyle];
            Plugin.package.appearance.has_highlights = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HasHighlights] == 1;
            Plugin.package.appearance.skin_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.SkinColor];
            Plugin.package.appearance.eye_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeColor];
            Plugin.package.appearance.hair_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairColor];
            Plugin.package.appearance.hair_color2 = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairColor2];
            Plugin.package.appearance.face_features = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceFeatures];
            Plugin.package.appearance.face_features_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceFeaturesColor];
            Plugin.package.appearance.eyebrows = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Eyebrows];
            Plugin.package.appearance.eye_color2 = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeColor2];
            Plugin.package.appearance.eye_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeShape];
            Plugin.package.appearance.nose_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.NoseShape];
            Plugin.package.appearance.jaw_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.JawShape];
            Plugin.package.appearance.lip_style = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.LipStyle];
            Plugin.package.appearance.lip_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.LipColor];
            Plugin.package.appearance.race_feature_size = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.RaceFeatureSize];
            Plugin.package.appearance.race_feature_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.RaceFeatureType];
            Plugin.package.appearance.bust_size = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.BustSize];
            Plugin.package.appearance.facepaint = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Facepaint];
            Plugin.package.appearance.facepaint_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FacepaintColor];
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
