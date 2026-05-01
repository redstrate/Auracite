using System;
using Dalamud.Game.Chat;
using Dalamud.Game.Text;
using Dalamud.Game.Text.SeStringHandling;

namespace Auracite;

public class PlaytimeStep : IStep
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

    public void Run()
    {
    }

    public string StepName()
    {
        return "Playtime";
    }

    public string StepDescription()
    {
        return "Type /playtime into the chat window.";
    }
    
    // Verbatim prefixes from LogMessage row 859 in the four supported client
    // languages. Any whitespace or colon between the prefix and the value is
    // stripped at extraction time so the same code path handles all four.
    private static readonly string[] PlaytimeMarkers =
    {
        "Total Play Time",      // EN
        "Temps de jeu total",   // FR
        "Gesamtspielzeit",      // DE
        "累積プレイ時間",         // JP — note: no colon between prefix and value
    };

    private static readonly char[] MarkerSeparators =
    {
        ':',        // EN, DE
        ' ',        // ASCII space
        ' ',   // NBSP — appears before the colon in FR
        '　',   // ideographic space — possible in JP
    };

    private void OnChatMessage(IHandleableChatMessage message)
    {
        if (message.LogKind != XivChatType.SystemMessage) return;
        var msgString = message.Message.ToString();

        foreach (var marker in PlaytimeMarkers)
        {
            var markerIdx = msgString.IndexOf(marker, StringComparison.Ordinal);
            if (markerIdx < 0) continue;

            var rest = msgString[(markerIdx + marker.Length)..]
                .TrimStart(MarkerSeparators)
                .TrimEnd();
            Plugin.package.playtime = rest;
            Completed?.Invoke();
            return;
        }
    }
}
