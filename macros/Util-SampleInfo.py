from os.path import join
from os import makedirs
from typing import Tuple
from collections import OrderedDict
from typing import Optional

SampleNotes = Tuple[int, str, str, str, int, str, str, str, bool, bool, bool, str, str, str]
CurrentSampleNotes:Optional[OrderedDict[str, SampleNotes]] = None

SampleInfoKeys = [
    "Version",
    "Microscope",
    "Block",
    "Grid",
    "Rod",
    "Investigator",
    "Experiment",
    "CapturedBy",
    "CameraGainReference",
    "CameraQuadrantReference",
    "NewFilament",
    "ProcedureChanges",
    "Observations",
    "Notes"
]

Version = 2
Microscope = ScopeName
NumRods = 4
MainInvestigator = "Jones"
MainExperiment = "RC3"
MainOperator = "Nat Nelson"

def PromptForSampleInfo() -> None:
    Blocks = []
    MoreBlocks = True
    while MoreBlocks:
        Blocks.append(EnterString("Block # / Section #"))
        MoreBlocks = YesNoBox("Any more blocks or sections on this grid?")

    Grid = EnterString("Grid #")
    Rod = EnterInt("Rod #", minvalue=1, maxvalue=NumRods)
    Investigator = EnterString("Investigator", MainInvestigator)
    Experiment = EnterString("Experiment Series", MainExperiment)
    CapturedBy = EnterString("Captured By", MainOperator)
    CameraGainReference = YesNoBox("Did you, or are you going to, run Gain reference?")
    CameraQuadrantReference = False
    NewFilament = YesNoBox("Was the filament just changed?")
    ProcedureChanges = ""
    Observations = ""
    OtherNotes = ""

    global CurrentSampleNotes
    CurrentSampleNotes = OrderedDict()
    for Block in Blocks:
        Notes = (Version, Microscope, Block, Grid, Rod, Investigator, Experiment, CapturedBy, CameraGainReference, CameraQuadrantReference, NewFilament, ProcedureChanges, Observations, OtherNotes)
        CurrentSampleNotes[Block] = Notes
        WriteNotesFiles(Block, Notes)

def PromptForProcessNotes() -> None:
    global CurrentSampleNotes
    assert CurrentSampleNotes is not None, "No sample notes have been entered!"
    for Block, Notes in CurrentSampleNotes.items():
        Version, Microscope, Block, Grid, Rod, Investigator, Experiment, CapturedBy, CameraGainReference, CameraQuadrantReference, NewFilament, _, _, _ = Notes
        ProcedureChanges = EnterString(f"{Block}: Any changes from normal procedure?")
        Observations = EnterString(f"{Block}: Any observations regarding the capture process or data quality?")
        OtherNotes = EnterString(f"{Block}: Other notes?")
        NewNotes = (Version, Microscope, Block, Grid, Rod, Investigator, Experiment, CapturedBy, CameraGainReference, CameraQuadrantReference, NewFilament, ProcedureChanges, Observations, OtherNotes)
        CurrentSampleNotes[Block] = NewNotes
        WriteNotesFiles(Block, NewNotes)

def GetCaptureDir(Block:str) -> str:
    assert CurrentSampleNotes is not None
    Notes = CurrentSampleNotes[Block]
    Investigator = Notes[SampleInfoKeys.index("Investigator")]
    Experiment = Notes[SampleInfoKeys.index("Experiment")]
    Dir = f"{Investigator}_{Experiment}_{Block}"
    if Investigator != MainInvestigator:
        Dir = f"core_{Dir}"
    return join(DataPath, Dir)

def WriteNotesFiles(Block:str, Notes:SampleNotes) -> None:
    # Make a data output folder for the block
    BlockFolder = GetCaptureDir(Block)
    makedirs(BlockFolder, exist_ok=True)
    # Write notes to a JSON file
    with open(join(BlockFolder, f"{Block}.json"), "w") as json:
        json.write("{" + newline)
        for idx, key in enumerate(SampleInfoKeys):
            jsonValue = Notes[idx]
            if isinstance(Notes[idx], str):
                jsonValue = f'"{Notes[idx]}"'
            elif isinstance(Notes[idx], bool):
                jsonValue = str(Notes[idx]).lower()

            json.write(f'  "{key}": {jsonValue}')
            if idx != len(SampleInfoKeys) - 1:
                json.write(",")
            json.write(newline)
        json.write("}")

    # Write notes to a TXT file
    with open(join(BlockFolder, f"{Block}.txt"), "w") as txt:
        for idx, key in enumerate(SampleInfoKeys):
            txt.write(f"{key}: {Notes[idx]}{newline}")