using System;
using System.IO;
using FFXIVClientStructs.FFXIV.Client.UI.Agent;
using Lumina.Excel.GeneratedSheets;
using SharpDX;
using SharpDX.Direct3D11;
using SharpDX.DXGI;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Png;
using SixLabors.ImageSharp.PixelFormats;

namespace Auracite;

public class AdventurerPlateStep : IStep
{
    public AdventurerPlateStep()
    {
        
    }

    public void Dispose()
    {
        
    }

    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        if (IsInPlateWindow())
        {
            unsafe
            {
                var image = GetCurrentCharaViewImage();
                Plugin.package.portrait = image.ToBase64String(PngFormat.Instance);
                Plugin.package.plate_title = Title?.Feminine; // TODO: Support mascs
                Plugin.package.plate_title_is_prefix = Title?.IsPrefix;
                Plugin.package.plate_class_job = ClassJob?.Name;
                Plugin.package.plate_class_job_level = AgentCharaCard.Instance()->Data->Level;
                Plugin.package.search_comment = AgentCharaCard.Instance()->Data->SearchComment.ToString();
                Completed?.Invoke();
            }
        }
    }
    
    public unsafe Image GetCurrentCharaViewImage()
    {
        var texture = CppObject.FromPointer<Texture2D>((nint)AgentCharaCard.Instance()->Data->PortraitTexture->D3D11Texture2D);
        var device = (Device5)(IntPtr)FFXIVClientStructs.FFXIV.Client.Graphics.Kernel.Device.Instance()->D3D11Forwarder;
        
        // Copy to a CPU-mapped staging texture 
        var desc = texture.Description;
        
        using var stagingTexture = new Texture2D(device, new Texture2DDescription()
        {
            ArraySize = 1,
            BindFlags = BindFlags.None,
            CpuAccessFlags = CpuAccessFlags.Read,
            Format = desc.Format,
            Height = desc.Height,
            Width = desc.Width,
            MipLevels = 1,
            OptionFlags = desc.OptionFlags,
            SampleDescription = new SampleDescription(1, 0),
            Usage = ResourceUsage.Staging
        });

        device.ImmediateContext.CopyResource(texture, stagingTexture);

        device.ImmediateContext.MapSubresource(stagingTexture, 0, MapMode.Read, SharpDX.Direct3D11.MapFlags.None, out var dataStream);

        using var pixelDataStream = new MemoryStream();
        dataStream.CopyTo(pixelDataStream);

        device.ImmediateContext.UnmapSubresource(stagingTexture, 0);
        
        return Image.LoadPixelData<Bgra32>(pixelDataStream.ToArray(), desc.Width, desc.Height);
    }
    
    private unsafe Title? Title {
        get {
            ushort titleId = AgentCharaCard.Instance()->Data->TitleId;
            return titleId == 0
                ? null
                : Plugin.DataManager.GetExcelSheet<Title>()?.GetRow(titleId);
        }
    }
    
    private unsafe ClassJob? ClassJob {
        get {
            ushort classJobId = AgentCharaCard.Instance()->Data->ClassJobId;
            return classJobId == 0
                ? null
                : Plugin.DataManager.GetExcelSheet<ClassJob>()?.GetRow(classJobId);
        }
    }

    public string StepName()
    {
        return "Adventurer Plate";
    }

    public string StepDescription()
    {
        return "Type /adventurerplate into the chat window or open your Adventurer Plate.";
    }

    private static unsafe bool IsInPlateWindow()
    {
        return AgentCharaCard.Instance()->AgentInterface.IsAgentActive();
    }
}