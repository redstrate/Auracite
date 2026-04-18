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
    bool NeedsUpdateEveryFrame()
    {
        return false;
    }
    
    delegate void CompletedDelegate();

    public static NameValue SaveNameValue<T>(uint key, Func<T, ReadOnlySeString> fieldSelector) where T : struct, IExcelRow<T> {
        var newValue = new NameValue();
        var row = Plugin.DataManager.GetExcelSheet<T>()?.GetRow(key);
        if (row != null) {
            newValue.name = fieldSelector(row.Value).ToString();
        }
        newValue.value = key;

        return newValue;
    }

    public static unsafe List<byte> ConsumeBitArray(BitArray array) {
        return new List<byte>(new ReadOnlySpan<byte>(array.Pointer, array.ByteLength).ToArray());
    }
}
