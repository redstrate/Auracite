using System;
using Dalamud.Game.Text;
using Dalamud.Game.Text.SeStringHandling;

namespace Auracite;

public class PlaytimeStep : IStep, IDisposable
{
    public PlaytimeStep()
    {
        Plugin.ChatGui.ChatMessage += OnChatMessage;
    }

    public void Dispose()
    {
        Plugin.ChatGui.ChatMessage -= OnChatMessage;
    }

    public event IStep.CompletedDelegate? Completed;

    public string StepName()
    {
        return "Playtime";
    }

    public string StepDescription()
    {
        return "Type /playtime into the chat window.";
    }
    
    private void OnChatMessage(XivChatType type, int timestamp, ref SeString sender, ref SeString message,
        ref bool ishandled)
    {
        var msgString = message.ToString();
        if (msgString.Contains("Total Play Time:") && type == XivChatType.SystemMessage)
        {
            Plugin.package.playtime = msgString.Split(": ")[1]; // TODO: lol
            Completed?.Invoke();
        }
    }
}