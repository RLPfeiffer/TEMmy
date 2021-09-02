# Type-annotated functions for SerialEM script commands.
# Some add additional safety checks.

def NumMontageTiles() -> int:
    return int(sem.ReportNumMontagePieces())

def SetWorkingDir(d:str) -> None:
    sem.SetDirectory(d)

def TurnOnFilament() -> None:
    sem.SetColumnOrGunValve(1)
    # Always turn on beam blank, for specimen safety:
    sem.SetBeamBlank(1)

def TurnOffFilament() -> None:
    sem.SetColumnOrGunValve(0)

def SetSpotSize(size:int) -> None:
    sem.SetSpotSize(size)

def OkBox(message:str) -> None:
    sem.OKBox(message)

def Record() -> None:
    sem.Record()