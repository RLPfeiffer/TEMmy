from os.path import exists
from typing import Callable
from typing import Any

# A Step is a function without arguments. Its return value will be ignored
Step = Callable[[], Any]

# Allow multiple named tutorials, i.e. RC3 vs. Core, as differently-keyed lists of lambdas in this variable:
Steps: dict[str, list[Step]] = {}

# TODO
Steps["RC3"] = [
    # TODO coach on closing the previous serialEM, and deciding whether to recapture.
    # TODO coach on specifics of switching the rod and specimen.
    TellOperator("Put a new specimen in the scope."),
    # TODO go to low mag 150x automatically. Only prompt to change aperture
    TellOperator("Go to low mag 150x with no aperture inserted."),
    # TODO on TEM2, coach a camera insertion workaround to avoid penning gauge spike with filament on?
    DoAutomatically(TurnOnFilament),

]

# TODO
Steps["Core"] = [
    lambda: print("Starting Core capture tutorial")
]

# TODO
Steps["S"] = [
    lambda: print("Starting Single-Tile Recapture tutorial")
]

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
    TutorialNames = Steps.keys()
    assert 3 == len(TutorialNames), "SerialEM can only fit 3 choices in a dialog box, so we can only have 3 tutorials."
    t1, t2, t3 = TutorialNames
    SetCurrentStep(0)
    SetCurrentTutorial(ThreeChoiceBox("What kind of capture are you running?", t1, t2, t3))
    RunCurrentStep()
    
# Run the current tutorial step. Some steps may automatically perform work and increment the step counter, but ones with information can be re-run as many times as necessary to remind the operator what step is next.
def RunCurrentStep() -> None:
    Steps[CurrentTutorial()][CurrentStep()]()

def RunNextStep() -> Any:
    if CurrentStep() + 1 < len(Steps[CurrentTutorial()]):
        SetCurrentStep(CurrentStep() + 1)
        RunCurrentStep()
    else:
        OkBox(f"Reached the end of tutorial {CurrentTutorial()}")
    # Must return a value so it can be used in fake multiline lambdas
    return 0

# Types of protocol step:
def TellOperator(message:str) -> Step:
    return lambda: OkBox(message)

def DoAutomatically(func:Callable[[], Any]) -> Step:
    return lambda: (func(), RunNextStep())
