using Dalamud.Game.ClientState.Objects.Enums;

namespace Auracite;

public class AppearanceStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (Plugin.ObjectTable.LocalPlayer != null)
        {
            Plugin.package.race = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Race];
            Plugin.package.gender = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Gender];
            Plugin.package.model_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.ModelType];
            Plugin.package.height = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Height];
            Plugin.package.tribe = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Tribe];
            Plugin.package.face_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceType];
            Plugin.package.hair_style = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairStyle];
            Plugin.package.has_highlights = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HasHighlights] == 1;
            Plugin.package.skin_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.SkinColor];
            Plugin.package.eye_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeColor];
            Plugin.package.hair_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairColor];
            Plugin.package.hair_color2 = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.HairColor2];
            Plugin.package.face_features = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceFeatures];
            Plugin.package.face_features_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FaceFeaturesColor];
            Plugin.package.eyebrows = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Eyebrows];
            Plugin.package.eye_color2 = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeColor2];
            Plugin.package.eye_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.EyeShape];
            Plugin.package.nose_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.NoseShape];
            Plugin.package.jaw_shape = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.JawShape];
            Plugin.package.lip_style = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.LipStyle];
            Plugin.package.lip_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.LipColor];
            Plugin.package.race_feature_size = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.RaceFeatureSize];
            Plugin.package.race_feature_type = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.RaceFeatureType];
            Plugin.package.bust_size = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.BustSize];
            Plugin.package.facepaint = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.Facepaint];
            Plugin.package.facepaint_color = Plugin.ObjectTable.LocalPlayer.Customize[(int)CustomizeIndex.FacepaintColor];
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
