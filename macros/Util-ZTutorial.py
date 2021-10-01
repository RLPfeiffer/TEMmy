# This file is named with a Z so it comes after every other Python function in Util files is defined

from os.path import exists
from typing import Callable
from typing import Any


# Allow multiple named tutorials, i.e. RC3 vs. Core, as differently-keyed lists of lambdas in this variable:
Steps: dict[str, list[Step]] = {}

# TODO
Steps["RC3"] = [
    # TODO coach on specifics of switching the rod and specimen.
    TellOperator("Put a new specimen in the scope."),
    # TODO go to low mag 150x automatically. Only prompt to change aperture
    TellOperator("Go to low mag 150x with no aperture inserted."),
    # TODO on TEM2, coach a camera insertion workaround to avoid penning gauge spike with filament on?
    DoAutomatically(TurnOnFilament),
    DoAutomatically(ScreenDown),
    TellOperator("Scroll the stage to find a region of formvar, and click 'Add Stage Pos' in the navigator window."),
    DoAutomatically(lambda: SetSpotSize(1)),
    TellOperator("Scroll the stage to the center of the tissue. Center and tighten the beam to the inner brackets. Then click 'Next Step' and wait for 7 minutes."),
    DoAutomatically(lambda: LowMagCook(7)),
    # TODO automatically open the last 150x snapshot
    TellOperator("Locate the center point at 150x, click it, and click 'Add Marker' in the navigator window."),
    # TODO automatically go to the new marker point (if "current navigator item" refers to the selected item in the UI)
    TellOperator("Click 'Go to Marker' in the navigator window."),
    DoAutomatically(Record),
    DoAutomatically(lambda: TakeSnapshotWithNotes("", False)),
    
    DoAutomatically(lambda: SetBeamBlank(True)),
    # TODO go to high mag 2000x automatically
    TellOperator("Go to high mag 2000x with the second aperture inserted."),
    # TODO if TEM1, prompt to spread the beam a lot to prevent a cook spot
    DoAutomatically(ScreenDown),
    TellOperator("Use the aperture dials to center the aperture."),
    TellOperator("Center the beam and use image wobble and the focus knob to adjust focus."),
    DoAutomatically(Record),
    # TODO automatically open the last 2000x screenshot
    TellOperator("Find the center point at 2000x, and click it. Then delete the last navigator item."),
    DoAutomatically(lambda: TakeSnapshotWithNotes("", False)),
    TellOperator("In the menubar, click Navigator -> Montaging and Grids -> Add Circle Polygon. Type 125"),
    # TODO automatically go to 5000x
    TellOperator("Go to 5000x."),
    TellOperator("In the navigator window, click and drag the circle polygon item above the formvar point."),
    TellOperator("In the navigator window, check 'Aquire', 'New File At Item', and select the idoc file. Choose to overwrite it."),
    # TODO automatically go to the center
    TellOperator("In the navigator window, click Go To XY"),
    DoAutomatically(ScreenDown),
    TellOperator("Tighten the beam and center it. Check focus again, then go to 100 Current density."),
    DoAutomatically(Autofocus),
    DoAutomatically(Record),
    TellOperator("If the number representing the center has shifted from where you put it, use 'Move item' to fix it, then click 'Stop Moving.'"),
    TellOperator("In the menubar, click Navigator -> AcquireAtItems. Choose 'CalibrateAndRecapturePy' if there are any holes in the section. Otherwise, choose 'HighMagCookPy' and move the mirror out of the way."),

    # TODO coach on closing the previous serialEM, and deciding whether to recapture.
]

# TODO
Steps["Core"] = [
    lambda: print("Starting Core capture tutorial")
]

# TODO
Steps["SingleTileRecaptures"] = [
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
    TutorialNames = [name for name in Steps.keys()]
    SetCurrentStep(0)
    SetCurrentTutorial(ManyChoiceBox("Choose a tutorial", TutorialNames))
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
