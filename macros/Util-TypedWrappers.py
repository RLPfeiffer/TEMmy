# Type-annotated functions for SerialEM commands

def NumMontageTiles() -> int:
    return int(sem.ReportNumMontagePieces())

def SetWorkingDir(d:str) -> None:
    sem.SetDirectory(d)