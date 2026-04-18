using System.Collections.Generic;
using FFXIVClientStructs.FFXIV.Client.Game.Character;
using FFXIVClientStructs.FFXIV.Client.Game.UI;
using Lumina.Excel.Sheets;

namespace Auracite;

public class TitleStep : IStep
{
    public TitleStep()
    {
    }

    public void Dispose()
    {
    }

    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        unsafe {
            var uiState = UIState.Instance();
            var playerState = PlayerState.Instance();
            if (uiState->TitleList.DataReceived)
            {
                Plugin.package!.titles = new List<byte>(uiState->TitleList.TitlesUnlockBitmask.ToArray());
                Plugin.package!.title = IStep.SaveNameValue<Title>(((Character*)Plugin.ObjectTable.LocalPlayer!.Address)->TitleId, title => playerState->Sex == 1 ? title.Feminine : title.Masculine);

                Completed?.Invoke();
            }
        }
    }

    public string StepName()
    {
        return "Titles";
    }

    public string StepDescription()
    {
        return "Open the title list.";
    }

    public bool NeedsUpdateEveryFrame()
    {
        return true; // So we can wait for the title window to open.
    }
}
