using System;

namespace Auracite;

public interface IStep : IDisposable
{
    public event CompletedDelegate Completed;

    void Run();

    string StepName();
    string StepDescription();
    
    delegate void CompletedDelegate();
}