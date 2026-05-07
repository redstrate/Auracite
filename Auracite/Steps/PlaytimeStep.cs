using System.Runtime.InteropServices;
using Dalamud.Hooking;
using FFXIVClientStructs.FFXIV.Client.UI;

namespace Auracite;

public class PlaytimeStep : IStep
{
    private readonly Hook<UIModule.Delegates.HandlePacket>? handlePacketHook = null!;

    public PlaytimeStep()
    {
        unsafe
        {
            handlePacketHook = Plugin.Hooking.HookFromAddress<UIModule.Delegates.HandlePacket>((nint)UIModule.StaticVirtualTablePointer->HandlePacket, HandlePacket);
        }
        handlePacketHook?.Enable();
    }

    public void Dispose()
    {
        handlePacketHook?.Dispose();
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

    private unsafe void HandlePacket(UIModule* thisPtr, UIModulePacketType type, uint uintParam, void* packet)
    {
        if (type == UIModulePacketType.PrintPlayTime)
        {
            var minutes = (uint)Marshal.ReadInt32((nint)packet + 0x10);
            Plugin.package.playtime = minutes;

            Completed?.Invoke();
        }
    }
}
