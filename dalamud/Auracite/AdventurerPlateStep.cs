using System;
using System.IO;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Dalamud.Utility;
using FFXIVClientStructs.FFXIV.Client.Graphics.Kernel;
using FFXIVClientStructs.FFXIV.Client.System.String;
using FFXIVClientStructs.FFXIV.Client.UI.Agent;
using FFXIVClientStructs.FFXIV.Client.UI.Misc;
using Lumina.Data.Files;
using Lumina.Excel.Sheets;
using SharpDX;
using SharpDX.Direct3D11;
using SharpDX.DXGI;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Png;
using SixLabors.ImageSharp.PixelFormats;

namespace Auracite;

public class AdventurerPlateStep : IStep
{
    // Remove when https://github.com/aers/FFXIVClientStructs/pull/1319 is merged
    enum DecorationType
    {
        Invalid = 0x0,
        Backing = 0x1,
        PatternOverlay = 0x2,
        PortraitFrame = 0x3,
        PlateFrame = 0x4,
        Accent = 0x5,
    }

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
                var storage = AgentCharaCard.Instance()->Data;
                var image = GetCurrentCharaViewImage();
                Plugin.package.portrait = image.ToBase64String(PngFormat.Instance);

                var plateDesign = storage->PlateDesign;
                
                if (plateDesign.BasePlate != 0)
                {
                    Plugin.package.base_plate = GetImage(ResolveCardBase(plateDesign.BasePlate))
                        .ToBase64String(PngFormat.Instance);
                }

                for (int i = 0; i < plateDesign.NumDecorations; i++)
                {
                    var decoration = storage->Decorations[i];
                    var rowIndex = plateDesign.Decorations[i];
                    if (rowIndex == 0)
                    {
                        continue;
                    }
                    
                    switch ((DecorationType)decoration.Index)
                    {
                        case DecorationType.PatternOverlay:
                        {
                            Plugin.package.pattern_overlay = GetImage(ResolveCardDecoration(rowIndex))
                                .ToBase64String(PngFormat.Instance);
                        }
                            break;
                        case DecorationType.Backing:
                        {
                            Plugin.package.backing = GetImage(ResolveCardDecoration(rowIndex))
                                .ToBase64String(PngFormat.Instance);
                        }
                            break;
                        case DecorationType.PortraitFrame:
                        {
                            Plugin.package.portrait_frame = GetImage(ResolveCardDecoration(rowIndex))
                                .ToBase64String(PngFormat.Instance);
                        }
                            break;
                        case DecorationType.PlateFrame:
                        {
                            Plugin.package.plate_frame = GetImage(ResolveCardDecoration(rowIndex))
                                .ToBase64String(PngFormat.Instance);
                        }
                            break;
                        case DecorationType.Accent:
                        {
                            Plugin.package.accent = GetImage(ResolveCardDecoration(rowIndex))
                                .ToBase64String(PngFormat.Instance);
                        }
                            break;
                    }
                }

                if (plateDesign.TopBorder != 0)
                {
                    Plugin.package.top_border = GetImage(ResolveCardHeaderTop(plateDesign.TopBorder))
                        .ToBase64String(PngFormat.Instance);
                }

                if (plateDesign.BottomBorder != 0)
                {
                    Plugin.package.bottom_border = GetImage(ResolveCardHeaderBottom(plateDesign.BottomBorder))
                        .ToBase64String(PngFormat.Instance);
                }

                Plugin.package.plate_title = Title?.Feminine.ToString(); // TODO: Support mascs
                Plugin.package.plate_title_is_prefix = Title?.IsPrefix;
                Plugin.package.plate_class_job = ClassJob?.Name.ToString();
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

    public Image GetImage(string path)
    {
        var tex = Plugin.DataManager.GetFile<TexFile>(path);
        tex.LoadFile();
        var imageData = tex.GetRgbaImageData();
        return Image.LoadPixelData<Rgba32>(imageData, tex.Header.Width, tex.Header.Height);
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

    public string ResolveCardBase(uint rowIndex)
    {
        var row = Plugin.DataManager.GetExcelSheet<CharaCardBase>()?.GetRow(rowIndex);
        return $"ui/icon/{row?.Image.ToString().Substring(0, 3)}000/{row?.Image}_hr1.tex";
    }
    
    public string? ResolveCardDecoration(uint rowIndex)
    {
        var row = Plugin.DataManager.GetExcelSheet<CharaCardDecoration>()?.GetRow(rowIndex);
        return $"ui/icon/{row?.Image.ToString().Substring(0, 3)}000/{row?.Image}_hr1.tex";
    }
    
    public string? ResolveCardHeaderTop(uint rowIndex)
    {
        var row = Plugin.DataManager.GetExcelSheet<CharaCardHeader>()?.GetRow(rowIndex);
        return $"ui/icon/{row?.TopImage.ToString().Substring(0, 3)}000/{row?.TopImage}_hr1.tex";
    }
    
    public string? ResolveCardHeaderBottom(uint rowIndex)
    {
        var row = Plugin.DataManager.GetExcelSheet<CharaCardHeader>()?.GetRow(rowIndex);
        return $"ui/icon/{row?.BottomImage.ToString().Substring(0, 3)}000/{row?.BottomImage}_hr1.tex";
    }
}
