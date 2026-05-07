using System;
using System.Collections.Generic;
using InteropGenerator.Runtime;
using Lumina.Excel;
using Lumina.Text.ReadOnly;

namespace Auracite;

public interface IStep : IDisposable
{
    public event CompletedDelegate Completed;

    void Run();

    string StepName();
    string StepDescription();
    bool IsEnd()
    {
        return false;
    }

    delegate void CompletedDelegate();

    public static NameValue SaveNameValue<T>(uint key, Func<T, ReadOnlySeString> fieldSelector) where T : struct, IExcelRow<T>
    {
        var row = Plugin.DataManager.GetExcelSheet<T>()?.GetRow(key);
        string? name = null;
        if (row != null)
        {
            name = fieldSelector(row.Value).ToString();
        }

        return new NameValue { name = name, value = key };
    }

    public static unsafe List<byte> ConsumeBitArray(BitArray array)
    {
        return new List<byte>(new ReadOnlySpan<byte>(array.Pointer, array.ByteLength).ToArray());
    }
}
