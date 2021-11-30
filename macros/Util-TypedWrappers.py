# Type-annotated functions for SerialEM script commands.
# Some add additional safety checks, or delays that ensure a process finishes before the script continues.
from time import sleep

def NumMontageTiles() -> int:
    return int(sem.ReportNumMontagePieces())

def SetWorkingDir(d:str) -> None:
    sem.SetDirectory(d)

def TurnOnFilament() -> None:
    if not IsFilamentOn():
        sem.SetColumnOrGunValve(1)
        # Always turn on beam blank, for specimen safety:
        sem.SetBeamBlank(1)
        # Wait for the filament to heat up:
        sleep(FilamentHeatupSec)

def TurnOffFilament() -> None:
    if IsFilamentOn():
        sem.SetColumnOrGunValve(0)
        sleep(FilamentCooldownSec)

def IsFilamentOn() -> bool:
    value:int = sem.ReportColumnOrGunValve()
    if value == 1:
        return True
    elif value == 0:
        return False
    else:
        # SerialEM can also report -1 for the filament status
        raise ValueError("filament current is inconsistent with the beam switch state")

def SetSpotSize(size:int) -> None:
    sem.SetSpotSize(size)

def OkBox(message:str) -> None:
    sem.OKBox(message)

def Record() -> None:
    sem.Record()

def SetBeamBlank(on:bool) -> None:
    sem.SetBeamBlank(1 if on else 0)

def ScreenDown() -> None:
    sem.ScreenDown()

def ScreenUp() -> None:
    sem.ScreenUp()

def Autofocus() -> None:
    sem.AutoFocus()

def SetMagIndex(i:int) -> None:
    sem.SetMagIndex(i)

def MoveToNavItem(i:Optional[int]) -> None:
    if i != None:
        sem.MoveToNavItem(i)
    else:
        sem.MoveToNavItem()