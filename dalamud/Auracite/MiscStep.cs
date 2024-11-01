using FFXIVClientStructs.FFXIV.Client.Game;
using FFXIVClientStructs.FFXIV.Client.Game.UI;

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