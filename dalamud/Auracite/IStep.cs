using System;

namespace Auracite;

public interface IStep
{
    public event CompletedDelegate Completed;

    void Run();

    string StepName();
    string StepDescription();
    
    delegate void CompletedDelegate();
}