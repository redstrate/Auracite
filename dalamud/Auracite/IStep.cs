using System;

namespace Auracite;

public interface IStep
{
    public event CompletedDelegate Completed;

    string StepName();
    string StepDescription();
    
    delegate void CompletedDelegate();
}