# Type-annotated functions for SerialEM script commands.
# Some add additional safety checks, or delays that ensure a process finishes before the script continues.
from time import sleep

def NumMontageTiles() -> int:
    return int(sem.ReportNumMontagePieces())

def SetWorkingDir(d:str) -> None:
    sem.SetDirectory(d)

def TurnOnFilament() -> None:
    sem.SetColumnOrGunValve(1)
    # Always turn on beam blank, for specimen safety:
    sem.SetBeamBlank(1)
    # Wait for the filament to heat up:
    sleep(FilamentHeatupSec)

def TurnOffFilament() -> None:
    sem.SetColumnOrGunValve(0)

def SetSpotSize(size:int) -> None:
    sem.SetSpotSize(size)

def OkBox(message:str) -> None:
    sem.OKBox(message)

def Record() -> None:
    sem.Record()