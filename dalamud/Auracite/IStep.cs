using System;

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
}
