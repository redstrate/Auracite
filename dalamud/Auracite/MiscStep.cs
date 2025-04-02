using FFXIVClientStructs.FFXIV.Client.Game;
using FFXIVClientStructs.FFXIV.Client.Game.UI;
using FFXIVClientStructs.FFXIV.Client.Game.Character;

namespace Auracite;

public class MiscStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        unsafe
        {
            Plugin.package.is_battle_mentor = PlayerState.Instance()->IsBattleMentor();
            Plugin.package.is_trade_mentor = PlayerState.Instance()->IsTradeMentor();
            Plugin.package.is_novice = PlayerState.Instance()->IsNovice();
            Plugin.package.is_returner = PlayerState.Instance()->IsReturner();
            Plugin.package.player_commendations = PlayerState.Instance()->PlayerCommendations;

            var localPlayer = Plugin.ClientState.LocalPlayer;
            if (localPlayer != null)
            {
                var gameObject = (Character*)localPlayer.Address;
                Plugin.package.voice = gameObject->Vfx.VoiceId;
            }
        }

        Completed?.Invoke();
    }

    public string StepName()
    {
        return "Misc Data";
    }

    public string StepDescription()
    {
        return "No user action required.";
    }

    public void Dispose()
    {
    }
}
