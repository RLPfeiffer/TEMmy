# Type-annotated functions for SerialEM commands

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