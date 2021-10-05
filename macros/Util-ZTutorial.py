# This file is named with a Z so it comes after every other Python function in Util files is defined

from os.path import exists, join
from typing import Callable
from typing import Any
from os import startfile
from glob import glob

# Allow multiple named tutorials, i.e. RC3 vs. Core, as differently-keyed lists of lambdas in this variable:
Steps: dict[str, list[Step]] = {}

NewSpecimenSteps:list[Step] = [
    # TODO coach on specifics of switching the rod and specimen.
    TellOperator("Put a new specimen in the scope."),
    DependingOnScope(DoNothing, TellOperator("Wait for the Penning Gauge to turn on (Green) and stabilize below 30.")),
    # TODO go to low mag 150x automatically. Only prompt to change aperture
    TellOperator("Go to low mag 150x with no aperture inserted."),
    # TODO on TEM2, coach a camera insertion workaround to avoid penning gauge spike with filament on?
    DoAutomatically(TurnOnFilament),
    DoAutomatically(ScreenDown),
    TellOperator("Scroll the stage to find a region of formvar, and click 'Add Stage Pos' in the navigator window."),
]

LowMagCookSteps:list[Step] = [
    DoAutomatically(lambda: SetSpotSize(1)),
    TellOperator("Scroll the stage to the center of the tissue. Remove the mirror. Center and tighten the beam to the inner brackets. Then click 'Next Step' and wait for 7 minutes."),
    DoAutomatically(lambda: LowMagCook(7)),
]

AcquireAtItemsMessage = "In the menubar, click Navigator -> AcquireAtItems. Choose '*'. Leave FilamentManager selected, and click OK. Then move the mirror out of the way."

def OpenLastRC3Snapshot(mag:int) -> Step:
    def step() -> None:
        try:
            startfile(glob(join(DropboxPath, "TEMSnapshots", f"Jones RC3 * x{mag} *.jpg"))[-1])
            RunNextStep()
        except:
            TellOperator(f"Open DROPBOX/TEMSnapshots and open the latest RC3 snapshot at x{mag}")
    return step

MainRC3Steps:list[Step] = [
    OpenLastRC3Snapshot(150),
    TellOperator("Locate the center point at 150x, click it, and click 'Add Marker' in the navigator window."),
    # TODO automatically go to the new marker point (if "current navigator item" refers to the selected item in the UI)
    TellOperator("Click 'Go to Marker' in the navigator window."),
    DoAutomatically(Record),
    DoAutomatically(lambda: TakeSnapshotWithNotes("", False)),
    DoAutomatically(lambda: SetBeamBlank(True)),
    # TODO go to high mag 2000x automatically
    TellOperator("Go to high mag 2000x with the second aperture inserted."),
    DependingOnScope(TellOperator("Spread the beam by several turns (by turning the 'brightness' knob clockwise.)"), DoNothing),
    DoAutomatically(ScreenDown),
    TellOperator("Use the aperture X/Y dials to center the aperture."),
    TellOperator("Center the beam and use image wobble and the focus knob to adjust focus. Make sure the beam is spread around 100 Current Density, and click Next Step."),
    DoAutomatically(Record),
    OpenLastRC3Snapshot(2000),
    TellOperator("Find the center point at 2000x, and click it. Then delete the last navigator item."),
    DoAutomatically(lambda: TakeSnapshotWithNotes("", False)),
    TellOperator("In the menubar, click Navigator -> Montaging and Grids -> Add Circle Polygon. Type 125"),
    # TODO automatically go to 5000x
    TellOperator("Go to 5000x."),
    TellOperator("In the navigator window, click and drag the circle polygon item above the formvar point in the item list."),
    TellOperator("With the circle polygon selected, check the boxes for 'Aquire', 'New File At Item', 'Montaged Images', 'Fit Montage to Polygon'. Make sure 'Go from center out and anchor at 2000x' is active and click ok. Then select the generated idoc file. Choose to overwrite it."),
    # TODO automatically go to the center
    TellOperator("In the navigator window, click Go To XY"),
    DoAutomatically(ScreenDown),
    TellOperator("Tighten the beam and center it. Check focus again, then go to 100 Current density and click Next Step."),
    DoAutomatically(Autofocus),
    DoAutomatically(Record),
    TellOperator("If the green number representing the circle's center has shifted from where you put it, use 'Move item' to fix it, then click 'Stop Moving.'"),
]

Steps["RC3"] = NewSpecimenSteps + LowMagCookSteps + MainRC3Steps + [
    DependingOnYesNo("Are there any holes in the section or formvar?", TellOperator(AcquireAtItemsMessage.replace("*", "CalibrateAndRecapturePy")), TellOperator(AcquireAtItemsMessage.replace("*", "HighMagCookPy")))
    # TODO coach on closing the previous serialEM, and deciding whether the overview indicates a recapture is needed
]

Steps["RC3 Recapture"] = NewSpecimenSteps + MainRC3Steps + [
    TellOperator(AcquireAtItemsMessage.replace("*", "CalibrateAndRecapturePy"))
    # TODO coach on closing the previous serialEM, and deciding whether the overview indicates another recapture is needed
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
