# This file is named with a ZA so it comes after every other Python function in Util files is defined but before the steps are defined

from typing import Callable
from typing import Any

# A Step is a void function without arguments.
Step = Callable[[], None]

# Types of protocol step for Tutorial.py:

# Display an instruction that must be carried out
def TellOperator(message:str) -> Step:
    def step() -> None:
        Pause(message)
        RunNextStep()
    return step

# Display an instruction that must be carried out manually on the TEM controls:
def TellOperatorTEM(message:str) -> Step:
    return TellOperator(f"Do the following using the TEM controls, then click Yes when it's done. If you need to pause and make a manual correction first, click No: {newline}{newline} {message}")

# Display an instruction that the operator must do in SerialEM, then click "Next Step"
def TellOperatorSEM(message:str) -> Step:    
    return lambda: OkBox(f"Click OK, do this in SerialEM, then click Next Step: {newline}{newline} {message}")

# Do something through SerialEM Scripting and keep going when it's finished:
def DoAutomatically(func:Callable[[], None]) -> Step:
    def step() -> None:
        func()
        RunNextStep()
    return step

def DoNothing() -> None:
    RunNextStep()

# Do a step differently depending on the scope being used
def DependingOnScope(tem1Step:Step, tem2Step:Step) -> Step:
    def step() -> None:
        if ScopeName == "TEM1":
            tem1Step()
        elif ScopeName == "TEM2":
            tem2Step()
        else:
            raise ValueError(f"ScopeName has unexpected value {ScopeName}")
    return step

def DependingOnYesNo(question:str, yesStep:Step, noStep:Step) -> Step:
    def step() -> None:
        if YesNoBox(question):
            yesStep()
        else:
            noStep()
    return step

def SkipSteps(steps:list[Step]) -> Step:
    def step() -> None:
        SetCurrentStep(CurrentStep() + len(steps))
        RunNextStep()
    return step

# The current step needs to be written to/read from a file because SerialEM python doesn't have persistent global variables between sesssions,
# and we want to preserve the current tutorial state if the operator has to restart SerialEM for any reason
CurrentStepFile = "C:/Users/VALUEDGATANCUSTOMER/CurrentTutorialStep.txt"

def CurrentStep() -> int:
    if exists(CurrentStepFile):
        with open(CurrentStepFile, "r") as f:
            return int(f.readline().strip())
    else:
        return 0

def SetCurrentStep(s: int) -> None:
    with open(CurrentStepFile, "w") as f:
        f.write(str(s))

# The current tutorial (RC3, Core, etc.) needs to be serialized between sessions just like the step counter
CurrentTutorialFile = "C:/Users/VALUEDGATANCUSTOMER/CurrentTutorial.txt"

def CurrentTutorial() -> str:
    assert exists(CurrentTutorialFile), "No tutorial is selected. Run StartTutorial() and choose one"
    with open(CurrentTutorialFile, "r") as f:
        return f.readline().strip()

def SetCurrentTutorial(t: str) -> None:
    with open(CurrentTutorialFile, "w") as f:
        f.write(t)

def StartTutorial() -> None:
    TutorialNames = [name for name in Steps.keys()]
    SetCurrentStep(0)
    SetCurrentTutorial(ManyChoiceBox("Choose a tutorial", TutorialNames))
    RunCurrentStep()
    
# Run the current tutorial step. Some steps may automatically perform work and increment the step counter, but ones with information can be re-run as many times as necessary to remind the operator what step is next.
def RunCurrentStep() -> None:
    Steps[CurrentTutorial()][CurrentStep()]()

def RunNextStep() -> None:
    if CurrentStep() + 1 < len(Steps[CurrentTutorial()]):
        SetCurrentStep(CurrentStep() + 1)
        RunCurrentStep()
    else:
        OkBox(f"Reached the end of tutorial {CurrentTutorial()}")

from os.path import exists, join
from typing import Callable
from typing import Any
from os import startfile
from glob import glob

# Allow multiple named tutorials, i.e. RC3 vs. Core, as differently-keyed lists of lambdas in this variable:
Steps: dict[str, list[Step]] = {}

