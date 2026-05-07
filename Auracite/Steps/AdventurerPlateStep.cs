using System;
using Dalamud.Utility;
using FFXIVClientStructs.FFXIV.Client.UI.Agent;
using Lumina.Data.Files;
using Lumina.Excel.Sheets;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.PixelFormats;
using static FFXIVClientStructs.FFXIV.Client.UI.Agent.AgentCharaCard;
using TerraFX.Interop.DirectX;
using System.Runtime.CompilerServices;

namespace Auracite;

public class AdventurerPlateStep : IStep
{
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
                if (storage == null) return;
                var image = GetCurrentCharaViewImage(storage);
                if (image == null) return; // texture not ready yet, retry next frame

                var plateDesign = storage->PlateDesign;

                if (plateDesign.BasePlate != 0)
                {
                    Plugin.base_plate = GetImage(ResolveCardBase(plateDesign.BasePlate));
                }

                for (int i = 0; i < plateDesign.NumDecorations; i++)
                {
                    var decoration = storage->Decorations[i];
                    var rowIndex = plateDesign.Decorations[i];
                    if (rowIndex == 0)
                    {
                        continue;
                    }

                    var path = ResolveCardDecoration(rowIndex);
                    if (path == null)
                    {
                        continue;
                    }

                    switch (decoration.Type)
                    {
                        case AgentCharaCard.DecorationType.PatternOverlay:
                            {
                                Plugin.pattern_overlay = GetImage(path);
                            }
                            break;
                        case AgentCharaCard.DecorationType.Backing:
                            {
                                Plugin.backing = GetImage(path);
                            }
                            break;
                        case AgentCharaCard.DecorationType.PortraitFrame:
                            {
                                Plugin.portrait_frame = GetImage(path);
                            }
                            break;
                        case AgentCharaCard.DecorationType.PlateFrame:
                            {
                                Plugin.plate_frame = GetImage(path);
                            }
                            break;
                        case AgentCharaCard.DecorationType.Accent:
                            {
                                Plugin.accent = GetImage(path);
                            }
                            break;
                    }
                }

                if (plateDesign.TopBorder != 0)
                {
                    var path = ResolveCardHeaderTop(plateDesign.TopBorder);
                    if (path != null)
                    {
                        Plugin.top_border = GetImage(path);
                    }
                }

                if (plateDesign.BottomBorder != 0)
                {
                    var path = ResolveCardHeaderBottom(plateDesign.BottomBorder);
                    if (path != null)
                    {
                        Plugin.bottom_border = GetImage(path);
                    }
                }

                Plugin.package.plate.title = Title?.Feminine.ToString(); // TODO: Support mascs
                Plugin.package.plate.title_is_prefix = Title?.IsPrefix;
                Plugin.package.plate.class_job = ClassJob?.Name.ToString();
                Plugin.package.plate.class_job_level = AgentCharaCard.Instance()->Data->Level;
                Plugin.package.plate.search_comment = AgentCharaCard.Instance()->Data->SearchComment.ToString();
                Plugin.package.plate.invert_portrait_placement = storage->InvertPortraitPlacement;
                Completed?.Invoke();
            }
        }
    }

    public unsafe Image? GetCurrentCharaViewImage(Storage* storage)
    {
        var portraitTexture = storage->PortraitTexture;
        if (portraitTexture == null) return null;
        var texture = (ID3D11Texture2D*)portraitTexture->D3D11Texture2D;
        if (texture == null) return null;

        var device = (ID3D11Device*)Plugin.PluginInterface.UiBuilder.DeviceHandle;

        D3D11_TEXTURE2D_DESC desc;
        texture->GetDesc(&desc);

        desc.BindFlags = 0;
        desc.CPUAccessFlags = (uint)D3D11_CPU_ACCESS_FLAG.D3D11_CPU_ACCESS_READ;
        desc.Usage = D3D11_USAGE.D3D11_USAGE_STAGING;
        desc.MiscFlags = 0;
        desc.MipLevels = 1;

        ID3D11Texture2D* stagingTexture;
        if (device->CreateTexture2D(&desc, null, &stagingTexture) < 0)
            return null;

        ID3D11DeviceContext* context;
        device->GetImmediateContext(&context);

        context->CopyResource((ID3D11Resource*)stagingTexture, (ID3D11Resource*)texture);

        D3D11_MAPPED_SUBRESOURCE mapped;
        if (context->Map((ID3D11Resource*)stagingTexture, 0, D3D11_MAP.D3D11_MAP_READ, 0, &mapped) < 0)
        {
            stagingTexture->Release();
            return null;
        }

        var sourcePtr = (nint)mapped.pData;
        var rowPitch = mapped.RowPitch;
        var image = new Image<Bgra32>((int)desc.Width, (int)desc.Height);

        image.ProcessPixelRows(accessor =>
        {
            for (var y = 0; y < accessor.Height; y++)
            {
                var destSpan = accessor.GetRowSpan(y);
                var src = (byte*)sourcePtr + y * rowPitch;
                Buffer.MemoryCopy(src, Unsafe.AsPointer(ref destSpan[0]), destSpan.Length * 4, destSpan.Length * 4);
            }
        });

        context->Unmap((ID3D11Resource*)stagingTexture, 0);
        stagingTexture->Release();

        return image;
    }

    public Image? GetImage(string path)
    {
        var tex = Plugin.DataManager.GetFile<TexFile>(path);
        if (tex == null)
        {
            return null;
        }
        tex.LoadFile();
        var imageData = tex.GetRgbaImageData();
        return Image.LoadPixelData<Rgba32>(imageData, tex.Header.Width, tex.Header.Height);
    }

    private unsafe Title? Title
    {
        get
        {
            ushort titleId = AgentCharaCard.Instance()->Data->TitleId;
            return titleId == 0
                ? null
                : Plugin.DataManager.GetExcelSheet<Title>()?.GetRow(titleId);
        }
    }

    private unsafe ClassJob? ClassJob
    {
        get
        {
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
        return AgentCharaCard.Instance()->AgentInterface.IsAgentActive() && AgentCharaCard.Instance()->AgentInterface.IsAddonShown();
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

    bool RequiresManualConfirmation()
    {
        return true;
    }
}
