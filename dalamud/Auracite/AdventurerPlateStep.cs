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
    public enum DecorationType
    {
        Invald = 0x0,
        Backing = 0x1,
        PatternOverlay = 0x2,
        PortraitFrame = 0x3,
        PlateFrame = 0x4,
        Accent = 0x5,
    }

    [StructLayout(LayoutKind.Explicit, Size = 0x8)]
    public unsafe struct DecorationSet
    {
        [FieldOffset(0x0)] public DecorationType Type;
    }

    [System.Runtime.InteropServices.StructLayoutAttribute(LayoutKind.Sequential, Pack = 1)]
    [InlineArray(5)]
    public struct FixedSizeArray5<T> where T : unmanaged
    {
        private T _element0;
    }

    // Currently upstreaming via: https://github.com/aers/FFXIVClientStructs/pull/1269
    [StructLayout(LayoutKind.Explicit, Size = 0x9B0)]
    public unsafe struct CustomStorage
    {
        /// True if the player has the "Edit Plate" window open.
        [FieldOffset(0x2)] public bool Editing;
        [FieldOffset(0x4)] public uint EntityId;
        [FieldOffset(0x8)] public ulong ContentId;
        
        [FieldOffset(0x1B)] public bool InvertPortraitPlacement;
        [FieldOffset(0x1C)] public byte BasePlate;
        [FieldOffset(0x1E)] public byte TopBorder;
        [FieldOffset(0x1F)] public byte BottomBorder;
        
        /// The number of decorations.
        /// This is any Pattern Overlay, Backing, Portrait Frame, Plate Frame and Accents.
        [FieldOffset(0x20)] public ushort NumDecorations;
        
        /// The size of this array is NumDecorations.
        [FieldOffset(0x22)] public FixedSizeArray5<ushort> DecorationRowIndices;
        
        [FieldOffset(0x60)] public Utf8String Name;
        [FieldOffset(0xC8)] public ushort WorldId;
        [FieldOffset(0xCA)] public byte ClassJobId;

        [FieldOffset(0xCC)] public byte GcRank;

        [FieldOffset(0xD0)] public ushort Level;
        [FieldOffset(0xD2)] public ushort TitleId;

        [FieldOffset(0xE0)] public Utf8String FreeCompany;
        [FieldOffset(0x148)] public Utf8String SearchComment;
        [FieldOffset(0x1B0)] public Utf8String SearchCommentRaw; // contains unresolved AutoTranslatePayloads

        [FieldOffset(0x258)] public uint Activity1IconId;
        [FieldOffset(0x260)] public Utf8String Activity1Name;
        [FieldOffset(0x2C8)] public uint Activity2IconId;
        [FieldOffset(0x2D0)] public Utf8String Activity2Name;
        [FieldOffset(0x338)] public uint Activity3IconId;
        [FieldOffset(0x340)] public Utf8String Activity3Name;
        [FieldOffset(0x3A8)] public uint Activity4IconId;
        [FieldOffset(0x3B0)] public Utf8String Activity4Name;
        [FieldOffset(0x418)] public uint Activity5IconId;
        [FieldOffset(0x420)] public Utf8String Activity5Name;
        [FieldOffset(0x488)] public uint Activity6IconId;
        [FieldOffset(0x490)] public Utf8String Activity6Name;

        [FieldOffset(0x540)] public CharaViewPortrait CharaView;

        /// The size of this array is NumDecorations.
        [FieldOffset(0x22C)] public FixedSizeArray5<DecorationSet> Decorations;

        [FieldOffset(0x960)] public Texture* PortraitTexture;
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
                var customData = (CustomStorage*)AgentCharaCard.Instance()->Data;
                var image = GetCurrentCharaViewImage();
                Plugin.package.portrait = image.ToBase64String(PngFormat.Instance);
                
                if (customData->BasePlate != 0)
                {
                    Plugin.package.base_plate = GetImage(ResolveCardBase(customData->BasePlate))
                        .ToBase64String(PngFormat.Instance);
                }

                for (int i = 0; i < customData->NumDecorations; i++)
                {
                    var decoration = customData->Decorations[i];
                    var rowIndex = customData->DecorationRowIndices[i];
                    if (rowIndex == 0)
                    {
                        continue;
                    }
                    
                    switch (decoration.Type)
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

                if (customData->TopBorder != 0)
                {
                    Plugin.package.top_border = GetImage(ResolveCardHeaderTop(customData->TopBorder))
                        .ToBase64String(PngFormat.Instance);
                }

                if (customData->BottomBorder != 0)
                {
                    Plugin.package.bottom_border = GetImage(ResolveCardHeaderBottom(customData->BottomBorder))
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